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

//package com.google.zxing;

use crate::common::Result;
use crate::{Exceptions, LuminanceSource};

/**
 * This class is used to help decode images from files which arrive as RGB data from
 * an ARGB pixel array. It does not support rotation.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Betaminos
 */
#[derive(Debug, Clone)]
pub struct RGBLuminanceSource {
    luminances: Box<[u8]>,
    dataWidth: usize,
    dataHeight: usize,
    width: usize,
    height: usize,
    invert: bool,
}

impl LuminanceSource for RGBLuminanceSource {
    const SUPPORTS_CROP: bool = true;

    /// gets a row, returns an empty row if we are out of bounds.
    fn get_row(&self, y: usize) -> Vec<u8> {
        if y >= self.get_height() {
            return Vec::new();
        }
        let width = self.get_width();

        let offset = (y) * self.dataWidth;

        let mut row = vec![0; width];

        row[..width].clone_from_slice(&self.luminances[offset..offset + width]);

        if self.invert {
            row = self.invert_block_of_bytes(row);
        }
        row
    }

    fn get_column(&self, _x: usize) -> Vec<u8> {
        unimplemented!()
    }

    fn get_matrix(&self) -> Vec<u8> {
        if self.invert {
            self.invert_block_of_bytes(self.luminances.to_vec())
        } else {
            self.luminances.to_vec()
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> Result<Self> {
        RGBLuminanceSource::new_complex(
            self.luminances
                .chunks_exact(self.dataWidth)
                .skip(top)
                .take(height)
                .flat_map(|row| row.iter().skip(left))
                .copied()
                .collect(),
            self.dataWidth,
            self.dataHeight,
            width,
            height,
        )
        .map_err(|_| Exceptions::UNSUPPORTED_OPERATION)
    }

    fn invert(&mut self) {
        self.invert = !self.invert;
    }

    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        let _width = self.get_width();
        let row_offset = (y) * self.dataWidth;
        let col_offset = x;

        self.luminances[row_offset + col_offset]
    }
}

impl RGBLuminanceSource {
    pub fn new_with_width_height_pixels(width: usize, height: usize, pixels: &[u32]) -> Self {
        let dataWidth = width;
        let dataHeight = height;

        // In order to measure pure decoding speed, we convert the entire image to a greyscale array
        // up front, which is the same as the Y channel of the YUVLuminanceSource in the real app.
        //
        // Total number of pixels suffices, can ignore shape
        let size = width * height;
        let mut luminances: Vec<u8> = vec![0; size];
        for offset in 0..size {
            let pixel = pixels[offset];
            let r = (pixel >> 16) & 0xff; // red
            let g2 = (pixel >> 7) & 0x1fe; // 2 * green
            let b = pixel & 0xff; // blue
                                  // Calculate green-favouring average cheaply
            luminances[offset] = ((r + g2 + b) / 4) as u8;
        }
        Self {
            luminances: luminances.into_boxed_slice(),
            dataWidth,
            dataHeight,
            width,
            height,
            invert: false,
        }
    }

    fn new_complex(
        pixels: Box<[u8]>,
        data_width: usize,
        data_height: usize,
        width: usize,
        height: usize,
    ) -> Result<Self> {
        if width > data_width || height > data_height {
            return Err(Exceptions::illegal_argument_with(
                "Crop rectangle does not fit within image data.",
            ));
        }
        Ok(Self {
            luminances: pixels,
            dataWidth: data_width,
            dataHeight: data_height,
            width,
            height,
            invert: false,
        })
    }
}
