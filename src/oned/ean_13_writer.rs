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
    oned::{upc_ean_reader, EAN13Reader},
    BarcodeFormat,
};

use super::{OneDimensionalCodeWriter, UPCEANReader, UPCEANWriter};

/**
 * This object renders an EAN13 code as a {@link BitMatrix}.
 *
 * @author aripollak@gmail.com (Ari Pollak)
 */
#[derive(OneDWriter, Default)]
pub struct EAN13Writer;
impl UPCEANWriter for EAN13Writer {}

impl OneDimensionalCodeWriter for EAN13Writer {
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>, crate::Exceptions> {
        let reader: EAN13Reader = EAN13Reader::default();
        let mut contents = contents.to_owned();
        let length = contents.chars().count();
        match length {
            12 => {
                // No check digit present, calculate it and add it

                // try {
                let check = reader.getStandardUPCEANChecksum(&contents)?;
                // } catch (FormatException fe) {
                // throw new IllegalArgumentException(fe);
                // }
                contents.push_str(&check.to_string());
            }
            13 => {
                //try {
                if !reader.checkStandardUPCEANChecksum(&contents)? {
                    return Err(Exceptions::IllegalArgumentException(Some(
                        "Contents do not pass checksum".to_owned(),
                    )));
                }
                //} catch (FormatException ignored) {
                //return Err( Exceptions::IllegalArgumentException("Illegal contents".to_owned()));
                //}
            }
            _ => {
                return Err(Exceptions::IllegalArgumentException(Some(format!(
                    "Requested contents should be 12 or 13 digits long, but got {length}"
                ))))
            }
        }

        EAN13Writer::checkNumeric(&contents)?;

        let firstDigit = contents.chars().next().unwrap().to_digit(10).unwrap() as usize; //, 10);
        let parities = EAN13Reader::FIRST_DIGIT_ENCODINGS[firstDigit];
        let mut result = [false; CODE_WIDTH];
        let mut pos = 0;

        pos +=
            EAN13Writer::appendPattern(&mut result, pos, &upc_ean_reader::START_END_PATTERN, true)
                as usize;

        // See EAN13Reader for a description of how the first digit & left bars are encoded
        for i in 1..=6 {
            // for (int i = 1; i <= 6; i++) {
            let mut digit = contents.chars().nth(i).unwrap().to_digit(10).unwrap() as usize; //Character.digit(contents.charAt(i), 10);
            if (parities >> (6 - i) & 1) == 1 {
                digit += 10;
            }
            pos += EAN13Writer::appendPattern(
                &mut result,
                pos,
                &upc_ean_reader::L_AND_G_PATTERNS[digit],
                false,
            ) as usize;
        }

        pos += EAN13Writer::appendPattern(&mut result, pos, &upc_ean_reader::MIDDLE_PATTERN, false)
            as usize;

        for i in 7..=12 {
            // for (int i = 7; i <= 12; i++) {
            // let digit = Character.digit(contents.charAt(i), 10);
            let digit = contents.chars().nth(i).unwrap().to_digit(10).unwrap() as usize; //Character.digit(contents.charAt(i), 10);

            pos += EAN13Writer::appendPattern(
                &mut result,
                pos,
                &upc_ean_reader::L_PATTERNS[digit],
                true,
            ) as usize;
        }
        EAN13Writer::appendPattern(&mut result, pos, &upc_ean_reader::START_END_PATTERN, true);

        Ok(result.to_vec())
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::EAN_13])
    }

    fn getDefaultMargin(&self) -> u32 {
        // CodaBar spec requires a side margin to be more than ten times wider than narrow space.
        // This seems like a decent idea for a default for all formats.
        Self::DEFAULT_MARGIN
    }
}

const CODE_WIDTH: usize = 3 + // start guard
      (7 * 6) + // left bars
      5 + // middle guard
      (7 * 6) + // right bars
      3; // end guard

/**
 * @author Ari Pollak
 */
#[cfg(test)]
mod EAN13WriterTestCase {
    use crate::{common::bit_matrix_test_case, BarcodeFormat, Writer};

    use super::EAN13Writer;

    #[test]
    fn testEncode() {
        let testStr =
        "00001010001011010011101100110010011011110100111010101011001101101100100001010111001001110100010010100000";
        let result = EAN13Writer::default()
            .encode(
                "5901234123457",
                &BarcodeFormat::EAN_13,
                testStr.chars().count() as i32,
                0,
            )
            .expect("exist");
        assert_eq!(testStr, bit_matrix_test_case::matrix_to_string(&result));
    }

    #[test]
    fn testAddChecksumAndEncode() {
        let testStr =
        "00001010001011010011101100110010011011110100111010101011001101101100100001010111001001110100010010100000";
        let result = EAN13Writer::default()
            .encode(
                "590123412345",
                &BarcodeFormat::EAN_13,
                testStr.chars().count() as i32,
                0,
            )
            .expect("exist");
        assert_eq!(testStr, bit_matrix_test_case::matrix_to_string(&result));
    }

    #[test]
    #[should_panic]
    fn testEncodeIllegalCharacters() {
        EAN13Writer::default()
            .encode("5901234123abc", &BarcodeFormat::EAN_13, 0, 0)
            .expect("encode");
    }
}
