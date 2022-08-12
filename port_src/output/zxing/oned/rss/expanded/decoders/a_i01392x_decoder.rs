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

 const HEADER_SIZE: i32 = 5 + 1 + 2;

 const LAST_DIGIT_SIZE: i32 = 2;
struct AI01392xDecoder {
    super: AI01decoder;
}

impl AI01392xDecoder {

    fn new( information: &BitArray) -> AI01392xDecoder {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
        if self.get_information().get_size() < HEADER_SIZE + GTIN_SIZE {
            throw NotFoundException::get_not_found_instance();
        }
         let buf: StringBuilder = StringBuilder::new();
        encode_compressed_gtin(&buf, HEADER_SIZE);
         let last_a_idigit: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(HEADER_SIZE + GTIN_SIZE, LAST_DIGIT_SIZE);
        buf.append("(392");
        buf.append(last_a_idigit);
        buf.append(')');
         let decoded_information: DecodedInformation = self.get_general_decoder().decode_general_purpose_field(HEADER_SIZE + GTIN_SIZE + LAST_DIGIT_SIZE, null);
        buf.append(&decoded_information.get_new_string());
        return Ok(buf.to_string());
    }
}

