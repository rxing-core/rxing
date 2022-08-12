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
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */

 const GTIN_SIZE: i32 = 40;
struct AI01decoder {
    super: AbstractExpandedDecoder;
}

impl AI01decoder {

    fn new( information: &BitArray) -> AI01decoder {
        super(information);
    }

    fn  encode_compressed_gtin(&self,  buf: &StringBuilder,  current_pos: i32)   {
        buf.append("(01)");
         let initial_position: i32 = buf.length();
        buf.append('9');
        self.encode_compressed_gtin_without_a_i(&buf, current_pos, initial_position);
    }

    fn  encode_compressed_gtin_without_a_i(&self,  buf: &StringBuilder,  current_pos: i32,  initial_buffer_position: i32)   {
         {
             let mut i: i32 = 0;
            while i < 4 {
                {
                     let current_block: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(current_pos + 10 * i, 10);
                    if current_block / 100 == 0 {
                        buf.append('0');
                    }
                    if current_block / 10 == 0 {
                        buf.append('0');
                    }
                    buf.append(current_block);
                }
                i += 1;
             }
         }

        ::append_check_digit(&buf, initial_buffer_position);
    }

    fn  append_check_digit( buf: &StringBuilder,  current_pos: i32)   {
         let check_digit: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 13 {
                {
                     let digit: i32 = buf.char_at(i + current_pos) - '0';
                    check_digit +=  if (i & 0x01) == 0 { 3 * digit } else { digit };
                }
                i += 1;
             }
         }

        check_digit = 10 - (check_digit % 10);
        if check_digit == 10 {
            check_digit = 0;
        }
        buf.append(check_digit);
    }
}

