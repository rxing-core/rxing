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
// package com::google::zxing::pdf417::decoder;

/**
 * @author Guenther Grau
 */

 const BARCODE_ROW_UNKNOWN: i32 = -1;
struct Codeword {

     let start_x: i32;

     let end_x: i32;

     let bucket: i32;

     let value: i32;

     let row_number: i32 = BARCODE_ROW_UNKNOWN;
}

impl Codeword {

    fn new( start_x: i32,  end_x: i32,  bucket: i32,  value: i32) -> Codeword {
        let .startX = start_x;
        let .endX = end_x;
        let .bucket = bucket;
        let .value = value;
    }

    fn  has_valid_row_number(&self) -> bool  {
        return self.is_valid_row_number(self.row_number);
    }

    fn  is_valid_row_number(&self,  row_number: i32) -> bool  {
        return row_number != BARCODE_ROW_UNKNOWN && self.bucket == (row_number % 3) * 3;
    }

    fn  set_row_number_as_row_indicator_column(&self)   {
        self.row_number = (self.value / 30) * 3 + self.bucket / 3;
    }

    fn  get_width(&self) -> i32  {
        return self.end_x - self.start_x;
    }

    fn  get_start_x(&self) -> i32  {
        return self.start_x;
    }

    fn  get_end_x(&self) -> i32  {
        return self.end_x;
    }

    fn  get_bucket(&self) -> i32  {
        return self.bucket;
    }

    fn  get_value(&self) -> i32  {
        return self.value;
    }

    fn  get_row_number(&self) -> i32  {
        return self.row_number;
    }

    fn  set_row_number(&self,  row_number: i32)   {
        self.rowNumber = row_number;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{}|{}", self.row_number, self.value);
    }
}

