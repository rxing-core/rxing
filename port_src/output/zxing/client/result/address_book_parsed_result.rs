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
// package com::google::zxing::client::result;

/**
 * Represents a parsed result that encodes contact information, like that in an address book
 * entry.
 *
 * @author Sean Owen
 */
pub struct AddressBookParsedResult {
    super: ParsedResult;

     let names: Vec<String>;

     let nicknames: Vec<String>;

     let pronunciation: String;

     let phone_numbers: Vec<String>;

     let phone_types: Vec<String>;

     let emails: Vec<String>;

     let email_types: Vec<String>;

     let instant_messenger: String;

     let note: String;

     let addresses: Vec<String>;

     let address_types: Vec<String>;

     let org: String;

     let birthday: String;

     let title: String;

     let urls: Vec<String>;

     let geo: Vec<String>;
}

impl AddressBookParsedResult {

    pub fn new( names: &Vec<String>,  phone_numbers: &Vec<String>,  phone_types: &Vec<String>,  emails: &Vec<String>,  email_types: &Vec<String>,  addresses: &Vec<String>,  address_types: &Vec<String>) -> AddressBookParsedResult {
        this(&names, null, null, &phone_numbers, &phone_types, &emails, &email_types, null, null, &addresses, &address_types, null, null, null, null, null);
    }

    pub fn new( names: &Vec<String>,  nicknames: &Vec<String>,  pronunciation: &String,  phone_numbers: &Vec<String>,  phone_types: &Vec<String>,  emails: &Vec<String>,  email_types: &Vec<String>,  instant_messenger: &String,  note: &String,  addresses: &Vec<String>,  address_types: &Vec<String>,  org: &String,  birthday: &String,  title: &String,  urls: &Vec<String>,  geo: &Vec<String>) -> AddressBookParsedResult {
        super(ParsedResultType::ADDRESSBOOK);
        if phone_numbers != null && phone_types != null && phone_numbers.len() != phone_types.len() {
            throw IllegalArgumentException::new("Phone numbers and types lengths differ");
        }
        if emails != null && email_types != null && emails.len() != email_types.len() {
            throw IllegalArgumentException::new("Emails and types lengths differ");
        }
        if addresses != null && address_types != null && addresses.len() != address_types.len() {
            throw IllegalArgumentException::new("Addresses and types lengths differ");
        }
        let .names = names;
        let .nicknames = nicknames;
        let .pronunciation = pronunciation;
        let .phoneNumbers = phone_numbers;
        let .phoneTypes = phone_types;
        let .emails = emails;
        let .emailTypes = email_types;
        let .instantMessenger = instant_messenger;
        let .note = note;
        let .addresses = addresses;
        let .addressTypes = address_types;
        let .org = org;
        let .birthday = birthday;
        let .title = title;
        let .urls = urls;
        let .geo = geo;
    }

    pub fn  get_names(&self) -> Vec<String>  {
        return self.names;
    }

    pub fn  get_nicknames(&self) -> Vec<String>  {
        return self.nicknames;
    }

    /**
   * In Japanese, the name is written in kanji, which can have multiple readings. Therefore a hint
   * is often provided, called furigana, which spells the name phonetically.
   *
   * @return The pronunciation of the getNames() field, often in hiragana or katakana.
   */
    pub fn  get_pronunciation(&self) -> String  {
        return self.pronunciation;
    }

    pub fn  get_phone_numbers(&self) -> Vec<String>  {
        return self.phone_numbers;
    }

    /**
   * @return optional descriptions of the type of each phone number. It could be like "HOME", but,
   *  there is no guaranteed or standard format.
   */
    pub fn  get_phone_types(&self) -> Vec<String>  {
        return self.phone_types;
    }

    pub fn  get_emails(&self) -> Vec<String>  {
        return self.emails;
    }

    /**
   * @return optional descriptions of the type of each e-mail. It could be like "WORK", but,
   *  there is no guaranteed or standard format.
   */
    pub fn  get_email_types(&self) -> Vec<String>  {
        return self.email_types;
    }

    pub fn  get_instant_messenger(&self) -> String  {
        return self.instant_messenger;
    }

    pub fn  get_note(&self) -> String  {
        return self.note;
    }

    pub fn  get_addresses(&self) -> Vec<String>  {
        return self.addresses;
    }

    /**
   * @return optional descriptions of the type of each e-mail. It could be like "WORK", but,
   *  there is no guaranteed or standard format.
   */
    pub fn  get_address_types(&self) -> Vec<String>  {
        return self.address_types;
    }

    pub fn  get_title(&self) -> String  {
        return self.title;
    }

    pub fn  get_org(&self) -> String  {
        return self.org;
    }

    pub fn  get_u_r_ls(&self) -> Vec<String>  {
        return self.urls;
    }

    /**
   * @return birthday formatted as yyyyMMdd (e.g. 19780917)
   */
    pub fn  get_birthday(&self) -> String  {
        return self.birthday;
    }

    /**
   * @return a location as a latitude/longitude pair
   */
    pub fn  get_geo(&self) -> Vec<String>  {
        return self.geo;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(100);
        maybe_append(&self.names, &result);
        maybe_append(&self.nicknames, &result);
        maybe_append(&self.pronunciation, &result);
        maybe_append(&self.title, &result);
        maybe_append(&self.org, &result);
        maybe_append(&self.addresses, &result);
        maybe_append(&self.phone_numbers, &result);
        maybe_append(&self.emails, &result);
        maybe_append(&self.instant_messenger, &result);
        maybe_append(&self.urls, &result);
        maybe_append(&self.birthday, &result);
        maybe_append(&self.geo, &result);
        maybe_append(&self.note, &result);
        return result.to_string();
    }
}

