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
 * Partially implements the iCalendar format's "VEVENT" format for specifying a
 * calendar event. See RFC 2445. This supports SUMMARY, LOCATION, GEO, DTSTART and DTEND fields.
 *
 * @author Sean Owen
 */
pub struct VEventResultParser {
    super: ResultParser;
}

impl VEventResultParser {

    pub fn  parse(&self,  result: &Result) -> CalendarParsedResult  {
         let raw_text: String = get_massaged_text(result);
         let v_event_start: i32 = raw_text.index_of("BEGIN:VEVENT");
        if v_event_start < 0 {
            return null;
        }
         let summary: String = ::match_single_v_card_prefixed_field("SUMMARY", &raw_text);
         let start: String = ::match_single_v_card_prefixed_field("DTSTART", &raw_text);
        if start == null {
            return null;
        }
         let end: String = ::match_single_v_card_prefixed_field("DTEND", &raw_text);
         let duration: String = ::match_single_v_card_prefixed_field("DURATION", &raw_text);
         let location: String = ::match_single_v_card_prefixed_field("LOCATION", &raw_text);
         let organizer: String = ::strip_mailto(&::match_single_v_card_prefixed_field("ORGANIZER", &raw_text));
         let mut attendees: Vec<String> = ::match_v_card_prefixed_field("ATTENDEE", &raw_text);
        if attendees != null {
             {
                 let mut i: i32 = 0;
                while i < attendees.len() {
                    {
                        attendees[i] = ::strip_mailto(attendees[i]);
                    }
                    i += 1;
                 }
             }

        }
         let description: String = ::match_single_v_card_prefixed_field("DESCRIPTION", &raw_text);
         let geo_string: String = ::match_single_v_card_prefixed_field("GEO", &raw_text);
         let mut latitude: f64;
         let mut longitude: f64;
        if geo_string == null {
            latitude = Double::NaN;
            longitude = Double::NaN;
        } else {
             let semicolon: i32 = geo_string.index_of(';');
            if semicolon < 0 {
                return null;
            }
            let tryResult1 = 0;
            'try1: loop {
            {
                latitude = Double::parse_double(&geo_string.substring(0, semicolon));
                longitude = Double::parse_double(&geo_string.substring(semicolon + 1));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( ignored: &NumberFormatException) {
                    return null;
                }  0 => break
            }

        }
        let tryResult1 = 0;
        'try1: loop {
        {
            return CalendarParsedResult::new(&summary, &start, &end, &duration, &location, &organizer, &attendees, &description, latitude, longitude);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &IllegalArgumentException) {
                return null;
            }  0 => break
        }

    }

    fn  match_single_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String) -> String  {
         let values: List<String> = VCardResultParser::match_single_v_card_prefixed_field(&prefix, &raw_text, true, false);
        return  if values == null || values.is_empty() { null } else { values.get(0) };
    }

    fn  match_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String) -> Vec<String>  {
         let values: List<List<String>> = VCardResultParser::match_v_card_prefixed_field(&prefix, &raw_text, true, false);
        if values == null || values.is_empty() {
            return null;
        }
         let size: i32 = values.size();
         let mut result: [Option<String>; size] = [None; size];
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    result[i] = values.get(i).get(0);
                }
                i += 1;
             }
         }

        return result;
    }

    fn  strip_mailto( s: &String) -> String  {
        if s != null && (s.starts_with("mailto:") || s.starts_with("MAILTO:")) {
            s = s.substring(7);
        }
        return s;
    }
}

