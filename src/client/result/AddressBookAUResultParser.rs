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

// import java.util.ArrayList;
// import java.util.List;

use crate::RXingResult;

use super::{AddressBookParsedRXingResult, ParsedClientResult, ResultParser};

/**
 * Implements KDDI AU's address book format. See
 * <a href="http://www.au.kddi.com/ezfactory/tec/two_dimensions/index.html">
 * http://www.au.kddi.com/ezfactory/tec/two_dimensions/index.html</a>.
 * (Thanks to Yuzo for translating!)
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    // MEMORY is mandatory; seems like a decent indicator, as does end-of-record separator CR/LF
    if !rawText.contains("MEMORY") || !rawText.contains("\r\n") {
        return None;
    }

    // NAME1 and NAME2 have specific uses, namely written name and pronunciation, respectively.
    // Therefore we treat them specially instead of as an array of names.
    let name = ResultParser::matchSinglePrefixedField("NAME1:", &rawText, '\r', true);
    let pronunciation = ResultParser::matchSinglePrefixedField("NAME2:", &rawText, '\r', true);

    let phoneNumbers = matchMultipleValuePrefix("TEL", &rawText);
    let emails = matchMultipleValuePrefix("MAIL", &rawText);
    let note = ResultParser::matchSinglePrefixedField("MEMORY:", &rawText, '\r', false)
        .unwrap_or_default();
    let address = ResultParser::matchSinglePrefixedField("ADD:", &rawText, '\r', true);
    let addresses = address.map_or_else(Vec::new, |add| vec![add]);

    if let Ok(new_data) = AddressBookParsedRXingResult::with_details(
        ResultParser::maybeWrap(name).unwrap_or_default(),
        Vec::new(),
        pronunciation.unwrap_or_default(),
        phoneNumbers,
        Vec::new(),
        emails, //Vec::new(),
        Vec::new(),
        String::default(),
        note,
        addresses,
        Vec::new(),
        String::default(),
        String::default(),
        String::default(),
        Vec::new(),
        Vec::new(),
    ) {
        Some(ParsedClientResult::AddressBookResult(new_data))
    } else {
        None
    }
}

fn matchMultipleValuePrefix(prefix: &str, rawText: &str) -> Vec<String> {
    let mut values = Vec::new();
    // For now, always 3, and always trim
    for i in 1..=3 {
        // for (int i = 1; i <= 3; i++) {
        let value =
            ResultParser::matchSinglePrefixedField(&format!("{prefix}{i}:"), rawText, '\r', true);

        if let Some(value) = value {
            values.push(value)
        } else {
            break;
        }
    }

    values
}
