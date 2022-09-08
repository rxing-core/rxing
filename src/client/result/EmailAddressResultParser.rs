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

// package com.google.zxing.client.result;

// import com.google.zxing.RXingResult;

// import java.util.Map;
// import java.util.regex.Pattern;

use regex::Regex;

use crate::RXingResult;

use super::{ParsedClientResult, ResultParser, EmailDoCoMoResultParser, EmailAddressParsedRXingResult};

/**
 * Represents a result that encodes an e-mail address, either as a plain address
 * like "joe@example.org" or a mailto: URL like "mailto:joe@example.org".
 *
 * @author Sean Owen
 */
  pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let comma_regex = Regex::new(",").unwrap();
    // private static final Pattern COMMA = Pattern.compile(",");
    let rawText = ResultParser::getMassagedText(result);
    if rawText.starts_with("mailto:") || rawText.starts_with("MAILTO:") {
      // If it starts with mailto:, assume it is definitely trying to be an email address
      let mut hostEmail = &rawText[7..];
      if let Some(queryStart) = hostEmail.find('?'){
        hostEmail = &hostEmail[..queryStart];
      }
      // int queryStart = hostEmail.indexOf('?');
      // if (queryStart >= 0) {
      //   hostEmail = hostEmail.substring(0, queryStart);
      // }
      // try {
        let tmp = if let Ok(res) = ResultParser::urlDecode(hostEmail){
          res
        }else {
          return None;
        };
        hostEmail = tmp.as_str();
      // } catch (IllegalArgumentException iae) {
      //   return null;
      // }
      let mut tos = if hostEmail.is_empty() {
        Vec::new()
      }else {
        comma_regex.split(hostEmail).into_iter().map(|s| s.to_owned()).collect()
      };
      // if (!hostEmail.isEmpty()) {
      //   tos = COMMA.split(hostEmail);
      // }
      let nameValues = ResultParser::parseNameValuePairs(&rawText);
      let mut ccs:Vec<String> = Vec::new();
      let mut bccs:Vec<String> = Vec::new();
      let mut subject = "".to_owned();
      let mut body = "".to_owned();
      if let Some(nv) = nameValues {
      // if (nameValues != null) {
        if tos.is_empty() {
          if let Some(tosString) = nv.get("to"){
            tos = comma_regex.split(tosString).into_iter().map(|s| s.to_owned()).collect();
          }
          // if tosString != null {
          //   tos = COMMA.split(tosString);
          // }
        }
        if let Some(ccString) = nv.get("cc"){
          ccs = comma_regex.split(ccString).into_iter().map(|s| s.to_owned()).collect();
        }
        // if ccString != null {
        //   ccs = COMMA.split(ccString);
        // }
        if let Some(bccString) = nv.get("bcc"){
          bccs = comma_regex.split(bccString).into_iter().map(|s| s.to_owned()).collect();
        }
        // if bccString != null {
        //   bccs = COMMA.split(bccString);
        // }
        subject = nv.get("subject").unwrap_or(&"".to_owned()).clone();
        body = nv.get("body").unwrap_or(&"".to_owned()).clone();
      }
      return Some(ParsedClientResult::EmailResult(EmailAddressParsedRXingResult::with_details(tos, ccs, bccs, subject.to_owned(), body.to_owned())));
    } else {
      let atext_alphanumeric = Regex::new("[a-zA-Z0-9@.!#$%&'*+\\-/=?^_`{|}~]+").unwrap();
      if !EmailDoCoMoResultParser::isBasicallyValidEmailAddress(&rawText,&atext_alphanumeric) {
        return None;
      }
      return Some(ParsedClientResult::EmailResult(EmailAddressParsedRXingResult::new(rawText)));
    }
  }
