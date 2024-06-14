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

use std::convert::TryFrom;

use regex::Regex;

use once_cell::sync::Lazy;

use crate::{common::CharacterSet, RXingResult};

use uriparse::URI;

use super::{AddressBookParsedRXingResult, ParsedClientResult, ResultParser};

static BEGIN_VCARD: Lazy<Regex> = Lazy::new(|| Regex::new("(?i:BEGIN:VCARD)").unwrap());
static VCARD_LIKE_DATE: Lazy<Regex> = Lazy::new(|| Regex::new("\\d{4}-?\\d{2}-?\\d{2}").unwrap());
static CR_LF_SPACE_TAB: Lazy<Regex> = Lazy::new(|| Regex::new("\r\n[ \t]").unwrap());
static NEWLINE_ESCAPE: Lazy<Regex> = Lazy::new(|| Regex::new("\\\\[nN]").unwrap());
static VCARD_ESCAPE: Lazy<Regex> = Lazy::new(|| Regex::new("\\\\([,;\\\\])").unwrap());
static EQUALS: Lazy<Regex> = Lazy::new(|| Regex::new("=").unwrap());
static UNESCAPED_SEMICOLONS: Lazy<fancy_regex::Regex> =
    Lazy::new(|| fancy_regex::Regex::new("(?<!\\\\);+").unwrap());
static SEMICOLON_OR_COMMA: Lazy<Regex> = Lazy::new(|| Regex::new("[;,]").unwrap());

// const BEGIN_VCARD: &'static str = "(?i:BEGIN:VCARD)"; //, Pattern.CASE_INSENSITIVE);
// const VCARD_LIKE_DATE: &'static str = "\\d{4}-?\\d{2}-?\\d{2}";
// const CR_LF_SPACE_TAB: &'static str = "\r\n[ \t]";
// const NEWLINE_ESCAPE: &'static str = "\\\\[nN]";
// const VCARD_ESCAPES: &'static str = "\\\\([,;\\\\])";
// const EQUALS: &'static str = "=";
const SEMICOLON: &str = ";";
// const UNESCAPED_SEMICOLONS: &'static str = "(?<!\\\\);+";
const COMMA: &str = ",";
// const SEMICOLON_OR_COMMA: &'static str = "[;,]";

/**
 * Parses contact information formatted according to the VCard (2.1) format. This is not a complete
 * implementation but should parse information as commonly encoded in 2D barcodes.
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    // Although we should insist on the raw text ending with "END:VCARD", there's no reason
    // to throw out everything else we parsed just because this was omitted. In fact, Eclair
    // is doing just that, and we can't parse its contacts without this leniency.
    let rawText = ResultParser::getMassagedText(result);

    // let semicolon_comma_regex = Regex::new(SEMICOLON_OR_COMMA).unwrap();

    // let rg = Regex::new(BEGIN_VCARD).unwrap();
    let mtch = BEGIN_VCARD.find(&rawText)?;
    // Matcher m = BEGIN_VCARD.matcher(rawText);
    if mtch.start() != 0 {
        return None;
    }

    let names: Vec<Vec<String>> =
        if let Some(m) = matchVCardPrefixedField("FN", &rawText, true, false) {
            m
        } else {
            // If no display names found, look for regular name fields and format them
            let mut n = matchVCardPrefixedField("N", &rawText, true, false).unwrap_or_default();
            formatNames(&mut n);
            n
        };
    // if names == null {

    // }
    let nicknames = if let Some(nicknameString) =
        matchSingleVCardPrefixedField("NICKNAME", &rawText, true, false)
    {
        nicknameString[0]
            .split(COMMA)
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
        // COMMA.split(nicknameString.get(0)
    } else {
        Vec::new()
    };
    // let nicknames = nicknameString == null ? null : COMMA.split(nicknameString.get(0));
    let phoneNumbers = matchVCardPrefixedField("TEL", &rawText, true, false);
    let emails = matchVCardPrefixedField("EMAIL", &rawText, true, false);
    let note = matchSingleVCardPrefixedField("NOTE", &rawText, false, false);
    let addresses = matchVCardPrefixedField("ADR", &rawText, true, true);
    let org = matchSingleVCardPrefixedField("ORG", &rawText, true, true);
    let birthday =
        if let Some(bday_array) = matchSingleVCardPrefixedField("BDAY", &rawText, true, false) {
            if isLikeVCardDate(&bday_array[0]) {
                bday_array
            } else {
                vec![String::default()]
            }
        } else {
            vec![String::default()]
        };
    // if birthday != null && !isLikeVCardDate(birthday.get(0)) {
    //   birthday = null;
    // }
    let title = matchSingleVCardPrefixedField("TITLE", &rawText, true, false);
    let urls = matchVCardPrefixedField("URL", &rawText, true, false);
    let instantMessenger = matchSingleVCardPrefixedField("IMPP", &rawText, true, false);
    let geoString = matchSingleVCardPrefixedField("GEO", &rawText, true, false);
    let geo = if let Some(geo_string) = geoString {
        SEMICOLON_OR_COMMA
            .split(&geo_string[0])
            .map(|x| x.to_owned())
            .collect()
        // SEMICOLON_OR_COMMA.split(geoString.unwrap()[0])
    } else {
        Vec::new()
    };
    // if geo.len() != 2 {
    //   geo = null;
    // }
    if let Ok(adb) = AddressBookParsedRXingResult::with_details(
        toPrimaryValues(Some(names)),
        nicknames,
        String::default(),
        toPrimaryValues(phoneNumbers.clone()),
        toTypes(phoneNumbers),
        toPrimaryValues(emails.clone()),
        toTypes(emails),
        toPrimaryValue(instantMessenger),
        toPrimaryValue(note),
        toPrimaryValues(addresses.clone()),
        toTypes(addresses),
        toPrimaryValue(org),
        toPrimaryValue(Some(birthday)),
        toPrimaryValue(title),
        toPrimaryValues(urls),
        geo,
    ) {
        Some(ParsedClientResult::AddressBookResult(adb))
    } else {
        None
    }
}

pub fn matchVCardPrefixedField(
    prefix: &str,
    rawText: &str,
    trim: bool,
    parseFieldDivider: bool,
) -> Option<Vec<Vec<String>>> {
    let mut matches: Vec<Vec<String>> = Vec::new();
    let mut i = 0_isize;
    let max = rawText.len() as isize;

    // let equals_regex = Regex::new(EQUALS).unwrap();
    // let unescaped_semis = fancy_regex::Regex::new(UNESCAPED_SEMICOLONS).unwrap();
    // let cr_lf_space_tab = Regex::new(CR_LF_SPACE_TAB).unwrap();
    // let newline_esc = Regex::new(NEWLINE_ESCAPE).unwrap();
    // let vcard_esc = Regex::new(VCARD_ESCAPES).unwrap();

    // At start or after newline, match prefix, followed by optional metadata
    // (led by ;) ultimately ending in colon
    let matcher_primary = Regex::new(&format!("(?:^|\\n)(?i:{prefix})(?:;([^:]*))?:")).unwrap();
    // let matcher_primary = Regex::new(&format!("(?:^|\n){}(.*)", prefix)).unwrap();

    //let lower_case_raw_text = rawText.to_lowercase();

    while i < max {
        //let rawText = rawText.to_lowercase();
        // Pattern.CASE_INSENSITIVE).matcher(rawText);
        if i > 0 {
            i -= 1; // Find from i-1 not i since looking at the preceding character
        }
        let cap_text = &rawText[i as usize..];
        let cap_maybe = matcher_primary.captures(cap_text);
        //let matcher_maybe = matcher_primary.find_at(&lower_case_raw_text, i);
        if cap_maybe.is_none() {
            break;
        }
        let matcher = cap_maybe?;
        i += matcher.get(0)?.end() as isize; // group 0 = whole pattern; end(0) is past final colon

        let metadataString = matcher.get(1); // group 1 = metadata substring
        let mut metadata: Vec<String> = Vec::new();
        let mut quotedPrintable = false;
        let mut quotedPrintableCharset = "";
        let mut valueType = "";
        if metadataString.is_some() {
            // let mds = metadataString?.as_str().split(SEMICOLON).collect();
            for metadatum in metadataString?.as_str().split(SEMICOLON) {
                // for (String metadatum : SEMICOLON.split(metadataString)) {
                // if (metadata == null) {
                //   metadata = new ArrayList<>(1);
                // }
                metadata.push(metadatum.to_owned());

                let metadatumTokens = EQUALS.splitn(metadatum, 2).collect::<Vec<&str>>();
                if metadatumTokens.len() > 1 {
                    let key = metadatumTokens[0];
                    let value = metadatumTokens[1];
                    if "ENCODING" == key.to_uppercase()
                        && "QUOTED-PRINTABLE" == value.to_uppercase()
                    {
                        quotedPrintable = true;
                    } else if "CHARSET" == key.to_uppercase() {
                        quotedPrintableCharset = value;
                    } else if "VALUE" == key.to_uppercase() {
                        valueType = value;
                    }
                }
            }
        }

        let matchStart = i; // Found the start of a match here
        while let Some(pos) = rawText[i as usize..].find('\n') {
            // Really, end in \r\n
            i += pos as isize; // + i;
                               // while (i = rawText.indexOf('\n', i)) >= 0 { // Really, end in \r\n
            if i < rawText.len() as isize- 1 &&           // But if followed by tab or space,
            (rawText.chars().nth(i as usize+ 1)? == ' ' ||        // this is only a continuation
             rawText.chars().nth(i as usize+ 1)? == '\t')
            {
                i += 2; // Skip \n and continutation whitespace
            } else if quotedPrintable &&             // If preceded by = in quoted printable
                   ((i >= 1 && rawText.chars().nth(i as usize- 1)? == '=') || // this is a continuation
                    (i >= 2 && rawText.chars().nth(i as usize - 2)? == '='))
            {
                i += 1; // Skip \n
            } else {
                break;
            }
        }

        if i < 0 {
            // No terminating end character? uh, done. Set i such that loop terminates and break
            i = max;
        } else if i > matchStart {
            // found a match
            // if matches == null {
            //   matches = new ArrayList<>(1); // lazy init
            // }
            if i >= 1 && rawText.chars().nth(i as usize - 1)? == '\r' {
                i -= 1; // Back up over \r, which really should be there
            }
            let mut element = rawText[matchStart as usize..i as usize].to_owned();
            if trim {
                element = element.trim().to_owned();
            }

            if quotedPrintable {
                element = decodeQuotedPrintable(&element, quotedPrintableCharset);
                if parseFieldDivider {
                    element = UNESCAPED_SEMICOLONS
                        .replace_all(&element, "\n")
                        .to_mut()
                        .trim()
                        .to_owned();
                    // element = UNESCAPED_SEMICOLONS.matcher(element).replaceAll("\n").trim();
                }
            } else {
                if parseFieldDivider {
                    element = UNESCAPED_SEMICOLONS
                        .replace_all(&element, "\n")
                        .to_mut()
                        .trim()
                        .to_owned();
                    // element = UNESCAPED_SEMICOLONS.matcher(element).replaceAll("\n").trim();
                }
                element = CR_LF_SPACE_TAB
                    .replace_all(&element, "")
                    .to_mut()
                    .to_owned();
                element = NEWLINE_ESCAPE
                    .replace_all(&element, "\n")
                    .to_mut()
                    .to_owned();
                element = VCARD_ESCAPE.replace_all(&element, "$1").to_mut().to_owned();
                // element = CR_LF_SPACE_TAB.matcher(element).replaceAll("");
                // element = NEWLINE_ESCAPE.matcher(element).replaceAll("\n");
                // element = VCARD_ESCAPES.matcher(element).replaceAll("$1");
            }
            // Only handle VALUE=uri specially
            if "uri" == valueType.to_lowercase() {
                // Don't actually support dereferencing URIs, but use scheme-specific part not URI
                // as value, to support tel: and mailto:
                if let Ok(uri) = URI::try_from(element.as_str()) {
                    element = uri.path().to_string();
                }
                // try {
                //   element = URI.create(element).getSchemeSpecificPart();
                // } catch (IllegalArgumentException iae) {
                //   // ignore
                // }
            }
            // if metadata == null {
            //   List<String> match = new ArrayList<>(1);
            //   match.add(element);
            //   matches.add(match);
            // } else {
            metadata.push(element);
            matches.push(metadata.into_iter().collect());
            // }
            i += 1;
        } else {
            i += 1;
        }
    }
    if matches.is_empty() {
        None
    } else {
        Some(matches)
    }
}

fn decodeQuotedPrintable(value: &str, charset: &str) -> String {
    let length = value.len();
    let mut result = String::with_capacity(length);
    let mut fragmentBuffer: Vec<u8> = Vec::new(); //new ByteArrayOutputStream();
    let mut i = 0;
    // for i in 0..length {
    while i < length {
        // for (int i = 0; i < length; i++) {
        let c = value.chars().nth(i).unwrap_or_default();
        if c == '\r' || c == '\n' {
            i += 1;
            continue;
        }
        if c == '=' && i < length - 2 {
            let nextChar = value.chars().nth(i + 1).unwrap();
            if nextChar != '\r' && nextChar != '\n' {
                let nextNextChar = value.chars().nth(i + 2).unwrap();
                let firstDigit =
                    ResultParser::parseHexDigit(nextChar).map_or_else(|| -1, |d| d as i32);
                let secondDigit =
                    ResultParser::parseHexDigit(nextNextChar).map_or_else(|| -1, |d| d as i32);
                if firstDigit >= 0 && secondDigit >= 0 {
                    fragmentBuffer.push(((firstDigit << 4) + secondDigit) as u8);
                } // else ignore it, assume it was incorrectly encoded
                i += 2;
            }
            i += 1;
            continue;
        }
        maybeAppendFragment(&mut fragmentBuffer, charset, &mut result);
        result.push(c);
        i += 1;
        // match c {
        //     '\r' | '\n' => break,
        //     '=' if i < length - 2 => {
        //         let nextChar = value.chars().nth(i + 1).unwrap();
        //         if nextChar != '\r' && nextChar != '\n' {
        //             let nextNextChar = value.chars().nth(i + 2).unwrap();
        //             let firstDigit = ResultParser::parseHexDigit(nextChar);
        //             let secondDigit = ResultParser::parseHexDigit(nextNextChar);
        //             if firstDigit >= 0 && secondDigit >= 0 {
        //                 fragmentBuffer.push(((firstDigit << 4) + secondDigit) as u8);
        //             } // else ignore it, assume it was incorrectly encoded
        //             i += 2;
        //         }
        //     }
        //     _ => {
        //         maybeAppendFragment(&mut fragmentBuffer, charset, &mut result);
        //         result.push(c);
        //     }
        // }
        // switch (c) {
        //   case '\r':
        //   case '\n':
        //     break;
        //   case '=':
        //     if (i < length - 2) {
        //       char nextChar = value.charAt(i + 1);
        //       if (nextChar != '\r' && nextChar != '\n') {
        //         char nextNextChar = value.charAt(i + 2);
        //         int firstDigit = parseHexDigit(nextChar);
        //         int secondDigit = parseHexDigit(nextNextChar);
        //         if (firstDigit >= 0 && secondDigit >= 0) {
        //           fragmentBuffer.write((firstDigit << 4) + secondDigit);
        //         } // else ignore it, assume it was incorrectly encoded
        //         i += 2;
        //       }
        //     }
        //     break;
        //   default:
        //     maybeAppendFragment(fragmentBuffer, charset, result);
        //     result.append(c);
        // }
    }
    maybeAppendFragment(&mut fragmentBuffer, charset, &mut result);

    result
}

fn maybeAppendFragment(fragmentBuffer: &mut Vec<u8>, charset: &str, result: &mut String) {
    if !fragmentBuffer.is_empty() {
        let fragmentBytes = fragmentBuffer.clone();
        let fragment;
        if charset.is_empty() {
            fragment = String::from_utf8(fragmentBytes).unwrap_or_else(|_| String::default());
            // fragment = new String(fragmentBytes, StandardCharsets.UTF_8);
        } else if let Some(enc) = CharacterSet::get_character_set_by_name(charset) {
            fragment = if let Ok(encoded_result) = enc.decode(&fragmentBytes) {
                encoded_result
            } else {
                String::from_utf8(fragmentBytes).unwrap_or_else(|_| String::default())
            }
        } else {
            fragment = String::from_utf8(fragmentBytes).unwrap_or_else(|_| String::default())
        }
        fragmentBuffer.clear();
        result.push_str(&fragment);
    }
}

pub fn matchSingleVCardPrefixedField(
    prefix: &str,
    rawText: &str,
    trim: bool,
    parseFieldDivider: bool,
) -> Option<Vec<String>> {
    let values = matchVCardPrefixedField(prefix, rawText, trim, parseFieldDivider)?;
    if values.is_empty() {
        return None;
    }
    Some(values.first()?.clone())
    // return values == null || values.isEmpty() ? null : values.get(0);
}

fn toPrimaryValue(list: Option<Vec<String>>) -> String {
    if let Some(l) = list {
        if l.is_empty() {
            String::default()
        } else {
            l.first().unwrap_or(&String::default()).clone()
        }
    } else {
        String::default()
    }
}

fn toPrimaryValues(lists: Option<Vec<Vec<String>>>) -> Vec<String> {
    let local_lists = lists.unwrap_or_default();
    if local_lists.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(local_lists.len()); // new ArrayList<>(lists.size());
    for list in local_lists {
        // for (List<String> list : lists) {
        let position = if list.len() > 1 { list.len() - 1 } else { 0 };
        if let Some(value) = list.get(position) {
            if !value.is_empty() {
                result.push(value.clone());
            }
        }
    }

    result
}

fn toTypes(lists: Option<Vec<Vec<String>>>) -> Vec<String> {
    let local_lists = lists.unwrap_or_default();
    if local_lists.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(local_lists.len()); //new ArrayList<>(lists.size());
    for list in local_lists {
        // for (List<String> list : lists) {
        if let Some(value) = list.first() {
            if !value.is_empty() {
                let mut v_type = String::new();
                let final_value = list.last().unwrap_or(&String::default()).clone();
                if !final_value.is_empty() {
                    for i in 0..list.len() - 1 {
                        // for (int i = 1; i < list.size(); i++) {
                        let metadatum = list.get(i).unwrap_or(&String::default()).clone();
                        if let Some(equals) = metadatum.find('=') {
                            if "TYPE" == (metadatum[0..equals]).to_uppercase() {
                                v_type = metadatum[equals + 1..].to_owned();
                                break;
                            }
                        } else {
                            // if (equals < 0) {
                            // take the whole thing as a usable label
                            metadatum.clone_into(&mut v_type);
                            break;
                        }
                    }
                    result.push(v_type);
                }
            }
        }
    }

    result
}

fn isLikeVCardDate(value: &str) -> bool {
    // let rg = Regex::new(VCARD_LIKE_DATE).unwrap();
    let matches = if let Some(mtch) = VCARD_LIKE_DATE.find(value) {
        mtch.start() == 0 && mtch.end() == value.len()
    } else {
        false
    };

    value.is_empty() || matches
}

/**
 * Formats name fields of the form "Public;John;Q.;Reverend;III" into a form like
 * "Reverend John Q. Public III".
 *
 * @param names name values to format, in place
 */
fn formatNames(names: &mut Vec<Vec<String>>) {
    if !names.is_empty() {
        for list in names {
            // for (List<String> list : names) {
            let mut pos = 0;
            while let Some(_fnd) = list.get(pos).unwrap_or(&String::default()).find('=') {
                pos += 1;
            }
            let name = list.get(pos).unwrap_or(&String::default()).clone();

            let mut components = vec![String::default(); 5];
            let mut start = 0;
            let mut end = 0;
            let mut componentIndex = 0;
            while componentIndex < components.len() - 1 && end < name.len() {
                end = if let Some(pos) = name[start..].find(';') {
                    pos + start
                } else {
                    break;
                };
                name[start..end].clone_into(&mut components[componentIndex]);
                componentIndex += 1;
                start = end + 1;
            }
             name[start..].clone_into(&mut components[componentIndex]);
            let mut newName = String::with_capacity(100);
            maybeAppendComponent(&components, 3, &mut newName);
            maybeAppendComponent(&components, 1, &mut newName);
            maybeAppendComponent(&components, 2, &mut newName);
            maybeAppendComponent(&components, 0, &mut newName);
            maybeAppendComponent(&components, 4, &mut newName);
             newName.trim().clone_into(&mut list[pos]);
        }
    }
}

fn maybeAppendComponent(components: &[String], i: usize, newName: &mut String) {
    if !components[i].is_empty() {
        if !newName.is_empty() {
            newName.push(' ');
        }
        newName.push_str(&components[i]);
    }
}
