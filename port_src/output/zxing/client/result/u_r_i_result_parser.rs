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
 * Tries to parse results that are a URI of some kind.
 * 
 * @author Sean Owen
 */

 const ALLOWED_URI_CHARS_PATTERN: Pattern = Pattern::compile("[-._~:/?#\\[\\]@!$&'()*+,;=%A-Za-z0-9]+");

 const USER_IN_HOST: Pattern = Pattern::compile(":/*([^/@]+)@[^/]+");

// See http://www.ietf.org/rfc/rfc2396.txt
 const URL_WITH_PROTOCOL_PATTERN: Pattern = Pattern::compile("[a-zA-Z][a-zA-Z0-9+-.]+:");

 const URL_WITHOUT_PROTOCOL_PATTERN: Pattern = Pattern::compile(format!("([a-zA-Z0-9\\-]+\\.){1,6}[a-zA-Z]{2,}(:\\d{1,5})?(/|\\?|$)"));
pub struct URIResultParser {
    super: ResultParser;
}

impl URIResultParser {

    pub fn  parse(&self,  result: &Result) -> URIParsedResult  {
         let raw_text: String = get_massaged_text(result);
        // Assume anything starting this way really means to be a URI
        if raw_text.starts_with("URL:") || raw_text.starts_with("URI:") {
            return URIParsedResult::new(&raw_text.substring(4).trim(), null);
        }
        raw_text = raw_text.trim();
        if !::is_basically_valid_u_r_i(&raw_text) || ::is_possibly_malicious_u_r_i(&raw_text) {
            return null;
        }
        return URIParsedResult::new(&raw_text, null);
    }

    /**
   * @return true if the URI contains suspicious patterns that may suggest it intends to
   *  mislead the user about its true nature. At the moment this looks for the presence
   *  of user/password syntax in the host/authority portion of a URI which may be used
   *  in attempts to make the URI's host appear to be other than it is. Example:
   *  http://yourbank.com@phisher.com  This URI connects to phisher.com but may appear
   *  to connect to yourbank.com at first glance.
   */
    fn  is_possibly_malicious_u_r_i( uri: &String) -> bool  {
        return !ALLOWED_URI_CHARS_PATTERN::matcher(&uri)::matches() || USER_IN_HOST::matcher(&uri)::find();
    }

    fn  is_basically_valid_u_r_i( uri: &String) -> bool  {
        if uri.contains(" ") {
            // Quick hack check for a common case
            return false;
        }
         let mut m: Matcher = URL_WITH_PROTOCOL_PATTERN::matcher(&uri);
        if m.find() && m.start() == 0 {
            // match at start only
            return true;
        }
        m = URL_WITHOUT_PROTOCOL_PATTERN::matcher(&uri);
        return m.find() && m.start() == 0;
    }
}

