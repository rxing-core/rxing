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
// package com::google::zxing;

/**
 * A wrapper implementation of {@link LuminanceSource} which inverts the luminances it returns -- black becomes
 * white and vice versa, and each value becomes (255-value).
 *
 * @author Sean Owen
 */
pub struct InvertedLuminanceSource {
    super: LuminanceSource;

     let delegate: LuminanceSource;
}

impl InvertedLuminanceSource {

    pub fn new( delegate: &LuminanceSource) -> InvertedLuminanceSource {
        super(&delegate.get_width(), &delegate.get_height());
        let .delegate = delegate;
    }

    pub fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8>  {
        row = self.delegate.get_row(y, &row);
         let width: i32 = get_width();
         {
             let mut i: i32 = 0;
            while i < width {
                {
                    row[i] = (255 - (row[i] & 0xFF)) as i8;
                }
                i += 1;
             }
         }

        return row;
    }

    pub fn  get_matrix(&self) -> Vec<i8>  {
         let matrix: Vec<i8> = self.delegate.get_matrix();
         let length: i32 = get_width() * get_height();
         let inverted_matrix: [i8; length] = [0; length];
         {
             let mut i: i32 = 0;
            while i < length {
                {
                    inverted_matrix[i] = (255 - (matrix[i] & 0xFF)) as i8;
                }
                i += 1;
             }
         }

        return inverted_matrix;
    }

    pub fn  is_crop_supported(&self) -> bool  {
        return self.delegate.is_crop_supported();
    }

    pub fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> LuminanceSource  {
        return InvertedLuminanceSource::new(&self.delegate.crop(left, top, width, height));
    }

    pub fn  is_rotate_supported(&self) -> bool  {
        return self.delegate.is_rotate_supported();
    }

    /**
   * @return original delegate {@link LuminanceSource} since invert undoes itself
   */
    pub fn  invert(&self) -> LuminanceSource  {
        return self.delegate;
    }

    pub fn  rotate_counter_clockwise(&self) -> LuminanceSource  {
        return InvertedLuminanceSource::new(&self.delegate.rotate_counter_clockwise());
    }

    pub fn  rotate_counter_clockwise45(&self) -> LuminanceSource  {
        return InvertedLuminanceSource::new(&self.delegate.rotate_counter_clockwise45());
    }
}

