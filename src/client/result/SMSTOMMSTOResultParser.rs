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

use crate::RXingResult;

use super::{ParsedClientResult, ResultParser, SMSParsedRXingResult};

/**
 * <p>Parses an "smsto:" URI result, whose format is not standardized but appears to be like:
 * {@code smsto:number(:body)}.</p>
 *
 * <p>This actually also parses URIs starting with "smsto:", "mmsto:", "SMSTO:", and
 * "MMSTO:", and treats them all the same way, and effectively converts them to an "sms:" URI
 * for purposes of forwarding to the platform.</p>
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    if !(rawText.starts_with("smsto:")
        || rawText.starts_with("SMSTO:")
        || rawText.starts_with("mmsto:")
        || rawText.starts_with("MMSTO:"))
    {
        return None;
    }
    // Thanks to dominik.wild for suggesting this enhancement to support
    // smsto:number:body URIs
    let mut number = &rawText[6..];
    let mut body = "";
    if let Some(body_start) = number.find(':') {
        body = &number[body_start + 1..];
        number = &number[..body_start];
    }

    Some(ParsedClientResult::SMSResult(
        SMSParsedRXingResult::with_singles(
            number.to_owned(),
            String::default(),
            String::default(),
            body.to_owned(),
        ),
    ))
}
