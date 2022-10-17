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

// import java.util.List;

use crate::RXingResult;

use super::{CalendarParsedRXingResult, ParsedClientResult, ResultParser, VCardResultParser};

/**
 * Partially implements the iCalendar format's "VEVENT" format for specifying a
 * calendar event. See RFC 2445. This supports SUMMARY, LOCATION, GEO, DTSTART and DTEND fields.
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    if let None = rawText.find("BEGIN:VEVENT") {
        return None;
    }
    // if (vEventStart < 0) {
    //   return null;
    // }

    let summary = matchSingleVCardPrefixedField("SUMMARY", &rawText);
    let start = matchSingleVCardPrefixedField("DTSTART", &rawText);
    if start.is_empty() {
        return None;
    }
    let end = matchSingleVCardPrefixedField("DTEND", &rawText);
    let duration = matchSingleVCardPrefixedField("DURATION", &rawText);
    let location = matchSingleVCardPrefixedField("LOCATION", &rawText);
    let organizer = stripMailto(&matchSingleVCardPrefixedField("ORGANIZER", &rawText));

    let mut attendees = matchVCardPrefixedField("ATTENDEE", &rawText);
    if !attendees.is_empty() {
        for i in 0..attendees.len() {
            // for (int i = 0; i < attendees.length; i++) {
            attendees[i] = stripMailto(&attendees[i]);
        }
    }
    let description = matchSingleVCardPrefixedField("DESCRIPTION", &rawText);

    let geoString = matchSingleVCardPrefixedField("GEO", &rawText);
    let latitude;
    let longitude;
    if geoString.is_empty() {
        latitude = f64::NAN;
        longitude = f64::NAN;
    } else {
        if let Some(semicolon) = geoString.find(';') {
            latitude = if let Ok(l) = geoString[..semicolon].parse() {
                l
            } else {
                return None;
            };
            longitude = if let Ok(l) = geoString[semicolon + 1..].parse() {
                l
            } else {
                return None;
            }
        } else {
            return None;
        }
        // if (semicolon < 0) {
        //   return null;
        // }
        // try {
        //   latitude = Double.parseDouble(geoString.substring(0, semicolon));
        //   longitude = Double.parseDouble(geoString.substring(semicolon + 1));
        // } catch (NumberFormatException ignored) {
        //   return null;
        // }
    }

    if let Ok(cpr) = CalendarParsedRXingResult::new(
        summary,
        start,
        end,
        duration,
        location,
        organizer,
        attendees,
        description,
        latitude,
        longitude,
    ) {
        Some(ParsedClientResult::CalendarEventResult(cpr))
    } else {
        None
    }

    // try {
    //   return new CalendarParsedRXingResult(summary,
    //                                   start,
    //                                   end,
    //                                   duration,
    //                                   location,
    //                                   organizer,
    //                                   attendees,
    //                                   description,
    //                                   latitude,
    //                                   longitude);
    // } catch (IllegalArgumentException ignored) {
    //   return null;
    // }
}

fn matchSingleVCardPrefixedField(prefix: &str, rawText: &str) -> String {
    if let Some(values) =
        VCardResultParser::matchSingleVCardPrefixedField(prefix, rawText, true, false)
    {
        if values.is_empty() {
            "".to_owned()
        } else {
            let tz_mod = if values.len() > 1 {
                if let Some(v) = values.get(values.len() - 2) {
                    if let Some(tz_loc) = v.find("TZID=") {
                        v[tz_loc + 5..].to_owned()
                    } else {
                        "".to_owned()
                    }
                } else {
                    "".to_owned()
                }
            } else {
                "".to_owned()
            };
            let root_time = values.last().unwrap().clone();
            format!("{}{}", root_time, tz_mod)
        }
    } else {
        "".to_owned()
    }
    // return values == null || values.isEmpty() ? null : values.get(0);
}

fn matchVCardPrefixedField(prefix: &str, rawText: &str) -> Vec<String> {
    if let Some(values) = VCardResultParser::matchVCardPrefixedField(prefix, rawText, true, false) {
        if values.is_empty() {
            Vec::new()
        } else {
            let size = values.len();
            let mut result = vec!["".to_owned(); size]; //new String[size];
            for i in 0..size {
                // for (int i = 0; i < size; i++) {
                result[i] = values.get(i).unwrap().get(0).unwrap().clone();
            }
            result
        }
    } else {
        Vec::new()
    }
    // if values == null || values.isEmpty() {
    //   return null;
    // }
    // int size = values.size();
    // String[] result = new String[size];
    // for (int i = 0; i < size; i++) {
    //   result[i] = values.get(i).get(0);
    // }
    // return result;
}

fn stripMailto(s: &str) -> String {
    let mut s = s;
    if !s.is_empty() && (s.starts_with("mailto:") || s.starts_with("MAILTO:")) {
        s = &s[7..];
    }
    s.to_owned()
}
