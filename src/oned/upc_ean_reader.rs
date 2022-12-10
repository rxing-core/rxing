/*
 * Copyright 2008 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    common::BitArray, BarcodeFormat, DecodeHintType, DecodeHintValue, Exceptions, RXingResult,
    RXingResultMetadataType, RXingResultMetadataValue, RXingResultPoint, Reader,
};

use super::{EANManufacturerOrgSupport, OneDReader, UPCEANExtensionSupport};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref EAN_MANUFACTURER_SUPPORT: EANManufacturerOrgSupport =
        EANManufacturerOrgSupport::default();
    pub static ref UPC_EAN_EXTENSION_SUPPORT: UPCEANExtensionSupport =
        UPCEANExtensionSupport::default();
}

// These two values are critical for determining how permissive the decoding will be.
// We've arrived at these values through a lot of trial and error. Setting them any higher
// lets false positives creep in quickly.
pub const MAX_AVG_VARIANCE: f32 = 0.48;
pub const MAX_INDIVIDUAL_VARIANCE: f32 = 0.7;

/**
 * Start/end guard pattern.
 */
pub const START_END_PATTERN: [u32; 3] = [1, 1, 1];

/**
 * Pattern marking the middle of a UPC/EAN pattern, separating the two halves.
 */
pub const MIDDLE_PATTERN: [u32; 5] = [1, 1, 1, 1, 1];
/**
 * end guard pattern.
 */
pub const END_PATTERN: [u32; 6] = [1, 1, 1, 1, 1, 1];
/**
 * "Odd", or "L" patterns used to encode UPC/EAN digits.
 */
pub const L_PATTERNS: [[u32; 4]; 10] = [
    [3, 2, 1, 1], // 0
    [2, 2, 2, 1], // 1
    [2, 1, 2, 2], // 2
    [1, 4, 1, 1], // 3
    [1, 1, 3, 2], // 4
    [1, 2, 3, 1], // 5
    [1, 1, 1, 4], // 6
    [1, 3, 1, 2], // 7
    [1, 2, 1, 3], // 8
    [3, 1, 1, 2], // 9
];

/**
 * As above but also including the "even", or "G" patterns used to encode UPC/EAN digits.
 */
pub const L_AND_G_PATTERNS: [[u32; 4]; 20] = {
    let mut new_array = [[0_u32; 4]; 20]; //new int[20][];
    let mut i = 0;
    while i < 10 {
        new_array[i] = L_PATTERNS[i];
        i += 1;
    }
    // new_array[0..10].copy_from_slice(&L_PATTERNS[0..10]);
    // System.arraycopy(L_PATTERNS, 0, L_AND_G_PATTERNS, 0, 10);
    let mut i = 10;
    while i < 20 {
        // for (int i = 10; i < 20; i++) {
        let widths = &L_PATTERNS[i - 10];
        let mut reversedWidths = [0_u32; 4]; //new int[widths.length];
        let mut j = 0;
        while j < 4 {
            // for (int j = 0; j < widths.length; j++) {
            reversedWidths[j] = widths[4 - j - 1];

            j += 1;
        }
        new_array[i] = reversedWidths;

        i += 1;
    }

    new_array
};

/**
 * <p>Encapsulates functionality and implementation that is common to UPC and EAN families
 * of one-dimensional barcodes.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */
pub trait UPCEANReader: OneDReader {
    // private final StringBuilder decodeRowStringBuffer;
    // private final UPCEANExtensionSupport extensionReader;
    // private final EANManufacturerOrgSupport eanManSupport;

    // protected UPCEANReader() {
    //   decodeRowStringBuffer = new StringBuilder(20);
    //   extensionReader = new UPCEANExtensionSupport();
    //   eanManSupport = new EANManufacturerOrgSupport();
    // }

    fn findStartGuardPattern(row: &BitArray) -> Result<[usize; 2], Exceptions>
    where
        Self: Sized,
    {
        let mut foundStart = false;
        let mut startRange = [0; 2]; //= null;
        let mut nextStart = 0;
        let mut counters = [0_u32; 3]; //vec![0_u32;START_END_PATTERN.len()];
        while !foundStart {
            counters.fill(0);
            // Arrays.fill(counters, 0, START_END_PATTERN.len(), 0);
            startRange = Self::findGuardPatternWithCounters(
                row,
                nextStart,
                false,
                &START_END_PATTERN,
                &mut counters,
            )?;
            let start = startRange[0];
            nextStart = startRange[1];
            // Make sure there is a quiet zone at least as big as the start pattern before the barcode.
            // If this check would run off the left edge of the image, do not accept this barcode,
            // as it is very likely to be a false positive.
            let quietStart = start as isize - (nextStart as isize - start as isize);
            if quietStart >= 0 {
                foundStart = row.isRange(quietStart as usize, start, false)?;
            }
        }

        Ok(startRange)
    }

    // @Override
    // public RXingResult decodeRow(int rowNumber, BitArray row, Map<DecodeHintType,?> hints)
    //     throws NotFoundException, ChecksumException, FormatException {
    //   return decodeRow(rowNumber, row, findStartGuardPattern(row), hints);
    // }

    /**
     * <p>Like {@link #decodeRow(int, BitArray, Map)}, but
     * allows caller to inform method about where the UPC/EAN start pattern is
     * found. This allows this to be computed once and reused across many implementations.</p>
     *
     * @param rowNumber row index into the image
     * @param row encoding of the row of the barcode image
     * @param startGuardRange start/end column where the opening start pattern was found
     * @param hints optional hints that influence decoding
     * @return {@link RXingResult} encapsulating the result of decoding a barcode in the row
     * @throws NotFoundException if no potential barcode is found
     * @throws ChecksumException if a potential barcode is found but does not pass its checksum
     * @throws FormatException if a potential barcode is found but format is invalid
     */
    fn decodeRowWithGuardRange(
        &self,
        rowNumber: u32,
        row: &BitArray,
        startGuardRange: &[usize; 2],
        hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult, Exceptions>
    where
        Self: Sized,
    {
        let resultPointCallback = hints.get(&DecodeHintType::NEED_RESULT_POINT_CALLBACK);
        let mut symbologyIdentifier = 0;

        if let Some(DecodeHintValue::NeedResultPointCallback(cb)) = resultPointCallback {
            cb(&RXingResultPoint::new(
                (startGuardRange[0] + startGuardRange[1]) as f32 / 2.0,
                rowNumber as f32,
            ));
        }

        let mut result = String::new(); //decodeRowStringBuffer;
        let endStart = self.decodeMiddle(row, startGuardRange, &mut result)?;

        if let Some(DecodeHintValue::NeedResultPointCallback(cb)) = resultPointCallback {
            cb(&RXingResultPoint::new(endStart as f32, rowNumber as f32));
        }

        let endRange = Self::decodeEnd(row, endStart)?;

        if let Some(DecodeHintValue::NeedResultPointCallback(cb)) = resultPointCallback {
            cb(&RXingResultPoint::new(
                (endRange[0] + endRange[1]) as f32 / 2.0,
                rowNumber as f32,
            ));
        }

        // Make sure there is a quiet zone at least as big as the end pattern after the barcode. The
        // spec might want more whitespace, but in practice this is the maximum we can count on.
        let end = endRange[1];
        let quietEnd = end + (end - endRange[0]);
        if quietEnd >= row.getSize() || !row.isRange(end, quietEnd, false)? {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        let resultString = result;
        // UPC/EAN should never be less than 8 chars anyway
        if resultString.chars().count() < 8 {
            return Err(Exceptions::FormatException("".to_owned()));
        }
        if !self.checkChecksum(&resultString)? {
            return Err(Exceptions::ChecksumException("".to_owned()));
        }

        let left = (startGuardRange[1] + startGuardRange[0]) as f32 / 2.0;
        let right: f32 = (endRange[1] + endRange[0]) as f32 / 2.0;
        let format = self.getBarcodeFormat();
        let mut decodeRXingResult = RXingResult::new(
            &resultString,
            Vec::new(), // no natural byte representation for these barcodes
            vec![
                RXingResultPoint::new(left, rowNumber as f32),
                RXingResultPoint::new(right, rowNumber as f32),
            ],
            format,
        );

        let mut extensionLength = 0;

        let mut attempt = || -> Result<(), Exceptions> {
            let extensionRXingResult =
                UPC_EAN_EXTENSION_SUPPORT.decodeRow(rowNumber, row, endRange[1])?;

            decodeRXingResult.putMetadata(
                RXingResultMetadataType::UPC_EAN_EXTENSION,
                RXingResultMetadataValue::UpcEanExtension(extensionRXingResult.getText().clone()),
            );
            decodeRXingResult.putAllMetadata(extensionRXingResult.getRXingResultMetadata().clone());
            decodeRXingResult
                .addRXingResultPoints(&mut extensionRXingResult.getRXingResultPoints().clone());
            extensionLength = extensionRXingResult.getText().chars().count();
            Ok(())
        };

        let try_result = attempt();
        // if let Err(Exceptions::ReaderException(_)) = try_result {
        // } else if try_result.is_err() {
        //     return Err(try_result.err().unwrap());
        // }

        // try {
        //   RXingResult extensionRXingResult = extensionReader.decodeRow(rowNumber, row, endRange[1]);
        //   decodeRXingResult.putMetadata(RXingResultMetadataType.UPC_EAN_EXTENSION, extensionRXingResult.getText());
        //   decodeRXingResult.putAllMetadata(extensionRXingResult.getRXingResultMetadata());
        //   decodeRXingResult.addRXingResultPoints(extensionRXingResult.getRXingResultPoints());
        //   extensionLength = extensionRXingResult.getText().length();
        // } catch (ReaderException re) {
        //   // continue
        // }

        if let Some(DecodeHintValue::AllowedEanExtensions(allowedExtensions)) =
            hints.get(&DecodeHintType::ALLOWED_EAN_EXTENSIONS)
        {
            let mut valid = false;
            for length in allowedExtensions {
                // for (int length : allowedExtensions) {
                if extensionLength == *length as usize {
                    valid = true;
                    break;
                }
            }
            if !valid {
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
        }
        // let allowedExtensions =
        //     hints == null ? null : (int[]) hints.get(DecodeHintType.ALLOWED_EAN_EXTENSIONS);
        // if (allowedExtensions != null) {
        //   let valid = false;
        //   for (int length : allowedExtensions) {
        //     if (extensionLength == length) {
        //       valid = true;
        //       break;
        //     }
        //   }
        //   if (!valid) {
        //     return Err(Exceptions::NotFoundException("".to_owned()));
        //   }
        // }

        if format == BarcodeFormat::EAN_13 || format == BarcodeFormat::UPC_A {
            let countryID = EAN_MANUFACTURER_SUPPORT.lookupCountryIdentifier(&resultString);
            if let Some(cid) = countryID {
                decodeRXingResult.putMetadata(
                    RXingResultMetadataType::POSSIBLE_COUNTRY,
                    RXingResultMetadataValue::PossibleCountry(cid),
                );
            }
        }
        if format == BarcodeFormat::EAN_8 {
            symbologyIdentifier = 4;
        }

        decodeRXingResult.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier(format!("]E{}", symbologyIdentifier)),
        );

        Ok(decodeRXingResult)
    }

    /**
     * @param s string of digits to check
     * @return {@link #checkStandardUPCEANChecksum(CharSequence)}
     * @throws FormatException if the string does not contain only digits
     */
    fn checkChecksum(&self, s: &str) -> Result<bool, Exceptions> {
        Self::checkStandardUPCEANChecksum(s)
    }

    /**
     * Computes the UPC/EAN checksum on a string of digits, and reports
     * whether the checksum is correct or not.
     *
     * @param s string of digits to check
     * @return true iff string of digits passes the UPC/EAN checksum algorithm
     * @throws FormatException if the string does not contain only digits
     */
    fn checkStandardUPCEANChecksum(s: &str) -> Result<bool, Exceptions> {
        let length = s.len();
        if length == 0 {
            return Ok(false);
        }
        let char_in_question = s.chars().nth(length - 1).unwrap();
        let check = char_in_question.is_digit(10);
        // let check = Character.digit(s.charAt(length - 1), 10);

        let check_against = &s[..length - 1]; //s.subSequence(0, length - 1);
        let calculated_checksum = Self::getStandardUPCEANChecksum(check_against)?;

        Ok(calculated_checksum
            == if check {
                char_in_question.to_digit(10).unwrap()
            } else {
                u32::MAX
            })
    }

    fn getStandardUPCEANChecksum(s: &str) -> Result<u32, Exceptions> {
        let length = s.chars().count();
        let mut sum = 0;
        let mut i = length as isize - 1;
        while i >= 0 {
            // for (int i = length - 1; i >= 0; i -= 2) {
            let digit = (s.chars().nth(i as usize).unwrap() as i32) - ('0' as i32);
            if digit < 0 || digit > 9 {
                return Err(Exceptions::FormatException("".to_owned()));
            }
            sum += digit;

            i -= 2;
        }
        sum *= 3;
        let mut i = length as isize - 2;
        while i >= 0 {
            // for (int i = length - 2; i >= 0; i -= 2) {
            let digit = (s.chars().nth(i as usize).unwrap() as i32) - ('0' as i32);
            if digit < 0 || digit > 9 {
                return Err(Exceptions::FormatException("".to_owned()));
            }
            sum += digit;

            i -= 2;
        }
        Ok(((1000 - sum) % 10) as u32)
    }

    fn decodeEnd(row: &BitArray, endStart: usize) -> Result<[usize; 2], Exceptions>
    where
        Self: Sized,
    {
        Self::findGuardPattern(row, endStart, false, &START_END_PATTERN)
    }

    fn findGuardPattern(
        row: &BitArray,
        rowOffset: usize,
        whiteFirst: bool,
        pattern: &[u32],
    ) -> Result<[usize; 2], Exceptions>
    where
        Self: Sized,
    {
        Self::findGuardPatternWithCounters(
            row,
            rowOffset,
            whiteFirst,
            pattern,
            &mut vec![0u32; pattern.len()],
        )
    }

    /**
     * @param row row of black/white values to search
     * @param rowOffset position to start search
     * @param whiteFirst if true, indicates that the pattern specifies white/black/white/...
     * pixel counts, otherwise, it is interpreted as black/white/black/...
     * @param pattern pattern of counts of number of black and white pixels that are being
     * searched for as a pattern
     * @param counters array of counters, as long as pattern, to re-use
     * @return start/end horizontal offset of guard pattern, as an array of two ints
     * @throws NotFoundException if pattern is not found
     */
    fn findGuardPatternWithCounters(
        row: &BitArray,
        rowOffset: usize,
        whiteFirst: bool,
        pattern: &[u32],
        counters: &mut [u32],
    ) -> Result<[usize; 2], Exceptions>
    where
        Self: Sized,
    {
        let width = row.getSize();
        let rowOffset = if whiteFirst {
            row.getNextUnset(rowOffset)
        } else {
            row.getNextSet(rowOffset)
        };
        let mut counterPosition = 0;
        let mut patternStart = rowOffset;
        let patternLength = pattern.len();
        let mut isWhite = whiteFirst;
        for x in rowOffset..width {
            // for (int x = rowOffset; x < width; x++) {
            if row.get(x) != isWhite {
                counters[counterPosition] += 1;
            } else {
                if counterPosition == patternLength - 1 {
                    if Self::patternMatchVariance(counters, pattern, MAX_INDIVIDUAL_VARIANCE)
                        < MAX_AVG_VARIANCE
                    {
                        return Ok([patternStart, x]);
                    }
                    patternStart += (counters[0] + counters[1]) as usize;
                    let slc = &counters[2..(counterPosition - 1 + 2)].to_vec();
                    counters[..(counterPosition - 1)].copy_from_slice(slc);
                    // System.arraycopy(counters, 2, counters, 0, counterPosition - 1);
                    counters[counterPosition - 1] = 0;
                    counters[counterPosition] = 0;
                    counterPosition -= 1;
                } else {
                    counterPosition += 1;
                }
                counters[counterPosition] = 1;
                isWhite = !isWhite;
            }
        }

        Err(Exceptions::NotFoundException("".to_owned()))
    }

    /**
     * Attempts to decode a single UPC/EAN-encoded digit.
     *
     * @param row row of black/white values to decode
     * @param counters the counts of runs of observed black/white/black/... values
     * @param rowOffset horizontal offset to start decoding from
     * @param patterns the set of patterns to use to decode -- sometimes different encodings
     * for the digits 0-9 are used, and this indicates the encodings for 0 to 9 that should
     * be used
     * @return horizontal offset of first pixel beyond the decoded digit
     * @throws NotFoundException if digit cannot be decoded
     */
    fn decodeDigit(
        row: &BitArray,
        counters: &mut [u32; 4],
        rowOffset: usize,
        patterns: &[[u32; 4]],
    ) -> Result<usize, Exceptions>
    where
        Self: Sized,
    {
        Self::recordPattern(row, rowOffset, counters)?;
        let mut bestVariance = MAX_AVG_VARIANCE; // worst variance we'll accept
        let mut bestMatch = -1_isize;
        let max = patterns.len();
        for i in 0..max {
            // for (int i = 0; i < max; i++) {
            let pattern = &patterns[i];
            let variance: f32 =
                Self::patternMatchVariance(counters, pattern, MAX_INDIVIDUAL_VARIANCE);
            if variance < bestVariance {
                bestVariance = variance;
                bestMatch = i as isize;
            }
        }
        if bestMatch >= 0 {
            Ok(bestMatch as usize)
        } else {
            Err(Exceptions::NotFoundException("".to_owned()))
        }
    }

    /**
     * Get the format of this decoder.
     *
     * @return The 1D format.
     */
    fn getBarcodeFormat(&self) -> BarcodeFormat;

    /**
     * Subclasses override this to decode the portion of a barcode between the start
     * and end guard patterns.
     *
     * @param row row of black/white values to search
     * @param startRange start/end offset of start guard pattern
     * @param resultString {@link StringBuilder} to append decoded chars to
     * @return horizontal offset of first pixel after the "middle" that was decoded
     * @throws NotFoundException if decoding could not complete successfully
     */
    fn decodeMiddle(
        &self,
        row: &BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<usize, Exceptions>;
}

pub(crate) struct StandIn;
impl UPCEANReader for StandIn {
    fn getBarcodeFormat(&self) -> BarcodeFormat {
        todo!()
    }

    fn decodeMiddle(
        &self,
        _row: &BitArray,
        _startRange: &[usize; 2],
        _resultString: &mut String,
    ) -> Result<usize, Exceptions> {
        todo!()
    }
}
impl OneDReader for StandIn {
    fn decodeRow(
        &mut self,
        _rowNumber: u32,
        _row: &BitArray,
        _hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult, Exceptions> {
        todo!()
    }
}

impl Reader for StandIn {
    fn decode(&mut self, _image: &crate::BinaryBitmap) -> Result<RXingResult, Exceptions> {
        todo!()
    }

    fn decode_with_hints(
        &mut self,
        _image: &crate::BinaryBitmap,
        _hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult, Exceptions> {
        todo!()
    }
}
