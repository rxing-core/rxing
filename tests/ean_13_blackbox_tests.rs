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
fn ean13_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/ean13-1",
        MultiFormatReader::default(),
        // EAN13Reader {},
        BarcodeFormat::EAN_13,
    );

    //  super("src/test/resources/blackbox/ean13-1", new MultiFormatReader(), BarcodeFormat.EAN_13);
    tester.add_test(30, 32, 0.0);
    tester.add_test(27, 32, 180.0);

    tester.test_black_box()
}

/**
 * This is a set of mobile image taken at 480x360 with difficult lighting.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn ean13_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/ean13-2",
        MultiFormatReader::default(),
        // EAN13Reader {},
        BarcodeFormat::EAN_13,
    );

    //   super("src/test/resources/blackbox/ean13-2", new MultiFormatReader(), BarcodeFormat.EAN_13);
    tester.add_test_complex(12, 17, 0, 1, 0.0);
    tester.add_test_complex(11, 17, 0, 1, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn ean13_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/ean13-3",
        MultiFormatReader::default(),
        // EAN13Reader {},
        BarcodeFormat::EAN_13,
    );

    //   super("src/test/resources/blackbox/ean13-3", new MultiFormatReader(), BarcodeFormat.EAN_13);
    tester.add_test(53, 55, 0.0);
    tester.add_test(55, 55, 180.0);

    tester.test_black_box()
}

/**
 * A very difficult set of images taken with extreme shadows and highlights.
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn ean13_black_box4_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/ean13-4",
        MultiFormatReader::default(),
        // EAN13Reader {},
        BarcodeFormat::EAN_13,
    );
    //   super("src/test/resources/blackbox/ean13-4", new MultiFormatReader(), BarcodeFormat.EAN_13);
    tester.add_test_complex(6, 13, 1, 1, 0.0);
    tester.add_test_complex(7, 13, 1, 1, 180.0);

    tester.test_black_box()
}

/**
 * A set of blurry images taken with a fixed-focus device.
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn ean13_black_box5_blurry_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/ean13-5",
        MultiFormatReader::default(),
        // EAN13Reader {},
        BarcodeFormat::EAN_13,
    );
    //   super("src/test/resources/blackbox/ean13-5", new MultiFormatReader(), BarcodeFormat.EAN_13);
    tester.add_test(0, 0, 0.0);
    tester.add_test(0, 0, 180.0);

    tester.test_black_box()
}
