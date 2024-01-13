/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2007 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::qrcode::{
    cpp_port::Type,
    decoder::{ErrorCorrectionLevel, FormatInformation},
};

const MASKED_TEST_FORMAT_INFO: u32 = 0x2BED;
const MASKED_TEST_FORMAT_INFO2: u32 =
    ((0x2BED << 1) & 0b1111111000000000) | 0b100000000 | (0x2BED & 0b11111111); // insert the 'Dark Module'
const UNMASKED_TEST_FORMAT_INFO: u32 = MASKED_TEST_FORMAT_INFO ^ 0x5412;
const MICRO_MASKED_TEST_FORMAT_INFO: u32 = 0x3BBA;
// const MICRO_UNMASKED_TEST_FORMAT_INFO: u32 = MICRO_MASKED_TEST_FORMAT_INFO ^ 0x4445;
const RMQR_MASKED_TEST_FORMAT_INFO: u32 = 0x20137;
const RMQR_MASKED_TEST_FORMAT_INFO_SUB: u32 = 0x1F1FE;

const FORMAT_INFO_MASK_QR_MODEL1: u32 = 0x2825;
const FORMAT_INFO_MASK_MICRO: u32 = 0x4445;
const FORMAT_INFO_MASK_RMQR: u32 = 0x1FAB2; // Finder pattern side
const FORMAT_INFO_MASK_RMQR_SUB: u32 = 0x20A7B; // Finder sub pattern side

fn DoFormatInformationTest(formatInfo: u32, expectedMask: u8, expectedECL: ErrorCorrectionLevel) {
    let parsedFormat = FormatInformation::DecodeMQR(formatInfo);
    assert!(parsedFormat.isValid());
    assert_eq!(expectedMask, parsedFormat.data_mask);
    assert_eq!(expectedECL, parsedFormat.error_correction_level);
}

// Helper for rMQR to unset `numBits` number of bits
fn RMQRUnsetBits(formatInfoBits: u32, numBits: u32) -> u32 {
    let mut formatInfoBits = formatInfoBits;
    let mut numBits = numBits as i32;
    for i in 0..18 {
        // for (int i = 0; i < 18 && numBits; i++) {
        if (formatInfoBits & (1 << i)) != 0 {
            formatInfoBits ^= 1 << i;
            numBits -= 1;
        }
        if numBits == 0 {
            break;
        }
    }
    formatInfoBits
}

fn cpp_eq(rhs: &FormatInformation, lhs: &FormatInformation) {
    assert!(rhs.cpp_eq(lhs))
}

#[test]
fn Decode() {
    // Normal case
    let expected = FormatInformation::DecodeQR(MASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2);
    assert!(expected.isValid());
    assert_eq!(0x07, expected.data_mask);
    assert_eq!(ErrorCorrectionLevel::Q, expected.error_correction_level);
    // where the code forgot the mask!
    // assert_eq!(
    //     expected,
    //     FormatInformation::DecodeQR(UNMASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2)
    // );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeQR(UNMASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2),
    )
}

#[test]
fn DecodeWithBitDifference() {
    let expected = FormatInformation::DecodeQR(MASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2);
    // 1,2,3,4 bits difference
    cpp_eq(
        &expected,
        &FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x01,
            MASKED_TEST_FORMAT_INFO2 ^ 0x01,
        ),
    );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x03,
            MASKED_TEST_FORMAT_INFO2 ^ 0x03,
        ),
    );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x07,
            MASKED_TEST_FORMAT_INFO2 ^ 0x07,
        ),
    );
    let unexpected = FormatInformation::DecodeQR(
        MASKED_TEST_FORMAT_INFO ^ 0x0F,
        MASKED_TEST_FORMAT_INFO2 ^ 0x0F,
    );
    assert!(!&expected.cpp_eq(&unexpected));
    assert!(!(unexpected.isValid() && unexpected.qr_type() == Type::Model2));
}

#[test]
fn DecodeWithMisread() {
    let expected = FormatInformation::DecodeQR(MASKED_TEST_FORMAT_INFO, MASKED_TEST_FORMAT_INFO2);
    cpp_eq(
        &expected,
        &FormatInformation::DecodeQR(
            MASKED_TEST_FORMAT_INFO ^ 0x03,
            MASKED_TEST_FORMAT_INFO2 ^ 0x0F,
        ),
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
    cpp_eq(
        &expected,
        &FormatInformation::DecodeMQR(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x01),
    );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeMQR(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x03),
    );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeMQR(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x07),
    );

    // Bigger bit differences can return valid FormatInformation objects but the data mask and error
    // correction levels do not match.
    //	EXPECT_TRUE(FormatInformation::DecodeFormatInformation(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x3f).isValid());
    //	EXPECT_NE(expected.dataMask(), FormatInformation::DecodeFormatInformation(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x3f).dataMask());
    //	EXPECT_NE(expected.errorCorrectionLevel(),
    //			  FormatInformation::DecodeFormatInformation(MICRO_MASKED_TEST_FORMAT_INFO ^ 0x3f).errorCorrectionLevel());
}

#[test]
fn DecodeRMQR() {
    // Normal case
    let expected = FormatInformation::DecodeRMQR(
        RMQR_MASKED_TEST_FORMAT_INFO,
        RMQR_MASKED_TEST_FORMAT_INFO_SUB,
    );
    assert!(expected.isValid());
    assert_eq!(4, expected.data_mask);
    assert_eq!(ErrorCorrectionLevel::H, expected.error_correction_level);
    assert_eq!(FORMAT_INFO_MASK_RMQR, expected.mask);
    // Not catered for: where the code forgot the mask!
}

#[test]
fn DecodeRMQRWithBitDifference() {
    let expected = FormatInformation::DecodeRMQR(
        RMQR_MASKED_TEST_FORMAT_INFO,
        RMQR_MASKED_TEST_FORMAT_INFO_SUB,
    );
    assert_eq!(expected.error_correction_level, ErrorCorrectionLevel::H);
    // 1,2,3,4,5 bits difference
    cpp_eq(
        &expected,
        &FormatInformation::DecodeRMQR(
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO, 1),
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO_SUB, 1),
        ),
    );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeRMQR(
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO, 2),
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO_SUB, 2),
        ),
    );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeRMQR(
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO, 3),
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO_SUB, 3),
        ),
    );
    cpp_eq(
        &expected,
        &FormatInformation::DecodeRMQR(
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO, 4),
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO_SUB, 4),
        ),
    );
    let unexpected = FormatInformation::DecodeRMQR(
        RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO, 5),
        RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO_SUB, 5),
    );
    assert!(!(expected == unexpected));
    assert!(!(unexpected.isValid()));
    assert!(unexpected.qr_type() == Type::RectMicro); // Note `mask` (used to determine type) set regardless
}

#[test]
fn DecodeRMQRWithMisread() {
    let expected = FormatInformation::DecodeRMQR(
        RMQR_MASKED_TEST_FORMAT_INFO,
        RMQR_MASKED_TEST_FORMAT_INFO_SUB,
    );
    {
        let actual = FormatInformation::DecodeRMQR(
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO, 2),
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO_SUB, 4),
        );
        cpp_eq(&expected, &actual);
        assert_eq!(actual.mask, FORMAT_INFO_MASK_RMQR);
    }
    {
        let actual = FormatInformation::DecodeRMQR(
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO, 5),
            RMQRUnsetBits(RMQR_MASKED_TEST_FORMAT_INFO_SUB, 4),
        );
        cpp_eq(&expected, &actual);
        assert_eq!(actual.mask, FORMAT_INFO_MASK_RMQR_SUB);
    }
}
