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
// package com::google::zxing::client::result;

/**
 * <p>Parses an "smsto:" URI result, whose format is not standardized but appears to be like:
 * {@code smsto:number(:body)}.</p>
 *
 * <p>This actually also parses URIs starting with "smsto:", "mmsto:", "SMSTO:", and
 * "MMSTO:", and treats them all the same way, and effectively converts them to an "sms:" URI
 * for purposes of forwarding to the platform.</p>
 *
 * @author Sean Owen
 */
pub struct SMSTOMMSTOResultParser {
    super: ResultParser;
}

impl SMSTOMMSTOResultParser {

    pub fn  parse(&self,  result: &Result) -> SMSParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !(raw_text.starts_with("smsto:") || raw_text.starts_with("SMSTO:") || raw_text.starts_with("mmsto:") || raw_text.starts_with("MMSTO:")) {
            return null;
        }
        // Thanks to dominik.wild for suggesting this enhancement to support
        // smsto:number:body URIs
         let mut number: String = raw_text.substring(6);
         let mut body: String = null;
         let body_start: i32 = number.index_of(':');
        if body_start >= 0 {
            body = number.substring(body_start + 1);
            number = number.substring(0, body_start);
        }
        return SMSParsedResult::new(&number, null, null, &body);
    }
}

