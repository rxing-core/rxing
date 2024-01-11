/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2008 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::qrcode::decoder::{Mode, Version};

#[test]
fn ForBits() {
    assert_eq!(
        Mode::TERMINATOR,
        Mode::CodecModeForBits(0x00, None).unwrap()
    );
    assert_eq!(Mode::NUMERIC, Mode::CodecModeForBits(0x01, None).unwrap());
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x02, None).unwrap()
    );
    assert_eq!(Mode::BYTE, Mode::CodecModeForBits(0x04, None).unwrap());
    assert_eq!(Mode::KANJI, Mode::CodecModeForBits(0x08, None).unwrap());
    assert!(Mode::CodecModeForBits(0x10, None).is_err());
}

#[test]
fn CharacterCount() {
    // Spot check a few values
    assert_eq!(
        10,
        Mode::CharacterCountBits(
            &Mode::NUMERIC,
            Version::FromNumber(5, false, false).unwrap()
        )
    );
    assert_eq!(
        12,
        Mode::CharacterCountBits(
            &Mode::NUMERIC,
            Version::FromNumber(26, false, false).unwrap()
        )
    );
    assert_eq!(
        14,
        Mode::CharacterCountBits(
            &Mode::NUMERIC,
            Version::FromNumber(40, false, false).unwrap()
        )
    );
    assert_eq!(
        9,
        Mode::CharacterCountBits(
            &Mode::ALPHANUMERIC,
            Version::FromNumber(6, false, false).unwrap()
        )
    );
    assert_eq!(
        8,
        Mode::CharacterCountBits(&Mode::BYTE, Version::FromNumber(7, false, false).unwrap())
    );
    assert_eq!(
        8,
        Mode::CharacterCountBits(&Mode::KANJI, Version::FromNumber(8, false, false).unwrap())
    );
}

#[test]
fn MicroForBits() {
    // M1
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(true)).unwrap()
    );
    // M2
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(true)).unwrap()
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x01, Some(true)).unwrap()
    );
    // M3
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(true)).unwrap()
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x01, Some(true)).unwrap()
    );
    assert_eq!(
        Mode::BYTE,
        Mode::CodecModeForBits(0x02, Some(true)).unwrap()
    );
    assert_eq!(
        Mode::KANJI,
        Mode::CodecModeForBits(0x03, Some(true)).unwrap()
    );
    // M4
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(true)).unwrap()
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x01, Some(true)).unwrap()
    );
    assert_eq!(
        Mode::BYTE,
        Mode::CodecModeForBits(0x02, Some(true)).unwrap()
    );
    assert_eq!(
        Mode::KANJI,
        Mode::CodecModeForBits(0x03, Some(true)).unwrap()
    );

    assert!(Mode::CodecModeForBits(0x04, Some(true)).is_err());
}

#[test]
fn MicroCharacterCount() {
    // Spot check a few values
    assert_eq!(
        3,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::FromNumber(1, true, false).unwrap())
    );
    assert_eq!(
        4,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::FromNumber(2, true, false).unwrap())
    );
    assert_eq!(
        6,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::FromNumber(4, true, false).unwrap())
    );
    assert_eq!(
        3,
        Mode::CharacterCountBits(
            &Mode::ALPHANUMERIC,
            Version::FromNumber(2, true, false).unwrap()
        )
    );
    assert_eq!(
        4,
        Mode::CharacterCountBits(&Mode::BYTE, Version::FromNumber(3, true, false).unwrap())
    );
    assert_eq!(
        4,
        Mode::CharacterCountBits(&Mode::KANJI, Version::FromNumber(4, true, false).unwrap())
    );
}
