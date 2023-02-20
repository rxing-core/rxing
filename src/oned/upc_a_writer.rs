/*
 * Copyright 2010 ZXing authors
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

use std::collections::HashMap;

use crate::{common::Result, BarcodeFormat, Exceptions, Writer};

use super::EAN13Writer;

/**
 * This object renders a UPC-A code as a {@link BitMatrix}.
 *
 * @author qwandor@google.com (Andrew Walbran)
 */
#[derive(Default)]
pub struct UPCAWriter(EAN13Writer);

impl Writer for UPCAWriter {
    fn encode(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<crate::common::BitMatrix> {
        self.encode_with_hints(contents, format, width, height, &HashMap::new())
    }

    fn encode_with_hints(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
        hints: &crate::EncodingHintDictionary,
    ) -> Result<crate::common::BitMatrix> {
        if format != &BarcodeFormat::UPC_A {
            return Err(Exceptions::illegal_argument_with(format!(
                "Can only encode UPC-A, but got {format:?}"
            )));
        }
        // Transform a UPC-A code into the equivalent EAN-13 code and write it that way
        self.0.encode_with_hints(
            &format!("0{contents}"),
            &BarcodeFormat::EAN_13,
            width,
            height,
            hints,
        )
    }
}

// private final EAN13Writer subWriter = new EAN13Writer();

/**
 * @author qwandor@google.com (Andrew Walbran)
 */
#[cfg(test)]
mod UPCAWriterTestCase {
    use crate::{common::bit_matrix_test_case, BarcodeFormat, Writer};

    use super::UPCAWriter;

    #[test]
    fn testEncode() {
        let testStr =
        "00001010100011011011101100010001011010111101111010101011100101110100100111011001101101100101110010100000";
        let result = UPCAWriter::default()
            .encode(
                "485963095124",
                &BarcodeFormat::UPC_A,
                testStr.chars().count() as i32,
                0,
            )
            .expect("ok");
        assert_eq!(testStr, bit_matrix_test_case::matrix_to_string(&result));
    }

    #[test]
    fn testAddChecksumAndEncode() {
        let testStr =
        "00001010011001001001101111010100011011000101011110101010001001001000111010011100101100110110110010100000";
        let result = UPCAWriter::default()
            .encode(
                "12345678901",
                &BarcodeFormat::UPC_A,
                testStr.chars().count() as i32,
                0,
            )
            .expect("ok");
        assert_eq!(testStr, bit_matrix_test_case::matrix_to_string(&result));
    }
}
