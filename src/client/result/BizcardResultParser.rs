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
 * Implements the "BIZCARD" address book entry format, though this has been
 * largely reverse-engineered from examples observed in the wild -- still
 * looking for a definitive reference.
 *
 * @author Sean Owen
 */
// Yes, we extend AbstractDoCoMoRXingResultParser since the format is very much
// like the DoCoMo MECARD format, but this is not technically one of
// DoCoMo's proposed formats
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    if !rawText.starts_with("BIZCARD:") {
        return None;
    }

    let firstName = ResultParser::match_single_do_co_mo_prefixed_field("N:", &rawText, true)
        .unwrap_or_default();
    let lastName = ResultParser::match_single_do_co_mo_prefixed_field("X:", &rawText, true)
        .unwrap_or_default();
    let fullName = buildName(&firstName, &lastName);
    let title = ResultParser::match_single_do_co_mo_prefixed_field("T:", &rawText, true).unwrap_or_default();
    let org = ResultParser::match_single_do_co_mo_prefixed_field("C:", &rawText, true).unwrap_or_default();
    let addresses = ResultParser::match_do_co_mo_prefixed_field("A:", &rawText);
    let phoneNumber1 = ResultParser::match_single_do_co_mo_prefixed_field("B:", &rawText, true)
        .unwrap_or_default();
    let phoneNumber2 = ResultParser::match_single_do_co_mo_prefixed_field("M:", &rawText, true)
        .unwrap_or_default();
    let phoneNumber3 = ResultParser::match_single_do_co_mo_prefixed_field("F:", &rawText, true)
        .unwrap_or_default();
    let email = ResultParser::match_single_do_co_mo_prefixed_field("E:", &rawText, true)
        .unwrap_or_default();

    if let Ok(adb) = AddressBookParsedRXingResult::with_details(
        ResultParser::maybeWrap(Some(fullName))?,
        Vec::new(),
        "".to_owned(),
        buildPhoneNumbers(phoneNumber1, phoneNumber2, phoneNumber3),
        Vec::new(),
        ResultParser::maybeWrap(Some(email))?,
        Vec::new(),
        "".to_owned(),
        "".to_owned(),
        addresses?,
        Vec::new(),
        org,
        "".to_owned(),
        title,
        Vec::new(),
        Vec::new(),
    ) {
        Some(ParsedClientResult::AddressBookResult(adb))
    } else {
        None
    }

    // return new AddressBookParsedRXingResult(maybeWrap(fullName),
    //                                    null,
    //                                    null,
    //                                    buildPhoneNumbers(phoneNumber1, phoneNumber2, phoneNumber3),
    //                                    null,
    //                                    maybeWrap(email),
    //                                    null,
    //                                    null,
    //                                    null,
    //                                    addresses,
    //                                    null,
    //                                    org,
    //                                    null,
    //                                    title,
    //                                    null,
    //                                    null);
}

fn buildPhoneNumbers(number1: String, number2: String, number3: String) -> Vec<String> {
    let mut numbers = Vec::new();

    if !number1.is_empty() {
        numbers.push(number1);
    }
    if !number2.is_empty() {
        numbers.push(number2);
    }
    if !number3.is_empty() {
        numbers.push(number3);
    }

    numbers
}

fn buildName(firstName: &str, lastName: &str) -> String {
    if firstName.is_empty() {
        lastName.to_owned()
    } else {
        if lastName.is_empty() {
            firstName.to_owned()
        } else {
            format!("{} {}", firstName, lastName)
        }
    }
}
