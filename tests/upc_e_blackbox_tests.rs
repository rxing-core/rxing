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

use rxing::{
    BarcodeFormat, MultiFormatReader,
};

mod common;

/**
 * @author Sean Owen
 */
#[test]
fn upceblack_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upce-1",
        MultiFormatReader::default(),
        // UPCEReader::default(),
        BarcodeFormat::UPC_E,
    );

    //  super("src/test/resources/blackbox/upce-1", new MultiFormatReader(), BarcodeFormat.UPC_E);
    tester.add_test(3, 3, 0.0);
    tester.add_test(3, 3, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[test]
fn upceblack_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upce-2",
        MultiFormatReader::default(),
        // UPCEReader::default(),
        BarcodeFormat::UPC_E,
    );

    //   super("src/test/resources/blackbox/upce-2", new MultiFormatReader(), BarcodeFormat.UPC_E);
    tester.add_test_complex(31, 35, 0, 1, 0.0);
    tester.add_test_complex(31, 35, 1, 1, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[test]
fn upceblack_box3_reflective_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upce-3",
        MultiFormatReader::default(),
        // UPCEReader::default(),
        BarcodeFormat::UPC_E,
    );

    //   super("src/test/resources/blackbox/upce-3", new MultiFormatReader(), BarcodeFormat.UPC_E);
    tester.add_test(6, 8, 0.0);
    tester.add_test(6, 8, 180.0);

    tester.test_black_box()
}
