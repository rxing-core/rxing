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
pub struct AddressBookDoCoMoResultParser {
    super: AbstractDoCoMoResultParser;
}

impl AddressBookDoCoMoResultParser {

    pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("MECARD:") {
            return null;
        }
         let raw_name: Vec<String> = match_do_co_mo_prefixed_field("N:", &raw_text);
        if raw_name == null {
            return null;
        }
         let name: String = ::parse_name(raw_name[0]);
         let pronunciation: String = match_single_do_co_mo_prefixed_field("SOUND:", &raw_text, true);
         let phone_numbers: Vec<String> = match_do_co_mo_prefixed_field("TEL:", &raw_text);
         let emails: Vec<String> = match_do_co_mo_prefixed_field("EMAIL:", &raw_text);
         let note: String = match_single_do_co_mo_prefixed_field("NOTE:", &raw_text, false);
         let addresses: Vec<String> = match_do_co_mo_prefixed_field("ADR:", &raw_text);
         let mut birthday: String = match_single_do_co_mo_prefixed_field("BDAY:", &raw_text, true);
        if !is_string_of_digits(&birthday, 8) {
            // No reason to throw out the whole card because the birthday is formatted wrong.
            birthday = null;
        }
         let urls: Vec<String> = match_do_co_mo_prefixed_field("URL:", &raw_text);
        // Although ORG may not be strictly legal in MECARD, it does exist in VCARD and we might as well
        // honor it when found in the wild.
         let org: String = match_single_do_co_mo_prefixed_field("ORG:", &raw_text, true);
        return AddressBookParsedResult::new(&maybe_wrap(&name), null, &pronunciation, &phone_numbers, null, &emails, null, null, &note, &addresses, null, &org, &birthday, null, &urls, null);
    }

    fn  parse_name( name: &String) -> String  {
         let comma: i32 = name.index_of(',');
        if comma >= 0 {
            // Format may be last,first; switch it around
            return name.substring(comma + 1) + ' ' + name.substring(0, comma);
        }
        return name;
    }
}

