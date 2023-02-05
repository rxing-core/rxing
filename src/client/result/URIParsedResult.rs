/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, V&ersion 2.0 (the "License");
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

// package com.google.zxing.client.result;

use super::{ParsedRXingResult, ParsedRXingResultType, ResultParser, URIResultParser};

/**
 * A simple result type encapsulating a URI that has no further interpretation.
 *
 * @author Sean Owen
 */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct URIParsedRXingResult {
    uri: String,
    title: String,
}
impl ParsedRXingResult for URIParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::URI
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(30);
        ResultParser::maybe_append_string(&self.title, &mut result);
        ResultParser::maybe_append_string(&self.uri, &mut result);
        result
    }
}
impl URIParsedRXingResult {
    pub fn new(uri: String, title: String) -> Self {
        Self {
            uri: Self::massage_uri(&uri),
            title,
        }
    }

    pub fn getURI(&self) -> &str {
        &self.uri
    }

    pub fn getTitle(&self) -> &str {
        &self.title
    }

    /**
     * @return true if the URI contains suspicious patterns that may suggest it intends to
     *  mislead the user about its true nature
     * @deprecated see {@link URIRXingResultParser#isPossiblyMaliciousURI(String)}
     */
    #[deprecated]
    pub fn is_possibly_malicious_uri(&self) -> bool {
        URIResultParser::is_possibly_malicious_uri(&self.uri)
    }

    /**
     * Transforms a string that represents a URI into something more proper, by adding or canonicalizing
     * the protocol.
     */
    fn massage_uri(uri: &str) -> String {
        let mut uri = String::from(uri.trim());
        // let protocolEnd = uri.find(':');
        if let Some(protocolEnd) = uri.find(':') {
            if Self::is_colon_followed_by_port_number(&uri, protocolEnd) {
                // No protocol, or found a colon, but it looks like it is after the host, so the protocol is still missing,
                // so assume http
                uri = format!("http://{}", &uri);
                // uri = updated_uri.as_str()
            }
        } else {
            uri = format!("http://{}", &uri);
        }

        uri
    }

    fn is_colon_followed_by_port_number(uri: &str, protocol_end: usize) -> bool {
        let start = protocol_end + 1;
        let nextSlash = if let Some(pos) = uri[start..].find('/') {
            pos + start
        } else {
            uri.chars().count()
        };
        // let nextSlash = uri.indexOf('/', start);
        // if (nextSlash < 0) {
        //   nextSlash = uri.length();
        // }
        ResultParser::isSubstringOfDigits(uri, start, nextSlash - start)
    }
}
