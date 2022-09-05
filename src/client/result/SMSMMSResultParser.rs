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
    let rawText = ResultParser::getMassagedText(result);
    if !(rawText.starts_with("sms:")
        || rawText.starts_with("SMS:")
        || rawText.starts_with("mms:")
        || rawText.starts_with("MMS:"))
    {
        return None;
    }

    let mut subject = "".to_owned();
    let mut body = "".to_owned();
    let mut querySyntax = false;

    // Check up front if this is a URI syntax string with query arguments
    if let Some(nameValuePairs) = ResultParser::parseNameValuePairs(&rawText) {
        if !nameValuePairs.is_empty() {
            subject = String::from(nameValuePairs.get("subject").unwrap_or(&"".to_owned()));
            body = String::from(nameValuePairs.get("body").unwrap_or(&"".to_owned()));
            querySyntax = true;
        }
    }

    // Drop sms, query portion
    let queryStart = rawText[4..].find('?');
    let sms_uriwithout_query;
    // If it's not query syntax, the question mark is part of the subject or message
    if queryStart.is_none() || !querySyntax {
        sms_uriwithout_query = &rawText[4..];
    } else {
        sms_uriwithout_query = &rawText[4..4 + queryStart.unwrap_or(0)];
    }

    let mut lastComma: i32 = -1;
    let mut comma: i32 = sms_uriwithout_query[(lastComma + 1) as usize..]
        .find(',')
        .unwrap_or(0) as i32;
    let mut numbers = Vec::with_capacity(1);
    let mut vias = Vec::with_capacity(1);
    while comma > lastComma {
        comma = sms_uriwithout_query[(lastComma + 1) as usize..]
            .find(',')
            .unwrap_or(0) as i32; //sms_uriwithout_query.indexOf(',', lastComma + 1);
        let numberPart =
            &sms_uriwithout_query[(lastComma + 1) as usize..(lastComma + 1 + comma) as usize];
        addNumberVia(&mut numbers, &mut vias, numberPart);
        lastComma = comma;
    }
    addNumberVia(
        &mut numbers,
        &mut vias,
        &sms_uriwithout_query[(lastComma + 1) as usize..],
    );

    Some(ParsedClientResult::SMSResult(
        SMSParsedRXingResult::with_arrays(numbers, vias, subject.to_owned(), body.to_owned()),
    ))

    // return new SMSParsedRXingResult(numbers.toArray(EMPTY_STR_ARRAY),
    //                            vias.toArray(EMPTY_STR_ARRAY),
    //                            subject,
    //                            body);
}

fn addNumberVia(numbers: &mut Vec<String>, vias: &mut Vec<String>, numberPart: &str) {
    if let Some(numberEnd) = numberPart.find(';') {
        // if numberEnd < 0 {
        numbers.push(numberPart[..numberEnd].to_string());
        let maybeVia = &numberPart[numberEnd + 1..];
        let via = if maybeVia.starts_with("via=") {
            &maybeVia[..4]
        } else {
            ""
        };
        vias.push(via.to_owned());
    } else {
        numbers.push(numberPart.to_owned());
        vias.push("".to_owned());
    }
}

// }
