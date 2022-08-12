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
struct AI013103decoder {
    super: AI013x0xDecoder;
}

impl AI013103decoder {

    fn new( information: &BitArray) -> AI013103decoder {
        super(information);
    }

    pub fn  add_weight_code(&self,  buf: &StringBuilder,  weight: i32)   {
        buf.append("(3103)");
    }

    pub fn  check_weight(&self,  weight: i32) -> i32  {
        return weight;
    }
}

