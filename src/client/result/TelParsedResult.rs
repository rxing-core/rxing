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

use super::{ParsedRXingResult, ParsedRXingResultType};

/**
 * Represents a parsed result that encodes a telephone number.
 *
 * @author Sean Owen
 */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TelParsedRXingResult {
    number: String,
    telURI: String,
    title: String,
}

impl ParsedRXingResult for TelParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::TEL
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(20);
        self.maybe_append(&self.number, &mut result);
        self.maybe_append(&self.title, &mut result);
        result
    }
}
impl TelParsedRXingResult {
    pub fn new(number: String, telURI: String, title: String) -> Self {
        Self {
            number,
            telURI,
            title,
        }
    }

    pub fn getNumber(&self) -> &str {
        &self.number
    }

    pub fn getTelURI(&self) -> &str {
        &self.telURI
    }

    pub fn getTitle(&self) -> &str {
        &self.title
    }
}
