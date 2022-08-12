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
 * Represents a result that encodes an e-mail address, either as a plain address
 * like "joe@example.org" or a mailto: URL like "mailto:joe@example.org".
 *
 * @author Sean Owen
 */

 const COMMA: Pattern = Pattern::compile(",");
pub struct EmailAddressResultParser {
    super: ResultParser;
}

impl EmailAddressResultParser {

    pub fn  parse(&self,  result: &Result) -> EmailAddressParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if raw_text.starts_with("mailto:") || raw_text.starts_with("MAILTO:") {
            // If it starts with mailto:, assume it is definitely trying to be an email address
             let host_email: String = raw_text.substring(7);
             let query_start: i32 = host_email.index_of('?');
            if query_start >= 0 {
                host_email = host_email.substring(0, query_start);
            }
            let tryResult1 = 0;
            'try1: loop {
            {
                host_email = url_decode(&host_email);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( iae: &IllegalArgumentException) {
                    return null;
                }  0 => break
            }

             let mut tos: Vec<String> = null;
            if !host_email.is_empty() {
                tos = COMMA::split(&host_email);
            }
             let name_values: Map<String, String> = parse_name_value_pairs(&raw_text);
             let mut ccs: Vec<String> = null;
             let mut bccs: Vec<String> = null;
             let mut subject: String = null;
             let mut body: String = null;
            if name_values != null {
                if tos == null {
                     let tos_string: String = name_values.get("to");
                    if tos_string != null {
                        tos = COMMA::split(&tos_string);
                    }
                }
                 let cc_string: String = name_values.get("cc");
                if cc_string != null {
                    ccs = COMMA::split(&cc_string);
                }
                 let bcc_string: String = name_values.get("bcc");
                if bcc_string != null {
                    bccs = COMMA::split(&bcc_string);
                }
                subject = name_values.get("subject");
                body = name_values.get("body");
            }
            return EmailAddressParsedResult::new(&tos, &ccs, &bccs, &subject, &body);
        } else {
            if !EmailDoCoMoResultParser::is_basically_valid_email_address(&raw_text) {
                return null;
            }
            return EmailAddressParsedResult::new(&raw_text);
        }
    }
}

