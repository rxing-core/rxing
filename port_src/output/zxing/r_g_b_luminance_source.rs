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
// package com::google::zxing;

/**
 * This class is used to help decode images from files which arrive as RGB data from
 * an ARGB pixel array. It does not support rotation.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Betaminos
 */
pub struct RGBLuminanceSource {
    super: LuminanceSource;

     let mut luminances: Vec<i8>;

     let data_width: i32;

     let data_height: i32;

     let mut left: i32;

     let mut top: i32;
}

impl RGBLuminanceSource {

    pub fn new( width: i32,  height: i32,  pixels: &Vec<i32>) -> RGBLuminanceSource {
        super(width, height);
        data_width = width;
        data_height = height;
        left = 0;
        top = 0;
        // In order to measure pure decoding speed, we convert the entire image to a greyscale array
        // up front, which is the same as the Y channel of the YUVLuminanceSource in the real app.
        //
        // Total number of pixels suffices, can ignore shape
         let size: i32 = width * height;
        luminances = : [i8; size] = [0; size];
         {
             let mut offset: i32 = 0;
            while offset < size {
                {
                     let pixel: i32 = pixels[offset];
                    // red
                     let r: i32 = (pixel >> 16) & 0xff;
                    // 2 * green
                     let g2: i32 = (pixel >> 7) & 0x1fe;
                    // blue
                     let b: i32 = pixel & 0xff;
                    // Calculate green-favouring average cheaply
                    luminances[offset] = ((r + g2 + b) / 4) as i8;
                }
                offset += 1;
             }
         }

    }

    fn new( pixels: &Vec<i8>,  data_width: i32,  data_height: i32,  left: i32,  top: i32,  width: i32,  height: i32) -> RGBLuminanceSource {
        super(width, height);
        if left + width > data_width || top + height > data_height {
            throw IllegalArgumentException::new("Crop rectangle does not fit within image data.");
        }
        let .luminances = pixels;
        let .dataWidth = data_width;
        let .dataHeight = data_height;
        let .left = left;
        let .top = top;
    }

    pub fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8>  {
        if y < 0 || y >= get_height() {
            throw IllegalArgumentException::new(format!("Requested row is outside the image: {}", y));
        }
         let width: i32 = get_width();
        if row == null || row.len() < width {
            row = : [i8; width] = [0; width];
        }
         let offset: i32 = (y + self.top) * self.data_width + self.left;
        System::arraycopy(&self.luminances, offset, &row, 0, width);
        return row;
    }

    pub fn  get_matrix(&self) -> Vec<i8>  {
         let width: i32 = get_width();
         let height: i32 = get_height();
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.data_width && height == self.data_height {
            return self.luminances;
        }
         let area: i32 = width * height;
         let matrix: [i8; area] = [0; area];
         let input_offset: i32 = self.top * self.data_width + self.left;
        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.data_width {
            System::arraycopy(&self.luminances, input_offset, &matrix, 0, area);
            return matrix;
        }
        // Otherwise copy one cropped row at a time.
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let output_offset: i32 = y * width;
                    System::arraycopy(&self.luminances, input_offset, &matrix, output_offset, width);
                    input_offset += self.data_width;
                }
                y += 1;
             }
         }

        return matrix;
    }

    pub fn  is_crop_supported(&self) -> bool  {
        return true;
    }

    pub fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> LuminanceSource  {
        return RGBLuminanceSource::new(&self.luminances, self.data_width, self.data_height, self.left + left, self.top + top, width, height);
    }
}

