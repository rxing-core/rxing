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
 * Parses the "URLTO" result format, which is of the form "URLTO:[title]:[url]".
 * This seems to be used sometimes, but I am not able to find documentation
 * on its origin or official format?
 *
 * @author Sean Owen
 */
pub struct URLTOResultParser {
    super: ResultParser;
}

impl URLTOResultParser {

    pub fn  parse(&self,  result: &Result) -> URIParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("urlto:") && !raw_text.starts_with("URLTO:") {
            return null;
        }
         let title_end: i32 = raw_text.index_of(':', 6);
        if title_end < 0 {
            return null;
        }
         let title: String =  if title_end <= 6 { null } else { raw_text.substring(6, title_end) };
         let uri: String = raw_text.substring(title_end + 1);
        return URIParsedResult::new(&uri, &title);
    }
}

