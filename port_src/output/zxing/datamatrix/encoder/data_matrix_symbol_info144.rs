/*
 * Copyright 2006 Jeremias Maerki
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
// package com::google::zxing::datamatrix::encoder;

struct DataMatrixSymbolInfo144 {
    super: SymbolInfo;
}

impl DataMatrixSymbolInfo144 {

    fn new() -> DataMatrixSymbolInfo144 {
        super(false, 1558, 620, 22, 22, 36, -1, 62);
    }

    pub fn  get_interleaved_block_count(&self) -> i32  {
        return 10;
    }

    pub fn  get_data_length_for_interleaved_block(&self,  index: i32) -> i32  {
        return  if (index <= 8) { 156 } else { 155 };
    }
}

