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
 * Represents a parsed result that encodes an email message including recipients, subject
 * and body text.
 *
 * @author Sean Owen
 */
pub struct EmailAddressParsedResult {
    super: ParsedResult;

     let tos: Vec<String>;

     let ccs: Vec<String>;

     let bccs: Vec<String>;

     let subject: String;

     let body: String;
}

impl EmailAddressParsedResult {

    fn new( to: &String) -> EmailAddressParsedResult {
        this( : vec![String; 1] = vec![to, ]
        , null, null, null, null);
    }

    fn new( tos: &Vec<String>,  ccs: &Vec<String>,  bccs: &Vec<String>,  subject: &String,  body: &String) -> EmailAddressParsedResult {
        super(ParsedResultType::EMAIL_ADDRESS);
        let .tos = tos;
        let .ccs = ccs;
        let .bccs = bccs;
        let .subject = subject;
        let .body = body;
    }

    /**
   * @return first elements of {@link #getTos()} or {@code null} if none
   * @deprecated use {@link #getTos()}
   */
    pub fn  get_email_address(&self) -> String  {
        return  if self.tos == null || self.tos.len() == 0 { null } else { self.tos[0] };
    }

    pub fn  get_tos(&self) -> Vec<String>  {
        return self.tos;
    }

    pub fn  get_c_cs(&self) -> Vec<String>  {
        return self.ccs;
    }

    pub fn  get_b_c_cs(&self) -> Vec<String>  {
        return self.bccs;
    }

    pub fn  get_subject(&self) -> String  {
        return self.subject;
    }

    pub fn  get_body(&self) -> String  {
        return self.body;
    }

    /**
   * @return "mailto:"
   * @deprecated without replacement
   */
    pub fn  get_mailto_u_r_i(&self) -> String  {
        return "mailto:";
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(30);
        maybe_append(&self.tos, &result);
        maybe_append(&self.ccs, &result);
        maybe_append(&self.bccs, &result);
        maybe_append(&self.subject, &result);
        maybe_append(&self.body, &result);
        return result.to_string();
    }
}

