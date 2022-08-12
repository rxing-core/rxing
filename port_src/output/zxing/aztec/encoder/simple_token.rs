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

struct SimpleToken {
    super: Token;

    // For normal words, indicates value and bitCount
     let value: i16;

     let bit_count: i16;
}

impl SimpleToken {

    fn new( previous: &Token,  value: i32,  bit_count: i32) -> SimpleToken {
        super(previous);
        let .value = value as i16;
        let .bitCount = bit_count as i16;
    }

    fn  append_to(&self,  bit_array: &BitArray,  text: &Vec<i8>)   {
        bit_array.append_bits(self.value, self.bit_count);
    }

    pub fn  to_string(&self) -> String  {
         let mut value: i32 = self.value & ((1 << self.bit_count) - 1);
        value |= 1 << self.bit_count;
        return '<' + Integer::to_binary_string(value | (1 << self.bit_count))::substring(1) + '>';
    }
}

