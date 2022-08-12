/*
 * Copyright 2022 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::google::zxing::pdf417::encoder;

pub struct PDF417HighLevelEncoderTestAdapter {
}

impl PDF417HighLevelEncoderTestAdapter {

    fn new() -> PDF417HighLevelEncoderTestAdapter {
    }

    pub fn  encode_high_level( msg: &String,  compaction: &Compaction,  encoding: &Charset,  auto_e_c_i: bool) -> /*  throws WriterException */Result<String, Rc<Exception>>   {
        return Ok(PDF417HighLevelEncoder::encode_high_level(&msg, compaction, &encoding, auto_e_c_i));
    }
}

