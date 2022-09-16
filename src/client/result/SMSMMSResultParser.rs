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

// package com.google.zxing.client.result;

// import com.google.zxing.RXingResult;

// import java.util.ArrayList;
// import java.util.Collection;
// import java.util.List;
// import java.util.Map;

/**
 * <p>Parses an "sms:" URI result, which specifies a number to SMS.
 * See <a href="http://tools.ietf.org/html/rfc5724"> RFC 5724</a> on this.</p>
 *
 * <p>This class supports "via" syntax for numbers, which is not part of the spec.
 * For example "+12125551212;via=+12124440101" may appear as a number.
 * It also supports a "subject" query parameter, which is not mentioned in the spec.
 * These are included since they were mentioned in earlier IETF drafts and might be
 * used.</p>
 *
 * <p>This actually also parses URIs starting with "mms:" and treats them all the same way,
 * and effectively converts them to an "sms:" URI for purposes of forwarding to the platform.</p>
 *
 * @author Sean Owen
 */
// public final class SMSMMSRXingResultParser extends RXingResultParser {
use crate::RXingResult;

use super::{ParsedClientResult, ResultParser, SMSParsedRXingResult};

// @Override
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let raw_text = ResultParser::getMassagedText(result);
    if !(raw_text.starts_with("sms:")
        || raw_text.starts_with("SMS:")
        || raw_text.starts_with("mms:")
        || raw_text.starts_with("MMS:"))
    {
        return None;
    }

    let mut subject = "".to_owned();
    let mut body = "".to_owned();
    let mut querySyntax = false;

    // Check up front if this is a URI syntax string with query arguments
    if let Some(name_value_pairs) = ResultParser::parseNameValuePairs(&raw_text) {
        if !name_value_pairs.is_empty() {
            subject = String::from(name_value_pairs.get("subject").unwrap_or(&"".to_owned()));
            body = String::from(name_value_pairs.get("body").unwrap_or(&"".to_owned()));
            querySyntax = true;
        }
    }

    // Drop sms, query portion
    let query_start = raw_text[4..].find('?');
    let sms_uriwithout_query;
    // If it's not query syntax, the question mark is part of the subject or message
    if query_start.is_none() || !querySyntax {
        sms_uriwithout_query = &raw_text[4..];
    } else {
        sms_uriwithout_query = &raw_text[4..4 + query_start.unwrap_or(0)];
    }

    let mut last_comma: i32 = -1;
    let mut comma: i32 = sms_uriwithout_query[(last_comma + 1) as usize..]
        .find(',')
        .unwrap_or(0) as i32;
    let mut numbers = Vec::with_capacity(1);
    let mut vias = Vec::with_capacity(1);
    while comma > last_comma {
        comma = sms_uriwithout_query[(last_comma + 1) as usize..]
            .find(',')
            .unwrap_or(0) as i32; //sms_uriwithout_query.indexOf(',', lastComma + 1);
        let number_part =
            &sms_uriwithout_query[(last_comma + 1) as usize..(last_comma + 1 + comma) as usize];
        add_number_via(&mut numbers, &mut vias, number_part);
        last_comma = comma;
    }
    add_number_via(
        &mut numbers,
        &mut vias,
        &sms_uriwithout_query[(if last_comma > 0 { last_comma + 1} else {last_comma}) as usize..],
    );

    Some(ParsedClientResult::SMSResult(
        SMSParsedRXingResult::with_arrays(numbers, vias, subject.to_owned(), body.to_owned()),
    ))

    // return new SMSParsedRXingResult(numbers.toArray(EMPTY_STR_ARRAY),
    //                            vias.toArray(EMPTY_STR_ARRAY),
    //                            subject,
    //                            body);
}

fn add_number_via(numbers: &mut Vec<String>, vias: &mut Vec<String>, number_part: &str) {
    if number_part.is_empty() {
        return
    }
    if let Some(number_end) = number_part.find(';') {
        // if numberEnd < 0 {
        numbers.push(number_part[..number_end].to_string());
        let maybe_via = &number_part[number_end + 1..];
        let via = if maybe_via.starts_with("via=") {
            &maybe_via[4..]
        } else {
            ""
        };
        if !via.is_empty() {
            vias.push(via.to_owned());
        }
    } else {
        numbers.push(number_part.to_owned());
        //vias.push("".to_owned());
    }
}

// }
