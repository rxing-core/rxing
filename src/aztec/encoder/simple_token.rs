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

use crate::common::BitArray;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SimpleToken {
    // For normal words, indicates value and bitCount
    value: u16,
    bitCount: u16,
}

impl SimpleToken {
    pub fn new(value: i32, bitCount: u32) -> Self {
        Self {
            value: value as u16,
            bitCount: bitCount as u16,
        }
    }

    pub fn appendTo(&self, bitArray: &mut BitArray, text: &[u8]) {
        bitArray
            .appendBits(self.value as u32, self.bitCount as usize)
            .expect("append should never fail");
    }

    // @Override
    // public String toString() {
    //   int value = this.value & ((1 << bitCount) - 1);
    //   value |= 1 << bitCount;
    //   return '<' + Integer.toBinaryString(value | (1 << bitCount)).substring(1) + '>';
    // }
}

impl fmt::Display for SimpleToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = self.value & ((1 << self.bitCount) - 1);
        value |= 1 << self.bitCount;
        write!(f, "<{:#016b}>", value | (1 << self.bitCount))
    }
}
