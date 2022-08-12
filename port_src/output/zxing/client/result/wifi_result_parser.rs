/*
 * Copyright 2010 ZXing authors
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

pub struct WifiResultParser {
    super: ResultParser;
}

impl WifiResultParser {

    pub fn  parse(&self,  result: &Result) -> WifiParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("WIFI:") {
            return null;
        }
        raw_text = raw_text.substring(&"WIFI:".length());
         let ssid: String = match_single_prefixed_field("S:", &raw_text, ';', false);
        if ssid == null || ssid.is_empty() {
            return null;
        }
         let pass: String = match_single_prefixed_field("P:", &raw_text, ';', false);
         let mut type: String = match_single_prefixed_field("T:", &raw_text, ';', false);
        if type == null {
            type = "nopass";
        }
        // Unfortunately, in the past, H: was not just used for boolean 'hidden', but 'phase 2 method'.
        // To try to retain backwards compatibility, we set one or the other based on whether the string
        // is 'true' or 'false':
         let mut hidden: bool = false;
         let phase2_method: String = match_single_prefixed_field("PH2:", &raw_text, ';', false);
         let h_value: String = match_single_prefixed_field("H:", &raw_text, ';', false);
        if h_value != null {
            // If PH2 was specified separately, or if the value is clearly boolean, interpret it as 'hidden'
            if phase2_method != null || "true".equals_ignore_case(&h_value) || "false".equals_ignore_case(&h_value) {
                hidden = Boolean::parse_boolean(&h_value);
            } else {
                phase2_method = h_value;
            }
        }
         let identity: String = match_single_prefixed_field("I:", &raw_text, ';', false);
         let anonymous_identity: String = match_single_prefixed_field("A:", &raw_text, ';', false);
         let eap_method: String = match_single_prefixed_field("E:", &raw_text, ';', false);
        return WifiParsedResult::new(&type, &ssid, &pass, hidden, &identity, &anonymous_identity, &eap_method, &phase2_method);
    }
}

