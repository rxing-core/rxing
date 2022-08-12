/*
 * Copyright (C) 2010 ZXing authors
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
// package com::google::zxing::oned;


 const EXTENSION_START_PATTERN: vec![Vec<i32>; 3] = vec![1, 1, 2, ]
;
struct UPCEANExtensionSupport {

     let two_support: UPCEANExtension2Support = UPCEANExtension2Support::new();

     let five_support: UPCEANExtension5Support = UPCEANExtension5Support::new();
}

impl UPCEANExtensionSupport {

    fn  decode_row(&self,  row_number: i32,  row: &BitArray,  row_offset: i32) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
         let extension_start_range: Vec<i32> = UPCEANReader::find_guard_pattern(row, row_offset, false, &EXTENSION_START_PATTERN);
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(self.five_support.decode_row(row_number, row, &extension_start_range));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &ReaderException) {
                return Ok(self.two_support.decode_row(row_number, row, &extension_start_range));
            }  0 => break
        }

    }
}

