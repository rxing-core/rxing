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

use once_cell::sync::Lazy;
use rxing_one_d_proc_derive::OneDReader;

use crate::common::{BitArray, Result};
use crate::{point_f, BarcodeFormat, Exceptions, RXingResult};

use super::{one_d_reader, OneDReader};

use crate::DecodeHints;

use crate::{RXingResultMetadataType, RXingResultMetadataValue};

/**
 * <p>Decodes Code 39 barcodes. Supports "Full ASCII Code 39" if USE_CODE_39_EXTENDED_MODE is set.</p>
 *
 * @author Sean Owen
 * @see Code93Reader
 */
#[derive(OneDReader)]
pub struct Code39Reader {
    usingCheckDigit: bool,
    extendedMode: bool,
    decodeRowRXingResult: String,
    // counters: Vec<u32>,
}
impl Default for Code39Reader {
    fn default() -> Self {
        Self::with_use_check_digit(false)
    }
}
impl OneDReader for Code39Reader {
    fn decode_row(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        _hints: &DecodeHints,
    ) -> Result<crate::RXingResult> {
        let mut counters = [0_u32; 9];
        self.decodeRowRXingResult.clear();

        let start = Self::findAsteriskPattern(row, &mut counters)?;
        // Read off white space
        let mut nextStart = row.getNextSet(start[1] as usize);
        let end = row.get_size();

        let mut decodedChar;
        let mut lastStart;
        loop {
            one_d_reader::record_pattern(row, nextStart, &mut counters)?;
            let pattern = Self::toNarrowWidePattern(&counters);
            if pattern < 0 {
                return Err(Exceptions::NOT_FOUND);
            }
            decodedChar = Self::patternToChar(pattern as u32)?;
            self.decodeRowRXingResult.push(decodedChar);
            lastStart = nextStart;
            for counter in &counters {
                nextStart += *counter as usize;
            }
            // Read off white space
            nextStart = row.getNextSet(nextStart);

            if decodedChar == '*' {
                break;
            }
        }
        self.decodeRowRXingResult
            .truncate(self.decodeRowRXingResult.len() - 1); // remove asterisk

        // Look for whitespace after pattern:
        let lastPatternSize = counters.iter().sum::<u32>();

        let whiteSpaceAfterEnd = nextStart - lastStart - lastPatternSize as usize;
        // If 50% of last pattern size, following last pattern, is not whitespace, fail
        // (but if it's whitespace to the very end of the image, that's OK)
        if nextStart != end && (whiteSpaceAfterEnd * 2) < lastPatternSize as usize {
            return Err(Exceptions::NOT_FOUND);
        }

        let cached_row_result = self.decodeRowRXingResult.chars().collect::<Vec<_>>();

        if self.usingCheckDigit {
            let max = cached_row_result.len() - 1;
            let mut total = 0;
            for i in 0..max {
                if let Some(pos) = Self::ALPHABET_STRING.find(
                    *cached_row_result
                        .get(i)
                        .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                ) {
                    total += pos;
                }
            }
            if cached_row_result
                .get(max)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                != C39R_CACHED_ALPHABET_STRING
                    .get(total % 43)
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
            {
                return Err(Exceptions::NOT_FOUND);
            }
            self.decodeRowRXingResult.truncate(max);
        }

        if self.decodeRowRXingResult.chars().count() == 0 {
            // false positive
            return Err(Exceptions::NOT_FOUND);
        }

        let resultString = if self.extendedMode {
            Self::decodeExtended(&self.decodeRowRXingResult)?
        } else {
            self.decodeRowRXingResult.clone()
        };

        let left = (start[1] + start[0]) as f32 / 2.0;
        let right = (lastStart + lastPatternSize as usize) as f32 / 2.0;

        let mut resultObject = RXingResult::new(
            &resultString,
            Vec::new(),
            vec![
                point_f(left, rowNumber as f32),
                point_f(right, rowNumber as f32),
            ],
            BarcodeFormat::CODE_39,
        );

        resultObject.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier("]A0".to_owned()),
        );

        Ok(resultObject)
    }
}

pub static C39R_CACHED_ALPHABET_STRING: Lazy<Vec<char>> =
    Lazy::new(|| Code39Reader::ALPHABET_STRING.chars().collect());

impl Code39Reader {
    pub const ALPHABET_STRING: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";

    /**
     * These represent the encodings of characters, as patterns of wide and narrow bars.
     * The 9 least-significant bits of each int correspond to the pattern of wide and narrow,
     * with 1s representing "wide" and 0s representing narrow.
     */
    pub const CHARACTER_ENCODINGS: [u32; 43] = [
        0x034, 0x121, 0x061, 0x160, 0x031, 0x130, 0x070, 0x025, 0x124, 0x064, // 0-9
        0x109, 0x049, 0x148, 0x019, 0x118, 0x058, 0x00D, 0x10C, 0x04C, 0x01C, // A-J
        0x103, 0x043, 0x142, 0x013, 0x112, 0x052, 0x007, 0x106, 0x046, 0x016, // K-T
        0x181, 0x0C1, 0x1C0, 0x091, 0x190, 0x0D0, 0x085, 0x184, 0x0C4, 0x0A8, // U-$
        0x0A2, 0x08A, 0x02A, // /-%
    ];

    pub const ASTERISK_ENCODING: u32 = 0x094;

    /**
     * Creates a reader that assumes all encoded data is data, and does not treat the final
     * character as a check digit. It will not decoded "extended Code 39" sequences.
     */
    pub fn new() -> Self {
        Self::with_use_check_digit(false)
    }

    /**
     * Creates a reader that can be configured to check the last character as a check digit.
     * It will not decoded "extended Code 39" sequences.
     *
     * @param usingCheckDigit if true, treat the last data character as a check digit, not
     * data, and verify that the checksum passes.
     */
    pub fn with_use_check_digit(usingCheckDigit: bool) -> Self {
        Self::with_all_config(usingCheckDigit, false)
    }

    /**
     * Creates a reader that can be configured to check the last character as a check digit,
     * or optionally attempt to decode "extended Code 39" sequences that are used to encode
     * the full ASCII character set.
     *
     * @param usingCheckDigit if true, treat the last data character as a check digit, not
     * data, and verify that the checksum passes.
     * @param extendedMode if true, will attempt to decode extended Code 39 sequences in the
     * text.
     */
    pub fn with_all_config(usingCheckDigit: bool, extendedMode: bool) -> Self {
        Self {
            usingCheckDigit,
            extendedMode,
            decodeRowRXingResult: String::with_capacity(20),
            // counters: vec![0; 9],
        }
    }

    fn findAsteriskPattern(row: &BitArray, counters: &mut [u32]) -> Result<Vec<u32>> {
        let width = row.get_size();
        let rowOffset = row.getNextSet(0);

        let mut counterPosition = 0;
        let mut patternStart = rowOffset;
        let mut isWhite = false;
        let patternLength = counters.len();

        for i in rowOffset..width {
            // for (int i = rowOffset; i < width; i++) {
            if row.get(i) != isWhite {
                counters[counterPosition] += 1;
            } else {
                if counterPosition == patternLength - 1 {
                    // Look for whitespace before start pattern, >= 50% of width of start pattern
                    if Self::toNarrowWidePattern(counters) == (Self::ASTERISK_ENCODING as i32)
                        && row.isRange(
                            0.max(
                                patternStart as isize - ((i as isize - patternStart as isize) / 2),
                            ) as usize,
                            patternStart,
                            false,
                        )?
                    {
                        return Ok(vec![patternStart as u32, i as u32]);
                        // return new int[]{patternStart, i};
                    }
                    patternStart += (counters[0] + counters[1]) as usize;

                    counters.copy_within(2..(counterPosition - 1 + 2), 0);
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
        Err(Exceptions::NOT_FOUND)
    }

    // For efficiency, returns -1 on failure. Not throwing here saved as many as 700 exceptions
    // per image when using some of our blackbox images.
    fn toNarrowWidePattern(counters: &[u32]) -> i32 {
        let numCounters = counters.len();
        let mut maxNarrowCounter = 0;
        let mut wideCounters;
        loop {
            let mut minCounter = u32::MAX;
            for counter in counters {
                if counter < &minCounter && counter > &maxNarrowCounter {
                    minCounter = *counter;
                }
            }
            maxNarrowCounter = minCounter;
            wideCounters = 0;
            let mut totalWideCountersWidth = 0;
            let mut pattern = 0;
            for (i, counter) in counters.iter().enumerate().take(numCounters) {
                if *counter > maxNarrowCounter {
                    pattern |= 1 << (numCounters - 1 - i);
                    wideCounters += 1;
                    totalWideCountersWidth += *counter;
                }
            }
            if wideCounters == 3 {
                // Found 3 wide counters, but are they close enough in width?
                // We can perform a cheap, conservative check to see if any individual
                // counter is more than 1.5 times the average:
                let mut i = 0;
                while i < numCounters && wideCounters > 0 {
                    let counter = counters[i];
                    if counter > maxNarrowCounter {
                        wideCounters -= 1;
                        // totalWideCountersWidth = 3 * average, so this checks if counter >= 3/2 * average
                        if (counter * 2) >= totalWideCountersWidth {
                            return -1;
                        }
                    }

                    i += 1;
                }
                return pattern;
            }

            if wideCounters <= 3 {
                break;
            }
        }
        -1
    }

    fn patternToChar(pattern: u32) -> Result<char> {
        for i in 0..Self::CHARACTER_ENCODINGS.len() {
            if Self::CHARACTER_ENCODINGS[i] == pattern {
                return C39R_CACHED_ALPHABET_STRING
                    .get(i)
                    .copied()
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS);
            }
        }
        if pattern == Self::ASTERISK_ENCODING {
            return Ok('*');
        }
        Err(Exceptions::NOT_FOUND)
    }

    fn decodeExtended(encoded: &str) -> Result<String> {
        let length = encoded.chars().count();
        let mut decoded = String::with_capacity(length); //new StringBuilder(length);
        let mut i = 0;
        let cached_encoded = encoded.chars().collect::<Vec<_>>();
        while i < length {
            // for i in 0..length {
            // for (int i = 0; i < length; i++) {
            let c = *cached_encoded
                .get(i)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
            if c == '+' || c == '$' || c == '%' || c == '/' {
                let next = *cached_encoded
                    .get(i + 1)
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                let mut decodedChar = '\0';
                match c {
                    '+' => {
                        // +A to +Z map to a to z
                        if next.is_ascii_uppercase() {
                            decodedChar = char::from_u32(next as u32 + 32)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                        } else {
                            return Err(Exceptions::NOT_FOUND);
                        }
                    }
                    '$' => {
                        // $A to $Z map to control codes SH to SB
                        if next.is_ascii_uppercase() {
                            decodedChar = char::from_u32(next as u32 - 64)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                        } else {
                            return Err(Exceptions::NOT_FOUND);
                        }
                    }
                    '%' => {
                        // %A to %E map to control codes ESC to US
                        if ('A'..='E').contains(&next) {
                            decodedChar = char::from_u32(next as u32 - 38)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                        } else if ('F'..='J').contains(&next) {
                            decodedChar = char::from_u32(next as u32 - 11)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                        } else if ('K'..='O').contains(&next) {
                            decodedChar = char::from_u32(next as u32 + 16)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                        } else if ('P'..='T').contains(&next) {
                            decodedChar = char::from_u32(next as u32 + 43)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                        } else if next == 'U' {
                            decodedChar = 0 as char;
                        } else if next == 'V' {
                            decodedChar = '@';
                        } else if next == 'W' {
                            decodedChar = '`';
                        } else if next == 'X' || next == 'Y' || next == 'Z' {
                            decodedChar = 127 as char;
                        } else {
                            return Err(Exceptions::NOT_FOUND);
                        }
                    }
                    '/' => {
                        // /A to /O map to ! to , and /Z maps to :
                        if ('A'..='O').contains(&next) {
                            decodedChar = char::from_u32(next as u32 - 32)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
                        } else if next == 'Z' {
                            decodedChar = ':';
                        } else {
                            return Err(Exceptions::NOT_FOUND);
                        }
                    }
                    _ => {}
                }
                decoded.push(decodedChar);
                // bump up i again since we read two characters
                i += 1;
            } else {
                decoded.push(c);
            }
            i += 1;
        }
        Ok(decoded)
    }
}

#[cfg(test)]
mod code_39_extended_mode_test_case {

    use crate::{
        common::BitMatrix,
        oned::{Code39Reader, OneDReader},
        DecodeHints,
    };
    #[test]
    fn testDecodeExtendedMode() {
        // \b -> 2408 \f -> 000c
        doTest("\u{0000}\u{0001}\u{0002}\u{0003}\u{0004}\u{0005}\u{0006}\u{0007}\u{0008}\t\n\u{000b}\u{000C}\r\u{000e}\u{000f}\u{0010}\u{0011}\u{0012}\u{0013}\u{0014}\u{0015}\u{0016}\u{0017}\u{0018}\u{0019}\u{001a}\u{001b}\u{001c}\u{001d}\u{001e}\u{001f}",
           "000001001011011010101001001001011001010101101001001001010110101001011010010010010101011010010110100100100101011011010010101001001001010101011001011010010010010101101011001010100100100101010110110010101001001001010101010011011010010010010101101010011010100100100101010110100110101001001001010101011001101010010010010101101010100110100100100101010110101001101001001001010110110101001010010010010101010110100110100100100101011010110100101001001001010101101101001010010010010101010101100110100100100101011010101100101001001001010101101011001010010010010101010110110010100100100101011001010101101001001001010100110101011010010010010101100110101010100100100101010010110101101001001001010110010110101010010010010101001101101010101001001001011010100101101010010010010101101001011010100100100101101101001010101001001001010101100101101010010010010110101100101010010110110100000");
        doTest(" !\"#$%&'()*+,-./0123456789:;<=>?",
           "00000100101101101010011010110101001001010010110101001011010010010100101011010010110100100101001011011010010101001001010010101011001011010010010100101101011001010100100101001010110110010101001001010010101010011011010010010100101101010011010100100101001010110100110101001001010010101011001101010010010100101101010100110100100101001010110101001101001010110110110010101101010010010100101101011010010101001101101011010010101101011001010110110110010101010100110101101101001101010101100110101010100101101101101001011010101100101101010010010100101001101101010101001001001010110110010101010010010010101010011011010100100100101101010011010101001001001010110100110101010010010010101011001101010010110110100000");
        doTest("@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_",
           "000010010110110101010010010010100110101011011010100101101011010010110110110100101010101100101101101011001010101101100101010101001101101101010011010101101001101010101100110101101010100110101101010011011011010100101010110100110110101101001010110110100101010101100110110101011001010110101100101010110110010110010101011010011010101101100110101010100101101011011001011010101001101101010101001001001011010101001101010010010010101101010011010100100100101101101010010101001001001010101101001101010010010010110101101001010010110110100000");
        doTest("`abcdefghijklmnopqrstuvwxyz{|}~",
           "000001001011011010101001001001011001101010101001010010010110101001011010010100100101011010010110100101001001011011010010101001010010010101011001011010010100100101101011001010100101001001010110110010101001010010010101010011011010010100100101101010011010100101001001010110100110101001010010010101011001101010010100100101101010100110100101001001010110101001101001010010010110110101001010010100100101010110100110100101001001011010110100101001010010010101101101001010010100100101010101100110100101001001011010101100101001010010010101101011001010010100100101010110110010100101001001011001010101101001010010010100110101011010010100100101100110101010100101001001010010110101101001010010010110010110101010010100100101001101101010101001001001010110110100101010010010010101010110011010100100100101101010110010101001001001010110101100101010010010010101011011001010010110110100000");
    }

    fn doTest(expectedRXingResult: &str, encodedRXingResult: &str) {
        let mut sut = Code39Reader::with_all_config(false, true);
        let matrix =
            BitMatrix::parse_strings(encodedRXingResult, "1", "0").expect("bitmatrix parse");
        // let row = BitArray::with_size(matrix.getWidth() as usize);
        let row = matrix.getRow(0);
        let result = sut
            .decode_row(0, &row, &DecodeHints::default())
            .expect("decode row");
        assert_eq!(expectedRXingResult, result.getText());
    }
}
