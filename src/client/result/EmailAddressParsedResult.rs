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

use super::{ParsedRXingResult, ParsedRXingResultType, ResultParser};

/**
 * Represents a parsed result that encodes an email message including recipients, subject
 * and body text.
 *
 * @author Sean Owen
 */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct EmailAddressParsedRXingResult {
    tos: Vec<String>,
    ccs: Vec<String>,
    bccs: Vec<String>,
    subject: String,
    body: String,
}
impl ParsedRXingResult for EmailAddressParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::EMAIL_ADDRESS
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(30);
        ResultParser::maybe_append_multiple(&self.tos, &mut result);
        ResultParser::maybe_append_multiple(&self.ccs, &mut result);
        ResultParser::maybe_append_multiple(&self.bccs, &mut result);
        ResultParser::maybe_append_string(&self.subject, &mut result);
        ResultParser::maybe_append_string(&self.body, &mut result);

        result
    }
}

impl EmailAddressParsedRXingResult {
    pub fn new(to: String) -> Self {
        Self::with_details(
            vec![to],
            Vec::new(),
            Vec::new(),
            String::default(),
            String::default(),
        )
    }

    pub fn with_details(
        tos: Vec<String>,
        ccs: Vec<String>,
        bccs: Vec<String>,
        subject: String,
        body: String,
    ) -> Self {
        Self {
            tos,
            ccs,
            bccs,
            subject,
            body,
        }
    }

    /**
     * @return first elements of {@link #getTos()} or {@code null} if none
     * @deprecated use {@link #getTos()}
     */
    #[deprecated]
    pub fn getEmailAddress(&self) -> &str {
        if self.tos.is_empty() {
            ""
        } else {
            &self.tos[0]
        }
        // return tos == null || tos.length == 0 ? null : tos[0];
    }

    pub fn getTos(&self) -> &[String] {
        &self.tos
    }

    pub fn getCCs(&self) -> &[String] {
        &self.ccs
    }

    pub fn getBCCs(&self) -> &[String] {
        &self.bccs
    }

    pub fn getSubject(&self) -> &str {
        &self.subject
    }

    pub fn getBody(&self) -> &str {
        &self.body
    }

    /**
     * @return "mailto:"
     * @deprecated without replacement
     */
    #[deprecated]
    pub fn getMailtoURI() -> &'static str {
        "mailto:"
    }
}
