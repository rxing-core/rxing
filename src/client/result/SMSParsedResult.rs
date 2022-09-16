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

use super::{ParsedRXingResult, ParsedRXingResultType, ResultParser};

/**
 * Represents a parsed result that encodes an SMS message, including recipients, subject
 * and body text.
 *
 * @author Sean Owen
 */
pub struct SMSParsedRXingResult {
    numbers: Vec<String>,
    vias: Vec<String>,
    subject: String,
    body: String,
}

impl ParsedRXingResult for SMSParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::SMS
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(100);
        ResultParser::maybe_append_multiple(&self.numbers, &mut result);
        ResultParser::maybe_append_string(&self.subject, &mut result);
        ResultParser::maybe_append_string(&self.body, &mut result);
        result
    }
}

impl SMSParsedRXingResult {
    pub fn with_singles(number: String, via: String, subject: String, body: String) -> Self {
        Self {
            numbers: vec![number],
            vias: vec![via],
            subject,
            body,
        }
    }

    pub fn with_arrays(
        numbers: Vec<String>,
        vias: Vec<String>,
        subject: String,
        body: String,
    ) -> Self {
        Self {
            numbers,
            vias,
            subject,
            body,
        }
    }

    pub fn getSMSURI(&self) -> String {
        let mut result = String::new();
        result.push_str("sms:");
        let mut first = true;
        for i in 0..self.numbers.len() {
            // for (int i = 0; i < numbers.length; i++) {
            if first {
                first = false;
            } else {
                result.push(',');
            }
            result.push_str(&self.numbers[i]);
            if !self.vias.is_empty() {
                result.push_str(";via=");
                result.push_str(&self.vias[i]);
            }
        }
        let has_body = !self.body.is_empty();
        let has_subject = !self.subject.is_empty();
        if has_body || has_subject {
            result.push('?');
            if has_body {
                result.push_str("body=");
                result.push_str(&self.body);
            }
            if has_subject {
                if has_body {
                    result.push('&');
                }
                result.push_str("subject=");
                result.push_str(&self.subject);
            }
        }
        result
    }

    pub fn getNumbers(&self) -> &Vec<String> {
        &self.numbers
    }

    pub fn getVias(&self) -> &Vec<String> {
        &self.vias
    }

    pub fn getSubject(&self) -> &str {
        &self.subject
    }

    pub fn getBody(&self) -> &str {
        &self.body
    }
}
