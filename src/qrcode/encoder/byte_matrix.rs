/*
 * Copyright 2008 ZXing authors
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

use crate::{common::BitMatrix, Exceptions};

/**
 * JAVAPORT: The original code was a 2D array of ints, but since it only ever gets assigned
 * -1, 0, and 1, I'm going to use less memory and go with bytes.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ByteMatrix {
    bytes: Vec<Vec<u8>>,
    width: u32,
    height: u32,
}

impl ByteMatrix {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            bytes: vec![vec![0u8; width as usize]; height as usize],
            width,
            height,
        }
    }

    pub fn getHeight(&self) -> u32 {
        self.height
    }

    pub fn getWidth(&self) -> u32 {
        self.width
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        self.bytes[y as usize][x as usize]
    }

    /**
     * @return an internal representation as bytes, in row-major order. array[y][x] represents point (x,y)
     */
    pub fn getArray(&self) -> &Vec<Vec<u8>> {
        &self.bytes
    }

    pub fn set(&mut self, x: u32, y: u32, value: u8) {
        self.bytes[y as usize][x as usize] = value;
    }

    pub fn set_bool(&mut self, x: u32, y: u32, value: bool) {
        self.bytes[y as usize][x as usize] = u8::from(value); //if value { 1 } else { 0 };
    }

    pub fn clear(&mut self, value: u8) {
        for row in self.bytes.iter_mut() {
            row.fill(value);
        }
    }
}

impl fmt::Display for ByteMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(2 * self.width as usize * self.height as usize + 2);
        for y in 0..self.height as usize {
            let bytesY = &self.bytes[y];
            for byte in bytesY.iter().take(self.width as usize) {
                match *byte {
                    0 => result.push_str(" 0"),
                    1 => result.push_str(" 1"),
                    _ => result.push_str("  "),
                };
            }
            result.push('\n');
        }
        write!(f, "{result}")
    }
}

impl TryFrom<ByteMatrix> for BitMatrix {
    type Error = Exceptions;

    fn try_from(value: ByteMatrix) -> Result<Self, Self::Error> {
        let mut bit_matrix = BitMatrix::new(value.getWidth(), value.getHeight())?;
        for y in 0..value.getHeight() {
            for x in 0..value.getWidth() {
                if value.get(x, y) > 0 {
                    bit_matrix.set(x, y);
                }
            }
        }
        Ok(bit_matrix)
    }
}
