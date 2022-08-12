/*
 * Copyright 2007 ZXing authors
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
 * @author Sean Owen
 */
pub struct BookmarkDoCoMoResultParser {
    super: AbstractDoCoMoResultParser;
}

impl BookmarkDoCoMoResultParser {

    pub fn  parse(&self,  result: &Result) -> URIParsedResult  {
         let raw_text: String = result.get_text();
        if !raw_text.starts_with("MEBKM:") {
            return null;
        }
         let title: String = match_single_do_co_mo_prefixed_field("TITLE:", &raw_text, true);
         let raw_uri: Vec<String> = match_do_co_mo_prefixed_field("URL:", &raw_text);
        if raw_uri == null {
            return null;
        }
         let uri: String = raw_uri[0];
        return  if URIResultParser::is_basically_valid_u_r_i(&uri) { URIParsedResult::new(&uri, &title) } else { null };
    }
}

