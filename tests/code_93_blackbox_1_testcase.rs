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
 * @author Sean Owen
 */
#[cfg(feature = "image_formats")]
#[test]
fn code93_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/code93-1",
        MultiFormatReader::default(),
        BarcodeFormat::CODE_93,
    );
    // super("src/test/resources/blackbox/code93-1", new MultiFormatReader(), BarcodeFormat.CODE_93);
    tester.add_test(3, 3, 0.0);
    tester.add_test(3, 3, 180.0);

    tester.test_black_box()
}
