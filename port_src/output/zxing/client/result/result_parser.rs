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
 * <p>Abstract class representing the result of decoding a barcode, as more than
 * a String -- as some type of structured data. This might be a subclass which represents
 * a URL, or an e-mail address. {@link #parseResult(Result)} will turn a raw
 * decoded string into the most appropriate type of structured representation.</p>
 *
 * <p>Thanks to Jeff Griffin for proposing rewrite of these classes that relies less
 * on exception-based mechanisms during parsing.</p>
 *
 * @author Sean Owen
 */

 const PARSERS: vec![Vec<ResultParser>; 20] = vec![BookmarkDoCoMoResultParser::new(), AddressBookDoCoMoResultParser::new(), EmailDoCoMoResultParser::new(), AddressBookAUResultParser::new(), VCardResultParser::new(), BizcardResultParser::new(), VEventResultParser::new(), EmailAddressResultParser::new(), SMTPResultParser::new(), TelResultParser::new(), SMSMMSResultParser::new(), SMSTOMMSTOResultParser::new(), GeoResultParser::new(), WifiResultParser::new(), URLTOResultParser::new(), URIResultParser::new(), ISBNResultParser::new(), ProductResultParser::new(), ExpandedProductResultParser::new(), VINResultParser::new(), ]
;

 const DIGITS: Pattern = Pattern::compile("\\d+");

 const AMPERSAND: Pattern = Pattern::compile("&");

 const EQUALS: Pattern = Pattern::compile("=");

 const BYTE_ORDER_MARK: &'static str = "﻿";

 const EMPTY_STR_ARRAY: [Option<String>; 0] = [None; 0];
pub struct ResultParser {
}

impl ResultParser {

    /**
   * Attempts to parse the raw {@link Result}'s contents as a particular type
   * of information (email, URL, etc.) and return a {@link ParsedResult} encapsulating
   * the result of parsing.
   *
   * @param theResult the raw {@link Result} to parse
   * @return {@link ParsedResult} encapsulating the parsing result
   */
    pub fn  parse(&self,  the_result: &Result) -> ParsedResult ;

    pub fn  get_massaged_text( result: &Result) -> String  {
         let mut text: String = result.get_text();
        if text.starts_with(&BYTE_ORDER_MARK) {
            text = text.substring(1);
        }
        return text;
    }

    pub fn  parse_result( the_result: &Result) -> ParsedResult  {
        for  let parser: ResultParser in PARSERS {
             let result: ParsedResult = parser.parse(the_result);
            if result != null {
                return result;
            }
        }
        return TextParsedResult::new(&the_result.get_text(), null);
    }

    pub fn  maybe_append( value: &String,  result: &StringBuilder)   {
        if value != null {
            result.append('\n');
            result.append(&value);
        }
    }

    pub fn  maybe_append( value: &Vec<String>,  result: &StringBuilder)   {
        if value != null {
            for  let s: String in value {
                result.append('\n');
                result.append(&s);
            }
        }
    }

    pub fn  maybe_wrap( value: &String) -> Vec<String>  {
        return  if value == null { null } else {  : vec![String; 1] = vec![value, ]
         };
    }

    pub fn  unescape_backslash( escaped: &String) -> String  {
         let backslash: i32 = escaped.index_of('\\');
        if backslash < 0 {
            return escaped;
        }
         let max: i32 = escaped.length();
         let unescaped: StringBuilder = StringBuilder::new(max - 1);
        unescaped.append(&escaped.to_char_array(), 0, backslash);
         let next_is_escaped: bool = false;
         {
             let mut i: i32 = backslash;
            while i < max {
                {
                     let c: char = escaped.char_at(i);
                    if next_is_escaped || c != '\\' {
                        unescaped.append(c);
                        next_is_escaped = false;
                    } else {
                        next_is_escaped = true;
                    }
                }
                i += 1;
             }
         }

        return unescaped.to_string();
    }

    pub fn  parse_hex_digit( c: char) -> i32  {
        if c >= '0' && c <= '9' {
            return c - '0';
        }
        if c >= 'a' && c <= 'f' {
            return 10 + (c - 'a');
        }
        if c >= 'A' && c <= 'F' {
            return 10 + (c - 'A');
        }
        return -1;
    }

    pub fn  is_string_of_digits( value: &CharSequence,  length: i32) -> bool  {
        return value != null && length > 0 && length == value.length() && DIGITS::matcher(&value)::matches();
    }

    pub fn  is_substring_of_digits( value: &CharSequence,  offset: i32,  length: i32) -> bool  {
        if value == null || length <= 0 {
            return false;
        }
         let max: i32 = offset + length;
        return value.length() >= max && DIGITS::matcher(&value.sub_sequence(offset, max))::matches();
    }

    fn  parse_name_value_pairs( uri: &String) -> Map<String, String>  {
         let param_start: i32 = uri.index_of('?');
        if param_start < 0 {
            return null;
        }
         let result: Map<String, String> = HashMap<>::new(3);
        for  let key_value: String in AMPERSAND::split(&uri.substring(param_start + 1)) {
            ::append_key_value(&key_value, &result);
        }
        return result;
    }

    fn  append_key_value( key_value: &CharSequence,  result: &Map<String, String>)   {
         let key_value_tokens: Vec<String> = EQUALS::split(&key_value, 2);
        if key_value_tokens.len() == 2 {
             let key: String = key_value_tokens[0];
             let mut value: String = key_value_tokens[1];
            let tryResult1 = 0;
            'try1: loop {
            {
                value = ::url_decode(&value);
                result.put(&key, &value);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( iae: &IllegalArgumentException) {
                }  0 => break
            }

        }
    }

    fn  url_decode( encoded: &String) -> String  {
        let tryResult1 = 0;
        'try1: loop {
        {
            return URLDecoder::decode(&encoded, "UTF-8");
        }
        break 'try1
        }
        match tryResult1 {
             catch ( uee: &UnsupportedEncodingException) {
                throw IllegalStateException::new(&uee);
            }  0 => break
        }

    }

    fn  match_prefixed_field( prefix: &String,  raw_text: &String,  end_char: char,  trim: bool) -> Vec<String>  {
         let mut matches: List<String> = null;
         let mut i: i32 = 0;
         let max: i32 = raw_text.length();
        while i < max {
            i = raw_text.index_of(&prefix, i);
            if i < 0 {
                break;
            }
            // Skip past this prefix we found to start
            i += prefix.length();
            // Found the start of a match here
             let start: i32 = i;
             let mut more: bool = true;
            while more {
                i = raw_text.index_of(end_char, i);
                if i < 0 {
                    // No terminating end character? uh, done. Set i such that loop terminates and break
                    i = raw_text.length();
                    more = false;
                } else if ::count_preceding_backslashes(&raw_text, i) % 2 != 0 {
                    // semicolon was escaped (odd count of preceding backslashes) so continue
                    i += 1;
                } else {
                    // found a match
                    if matches == null {
                        // lazy init
                        matches = ArrayList<>::new(3);
                    }
                     let mut element: String = ::unescape_backslash(&raw_text.substring(start, i));
                    if trim {
                        element = element.trim();
                    }
                    if !element.is_empty() {
                        matches.add(&element);
                    }
                    i += 1;
                    more = false;
                }
            }
        }
        if matches == null || matches.is_empty() {
            return null;
        }
        return matches.to_array(&EMPTY_STR_ARRAY);
    }

    fn  count_preceding_backslashes( s: &CharSequence,  pos: i32) -> i32  {
         let mut count: i32 = 0;
         {
             let mut i: i32 = pos - 1;
            while i >= 0 {
                {
                    if s.char_at(i) == '\\' {
                        count += 1;
                    } else {
                        break;
                    }
                }
                i -= 1;
             }
         }

        return count;
    }

    fn  match_single_prefixed_field( prefix: &String,  raw_text: &String,  end_char: char,  trim: bool) -> String  {
         let matches: Vec<String> = ::match_prefixed_field(&prefix, &raw_text, end_char, trim);
        return  if matches == null { null } else { matches[0] };
    }
}

