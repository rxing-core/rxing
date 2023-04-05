/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2007 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::qrcode::decoder::{ErrorCorrectionLevel, FormatInformation};

const MASKED_TEST_FORMAT_INFO: u32 = 0x2BED;
const MASKED_TEST_FORMAT_INFO2: u32 =
    ((0x2BED << 1) & 0b1111111000000000) | 0b100000000 | (0x2BED & 0b11111111); // insert the 'Dark Module'
const UNMASKED_TEST_FORMAT_INFO: u32 = MASKED_TEST_FORMAT_INFO ^ 0x5412;
const MICRO_MASKED_TEST_FORMAT_INFO: u32 = 0x3BBA;
// const MICRO_UNMASKED_TEST_FORMAT_INFO: u32 = MICRO_MASKED_TEST_FORMAT_INFO ^ 0x4445;

fn DoFormatInformationTest(formatInfo: u32, expectedMask: u8, expectedECL: ErrorCorrectionLevel) {
    let parsedFormat = FormatInformation::DecodeMQR(formatInfo);
    assert!(parsedFormat.isValid());
    assert_eq!(expectedMask, parsedFormat.data_mask);
    assert_eq!(expectedECL, parsedFormat.error_correction_level);
}

#[test]
fn Decode() {
    // Normal case
    let expected = FormatInformation::DecodeQR(MASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2);
    assert!(expected.isValid());
    assert_eq!(0x07, expected.data_mask);
    assert_eq!(ErrorCorrectionLevel::Q, expected.error_correction_level);
    // where the code forgot the mask!
    assert_eq!(
        expected,
        FormatInformation::DecodeQR(UNMASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2)
    );
}

#[test]
fn DecodeWithBitDifference() {
    let expected = FormatInformation::DecodeQR(MASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2);
    // 1,2,3,4 bits difference
    assert_eq!(
        expected,
        FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x01,
            MASKED_TEST_FORMAT_INFO2 ^ 0x01
        )
    );
    assert_eq!(
        expected,
        FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x03,
            MASKED_TEST_FORMAT_INFO2 ^ 0x03
        )
    );
    assert_eq!(
        expected,
        FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x07,
            MASKED_TEST_FORMAT_INFO2 ^ 0x07
        )
    );
    assert!(!FormatInformation::DecodeQR(
        MASKED_TEST_FORMAT_INFO ^ 0x0F,
        MASKED_TEST_FORMAT_INFO2 ^ 0x0F
    )
    .isValid());
}

#[test]
fn DecodeWithMisread() {
    let expected = FormatInformation::DecodeQR(MASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2);
    assert_eq!(
        expected,
        FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x03,
            MASKED_TEST_FORMAT_INFO2 ^ 0x0F
        )
    );
}

#[test]
fn DecodeMicro() {
    // Normal cases.
    DoFormatInformationTest(0x4445, 0x0, ErrorCorrectionLevel::L);
    DoFormatInformationTest(0x4172, 0x1, ErrorCorrectionLevel::L);
    DoFormatInformationTest(0x5fc0, 0x2, ErrorCorrectionLevel::L);
    DoFormatInformationTest(0x5af7, 0x3, ErrorCorrectionLevel::L);
    DoFormatInformationTest(0x6793, 0x0, ErrorCorrectionLevel::M);
    DoFormatInformationTest(0x62a4, 0x1, ErrorCorrectionLevel::M);
    DoFormatInformationTest(0x3e8d, 0x2, ErrorCorrectionLevel::Q);
    DoFormatInformationTest(MICRO_MASKED_TEST_FORMAT_INFO, 0x3, ErrorCorrectionLevel::Q);

    // where the code forgot the mask!
    //	DoFormatInformationTest(MICRO_UNMASKED_TEST_FORMAT_INFO, 0x3, ErrorCorrectionLevel::Quality);
}

// This doesn't work as expected because the implementation of the decode tries with
// and without the mask (0x4445).  This effectively adds a tolerance of 5 bits to the Hamming
// distance calculation.
#[test]
fn DecodeMicroWithBitDifference() {
    let expected = FormatInformation::DecodeMQR(MICRO_MASKED_TEST_FORMAT_INFO);

    // 1,2,3 bits difference
    assert_eq!(
        expected,
        FormatInformation::DecodeMQR(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x01)
    );
    assert_eq!(
        expected,
        FormatInformation::DecodeMQR(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x03)
    );
    assert_eq!(
        expected,
        FormatInformation::DecodeMQR(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x07)
    );

    // Bigger bit differences can return valid FormatInformation objects but the data mask and error
    // correction levels do not match.
    //	EXPECT_TRUE(FormatInformation::DecodeFormatInformation(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x3f).isValid());
    //	EXPECT_NE(expected.dataMask(), FormatInformation::DecodeFormatInformation(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x3f).dataMask());
    //	EXPECT_NE(expected.errorCorrectionLevel(),
    //			  FormatInformation::DecodeFormatInformation(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x3f).errorCorrectionLevel());
}
