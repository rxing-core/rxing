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
 * A simple result type encapsulating a URI that has no further interpretation.
 *
 * @author Sean Owen
 */
pub struct URIParsedResult {
    super: ParsedResult;

     let uri: String;

     let title: String;
}

impl URIParsedResult {

    pub fn new( uri: &String,  title: &String) -> URIParsedResult {
        super(ParsedResultType::URI);
        let .uri = ::massage_u_r_i(&uri);
        let .title = title;
    }

    pub fn  get_u_r_i(&self) -> String  {
        return self.uri;
    }

    pub fn  get_title(&self) -> String  {
        return self.title;
    }

    /**
   * @return true if the URI contains suspicious patterns that may suggest it intends to
   *  mislead the user about its true nature
   * @deprecated see {@link URIResultParser#isPossiblyMaliciousURI(String)}
   */
    pub fn  is_possibly_malicious_u_r_i(&self) -> bool  {
        return URIResultParser::is_possibly_malicious_u_r_i(&self.uri);
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(30);
        maybe_append(&self.title, &result);
        maybe_append(&self.uri, &result);
        return result.to_string();
    }

    /**
   * Transforms a string that represents a URI into something more proper, by adding or canonicalizing
   * the protocol.
   */
    fn  massage_u_r_i( uri: &String) -> String  {
        uri = uri.trim();
         let protocol_end: i32 = uri.index_of(':');
        if protocol_end < 0 || ::is_colon_followed_by_port_number(&uri, protocol_end) {
            // No protocol, or found a colon, but it looks like it is after the host, so the protocol is still missing,
            // so assume http
            uri = format!("http://{}", uri);
        }
        return uri;
    }

    fn  is_colon_followed_by_port_number( uri: &String,  protocol_end: i32) -> bool  {
         let start: i32 = protocol_end + 1;
         let next_slash: i32 = uri.index_of('/', start);
        if next_slash < 0 {
            next_slash = uri.length();
        }
        return ResultParser::is_substring_of_digits(&uri, start, next_slash - start);
    }
}

