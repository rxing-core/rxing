/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2008 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

// #include "qrcode/QRErrorCorrectionLevel.h"

// #include "gtest/gtest.h"

// using namespace ZXing;
// using namespace ZXing::QRCode;

use crate::qrcode::decoder::ErrorCorrectionLevel;

#[test]
fn ForBits() {
    assert_eq!(
        ErrorCorrectionLevel::M,
        ErrorCorrectionLevel::ECLevelFromBits(0, false)
    );
    assert_eq!(
        ErrorCorrectionLevel::L,
        ErrorCorrectionLevel::ECLevelFromBits(1, false)
    );
    assert_eq!(
        ErrorCorrectionLevel::H,
        ErrorCorrectionLevel::ECLevelFromBits(2, false)
    );
    assert_eq!(
        ErrorCorrectionLevel::Q,
        ErrorCorrectionLevel::ECLevelFromBits(3, false)
    );
}

#[test]
fn ForMicroBits() {
    assert_eq!(
        ErrorCorrectionLevel::L,
        ErrorCorrectionLevel::ECLevelFromBits(0, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::L,
        ErrorCorrectionLevel::ECLevelFromBits(1, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::M,
        ErrorCorrectionLevel::ECLevelFromBits(2, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::L,
        ErrorCorrectionLevel::ECLevelFromBits(3, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::M,
        ErrorCorrectionLevel::ECLevelFromBits(4, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::L,
        ErrorCorrectionLevel::ECLevelFromBits(5, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::M,
        ErrorCorrectionLevel::ECLevelFromBits(6, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::Q,
        ErrorCorrectionLevel::ECLevelFromBits(7, true)
    );

    assert_eq!(
        ErrorCorrectionLevel::Q,
        ErrorCorrectionLevel::ECLevelFromBitsSigned(-1, true)
    );
    assert_eq!(
        ErrorCorrectionLevel::L,
        ErrorCorrectionLevel::ECLevelFromBits(8, true)
    );
}

#[test]
fn ToString() {
    // using namespace std::literals;

    assert_eq!("L", ErrorCorrectionLevel::L.to_string());
    assert_eq!("M", ErrorCorrectionLevel::M.to_string());
    assert_eq!("Q", ErrorCorrectionLevel::Q.to_string());
    assert_eq!("H", ErrorCorrectionLevel::H.to_string());
}
