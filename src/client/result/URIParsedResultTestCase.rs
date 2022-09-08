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
 * Tests {@link URIParsedRXingResult}.
 *
 * @author Sean Owen
 */
// public final class URIParsedRXingResultTestCase extends Assert {
use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType, ResultParser},
    BarcodeFormat, RXingResult,
};

#[test]
fn test_bookmark_docomo() {
    do_test("MEBKM:URL:google.com;;", "http://google.com", "");
    do_test("MEBKM:URL:http://google.com;;", "http://google.com", "");
    do_test(
        "MEBKM:URL:google.com;TITLE:Google;",
        "http://google.com",
        "Google",
    );
}

#[test]
fn test_uri() {
    do_test("google.com", "http://google.com", "");
    do_test("123.com", "http://123.com", "");
    do_test("http://google.com", "http://google.com", "");
    do_test("https://google.com", "https://google.com", "");
    do_test("google.com:443", "http://google.com:443", "");
    do_test("https://www.google.com/calendar/hosted/google.com/embed?mode=AGENDA&force_login=true&src=google.com_726f6f6d5f6265707075@resource.calendar.google.com",
           "https://www.google.com/calendar/hosted/google.com/embed?mode=AGENDA&force_login=true&src=google.com_726f6f6d5f6265707075@resource.calendar.google.com",
           "");
    do_test(
        "otpauth://remoteaccess?devaddr=00%a1b2%c3d4&devname=foo&key=bar",
        "otpauth://remoteaccess?devaddr=00%a1b2%c3d4&devname=foo&key=bar",
        "",
    );
    do_test("s3://amazon.com:8123", "s3://amazon.com:8123", "");
    do_test(
        "HTTP://R.BEETAGG.COM/?12345",
        "HTTP://R.BEETAGG.COM/?12345",
        "",
    );
}

#[test]
fn test_not_uri() {
    do_test_not_uri("google.c");
    do_test_not_uri(".com");
    do_test_not_uri(":80/");
    do_test_not_uri("ABC,20.3,AB,AD");
    do_test_not_uri("http://google.com?q=foo bar");
    do_test_not_uri("12756.501");
    do_test_not_uri("google.50");
    do_test_not_uri("foo.bar.bing.baz.foo.bar.bing.baz");
}

#[test]
fn test_urlto() {
    do_test("urlto::bar.com", "http://bar.com", "");
    do_test("urlto::http://bar.com", "http://bar.com", "");
    do_test("urlto:foo:bar.com", "http://bar.com", "foo");
}

#[test]
fn test_garbage() {
    do_test_not_uri("Da65cV1g^>%^f0bAbPn1CJB6lV7ZY8hs0Sm:DXU0cd]GyEeWBz8]bUHLB");
    // used \u{0008} as java \b
    do_test_not_uri("DEA\u{0003}\u{0019}M\u{0006}\u{0000}\u{0008}√•\u{0000}¬áHO\u{0000}X$\u{0001}\u{0000}\u{001F}wfc\u{0007}!√æ¬ì¬ò\
                 \u{0013}\u{0013}¬æZ{√π√é√ù√ö¬óZ¬ß¬®+y_zb√±k\u{0011}7¬∏\u{000E}¬Ü√ú\u{0000}\u{0000}\u{0000}\u{0000}\
                 \u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\
                 \u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}\u{0000}¬£.ux");
}

#[test]
fn test_is_possibly_malicious() {
    do_test_is_possibly_malicious("http://google.com", false);
    do_test_is_possibly_malicious("http://google.com@evil.com", true);
    do_test_is_possibly_malicious("http://google.com:@evil.com", true);
    do_test_is_possibly_malicious("google.com:@evil.com", false);
    do_test_is_possibly_malicious("https://google.com:443", false);
    do_test_is_possibly_malicious("https://google.com:443/", false);
    do_test_is_possibly_malicious("https://evil@google.com:443", true);
    do_test_is_possibly_malicious("http://google.com/foo@bar", false);
    do_test_is_possibly_malicious("http://google.com/@@", false);
}

#[test]
fn test_malicious_unicode() {
    do_test_is_possibly_malicious("https://google.com\u{2215}.evil.com/stuff", true);
    do_test_is_possibly_malicious("\u{202e}https://dylankatz.com/moc.elgoog.www//:sptth", true);
}

#[test]
fn test_exotic() {
    do_test(
        "bitcoin:mySD89iqpmptrK3PhHFW9fa7BXiP7ANy3Y",
        "bitcoin:mySD89iqpmptrK3PhHFW9fa7BXiP7ANy3Y",
        "",
    );
    do_test("BTCTX:-TC4TO3$ZYZTC5NC83/SYOV+YGUGK:$BSF0P8/STNTKTKS.V84+JSA$LB+EHCG+8A725.2AZ-NAVX3VBV5K4MH7UL2.2M:F*M9HSL*$2P7T*FX.ZT80GWDRV0QZBPQ+O37WDCNZBRM3EQ0S9SZP+3BPYZG02U/LA*89C2U.V1TS.CT1VF3DIN*HN3W-O-0ZAKOAB32/.8:J501GJJTTWOA+5/6$MIYBERPZ41NJ6-WSG/*Z48ZH*LSAOEM*IXP81L:$F*W08Z60CR*C*P.JEEVI1F02J07L6+W4L1G$/IC*$16GK6A+:I1-:LJ:Z-P3NW6Z6ADFB-F2AKE$2DWN23GYCYEWX9S8L+LF$VXEKH7/R48E32PU+A:9H:8O5",
           "BTCTX:-TC4TO3$ZYZTC5NC83/SYOV+YGUGK:$BSF0P8/STNTKTKS.V84+JSA$LB+EHCG+8A725.2AZ-NAVX3VBV5K4MH7UL2.2M:F*M9HSL*$2P7T*FX.ZT80GWDRV0QZBPQ+O37WDCNZBRM3EQ0S9SZP+3BPYZG02U/LA*89C2U.V1TS.CT1VF3DIN*HN3W-O-0ZAKOAB32/.8:J501GJJTTWOA+5/6$MIYBERPZ41NJ6-WSG/*Z48ZH*LSAOEM*IXP81L:$F*W08Z60CR*C*P.JEEVI1F02J07L6+W4L1G$/IC*$16GK6A+:I1-:LJ:Z-P3NW6Z6ADFB-F2AKE$2DWN23GYCYEWX9S8L+LF$VXEKH7/R48E32PU+A:9H:8O5",
               "");
    do_test(
        "opc.tcp://test.samplehost.com:4841",
        "opc.tcp://test.samplehost.com:4841",
        "",
    );
}

fn do_test(contents: &str, uri: &str, title: &str) {
    let fake_rxing_result =
        RXingResult::new(contents, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fake_rxing_result);
    assert_eq!(ParsedRXingResultType::URI, result.getType());
    if let ParsedClientResult::URIResult(uriRXingResult) = result {
        assert_eq!(uri, uriRXingResult.getURI());
        assert_eq!(title, uriRXingResult.getTitle());
    } else {
        panic!("Expected ParsedClientResult::URIResult(uriRXingResult)");
    }
}

fn do_test_not_uri(text: &str) {
    let fake_rxing_result = RXingResult::new(text, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fake_rxing_result);
    assert_eq!(ParsedRXingResultType::TEXT, result.getType());
    assert_eq!(text, result.getDisplayRXingResult());
}

fn do_test_is_possibly_malicious(uri: &str, malicious: bool) {
    let fake_rxing_result = RXingResult::new(uri, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fake_rxing_result);
    assert_eq!(
        if malicious {
            ParsedRXingResultType::TEXT
        } else {
            ParsedRXingResultType::URI
        },
        result.getType()
    );
}

// }
