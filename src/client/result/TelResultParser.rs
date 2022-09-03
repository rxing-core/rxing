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

// import com.google.zxing.RXingResult;

use super::{RXingResultParser, TelParsedRXingResult, ParsedRXingResult, ParsedClientResult, ResultParser};

/**
 * Parses a "tel:" URI result, which specifies a phone number.
 *
 * @author Sean Owen
 */
pub struct TelRXingResultParser  {}

impl RXingResultParser for TelRXingResultParser {

    fn parse(&self, theRXingResult: &crate::RXingResult) -> Option<ParsedClientResult> {
      let rawText = ResultParser::getMassagedText(theRXingResult);
      if !rawText.starts_with("tel:") && !rawText.starts_with("TEL:") {
        return None;
      }
      // Normalize "TEL:" to "tel:"
      let telURI = if rawText.starts_with("TEL:")  {format!("tel:{}", &rawText[4..])} else {rawText.clone()};
      // Drop tel, query portion
      let queryStart = rawText[4..].find('?');
      let number = if let Some(v) = queryStart {
        &rawText[4..v+4]
      }else {
        &rawText[4..]
      };
      // let number = queryStart < 0 ?  : ;
      Some(ParsedClientResult::TelResult(TelParsedRXingResult::new(number.to_owned(), telURI.to_owned(), "".to_owned())))
    }
}