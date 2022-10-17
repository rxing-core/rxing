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
// import java.util.Calendar;
// import java.util.Locale;
// import java.util.TimeZone;

use chrono::{TimeZone, Utc};

use crate::{client::result::ParsedRXingResult, BarcodeFormat, RXingResult};

use super::{ParsedRXingResultType, ResultParser};

/**
 * Tests {@link ParsedRXingResult}.
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 */

// fn setUp() {
//     Locale.setDefault(Locale.ENGLISH);
//     TimeZone.setDefault(TimeZone.getTimeZone("GMT"));
//   }

#[test]
fn test_text_type() {
    do_test_rxing_result("", "", ParsedRXingResultType::TEXT);
    do_test_rxing_result("foo", "foo", ParsedRXingResultType::TEXT);
    do_test_rxing_result("Hi.", "Hi.", ParsedRXingResultType::TEXT);
    do_test_rxing_result(
        "This is a test",
        "This is a test",
        ParsedRXingResultType::TEXT,
    );
    do_test_rxing_result(
        "This is a test\nwith newlines",
        "This is a test\nwith newlines",
        ParsedRXingResultType::TEXT,
    );
    do_test_rxing_result(
        "This: a test with lots of @ nearly-random punctuation! No? OK then.",
        "This: a test with lots of @ nearly-random punctuation! No? OK then.",
        ParsedRXingResultType::TEXT,
    );
}

#[test]
fn test_bookmark_type() {
    do_test_rxing_result(
        "MEBKM:URL:google.com;;",
        "http://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "MEBKM:URL:google.com;TITLE:Google;;",
        "Google\nhttp://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "MEBKM:TITLE:Google;URL:google.com;;",
        "Google\nhttp://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "MEBKM:URL:http://google.com;;",
        "http://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "MEBKM:URL:HTTPS://google.com;;",
        "HTTPS://google.com",
        ParsedRXingResultType::URI,
    );
}

#[test]
fn test_urltotype() {
    do_test_rxing_result(
        "urlto:foo:bar.com",
        "foo\nhttp://bar.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "URLTO:foo:bar.com",
        "foo\nhttp://bar.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "URLTO::bar.com",
        "http://bar.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "URLTO::http://bar.com",
        "http://bar.com",
        ParsedRXingResultType::URI,
    );
}

#[test]
fn test_email_type() {
    do_test_rxing_result(
        "MATMSG:TO:srowen@example.org;;",
        "srowen@example.org",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result(
        "MATMSG:TO:srowen@example.org;SUB:Stuff;;",
        "srowen@example.org\nStuff",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result(
        "MATMSG:TO:srowen@example.org;SUB:Stuff;BODY:This is some text;;",
        "srowen@example.org\nStuff\nThis is some text",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result(
        "MATMSG:SUB:Stuff;BODY:This is some text;TO:srowen@example.org;;",
        "srowen@example.org\nStuff\nThis is some text",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result(
        "TO:srowen@example.org;SUB:Stuff;BODY:This is some text;;",
        "TO:srowen@example.org;SUB:Stuff;BODY:This is some text;;",
        ParsedRXingResultType::TEXT,
    );
}

#[test]
fn test_email_address_type() {
    do_test_rxing_result(
        "srowen@example.org",
        "srowen@example.org",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result(
        "mailto:srowen@example.org",
        "srowen@example.org",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result(
        "MAILTO:srowen@example.org",
        "srowen@example.org",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result(
        "srowen@example",
        "srowen@example",
        ParsedRXingResultType::EMAIL_ADDRESS,
    );
    do_test_rxing_result("srowen", "srowen", ParsedRXingResultType::TEXT);
    do_test_rxing_result(
        "Let's meet @ 2",
        "Let's meet @ 2",
        ParsedRXingResultType::TEXT,
    );
}

#[test]
fn test_address_book_type() {
    do_test_rxing_result(
        "MECARD:N:Sean Owen;;",
        "Sean Owen",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "MECARD:TEL:+12125551212;N:Sean Owen;;",
        "Sean Owen\n+12125551212",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "MECARD:TEL:+12125551212;N:Sean Owen;URL:google.com;;",
        "Sean Owen\n+12125551212\ngoogle.com",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "MECARD:TEL:+12125551212;N:Sean Owen;URL:google.com;EMAIL:srowen@example.org;",
        "Sean Owen\n+12125551212\nsrowen@example.org\ngoogle.com",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "MECARD:ADR:76 9th Ave;N:Sean Owen;URL:google.com;EMAIL:srowen@example.org;",
        "Sean Owen\n76 9th Ave\nsrowen@example.org\ngoogle.com",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "MECARD:BDAY:19760520;N:Sean Owen;URL:google.com;EMAIL:srowen@example.org;",
        "Sean Owen\nsrowen@example.org\ngoogle.com\n19760520",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "MECARD:ORG:Google;N:Sean Owen;URL:google.com;EMAIL:srowen@example.org;",
        "Sean Owen\nGoogle\nsrowen@example.org\ngoogle.com",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "MECARD:NOTE:ZXing Team;N:Sean Owen;URL:google.com;EMAIL:srowen@example.org;",
        "Sean Owen\nsrowen@example.org\ngoogle.com\nZXing Team",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "N:Sean Owen;TEL:+12125551212;;",
        "N:Sean Owen;TEL:+12125551212;;",
        ParsedRXingResultType::TEXT,
    );
}

#[test]
fn test_address_book_autype() {
    do_test_rxing_result("MEMORY:\r\n", "", ParsedRXingResultType::ADDRESSBOOK);
    do_test_rxing_result(
        "MEMORY:foo\r\nNAME1:Sean\r\n",
        "Sean\nfoo",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "TEL1:+12125551212\r\nMEMORY:\r\n",
        "+12125551212",
        ParsedRXingResultType::ADDRESSBOOK,
    );
}

#[test]
fn test_bizcard() {
    do_test_rxing_result(
        "BIZCARD:N:Sean;X:Owen;C:Google;A:123 Main St;M:+12225551212;E:srowen@example.org;",
        "Sean Owen\nGoogle\n123 Main St\n+12225551212\nsrowen@example.org",
        ParsedRXingResultType::ADDRESSBOOK,
    );
}

#[test]
fn test_upca() {
    do_test_rxing_result_long(
        "123456789012",
        "123456789012",
        ParsedRXingResultType::PRODUCT,
        BarcodeFormat::UPC_A,
    );
    do_test_rxing_result_long(
        "1234567890123",
        "1234567890123",
        ParsedRXingResultType::PRODUCT,
        BarcodeFormat::UPC_A,
    );
    do_test_rxing_result("12345678901", "12345678901", ParsedRXingResultType::TEXT);
}

#[test]
fn test_upce() {
    do_test_rxing_result_long(
        "01234565",
        "01234565",
        ParsedRXingResultType::PRODUCT,
        BarcodeFormat::UPC_E,
    );
}

#[test]
fn test_ean() {
    do_test_rxing_result_long(
        "00393157",
        "00393157",
        ParsedRXingResultType::PRODUCT,
        BarcodeFormat::EAN_8,
    );
    do_test_rxing_result("00393158", "00393158", ParsedRXingResultType::TEXT);
    do_test_rxing_result_long(
        "5051140178499",
        "5051140178499",
        ParsedRXingResultType::PRODUCT,
        BarcodeFormat::EAN_13,
    );
    do_test_rxing_result(
        "5051140178490",
        "5051140178490",
        ParsedRXingResultType::TEXT,
    );
}

#[test]
fn test_isbn() {
    do_test_rxing_result_long(
        "9784567890123",
        "9784567890123",
        ParsedRXingResultType::ISBN,
        BarcodeFormat::EAN_13,
    );
    do_test_rxing_result_long(
        "9794567890123",
        "9794567890123",
        ParsedRXingResultType::ISBN,
        BarcodeFormat::EAN_13,
    );
    do_test_rxing_result("97845678901", "97845678901", ParsedRXingResultType::TEXT);
    do_test_rxing_result("97945678901", "97945678901", ParsedRXingResultType::TEXT);
}

#[test]
fn test_uri() {
    do_test_rxing_result(
        "http://google.com",
        "http://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "google.com",
        "http://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "https://google.com",
        "https://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "HTTP://google.com",
        "HTTP://google.com",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "http://google.com/foobar",
        "http://google.com/foobar",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "https://google.com:443/foobar",
        "https://google.com:443/foobar",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "google.com:443",
        "http://google.com:443",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "google.com:443/",
        "http://google.com:443/",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "google.com:443/foobar",
        "http://google.com:443/foobar",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "http://google.com:443/foobar",
        "http://google.com:443/foobar",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "https://google.com:443/foobar",
        "https://google.com:443/foobar",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "ftp://google.com/fake",
        "ftp://google.com/fake",
        ParsedRXingResultType::URI,
    );
    do_test_rxing_result(
        "gopher://google.com/obsolete",
        "gopher://google.com/obsolete",
        ParsedRXingResultType::URI,
    );
}

#[test]
fn test_geo() {
    do_test_rxing_result("geo:1,2", "1, 2", ParsedRXingResultType::GEO);
    do_test_rxing_result("GEO:1,2", "1, 2", ParsedRXingResultType::GEO);
    do_test_rxing_result("geo:1,2,3", "1, 2, 3m", ParsedRXingResultType::GEO);
    do_test_rxing_result(
        "geo:80.33,-32.3344,3.35",
        "80.33, -32.3344, 3.35m",
        ParsedRXingResultType::GEO,
    );
    do_test_rxing_result("geo", "geo", ParsedRXingResultType::TEXT);
    do_test_rxing_result("geography", "geography", ParsedRXingResultType::TEXT);
}

#[test]
fn test_tel() {
    do_test_rxing_result("tel:+15551212", "+15551212", ParsedRXingResultType::TEL);
    do_test_rxing_result("TEL:+15551212", "+15551212", ParsedRXingResultType::TEL);
    do_test_rxing_result(
        "tel:212 555 1212",
        "212 555 1212",
        ParsedRXingResultType::TEL,
    );
    do_test_rxing_result("tel:2125551212", "2125551212", ParsedRXingResultType::TEL);
    do_test_rxing_result(
        "tel:212-555-1212",
        "212-555-1212",
        ParsedRXingResultType::TEL,
    );
    do_test_rxing_result("tel", "tel", ParsedRXingResultType::TEXT);
    do_test_rxing_result("telephone", "telephone", ParsedRXingResultType::TEXT);
}

#[test]
fn test_vcard() {
    do_test_rxing_result(
        "BEGIN:VCARD\r\nEND:VCARD",
        "",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "BEGIN:VCARD\r\nN:Owen;Sean\r\nEND:VCARD",
        "Sean Owen",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "BEGIN:VCARD\r\nVERSION:2.1\r\nN:Owen;Sean\r\nEND:VCARD",
        "Sean Owen",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result(
        "BEGIN:VCARD\r\nADR;HOME:123 Main St\r\nVERSION:2.1\r\nN:Owen;Sean\r\nEND:VCARD",
        "Sean Owen\n123 Main St",
        ParsedRXingResultType::ADDRESSBOOK,
    );
    do_test_rxing_result("BEGIN:VCARD", "", ParsedRXingResultType::ADDRESSBOOK);
}

#[test]
fn test_vevent() {
    // UTC times
    do_test_rxing_result("BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504T123456Z\r\nDTEND:20080505T234555Z\r\nEND:VEVENT\r\nEND:VCALENDAR",
        &format!("foo\n{}\n{}",format_time(2008, 5, 4, 12, 34, 56),format_time(2008, 5, 5, 23, 45, 55)),
        ParsedRXingResultType::CALENDAR);
    do_test_rxing_result("BEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504T123456Z\r\nDTEND:20080505T234555Z\r\nEND:VEVENT", &format!("foo\n{}\n{}" ,
    format_time(2008, 5, 4, 12, 34, 56),format_time(2008, 5, 5, 23, 45, 55)),
        ParsedRXingResultType::CALENDAR);
    // Local times
    do_test_rxing_result("BEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504T123456\r\nDTEND:20080505T234555\r\nEND:VEVENT", &format!("foo\n{}\n{}" ,
    format_time(2008, 5, 4, 12, 34, 56),format_time(2008, 5, 5, 23, 45, 55)),
        ParsedRXingResultType::CALENDAR);
    // Date only (all day event)
    do_test_rxing_result(
        "BEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504\r\nDTEND:20080505\r\nEND:VEVENT",
        &format!(
            "foo\n{}\n{}",
            format_date(2008, 5, 4),
            format_date(2008, 5, 5)
        ),
        ParsedRXingResultType::CALENDAR,
    );
    // Start time only
    do_test_rxing_result(
        "BEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504T123456Z\r\nEND:VEVENT",
        &format!("foo\n{}", format_time(2008, 5, 4, 12, 34, 56)),
        ParsedRXingResultType::CALENDAR,
    );
    do_test_rxing_result(
        "BEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504T123456\r\nEND:VEVENT",
        &format!("foo\n{}", format_time(2008, 5, 4, 12, 34, 56)),
        ParsedRXingResultType::CALENDAR,
    );
    do_test_rxing_result(
        "BEGIN:VEVENT\r\nSUMMARY:foo\r\nDTSTART:20080504\r\nEND:VEVENT",
        &format!("foo\n{}", format_date(2008, 5, 4)),
        ParsedRXingResultType::CALENDAR,
    );
    do_test_rxing_result(
        "BEGIN:VEVENT\r\nDTEND:20080505T\r\nEND:VEVENT",
        "BEGIN:VEVENT\r\nDTEND:20080505T\r\nEND:VEVENT",
        ParsedRXingResultType::TEXT,
    );
    // Yeah, it's OK that this is thought of as maybe a URI as long as it's not CALENDAR
    // Make sure illegal entries without newlines don't crash
    do_test_rxing_result(
        "BEGIN:VEVENTSUMMARY:EventDTSTART:20081030T122030ZDTEND:20081030T132030ZEND:VEVENT",
        "BEGIN:VEVENTSUMMARY:EventDTSTART:20081030T122030ZDTEND:20081030T132030ZEND:VEVENT",
        ParsedRXingResultType::URI,
    );
}

fn format_date(year: i32, month: u32, day: u32) -> String {
    let dtm = Utc.ymd(year, month, day);
    dtm.format("%F").to_string()
    // Calendar cal = Calendar.getInstance();
    // cal.clear();
    // cal.set(year, month - 1, day);
    // return DateFormat.getDateInstance(DateFormat.MEDIUM, Locale.US).format(cal.getTime());
}

fn format_time(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> String {
    let dtm = Utc.ymd(year, month, day).and_hms(hour, min, sec);
    dtm.format("%c").to_string()
    // Calendar cal = Calendar.getInstance();
    // cal.clear();
    // cal.set(year, month - 1, day, hour, min, sec);
    // return DateFormat.getDateTimeInstance(DateFormat.MEDIUM, DateFormat.MEDIUM, Locale.US).format(cal.getTime());
}

#[test]
fn test_sms() {
    do_test_rxing_result("sms:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result("SMS:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result(
        "sms:+15551212;via=999333",
        "+15551212",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "sms:+15551212?subject=foo&body=bar",
        "+15551212\nfoo\nbar",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "sms:+15551212,+12124440101",
        "+15551212\n+12124440101",
        ParsedRXingResultType::SMS,
    );
}

#[test]
fn test_smsto() {
    do_test_rxing_result("SMSTO:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result("smsto:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result(
        "smsto:+15551212:subject",
        "+15551212\nsubject",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "smsto:+15551212:My message",
        "+15551212\nMy message",
        ParsedRXingResultType::SMS,
    );
    // Need to handle question mark in the subject
    do_test_rxing_result(
        "smsto:+15551212:What's up?",
        "+15551212\nWhat's up?",
        ParsedRXingResultType::SMS,
    );
    // Need to handle colon in the subject
    do_test_rxing_result(
        "smsto:+15551212:Directions: Do this",
        "+15551212\nDirections: Do this",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "smsto:212-555-1212:Here's a longer message. Should be fine.",
        "212-555-1212\nHere's a longer message. Should be fine.",
        ParsedRXingResultType::SMS,
    );
}

#[test]
fn test_mms() {
    do_test_rxing_result("mms:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result("MMS:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result(
        "mms:+15551212;via=999333",
        "+15551212",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "mms:+15551212?subject=foo&body=bar",
        "+15551212\nfoo\nbar",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "mms:+15551212,+12124440101",
        "+15551212\n+12124440101",
        ParsedRXingResultType::SMS,
    );
}

#[test]
fn test_mmsto() {
    do_test_rxing_result("MMSTO:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result("mmsto:+15551212", "+15551212", ParsedRXingResultType::SMS);
    do_test_rxing_result(
        "mmsto:+15551212:subject",
        "+15551212\nsubject",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "mmsto:+15551212:My message",
        "+15551212\nMy message",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "mmsto:+15551212:What's up?",
        "+15551212\nWhat's up?",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "mmsto:+15551212:Directions: Do this",
        "+15551212\nDirections: Do this",
        ParsedRXingResultType::SMS,
    );
    do_test_rxing_result(
        "mmsto:212-555-1212:Here's a longer message. Should be fine.",
        "212-555-1212\nHere's a longer message. Should be fine.",
        ParsedRXingResultType::SMS,
    );
}

fn do_test_rxing_result(contents: &str, golden_rxing_result: &str, r_type: ParsedRXingResultType) {
    do_test_rxing_result_long(
        contents,
        golden_rxing_result,
        r_type,
        BarcodeFormat::QR_CODE,
    );
    // QR code is arbitrary
}

fn do_test_rxing_result_long(
    contents: &str,
    golden_rxing_result: &str,
    r_type: ParsedRXingResultType,
    bc_format: BarcodeFormat,
) {
    let fake_rxing_result = RXingResult::new(contents, Vec::new(), Vec::new(), bc_format);
    let result = ResultParser::parseRXingResult(&fake_rxing_result);
    //assertNotNull(result);
    assert_eq!(r_type, result.getType());

    let display_rxing_result = result.getDisplayRXingResult();
    assert_eq!(golden_rxing_result, display_rxing_result);
}
