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
    common::{BitArray, Result},
    BarcodeFormat, DecodeHintType, DecodeHintValue, Exceptions, RXingResult,
    RXingResultMetadataType, RXingResultMetadataValue, Point, Reader,
};

use super::{one_d_reader, EANManufacturerOrgSupport, OneDReader, UPCEANExtensionSupport};

use once_cell::sync::Lazy;

pub static EAN_MANUFACTURER_SUPPORT: Lazy<EANManufacturerOrgSupport> =
    Lazy::new(EANManufacturerOrgSupport::default);
pub static UPC_EAN_EXTENSION_SUPPORT: Lazy<UPCEANExtensionSupport> =
    Lazy::new(UPCEANExtensionSupport::default);

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
    let mut new_array = [[0_u32; 4]; 20];
    let mut i = 0;
    while i < 10 {
        new_array[i] = L_PATTERNS[i];
        i += 1;
    }
    let mut i = 10;
    while i < 20 {
        let widths = &L_PATTERNS[i - 10];
        let mut reversedWidths = [0_u32; 4];
        let mut j = 0;
        while j < 4 {
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
    fn findStartGuardPattern(&self, row: &BitArray) -> Result<[usize; 2]> {
        let mut foundStart = false;
        let mut startRange = [0; 2];
        let mut nextStart = 0;
        let mut counters = [0_u32; 3];
        while !foundStart {
            counters.fill(0);

            startRange = self.findGuardPatternWithCounters(
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
    ) -> Result<RXingResult> {
        let resultPointCallback = hints.get(&DecodeHintType::NEED_RESULT_POINT_CALLBACK);
        let mut symbologyIdentifier = 0;

        if let Some(DecodeHintValue::NeedResultPointCallback(cb)) = resultPointCallback {
            cb(&Point::new(
                (startGuardRange[0] + startGuardRange[1]) as f32 / 2.0,
                rowNumber as f32,
            ));
        }

        let mut result = String::new();
        let endStart = self.decodeMiddle(row, startGuardRange, &mut result)?;

        if let Some(DecodeHintValue::NeedResultPointCallback(cb)) = resultPointCallback {
            cb(&Point::new(endStart as f32, rowNumber as f32));
        }

        let endRange = self.decodeEnd(row, endStart)?;

        if let Some(DecodeHintValue::NeedResultPointCallback(cb)) = resultPointCallback {
            cb(&Point::new(
                (endRange[0] + endRange[1]) as f32 / 2.0,
                rowNumber as f32,
            ));
        }

        // Make sure there is a quiet zone at least as big as the end pattern after the barcode. The
        // spec might want more whitespace, but in practice this is the maximum we can count on.
        let end = endRange[1];
        let quietEnd = end + (end - endRange[0]);
        if quietEnd >= row.getSize() || !row.isRange(end, quietEnd, false)? {
            return Err(Exceptions::NotFoundException(None));
        }

        let resultString = result;

        // UPC/EAN should never be less than 8 chars anyway
        if resultString.chars().count() < 8 {
            return Err(Exceptions::FormatException(None));
        }

        if !self.checkChecksum(&resultString)? {
            return Err(Exceptions::ChecksumException(None));
        }

        let left = (startGuardRange[1] + startGuardRange[0]) as f32 / 2.0;
        let right: f32 = (endRange[1] + endRange[0]) as f32 / 2.0;
        let format = self.getBarcodeFormat();
        let mut decodeRXingResult = RXingResult::new(
            &resultString,
            Vec::new(), // no natural byte representation for these barcodes
            vec![
                Point::new(left, rowNumber as f32),
                Point::new(right, rowNumber as f32),
            ],
            format,
        );

        let mut extensionLength = 0;

        let mut attempt = || -> Result<()> {
            let extensionRXingResult =
                UPC_EAN_EXTENSION_SUPPORT.decodeRow(rowNumber, row, endRange[1])?;

            decodeRXingResult.putMetadata(
                RXingResultMetadataType::UPC_EAN_EXTENSION,
                RXingResultMetadataValue::UpcEanExtension(
                    extensionRXingResult.getText().to_owned(),
                ),
            );
            decodeRXingResult.putAllMetadata(extensionRXingResult.getRXingResultMetadata().clone());
            decodeRXingResult
                .addPoints(&mut extensionRXingResult.getPoints().clone());
            extensionLength = extensionRXingResult.getText().chars().count();
            Ok(())
        };

        let _try_result = attempt();

        if let Some(DecodeHintValue::AllowedEanExtensions(allowedExtensions)) =
            hints.get(&DecodeHintType::ALLOWED_EAN_EXTENSIONS)
        {
            let mut valid = false;
            for length in allowedExtensions {
                if extensionLength == *length as usize {
                    valid = true;
                    break;
                }
            }
            if !valid {
                return Err(Exceptions::NotFoundException(None));
            }
        }

        if format == BarcodeFormat::EAN_13 || format == BarcodeFormat::UPC_A {
            let countryID = EAN_MANUFACTURER_SUPPORT.lookupCountryIdentifier(&resultString);
            if let Some(cid) = countryID {
                decodeRXingResult.putMetadata(
                    RXingResultMetadataType::POSSIBLE_COUNTRY,
                    RXingResultMetadataValue::PossibleCountry(cid.to_owned()),
                );
            }
        }

        if format == BarcodeFormat::EAN_8 {
            symbologyIdentifier = 4;
        }

        decodeRXingResult.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier(format!("]E{symbologyIdentifier}")),
        );

        Ok(decodeRXingResult)
    }

    /**
     * @param s string of digits to check
     * @return {@link #checkStandardUPCEANChecksum(CharSequence)}
     * @throws FormatException if the string does not contain only digits
     */
    fn checkChecksum(&self, s: &str) -> Result<bool> {
        self.checkStandardUPCEANChecksum(s)
    }

    /**
     * Computes the UPC/EAN checksum on a string of digits, and reports
     * whether the checksum is correct or not.
     *
     * @param s string of digits to check
     * @return true iff string of digits passes the UPC/EAN checksum algorithm
     * @throws FormatException if the string does not contain only digits
     */
    fn checkStandardUPCEANChecksum(&self, s: &str) -> Result<bool> {
        let length = s.len();
        if length == 0 {
            return Ok(false);
        }
        let char_in_question = s
            .chars()
            .nth(length - 1)
            .ok_or(Exceptions::IndexOutOfBoundsException(None))?;
        let check = char_in_question.is_ascii_digit();

        let check_against = &s[..length - 1]; //s.subSequence(0, length - 1);
        let calculated_checksum = self.getStandardUPCEANChecksum(check_against)?;

        Ok(calculated_checksum
            == if check {
                char_in_question
                    .to_digit(10)
                    .ok_or(Exceptions::ParseException(None))?
            } else {
                u32::MAX
            })
    }

    fn getStandardUPCEANChecksum(&self, s: &str) -> Result<u32> {
        let length = s.chars().count();
        let mut sum = 0;
        let mut i = length as isize - 1;
        while i >= 0 {
            // for (int i = length - 1; i >= 0; i -= 2) {
            let digit =
                (s.chars()
                    .nth(i as usize)
                    .ok_or(Exceptions::IndexOutOfBoundsException(None))? as i32)
                    - ('0' as i32);
            if !(0..=9).contains(&digit) {
                return Err(Exceptions::FormatException(None));
            }
            sum += digit;

            i -= 2;
        }
        sum *= 3;
        let mut i = length as isize - 2;
        while i >= 0 {
            // for (int i = length - 2; i >= 0; i -= 2) {
            let digit =
                (s.chars()
                    .nth(i as usize)
                    .ok_or(Exceptions::IndexOutOfBoundsException(None))? as i32)
                    - ('0' as i32);
            if !(0..=9).contains(&digit) {
                return Err(Exceptions::FormatException(None));
            }
            sum += digit;

            i -= 2;
        }
        Ok(((1000 - sum) % 10) as u32)
    }

    fn decodeEnd(&self, row: &BitArray, endStart: usize) -> Result<[usize; 2]> {
        self.findGuardPattern(row, endStart, false, &START_END_PATTERN)
    }

    fn findGuardPattern(
        &self,
        row: &BitArray,
        rowOffset: usize,
        whiteFirst: bool,
        pattern: &[u32],
    ) -> Result<[usize; 2]> {
        self.findGuardPatternWithCounters(
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
        &self,
        row: &BitArray,
        rowOffset: usize,
        whiteFirst: bool,
        pattern: &[u32],
        counters: &mut [u32],
    ) -> Result<[usize; 2]> {
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
                    if one_d_reader::patternMatchVariance(
                        counters,
                        pattern,
                        MAX_INDIVIDUAL_VARIANCE,
                    ) < MAX_AVG_VARIANCE
                    {
                        return Ok([patternStart, x]);
                    }
                    patternStart += (counters[0] + counters[1]) as usize;

                    counters.copy_within(2..(counterPosition - 1 + 2), 0);

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

        Err(Exceptions::NotFoundException(None))
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
        &self,
        row: &BitArray,
        counters: &mut [u32; 4],
        rowOffset: usize,
        patterns: &[[u32; 4]],
    ) -> Result<usize> {
        one_d_reader::recordPattern(row, rowOffset, counters)?;
        let mut bestVariance = MAX_AVG_VARIANCE; // worst variance we'll accept
        let mut bestMatch = -1_isize;
        let max = patterns.len();
        for (i, pattern) in patterns.iter().enumerate().take(max) {
            let variance: f32 =
                one_d_reader::patternMatchVariance(counters, pattern, MAX_INDIVIDUAL_VARIANCE);
            if variance < bestVariance {
                bestVariance = variance;
                bestMatch = i as isize;
            }
        }
        if bestMatch >= 0 {
            Ok(bestMatch as usize)
        } else {
            Err(Exceptions::NotFoundException(None))
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
    ) -> Result<usize>;
}

pub(crate) struct StandInStruct;
impl UPCEANReader for StandInStruct {
    fn getBarcodeFormat(&self) -> BarcodeFormat {
        todo!()
    }

    fn decodeMiddle(
        &self,
        _row: &BitArray,
        _startRange: &[usize; 2],
        _resultString: &mut String,
    ) -> Result<usize> {
        todo!()
    }
}
impl OneDReader for StandInStruct {
    fn decodeRow(
        &mut self,
        _rowNumber: u32,
        _row: &BitArray,
        _hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult> {
        todo!()
    }
}

impl Reader for StandInStruct {
    fn decode(&mut self, _image: &mut crate::BinaryBitmap) -> Result<RXingResult> {
        todo!()
    }

    fn decode_with_hints(
        &mut self,
        _image: &mut crate::BinaryBitmap,
        _hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult> {
        todo!()
    }
}

pub(crate) const STAND_IN: StandInStruct = StandInStruct {};
