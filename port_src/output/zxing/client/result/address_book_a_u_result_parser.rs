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
// package com::google::zxing::client::result;

/**
 * Implements KDDI AU's address book format. See
 * <a href="http://www.au.kddi.com/ezfactory/tec/two_dimensions/index.html">
 * http://www.au.kddi.com/ezfactory/tec/two_dimensions/index.html</a>.
 * (Thanks to Yuzo for translating!)
 *
 * @author Sean Owen
 */
pub struct AddressBookAUResultParser {
    super: ResultParser;
}

impl AddressBookAUResultParser {

    pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
         let raw_text: String = get_massaged_text(result);
        // MEMORY is mandatory; seems like a decent indicator, as does end-of-record separator CR/LF
        if !raw_text.contains("MEMORY") || !raw_text.contains("\r\n") {
            return null;
        }
        // NAME1 and NAME2 have specific uses, namely written name and pronunciation, respectively.
        // Therefore we treat them specially instead of as an array of names.
         let name: String = match_single_prefixed_field("NAME1:", &raw_text, '\r', true);
         let pronunciation: String = match_single_prefixed_field("NAME2:", &raw_text, '\r', true);
         let phone_numbers: Vec<String> = ::match_multiple_value_prefix("TEL", &raw_text);
         let emails: Vec<String> = ::match_multiple_value_prefix("MAIL", &raw_text);
         let note: String = match_single_prefixed_field("MEMORY:", &raw_text, '\r', false);
         let address: String = match_single_prefixed_field("ADD:", &raw_text, '\r', true);
         let addresses: Vec<String> =  if address == null { null } else {  : vec![String; 1] = vec![address, ]
         };
        return AddressBookParsedResult::new(&maybe_wrap(&name), null, &pronunciation, &phone_numbers, null, &emails, null, null, &note, &addresses, null, null, null, null, null, null);
    }

    fn  match_multiple_value_prefix( prefix: &String,  raw_text: &String) -> Vec<String>  {
         let mut values: List<String> = null;
        // For now, always 3, and always trim
         {
             let mut i: i32 = 1;
            while i <= 3 {
                {
                     let value: String = match_single_prefixed_field(format!("{}{}:", prefix, i), &raw_text, '\r', true);
                    if value == null {
                        break;
                    }
                    if values == null {
                        // lazy init
                        values = ArrayList<>::new(3);
                    }
                    values.add(&value);
                }
                i += 1;
             }
         }

        if values == null {
            return null;
        }
        return values.to_array(EMPTY_STR_ARRAY);
    }
}

