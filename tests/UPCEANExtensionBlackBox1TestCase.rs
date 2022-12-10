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

use rxing::{oned::EAN13Reader, BarcodeFormat, MultiFormatReader};

mod common;

/**
 * @author Sean Owen
 */
#[test]
fn upceanextension_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upcean-extension-1",
        // MultiFormatReader::default(),
        EAN13Reader {},
        BarcodeFormat::EAN_13,
    );
    // super("src/test/resources/blackbox/upcean-extension-1", new MultiFormatReader(), BarcodeFormat.EAN_13);
    tester.add_test(2, 2, 0.0);

    tester.test_black_box()
}
