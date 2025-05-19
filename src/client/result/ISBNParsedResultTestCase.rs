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
 * Tests {@link ISBNParsedRXingResult}.
 *
 * @author Sean Owen
 */
// public final class ISBNParsedRXingResultTestCase extends Assert {
use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType},
    BarcodeFormat, RXingResult,
};

use super::ResultParser;

#[test]
fn testISBN() {
    doTest("9784567890123");
}

fn doTest(contents: &str) {
    let fakeRXingResult = RXingResult::new(contents, vec![0; 0], vec![], BarcodeFormat::EAN_13);
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::Isbn, result.getType());
    if let ParsedClientResult::ISBNResult(res) = result {
        assert_eq!(contents, res.getISBN());
    } else {
        panic!("expected ISBNResult")
    }
    // ISBNParsedRXingResult isbnRXingResult = (ISBNParsedRXingResult) result;
}

// }
