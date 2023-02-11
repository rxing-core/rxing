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

// import java.util.regex.Pattern;

use regex::Regex;

use crate::RXingResult;

use super::{EmailAddressParsedRXingResult, ParsedClientResult, ResultParser};

use once_cell::sync::Lazy;

static ATEXT_ALPHANUMERIC: Lazy<Regex> =
    Lazy::new(|| Regex::new("[a-zA-Z0-9@.!#$%&'*+\\-/=?^_`{|}~]+").unwrap());

/**
 * Implements the "MATMSG" email message entry format.
 *
 * Supported keys: TO, SUB, BODY
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    if !rawText.starts_with("MATMSG:") {
        return None;
    }
    let tos = ResultParser::match_docomo_prefixed_field("TO:", &rawText)?;

    for to in &tos {
        if !isBasicallyValidEmailAddress(to, &ATEXT_ALPHANUMERIC) {
            return None;
        }
    }
    let subject = ResultParser::match_single_docomo_prefixed_field("SUB:", &rawText, false)
        .unwrap_or_default();
    let body = ResultParser::match_single_docomo_prefixed_field("BODY:", &rawText, false)
        .unwrap_or_default();
    Some(ParsedClientResult::EmailResult(
        EmailAddressParsedRXingResult::with_details(tos, Vec::new(), Vec::new(), subject, body),
    ))
}

/**
 * This implements only the most basic checking for an email address's validity -- that it contains
 * an '@' and contains no characters disallowed by RFC 2822. This is an overly lenient definition of
 * validity. We want to generally be lenient here since this class is only intended to encapsulate what's
 * in a barcode, not "judge" it.
 */
pub fn isBasicallyValidEmailAddress(email: &str, regex: &Regex) -> bool {
    let email_exists = !email.is_empty();
    let email_has_at = matches!(email.find('@'), Some(_));
    let email_alphamatcher = if let Some(mtch) = regex.find(email) {
        mtch.start() == 0 && mtch.end() == email.len()
    } else {
        false
    };
    email_exists && email_alphamatcher && email_has_at
    // return email != null && ATEXT_ALPHANUMERIC.matcher(email).matches() && email.indexOf('@') >= 0;
}
