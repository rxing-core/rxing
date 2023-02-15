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

use rxing_one_d_proc_derive::OneDReader;

use crate::{common::BitArray, BarcodeFormat, DecodeHintValue, Exceptions, RXingResult};

use super::{one_d_reader, OneDReader};

const MAX_AVG_VARIANCE: f32 = 0.38;
const MAX_INDIVIDUAL_VARIANCE: f32 = 0.5;

const W: u32 = 3; // Pixel width of a 3x wide line
const W_LOWER: u32 = 2; // Pixel width of a 2x wide line
const N: u32 = 1; // Pixed width of a narrow line

/** Valid ITF lengths. Anything longer than the largest value is also allowed. */
const DEFAULT_ALLOWED_LENGTHS: [u32; 5] = [6, 8, 10, 12, 14];

/**
 * Start/end guard pattern.
 *
 * Note: The end pattern is reversed because the row is reversed before
 * searching for the END_PATTERN
 */
const START_PATTERN: [u32; 4] = [N, N, N, N];
const END_PATTERN_REVERSED: [[u32; 3]; 2] = [
    [N, N, W_LOWER], // 2x
    [N, N, W],       // 3x
];

// See ITFWriter.PATTERNS

/**
 * Patterns of Wide / Narrow lines to indicate each digit
 */
const PATTERNS: [[u32; 5]; 20] = [
    [N, N, W_LOWER, W_LOWER, N], // 0
    [W_LOWER, N, N, N, W_LOWER], // 1
    [N, W_LOWER, N, N, W_LOWER], // 2
    [W_LOWER, W_LOWER, N, N, N], // 3
    [N, N, W_LOWER, N, W_LOWER], // 4
    [W_LOWER, N, W_LOWER, N, N], // 5
    [N, W_LOWER, W_LOWER, N, N], // 6
    [N, N, N, W_LOWER, W_LOWER], // 7
    [W_LOWER, N, N, W_LOWER, N], // 8
    [N, W_LOWER, N, W_LOWER, N], // 9
    [N, N, W, W, N],             // 0
    [W, N, N, N, W],             // 1
    [N, W, N, N, W],             // 2
    [W, W, N, N, N],             // 3
    [N, N, W, N, W],             // 4
    [W, N, W, N, N],             // 5
    [N, W, W, N, N],             // 6
    [N, N, N, W, W],             // 7
    [W, N, N, W, N],             // 8
    [N, W, N, W, N],             // 9
];

/**
 * <p>Implements decoding of the ITF format, or Interleaved Two of Five.</p>
 *
 * <p>This Reader will scan ITF barcodes of certain lengths only.
 * At the moment it reads length 6, 8, 10, 12, 14, 16, 18, 20, 24, and 44 as these have appeared "in the wild". Not all
 * lengths are scanned, especially shorter ones, to avoid false positives. This in turn is due to a lack of
 * required checksum function.</p>
 *
 * <p>The checksum is optional and is not applied by this Reader. The consumer of the decoded
 * value will have to apply a checksum if required.</p>
 *
 * <p><a href="http://en.wikipedia.org/wiki/Interleaved_2_of_5">http://en.wikipedia.org/wiki/Interleaved_2_of_5</a>
 * is a great reference for Interleaved 2 of 5 information.</p>
 *
 * @author kevin.osullivan@sita.aero, SITA Lab.
 */
#[derive(OneDReader)]
pub struct ITFReader {
    // Stores the actual narrow line width of the image being decoded.
    narrowLineWidth: i32,
}

impl Default for ITFReader {
    fn default() -> Self {
        Self {
            narrowLineWidth: -1,
        }
    }
}

impl OneDReader for ITFReader {
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        // Find out where the Middle section (payload) starts & ends
        let mut row = row.clone();
        let startRange = self.decodeStart(&row)?;
        let endRange = self.decodeEnd(&mut row)?;

        let mut result = String::with_capacity(20); //new StringBuilder(20);
        self.decodeMiddle(&row, startRange[1], endRange[0], &mut result)?;
        let resultString = result; //.toString();

        let allowedLengths = if let Some(DecodeHintValue::AllowedLengths(al)) =
            hints.get(&DecodeHintType::ALLOWED_LENGTHS)
        {
            al.clone()
        } else {
            DEFAULT_ALLOWED_LENGTHS.to_vec()
        };

        // To avoid false positives with 2D barcodes (and other patterns), make
        // an assumption that the decoded string must be a 'standard' length if it's short
        let length = resultString.chars().count();
        let mut lengthOK = false;
        let mut maxAllowedLength = 0;
        for allowedLength in allowedLengths {
            if length == allowedLength as usize {
                lengthOK = true;
                break;
            }
            maxAllowedLength = std::cmp::max(allowedLength, maxAllowedLength);
        }
        if !lengthOK && length > maxAllowedLength as usize {
            lengthOK = true;
        }
        if !lengthOK {
            return Err(Exceptions::format);
        }

        let mut resultObject = RXingResult::new(
            &resultString,
            Vec::new(), // no natural byte representation for these barcodes
            vec![
                RXingResultPoint::new(startRange[1] as f32, rowNumber as f32),
                RXingResultPoint::new(endRange[0] as f32, rowNumber as f32),
            ],
            BarcodeFormat::ITF,
        );

        resultObject.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier("]I0".to_owned()),
        );

        Ok(resultObject)
    }
}
impl ITFReader {
    /**
     * @param row          row of black/white values to search
     * @param payloadStart offset of start pattern
     * @param resultString {@link StringBuilder} to append decoded chars to
     * @throws NotFoundException if decoding could not complete successfully
     */
    fn decodeMiddle(
        &self,
        row: &BitArray,
        payloadStart: usize,
        payloadEnd: usize,
        resultString: &mut String,
    ) -> Result<(), Exceptions> {
        let mut payloadStart = payloadStart;
        // Digits are interleaved in pairs - 5 black lines for one digit, and the
        // 5 interleaved white lines for the second digit.
        // Therefore, need to scan 10 lines and then
        // split these into two arrays
        let mut counterDigitPair = [0_u32; 10]; //new int[10];
        let mut counterBlack = [0_u32; 5]; //new int[5];
        let mut counterWhite = [0_u32; 5]; //new int[5];

        while payloadStart < payloadEnd {
            // Get 10 runs of black/white.
            one_d_reader::recordPattern(row, payloadStart, &mut counterDigitPair)?;
            // Split them into each array
            for k in 0..5 {
                let twoK = 2 * k;
                counterBlack[k] = counterDigitPair[twoK];
                counterWhite[k] = counterDigitPair[twoK + 1];
            }

            let mut bestMatch = self.decodeDigit(&counterBlack)?;
            resultString.push(char::from_u32('0' as u32 + bestMatch).ok_or(Exceptions::parse)?);
            bestMatch = self.decodeDigit(&counterWhite)?;
            resultString.push(char::from_u32('0' as u32 + bestMatch).ok_or(Exceptions::parse)?);

            payloadStart += counterDigitPair.iter().sum::<u32>() as usize;
        }

        Ok(())
    }

    /**
     * Identify where the start of the middle / payload section starts.
     *
     * @param row row of black/white values to search
     * @return Array, containing index of start of 'start block' and end of
     *         'start block'
     */
    fn decodeStart(&mut self, row: &BitArray) -> Result<[usize; 2], Exceptions> {
        let endStart = Self::skipWhiteSpace(row)?;
        let startPattern = self.findGuardPattern(row, endStart, &START_PATTERN)?;

        // Determine the width of a narrow line in pixels. We can do this by
        // getting the width of the start pattern and dividing by 4 because its
        // made up of 4 narrow lines.
        self.narrowLineWidth = (startPattern[1] - startPattern[0]) as i32 / 4;

        self.validateQuietZone(row, startPattern[0])?;

        Ok(startPattern)
    }

    /**
     * The start & end patterns must be pre/post fixed by a quiet zone. This
     * zone must be at least 10 times the width of a narrow line.  Scan back until
     * we either get to the start of the barcode or match the necessary number of
     * quiet zone pixels.
     *
     * Note: Its assumed the row is reversed when using this method to find
     * quiet zone after the end pattern.
     *
     * ref: http://www.barcode-1.net/i25code.html
     *
     * @param row bit array representing the scanned barcode.
     * @param startPattern index into row of the start or end pattern.
     * @throws NotFoundException if the quiet zone cannot be found
     */
    fn validateQuietZone(&self, row: &BitArray, startPattern: usize) -> Result<(), Exceptions> {
        let mut quietCount = self.narrowLineWidth * 10; // expect to find this many pixels of quiet zone

        // if there are not so many pixel at all let's try as many as possible
        quietCount = quietCount.min(startPattern as i32);

        let mut i = startPattern as isize - 1;
        while quietCount > 0 && i >= 0 {
            if row.get(i as usize) {
                break;
            }
            quietCount -= 1;
            i -= 1;
        }

        if quietCount != 0 {
            // Unable to find the necessary number of quiet zone pixels.
            Err(Exceptions::notFound)
        } else {
            Ok(())
        }
    }

    /**
     * Skip all whitespace until we get to the first black line.
     *
     * @param row row of black/white values to search
     * @return index of the first black line.
     * @throws NotFoundException Throws exception if no black lines are found in the row
     */
    fn skipWhiteSpace(row: &BitArray) -> Result<usize, Exceptions> {
        let width = row.getSize();
        let endStart = row.getNextSet(0);
        if endStart == width {
            return Err(Exceptions::notFound);
        }

        Ok(endStart)
    }

    /**
     * Identify where the end of the middle / payload section ends.
     *
     * @param row row of black/white values to search
     * @return Array, containing index of start of 'end block' and end of 'end
     *         block'
     */
    fn decodeEnd(&self, row: &mut BitArray) -> Result<[usize; 2], Exceptions> {
        // For convenience, reverse the row and then
        // search from 'the start' for the end block
        row.reverse();
        let interim_function = || -> Result<[usize; 2], Exceptions> {
            let endStart = Self::skipWhiteSpace(row)?;
            let mut endPattern =
                if let Ok(ptrn) = self.findGuardPattern(row, endStart, &END_PATTERN_REVERSED[0]) {
                    ptrn
                } else {
                    self.findGuardPattern(row, endStart, &END_PATTERN_REVERSED[1])?
                };

            // The start & end patterns must be pre/post fixed by a quiet zone. This
            // zone must be at least 10 times the width of a narrow line.
            // ref: http://www.barcode-1.net/i25code.html
            self.validateQuietZone(row, endPattern[0])?;

            // Now recalculate the indices of where the 'endblock' starts & stops to
            // accommodate the reversed nature of the search
            let temp = endPattern[0];
            endPattern[0] = row.getSize() - endPattern[1];
            endPattern[1] = row.getSize() - temp;

            Ok(endPattern)
        };
        let res = interim_function();
        // Put the row back the right way.
        row.reverse();

        res
    }

    /**
     * @param row       row of black/white values to search
     * @param rowOffset position to start search
     * @param pattern   pattern of counts of number of black and white pixels that are
     *                  being searched for as a pattern
     * @return start/end horizontal offset of guard pattern, as an array of two
     *         ints
     * @throws NotFoundException if pattern is not found
     */
    fn findGuardPattern(
        &self,
        row: &BitArray,
        rowOffset: usize,
        pattern: &[u32],
    ) -> Result<[usize; 2], Exceptions> {
        let patternLength = pattern.len();
        let mut counters = vec![0u32; patternLength]; //new int[patternLength];
        let width = row.getSize();
        let mut isWhite = false;

        let mut counterPosition = 0;
        let mut patternStart = rowOffset;
        for x in rowOffset..width {
            // for (int x = rowOffset; x < width; x++) {
            if row.get(x) != isWhite {
                counters[counterPosition] += 1;
            } else {
                if counterPosition == patternLength - 1 {
                    if one_d_reader::patternMatchVariance(
                        &counters,
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
        Err(Exceptions::notFound)
    }

    /**
     * Attempts to decode a sequence of ITF black/white lines into single
     * digit.
     *
     * @param counters the counts of runs of observed black/white/black/... values
     * @return The decoded digit
     * @throws NotFoundException if digit cannot be decoded
     */
    fn decodeDigit(&self, counters: &[u32]) -> Result<u32, Exceptions> {
        let mut bestVariance = MAX_AVG_VARIANCE; // worst variance we'll accept
        let mut bestMatch = -1_isize;
        let max = PATTERNS.len();
        for (i, pattern) in PATTERNS.iter().enumerate().take(max) {
            let variance =
                one_d_reader::patternMatchVariance(counters, pattern, MAX_INDIVIDUAL_VARIANCE);
            if variance < bestVariance {
                bestVariance = variance;
                bestMatch = i as isize;
            } else if variance == bestVariance {
                // if we find a second 'best match' with the same variance, we can not reliably report to have a suitable match
                bestMatch = -1;
            }
        }
        if bestMatch >= 0 {
            Ok(bestMatch as u32 % 10)
        } else {
            Err(Exceptions::notFound)
        }
    }
}
