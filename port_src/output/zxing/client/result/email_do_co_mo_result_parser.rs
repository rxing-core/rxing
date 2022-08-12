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
 * Implements the "MATMSG" email message entry format.
 *
 * Supported keys: TO, SUB, BODY
 *
 * @author Sean Owen
 */

 const ATEXT_ALPHANUMERIC: Pattern = Pattern::compile("[a-zA-Z0-9@.!#$%&'*+\\-/=?^_`{|}~]+");
pub struct EmailDoCoMoResultParser {
    super: AbstractDoCoMoResultParser;
}

impl EmailDoCoMoResultParser {

    pub fn  parse(&self,  result: &Result) -> EmailAddressParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("MATMSG:") {
            return null;
        }
         let tos: Vec<String> = match_do_co_mo_prefixed_field("TO:", &raw_text);
        if tos == null {
            return null;
        }
        for  let to: String in tos {
            if !::is_basically_valid_email_address(&to) {
                return null;
            }
        }
         let subject: String = match_single_do_co_mo_prefixed_field("SUB:", &raw_text, false);
         let body: String = match_single_do_co_mo_prefixed_field("BODY:", &raw_text, false);
        return EmailAddressParsedResult::new(&tos, null, null, &subject, &body);
    }

    /**
   * This implements only the most basic checking for an email address's validity -- that it contains
   * an '@' and contains no characters disallowed by RFC 2822. This is an overly lenient definition of
   * validity. We want to generally be lenient here since this class is only intended to encapsulate what's
   * in a barcode, not "judge" it.
   */
    fn  is_basically_valid_email_address( email: &String) -> bool  {
        return email != null && ATEXT_ALPHANUMERIC::matcher(&email)::matches() && email.index_of('@') >= 0;
    }
}

