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

/**
 * <p>Parses an "smtp:" URI result, whose format is not standardized but appears to be like:
 * {@code smtp[:subject[:body]]}.</p>
 *
 * @author Sean Owen
 */
pub struct SMTPResultParser {
    super: ResultParser;
}

impl SMTPResultParser {

    pub fn  parse(&self,  result: &Result) -> EmailAddressParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !(raw_text.starts_with("smtp:") || raw_text.starts_with("SMTP:")) {
            return null;
        }
         let email_address: String = raw_text.substring(5);
         let mut subject: String = null;
         let mut body: String = null;
         let mut colon: i32 = email_address.index_of(':');
        if colon >= 0 {
            subject = email_address.substring(colon + 1);
            email_address = email_address.substring(0, colon);
            colon = subject.index_of(':');
            if colon >= 0 {
                body = subject.substring(colon + 1);
                subject = subject.substring(0, colon);
            }
        }
        return EmailAddressParsedResult::new( : vec![String; 1] = vec![email_address, ]
        , null, null, &subject, &body);
    }
}

