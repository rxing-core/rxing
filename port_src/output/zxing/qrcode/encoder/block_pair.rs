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
// package com::google::zxing::qrcode::encoder;

struct BlockPair {

     let data_bytes: Vec<i8>;

     let error_correction_bytes: Vec<i8>;
}

impl BlockPair {

    fn new( data: &Vec<i8>,  error_correction: &Vec<i8>) -> BlockPair {
        data_bytes = data;
        error_correction_bytes = error_correction;
    }

    pub fn  get_data_bytes(&self) -> Vec<i8>  {
        return self.data_bytes;
    }

    pub fn  get_error_correction_bytes(&self) -> Vec<i8>  {
        return self.error_correction_bytes;
    }
}

