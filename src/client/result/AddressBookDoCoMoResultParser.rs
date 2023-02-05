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

use super::{AddressBookParsedRXingResult, ParsedClientResult, ResultParser};

/**
 * Implements the "MECARD" address book entry format.
 *
 * Supported keys: N, SOUND, TEL, EMAIL, NOTE, ADR, BDAY, URL, plus ORG
 * Unsupported keys: TEL-AV, NICKNAME
 *
 * Except for TEL, multiple values for keys are also not supported;
 * the first one found takes precedence.
 *
 * Our understanding of the MECARD format is based on this document:
 *
 * http://www.mobicode.org.tw/files/OMIA%20Mobile%20Bar%20Code%20Standard%20v3.2.1.doc
 *
 * @author Sean Owen
 */
// public final class AddressBookDoCoMoRXingResultParser extends AbstractDoCoMoRXingResultParser {
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    if !rawText.starts_with("MECARD:") {
        return None;
    }
    let rawName = ResultParser::match_docomo_prefixed_field("N:", &rawText)?;

    let name = parseName(&rawName[0]);
    let pronunciation = ResultParser::match_single_docomo_prefixed_field("SOUND:", &rawText, true)
        .unwrap_or_default();
    let phoneNumbers =
        ResultParser::match_docomo_prefixed_field("TEL:", &rawText).unwrap_or_default();
    let emails = ResultParser::match_docomo_prefixed_field("EMAIL:", &rawText).unwrap_or_default();
    let note = ResultParser::match_single_docomo_prefixed_field("NOTE:", &rawText, false)
        .unwrap_or_default();
    let addresses = ResultParser::match_docomo_prefixed_field("ADR:", &rawText).unwrap_or_default();
    let mut birthday = ResultParser::match_single_docomo_prefixed_field("BDAY:", &rawText, true)
        .unwrap_or_default();
    if !ResultParser::isStringOfDigits(&birthday, 8) {
        // No reason to throw out the whole card because the birthday is formatted wrong.
        birthday = String::default();
    }
    let urls = ResultParser::match_docomo_prefixed_field("URL:", &rawText).unwrap_or_default();

    // Although ORG may not be strictly legal in MECARD, it does exist in VCARD and we might as well
    // honor it when found in the wild.
    let org = ResultParser::match_single_docomo_prefixed_field("ORG:", &rawText, true)
        .unwrap_or_default();

    if let Ok(new_adb) = AddressBookParsedRXingResult::with_details(
        ResultParser::maybeWrap(Some(name))?,
        Vec::new(),
        pronunciation,
        phoneNumbers,
        Vec::new(),
        emails,
        Vec::new(),
        String::default(),
        note,
        addresses,
        Vec::new(),
        org,
        birthday,
        String::default(),
        urls,
        Vec::new(),
    ) {
        Some(ParsedClientResult::AddressBookResult(new_adb))
    } else {
        None
    }
}

fn parseName(name: &str) -> String {
    if let Some(comma) = name.find(',') {
        format!("{} {}", &name[comma + 1..], &name[0..comma])
    } else {
        name.to_owned()
    }
    // let comma = name.indexOf(',');
    // if (comma >= 0) {
    //   // Format may be last,first; switch it around
    //   return name.substring(comma + 1) + ' ' + name.substring(0, comma);
    // }
    // return name;
}

// }
