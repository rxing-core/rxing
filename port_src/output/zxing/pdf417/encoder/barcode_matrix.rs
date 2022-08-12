/*
 * Copyright 2011 ZXing authors
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
// package com::google::zxing::pdf417::encoder;

/**
 * Holds all of the information for a barcode in a format where it can be easily accessible
 *
 * @author Jacob Haynes
 */
pub struct BarcodeMatrix {

     let mut matrix: Vec<BarcodeRow>;

     let current_row: i32;

     let height: i32;

     let width: i32;
}

impl BarcodeMatrix {

    /**
   * @param height the height of the matrix (Rows)
   * @param width  the width of the matrix (Cols)
   */
    fn new( height: i32,  width: i32) -> BarcodeMatrix {
        matrix = : [Option<BarcodeRow>; height] = [None; height];
        //Initializes the array to the correct width
         {
             let mut i: i32 = 0, let matrix_length: i32 = matrix.len();
            while i < matrix_length {
                {
                    matrix[i] = BarcodeRow::new((width + 4) * 17 + 1);
                }
                i += 1;
             }
         }

        let .width = width * 17;
        let .height = height;
        let .currentRow = -1;
    }

    fn  set(&self,  x: i32,  y: i32,  value: i8)   {
        self.matrix[y].set(x, value);
    }

    fn  start_row(&self)   {
        self.current_row += 1;
    }

    fn  get_current_row(&self) -> BarcodeRow  {
        return self.matrix[self.current_row];
    }

    pub fn  get_matrix(&self) -> Vec<Vec<i8>>  {
        return self.get_scaled_matrix(1, 1);
    }

    pub fn  get_scaled_matrix(&self,  x_scale: i32,  y_scale: i32) -> Vec<Vec<i8>>  {
         let matrix_out: [[i8; self.width * x_scale]; self.height * y_scale] = [[0; self.width * x_scale]; self.height * y_scale];
         let y_max: i32 = self.height * y_scale;
         {
             let mut i: i32 = 0;
            while i < y_max {
                {
                    matrix_out[y_max - i - 1] = self.matrix[i / y_scale].get_scaled_row(x_scale);
                }
                i += 1;
             }
         }

        return matrix_out;
    }
}

