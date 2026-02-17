/*
 * Copyright 2026 RXing authors
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

use crate::common::BitArray;
use crate::LuminanceSource;
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;

static SPACES: Lazy<Regex> = Lazy::new(|| Regex::new("\\s+").unwrap());
static DOTX: Lazy<Regex> = Lazy::new(|| Regex::new("[^.X]").unwrap());

#[derive(Clone)]
pub struct MockLuminanceSource {
    width: usize,
    height: usize,
    luminances: Vec<u8>,
}

impl MockLuminanceSource {
    pub fn new(width: usize, height: usize, luminances: Vec<u8>) -> Self {
        Self {
            width,
            height,
            luminances,
        }
    }
}

impl LuminanceSource for MockLuminanceSource {
    fn get_row(&self, y: usize) -> Option<Cow<'_, [u8]>> {
        let offset = y * self.width;
        Some(Cow::Borrowed(&self.luminances[offset..offset + self.width]))
    }

    fn get_column(&self, x: usize) -> Vec<u8> {
        let mut column = Vec::with_capacity(self.height);
        for y in 0..self.height {
            column.push(self.luminances[y * self.width + x]);
        }
        column
    }

    fn get_matrix(&self) -> Vec<u8> {
        self.luminances.clone()
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn invert(&mut self) {
        for l in self.luminances.iter_mut() {
            *l = 255 - *l;
        }
    }

    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        self.luminances[y * self.width + x]
    }
}

#[allow(dead_code)]
pub fn to_bit_array(bits: &str) -> BitArray {
    let mut ba_in = BitArray::new();
    let str = DOTX.replace_all(bits, "");
    for a_str in str.chars() {
        ba_in.appendBit(a_str == 'X');
    }

    ba_in
}

#[allow(dead_code)]
pub fn to_boolean_array(bit_array: &BitArray) -> Vec<bool> {
    let mut result = vec![false; bit_array.get_size()];
    for (i, res) in result.iter_mut().enumerate() {
        *res = bit_array.get(i);
    }
    result
}

#[allow(dead_code)]
pub fn strip_space(s: &str) -> String {
    SPACES.replace_all(s, "").to_string()
}

#[allow(dead_code)]
pub fn make_larger(input: &crate::common::BitMatrix, factor: u32) -> crate::common::BitMatrix {
    let width = input.getWidth();
    let height = input.getHeight();
    let mut output = crate::common::BitMatrix::new(width * factor, height * factor).expect("new");
    for input_y in 0..height {
        for input_x in 0..width {
            if input.get(input_x, input_y) {
                output
                    .setRegion(input_x * factor, input_y * factor, factor, factor)
                    .expect("region set should be ok");
            }
        }
    }
    output
}

#[allow(dead_code)]
pub fn arrays_are_equal<T: Eq + Default>(left: &[T], right: &[T], size: usize) -> bool {
    for i in 0..size {
        if left[i] != right[i] {
            return false;
        }
    }
    true
}
