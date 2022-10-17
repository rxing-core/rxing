/*
 * Copyright 2007 ZXing authors
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

use crate::qrcode::decoder::{ErrorCorrectionLevel, FormatInformation};

/**
 * @author Sean Owen
 */

const MASKED_TEST_FORMAT_INFO: u32 = 0x2BED;
const UNMASKED_TEST_FORMAT_INFO: u32 = MASKED_TEST_FORMAT_INFO ^ 0x5412;

#[test]
fn testBitsDiffering() {
    assert_eq!(0, FormatInformation::numBitsDiffering(1, 1));
    assert_eq!(1, FormatInformation::numBitsDiffering(0, 2));
    assert_eq!(2, FormatInformation::numBitsDiffering(1, 2));
    assert_eq!(32, FormatInformation::numBitsDiffering(-1i32 as u32, 0));
}

#[test]
fn testDecode() {
    // Normal case
    let expected = FormatInformation::decodeFormatInformation(
        MASKED_TEST_FORMAT_INFO,
        MASKED_TEST_FORMAT_INFO,
    );
    assert!(expected.is_some());
    let expected = expected.unwrap();
    assert_eq!(0x07, expected.getDataMask());
    assert_eq!(ErrorCorrectionLevel::Q, expected.getErrorCorrectionLevel());
    // where the code forgot the mask!
    assert_eq!(
        expected,
        FormatInformation::decodeFormatInformation(
            UNMASKED_TEST_FORMAT_INFO,
            MASKED_TEST_FORMAT_INFO
        )
        .expect("return")
    );
}

#[test]
fn testDecodeWithBitDifference() {
    let expected = FormatInformation::decodeFormatInformation(
        MASKED_TEST_FORMAT_INFO,
        MASKED_TEST_FORMAT_INFO,
    )
    .unwrap();
    // 1,2,3,4 bits difference
    assert_eq!(
        expected,
        FormatInformation::decodeFormatInformation(
            MASKED_TEST_FORMAT_INFO ^ 0x01,
            MASKED_TEST_FORMAT_INFO ^ 0x01
        )
        .expect("return")
    );
    assert_eq!(
        expected,
        FormatInformation::decodeFormatInformation(
            MASKED_TEST_FORMAT_INFO ^ 0x03,
            MASKED_TEST_FORMAT_INFO ^ 0x03
        )
        .expect("return")
    );
    assert_eq!(
        expected,
        FormatInformation::decodeFormatInformation(
            MASKED_TEST_FORMAT_INFO ^ 0x07,
            MASKED_TEST_FORMAT_INFO ^ 0x07
        )
        .expect("return")
    );
    assert!(FormatInformation::decodeFormatInformation(
        MASKED_TEST_FORMAT_INFO ^ 0x0F,
        MASKED_TEST_FORMAT_INFO ^ 0x0F
    )
    .is_none());
}

#[test]
fn testDecodeWithMisread() {
    let expected = FormatInformation::decodeFormatInformation(
        MASKED_TEST_FORMAT_INFO,
        MASKED_TEST_FORMAT_INFO,
    )
    .unwrap();
    assert_eq!(
        expected,
        FormatInformation::decodeFormatInformation(
            MASKED_TEST_FORMAT_INFO ^ 0x03,
            MASKED_TEST_FORMAT_INFO ^ 0x0F
        )
        .unwrap()
    );
}
