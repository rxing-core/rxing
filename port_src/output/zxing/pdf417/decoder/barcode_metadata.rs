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
struct BarcodeMetadata {

     let column_count: i32;

     let error_correction_level: i32;

     let row_count_upper_part: i32;

     let row_count_lower_part: i32;

     let row_count: i32;
}

impl BarcodeMetadata {

    fn new( column_count: i32,  row_count_upper_part: i32,  row_count_lower_part: i32,  error_correction_level: i32) -> BarcodeMetadata {
        let .columnCount = column_count;
        let .errorCorrectionLevel = error_correction_level;
        let .rowCountUpperPart = row_count_upper_part;
        let .rowCountLowerPart = row_count_lower_part;
        let .rowCount = row_count_upper_part + row_count_lower_part;
    }

    fn  get_column_count(&self) -> i32  {
        return self.column_count;
    }

    fn  get_error_correction_level(&self) -> i32  {
        return self.error_correction_level;
    }

    fn  get_row_count(&self) -> i32  {
        return self.row_count;
    }

    fn  get_row_count_upper_part(&self) -> i32  {
        return self.row_count_upper_part;
    }

    fn  get_row_count_lower_part(&self) -> i32  {
        return self.row_count_lower_part;
    }
}

