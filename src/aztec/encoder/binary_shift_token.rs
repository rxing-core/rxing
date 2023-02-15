/*
 * Copyright 2013 ZXing authors
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

use std::fmt;

use crate::common::{BitArray, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryShiftToken {
    binary_shift_start: u32,
    binary_shift_byte_count: u32,
}

impl BinaryShiftToken {
    pub fn new(binary_shift_start: u32, binary_shift_byte_count: u32) -> Self {
        Self {
            binary_shift_start,
            binary_shift_byte_count,
        }
    }

    pub fn appendTo(&self, bit_array: &mut BitArray, text: &[u8]) -> Result<()> {
        let bsbc = self.binary_shift_byte_count as usize;
        for i in 0..bsbc {
            // for (int i = 0; i < bsbc; i++) {
            if i == 0 || (i == 31 && bsbc <= 62) {
                // We need a header before the first character, and before
                // character 31 when the total byte code is <= 62
                bit_array.appendBits(31, 5)?; // BINARY_SHIFT
                if bsbc > 62 {
                    bit_array.appendBits(bsbc as u32 - 31, 16)?;
                } else if i == 0 {
                    // 1 <= binaryShiftByteCode <= 62
                    bit_array.appendBits(bsbc.min(31) as u32, 5)?;
                } else {
                    // 32 <= binaryShiftCount <= 62 and i == 31
                    bit_array.appendBits(bsbc as u32 - 31, 5)?;
                }
            }
            bit_array.appendBits(text[self.binary_shift_start as usize + i].into(), 8)?;
        }
        Ok(())
    }

    // @Override
    // public String toString() {
    //   return "<" + binaryShiftStart + "::" + (binaryShiftStart + binaryShiftByteCount - 1) + '>';
    // }
}

impl fmt::Display for BinaryShiftToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<{}::{}>",
            self.binary_shift_start,
            (self.binary_shift_start + self.binary_shift_byte_count - 1)
        )
    }
}
