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
 * Represents a parsed result that encodes an SMS message, including recipients, subject
 * and body text.
 *
 * @author Sean Owen
 */
pub struct SMSParsedResult {
    super: ParsedResult;

     let mut numbers: Vec<String>;

     let mut vias: Vec<String>;

     let subject: String;

     let body: String;
}

impl SMSParsedResult {

    pub fn new( number: &String,  via: &String,  subject: &String,  body: &String) -> SMSParsedResult {
        super(ParsedResultType::SMS);
        let .numbers =  : vec![String; 1] = vec![number, ]
        ;
        let .vias =  : vec![String; 1] = vec![via, ]
        ;
        let .subject = subject;
        let .body = body;
    }

    pub fn new( numbers: &Vec<String>,  vias: &Vec<String>,  subject: &String,  body: &String) -> SMSParsedResult {
        super(ParsedResultType::SMS);
        let .numbers = numbers;
        let .vias = vias;
        let .subject = subject;
        let .body = body;
    }

    pub fn  get_s_m_s_u_r_i(&self) -> String  {
         let result: StringBuilder = StringBuilder::new();
        result.append("sms:");
         let mut first: bool = true;
         {
             let mut i: i32 = 0;
            while i < self.numbers.len() {
                {
                    if first {
                        first = false;
                    } else {
                        result.append(',');
                    }
                    result.append(self.numbers[i]);
                    if self.vias != null && self.vias[i] != null {
                        result.append(";via=");
                        result.append(self.vias[i]);
                    }
                }
                i += 1;
             }
         }

         let has_body: bool = self.body != null;
         let has_subject: bool = self.subject != null;
        if has_body || has_subject {
            result.append('?');
            if has_body {
                result.append("body=");
                result.append(&self.body);
            }
            if has_subject {
                if has_body {
                    result.append('&');
                }
                result.append("subject=");
                result.append(&self.subject);
            }
        }
        return result.to_string();
    }

    pub fn  get_numbers(&self) -> Vec<String>  {
        return self.numbers;
    }

    pub fn  get_vias(&self) -> Vec<String>  {
        return self.vias;
    }

    pub fn  get_subject(&self) -> String  {
        return self.subject;
    }

    pub fn  get_body(&self) -> String  {
        return self.body;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(100);
        maybe_append(&self.numbers, &result);
        maybe_append(&self.subject, &result);
        maybe_append(&self.body, &result);
        return result.to_string();
    }
}

