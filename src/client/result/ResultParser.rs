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

use once_cell::sync::Lazy;

use crate::{common::Result, exceptions::Exceptions, RXingResult};

use super::{
    AddressBookAUResultParser, AddressBookDoCoMoResultParser, BizcardResultParser,
    BookmarkDoCoMoResultParser, EmailAddressResultParser, EmailDoCoMoResultParser,
    ExpandedProductResultParser, GeoResultParser, ISBNResultParser, ParsedClientResult,
    ProductResultParser, SMSMMSResultParser, SMSTOMMSTOResultParser, SMTPResultParser,
    TelResultParser, TextParsedRXingResult, URIResultParser, URLTOResultParser, VCardResultParser,
    VEventResultParser, VINResultParser, WifiResultParser,
};

/*
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

pub type ParserFunction = dyn Fn(&RXingResult) -> Option<ParsedClientResult>;

static DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new("\\d+").unwrap());

// const DIGITS: &'static str = "\\d+"; //= Pattern.compile("\\d+");
const AMPERSAND: &str = "&"; // private static final Pattern AMPERSAND = Pattern.compile("&");
const EQUALS: &str = "="; //private static final Pattern EQUALS = Pattern.compile("=");
const BYTE_ORDER_MARK: &str = "\u{feff}"; //private static final String BYTE_ORDER_MARK = "\ufeff";

// const EMPTY_STR_ARRAY: &'static str = "";

pub fn getMassagedText(result: &RXingResult) -> String {
    result
        .getText()
        .trim_start_matches(BYTE_ORDER_MARK)
        .to_owned()
    // if text.starts_with(BYTE_ORDER_MARK) {
    //     text = &text[1..];
    // }
    // text.to_owned()
}

pub fn parse_result_with_parsers(
    the_rxing_result: &RXingResult,
    parsers: &[&ParserFunction],
) -> ParsedClientResult {
    for parser in parsers {
        let result = parser(the_rxing_result);
        if let Some(res) = result {
            return res;
        }
    }
    parseRXingResult(the_rxing_result)
}

pub fn parse_result_with_parser<F: Fn(&RXingResult) -> Option<ParsedClientResult>>(
    the_rxing_result: &RXingResult,
    parser: F,
) -> Option<ParsedClientResult> {
    parser(the_rxing_result)
}

pub fn parseRXingResult(the_rxing_result: &RXingResult) -> ParsedClientResult {
    let PARSERS: [&ParserFunction; 20] = [
        &BookmarkDoCoMoResultParser::parse,
        &AddressBookDoCoMoResultParser::parse,
        &EmailDoCoMoResultParser::parse,
        &AddressBookAUResultParser::parse,
        &VCardResultParser::parse,
        &BizcardResultParser::parse,
        &VEventResultParser::parse,
        &EmailAddressResultParser::parse,
        &SMTPResultParser::parse,
        &TelResultParser::parse,
        &SMSMMSResultParser::parse,
        &SMSTOMMSTOResultParser::parse,
        &GeoResultParser::parse,
        &WifiResultParser::parse,
        &URLTOResultParser::parse,
        &URIResultParser::parse,
        &ISBNResultParser::parse,
        &ProductResultParser::parse,
        &ExpandedProductResultParser::parse,
        &VINResultParser::parse,
    ];

    for parser in PARSERS {
        let result = parser(the_rxing_result);
        if let Some(res) = result {
            return res;
        }
    }
    //   ParsedRXingResult result = parser.parse(theRXingResult);
    //   if (result != null) {
    //     return result;
    //   }
    // }

    ParsedClientResult::TextResult(TextParsedRXingResult::new(
        the_rxing_result.getText().to_owned(),
        String::default(),
    ))
}

pub fn maybe_append_string(value: &str, result: &mut String) {
    if !value.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(value);
    }
}

pub fn maybe_append_multiple(value: &[String], result: &mut String) {
    for s in value {
        // for (String s : value) {
        if !s.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(s);
        }
    }
}

#[inline(always)]
pub fn maybeWrap(value: Option<String>) -> Option<Vec<String>> {
    if value.is_none() {
        None
    } else {
        Some(vec![value.unwrap()])
    }
}

pub fn unescapeBackslash(escaped: &str) -> String {
    let backslash = escaped.find('\\');
    if backslash.is_none() {
        return escaped.to_owned();
    }
    let max = escaped.chars().count();
    let backslash = backslash.unwrap_or(0);
    let mut unescaped = escaped.chars().take(backslash).collect::<String>();
    unescaped.reserve(max - 1);
    let mut nextIsEscaped = false;
    for c in escaped.chars().skip(backslash) {
        if nextIsEscaped || c != '\\' {
            unescaped.push(c);
            nextIsEscaped = false;
        } else {
            nextIsEscaped = true;
        }
    }

    unescaped
}

#[inline(always)]
pub fn parseHexDigit(c: char) -> Option<u32> {
    c.to_digit(16)
    // let Some(v) = c.to_digit(16) else {
    //     return -1
    // };
    // v as i32
    // if c.is_ascii_digit() {
    //     return (c as u8 - b'0') as i32;
    // }
    // if ('a'..='f').contains(&c) {
    //     return 10 + (c as u8 - b'a') as i32;
    // }
    // if ('A'..='F').contains(&c) {
    //     return 10 + (c as u8 - b'A') as i32;
    // }
    // -1
}

#[inline(always)]
pub fn isStringOfDigits(value: &str, length: usize) -> bool {
    !value.is_empty() && length > 0 && length == value.len() && DIGITS.is_match(value)
}

pub fn isSubstringOfDigits(value: &str, offset: usize, length: usize) -> bool {
    if value.is_empty() || length == 0 {
        return false;
    }
    let max = offset + length;

    let sub_seq: String = value.chars().skip(offset).take(length).collect(); //&value[offset..max];

    let is_a_match = if let Some(mtch) = DIGITS.find(&sub_seq) {
        mtch.start() == 0 && mtch.end() == sub_seq.chars().count()
    } else {
        false
    };

    value.len() >= max && is_a_match
}

pub fn parseNameValuePairs(uri: &str) -> Option<HashMap<String, String>> {
    let paramStart = uri.find('?');
    paramStart?;
    let mut result = HashMap::with_capacity(3);
    let paramStart = paramStart.unwrap_or(0);

    let sub_str = &uri[paramStart + 1..]; // This is likely ok because we're looking for a specific single byte charaacter
    let list = sub_str.split(AMPERSAND);
    for keyValue in list {
        appendKeyValue(keyValue, &mut result);
    }
    Some(result)
}

pub fn appendKeyValue(keyValue: &str, result: &mut HashMap<String, String>) {
    let keyValueTokens = keyValue.split(EQUALS); //Self::EQUALS.split(keyValue, 2);

    let kvp: Vec<&str> = keyValueTokens.take(2).collect();
    if let [key, value] = kvp[..] {
        let p_value = urlDecode(value).unwrap_or_else(|_| String::default());
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

pub fn urlDecode(encoded: &str) -> Result<String> {
    if let Ok(decoded) = decode(encoded) {
        Ok(decoded.to_string())
    } else {
        Err(Exceptions::illegal_state_with(
            "UnsupportedEncodingException",
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
        i += prefix.chars().count(); // Skip past this prefix we found to start
        let start = i; // Found the start of a match here
        let mut more = true;
        while more {
            if let Some(next_index) = rawText[i..].find(endChar) {
                i += next_index;
            } else {
                // No terminating end character? uh, done. Set i such that loop terminates and break
                i = rawText.chars().count();
                more = false;
                continue;
            }

            if !countPrecedingBackslashes(rawText, i).is_multiple_of(2) {
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
    let cached_s = s.chars().collect::<Vec<_>>();
    for i in (0..pos).rev() {
        // for (int i = pos - 1; i >= 0; i--) {
        if cached_s[i] == '\\' {
            count += 1;
        } else {
            break;
        }
    }
    count
}

pub fn matchSinglePrefixedField(
    prefix: &str,
    rawText: &str,
    endChar: char,
    trim: bool,
) -> Option<String> {
    let matches = matchPrefixedField(prefix, rawText, endChar, trim);
    matches.map(|m| m[0].clone())
    // return matches == null ? null : matches[0];
}

pub fn match_docomo_prefixed_field(prefix: &str, raw_text: &str) -> Option<Vec<String>> {
    matchPrefixedField(prefix, raw_text, ';', true)
}

pub fn match_single_docomo_prefixed_field(
    prefix: &str,
    raw_text: &str,
    trim: bool,
) -> Option<String> {
    matchSinglePrefixedField(prefix, raw_text, ';', trim)
}

#[cfg(test)]
mod tests {
    use crate::{
        client::result::{
            OtherParsedResult, ParsedClientResult, ParsedRXingResult, TextParsedRXingResult,
        },
        RXingResult,
    };

    use super::parse_result_with_parser;

    #[test]
    fn test_single_parser() {
        let result: RXingResult = RXingResult::new(
            "text",
            vec![12, 23, 54, 23],
            Vec::new(),
            crate::BarcodeFormat::EAN_13,
        );
        let p_res = parse_result_with_parser(&result, |_| {
            Some(ParsedClientResult::TextResult(TextParsedRXingResult::new(
                String::from("parsed with parser"),
                String::from("en/us"),
            )))
        })
        .unwrap();
        assert_eq!(p_res.to_string(), "parsed with parser");
    }

    #[test]
    fn test_other_parser() {
        let result: RXingResult = RXingResult::new(
            "text",
            vec![12, 23, 54, 23],
            Vec::new(),
            crate::BarcodeFormat::EAN_13,
        );
        let p_res = parse_result_with_parser(&result, |v| {
            Some(ParsedClientResult::Other(OtherParsedResult::new(Box::new(
                v.getRawBytes().to_vec(),
            ))))
        })
        .unwrap();

        assert_eq!(p_res.getDisplayRXingResult(), "Any { .. }");

        if let ParsedClientResult::Other(opr) = p_res {
            if let Some(d) = opr.get_data().downcast_ref::<Vec<u8>>() {
                assert_eq!(d, result.getRawBytes());
            } else {
                panic!("did not get vec<u8>");
            }
        } else {
            panic!("did not get ParsedClientResult::Other");
        }
    }
}
