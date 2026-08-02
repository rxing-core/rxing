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
    BarcodeFormat, Binarizer, Error, RXingResult, RXingResultMetadataType,
    RXingResultMetadataValue, Reader,
    common::{BitArray, Result},
    oned::{
        oned_constants::upc_ean_shared::START_END_PATTERN,
        upcean_common::checkStandardUPCEANChecksum,
    },
    point,
};

use super::{EANManufacturerOrgSupport, OneDReader, UPCEANExtensionSupport, one_d_reader};

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
 * <p>Encapsulates functionality and implementation that is common to UPC and EAN families
 * of one-dimensional barcodes.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */
pub trait UPCEANReader: OneDReader {
    fn find_start_guard_pattern(&self, row: &BitArray) -> Result<[usize; 2]> {
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
        hints: &crate::DecodeHints,
    ) -> Result<RXingResult> {
        let resultPointCallback = &hints.NeedResultPointCallback;
        let mut symbologyIdentifier = 0;

        if let Some(cb) = resultPointCallback {
            cb(point(
                (startGuardRange[0] + startGuardRange[1]) as f32 / 2.0,
                rowNumber as f32,
            ));
        }

        let mut result = String::new();
        let endStart = self.decodeMiddle(row, startGuardRange, &mut result)?;

        if let Some(cb) = resultPointCallback {
            cb(point(endStart as f32, rowNumber as f32));
        }

        let endRange = self.decodeEnd(row, endStart)?;

        if let Some(cb) = resultPointCallback {
            cb(point(
                (endRange[0] + endRange[1]) as f32 / 2.0,
                rowNumber as f32,
            ));
        }

        // Make sure there is a quiet zone at least as big as the end pattern after the barcode. The
        // spec might want more whitespace, but in practice this is the maximum we can count on.
        let end = endRange[1];
        let quietEnd = end + (end - endRange[0]);
        if quietEnd >= row.get_size() || !row.isRange(end, quietEnd, false)? {
            return Err(Error::NOT_FOUND);
        }

        let resultString = result;

        // UPC/EAN should never be less than 8 chars anyway
        if resultString.chars().count() < 8 {
            return Err(Error::Format {
                message: "UPC/EAN should never be less than 8 chars anyway".into(),
                source: None,
            });
        }

        if !self.checkChecksum(&resultString)? {
            return Err(Error::CHECKSUM);
        }

        let left = (startGuardRange[1] + startGuardRange[0]) as f32 / 2.0;
        let right: f32 = (endRange[1] + endRange[0]) as f32 / 2.0;
        let format = self.getBarcodeFormat();
        let mut decodeRXingResult = RXingResult::new(
            &resultString,
            Vec::new(), // no natural byte representation for these barcodes
            vec![
                point(left, rowNumber as f32),
                point(right, rowNumber as f32),
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
            decodeRXingResult.addPoints(&mut extensionRXingResult.getPoints().to_vec());
            extensionLength = extensionRXingResult.getText().chars().count();
            Ok(())
        };

        let _try_result = attempt();

        if let Some(allowedExtensions) = &hints.AllowedEanExtensions {
            let mut valid = false;
            for length in allowedExtensions {
                if extensionLength == *length as usize {
                    valid = true;
                    break;
                }
            }
            if !valid {
                return Err(Error::NOT_FOUND);
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
        checkStandardUPCEANChecksum(s)
    }

    fn decodeEnd(&self, row: &BitArray, endStart: usize) -> Result<[usize; 2]> {
        self.findGuardPattern(row, endStart, false, &START_END_PATTERN)
    }

    fn findGuardPattern<const N: usize>(
        &self,
        row: &BitArray,
        rowOffset: usize,
        whiteFirst: bool,
        pattern: &[u32; N],
    ) -> Result<[usize; 2]> {
        self.findGuardPatternWithCounters(row, rowOffset, whiteFirst, pattern, &mut [0u32; N])
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
    fn findGuardPatternWithCounters<const N: usize>(
        &self,
        row: &BitArray,
        rowOffset: usize,
        whiteFirst: bool,
        pattern: &[u32; N],
        counters: &mut [u32; N],
    ) -> Result<[usize; 2]> {
        let width = row.get_size();
        let rowOffset = if whiteFirst {
            row.getNextUnset(rowOffset)
        } else {
            row.getNextSet(rowOffset)
        };
        let mut counterPosition = 0;
        let mut patternStart = rowOffset;
        let patternLength = N;
        let mut isWhite = whiteFirst;
        for x in rowOffset..width {
            // for (int x = rowOffset; x < width; x++) {
            if row.get(x) != isWhite {
                counters[counterPosition] += 1;
            } else {
                if counterPosition == patternLength - 1 {
                    if one_d_reader::pattern_match_variance(
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

        Err(Error::NOT_FOUND)
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
        one_d_reader::record_pattern(row, rowOffset, counters)?;
        let mut bestVariance = MAX_AVG_VARIANCE; // worst variance we'll accept
        let mut bestMatch = -1_isize;
        let max = patterns.len();
        for (i, pattern) in patterns.iter().enumerate().take(max) {
            let variance: f32 =
                one_d_reader::pattern_match_variance(counters, pattern, MAX_INDIVIDUAL_VARIANCE);
            if variance < bestVariance {
                bestVariance = variance;
                bestMatch = i as isize;
            }
        }
        if bestMatch >= 0 {
            Ok(bestMatch as usize)
        } else {
            Err(Error::NOT_FOUND)
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
        unimplemented!()
    }

    fn decodeMiddle(
        &self,
        _row: &BitArray,
        _startRange: &[usize; 2],
        _resultString: &mut String,
    ) -> Result<usize> {
        unimplemented!()
    }
}
impl OneDReader for StandInStruct {
    fn decode_row(
        &mut self,
        _rowNumber: u32,
        _row: &BitArray,
        _hints: &crate::DecodeHints,
    ) -> Result<RXingResult> {
        unimplemented!()
    }
}

impl Reader for StandInStruct {
    fn decode<B: Binarizer>(&mut self, _image: &mut crate::BinaryBitmap<B>) -> Result<RXingResult> {
        unimplemented!()
    }

    fn decode_with_hints<B: Binarizer>(
        &mut self,
        _image: &mut crate::BinaryBitmap<B>,
        _hints: &crate::DecodeHints,
    ) -> Result<RXingResult> {
        unimplemented!()
    }
}

pub(crate) const STAND_IN: StandInStruct = StandInStruct {};
