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
// import org.junit.Test;

use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType, ResultParser},
    BarcodeFormat, RXingResult,
};

/**
 * Tests {@link EmailAddressParsedRXingResult}.
 *
 * @author Sean Owen
 */

#[test]
fn testEmailAddress() {
    do_test_single("srowen@example.org", "srowen@example.org", "", "");
    do_test_single("mailto:srowen@example.org", "srowen@example.org", "", "");
}

#[test]
fn testTos() {
    do_test(
        "mailto:srowen@example.org,bob@example.org",
        &vec!["srowen@example.org", "bob@example.org"],
        &Vec::new(),
        &Vec::new(),
        "",
        "",
    );
    do_test(
        "mailto:?to=srowen@example.org,bob@example.org",
        &vec!["srowen@example.org", "bob@example.org"],
        &Vec::new(),
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testCCs() {
    do_test(
        "mailto:?cc=srowen@example.org",
        &Vec::new(),
        &vec!["srowen@example.org"],
        &Vec::new(),
        "",
        "",
    );
    do_test(
        "mailto:?cc=srowen@example.org,bob@example.org",
        &Vec::new(),
        &vec!["srowen@example.org", "bob@example.org"],
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testBCCs() {
    do_test(
        "mailto:?bcc=srowen@example.org",
        &Vec::new(),
        &Vec::new(),
        &vec!["srowen@example.org"],
        "",
        "",
    );
    do_test(
        "mailto:?bcc=srowen@example.org,bob@example.org",
        &Vec::new(),
        &Vec::new(),
        &vec!["srowen@example.org", "bob@example.org"],
        "",
        "",
    );
}

#[test]
fn testAll() {
    do_test(
        "mailto:bob@example.org?cc=foo@example.org&bcc=srowen@example.org&subject=baz&body=buzz",
        &vec!["bob@example.org"],
        &vec!["foo@example.org"],
        &vec!["srowen@example.org"],
        "baz",
        "buzz",
    );
}

#[test]
fn testEmailDocomo() {
    do_test_single(
        "MATMSG:TO:srowen@example.org;;",
        "srowen@example.org",
        "",
        "",
    );
    do_test_single(
        "MATMSG:TO:srowen@example.org;SUB:Stuff;;",
        "srowen@example.org",
        "Stuff",
        "",
    );
    do_test_single(
        "MATMSG:TO:srowen@example.org;SUB:Stuff;BODY:This is some text;;",
        "srowen@example.org",
        "Stuff",
        "This is some text",
    );
}

#[test]
fn testSMTP() {
    do_test_single("smtp:srowen@example.org", "srowen@example.org", "", "");
    do_test_single("SMTP:srowen@example.org", "srowen@example.org", "", "");
    do_test_single(
        "smtp:srowen@example.org:foo",
        "srowen@example.org",
        "foo",
        "",
    );
    do_test_single(
        "smtp:srowen@example.org:foo:bar",
        "srowen@example.org",
        "foo",
        "bar",
    );
}

fn do_test_single(contents: &str, to: &str, subject: &str, body: &str) {
    do_test(contents, &vec![to], &Vec::new(), &Vec::new(), subject, body);
}

fn do_test(contents: &str, tos: &[&str], ccs: &[&str], bccs: &[&str], subject: &str, body: &str) {
    let fakeRXingResult =
        RXingResult::new(contents, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::EMAIL_ADDRESS, result.getType());
    if let ParsedClientResult::EmailResult(emailRXingResult) = result {
        assert_eq!(tos, emailRXingResult.getTos());
        assert_eq!(ccs, emailRXingResult.getCCs());
        assert_eq!(bccs, emailRXingResult.getBCCs());
        assert_eq!(subject, emailRXingResult.getSubject());
        assert_eq!(body, emailRXingResult.getBody());
    } else {
        panic!("Expected EmailResult");
    }
}
