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
// package com::google::zxing::oned::rss;

/**
 * Encapsulates an RSS barcode finder pattern, including its start/end position and row.
 */
pub struct FinderPattern {

     let value: i32;

     let start_end: Vec<i32>;

     let result_points: Vec<ResultPoint>;
}

impl FinderPattern {

    pub fn new( value: i32,  start_end: &Vec<i32>,  start: i32,  end: i32,  row_number: i32) -> FinderPattern {
        let .value = value;
        let .startEnd = start_end;
        let .resultPoints =  : vec![ResultPoint; 2] = vec![ResultPoint::new(start, row_number), ResultPoint::new(end, row_number), ]
        ;
    }

    pub fn  get_value(&self) -> i32  {
        return self.value;
    }

    pub fn  get_start_end(&self) -> Vec<i32>  {
        return self.start_end;
    }

    pub fn  get_result_points(&self) -> Vec<ResultPoint>  {
        return self.result_points;
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof FinderPattern) {
            return false;
        }
         let that: FinderPattern = o as FinderPattern;
        return self.value == that.value;
    }

    pub fn  hash_code(&self) -> i32  {
        return self.value;
    }
}

