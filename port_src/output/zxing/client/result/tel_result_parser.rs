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
 * Parses a "tel:" URI result, which specifies a phone number.
 *
 * @author Sean Owen
 */
pub struct TelResultParser {
    super: ResultParser;
}

impl TelResultParser {

    pub fn  parse(&self,  result: &Result) -> TelParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("tel:") && !raw_text.starts_with("TEL:") {
            return null;
        }
        // Normalize "TEL:" to "tel:"
         let tel_u_r_i: String =  if raw_text.starts_with("TEL:") { format!("tel:{}", raw_text.substring(4)) } else { raw_text };
        // Drop tel, query portion
         let query_start: i32 = raw_text.index_of('?', 4);
         let number: String =  if query_start < 0 { raw_text.substring(4) } else { raw_text.substring(4, query_start) };
        return TelParsedResult::new(&number, &tel_u_r_i, null);
    }
}

