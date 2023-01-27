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

// import java.text.DateFormat;
// import java.text.ParseException;
// import java.text.SimpleDateFormat;
// import java.util.Calendar;
// import java.util.Date;
// import java.util.GregorianCalendar;
// import java.util.Locale;
// import java.util.TimeZone;
// import java.util.regex.Matcher;
// import java.util.regex.Pattern;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::exceptions::Exceptions;

use super::{maybe_append_multiple, maybe_append_string, ParsedRXingResult, ParsedRXingResultType};

// const RFC2445_DURATION: &'static str =
//     "P(?:(\\d+)W)?(?:(\\d+)D)?(?:T(?:(\\d+)H)?(?:(\\d+)M)?(?:(\\d+)S)?)?";
const RFC2445_DURATION_FIELD_UNITS: [i64; 5] = [
    7 * 24 * 60 * 60 * 1000i64, // 1 week
    24 * 60 * 60 * 1000i64,     // 1 day
    60 * 60 * 1000i64,          // 1 hour
    60 * 1000i64,               // 1 minute
    1000i64,                    // 1 second
];

static DATE_TIME: Lazy<Regex> = Lazy::new(|| Regex::new("[0-9]{8}(T[0-9]{6}Z?)?").unwrap());
static RFC2445_DURATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new("P(?:(\\d+)W)?(?:(\\d+)D)?(?:T(?:(\\d+)H)?(?:(\\d+)M)?(?:(\\d+)S)?)?").unwrap()
});

// const DATE_TIME: &'static str = "[0-9]{8}(T[0-9]{6}Z?)?";

/**
 * Represents a parsed result that encodes a calendar event at a certain time, optionally
 * with attendees and a location.
 *
 * @author Sean Owen
 */
#[derive(Debug)]
pub struct CalendarParsedRXingResult {
    summary: String,
    start: i64,
    startAllDay: bool,
    end: i64,
    endAllDay: bool,
    location: String,
    organizer: String,
    attendees: Vec<String>,
    description: String,
    latitude: f64,
    longitude: f64,
}

impl ParsedRXingResult for CalendarParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::CALENDAR
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(100);
        maybe_append_string(&self.summary, &mut result);
        maybe_append_string(
            &Self::format_event(self.startAllDay, self.start),
            &mut result,
        );
        maybe_append_string(&Self::format_event(self.endAllDay, self.end), &mut result);
        maybe_append_string(&self.location, &mut result);
        maybe_append_string(&self.organizer, &mut result);
        maybe_append_multiple(&self.attendees, &mut result);
        maybe_append_string(&self.description, &mut result);

        result
    }
}

impl CalendarParsedRXingResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        summary: String,
        startString: String,
        endString: String,
        durationString: String,
        location: String,
        organizer: String,
        attendees: Vec<String>,
        description: String,
        latitude: f64,
        longitude: f64,
    ) -> Result<Self, Exceptions> {
        let start = Self::parseDate(startString.clone())?;
        let end = if endString.is_empty() {
            let durationMS = Self::parseDurationMS(&durationString);
            if durationMS < 0i64 {
                -1i64
            } else {
                start + (durationMS / 1000)
            }
        } else {
            Self::parseDate(endString.clone())?
        };

        // try {
        //   this.start = parseDate(startString);
        // } catch (ParseException pe) {
        //   throw new IllegalArgumentException(pe.toString());
        // }

        // if (endString == null) {
        //   long durationMS = parseDurationMS(durationString);
        //   end = durationMS < 0L ? -1L : start + durationMS;
        // } else {
        //   try {
        //     this.end = parseDate(endString);
        //   } catch (ParseException pe) {
        //     throw new IllegalArgumentException(pe.toString());
        //   }
        // }

        let startAllDay = startString.len() == 8;
        let endAllDay = !endString.is_empty() && endString.len() == 8;

        Ok(Self {
            summary,
            start,
            startAllDay,
            end,
            endAllDay,
            location,
            organizer,
            attendees,
            description,
            latitude,
            longitude,
        })
    }

    /**
     * Parses a string as a date. RFC 2445 allows the start and end fields to be of type DATE (e.g. 20081021)
     * or DATE-TIME (e.g. 20081021T123000 for local time, or 20081021T123000Z for UTC).
     *
     * @param when The string to parse
     * @throws ParseException if not able to parse as a date
     */
    fn parseDate(when: String) -> Result<i64, Exceptions> {
        // let date_time_regex = Regex::new(DATE_TIME).unwrap();
        if !DATE_TIME.is_match(&when) {
            return Err(Exceptions::ParseException(Some(when)));
        }
        if when.len() == 8 {
            // Show only year/month/day
            let date_format_string = "%Y%m%dT%H%M%SZ";
            // DateFormat format = new SimpleDateFormat("yyyyMMdd", Locale.ENGLISH);
            // For dates without a time, for purposes of interacting with Android, the resulting timestamp
            // needs to be midnight of that day in GMT. See:
            // http://code.google.com/p/android/issues/detail?id=8330
            return match Utc.datetime_from_str(&format!("{}T000000Z", &when,), date_format_string) {
                Ok(dtm) => Ok(dtm.timestamp()),
                Err(e) => panic!("{}", e),
                // Err(e) => Err(Exceptions::ParseException(format!(
                //     "couldn't parse string: {}",
                //     e
                // ))),
            };
            // let dtm = DateTime::parse_from_str(&when, date_format_string).unwrap();
            // let dtm = dtm.with_timezone(&Utc);

            // // format.setTimeZone(TimeZone.getTimeZone("GMT"));
            // // return format.parse(when).getTime();
            // return Ok(dtm.timestamp());
        }
        // The when string can be local time, or UTC if it ends with a Z
        if when.len() == 16 && when.chars().nth(15).unwrap() == 'Z' {
            return match Utc.datetime_from_str(&when, "%Y%m%dT%H%M%SZ") {
                Ok(dtm) => Ok(dtm.with_timezone(&Utc).timestamp()),
                Err(e) => Err(Exceptions::ParseException(Some(format!(
                    "couldn't parse string: {e}"
                )))),
            };
            // let dtm = DateTime::parse_from_str(&when, "%Y%m%dT%H%M%S").unwrap().with_timezone(&Utc);
            // //let milliseconds = Self::parseDateTimeString(&when[0..15]);
            // // Calendar calendar = new GregorianCalendar();
            // // // Account for time zone difference
            // // milliseconds += calendar.get(Calendar.ZONE_OFFSET);
            // // // Might need to correct for daylight savings time, but use target time since
            // // // now might be in DST but not then, or vice versa
            // // calendar.setTime(new Date(milliseconds));
            // // return milliseconds + calendar.get(Calendar.DST_OFFSET);
            // return Ok(dtm.timestamp());
        }
        // Try once more, with weird tz formatting
        if when.len() > 16 {
            let time_part = &when[..15];
            let tz_part = &when[15..];
            let tz_parsed: Tz = match tz_part.parse() {
                Ok(time_zone) => time_zone,
                Err(e) => {
                    return Err(Exceptions::ParseException(Some(format!(
                        "couldn't parse timezone '{tz_part}': {e}"
                    ))))
                }
            };
            return match Utc.datetime_from_str(time_part, "%Y%m%dT%H%M%S") {
                Ok(dtm) => Ok(dtm.with_timezone(&tz_parsed).timestamp()),
                Err(e) => Err(Exceptions::ParseException(Some(format!(
                    "couldn't parse string: {e}"
                )))),
            };
        }

        // Try a final time with an exact length
        if when.len() == 15 {
            return match Utc.datetime_from_str(&when, "%Y%m%dT%H%M%S") {
                Ok(dtm) => Ok(dtm.timestamp()),
                Err(e) => Err(Exceptions::ParseException(Some(format!(
                    "couldn't parse local time: {e}"
                )))),
            };
        }
        Self::parseDateTimeString(&when)
    }

    fn format_event(allDay: bool, date: i64) -> String {
        if date < 0 {
            return "".to_owned();
        }
        let format_string = if allDay { "%F" } else { "%c" };
        // DateFormat format = allDay
        //     ? DateFormat.getDateInstance(DateFormat.MEDIUM)
        //     : DateFormat.getDateTimeInstance(DateFormat.MEDIUM, DateFormat.MEDIUM);
        // return format.format(date);
        if let Some(dtm) = NaiveDateTime::from_timestamp_opt(date, 0) {
            dtm.format(format_string).to_string()
        } else {
            String::from("")
        }
    }

    fn parseDurationMS(durationString: &str) -> i64 {
        if durationString.is_empty() {
            return -1;
        }
        // let regex = Regex::new(RFC2445_DURATION).unwrap();
        if let Some(m) = RFC2445_DURATION.captures(durationString) {
            let mut durationMS = 0i64;
            for (i, unit) in RFC2445_DURATION_FIELD_UNITS.iter().enumerate() {
                // for i in 0..RFC2445_DURATION_FIELD_UNITS.len() {
                // for (int i = 0; i < RFC2445_DURATION_FIELD_UNITS.length; i++) {
                let fieldValue = m.get(i + 1);
                if let Some(parseable) = fieldValue {
                    let z = parseable.as_str().parse::<i64>().unwrap();
                    durationMS += unit * z;
                }
            }
            durationMS
        } else {
            -1
        }
        // if (!m.matches()) {
        //   return -1L;
        // }
        // long durationMS = 0L;
        // for (int i = 0; i < RFC2445_DURATION_FIELD_UNITS.length; i++) {
        //   String fieldValue = m.group(i + 1);
        //   if (fieldValue != null) {
        //     durationMS += RFC2445_DURATION_FIELD_UNITS[i] * Integer.parseInt(fieldValue);
        //   }
        // }
        // return durationMS;
    }

    fn parseDateTimeString(dateTimeString: &str) -> Result<i64, Exceptions> {
        if let Ok(dtm) = DateTime::parse_from_str(dateTimeString, "%Y%m%dT%H%M%S") {
            Ok(dtm.timestamp())
        } else {
            Err(Exceptions::ParseException(Some(format!(
                "Couldn't parse {dateTimeString}"
            ))))
        }
        // DateFormat format = new SimpleDateFormat("yyyyMMdd'T'HHmmss", Locale.ENGLISH);
        // return format.parse(dateTimeString).getTime();
    }

    pub fn getSummary(&self) -> &String {
        &self.summary
    }

    /**
     * @return start time
     * @see #getEndTimestamp()
     */
    pub fn getStartTimestamp(&self) -> i64 {
        self.start
    }

    /**
     * @return true if start time was specified as a whole day
     */
    pub fn isStartAllDay(&self) -> bool {
        self.startAllDay
    }

    /**
     * @return event end {@link Date}, or -1 if event has no duration
     * @see #getStartTimestamp()
     */
    pub fn getEndTimestamp(&self) -> i64 {
        self.end
    }

    /**
     * @return true if end time was specified as a whole day
     */
    pub fn isEndAllDay(&self) -> bool {
        self.endAllDay
    }

    pub fn getLocation(&self) -> &str {
        &self.location
    }

    pub fn getOrganizer(&self) -> &str {
        &self.organizer
    }

    pub fn getAttendees(&self) -> &Vec<String> {
        &self.attendees
    }

    pub fn getDescription(&self) -> &str {
        &self.description
    }

    pub fn getLatitude(&self) -> f64 {
        self.latitude
    }

    pub fn getLongitude(&self) -> f64 {
        self.longitude
    }
}

impl PartialEq for CalendarParsedRXingResult {
    fn eq(&self, other: &Self) -> bool {
        self.summary == other.summary
            && self.start == other.start
            && self.startAllDay == other.startAllDay
            && self.end == other.end
            && self.endAllDay == other.endAllDay
            && self.location == other.location
            && self.organizer == other.organizer
            && self.attendees == other.attendees
            && self.description == other.description
            && self.latitude == other.latitude
            && self.longitude == other.longitude
    }
}

impl Eq for CalendarParsedRXingResult {}
