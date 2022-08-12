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
// package com::google::zxing::oned;

/**
 * <p>Implements decoding of the EAN-8 format.</p>
 *
 * @author Sean Owen
 */
pub struct EAN8Reader {
    super: UPCEANReader;

     let decode_middle_counters: Vec<i32>;
}

impl EAN8Reader {

    pub fn new() -> EAN8Reader {
        decode_middle_counters = : [i32; 4] = [0; 4];
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.decode_middle_counters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let end: i32 = row.get_size();
         let row_offset: i32 = start_range[1];
         {
             let mut x: i32 = 0;
            while x < 4 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_PATTERNS);
                    result.append(('0' + best_match) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                }
                x += 1;
             }
         }

         let middle_range: Vec<i32> = find_guard_pattern(row, row_offset, true, MIDDLE_PATTERN);
        row_offset = middle_range[1];
         {
             let mut x: i32 = 0;
            while x < 4 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_PATTERNS);
                    result.append(('0' + best_match) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                }
                x += 1;
             }
         }

        return Ok(row_offset);
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::EAN_8;
    }
}

