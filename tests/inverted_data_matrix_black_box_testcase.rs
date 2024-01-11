/*
 * Copyright 2008 ZXing authors
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

use rxing::{DecodeHintType, DecodeHintValue, MultiFormatReader};

mod common;

/**
 * Inverted barcodes
 */
#[cfg(feature = "image-formats")]
#[test]
fn inverted_data_matrix_black_box_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/inverted",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::DATA_MATRIX,
    );
    // super("src/test/resources/blackbox/inverted", new MultiFormatReader(), BarcodeFormat.DATA_MATRIX);
    tester.add_hint(
        DecodeHintType::ALSO_INVERTED,
        DecodeHintValue::AlsoInverted(true),
    );
    tester.add_test(1, 1, 0.0);
    tester.add_test(1, 1, 90.0);
    tester.add_test(1, 1, 180.0);
    tester.add_test(1, 1, 270.0);

    tester.test_black_box();
}
