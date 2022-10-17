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
 * Tests {@link ProductParsedRXingResult}.
 *
 * @author Sean Owen
 */
// public final class ProductParsedRXingResultTestCase extends Assert {
use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType},
    BarcodeFormat, RXingResult,
};

use super::ResultParser;

#[test]
fn test_product() {
    do_test("123456789012", "123456789012", BarcodeFormat::UPC_A);
    do_test("00393157", "00393157", BarcodeFormat::EAN_8);
    do_test("5051140178499", "5051140178499", BarcodeFormat::EAN_13);
    do_test("01234565", "012345000065", BarcodeFormat::UPC_E);
}

fn do_test(contents: &str, normalized: &str, format: BarcodeFormat) {
    let fake_rxing_result = RXingResult::new(contents, Vec::new(), Vec::new(), format);
    let result = ResultParser::parseRXingResult(&fake_rxing_result);
    assert_eq!(ParsedRXingResultType::PRODUCT, result.getType());

    if let ParsedClientResult::ProductResult(product_rxing_result) = result {
        assert_eq!(contents, product_rxing_result.getProductID());
        assert_eq!(normalized, product_rxing_result.getNormalizedProductID());
    } else {
        panic!("Expected ParsedClientResult::ProductResult")
    }
}

// }
