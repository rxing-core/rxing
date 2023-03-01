/*
 * Copyright 2011 ZXing authors
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

use rxing_one_d_proc_derive::OneDWriter;

use crate::common::Result;
use crate::BarcodeFormat;

use super::{CodaBarReader, OneDimensionalCodeWriter};

const START_END_CHARS: [char; 4] = ['A', 'B', 'C', 'D'];
const ALT_START_END_CHARS: [char; 4] = ['T', 'N', '*', 'E'];
const CHARS_WHICH_ARE_TEN_LENGTH_EACH_AFTER_DECODED: [char; 4] = ['/', ':', '+', '.'];
const DEFAULT_GUARD: char = START_END_CHARS[0];

/**
 * This class renders CodaBar as {@code boolean[]}.
 *
 * @author dsbnatut@gmail.com (Kazuki Nishiura)
 */
#[derive(OneDWriter, Default)]
pub struct CodaBarWriter;

impl OneDimensionalCodeWriter for CodaBarWriter {
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>> {
        let contents = if contents.chars().count() < 2 {
            // Can't have a start/end guard, so tentatively add default guards
            format!("{DEFAULT_GUARD}{contents}{DEFAULT_GUARD}")
        } else {
            // Verify input and calculate decoded length.
            let firstChar = contents
                .chars()
                .next()
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                .to_ascii_uppercase();
            let lastChar = contents
                .chars()
                .nth(contents.chars().count() - 1)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                .to_ascii_uppercase();
            let startsNormal = CodaBarReader::arrayContains(&START_END_CHARS, firstChar);
            let endsNormal = CodaBarReader::arrayContains(&START_END_CHARS, lastChar);
            let startsAlt = CodaBarReader::arrayContains(&ALT_START_END_CHARS, firstChar);
            let endsAlt = CodaBarReader::arrayContains(&ALT_START_END_CHARS, lastChar);
            if startsNormal {
                if !endsNormal {
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Invalid start/end guards: {contents}"
                    )));
                }
                // else already has valid start/end
                contents.to_owned()
            } else if startsAlt {
                if !endsAlt {
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Invalid start/end guards: {contents}"
                    )));
                }
                // else already has valid start/end
                contents.to_owned()
            } else {
                // Doesn't start with a guard
                if endsNormal || endsAlt {
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Invalid start/end guards: {contents}"
                    )));
                }
                // else doesn't end with guard either, so add a default
                format!("{DEFAULT_GUARD}{contents}{DEFAULT_GUARD}")
            }
        };

        // The start character and the end character are decoded to 10 length each.
        let mut resultLength = 20;
        for ch in contents[1..contents.chars().count() - 1].chars() {
            if ch.is_ascii_digit() || ch == '-' || ch == '$' {
                resultLength += 9;
            } else if CodaBarReader::arrayContains(
                &CHARS_WHICH_ARE_TEN_LENGTH_EACH_AFTER_DECODED,
                ch,
            ) {
                resultLength += 10;
            } else {
                return Err(Exceptions::illegal_argument_with(format!(
                    "Cannot encode : '{ch}'"
                )));
            }
        }
        // A blank is placed between each character.
        resultLength += contents.chars().count() - 1;

        let mut result = vec![false; resultLength];
        let mut position = 0;
        for index in 0..contents.chars().count() {
            // for (int index = 0; index < contents.length(); index++) {
            let mut c = contents
                .chars()
                .nth(index)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                .to_ascii_uppercase();
            if index == 0 || index == contents.chars().count() - 1 {
                // The start/end chars are not in the CodaBarReader.ALPHABET.
                c = match c {
                    'T' => 'A',
                    'N' => 'B',
                    '*' => 'C',
                    'E' => 'D',
                    _ => c,
                }
            }
            let mut code = 0;
            for i in 0..CodaBarReader::ALPHABET.len() {
                // Found any, because I checked above.
                if c == CodaBarReader::ALPHABET[i] {
                    code = CodaBarReader::CHARACTER_ENCODINGS[i];
                    break;
                }
            }
            let mut color = true;
            let mut counter = 0;
            let mut bit = 0;
            while bit < 7 {
                // A character consists of 7 digit.
                result[position] = color;
                position += 1;
                if ((code >> (6 - bit)) & 1) == 0 || counter == 1 {
                    color = !color; // Flip the color.
                    bit += 1;
                    counter = 0;
                } else {
                    counter += 1;
                }
            }
            if index < contents.chars().count() - 1 {
                result[position] = false;
                position += 1;
            }
        }
        Ok(result)
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::CODABAR])
    }
}

/**
 * @author dsbnatut@gmail.com (Kazuki Nishiura)
 * @author Sean Owen
 */
#[cfg(test)]
mod CodaBarWriterTestCase {
    use crate::{
        common::{bit_matrix_test_helpers, BitMatrix},
        BarcodeFormat, Writer,
    };

    use super::CodaBarWriter;

    #[test]
    fn testEncode() {
        doTest(
            "B515-3/B",
            concat!(
                "00000",
                "1001001011",
                "0110101001",
                "0101011001",
                "0110101001",
                "0101001101",
                "0110010101",
                "01101101011",
                "01001001011",
                "00000"
            ),
        );
    }

    #[test]
    fn testEncode2() {
        doTest(
            "T123T",
            concat!(
                "00000",
                "1011001001",
                "0101011001",
                "0101001011",
                "0110010101",
                "01011001001",
                "00000"
            ),
        );
    }

    #[test]
    fn testAltStartEnd() {
        assert_eq!(encode("T123456789-$T"), encode("A123456789-$A"));
    }

    fn doTest(input: &str, expected: &str) {
        let result = encode(input);
        assert_eq!(expected, bit_matrix_test_helpers::matrix_to_string(&result));
    }

    fn encode(input: &str) -> BitMatrix {
        CodaBarWriter::default()
            .encode(input, &BarcodeFormat::CODABAR, 0, 0)
            .expect("must encode")
    }
}
