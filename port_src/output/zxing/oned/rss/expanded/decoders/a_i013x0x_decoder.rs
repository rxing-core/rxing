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

 const HEADER_SIZE: i32 = 4 + 1;

 const WEIGHT_SIZE: i32 = 15;
struct AI013x0xDecoder {
    super: AI01weightDecoder;
}

impl AI013x0xDecoder {

    fn new( information: &BitArray) -> AI013x0xDecoder {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if self.get_information().get_size() != HEADER_SIZE + GTIN_SIZE + WEIGHT_SIZE {
            throw NotFoundException::get_not_found_instance();
        }
         let buf: StringBuilder = StringBuilder::new();
        encode_compressed_gtin(&buf, HEADER_SIZE);
        encode_compressed_weight(&buf, HEADER_SIZE + GTIN_SIZE, WEIGHT_SIZE);
        return Ok(buf.to_string());
    }
}

