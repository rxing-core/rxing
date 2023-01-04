/*
 * Copyright 2014 ZXing authors
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

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.RXingResult;

// import java.util.regex.Pattern;

use regex::Regex;

use crate::{
    client::result::VINParsedRXingResult, exceptions::Exceptions, BarcodeFormat, RXingResult,
};

use super::ParsedClientResult;

use lazy_static::lazy_static;

lazy_static! {
    static ref IOQ_MATCHER: Regex = Regex::new(IOQ).unwrap();
    static ref AZ09_MATCHER: Regex = Regex::new(AZ09).unwrap();
}

/**
 * Detects a result that is likely a vehicle identification number.
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    if result.getBarcodeFormat() != &BarcodeFormat::CODE_39 {
        return None;
    }

    let raw_text_res = result.getText().trim();
    let raw_text = IOQ_MATCHER.replace_all(raw_text_res, "").to_string();
    // rawText = IOQ.matcher(rawText).replaceAll("").trim();
    AZ09_MATCHER.find(&raw_text)?;
    // if !AZ09.matcher(rawText).matches() {
    //   return null;
    // }
    let check_cs = check_checksum(&raw_text).unwrap_or(false);
    if !check_cs {
        return None;
    }
    let wmi = &raw_text[..3];
    // try {
    // if (!checkChecksum(rawText)) {
    //   return null;
    // }
    // String wmi = rawText.substring(0, 3);
    let country_code = country_code(wmi).unwrap_or("");
    let model_year = model_year(raw_text.chars().nth(9).unwrap_or('_'));
    if model_year.is_err() {
        return None;
    }
    Some(ParsedClientResult::VINResult(VINParsedRXingResult::new(
        raw_text.to_owned(),
        wmi.to_owned(),
        raw_text[3..9].to_owned(),
        raw_text[9..17].to_owned(),
        country_code.to_owned(),
        raw_text[3..8].to_owned(),
        model_year.unwrap(),
        raw_text.chars().nth(10).unwrap(),
        raw_text[11..].to_owned(),
    )))
    //   return new VINParsedRXingResult(rawText,
    //       wmi,
    //       rawText.substring(3, 9),
    //       rawText.substring(9, 17),
    //       countryCode(wmi),
    //       rawText.substring(3, 8),
    //       modelYear(rawText.charAt(9)),
    //       rawText.charAt(10),
    //       rawText.substring(11));
    // } catch (IllegalArgumentException iae) {
    //   return null;
    // }
}

const IOQ: &str = "[IOQ]";
const AZ09: &str = "[A-Z0-9]{17}";

fn check_checksum(vin: &str) -> Result<bool, Exceptions> {
    let mut sum = 0;
    for i in 0..vin.len() {
        // for (int i = 0; i < vin.length(); i++) {
        sum += vin_position_weight(i + 1)? as u32 * vin_char_value(vin.chars().nth(i).unwrap())?;
    }
    let check_to_char = vin.chars().nth(8).unwrap();
    let expected_check_char = check_char((sum % 11) as u8)?;
    Ok(check_to_char == expected_check_char)
}

fn vin_char_value(c: char) -> Result<u32, Exceptions> {
    match c {
        'A'..='I' => Ok((c as u8 as u32 - b'A' as u32) + 1),
        'J'..='R' => Ok((c as u8 as u32 - b'J' as u32) + 1),
        'S'..='Z' => Ok((c as u8 as u32 - b'S' as u32) + 2),
        '0'..='9' => Ok(c as u8 as u32 - b'0' as u32),
        _ => Err(Exceptions::IllegalArgumentException(Some(
            "vin char out of range".to_owned(),
        ))),
    }
    // if c >= 'A' && c <= 'I' {
    //     return Ok((c as u8 as u32 - 'A' as u8 as u32) + 1);
    // }
    // if c >= 'J' && c <= 'R' {
    //     return Ok((c as u8 as u32 - 'J' as u8 as u32) + 1);
    // }
    // if c >= 'S' && c <= 'Z' {
    //     return Ok((c as u8 as u32 - 'S' as u8 as u32) + 2);
    // }
    // if c >= '0' && c <= '9' {
    //     return Ok(c as u8 as u32 - '0' as u8 as u32);
    // }
    // Err(Exceptions::IllegalArgumentException(
    //     "vin char out of range".to_owned(),
    // ))
}

fn vin_position_weight(position: usize) -> Result<usize, Exceptions> {
    match position {
        1..=7 => Ok(9 - position),
        8 => Ok(10),
        9 => Ok(0),
        10..=17 => Ok(19 - position),
        _ => Err(Exceptions::IllegalArgumentException(Some(
            "vin position weight out of bounds".to_owned(),
        ))),
    }
    // if position >= 1 && position <= 7 {
    //   return 9 - position;
    // }
    // if position == 8 {
    //   return 10;
    // }
    // if position == 9 {
    //   return 0;
    // }
    // if position >= 10 && position <= 17 {
    //   return 19 - position;
    // }
    // throw new IllegalArgumentException();
}

fn check_char(remainder: u8) -> Result<char, Exceptions> {
    match remainder {
        0..=9 => Ok((b'0' + remainder) as char),
        10 => Ok('X'),
        _ => Err(Exceptions::IllegalArgumentException(Some(
            "remainder too high".to_owned(),
        ))),
    }
    // if remainder < 10 {
    //     return Ok(('0' as u8 + remainder) as char);
    // }
    // if remainder == 10 {
    //     return Ok('X');
    // }
    // Err(Exceptions::IllegalArgumentException(
    //     "remainder too high".to_owned(),
    // ))
}

fn model_year(c: char) -> Result<u32, Exceptions> {
    match c {
        'E'..='H' => Ok((c as u8 as u32 - b'E' as u32) + 1984),
        'J'..='N' => Ok((c as u8 as u32 - b'J' as u32) + 1988),
        'P' => Ok(1993),
        'R'..='T' => Ok((c as u8 as u32 - b'R' as u32) + 1994),
        'V'..='Y' => Ok((c as u8 as u32 - b'V' as u32) + 1997),
        '1'..='9' => Ok((c as u8 as u32 - b'1' as u32) + 2001),
        'A'..='D' => Ok((c as u8 as u32 - b'A' as u32) + 2010),
        _ => Err(Exceptions::IllegalArgumentException(Some(String::from(
            "model year argument out of range",
        )))),
    }
    // if c >= 'E' && c <= 'H' {
    //   return (c - 'E') + 1984;
    // }
    // if c >= 'J' && c <= 'N' {
    //   return (c - 'J') + 1988;
    // }
    // if c == 'P' {
    //   return 1993;
    // }
    // if c >= 'R' && c <= 'T' {
    //   return (c - 'R') + 1994;
    // }
    // if c >= 'V' && c <= 'Y' {
    //   return (c - 'V') + 1997;
    // }
    // if c >= '1' && c <= '9' {
    //   return (c - '1') + 2001;
    // }
    // if c >= 'A' && c <= 'D' {
    //   return (c - 'A') + 2010;
    // }
    // throw new IllegalArgumentException();
}

fn country_code(wmi: &str) -> Option<&'static str> {
    let c1 = wmi.chars().next().unwrap();
    let c2 = wmi.chars().nth(1).unwrap();
    match c1 {
        '1' | '4' | '5' => Some("US"),
        '2' => Some("CA"),
        '3' if ('A'..='W').contains(&c2) => Some("MX"),
        '9' if (('A'..='E').contains(&c2) || ('3'..='9').contains(&c2)) => Some("BR"),
        'J' if ('A'..='T').contains(&c2) => Some("JP"),
        'K' if ('L'..='R').contains(&c2) => Some("KO"),
        'L' => Some("CN"),
        'M' if ('A'..='E').contains(&c2) => Some("IN"),
        'S' if ('A'..='M').contains(&c2) => Some("UK"),
        'S' if ('N'..='T').contains(&c2) => Some("DE"),
        'V' if ('F'..='R').contains(&c2) => Some("FR"),
        'V' if ('S'..='W').contains(&c2) => Some("ES"),
        'W' => Some("DE"),
        'X' if (c2 == '0' || ('3'..='9').contains(&c2)) => Some("RU"),
        'Z' if ('A'..='R').contains(&c2) => Some("IT"),
        _ => None,
    }
}
