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

// import java.io.UnsupportedEncodingException;
// import java.net.URLDecoder;
// import java.util.ArrayList;
// import java.util.HashMap;
// import java.util.List;
// import java.util.Map;
// import java.util.regex.Pattern;

use std::collections::HashMap;

use regex::Regex;
use urlencoding::decode;

use crate::{exceptions::Exceptions, RXingResult};

use super::{
    BookmarkDoCoMoResultParser, EmailAddressResultParser, EmailDoCoMoResultParser, GeoResultParser,
    ISBNResultParser, ParsedClientResult, ParsedRXingResult, ProductResultParser,
    SMSMMSResultParser, SMTPResultParser, TelResultParser, TextParsedRXingResult, URIResultParser,
    URLTOResultParser, VINResultParser, WifiResultParser,
};

/**
 * <p>Abstract class representing the result of decoding a barcode, as more than
 * a String -- as some type of structured data. This might be a subclass which represents
 * a URL, or an e-mail address. {@link #parseRXingResult(RXingResult)} will turn a raw
 * decoded string into the most appropriate type of structured representation.</p>
 *
 * <p>Thanks to Jeff Griffin for proposing rewrite of these classes that relies less
 * on exception-based mechanisms during parsing.</p>
 *
 * @author Sean Owen
 */
// pub trait RXingResultParser {
//     // const PARSERS: [&'static str; 20] = [
//     //     "BookmarkDoCoMoRXingResultParser",
//     //     "AddressBookDoCoMoRXingResultParser",
//     //     "EmailDoCoMoRXingResultParser",
//     //     "AddressBookAURXingResultParser",
//     //     "VCardRXingResultParser",
//     //     "BizcardRXingResultParser",
//     //     "VEventRXingResultParser",
//     //     "EmailAddressRXingResultParser",
//     //     "SMTPRXingResultParser",
//     //     "TelRXingResultParser",
//     //     "SMSMMSRXingResultParser",
//     //     "SMSTOMMSTORXingResultParser",
//     //     "GeoRXingResultParser",
//     //     "WifiRXingResultParser",
//     //     "URLTORXingResultParser",
//     //     "URIRXingResultParser",
//     //     "ISBNRXingResultParser",
//     //     "ProductRXingResultParser",
//     //     "ExpandedProductRXingResultParser",
//     //     "VINRXingResultParser",
//     // ];

//     /**
//      * Attempts to parse the raw {@link RXingResult}'s contents as a particular type
//      * of information (email, URL, etc.) and return a {@link ParsedRXingResult} encapsulating
//      * the result of parsing.
//      *
//      * @param theRXingResult the raw {@link RXingResult} to parse
//      * @return {@link ParsedRXingResult} encapsulating the parsing result
//      */
//     fn parse(&self, theRXingResult: &RXingResult) -> Option<ParsedClientResult>;
// }

type ParserFunction = dyn Fn(&RXingResult) -> Option<ParsedClientResult>;

const DIGITS: &'static str = "\\d+"; //= Pattern.compile("\\d+");
const AMPERSAND: &'static str = "&"; // private static final Pattern AMPERSAND = Pattern.compile("&");
const EQUALS: &'static str = "="; //private static final Pattern EQUALS = Pattern.compile("=");
const BYTE_ORDER_MARK: &'static str = "\u{feff}"; //private static final String BYTE_ORDER_MARK = "\ufeff";

const EMPTY_STR_ARRAY: &'static str = "";

pub fn getMassagedText(result: &RXingResult) -> String {
    let mut text = result.getText().clone();
    if text.starts_with(BYTE_ORDER_MARK) {
        text = text[1..].to_owned();
    }
    return text;
}

pub fn parseRXingResult(theRXingResult: &RXingResult) -> ParsedClientResult {
    let PARSERS: [&ParserFunction; 14] = [
        &BookmarkDoCoMoResultParser::parse,
        //     new AddressBookDoCoMoRXingResultParser(),
        &EmailDoCoMoResultParser::parse,
        //     new AddressBookAURXingResultParser(),
        //     new VCardRXingResultParser(),
        //     new BizcardRXingResultParser(),
        //     new VEventRXingResultParser(),
        &EmailAddressResultParser::parse,
        &SMTPResultParser::parse,
        &TelResultParser::parse,
        &SMSMMSResultParser::parse,
        &SMSMMSResultParser::parse,
        &GeoResultParser::parse,
        &WifiResultParser::parse,
        &URLTOResultParser::parse,
        &URIResultParser::parse,
        &ISBNResultParser::parse,
        &ProductResultParser::parse,
        //     new ExpandedProductRXingResultParser(),
        &VINResultParser::parse,
    ];

    for parser in PARSERS {
        let result = parser(theRXingResult);
        if result.is_some() {
            return result.unwrap();
        }
    }
    //   ParsedRXingResult result = parser.parse(theRXingResult);
    //   if (result != null) {
    //     return result;
    //   }
    // }

    ParsedClientResult::TextResult(TextParsedRXingResult::new(
        theRXingResult.getText().to_owned(),
        "".to_owned(),
    ))
}

pub fn maybe_append_string(value: &str, result: &mut String) {
    if !value.is_empty() {
        result.push('\n');
        result.push_str(value);
    }
}

pub fn maybe_append_multiple(value: &[String], result: &mut String) {
    if !value.is_empty() {
        for s in value {
            // for (String s : value) {
            if !s.is_empty() {
                result.push('\n');
                result.push_str(s);
            }
        }
    }
}

pub fn maybeWrap(value: Option<&str>) -> Option<Vec<String>> {
    if value.is_none() {
        None
    } else {
        Some(vec![value.unwrap().to_owned()])
    }
}

pub fn unescapeBackslash(escaped: &str) -> String {
    let backslash = escaped.find('\\');
    if backslash.is_none() {
        return escaped.to_owned();
    }
    let max = escaped.len();
    let mut unescaped = String::with_capacity(max - 1);
    let backslash = backslash.unwrap_or(0);
    unescaped.push_str(&escaped[0..backslash]);
    let mut nextIsEscaped = false;
    for i in backslash..max {
        // for (int i = backslash; i < max; i++) {
        let c = escaped.chars().nth(i).unwrap();
        if nextIsEscaped || c != '\\' {
            unescaped.push(c);
            nextIsEscaped = false;
        } else {
            nextIsEscaped = true;
        }
    }

    unescaped
}

pub fn parseHexDigit(c: char) -> i32 {
    if c >= '0' && c <= '9' {
        return (c as u8 - '0' as u8) as i32;
    }
    if c >= 'a' && c <= 'f' {
        return 10 + (c as u8 - 'a' as u8) as i32;
    }
    if c >= 'A' && c <= 'F' {
        return 10 + (c as u8 - 'A' as u8) as i32;
    }
    return -1;
}

pub fn isStringOfDigits(value: &str, length: usize) -> bool {
    let matcher = Regex::new(DIGITS).unwrap();
    !value.is_empty() && length > 0 && length == value.len() && matcher.is_match(value)
}

pub fn isSubstringOfDigits(value: &str, offset: usize, length: usize) -> bool {
    if value.is_empty() || length <= 0 {
        return false;
    }
    let max = offset as usize + length;

    let matcher = Regex::new(DIGITS).unwrap();
    let sub_seq = &value[offset as usize..max];

    let is_a_match = if let Some(mtch) = matcher.find(sub_seq) {
        if mtch.start() == 0 && mtch.end() == sub_seq.len() {
            true
        } else {
            false
        }
    } else {
        false
    };

    value.len() >= max && is_a_match
}

pub fn parseNameValuePairs(uri: &str) -> Option<HashMap<String, String>> {
    let paramStart = uri.find('?');
    if paramStart.is_none() {
        return None;
    }
    let mut result = HashMap::with_capacity(3);
    let paramStart = paramStart.unwrap_or(0);

    let sub_str = &uri[paramStart + 1..];
    let list = sub_str.split(AMPERSAND);
    for keyValue in list {
        appendKeyValue(keyValue, &mut result);
    }

    // for keyValue in Self::AMPERSAND.split(uri[paramStart + 1..]) {
    // // for (String keyValue : AMPERSAND.split(uri.substring(paramStart + 1))) {
    //   Self::appendKeyValue(keyValue, &mut result);
    // }
    Some(result)
}

pub fn appendKeyValue(keyValue: &str, result: &mut HashMap<String, String>) {
    let keyValueTokens = keyValue.split(EQUALS); //Self::EQUALS.split(keyValue, 2);

    let kvp: Vec<&str> = keyValueTokens.take(2).collect();
    if let [key, value] = kvp[..] {
        let p_value = urlDecode(value).unwrap_or("".to_owned());
        result.insert(key.to_owned(), p_value);
    }

    // if keyValueTokens.len() == 2 {
    //   let key = keyValueTokens[0];
    //   let value = keyValueTokens[1];
    //   try {
    //     value = Self::urlDecode(value);
    //     result.put(key, value);
    //   } catch (IllegalArgumentException iae) {
    //     // continue; invalid data such as an escape like %0t
    //   }
    // }
}

pub fn urlDecode(encoded: &str) -> Result<String, Exceptions> {
    if let Ok(decoded) = decode(encoded) {
        Ok(decoded.to_string())
    } else {
        Err(Exceptions::IllegalStateException(
            "UnsupportedEncodingException".to_owned(),
        ))
    }
}

pub fn matchPrefixedField(
    prefix: &str,
    rawText: &str,
    endChar: char,
    trim: bool,
) -> Option<Vec<String>> {
    let mut matches = Vec::new();
    let mut i = 0;
    let max = rawText.len();
    while i < max {
        i = if let Some(loc) = rawText[i..].find(prefix) {
            loc + i
        } else {
            break;
        };
        //   i = rawText.indexOf(prefix, i);
        //   if (i < 0) {
        //     break;
        //   }
        i += prefix.len(); // Skip past this prefix we found to start
        let start = i; // Found the start of a match here
        let mut more = true;
        while more {
            let next_index = rawText[i..].find(endChar);
            if next_index.is_none() {
                // No terminating end character? uh, done. Set i such that loop terminates and break
                i = rawText.len();
                more = false;
                continue;
            } else {
                i += next_index.unwrap();
            }

            if countPrecedingBackslashes(rawText, i) % 2 != 0 {
                // semicolon was escaped (odd count of preceding backslashes) so continue
                i += 1;
            } else {
                // found a match
                let mut element = unescapeBackslash(&rawText[start..i]);
                if trim {
                    element = element.trim().to_owned();
                }
                if !element.is_empty() {
                    matches.push(element);
                }
                i += 1;
                more = false;
            }

            // i = rawText.indexOf(endChar, i);
            // if i < 0 {
            //   // No terminating end character? uh, done. Set i such that loop terminates and break
            //   i = rawText.len();
            //   more = false;
            // } else if countPrecedingBackslashes(rawText, i) % 2 != 0 {
            //   // semicolon was escaped (odd count of preceding backslashes) so continue
            //   i+=1;
            // } else {
            //   // found a match
            //   let element = unescapeBackslash(&rawText[start..start+i]);
            //   if (trim) {
            //     element = element.trim();
            //   }
            //   if (!element.isEmpty()) {
            //     matches.add(element);
            //   }
            //   i+=1;
            //   more = false;
            // }
        }
    }
    if matches.is_empty() {
        return None;
    }

    Some(matches)
}

pub fn countPrecedingBackslashes(s: &str, pos: usize) -> u32 {
    let mut count = 0;
    for i in (0..pos).rev() {
        // for (int i = pos - 1; i >= 0; i--) {
        if s.chars().nth(i).unwrap() == '\\' {
            count += 1;
        } else {
            break;
        }
    }
    return count;
}

pub fn matchSinglePrefixedField(
    prefix: &str,
    rawText: &str,
    endChar: char,
    trim: bool,
) -> Option<String> {
    let matches = matchPrefixedField(prefix, rawText, endChar, trim);
    if let Some(m) = matches {
        Some(m[0].clone())
    } else {
        None
    }
    // return matches == null ? null : matches[0];
}

pub fn match_do_co_mo_prefixed_field(prefix: &str, raw_text: &str) -> Option<Vec<String>> {
    matchPrefixedField(prefix, raw_text, ';', true)
}

pub fn match_single_do_co_mo_prefixed_field(
    prefix: &str,
    raw_text: &str,
    trim: bool,
) -> Option<String> {
    matchSinglePrefixedField(prefix, raw_text, ';', trim)
}
