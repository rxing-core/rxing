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

use rxing_one_d_proc_derive::OneDWriter;

use crate::common::Result;
use crate::BarcodeFormat;

use super::OneDimensionalCodeWriter;

/**
 * This object renders a ITF code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */
#[derive(OneDWriter, Default)]
pub struct ITFWriter;

impl OneDimensionalCodeWriter for ITFWriter {
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>> {
        let length = contents.chars().count();
        if length % 2 != 0 {
            return Err(Exceptions::illegal_argument_with(
                "The length of the input should be even",
            ));
        }
        if length > 80 {
            return Err(Exceptions::illegal_argument_with(format!(
                "Requested contents should be less than 80 digits long, but got {length}"
            )));
        }

        Self::checkNumeric(contents)?;

        let mut result = vec![false; 9 + 9 * length];
        let mut pos = Self::appendPattern(&mut result, 0, &START_PATTERN, true) as usize;
        let mut i = 0;
        while i < length {
            let one = contents
                .chars()
                .nth(i)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                .to_digit(10)
                .ok_or(Exceptions::PARSE)? as usize;
            let two = contents
                .chars()
                .nth(i + 1)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                .to_digit(10)
                .ok_or(Exceptions::PARSE)? as usize;
            let mut encoding = [0; 10];
            for j in 0..5 {
                encoding[2 * j] = PATTERNS[one][j];
                encoding[2 * j + 1] = PATTERNS[two][j];
            }
            pos += Self::appendPattern(&mut result, pos, &encoding, true) as usize;

            i += 2;
        }
        Self::appendPattern(&mut result, pos, &END_PATTERN, true);

        Ok(result)
    }
    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::ITF])
    }
}

const START_PATTERN: [usize; 4] = [1, 1, 1, 1];
const END_PATTERN: [usize; 3] = [3, 1, 1];

const W: usize = 3; // Pixel width of a 3x wide line
const N: usize = 1; // Pixed width of a narrow line

// See ITFReader.PATTERNS

const PATTERNS: [[usize; 5]; 10] = [
    [N, N, W, W, N], // 0
    [W, N, N, N, W], // 1
    [N, W, N, N, W], // 2
    [W, W, N, N, N], // 3
    [N, N, W, N, W], // 4
    [W, N, W, N, N], // 5
    [N, W, W, N, N], // 6
    [N, N, N, W, W], // 7
    [W, N, N, W, N], // 8
    [N, W, N, W, N], // 9
];

/**
 * Tests {@link ITFWriter}.
 */
#[cfg(test)]
mod ITFWriterTestCase {
    use crate::{common::bit_matrix_test_case, BarcodeFormat, Writer};

    use super::ITFWriter;

    #[test]
    fn testEncode() {
        doTest(
            "00123456789012",
            "0000010101010111000111000101110100010101110001110111010001010001110100011\
100010101000101011100011101011101000111000101110100010101110001110100000",
        );
    }

    fn doTest(input: &str, expected: &str) {
        let result = ITFWriter::default()
            .encode(input, &BarcodeFormat::ITF, 0, 0)
            .expect("encode");
        assert_eq!(expected, bit_matrix_test_case::matrix_to_string(&result));
    }

    //@Test(expected = IllegalArgumentException.class)
    #[test]
    #[should_panic]
    fn testEncodeIllegalCharacters() {
        ITFWriter::default()
            .encode("00123456789abc", &BarcodeFormat::ITF, 0, 0)
            .expect("should fail");
    }
}
