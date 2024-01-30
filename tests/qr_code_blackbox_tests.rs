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

use rxing::{qrcode::QRCodeReader, BarcodeFormat, MultiFormatReader};

mod common;

/**
 * @author Sean Owen
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-1",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::QR_CODE,
    );
    // super("src/test/resources/blackbox/qrcode-1", new MultiFormatReader(), BarcodeFormat.QR_CODE);
    tester.add_test(17, 17, 0.0);
    tester.add_test(14, 14, 90.0);
    tester.add_test(17, 17, 180.0);
    tester.add_test(14, 14, 270.0);

    tester.test_black_box();
}

/**
 * @author Sean Owen
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-2",
        // MultiFormatReader::default(),
        QRCodeReader {},
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(31, 31, 0.0);
    tester.add_test(29, 29, 90.0);
    tester.add_test(30, 30, 180.0);
    tester.add_test(30, 30, 270.0);

    tester.test_black_box();
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-3",
        MultiFormatReader::default(),
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(38, 38, 0.0);
    tester.add_test(39, 39, 90.0);
    tester.add_test(36, 36, 180.0);
    tester.add_test(39, 39, 270.0);

    tester.test_black_box();
}

/**
 * Tests of various QR Codes from t-shirts, which are notoriously not flat.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box4_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-4",
        MultiFormatReader::default(),
        // QRCodeReader::new(),
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(36, 36, 0.0);
    tester.add_test(35, 35, 90.0);
    tester.add_test(35, 35, 180.0);
    tester.add_test(35, 35, 270.0);

    tester.test_black_box();
}

/**
 * Some very difficult exposure conditions including self-shadowing, which happens a lot when
 * pointing down at a barcode (i.e. the phone's shadow falls across part of the image).
 * The global histogram gets about 5/15, where the local one gets 15/15.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box5_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-5",
        MultiFormatReader::default(),
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(19, 19, 0.0);
    tester.add_test(19, 19, 90.0);
    tester.add_test(19, 19, 180.0);
    tester.add_test(19, 19, 270.0);

    tester.test_black_box();
}

/**
 * These tests are supplied by Tim Gernat and test finder pattern detection at small size and under
 * rotation, which was a weak spot.
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box6_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-6",
        MultiFormatReader::default(),
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(15, 15, 0.0);
    tester.add_test(14, 14, 90.0);
    tester.add_test(13, 13, 180.0);
    tester.add_test(14, 14, 270.0);

    tester.test_black_box();
}
