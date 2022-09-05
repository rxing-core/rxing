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
 * Tests {@link SMSParsedRXingResult}.
 *
 * @author Sean Owen
 */
// public final class SMSMMSParsedRXingResultTestCase extends Assert {
use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType, ResultParser},
    BarcodeFormat, RXingResult,
};

#[test]
fn test_sms() {
    do_test("sms:+15551212", "+15551212", "", "", "", "sms:+15551212");
    do_test(
        "sms:+15551212?subject=foo&body=bar",
        "+15551212",
        "foo",
        "bar",
        "",
        "sms:+15551212?body=bar&subject=foo",
    );
    do_test(
        "sms:+15551212;via=999333",
        "+15551212",
        "",
        "",
        "999333",
        "sms:+15551212;via=999333",
    );
}

#[test]
fn test_mms() {
    do_test("mms:+15551212", "+15551212", "", "", "", "sms:+15551212");
    do_test(
        "mms:+15551212?subject=foo&body=bar",
        "+15551212",
        "foo",
        "bar",
        "",
        "sms:+15551212?body=bar&subject=foo",
    );
    do_test(
        "mms:+15551212;via=999333",
        "+15551212",
        "",
        "",
        "999333",
        "sms:+15551212;via=999333",
    );
}

fn do_test(contents: &str, number: &str, subject: &str, body: &str, via: &str, parsedURI: &str) {
    let fake_rxing_result =
        RXingResult::new(contents, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fake_rxing_result);
    assert_eq!(ParsedRXingResultType::SMS, result.getType());

    if let ParsedClientResult::SMSResult(smsRXingResult) = result {
        assert_eq!(&vec![number], smsRXingResult.getNumbers());
        assert_eq!(subject, smsRXingResult.getSubject());
        assert_eq!(body, smsRXingResult.getBody());
        assert_eq!(&vec![via], smsRXingResult.getVias());
        assert_eq!(parsedURI, smsRXingResult.getSMSURI());
    } else {
        panic!("Expected ParsedClientResult::SMSResult");
    }
}

// }
