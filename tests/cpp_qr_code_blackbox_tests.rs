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

use rxing::{
    qrcode::cpp_port::QrReader, BarcodeFormat, MultiFormatReader, MultiUseMultiFormatReader,
};

mod common;

/**
 * @author Sean Owen
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-1",
        QrReader,
        rxing::BarcodeFormat::QR_CODE,
    );
    // super("src/test/resources/blackbox/qrcode-1", new MultiFormatReader(), BarcodeFormat.QR_CODE);
    tester.add_test(17, 17, 0.0);
    tester.add_test(14, 14, 90.0);
    tester.add_test(17, 17, 180.0);
    tester.add_test(16, 16, 270.0);

    tester.test_black_box();
}

/**
 * @author Sean Owen
 */

// #[test]
// fn qrcode_black_box2_test_case() {
//     let mut tester = common::AbstractBlackBoxTestCase::new(
//         "test_resources/blackbox/qrcode-2",
//         // MultiFormatReader::default(),
//         QrReader {},
//         BarcodeFormat::QR_CODE,
//     );
//     tester.add_test(31, 31, 0.0);
//     tester.add_test(29, 29, 90.0);
//     tester.add_test(30, 30, 180.0);
//     tester.add_test(30, 30, 270.0);

//     tester.ignore_pure = false;

//     tester.test_black_box();
// }

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[cfg(feature = "image_formats")]
#[test]
fn qrcode_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-3",
        QrReader,
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
        QrReader,
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
        QrReader,
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(16, 16, 0.0);
    tester.add_test(16, 16, 90.0);
    tester.add_test(16, 16, 180.0);
    tester.add_test(16, 16, 270.0);

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
        QrReader,
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(15, 15, 0.0);
    tester.add_test(14, 14, 90.0);
    tester.add_test(13, 13, 180.0);
    tester.add_test(14, 14, 270.0);

    tester.test_black_box();
}

#[cfg(feature = "image_formats")]
#[test]
fn mqr_black_box_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/microqrcode-1",
        QrReader,
        BarcodeFormat::MICRO_QR_CODE,
    );

    tester.add_test(15, 15, 0.0);
    tester.add_test(15, 15, 90.0);
    tester.add_test(15, 13, 180.0);
    tester.add_test(15, 15, 270.0);

    tester.test_black_box();
}

/**
 * @author Sean Owen
 *
 * 16.png seems to be an odd case where in both c++ and rs the result of a pure read
 * is different than the result of a detect. there is an extra ' ' in the output of
 * the pure read. I'm not sure how to fix this, as it occurs in both and seems to just
 * be a function of the barcode.
 */

#[cfg(feature = "image_formats")]
#[test]
fn cpp_qrcode_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/qrcode-1",
        QrReader,
        rxing::BarcodeFormat::QR_CODE,
    );
    // super("src/test/resources/blackbox/qrcode-1", new MultiFormatReader(), BarcodeFormat.QR_CODE);
    tester.add_test(16, 16, 0.0);
    tester.add_test(16, 16, 90.0);
    tester.add_test(16, 16, 180.0);
    tester.add_test(16, 16, 270.0);

    tester.test_black_box();
}

/**
 * @author Sean Owen
 */

#[cfg(feature = "image_formats")]
#[test]
fn cpp_qrcode_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/qrcode-2",
        MultiUseMultiFormatReader::default(),
        // QrReader::default(),
        BarcodeFormat::QR_CODE,
    );

    tester.add_test(46, 48, 0.0);
    tester.add_test(46, 48, 90.0);
    tester.add_test(46, 48, 180.0);
    tester.add_test(46, 48, 270.0);

    tester.ignore_pure = true;

    tester.add_hint(
        rxing::DecodeHintType::ALSO_INVERTED,
        rxing::DecodeHintValue::AlsoInverted(true),
    );

    tester.test_black_box();
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[cfg(feature = "image_formats")]
#[test]
fn cpp_qrcode_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/qrcode-3",
        QrReader,
        BarcodeFormat::QR_CODE,
    );

    tester.add_test(28, 28, 0.0);
    tester.add_test(28, 28, 90.0);
    tester.add_test(28, 28, 180.0);
    tester.add_test(27, 27, 270.0);

    tester.test_black_box();
}

/**
 * Tests of various QR Codes from t-shirts, which are notoriously not flat.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[cfg(feature = "image_formats")]
#[test]
fn cpp_qrcode_black_box4_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/qrcode-4",
        QrReader,
        // QRCodeReader::new(),
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(29, 29, 0.0);
    tester.add_test(29, 29, 90.0);
    tester.add_test(29, 29, 180.0);
    tester.add_test(29, 29, 270.0);

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
fn cpp_qrcode_black_box5_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/qrcode-5",
        QrReader,
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(16, 16, 0.0);
    tester.add_test(16, 16, 90.0);
    tester.add_test(16, 16, 180.0);
    tester.add_test(16, 16, 270.0);

    tester.test_black_box();
}

/**
 * These tests are supplied by Tim Gernat and test finder pattern detection at small size and under
 * rotation, which was a weak spot.
 */

#[cfg(feature = "image_formats")]
#[test]
fn cpp_qrcode_black_box6_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/qrcode-6",
        QrReader,
        BarcodeFormat::QR_CODE,
    );
    tester.add_test(15, 15, 0.0);
    tester.add_test(15, 15, 90.0);
    tester.add_test(15, 15, 180.0);
    tester.add_test(15, 15, 270.0);

    tester.test_black_box();
}

#[cfg(feature = "image_formats")]
#[test]
fn cpp_qrcode_black_box7_test_case() {
    let mut tester = common::MultiImageSpanAbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/qrcode-7",
        QrReader,
        rxing::BarcodeFormat::QR_CODE,
    );

    // super("src/test/resources/blackbox/pdf417-4", null, BarcodeFormat.PDF_417);
    tester.add_test_complex(1, 1, 0, 0, 0.0);

    tester.test_black_box();
}

#[cfg(feature = "image_formats")]
#[test]
fn cpp_rmqr_blackbox_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/cpp/rmqrcode-1",
        MultiFormatReader::default(),
        BarcodeFormat::RECTANGULAR_MICRO_QR_CODE,
    );
    // tester.ignore_pure = true;
    tester.add_test(3, 3, 0.0);
    tester.add_test(3, 3, 90.0);
    tester.add_test(3, 3, 180.0);
    tester.add_test(3, 3, 270.0);

    tester.add_hint(
        rxing::DecodeHintType::ALSO_INVERTED,
        rxing::DecodeHintValue::AlsoInverted(true),
    );

    tester.test_black_box();
}

//
