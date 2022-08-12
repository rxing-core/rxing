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
 * Represents a parsed result that encodes a calendar event at a certain time, optionally
 * with attendees and a location.
 *
 * @author Sean Owen
 */

 const RFC2445_DURATION: Pattern = Pattern::compile("P(?:(\\d+)W)?(?:(\\d+)D)?(?:T(?:(\\d+)H)?(?:(\\d+)M)?(?:(\\d+)S)?)?");

 const RFC2445_DURATION_FIELD_UNITS: vec![Vec<i64>; 5] = vec![// 1 week
7 * 24 * 60 * 60 * 1000, // 1 day
24 * 60 * 60 * 1000, // 1 hour
60 * 60 * 1000, // 1 minute
60 * 1000, // 1 second
1000, ]
;

 const DATE_TIME: Pattern = Pattern::compile("[0-9]{8}(T[0-9]{6}Z?)?");
pub struct CalendarParsedResult {
    super: ParsedResult;

     let summary: String;

     let mut start: i64;

     let start_all_day: bool;

     let mut end: i64;

     let end_all_day: bool;

     let location: String;

     let organizer: String;

     let attendees: Vec<String>;

     let description: String;

     let latitude: f64;

     let longitude: f64;
}

impl CalendarParsedResult {

    pub fn new( summary: &String,  start_string: &String,  end_string: &String,  duration_string: &String,  location: &String,  organizer: &String,  attendees: &Vec<String>,  description: &String,  latitude: f64,  longitude: f64) -> CalendarParsedResult {
        super(ParsedResultType::CALENDAR);
        let .summary = summary;
        let tryResult1 = 0;
        'try1: loop {
        {
            let .start = ::parse_date(&start_string);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( pe: &ParseException) {
                throw IllegalArgumentException::new(&pe.to_string());
            }  0 => break
        }

        if end_string == null {
             let duration_m_s: i64 = ::parse_duration_m_s(&duration_string);
            end =  if duration_m_s < 0 { -1 } else { start + duration_m_s };
        } else {
            let tryResult1 = 0;
            'try1: loop {
            {
                let .end = ::parse_date(&end_string);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( pe: &ParseException) {
                    throw IllegalArgumentException::new(&pe.to_string());
                }  0 => break
            }

        }
        let .startAllDay = start_string.length() == 8;
        let .endAllDay = end_string != null && end_string.length() == 8;
        let .location = location;
        let .organizer = organizer;
        let .attendees = attendees;
        let .description = description;
        let .latitude = latitude;
        let .longitude = longitude;
    }

    pub fn  get_summary(&self) -> String  {
        return self.summary;
    }

    /**
   * @return start time
   * @deprecated use {@link #getStartTimestamp()}
   */
    pub fn  get_start(&self) -> Date  {
        return Date::new(self.start);
    }

    /**
   * @return start time
   * @see #getEndTimestamp()
   */
    pub fn  get_start_timestamp(&self) -> i64  {
        return self.start;
    }

    /**
   * @return true if start time was specified as a whole day
   */
    pub fn  is_start_all_day(&self) -> bool  {
        return self.start_all_day;
    }

    /**
   * @return event end {@link Date}, or {@code null} if event has no duration
   * @deprecated use {@link #getEndTimestamp()}
   */
    pub fn  get_end(&self) -> Date  {
        return  if self.end < 0 { null } else { Date::new(self.end) };
    }

    /**
   * @return event end {@link Date}, or -1 if event has no duration
   * @see #getStartTimestamp()
   */
    pub fn  get_end_timestamp(&self) -> i64  {
        return self.end;
    }

    /**
   * @return true if end time was specified as a whole day
   */
    pub fn  is_end_all_day(&self) -> bool  {
        return self.end_all_day;
    }

    pub fn  get_location(&self) -> String  {
        return self.location;
    }

    pub fn  get_organizer(&self) -> String  {
        return self.organizer;
    }

    pub fn  get_attendees(&self) -> Vec<String>  {
        return self.attendees;
    }

    pub fn  get_description(&self) -> String  {
        return self.description;
    }

    pub fn  get_latitude(&self) -> f64  {
        return self.latitude;
    }

    pub fn  get_longitude(&self) -> f64  {
        return self.longitude;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(100);
        maybe_append(&self.summary, &result);
        maybe_append(&::format(self.start_all_day, self.start), &result);
        maybe_append(&::format(self.end_all_day, self.end), &result);
        maybe_append(&self.location, &result);
        maybe_append(&self.organizer, &result);
        maybe_append(&self.attendees, &result);
        maybe_append(&self.description, &result);
        return result.to_string();
    }

    /**
   * Parses a string as a date. RFC 2445 allows the start and end fields to be of type DATE (e.g. 20081021)
   * or DATE-TIME (e.g. 20081021T123000 for local time, or 20081021T123000Z for UTC).
   *
   * @param when The string to parse
   * @throws ParseException if not able to parse as a date
   */
    fn  parse_date( when: &String) -> /*  throws ParseException */Result<i64, Rc<Exception>>   {
        if !DATE_TIME::matcher(&when)::matches() {
            throw ParseException::new(&when, 0);
        }
        if when.length() == 8 {
            // Show only year/month/day
             let format: DateFormat = SimpleDateFormat::new("yyyyMMdd", Locale::ENGLISH);
            // For dates without a time, for purposes of interacting with Android, the resulting timestamp
            // needs to be midnight of that day in GMT. See:
            // http://code.google.com/p/android/issues/detail?id=8330
            format.set_time_zone(&TimeZone::get_time_zone("GMT"));
            return Ok(format.parse(&when).get_time());
        }
        // The when string can be local time, or UTC if it ends with a Z
        if when.length() == 16 && when.char_at(15) == 'Z' {
             let mut milliseconds: i64 = ::parse_date_time_string(&when.substring(0, 15));
             let calendar: Calendar = GregorianCalendar::new();
            // Account for time zone difference
            milliseconds += calendar.get(Calendar::ZONE_OFFSET);
            // Might need to correct for daylight savings time, but use target time since
            // now might be in DST but not then, or vice versa
            calendar.set_time(Date::new(milliseconds));
            return Ok(milliseconds + calendar.get(Calendar::DST_OFFSET));
        }
        return Ok(::parse_date_time_string(&when));
    }

    fn  format( all_day: bool,  date: i64) -> String  {
        if date < 0 {
            return null;
        }
         let format: DateFormat =  if all_day { DateFormat::get_date_instance(DateFormat::MEDIUM) } else { DateFormat::get_date_time_instance(DateFormat::MEDIUM, DateFormat::MEDIUM) };
        return format.format(date);
    }

    fn  parse_duration_m_s( duration_string: &CharSequence) -> i64  {
        if duration_string == null {
            return -1;
        }
         let m: Matcher = RFC2445_DURATION::matcher(&duration_string);
        if !m.matches() {
            return -1;
        }
         let duration_m_s: i64 = 0;
         {
             let mut i: i32 = 0;
            while i < RFC2445_DURATION_FIELD_UNITS.len() {
                {
                     let field_value: String = m.group(i + 1);
                    if field_value != null {
                        duration_m_s += RFC2445_DURATION_FIELD_UNITS[i] * Integer::parse_int(&field_value);
                    }
                }
                i += 1;
             }
         }

        return duration_m_s;
    }

    fn  parse_date_time_string( date_time_string: &String) -> /*  throws ParseException */Result<i64, Rc<Exception>>   {
         let format: DateFormat = SimpleDateFormat::new("yyyyMMdd'T'HHmmss", Locale::ENGLISH);
        return Ok(format.parse(&date_time_string).get_time());
    }
}

