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
// package com::google::zxing::oned::rss::expanded::decoders;

/**
 * @author Pablo OrduÃ±a, University of Deusto (pablo.orduna@deusto.es)
 */
struct AI01weightDecoder {
    super: AI01decoder;
}

impl AI01weightDecoder {

    fn new( information: &BitArray) -> AI01weightDecoder {
        super(information);
    }

    fn  encode_compressed_weight(&self,  buf: &StringBuilder,  current_pos: i32,  weight_size: i32)   {
         let original_weight_numeric: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(current_pos, weight_size);
        self.add_weight_code(&buf, original_weight_numeric);
         let weight_numeric: i32 = self.check_weight(original_weight_numeric);
         let current_divisor: i32 = 100000;
         {
             let mut i: i32 = 0;
            while i < 5 {
                {
                    if weight_numeric / current_divisor == 0 {
                        buf.append('0');
                    }
                    current_divisor /= 10;
                }
                i += 1;
             }
         }

        buf.append(weight_numeric);
    }

    pub fn  add_weight_code(&self,  buf: &StringBuilder,  weight: i32)  ;

    pub fn  check_weight(&self,  weight: i32) -> i32 ;
}

