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
    luminances: Vec<u8>,
    dataWidth: usize,
    dataHeight: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    invert: bool,
}

impl LuminanceSource for RGBLuminanceSource {
    /// gets a row, returns an empty row if we are out of bounds.
    fn getRow(&self, y: usize) -> Vec<u8> {
        if y >= self.getHeight() {
            return Vec::new();
        }
        let width = self.getWidth();

        let offset = (y + self.top) * self.dataWidth + self.left;

        let mut row = vec![0; width];

        row[..width].clone_from_slice(&self.luminances[offset..offset + width]);

        if self.invert {
            row = self.invert_block_of_bytes(row);
        }
        row
    }

    fn getMatrix(&self) -> Vec<u8> {
        let width = self.getWidth();
        let height = self.getHeight();

        // If the caller asks for the entire underlying image, save the copy and give them the
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.dataWidth && height == self.dataHeight {
            let mut z = self.luminances.clone();
            if self.invert {
                z = self.invert_block_of_bytes(z);
            }
            return z;
        }

        let area = width * height;
        let mut matrix = vec![0; area];
        let mut inputOffset = self.top * self.dataWidth + self.left;

        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.dataWidth {
            matrix[..area].clone_from_slice(&self.luminances[inputOffset..area + inputOffset]);
            if self.invert {
                matrix = self.invert_block_of_bytes(matrix);
            }
            return matrix;
        }

        // Otherwise copy one cropped row at a time.
        for y in 0..height {
            let outputOffset = y * width;
            matrix[outputOffset..width + outputOffset]
                .clone_from_slice(&self.luminances[inputOffset..width + inputOffset]);
            inputOffset += self.dataWidth;
        }

        if self.invert {
            matrix = self.invert_block_of_bytes(matrix);
        }
        matrix
    }

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
    }

    fn isCropSupported(&self) -> bool {
        true
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>> {
        match RGBLuminanceSource::new_complex(
            &self.luminances,
            self.dataWidth,
            self.dataHeight,
            self.left + left,
            self.top + top,
            width,
            height,
        ) {
            Ok(crop) => Ok(Box::new(crop)),
            Err(_error) => Err(Exceptions::UNSUPPORTED_OPERATION),
        }
    }

    fn invert(&mut self) {
        self.invert = !self.invert;
    }
}

impl RGBLuminanceSource {
    pub fn new_with_width_height_pixels(width: usize, height: usize, pixels: &[u32]) -> Self {
        let dataWidth = width;
        let dataHeight = height;
        let left = 0;
        let top = 0;

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
            luminances,
            dataWidth,
            dataHeight,
            left,
            top,
            width,
            height,
            invert: false,
        }
    }

    fn new_complex(
        pixels: &[u8],
        data_width: usize,
        data_height: usize,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Self> {
        if left + width > data_width || top + height > data_height {
            return Err(Exceptions::illegal_argument_with(
                "Crop rectangle does not fit within image data.",
            ));
        }
        Ok(Self {
            luminances: pixels.to_owned(),
            dataWidth: data_width,
            dataHeight: data_height,
            left,
            top,
            width,
            height,
            invert: false,
        })
    }
}
