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

//first bit encodes the linkage flag,
 const HEADER_SIZE: i32 = 1 + 1 + 2;
struct AI01AndOtherAIs {
    super: AI01decoder;
}

impl AI01AndOtherAIs {

    //the second one is the encodation method, and the other two are for the variable length
    fn new( information: &BitArray) -> AI01AndOtherAIs {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
         let buff: StringBuilder = StringBuilder::new();
        buff.append("(01)");
         let initial_gtin_position: i32 = buff.length();
         let first_gtin_digit: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(HEADER_SIZE, 4);
        buff.append(first_gtin_digit);
        self.encode_compressed_gtin_without_a_i(&buff, HEADER_SIZE + 4, initial_gtin_position);
        return Ok(self.get_general_decoder().decode_all_codes(&buff, HEADER_SIZE + 44));
    }
}

