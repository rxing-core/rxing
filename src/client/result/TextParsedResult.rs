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

use super::{ParsedRXingResult, ParsedRXingResultType};

/**
 * A simple result type encapsulating a string that has no further
 * interpretation.
 *
 * @author Sean Owen
 */
#[derive(PartialEq, Eq,Hash, Debug)]
pub struct TextParsedRXingResult {
    text: String,
    language: String,
}

impl ParsedRXingResult for TextParsedRXingResult {
    fn getType(&self) -> ParsedRXingResultType {
        ParsedRXingResultType::TEXT
    }

    fn getDisplayRXingResult(&self) -> String {
        self.text.clone()
    }
}

impl TextParsedRXingResult {
    pub fn new(text: String, language: String) -> Self {
        Self { text, language }
    }

    pub fn getText(&self) -> &str {
        &self.text
    }

    pub fn getLanguage(&self) -> &str {
        &self.language
    }
}
