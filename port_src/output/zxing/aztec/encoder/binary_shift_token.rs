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
// package com::google::zxing::aztec::encoder;

struct BinaryShiftToken {
    super: Token;

     let binary_shift_start: i32;

     let binary_shift_byte_count: i32;
}

impl BinaryShiftToken {

    fn new( previous: &Token,  binary_shift_start: i32,  binary_shift_byte_count: i32) -> BinaryShiftToken {
        super(previous);
        let .binaryShiftStart = binary_shift_start;
        let .binaryShiftByteCount = binary_shift_byte_count;
    }

    pub fn  append_to(&self,  bit_array: &BitArray,  text: &Vec<i8>)   {
         let bsbc: i32 = self.binary_shift_byte_count;
         {
             let mut i: i32 = 0;
            while i < bsbc {
                {
                    if i == 0 || (i == 31 && bsbc <= 62) {
                        // We need a header before the first character, and before
                        // character 31 when the total byte code is <= 62
                        // BINARY_SHIFT
                        bit_array.append_bits(31, 5);
                        if bsbc > 62 {
                            bit_array.append_bits(bsbc - 31, 16);
                        } else if i == 0 {
                            // 1 <= binaryShiftByteCode <= 62
                            bit_array.append_bits(&Math::min(bsbc, 31), 5);
                        } else {
                            // 32 <= binaryShiftCount <= 62 and i == 31
                            bit_array.append_bits(bsbc - 31, 5);
                        }
                    }
                    bit_array.append_bits(text[self.binary_shift_start + i], 8);
                }
                i += 1;
             }
         }

    }

    pub fn  to_string(&self) -> String  {
        return format!("<{}::{}>", self.binary_shift_start, (self.binary_shift_start + self.binary_shift_byte_count - 1));
    }
}

