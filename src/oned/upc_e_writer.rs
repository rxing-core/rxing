/*
 * Copyright 2009 ZXing authors
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

use super::{
    upc_e_reader, upc_ean_reader, OneDimensionalCodeWriter, UPCEANReader, UPCEANWriter, UPCEReader,
};

const CODE_WIDTH: usize = 3 + // start guard
      (7 * 6) + // bars
      6; // end guard

/**
 * This object renders an UPC-E code as a {@link BitMatrix}.
 *
 * @author 0979097955s@gmail.com (RX)
 */
#[derive(OneDWriter, Default)]
pub struct UPCEWriter;

impl UPCEANWriter for UPCEWriter {}

impl OneDimensionalCodeWriter for UPCEWriter {
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>> {
        let length = contents.chars().count();
        let mut contents = contents.to_owned();
        let reader = UPCEReader;
        match length {
            7 => {
                // No check digit present, calculate it and add it
                let check = reader.getStandardUPCEANChecksum(
                    &upc_e_reader::convertUPCEtoUPCA(&contents)
                        .ok_or(Exceptions::ILLEGAL_ARGUMENT)?
                        .chars()
                        .collect::<Vec<_>>(),
                )?;
                contents.push_str(&check.to_string());
            }
            8 => {
                if !reader.checkStandardUPCEANChecksum(
                    &upc_e_reader::convertUPCEtoUPCA(&contents)
                        .ok_or(Exceptions::ILLEGAL_ARGUMENT)?,
                )? {
                    return Err(Exceptions::illegal_argument_with(
                        "Contents do not pass checksum",
                    ));
                }
            }
            _ => {
                return Err(Exceptions::illegal_argument_with(format!(
                    "Requested contents should be 7 or 8 digits long, but got {length}"
                )))
            }
        }

        Self::checkNumeric(&contents)?;

        let firstDigit = contents
            .chars()
            .next()
            .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
            .to_digit(10)
            .ok_or(Exceptions::PARSE)? as usize; //Character.digit(contents.charAt(0), 10);
        if firstDigit != 0 && firstDigit != 1 {
            return Err(Exceptions::illegal_argument_with(
                "Number system must be 0 or 1",
            ));
        }

        let checkDigit = contents
            .chars()
            .nth(7)
            .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
            .to_digit(10)
            .ok_or(Exceptions::PARSE)? as usize; //Character.digit(contents.charAt(7), 10);
        let parities = UPCEReader::NUMSYS_AND_CHECK_DIGIT_PATTERNS[firstDigit][checkDigit];
        let mut result = [false; CODE_WIDTH];

        let mut pos =
            Self::appendPattern(&mut result, 0, &upc_ean_reader::START_END_PATTERN, true) as usize;

        for i in 1..=6 {
            // for (int i = 1; i <= 6; i++) {
            let mut digit = contents
                .chars()
                .nth(i)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                .to_digit(10)
                .ok_or(Exceptions::PARSE)? as usize; //Character.digit(contents.charAt(i), 10);
            if (parities >> (6 - i) & 1) == 1 {
                digit += 10;
            }
            pos += Self::appendPattern(
                &mut result,
                pos,
                &upc_ean_reader::L_AND_G_PATTERNS[digit],
                false,
            ) as usize;
        }

        Self::appendPattern(&mut result, pos, &upc_ean_reader::END_PATTERN, false);

        Ok(result.to_vec())
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::UPC_E])
    }

    fn getDefaultMargin(&self) -> u32 {
        Self::DEFAULT_MARGIN
    }
}

/**
 * Tests {@link UPCEWriter}.
 */
#[cfg(test)]
mod UPCEWriterTestCase {
    use crate::{common::bit_matrix_test_case, BarcodeFormat, Writer};

    use super::UPCEWriter;

    #[test]
    fn testEncode() {
        doTest(
            "05096893",
            "0000000000010101110010100111000101101011110110111001011101010100000000000",
        );
    }

    #[test]
    fn testEncodeSystem1() {
        doTest(
            "12345670",
            "0000000000010100100110111101010001101110010000101001000101010100000000000",
        );
    }

    #[test]
    fn testAddChecksumAndEncode() {
        doTest(
            "0509689",
            "0000000000010101110010100111000101101011110110111001011101010100000000000",
        );
    }

    fn doTest(content: &str, encoding: &str) {
        let result = UPCEWriter
            .encode(
                content,
                &BarcodeFormat::UPC_E,
                encoding.chars().count() as i32,
                0,
            )
            .expect("ok");
        assert_eq!(encoding, bit_matrix_test_case::matrix_to_string(&result));
    }

    #[test]
    #[should_panic]
    fn testEncodeIllegalCharacters() {
        UPCEWriter
            .encode("05096abc", &BarcodeFormat::UPC_E, 0, 0)
            .expect("ok");
    }
}
