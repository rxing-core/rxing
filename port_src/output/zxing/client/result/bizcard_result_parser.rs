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
 * Implements the "BIZCARD" address book entry format, though this has been
 * largely reverse-engineered from examples observed in the wild -- still
 * looking for a definitive reference.
 *
 * @author Sean Owen
 */
pub struct BizcardResultParser {
    super: AbstractDoCoMoResultParser;
}

impl BizcardResultParser {

    // Yes, we extend AbstractDoCoMoResultParser since the format is very much
    // like the DoCoMo MECARD format, but this is not technically one of
    // DoCoMo's proposed formats
    pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("BIZCARD:") {
            return null;
        }
         let first_name: String = match_single_do_co_mo_prefixed_field("N:", &raw_text, true);
         let last_name: String = match_single_do_co_mo_prefixed_field("X:", &raw_text, true);
         let full_name: String = ::build_name(&first_name, &last_name);
         let title: String = match_single_do_co_mo_prefixed_field("T:", &raw_text, true);
         let org: String = match_single_do_co_mo_prefixed_field("C:", &raw_text, true);
         let addresses: Vec<String> = match_do_co_mo_prefixed_field("A:", &raw_text);
         let phone_number1: String = match_single_do_co_mo_prefixed_field("B:", &raw_text, true);
         let phone_number2: String = match_single_do_co_mo_prefixed_field("M:", &raw_text, true);
         let phone_number3: String = match_single_do_co_mo_prefixed_field("F:", &raw_text, true);
         let email: String = match_single_do_co_mo_prefixed_field("E:", &raw_text, true);
        return AddressBookParsedResult::new(&maybe_wrap(&full_name), null, null, &::build_phone_numbers(&phone_number1, &phone_number2, &phone_number3), null, &maybe_wrap(&email), null, null, null, &addresses, null, &org, null, &title, null, null);
    }

    fn  build_phone_numbers( number1: &String,  number2: &String,  number3: &String) -> Vec<String>  {
         let numbers: List<String> = ArrayList<>::new(3);
        if number1 != null {
            numbers.add(&number1);
        }
        if number2 != null {
            numbers.add(&number2);
        }
        if number3 != null {
            numbers.add(&number3);
        }
         let size: i32 = numbers.size();
        if size == 0 {
            return null;
        }
        return numbers.to_array(: [Option<String>; size] = [None; size]);
    }

    fn  build_name( first_name: &String,  last_name: &String) -> String  {
        if first_name == null {
            return last_name;
        } else {
            return  if last_name == null { first_name } else { format!("{} {}", first_name, last_name) };
        }
    }
}

