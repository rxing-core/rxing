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

use crate::exceptions::Exceptions;

use super::{ParsedRXingResult, ParsedRXingResultType, ResultParser};

/**
 * Represents a parsed result that encodes contact information, like that in an address book
 * entry.
 *
 * @author Sean Owen
 */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AddressBookParsedRXingResult {
    names: Vec<String>,
    nicknames: Vec<String>,
    pronunciation: String,
    phone_numbers: Vec<String>,
    phone_types: Vec<String>,
    emails: Vec<String>,
    email_types: Vec<String>,
    instant_messenger: String,
    note: String,
    addresses: Vec<String>,
    address_types: Vec<String>,
    org: String,
    birthday: String,
    title: String,
    urls: Vec<String>,
    geo: Vec<String>,
}
impl ParsedRXingResult for AddressBookParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::ADDRESSBOOK
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(100);

        ResultParser::maybe_append_multiple(&self.names, &mut result);
        ResultParser::maybe_append_multiple(&self.nicknames, &mut result);
        ResultParser::maybe_append_string(&self.pronunciation, &mut result);
        ResultParser::maybe_append_string(&self.title, &mut result);
        ResultParser::maybe_append_string(&self.org, &mut result);
        ResultParser::maybe_append_multiple(&self.addresses, &mut result);
        ResultParser::maybe_append_multiple(&self.phone_numbers, &mut result);
        ResultParser::maybe_append_multiple(&self.emails, &mut result);
        ResultParser::maybe_append_string(&self.instant_messenger, &mut result);
        ResultParser::maybe_append_multiple(&self.urls, &mut result);
        ResultParser::maybe_append_string(&self.birthday, &mut result);
        ResultParser::maybe_append_multiple(&self.geo, &mut result);
        ResultParser::maybe_append_string(&self.note, &mut result);

        result
    }
}
impl AddressBookParsedRXingResult {
    pub fn new(
        names: Vec<String>,
        phone_numbers: Vec<String>,
        phone_types: Vec<String>,
        emails: Vec<String>,
        email_types: Vec<String>,
        addresses: Vec<String>,
        address_types: Vec<String>,
    ) -> Result<Self, Exceptions> {
        Self::with_details(
            names,
            Vec::new(),
            "".to_owned(),
            phone_numbers,
            phone_types,
            emails,
            email_types,
            "".to_owned(),
            "".to_owned(),
            addresses,
            address_types,
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub fn with_details(
        names: Vec<String>,
        nicknames: Vec<String>,
        pronunciation: String,
        phone_numbers: Vec<String>,
        phone_types: Vec<String>,
        emails: Vec<String>,
        email_types: Vec<String>,
        instant_messenger: String,
        note: String,
        addresses: Vec<String>,
        address_types: Vec<String>,
        org: String,
        birthday: String,
        title: String,
        urls: Vec<String>,
        geo: Vec<String>,
    ) -> Result<Self, Exceptions> {
        if phone_numbers.len() != phone_types.len() && !phone_types.is_empty() {
            return Err(Exceptions::IllegalArgumentException(Some(
                "Phone numbers and types lengths differ".to_owned(),
            )));
        }
        if emails.len() != email_types.len() && !email_types.is_empty() {
            return Err(Exceptions::IllegalArgumentException(Some(
                "Emails and types lengths differ".to_owned(),
            )));
        }
        if addresses.len() != address_types.len() && !address_types.is_empty() {
            return Err(Exceptions::IllegalArgumentException(Some(
                "Addresses and types lengths differ".to_owned(),
            )));
        }
        Ok(Self {
            names,
            nicknames,
            pronunciation,
            phone_numbers,
            phone_types,
            emails,
            email_types,
            instant_messenger,
            note,
            addresses,
            address_types,
            org,
            birthday,
            title,
            urls,
            geo,
        })
    }

    pub fn getNames(&self) -> &Vec<String> {
        &self.names
    }

    pub fn getNicknames(&self) -> &Vec<String> {
        &self.nicknames
    }

    /**
     * In Japanese, the name is written in kanji, which can have multiple readings. Therefore a hint
     * is often provided, called furigana, which spells the name phonetically.
     *
     * @return The pronunciation of the getNames() field, often in hiragana or katakana.
     */
    pub fn getPronunciation(&self) -> &str {
        &self.pronunciation
    }

    pub fn getPhoneNumbers(&self) -> &Vec<String> {
        &self.phone_numbers
    }

    /**
     * @return optional descriptions of the type of each phone number. It could be like "HOME", but,
     *  there is no guaranteed or standard format.
     */
    pub fn getPhoneTypes(&self) -> &Vec<String> {
        &self.phone_types
    }

    pub fn getEmails(&self) -> &Vec<String> {
        &self.emails
    }

    /**
     * @return optional descriptions of the type of each e-mail. It could be like "WORK", but,
     *  there is no guaranteed or standard format.
     */
    pub fn getEmailTypes(&self) -> &Vec<String> {
        &self.email_types
    }

    pub fn getInstantMessenger(&self) -> &str {
        &self.instant_messenger
    }

    pub fn getNote(&self) -> &str {
        &self.note
    }

    pub fn getAddresses(&self) -> &Vec<String> {
        &self.addresses
    }

    /**
     * @return optional descriptions of the type of each e-mail. It could be like "WORK", but,
     *  there is no guaranteed or standard format.
     */
    pub fn getAddressTypes(&self) -> &Vec<String> {
        &self.address_types
    }

    pub fn getTitle(&self) -> &str {
        &self.title
    }

    pub fn getOrg(&self) -> &str {
        &self.org
    }

    pub fn getURLs(&self) -> &Vec<String> {
        &self.urls
    }

    /**
     * @return birthday formatted as yyyyMMdd (e.g. 19780917)
     */
    pub fn getBirthday(&self) -> &str {
        &self.birthday
    }

    /**
     * @return a location as a latitude/longitude pair
     */
    pub fn getGeo(&self) -> &Vec<String> {
        &self.geo
    }
}
