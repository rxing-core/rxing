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

use rxing::{aztec::AztecReader, BarcodeFormat};

mod common;

/**
 * @author David Olivier
 */
#[test]
fn aztec_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/aztec-1",
        AztecReader {},
        BarcodeFormat::AZTEC,
    );

    // super("src/test/resources/blackbox/aztec-1", AztecReader::new(), BarcodeFormat::AZTEC);
    tester.addTest(14, 14, 0.0);
    tester.addTest(14, 14, 90.0);
    tester.addTest(14, 14, 180.0);
    tester.addTest(14, 14, 270.0);

    tester.testBlackBox();
}

/**
 * A test of Aztec barcodes under real world lighting conditions, taken with a mobile phone.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[test]
fn aztec_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/aztec-2",
        AztecReader {},
        BarcodeFormat::AZTEC,
    );
    // super(, new AztecReader(), BarcodeFormat.AZTEC);
    tester.addTest(5, 5, 0.0);
    tester.addTest(4, 4, 90.0);
    tester.addTest(6, 6, 180.0);
    tester.addTest(3, 3, 270.0);

    tester.testBlackBox();
}
