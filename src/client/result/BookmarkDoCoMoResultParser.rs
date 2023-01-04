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

use super::{ParsedClientResult, ResultParser, URIParsedRXingResult, URIResultParser};

/**
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = result.getText();
    if !rawText.starts_with("MEBKM:") {
        return None;
    }
    let title = ResultParser::match_single_do_co_mo_prefixed_field("TITLE:", rawText, true);
    let rawUri = ResultParser::match_do_co_mo_prefixed_field("URL:", rawText);
    rawUri.as_ref()?;
    let uri = rawUri?[0].clone();
    if URIResultParser::is_basically_valid_uri(&uri) {
        Some(ParsedClientResult::URIResult(URIParsedRXingResult::new(
            uri,
            title.unwrap_or_else(|| "".to_owned()),
        )))
    } else {
        None
    }

    // return URIRXingResultParser.isBasicallyValidURI(uri) ? new URIParsedRXingResult(uri, title) : null;
}

// }
