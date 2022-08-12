/*
 * Copyright 2012 ZXing authors
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
 * Data object to specify the minimum and maximum number of rows and columns for a PDF417 barcode.
 *
 * @author qwandor@google.com (Andrew Walbran)
 */
pub struct Dimensions {

     let min_cols: i32;

     let max_cols: i32;

     let min_rows: i32;

     let max_rows: i32;
}

impl Dimensions {

    pub fn new( min_cols: i32,  max_cols: i32,  min_rows: i32,  max_rows: i32) -> Dimensions {
        let .minCols = min_cols;
        let .maxCols = max_cols;
        let .minRows = min_rows;
        let .maxRows = max_rows;
    }

    pub fn  get_min_cols(&self) -> i32  {
        return self.min_cols;
    }

    pub fn  get_max_cols(&self) -> i32  {
        return self.max_cols;
    }

    pub fn  get_min_rows(&self) -> i32  {
        return self.min_rows;
    }

    pub fn  get_max_rows(&self) -> i32  {
        return self.max_rows;
    }
}

