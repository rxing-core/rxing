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

// package com.google.zxing.client.result;

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.RXingResult;
// import org.junit.Assert;
// import org.junit.Before;
// import org.junit.Test;

// import java.text.DateFormat;
// import java.text.SimpleDateFormat;
// import java.util.Locale;
// import java.util.TimeZone;

// /**
//  * Tests {@link CalendarParsedRXingResult}.
//  *
//  * @author Sean Owen
//  */
// public final class CalendarParsedRXingResultTestCase extends Assert {

use chrono::NaiveDateTime;

use crate::{
    client::result::{ParsedClientResult, ParsedRXingResultType},
    BarcodeFormat, RXingResult,
};

use super::{ParsedRXingResult, ResultParser};

const EPSILON: f64 = 1.0E-10;
// const DATE_FORMAT_STRING: &'static str = "yyyyMMdd'T'HHmmss'Z'";

// private static DateFormat makeGMTFormat() {
//   DateFormat format = new SimpleDateFormat("yyyyMMdd'T'HHmmss'Z'", Locale.ENGLISH);
//   format.setTimeZone(TimeZone.getTimeZone("GMT"));
//   return format;
// }

// @Before
// public void setUp() {
//   Locale.setDefault(Locale.ENGLISH);
//   TimeZone.setDefault(TimeZone.getTimeZone("GMT"));
// }

#[test]
fn testStartEnd() {
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nDTEND:20080505T234555Z\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "", "20080504T123456Z", "20080505T234555Z");
}

#[test]
fn testNoVCalendar() {
    doTestShort(
        "BEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nDTEND:20080505T234555Z\r\nEND:VEVENT",
        "",
        "",
        "",
        "20080504T123456Z",
        "20080505T234555Z",
    );
}

#[test]
fn testStart() {
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "", "20080504T123456Z", "");
}

#[test]
fn testDuration() {
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nDURATION:P1D\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "", "20080504T123456Z", "20080505T123456Z");
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nDURATION:P1DT2H3M4S\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "", "20080504T123456Z", "20080505T143800Z");
}

#[test]
fn testSummary() {
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504T123456Z\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "foo", "", "20080504T123456Z", "");
}

#[test]
fn testLocation() {
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nLOCATION:Miami\r\nDTSTART:20080504T123456Z\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "Miami", "20080504T123456Z", "");
}

#[test]
fn testDescription() {
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nDESCRIPTION:This is a test\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "This is a test", "", "", "20080504T123456Z", "");
    doTestShort(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nDESCRIPTION:This is a test\r\n\t with a continuation\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "This is a test with a continuation", "", "", "20080504T123456Z", "");
}

#[test]
fn testGeo() {
    doTest(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nGEO:-12.345;-45.678\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "", "20080504T123456Z", "", "", &Vec::new(), -12.345, -45.678);
}

#[test]
fn testBadGeo() {
    // Not parsed as VEVENT
    let fakeRXingResult = RXingResult::new(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nGEO:-12.345\r\nEND:VEVENT\r\nEND:VCALENDAR",
        Vec::new(),
        Vec::new(),
        BarcodeFormat::QR_CODE,
    );
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::TEXT, result.getType());
}

#[test]
fn testOrganizer() {
    doTest(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nORGANIZER:mailto:bob@example.org\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "", "20080504T123456Z", "", "bob@example.org", &Vec::new(), f64::NAN, f64::NAN);
}

#[test]
fn testAttendees() {
    doTest(
        "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20080504T123456Z\r\nATTENDEE:mailto:bob@example.org\r\nATTENDEE:mailto:alice@example.org\r\nEND:VEVENT\r\nEND:VCALENDAR",
        "", "", "", "20080504T123456Z", "", "",
        &vec!["bob@example.org", "alice@example.org"], f64::NAN, f64::NAN);
}

#[test]
fn testVEventEscapes() {
    doTestShort("BEGIN:VEVENT\nCREATED:20111109T110351Z\nLAST-MODIFIED:20111109T170034Z\nDTSTAMP:20111109T170034Z\nUID:0f6d14ef-6cb7-4484-9080-61447ccdf9c2\nSUMMARY:Summary line\nCATEGORIES:Private\nDTSTART;TZID=Europe/Vienna:20111110T110000\nDTEND;TZID=Europe/Vienna:20111110T120000\nLOCATION:Location\\, with\\, escaped\\, commas\nDESCRIPTION:Meeting with a friend\\nlook at homepage first\\n\\n\n  \\n\nSEQUENCE:1\nX-MOZ-GENERATION:1\nEND:VEVENT",
           "Meeting with a friend\nlook at homepage first\n\n\n  \n",
           "Summary line",
           "Location, with, escaped, commas",
           "20111110T110000Z",
           "20111110T120000Z");
}

#[test]
fn testAllDayValueDate() {
    doTestShort(
        "BEGIN:VEVENT\nDTSTART;VALUE=DATE:20111110\nDTEND;VALUE=DATE:20111110\nEND:VEVENT",
        "",
        "",
        "",
        "20111110T000000Z",
        "20111110T000000Z",
    );
}

fn doTestShort(
    contents: &str,
    description: &str,
    summary: &str,
    location: &str,
    startString: &str,
    endString: &str,
) {
    doTest(
        contents,
        description,
        summary,
        location,
        startString,
        endString,
        "",
        &Vec::new(),
        f64::NAN,
        f64::NAN,
    );
}

fn doTest(
    contents: &str,
    description: &str,
    summary: &str,
    location: &str,
    startString: &str,
    endString: &str,
    organizer: &str,
    attendees: &[&str],
    latitude: f64,
    longitude: f64,
) {
    let fakeRXingResult =
        RXingResult::new(contents, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::CALENDAR, result.getType());
    if let ParsedClientResult::CalendarEventResult(calRXingResult) = result {
        assert_eq!(description, calRXingResult.getDescription());
        assert_eq!(summary, calRXingResult.getSummary());
        assert_eq!(location, calRXingResult.getLocation());
        let dateFormat = "%Y%m%dT%H%M%SZ"; //makeGMTFormat();
        assert_eq!(
            startString,
            format_date_string(calRXingResult.getStartTimestamp(), dateFormat) // dateFormat.format(calRXingResult.getStartTimestamp())
        );
        assert_eq!(
            endString,
            if calRXingResult.getEndTimestamp() < 0i64 {
                "".to_owned()
            } else {
                format_date_string(calRXingResult.getEndTimestamp(), dateFormat)
                // dateFormat.format(calRXingResult.getEndTimestamp())
            }
        );
        assert_eq!(organizer, calRXingResult.getOrganizer());
        assert_eq!(attendees, calRXingResult.getAttendees());
        assertEqualOrNaN(latitude, calRXingResult.getLatitude());
        assertEqualOrNaN(longitude, calRXingResult.getLongitude());
    } else {
        panic!("Expected Calendar");
    }
}

fn assertEqualOrNaN(expected: f64, actual: f64) {
    if expected.is_nan() {
        assert!(actual.is_nan());
    } else {
        assert!(expected - actual < EPSILON && actual - expected < EPSILON);
        // assert_eq!(expected, actual, EPSILON);
    }
}

fn format_date_string(timestamp: i64, format_string: &str) -> String {
    if let Some(dtm) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
        dtm.format(format_string).to_string()
    } else {
        String::from("")
    }
    // DateTime::from(timestamp,0).with_timezone(Utc).format(format_string).to_string()
}
