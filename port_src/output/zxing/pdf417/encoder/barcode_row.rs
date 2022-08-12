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
 * @author Jacob Haynes
 */
struct BarcodeRow {

     let mut row: Vec<i8>;

    //A tacker for position in the bar
     let current_location: i32;
}

impl BarcodeRow {

    /**
   * Creates a Barcode row of the width
   */
    fn new( width: i32) -> BarcodeRow {
        let .row = : [i8; width] = [0; width];
        current_location = 0;
    }

    /**
   * Sets a specific location in the bar
   *
   * @param x The location in the bar
   * @param value Black if true, white if false;
   */
    fn  set(&self,  x: i32,  value: i8)   {
        self.row[x] = value;
    }

    /**
   * Sets a specific location in the bar
   *
   * @param x The location in the bar
   * @param black Black if true, white if false;
   */
    fn  set(&self,  x: i32,  black: bool)   {
        self.row[x] = ( if black { 1 } else { 0 }) as i8;
    }

    /**
   * @param black A boolean which is true if the bar black false if it is white
   * @param width How many spots wide the bar is.
   */
    fn  add_bar(&self,  black: bool,  width: i32)   {
         {
             let mut ii: i32 = 0;
            while ii < width {
                {
                    self.set(self.current_location += 1 !!!check!!! post increment, black);
                }
                ii += 1;
             }
         }

    }

    /**
   * This function scales the row
   *
   * @param scale How much you want the image to be scaled, must be greater than or equal to 1.
   * @return the scaled row
   */
    fn  get_scaled_row(&self,  scale: i32) -> Vec<i8>  {
         let mut output: [i8; self.row.len() * scale] = [0; self.row.len() * scale];
         {
             let mut i: i32 = 0;
            while i < output.len() {
                {
                    output[i] = self.row[i / scale];
                }
                i += 1;
             }
         }

        return output;
    }
}

