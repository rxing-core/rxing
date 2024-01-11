/*
 * Copyright 2016 ZXing authors
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
#![cfg(feature = "image")]

use rxing::{BarcodeFormat, DecodeHintType, MultiFormatReader};

mod common;

/**
 * Tests {@link MaxiCodeReader} against a fixed set of test images.
 */
#[test]
fn maxicode1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/maxicode-1",
        MultiFormatReader::default(),
        BarcodeFormat::MAXICODE,
    );
    // super("src/test/resources/blackbox/maxicode-1", new MultiFormatReader(), BarcodeFormat.MAXICODE);
    tester.add_test(7, 8, 0.0);
    tester.add_test(0, 4, 90.0);
    tester.add_test(0, 3, 180.0);

    tester.test_black_box();
}

/**
 * Tests all characters in Set A.
 *
 * @author Daniel Gredler
 * @see <a href="https://github.com/zxing/zxing/issues/1543">Defect 1543</a>
 */
#[test]
fn maxi_code_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/maxicode-1",
        MultiFormatReader::default(),
        BarcodeFormat::MAXICODE,
    );
    // super("src/test/resources/blackbox/maxicode-1", new MultiFormatReader(), BarcodeFormat.MAXICODE);
    tester.add_hint(
        DecodeHintType::PURE_BARCODE,
        rxing::DecodeHintValue::PureBarcode(true),
    );

    tester.add_test(7, 7, 0.0);

    tester.test_black_box();
}
