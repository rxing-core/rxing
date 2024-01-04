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

use crate::common::Result;
use crate::oned::telepen_common;
use crate::BarcodeFormat;
use regex::Regex;
use rxing_one_d_proc_derive::OneDWriter;

use super::OneDimensionalCodeWriter;

/**
 * This class renders Telepen as {@code boolean[]}.
 *
 * @author Chris Wood
 */
#[derive(OneDWriter, Default)]
pub struct TelepenWriter;

impl OneDimensionalCodeWriter for TelepenWriter {
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>> {
        self.encode_oned_with_hints(contents, &HashMap::new())
    }

    fn encode_oned_with_hints(
        &self,
        contents: &str,
        hints: &crate::EncodingHintDictionary,
    ) -> Result<Vec<bool>> {
        let mut decodedContents = contents.to_string();

        if matches!(
            hints.get(&EncodeHintType::TELEPEN_AS_NUMERIC),
            Some(EncodeHintValue::TelepenAsNumeric(true))
        ) {
            decodedContents = telepen_common::numeric_to_ascii(contents)?;
        }

        // Calculate the checksum character
        let checksum = telepen_common::calculate_checksum(&decodedContents);

        // Build binary string
        let mut binary = String::new();

        // Opening character is always _
        binary = self.add_to_binary('_', binary);

        // Content
        for c in decodedContents.chars() {
            if c as u32 > 127 {
                return Err(Exceptions::illegal_argument_with(
                    "Telepen only supports ASCII characters.".to_string(),
                ));
            }

            binary = self.add_to_binary(c, binary)
        }

        // Checksum
        binary = self.add_to_binary(checksum, binary);

        // Closing character is always z.
        binary = self.add_to_binary('z', binary);

        let re = Regex::new(r"^01|10$|01*0|00|1").unwrap();
        let matches: Vec<&str> = re.find_iter(&binary).map(|m| m.as_str()).collect();

        let mut result = vec![false; 2000];
        let mut resultPosition = 0;

        for b in matches {
            match b {
                "010" => {
                    // BBB...
                    result[resultPosition] = true;
                    result[resultPosition + 1] = true;
                    result[resultPosition + 2] = true;
                    result[resultPosition + 3] = false;
                    result[resultPosition + 4] = false;
                    result[resultPosition + 5] = false;

                    resultPosition += 6;
                }
                "00" => {
                    // BBB.
                    result[resultPosition] = true;
                    result[resultPosition + 1] = true;
                    result[resultPosition + 2] = true;
                    result[resultPosition + 3] = false;

                    resultPosition += 4;
                }
                "1" => {
                    // B.
                    result[resultPosition] = true;
                    result[resultPosition + 1] = false;

                    resultPosition += 2;
                }
                "01" => {
                    // B...
                    result[resultPosition] = true;
                    result[resultPosition + 1] = false;
                    result[resultPosition + 2] = false;
                    result[resultPosition + 3] = false;

                    resultPosition += 4;
                }
                "10" => {
                    // B...
                    result[resultPosition] = true;
                    result[resultPosition + 1] = false;
                    result[resultPosition + 2] = false;
                    result[resultPosition + 3] = false;

                    resultPosition += 4;
                }
                "0" => {
                    return Err(Exceptions::illegal_argument_with(
                        "Invalid bit combination!".to_string(),
                    ));
                }
                _ => {
                    // B... (B.)* B...
                    result[resultPosition] = true;
                    result[resultPosition + 1] = false;
                    result[resultPosition + 2] = false;
                    result[resultPosition + 3] = false;

                    resultPosition += 4;

                    for _j in 2..b.len() - 2 {
                        result[resultPosition] = true;
                        result[resultPosition + 1] = false;

                        resultPosition += 2;
                    }

                    result[resultPosition] = true;
                    result[resultPosition + 1] = false;
                    result[resultPosition + 2] = false;
                    result[resultPosition + 3] = false;

                    resultPosition += 4;
                }
            }
        }

        Ok(result[0..resultPosition - 1].to_vec())
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::TELEPEN])
    }
}

impl TelepenWriter {
    fn get_bits(&self, byte: u8) -> Vec<bool> {
        let mut bits = vec![false; 8];
        let mut oneCount = 0;

        for (i, bit) in bits.iter_mut().enumerate().take(8) {
            let mask = 1 << i;
            *bit = (mask & byte) > 0;

            if *bit {
                oneCount += 1;
            }
        }

        // Set parity bit - there must be an even number
        // of 1s in the 8 bits.
        bits[7] = oneCount % 2 != 0;

        bits
    }

    fn add_to_binary(&self, c: char, mut binary: String) -> String {
        let byte = c as u8;
        let bits = self.get_bits(byte);

        for bit in bits.iter().take(8) {
            binary.push(if *bit { '1' } else { '0' });
        }

        binary
    }
}

/**
 * @author Chris Wood
 */
#[cfg(test)]
mod TelepenWriterTestCase {
    use std::collections::HashMap;

    use crate::{
        common::{bit_matrix_test_case, BitMatrix},
        BarcodeFormat, EncodeHintType, EncodeHintValue, Writer,
    };

    use super::TelepenWriter;

    #[test]
    #[should_panic]
    fn testAsciiOnly() {
        encode("АБВГДЕЁЖ");
    }

    #[test]
    fn testEncode() {
        doTest(
            "Hello world!",
            concat!(
                "00000",
                "10101010101110001110111000111000101110001000100011101010100010001110101010001000101010101000100011101110111000101010101000101000101010101000100011100010001010001110101010001000111010111010101010111011101011101110001010111010111000101010101",
                "00000",
            ),
            false
        );

        doTest(
            "11058474",
            concat!(
                "00000",
                "101010101011100010001000111000101110111011100010101010101000100010111000100010001010111010001000111000101010101",
                "00000",
            ),
            true
        );
    }

    fn doTest(input: &str, expected: &str, numeric: bool) {
        let result: BitMatrix = if numeric {
            encode_with_hints(input)
        } else {
            encode(input)
        };

        assert_eq!(expected, bit_matrix_test_case::matrix_to_string(&result));
    }

    fn encode(input: &str) -> BitMatrix {
        TelepenWriter
            .encode(input, &BarcodeFormat::TELEPEN, 0, 0)
            .expect("must encode")
    }

    fn encode_with_hints(input: &str) -> BitMatrix {
        let mut hints = HashMap::new();
        hints.insert(
            EncodeHintType::TELEPEN_AS_NUMERIC,
            EncodeHintValue::TelepenAsNumeric(true),
        );

        TelepenWriter
            .encode_with_hints(input, &BarcodeFormat::TELEPEN, 0, 0, &hints)
            .expect("must encode")
    }
}
