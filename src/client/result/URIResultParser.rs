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

// package com.google.zxing.client.result;

// import com.google.zxing.RXingResult;

// import java.util.regex.Matcher;
// import java.util.regex.Pattern;

use lazy_static::lazy_static;
/**
 * Tries to parse results that are a URI of some kind.
 *
 * @author Sean Owen
 */
// public final class URIRXingResultParser extends RXingResultParser {
use regex::Regex;

use crate::RXingResult;

use super::{ParsedClientResult, ResultParser, URIParsedRXingResult};

lazy_static! {
    static ref ALLOWED_URI_CHARS: Regex =
        Regex::new(ALLOWED_URI_CHARS_PATTERN).expect("Regex patterns should always copile");
    static ref USER_IN_HOST: Regex =
        Regex::new(":/*([^/@]+)@[^/]+").expect("Regex patterns should always copile");
    static ref URL_WITH_PROTOCOL_PATTERN: Regex = Regex::new("[a-zA-Z][a-zA-Z0-9+-.]+:").unwrap();
    static ref URL_WITHOUT_PROTOCOL_PATTERN: Regex =
        Regex::new("([a-zA-Z0-9\\-]+\\.){1,6}[a-zA-Z]{2,}(:\\d{1,5})?(/|\\?|$)").unwrap();
}

const ALLOWED_URI_CHARS_PATTERN: &'static str = "[-._~:/?#\\[\\]@!$&'()*+,;=%A-Za-z0-9]+";
// const USER_IN_HOST: &'static str = ":/*([^/@]+)@[^/]+";
/// See http://www.ietf.org/rfc/rfc2396.txt
// const URL_WITH_PROTOCOL_PATTERN: &'static str = "[a-zA-Z][a-zA-Z0-9+-.]+:";
/// (host name elements; allow up to say 6 domain elements), (maybe port), (query, path or nothing)
// const URL_WITHOUT_PROTOCOL_PATTERN: &'static str =
//     "([a-zA-Z0-9\\-]+\\.){1,6}[a-zA-Z]{2,}(:\\d{1,5})?(/|\\?|$)";

pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let raw_text = ResultParser::getMassagedText(result);
    // We specifically handle the odd "URL" scheme here for simplicity and add "URI" for fun
    // Assume anything starting this way really means to be a URI
    if raw_text.starts_with("URL:") || raw_text.starts_with("URI:") {
        return Some(ParsedClientResult::URIResult(URIParsedRXingResult::new(
            raw_text[4..].trim().to_owned(),
            "".to_owned(),
        )));
        // return new URIParsedRXingResult(rawText.substring(4).trim(), null);
    }
    let raw_text = raw_text.trim();
    if !is_basically_valid_uri(raw_text) || is_possibly_malicious_uri(raw_text) {
        return None;
    }
    Some(ParsedClientResult::URIResult(URIParsedRXingResult::new(
        raw_text.to_owned(),
        "".to_owned(),
    )))
}

/**
 * @return true if the URI contains suspicious patterns that may suggest it intends to
 *  mislead the user about its true nature. At the moment this looks for the presence
 *  of user/password syntax in the host/authority portion of a URI which may be used
 *  in attempts to make the URI's host appear to be other than it is. Example:
 *  http://yourbank.com@phisher.com  This URI connects to phisher.com but may appear
 *  to connect to yourbank.com at first glance.
 */
pub fn is_possibly_malicious_uri(uri: &str) -> bool {
    let allowed = if let Some(fnd) = ALLOWED_URI_CHARS.find(uri) {
        if fnd.start() == 0 && fnd.end() == uri.len() {
            true
        } else {
            false
        }
    } else {
        false
    };
    let user = USER_IN_HOST.is_match(uri);

    !allowed || user
}

pub fn is_basically_valid_uri(uri: &str) -> bool {
    if uri.contains(" ") {
        // Quick hack check for a common case
        return false;
    }
    // let m = Regex::new(URL_WITH_PROTOCOL_PATTERN).expect("Regex patterns should always copile"); //.matcher(uri);
    if let Some(found) = URL_WITH_PROTOCOL_PATTERN.find(uri) {
        if found.start() == 0 {
            // match at start only
            return true;
        }
    }

    // let m = Regex::new(URL_WITHOUT_PROTOCOL_PATTERN).expect("Regex patterns should always copile"); //.matcher(uri);
    if let Some(found) = URL_WITHOUT_PROTOCOL_PATTERN.find(uri) {
        if found.start() == 0 {
            // match at start only
            true
        } else {
            false
        }
    } else {
        false
    }
}

// }
