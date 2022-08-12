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
 * Parses contact information formatted according to the VCard (2.1) format. This is not a complete
 * implementation but should parse information as commonly encoded in 2D barcodes.
 *
 * @author Sean Owen
 */

 const BEGIN_VCARD: Pattern = Pattern::compile("BEGIN:VCARD", Pattern::CASE_INSENSITIVE);

 const VCARD_LIKE_DATE: Pattern = Pattern::compile("\\d{4}-?\\d{2}-?\\d{2}");

 const CR_LF_SPACE_TAB: Pattern = Pattern::compile("\r\n[ \t]");

 const NEWLINE_ESCAPE: Pattern = Pattern::compile("\\\\[nN]");

 const VCARD_ESCAPES: Pattern = Pattern::compile("\\\\([,;\\\\])");

 const EQUALS: Pattern = Pattern::compile("=");

 const SEMICOLON: Pattern = Pattern::compile(";");

 const UNESCAPED_SEMICOLONS: Pattern = Pattern::compile("(?<!\\\\);+");

 const COMMA: Pattern = Pattern::compile(",");

 const SEMICOLON_OR_COMMA: Pattern = Pattern::compile("[;,]");
pub struct VCardResultParser {
    super: ResultParser;
}

impl VCardResultParser {

    pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
        // Although we should insist on the raw text ending with "END:VCARD", there's no reason
        // to throw out everything else we parsed just because this was omitted. In fact, Eclair
        // is doing just that, and we can't parse its contacts without this leniency.
         let raw_text: String = get_massaged_text(result);
         let m: Matcher = BEGIN_VCARD::matcher(&raw_text);
        if !m.find() || m.start() != 0 {
            return null;
        }
         let mut names: List<List<String>> = ::match_v_card_prefixed_field("FN", &raw_text, true, false);
        if names == null {
            // If no display names found, look for regular name fields and format them
            names = ::match_v_card_prefixed_field("N", &raw_text, true, false);
            ::format_names(&names);
        }
         let nickname_string: List<String> = ::match_single_v_card_prefixed_field("NICKNAME", &raw_text, true, false);
         let nicknames: Vec<String> =  if nickname_string == null { null } else { COMMA::split(&nickname_string.get(0)) };
         let phone_numbers: List<List<String>> = ::match_v_card_prefixed_field("TEL", &raw_text, true, false);
         let emails: List<List<String>> = ::match_v_card_prefixed_field("EMAIL", &raw_text, true, false);
         let note: List<String> = ::match_single_v_card_prefixed_field("NOTE", &raw_text, false, false);
         let addresses: List<List<String>> = ::match_v_card_prefixed_field("ADR", &raw_text, true, true);
         let org: List<String> = ::match_single_v_card_prefixed_field("ORG", &raw_text, true, true);
         let mut birthday: List<String> = ::match_single_v_card_prefixed_field("BDAY", &raw_text, true, false);
        if birthday != null && !::is_like_v_card_date(&birthday.get(0)) {
            birthday = null;
        }
         let title: List<String> = ::match_single_v_card_prefixed_field("TITLE", &raw_text, true, false);
         let urls: List<List<String>> = ::match_v_card_prefixed_field("URL", &raw_text, true, false);
         let instant_messenger: List<String> = ::match_single_v_card_prefixed_field("IMPP", &raw_text, true, false);
         let geo_string: List<String> = ::match_single_v_card_prefixed_field("GEO", &raw_text, true, false);
         let mut geo: Vec<String> =  if geo_string == null { null } else { SEMICOLON_OR_COMMA::split(&geo_string.get(0)) };
        if geo != null && geo.len() != 2 {
            geo = null;
        }
        return AddressBookParsedResult::new(&::to_primary_values(&names), &nicknames, null, &::to_primary_values(&phone_numbers), &::to_types(&phone_numbers), &::to_primary_values(&emails), &::to_types(&emails), &::to_primary_value(&instant_messenger), &::to_primary_value(&note), &::to_primary_values(&addresses), &::to_types(&addresses), &::to_primary_value(&org), &::to_primary_value(&birthday), &::to_primary_value(&title), &::to_primary_values(&urls), &geo);
    }

    fn  match_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String,  trim: bool,  parse_field_divider: bool) -> List<List<String>>  {
         let mut matches: List<List<String>> = null;
         let mut i: i32 = 0;
         let max: i32 = raw_text.length();
        while i < max {
            // At start or after newline, match prefix, followed by optional metadata 
            // (led by ;) ultimately ending in colon
             let matcher: Matcher = Pattern::compile(format!("(?:^|\n){}(?:;([^:]*))?:", prefix), Pattern::CASE_INSENSITIVE)::matcher(&raw_text);
            if i > 0 {
                // Find from i-1 not i since looking at the preceding character
                i -= 1;
            }
            if !matcher.find(i) {
                break;
            }
            // group 0 = whole pattern; end(0) is past final colon
            i = matcher.end(0);
            // group 1 = metadata substring
             let metadata_string: String = matcher.group(1);
             let mut metadata: List<String> = null;
             let quoted_printable: bool = false;
             let quoted_printable_charset: String = null;
             let value_type: String = null;
            if metadata_string != null {
                for  let metadatum: String in SEMICOLON::split(&metadata_string) {
                    if metadata == null {
                        metadata = ArrayList<>::new(1);
                    }
                    metadata.add(&metadatum);
                     let metadatum_tokens: Vec<String> = EQUALS::split(&metadatum, 2);
                    if metadatum_tokens.len() > 1 {
                         let key: String = metadatum_tokens[0];
                         let value: String = metadatum_tokens[1];
                        if "ENCODING".equals_ignore_case(&key) && "QUOTED-PRINTABLE".equals_ignore_case(&value) {
                            quoted_printable = true;
                        } else if "CHARSET".equals_ignore_case(&key) {
                            quoted_printable_charset = value;
                        } else if "VALUE".equals_ignore_case(&key) {
                            value_type = value;
                        }
                    }
                }
            }
            // Found the start of a match here
             let match_start: i32 = i;
            while (i = raw_text.index_of('\n', i)) >= 0 {
                // Really, end in \r\n
                if // But if followed by tab or space,
                i < raw_text.length() - 1 && (// this is only a continuation
                raw_text.char_at(i + 1) == ' ' || raw_text.char_at(i + 1) == '\t') {
                    // Skip \n and continutation whitespace
                    i += 2;
                } else if // If preceded by = in quoted printable
                quoted_printable && (// this is a continuation
                (i >= 1 && raw_text.char_at(i - 1) == '=') || (i >= 2 && raw_text.char_at(i - 2) == '=')) {
                    // Skip \n
                    i += 1;
                } else {
                    break;
                }
            }
            if i < 0 {
                // No terminating end character? uh, done. Set i such that loop terminates and break
                i = max;
            } else if i > match_start {
                // found a match
                if matches == null {
                    // lazy init
                    matches = ArrayList<>::new(1);
                }
                if i >= 1 && raw_text.char_at(i - 1) == '\r' {
                    // Back up over \r, which really should be there
                    i -= 1;
                }
                 let mut element: String = raw_text.substring(match_start, i);
                if trim {
                    element = element.trim();
                }
                if quoted_printable {
                    element = ::decode_quoted_printable(&element, &quoted_printable_charset);
                    if parse_field_divider {
                        element = UNESCAPED_SEMICOLONS::matcher(&element)::replace_all("\n")::trim();
                    }
                } else {
                    if parse_field_divider {
                        element = UNESCAPED_SEMICOLONS::matcher(&element)::replace_all("\n")::trim();
                    }
                    element = CR_LF_SPACE_TAB::matcher(&element)::replace_all("");
                    element = NEWLINE_ESCAPE::matcher(&element)::replace_all("\n");
                    element = VCARD_ESCAPES::matcher(&element)::replace_all("$1");
                }
                // Only handle VALUE=uri specially
                if "uri".equals(&value_type) {
                    // as value, to support tel: and mailto:
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        element = URI::create(&element)::get_scheme_specific_part();
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( iae: &IllegalArgumentException) {
                        }  0 => break
                    }

                }
                if metadata == null {
                     let match: List<String> = ArrayList<>::new(1);
                    match.add(&element);
                    matches.add(&match);
                } else {
                    metadata.add(0, &element);
                    matches.add(&metadata);
                }
                i += 1;
            } else {
                i += 1;
            }
        }
        return matches;
    }

    fn  decode_quoted_printable( value: &CharSequence,  charset: &String) -> String  {
         let length: i32 = value.length();
         let result: StringBuilder = StringBuilder::new(length);
         let fragment_buffer: ByteArrayOutputStream = ByteArrayOutputStream::new();
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let c: char = value.char_at(i);
                    match c {
                          '\r' => 
                             {
                            }
                          '\n' => 
                             {
                                break;
                            }
                          '=' => 
                             {
                                if i < length - 2 {
                                     let next_char: char = value.char_at(i + 1);
                                    if next_char != '\r' && next_char != '\n' {
                                         let next_next_char: char = value.char_at(i + 2);
                                         let first_digit: i32 = parse_hex_digit(next_char);
                                         let second_digit: i32 = parse_hex_digit(next_next_char);
                                        if first_digit >= 0 && second_digit >= 0 {
                                            fragment_buffer.write((first_digit << 4) + second_digit);
                                        }
                                        // else ignore it, assume it was incorrectly encoded
                                        i += 2;
                                    }
                                }
                                break;
                            }
                        _ => 
                             {
                                ::maybe_append_fragment(&fragment_buffer, &charset, &result);
                                result.append(c);
                            }
                    }
                }
                i += 1;
             }
         }

        ::maybe_append_fragment(&fragment_buffer, &charset, &result);
        return result.to_string();
    }

    fn  maybe_append_fragment( fragment_buffer: &ByteArrayOutputStream,  charset: &String,  result: &StringBuilder)   {
        if fragment_buffer.size() > 0 {
             let fragment_bytes: Vec<i8> = fragment_buffer.to_byte_array();
             let mut fragment: String;
            if charset == null {
                fragment = String::new(&fragment_bytes, StandardCharsets::UTF_8);
            } else {
                let tryResult1 = 0;
                'try1: loop {
                {
                    fragment = String::new(&fragment_bytes, &charset);
                }
                break 'try1
                }
                match tryResult1 {
                     catch ( e: &UnsupportedEncodingException) {
                        fragment = String::new(&fragment_bytes, StandardCharsets::UTF_8);
                    }  0 => break
                }

            }
            fragment_buffer.reset();
            result.append(&fragment);
        }
    }

    fn  match_single_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String,  trim: bool,  parse_field_divider: bool) -> List<String>  {
         let values: List<List<String>> = ::match_v_card_prefixed_field(&prefix, &raw_text, trim, parse_field_divider);
        return  if values == null || values.is_empty() { null } else { values.get(0) };
    }

    fn  to_primary_value( list: &List<String>) -> String  {
        return  if list == null || list.is_empty() { null } else { list.get(0) };
    }

    fn  to_primary_values( lists: &Collection<List<String>>) -> Vec<String>  {
        if lists == null || lists.is_empty() {
            return null;
        }
         let result: List<String> = ArrayList<>::new(&lists.size());
        for  let list: List<String> in lists {
             let value: String = list.get(0);
            if value != null && !value.is_empty() {
                result.add(&value);
            }
        }
        return result.to_array(EMPTY_STR_ARRAY);
    }

    fn  to_types( lists: &Collection<List<String>>) -> Vec<String>  {
        if lists == null || lists.is_empty() {
            return null;
        }
         let result: List<String> = ArrayList<>::new(&lists.size());
        for  let list: List<String> in lists {
             let value: String = list.get(0);
            if value != null && !value.is_empty() {
                 let mut type: String = null;
                 {
                     let mut i: i32 = 1;
                    while i < list.size() {
                        {
                             let metadatum: String = list.get(i);
                             let equals: i32 = metadatum.index_of('=');
                            if equals < 0 {
                                // take the whole thing as a usable label
                                type = metadatum;
                                break;
                            }
                            if "TYPE".equals_ignore_case(&metadatum.substring(0, equals)) {
                                type = metadatum.substring(equals + 1);
                                break;
                            }
                        }
                        i += 1;
                     }
                 }

                result.add(&type);
            }
        }
        return result.to_array(EMPTY_STR_ARRAY);
    }

    fn  is_like_v_card_date( value: &CharSequence) -> bool  {
        return value == null || VCARD_LIKE_DATE::matcher(&value)::matches();
    }

    /**
   * Formats name fields of the form "Public;John;Q.;Reverend;III" into a form like
   * "Reverend John Q. Public III".
   *
   * @param names name values to format, in place
   */
    fn  format_names( names: &Iterable<List<String>>)   {
        if names != null {
            for  let list: List<String> in names {
                 let name: String = list.get(0);
                 let mut components: [Option<String>; 5] = [None; 5];
                 let mut start: i32 = 0;
                 let mut end: i32;
                 let component_index: i32 = 0;
                while component_index < components.len() - 1 && (end = name.index_of(';', start)) >= 0 {
                    components[component_index] = name.substring(start, end);
                    component_index += 1;
                    start = end + 1;
                }
                components[component_index] = name.substring(start);
                 let new_name: StringBuilder = StringBuilder::new(100);
                ::maybe_append_component(&components, 3, &new_name);
                ::maybe_append_component(&components, 1, &new_name);
                ::maybe_append_component(&components, 2, &new_name);
                ::maybe_append_component(&components, 0, &new_name);
                ::maybe_append_component(&components, 4, &new_name);
                list.set(0, &new_name.to_string().trim());
            }
        }
    }

    fn  maybe_append_component( components: &Vec<String>,  i: i32,  new_name: &StringBuilder)   {
        if components[i] != null && !components[i].is_empty() {
            if new_name.length() > 0 {
                new_name.append(' ');
            }
            new_name.append(components[i]);
        }
    }
}

