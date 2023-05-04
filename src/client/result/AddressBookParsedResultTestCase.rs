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

/**
 * Tests {@link AddressBookParsedRXingResult}.
 *
 * @author Sean Owen
 */
// public final class AddressBookParsedRXingResultTestCase extends Assert {
use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType, ResultParser},
    BarcodeFormat, RXingResult,
};

#[test]
fn testAddressBookDocomo() {
    doTest(
        "MECARD:N:Sean Owen;;",
        "",
        &["Sean Owen"],
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
    doTest(
        "MECARD:NOTE:ZXing Team;N:Sean Owen;URL:google.com;EMAIL:srowen@example.org;;",
        "",
        &["Sean Owen"],
        "",
        &Vec::new(),
        &["srowen@example.org"],
        &Vec::new(),
        &Vec::new(),
        "",
        &["google.com"],
        "",
        "ZXing Team",
    );
}

#[test]
fn testAddressBookAU() {
    doTest(
        "MEMORY:foo\r\nNAME1:Sean\r\nTEL1:+12125551212\r\n",
        "",
        &["Sean"],
        "",
        &Vec::new(),
        &Vec::new(),
        &["+12125551212"],
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "foo",
    );
}

#[test]
fn testVCard() {
    doTest(
        "BEGIN:VCARD\r\nADR;HOME:123 Main St\r\nVERSION:2.1\r\nN:Owen;Sean\r\nEND:VCARD",
        "",
        &["Sean Owen"],
        "",
        &["123 Main St"],
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testVCardFullN() {
    doTest(
        "BEGIN:VCARD\r\nVERSION:2.1\r\nN:Owen;Sean;T;Mr.;Esq.\r\nEND:VCARD",
        "",
        &["Mr. Sean T Owen Esq."],
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testVCardFullN2() {
    doTest(
        "BEGIN:VCARD\r\nVERSION:2.1\r\nN:Owen;Sean;;;\r\nEND:VCARD",
        "",
        &["Sean Owen"],
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testVCardFullN3() {
    doTest(
        "BEGIN:VCARD\r\nVERSION:2.1\r\nN:;Sean;;;\r\nEND:VCARD",
        "",
        &["Sean"],
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testVCardCaseInsensitive() {
    doTest(
        "begin:vcard\r\nadr;HOME:123 Main St\r\nVersion:2.1\r\nn:Owen;Sean\r\nEND:VCARD",
        "",
        &["Sean Owen"],
        "",
        &["123 Main St"],
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testEscapedVCard() {
    doTest("BEGIN:VCARD\r\nADR;HOME:123\\;\\\\ Main\\, St\\nHome\r\nVERSION:2.1\r\nN:Owen;Sean\r\nEND:VCARD",
           "", &["Sean Owen"], "", &["123;\\ Main, St\nHome"],
           &Vec::new(), &Vec::new(), &Vec::new(), "", &Vec::new(), "", "");
}

#[test]
fn testBizcard() {
    doTest(
        "BIZCARD:N:Sean;X:Owen;C:Google;A:123 Main St;M:+12125551212;E:srowen@example.org;",
        "",
        &["Sean Owen"],
        "",
        &["123 Main St"],
        &["srowen@example.org"],
        &["+12125551212"],
        &Vec::new(),
        "Google",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testSeveralAddresses() {
    doTest("MECARD:N:Foo Bar;ORG:Company;TEL:5555555555;EMAIL:foo.bar@xyz.com;ADR:City, 10001;ADR:City, 10001;NOTE:This is the memo.;;",
           "", &["Foo Bar"], "", &["City, 10001", "City, 10001"],
           &["foo.bar@xyz.com"],
           &["5555555555"], &Vec::new(), "Company", &Vec::new(), "", "This is the memo.");
}

#[test]
fn testQuotedPrintable() {
    doTest(
        "BEGIN:VCARD\r\nADR;HOME;CHARSET=UTF-8;ENCODING=QUOTED-PRINTABLE:;;=38=38=20=4C=79=6E=62=72=6F=6F=6B=0D=0A=43=\r\n=4F=20=36=39=39=\r\n=39=39;;;\r\nEND:VCARD",
        "",
        &Vec::new(),
        "",
        &["88 Lynbrook\r\nCO 69999"],
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testVCardEscape() {
    doTest(
        "BEGIN:VCARD\r\nNOTE:foo\\nbar\r\nEND:VCARD",
        "",
        &Vec::new(),
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "foo\nbar",
    );
    doTest(
        "BEGIN:VCARD\r\nNOTE:foo\\;bar\r\nEND:VCARD",
        "",
        &Vec::new(),
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "foo;bar",
    );
    doTest(
        "BEGIN:VCARD\r\nNOTE:foo\\\\bar\r\nEND:VCARD",
        "",
        &Vec::new(),
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "foo\\bar",
    );
    doTest(
        "BEGIN:VCARD\r\nNOTE:foo\\,bar\r\nEND:VCARD",
        "",
        &Vec::new(),
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "foo,bar",
    );
}

#[test]
fn testVCardValueURI() {
    doTest(
        "BEGIN:VCARD\r\nTEL;VALUE=uri:tel:+1-555-555-1212\r\nEND:VCARD",
        "",
        &Vec::new(),
        "",
        &Vec::new(),
        &Vec::new(),
        &["+1-555-555-1212"],
        &[""],
        "",
        &Vec::new(),
        "",
        "",
    );

    doTest(
        "BEGIN:VCARD\r\nN;VALUE=text:Owen;Sean\r\nEND:VCARD",
        "",
        &["Sean Owen"],
        "",
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        &Vec::new(),
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[test]
fn testVCardTypes() {
    doTest(
        "BEGIN:VCARD\r\nTEL;HOME:\r\nTEL;WORK:10\r\nTEL:20\r\nTEL;CELL:30\r\nEND:VCARD",
        "",
        &Vec::new(),
        "",
        &Vec::new(),
        &Vec::new(),
        &["10", "20", "30"],
        &["WORK", "", "CELL"],
        "",
        &Vec::new(),
        "",
        "",
    );
}

#[allow(clippy::too_many_arguments)]
fn doTest(
    contents: &str,
    title: &str,
    names: &[&str],
    pronunciation: &str,
    addresses: &[&str],
    emails: &[&str],
    phoneNumbers: &[&str],
    phoneTypes: &[&str],
    org: &str,
    urls: &[&str],
    birthday: &str,
    note: &str,
) {
    let fakeRXingResult =
        RXingResult::new(contents, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::ADDRESSBOOK, result.getType());
    if let ParsedClientResult::AddressBookResult(addressRXingResult) = result {
        assert_eq!(title, addressRXingResult.getTitle());
        assert_eq!(names, addressRXingResult.getNames());
        assert_eq!(pronunciation, addressRXingResult.getPronunciation());
        assert_eq!(addresses, addressRXingResult.getAddresses());
        assert_eq!(emails, addressRXingResult.getEmails());
        assert_eq!(phoneNumbers, addressRXingResult.getPhoneNumbers());
        assert_eq!(phoneTypes, addressRXingResult.getPhoneTypes());
        assert_eq!(org, addressRXingResult.getOrg());
        assert_eq!(urls, addressRXingResult.getURLs());
        assert_eq!(birthday, addressRXingResult.getBirthday());
        assert_eq!(note, addressRXingResult.getNote());
    } else {
        panic!("Expected address book result");
    }
}

// }
