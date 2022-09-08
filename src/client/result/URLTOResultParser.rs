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

use crate::RXingResult;

use super::{ParsedClientResult, ResultParser, URIParsedRXingResult};

/**
 * Parses the "URLTO" result format, which is of the form "URLTO:[title]:[url]".
 * This seems to be used sometimes, but I am not able to find documentation
 * on its origin or official format?
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    if !rawText.starts_with("urlto:") && !rawText.starts_with("URLTO:") {
        return None;
    }
    let titleEnd = if let Some(pos) = rawText[6..].find(':') {
        pos + 6
    } else {
        return None;
    };
    let title = if titleEnd <= 6 {
        ""
    } else {
        &rawText[6..titleEnd]
    };
    let uri = &rawText[titleEnd + 1..];

    Some(ParsedClientResult::URIResult(URIParsedRXingResult::new(
        uri.to_owned(),
        title.to_owned(),
    )))
}

// }
