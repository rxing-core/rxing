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

use crate::{
    oned::{EAN8Reader, UPCEANReader},
    BarcodeFormat,
};

use super::{upc_ean_reader, OneDimensionalCodeWriter, UPCEANWriter};

/**
 * This object renders an EAN8 code as a {@link BitMatrix}.
 *
 * @author aripollak@gmail.com (Ari Pollak)
 */
#[derive(OneDWriter, Default)]
pub struct EAN8Writer;

const CODE_WIDTH: usize = 3 + // start guard
      (7 * 4) + // left bars
      5 + // middle guard
      (7 * 4) + // right bars
      3; // end guard

impl UPCEANWriter for EAN8Writer {}
impl OneDimensionalCodeWriter for EAN8Writer {
    /**
     * @return a byte array of horizontal pixels (false = white, true = black)
     */
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>, Exceptions> {
        let length = contents.chars().count();
        let reader = EAN8Reader::default();
        let mut contents = contents.to_owned();
        match length {
            7 => {
                // No check digit present, calculate it and add it
                let check = reader.getStandardUPCEANChecksum(&contents)?;
                // try {
                //   check = UPCEANReader.getStandardUPCEANChecksum(contents);
                // } catch (FormatException fe) {
                //   throw new IllegalArgumentException(fe);
                // }
                contents.push_str(&check.to_string());
            }
            8 =>
            // try {
            {
                if !EAN8Reader.checkStandardUPCEANChecksum(&contents)? {
                    return Err(Exceptions::IllegalArgumentException(Some(
                        "Contents do not pass checksum".to_owned(),
                    )));
                }
            }
            // } catch (FormatException ignored) {
            //   throw new IllegalArgumentException("Illegal contents");
            // }},
            _ => {
                return Err(Exceptions::IllegalArgumentException(Some(format!(
                    "Requested contents should be 7 or 8 digits long, but got {}",
                    length
                ))))
            }
        }

        Self::checkNumeric(&contents)?;

        let mut result = [false; CODE_WIDTH]; //new boolean[CODE_WIDTH];
        let mut pos = 0;

        pos += Self::appendPattern(&mut result, pos, &upc_ean_reader::START_END_PATTERN, true)
            as usize;

        for i in 0..=3 {
            // for (int i = 0; i <= 3; i++) {
            let digit = contents.chars().nth(i).unwrap().to_digit(10).unwrap() as usize; //Character.digit(contents.charAt(i), 10);
            pos += Self::appendPattern(&mut result, pos, &upc_ean_reader::L_PATTERNS[digit], false)
                as usize;
        }

        pos +=
            Self::appendPattern(&mut result, pos, &upc_ean_reader::MIDDLE_PATTERN, false) as usize;

        for i in 4..=7 {
            // for (int i = 4; i <= 7; i++) {
            let digit = contents.chars().nth(i).unwrap().to_digit(10).unwrap() as usize; //Character.digit(contents.charAt(i), 10);
            pos += Self::appendPattern(&mut result, pos, &upc_ean_reader::L_PATTERNS[digit], true)
                as usize;
        }
        Self::appendPattern(&mut result, pos, &upc_ean_reader::START_END_PATTERN, true);

        Ok(result.to_vec())
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::EAN_8])
    }

    fn getDefaultMargin(&self) -> u32 {
        Self::DEFAULT_MARGIN
    }
}

/**
 * @author Ari Pollak
 */
#[cfg(test)]
mod EAN8WriterTestCase {
    use crate::{common::BitMatrixTestCase, BarcodeFormat, Writer};

    use super::EAN8Writer;

    #[test]
    fn testEncode() {
        let testStr =
            "0000001010001011010111101111010110111010101001110111001010001001011100101000000";
        let result = EAN8Writer::default()
            .encode(
                "96385074",
                &BarcodeFormat::EAN_8,
                testStr.chars().count() as i32,
                0,
            )
            .expect("ok");
        assert_eq!(testStr, BitMatrixTestCase::matrix_to_string(&result));
    }

    #[test]
    fn testAddChecksumAndEncode() {
        let testStr =
            "0000001010001011010111101111010110111010101001110111001010001001011100101000000";
        let result = EAN8Writer::default()
            .encode(
                "9638507",
                &BarcodeFormat::EAN_8,
                testStr.chars().count() as i32,
                0,
            )
            .expect("ok");
        assert_eq!(testStr, BitMatrixTestCase::matrix_to_string(&result));
    }

    #[test]
    #[should_panic]
    fn testEncodeIllegalCharacters() {
        EAN8Writer::default()
            .encode("96385abc", &BarcodeFormat::EAN_8, 0, 0)
            .expect("ok");
    }
}
