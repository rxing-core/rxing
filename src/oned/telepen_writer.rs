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
use regex::Regex;
use crate::common::Result;
use crate::BarcodeFormat;

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
        // Calculate the checksum character
        let mut sum = 0;

        for c in contents.chars() {
            sum += c as u32;
        }

        let remainder = sum % 127;
        let diff = 127 - remainder;
        let checksum = if diff != 127 {
            diff as u8 as char
        }
        else {
            0 as char
        };
        
        // Build binary string
        let mut binary = String::new();

        // Opening character is always _
        let mut byte = "_".as_bytes()[0];
        let mut bits = self.get_bits(byte);
        
        for i in 0..8 {
            binary.push(if bits[i] {
                '1'
            }
            else {
                '0'
            });
        }

        // Content
        for index in 0..contents.chars().count() {
            byte = contents.chars().nth(index).unwrap() as u8;
            bits = self.get_bits(byte);
            
            for i in 0..8 {
                binary.push(if bits[i] {
                    '1'
                }
                else {
                    '0'
                });
            }
        }

        // Checksum
        byte = checksum as u8;
        bits = self.get_bits(byte);
        
        for i in 0..8 {
            binary.push(if bits[i] {
                '1'
            }
            else {
                '0'
            });
        }

        // Closing character is always z.
        byte = "z".as_bytes()[0];
        bits = self.get_bits(byte);
        
        for i in 0..8 {
            binary.push(if bits[i] {
                '1'
            }
            else {
                '0'
            });
        }

        let re = Regex::new(r"^01|10$|01*0|00|1").unwrap();
        let matches: Vec<&str> = re.find_iter(&binary).map(|m| m.as_str()).collect();

        let mut result = vec![false; 2000];
        let mut resultPosition = 0;

        for b in matches {
            if b == "010" {
                // BBB...
                result[resultPosition] = true;
                result[resultPosition + 1] = true;
                result[resultPosition + 2] = true;
                result[resultPosition + 3] = false;
                result[resultPosition + 4] = false;
                result[resultPosition + 5] = false;

                resultPosition += 6;
            }
            else if b == "00" {
                // BBB.
                result[resultPosition] = true;
                result[resultPosition + 1] = true;
                result[resultPosition + 2] = true;
                result[resultPosition + 3] = false;

                resultPosition += 4;
            }
            else if b == "1" {
                // B.
                result[resultPosition] = true;
                result[resultPosition + 1] = false;

                resultPosition += 2;
            }
            else if b == "01" {
                // B...
                result[resultPosition] = true;
                result[resultPosition + 1] = false;
                result[resultPosition + 2] = false;
                result[resultPosition + 3] = false;

                resultPosition += 4;
            }
            else if b == "10" {
                // B...
                result[resultPosition] = true;
                result[resultPosition + 1] = false;
                result[resultPosition + 2] = false;
                result[resultPosition + 3] = false;

                resultPosition += 4;
            }
            else if b == "0" {
                return Err(Exceptions::illegal_argument_with(format!(
                    "Invalid bit combination!"
                )));
            }
            else {
                // B... (B.)* B...
                result[resultPosition] = true;
                result[resultPosition + 1] = false;
                result[resultPosition + 2] = false;
                result[resultPosition + 3] = false;

                resultPosition += 4;

                for _j in 2 .. b.len() - 2 {
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

        Ok(result[0 .. resultPosition - 1].to_vec())
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::TELEPEN])
    }
}

impl TelepenWriter {
    fn get_bits(&self, byte: u8) -> Vec<bool> {
        let mut bits = vec![false; 8];
        let mut oneCount = 0;

        for i in 0..8 {
            let mask = 1 << i;
            bits[i] = (mask & byte) > 0;

            if bits[i] {
                oneCount += 1;
            }
        }

        // Set parity bit - there must be an even number
        // of 1s in the 8 bits.
        bits[7] = oneCount % 2 != 0;

        return bits;
    }
}

/**
 * @author Chris Wood
 */
#[cfg(test)]
mod TelepenWriterTestCase {
    use crate::{
        common::{bit_matrix_test_case, BitMatrix},
        BarcodeFormat, Writer,
    };

    use super::TelepenWriter;

    #[test]
    fn testEncode() {
        doTest(
            "Hello world!",
            concat!(
                "00000",
                "10101010101110001110111000111000101110001000100011101010100010001110101010001000101010101000100011101110111000101010101000101000101010101000100011100010001010001110101010001000111010111010101010111011101011101110001010111010111000101010101",
                "00000"
            ),
        );
    }
    
    fn doTest(input: &str, expected: &str) {
        let result = encode(input);
        assert_eq!(expected, bit_matrix_test_case::matrix_to_string(&result));
    }

    fn encode(input: &str) -> BitMatrix {
        TelepenWriter
            .encode(input, &BarcodeFormat::TELEPEN, 0, 0)
            .expect("must encode")
    }
}
