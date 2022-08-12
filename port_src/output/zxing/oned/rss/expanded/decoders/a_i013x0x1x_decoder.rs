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

 const HEADER_SIZE: i32 = 7 + 1;

 const WEIGHT_SIZE: i32 = 20;

 const DATE_SIZE: i32 = 16;
struct AI013x0x1xDecoder {
    super: AI01weightDecoder;

     let date_code: String;

     let first_a_idigits: String;
}

impl AI013x0x1xDecoder {

    fn new( information: &BitArray,  first_a_idigits: &String,  date_code: &String) -> AI013x0x1xDecoder {
        super(information);
        let .dateCode = date_code;
        let .firstAIdigits = first_a_idigits;
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if self.get_information().get_size() != HEADER_SIZE + GTIN_SIZE + WEIGHT_SIZE + DATE_SIZE {
            throw NotFoundException::get_not_found_instance();
        }
         let buf: StringBuilder = StringBuilder::new();
        encode_compressed_gtin(&buf, HEADER_SIZE);
        encode_compressed_weight(&buf, HEADER_SIZE + GTIN_SIZE, WEIGHT_SIZE);
        self.encode_compressed_date(&buf, HEADER_SIZE + GTIN_SIZE + WEIGHT_SIZE);
        return Ok(buf.to_string());
    }

    fn  encode_compressed_date(&self,  buf: &StringBuilder,  current_pos: i32)   {
         let numeric_date: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(current_pos, DATE_SIZE);
        if numeric_date == 38400 {
            return;
        }
        buf.append('(');
        buf.append(self.dateCode);
        buf.append(')');
         let day: i32 = numeric_date % 32;
        numeric_date /= 32;
         let month: i32 = numeric_date % 12 + 1;
        numeric_date /= 12;
         let year: i32 = numeric_date;
        if year / 10 == 0 {
            buf.append('0');
        }
        buf.append(year);
        if month / 10 == 0 {
            buf.append('0');
        }
        buf.append(month);
        if day / 10 == 0 {
            buf.append('0');
        }
        buf.append(day);
    }

    pub fn  add_weight_code(&self,  buf: &StringBuilder,  weight: i32)   {
        buf.append('(');
        buf.append(self.firstAIdigits);
        buf.append(weight / 100000);
        buf.append(')');
    }

    pub fn  check_weight(&self,  weight: i32) -> i32  {
        return weight % 100000;
    }
}

