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
 * <p>Parses an "sms:" URI result, which specifies a number to SMS.
 * See <a href="http://tools.ietf.org/html/rfc5724"> RFC 5724</a> on this.</p>
 *
 * <p>This class supports "via" syntax for numbers, which is not part of the spec.
 * For example "+12125551212;via=+12124440101" may appear as a number.
 * It also supports a "subject" query parameter, which is not mentioned in the spec.
 * These are included since they were mentioned in earlier IETF drafts and might be
 * used.</p>
 *
 * <p>This actually also parses URIs starting with "mms:" and treats them all the same way,
 * and effectively converts them to an "sms:" URI for purposes of forwarding to the platform.</p>
 *
 * @author Sean Owen
 */
pub struct SMSMMSResultParser {
    super: ResultParser;
}

impl SMSMMSResultParser {

    pub fn  parse(&self,  result: &Result) -> SMSParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !(raw_text.starts_with("sms:") || raw_text.starts_with("SMS:") || raw_text.starts_with("mms:") || raw_text.starts_with("MMS:")) {
            return null;
        }
        // Check up front if this is a URI syntax string with query arguments
         let name_value_pairs: Map<String, String> = parse_name_value_pairs(&raw_text);
         let mut subject: String = null;
         let mut body: String = null;
         let query_syntax: bool = false;
        if name_value_pairs != null && !name_value_pairs.is_empty() {
            subject = name_value_pairs.get("subject");
            body = name_value_pairs.get("body");
            query_syntax = true;
        }
        // Drop sms, query portion
         let query_start: i32 = raw_text.index_of('?', 4);
         let sms_u_r_i_without_query: String;
        // If it's not query syntax, the question mark is part of the subject or message
        if query_start < 0 || !query_syntax {
            sms_u_r_i_without_query = raw_text.substring(4);
        } else {
            sms_u_r_i_without_query = raw_text.substring(4, query_start);
        }
         let last_comma: i32 = -1;
         let mut comma: i32;
         let numbers: List<String> = ArrayList<>::new(1);
         let vias: List<String> = ArrayList<>::new(1);
        while (comma = sms_u_r_i_without_query.index_of(',', last_comma + 1)) > last_comma {
             let number_part: String = sms_u_r_i_without_query.substring(last_comma + 1, comma);
            ::add_number_via(&numbers, &vias, &number_part);
            last_comma = comma;
        }
        ::add_number_via(&numbers, &vias, &sms_u_r_i_without_query.substring(last_comma + 1));
        return SMSParsedResult::new(&numbers.to_array(EMPTY_STR_ARRAY), &vias.to_array(EMPTY_STR_ARRAY), &subject, &body);
    }

    fn  add_number_via( numbers: &Collection<String>,  vias: &Collection<String>,  number_part: &String)   {
         let number_end: i32 = number_part.index_of(';');
        if number_end < 0 {
            numbers.add(&number_part);
            vias.add(null);
        } else {
            numbers.add(&number_part.substring(0, number_end));
             let maybe_via: String = number_part.substring(number_end + 1);
             let mut via: String;
            if maybe_via.starts_with("via=") {
                via = maybe_via.substring(4);
            } else {
                via = null;
            }
            vias.add(&via);
        }
    }
}

