/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2008 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::qrcode::{
    cpp_port::Type,
    decoder::{Mode, Version},
};

#[test]
fn ForBits() {
    assert_eq!(
        Mode::TERMINATOR,
        Mode::CodecModeForBits(0x00, Some(Type::Model2)).unwrap()
    );
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x01, Some(Type::Model2)).unwrap()
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x02, Some(Type::Model2)).unwrap()
    );
    assert_eq!(
        Mode::BYTE,
        Mode::CodecModeForBits(0x04, Some(Type::Model2)).unwrap()
    );
    assert_eq!(
        Mode::KANJI,
        Mode::CodecModeForBits(0x08, Some(Type::Model2)).unwrap()
    );
    assert!(Mode::CodecModeForBits(0x10, Some(Type::Model2)).is_err());
}

#[test]
fn CharacterCount() {
    // Spot check a few values
    assert_eq!(
        10,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::Model2(5).unwrap())
    );
    assert_eq!(
        12,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::Model2(26).unwrap())
    );
    assert_eq!(
        14,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::Model2(40).unwrap())
    );
    assert_eq!(
        9,
        Mode::CharacterCountBits(&Mode::ALPHANUMERIC, Version::Model2(6).unwrap())
    );
    assert_eq!(
        8,
        Mode::CharacterCountBits(&Mode::BYTE, Version::Model2(7).unwrap())
    );
    assert_eq!(
        8,
        Mode::CharacterCountBits(&Mode::KANJI, Version::Model2(8).unwrap())
    );
}

#[test]
fn MicroForBits() {
    // M1
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(Type::Micro)).unwrap()
    );
    // M2
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(Type::Micro)).unwrap()
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x01, Some(Type::Micro)).unwrap()
    );
    // M3
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(Type::Micro)).unwrap()
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x01, Some(Type::Micro)).unwrap()
    );
    assert_eq!(
        Mode::BYTE,
        Mode::CodecModeForBits(0x02, Some(Type::Micro)).unwrap()
    );
    assert_eq!(
        Mode::KANJI,
        Mode::CodecModeForBits(0x03, Some(Type::Micro)).unwrap()
    );
    // M4
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x00, Some(Type::Micro)).unwrap()
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x01, Some(Type::Micro)).unwrap()
    );
    assert_eq!(
        Mode::BYTE,
        Mode::CodecModeForBits(0x02, Some(Type::Micro)).unwrap()
    );
    assert_eq!(
        Mode::KANJI,
        Mode::CodecModeForBits(0x03, Some(Type::Micro)).unwrap()
    );

    assert!(Mode::CodecModeForBits(0x04, Some(Type::Micro)).is_err());
}

#[test]
fn MicroCharacterCount() {
    // Spot check a few values
    assert_eq!(
        3,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::Micro(1).unwrap())
    );
    assert_eq!(
        4,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::Micro(2).unwrap())
    );
    assert_eq!(
        6,
        Mode::CharacterCountBits(&Mode::NUMERIC, Version::Micro(4).unwrap())
    );
    assert_eq!(
        3,
        Mode::CharacterCountBits(&Mode::ALPHANUMERIC, Version::Micro(2).unwrap())
    );
    assert_eq!(
        4,
        Mode::CharacterCountBits(&Mode::BYTE, Version::Micro(3).unwrap())
    );
    assert_eq!(
        4,
        Mode::CharacterCountBits(&Mode::KANJI, Version::Micro(4).unwrap())
    );
}

#[test]
fn RMQRForBits() {
    assert_eq!(
        Mode::TERMINATOR,
        Mode::CodecModeForBits(0x00, Some(Type::RectMicro)).expect("could not decode 0x00")
    );
    assert_eq!(
        Mode::NUMERIC,
        Mode::CodecModeForBits(0x01, Some(Type::RectMicro)).expect("could not decode 0x01")
    );
    assert_eq!(
        Mode::ALPHANUMERIC,
        Mode::CodecModeForBits(0x02, Some(Type::RectMicro)).expect("could not decode 0x02")
    );
    assert_eq!(
        Mode::BYTE,
        Mode::CodecModeForBits(0x03, Some(Type::RectMicro)).expect("could not decode 0x03")
    );
    assert_eq!(
        Mode::KANJI,
        Mode::CodecModeForBits(0x04, Some(Type::RectMicro)).expect("could not decode 0x04")
    );
    assert_eq!(
        Mode::FNC1_FIRST_POSITION,
        Mode::CodecModeForBits(0x05, Some(Type::RectMicro)).expect("could not decode 0x05")
    );
    assert_eq!(
        Mode::FNC1_SECOND_POSITION,
        Mode::CodecModeForBits(0x06, Some(Type::RectMicro)).expect("could not decode 0x06")
    );
    assert_eq!(
        Mode::ECI,
        Mode::CodecModeForBits(0x07, Some(Type::RectMicro)).expect("could not decode 0x07")
    );
    assert!(Mode::CodecModeForBits(0x08, Some(Type::RectMicro)).is_err());
}

#[test]
fn RMQRCharacterCount() {
    // Spot check a few values
    assert_eq!(
        7,
        Mode::CharacterCountBits(
            &Mode::NUMERIC,
            Version::rMQR(5).expect("should return version")
        )
    );
    assert_eq!(
        8,
        Mode::CharacterCountBits(
            &Mode::NUMERIC,
            Version::rMQR(26).expect("should return version")
        )
    );
    assert_eq!(
        9,
        Mode::CharacterCountBits(
            &Mode::NUMERIC,
            Version::rMQR(32).expect("should return version")
        )
    );
    assert_eq!(
        5,
        Mode::CharacterCountBits(
            &Mode::ALPHANUMERIC,
            Version::rMQR(6).expect("should return version")
        )
    );
    assert_eq!(
        5,
        Mode::CharacterCountBits(
            &Mode::BYTE,
            Version::rMQR(7).expect("should return version")
        )
    );
    assert_eq!(
        5,
        Mode::CharacterCountBits(
            &Mode::KANJI,
            Version::rMQR(8).expect("should return version")
        )
    );
}
