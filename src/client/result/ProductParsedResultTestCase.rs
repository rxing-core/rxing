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

use crate::{BarcodeFormat, RXingResult, client::result::{ParsedRXingResult, ParsedRXingResultType, ProductParsedResult, ParsedClientResult}};

use super::ResultParser;

  #[test]
  fn testProduct() {
    doTest("123456789012", "123456789012", BarcodeFormat::UPC_A);
    doTest("00393157", "00393157", BarcodeFormat::EAN_8);
    doTest("5051140178499", "5051140178499", BarcodeFormat::EAN_13);
    doTest("01234565", "012345000065", BarcodeFormat::UPC_E);
  }

   fn doTest( contents:&str,  normalized:&str,  format:BarcodeFormat) {
    let fakeRXingResult =  RXingResult::new(contents, Vec::new(), Vec::new(), format);
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::PRODUCT, result.getType());

    if let ParsedClientResult::ProductResult(productRXingResult) = result {
      assert_eq!(contents, productRXingResult.getProductID());
      assert_eq!(normalized, productRXingResult.getNormalizedProductID());
    }else{
      panic!("Expected ParsedClientResult::ProductResult")
    }

  }

// }