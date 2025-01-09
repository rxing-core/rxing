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

// package com.google.zxing.client.result;

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.RXingResult;

use crate::BarcodeFormat;

use super::{ISBNParsedRXingResult, ParsedClientResult, ResultParser};

/**
 * Parses strings of digits that represent a ISBN.
 *
 * @author jbreiden@google.com (Jeff Breidenbach)
 */
pub fn parse(theRXingResult: &crate::RXingResult) -> Option<super::ParsedClientResult> {
    let format = theRXingResult.getBarcodeFormat();
    if *format != BarcodeFormat::EAN_13 {
        return None;
    }
    let rawText = ResultParser::getMassagedText(theRXingResult);
    let length = rawText.len();
    if length != 13 {
        return None;
    }
    if !rawText.starts_with("978") && !rawText.starts_with("979") {
        return None;
    }

    Some(ParsedClientResult::ISBNResult(ISBNParsedRXingResult::new(
        rawText,
    )))
}
// }
