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

use rxing::{BarcodeFormat, MultiFormatReader};

mod common;

/**
 * @author kevin.osullivan@sita.aero
 */
#[cfg(feature = "image_formats")]
#[test]
fn itfblack_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/itf-1",
        MultiFormatReader::default(),
        BarcodeFormat::ITF,
    );

    //   super("src/test/resources/blackbox/itf-1", new MultiFormatReader(), BarcodeFormat.ITF);
    tester.add_test(14, 14, 0.0);
    tester.add_test(14, 14, 180.0);

    tester.test_black_box();
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn itfblack_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/itf-2",
        MultiFormatReader::default(),
        BarcodeFormat::ITF,
    );

    //   super("src/test/resources/blackbox/itf-2", new MultiFormatReader(), BarcodeFormat.ITF);
    tester.add_test(13, 13, 0.0);
    tester.add_test(13, 13, 180.0);

    tester.test_black_box();
}
