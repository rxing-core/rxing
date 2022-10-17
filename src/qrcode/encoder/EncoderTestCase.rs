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

use std::collections::HashMap;

use crate::{
    common::BitArray,
    qrcode::{
        decoder::{ErrorCorrectionLevel, Mode, Version},
        encoder::{encoder, MinimalEncoder},
    },
    EncodeHintType, EncodeHintValue,
};
use encoding::EncodingRef;
use lazy_static::lazy_static;

use super::QRCode;

lazy_static! {
    static ref SHIFT_JIS_CHARSET: EncodingRef =
        encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
}

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author mysen@google.com (Chris Mysen) - ported from C++
 */

#[test]
fn testGetAlphanumericCode() {
    // The first ten code points are numbers.
    for i in 0..10u8 {
        // for (int i = 0; i < 10; ++i) {
        assert_eq!(i as i8, encoder::getAlphanumericCode((b'0' + i) as u32));
    }

    // The next 26 code points are capital alphabet letters.
    for i in 10..36 {
        // for (int i = 10; i < 36; ++i) {
        assert_eq!(
            i as i8,
            encoder::getAlphanumericCode((b'A' + i - 10) as u32)
        );
    }

    // Others are symbol letters
    assert_eq!(36, encoder::getAlphanumericCode(b' ' as u32));
    assert_eq!(37, encoder::getAlphanumericCode(b'$' as u32));
    assert_eq!(38, encoder::getAlphanumericCode(b'%' as u32));
    assert_eq!(39, encoder::getAlphanumericCode(b'*' as u32));
    assert_eq!(40, encoder::getAlphanumericCode(b'+' as u32));
    assert_eq!(41, encoder::getAlphanumericCode(b'-' as u32));
    assert_eq!(42, encoder::getAlphanumericCode(b'.' as u32));
    assert_eq!(43, encoder::getAlphanumericCode(b'/' as u32));
    assert_eq!(44, encoder::getAlphanumericCode(b':' as u32));

    // Should return -1 for other letters;
    assert_eq!(-1, encoder::getAlphanumericCode(b'a' as u32));
    assert_eq!(-1, encoder::getAlphanumericCode(b'#' as u32));
    assert_eq!(-1, encoder::getAlphanumericCode(b'\0' as u32));
}

#[test]
fn testChooseMode() {
    // Numeric Mode::
    assert_eq!(Mode::NUMERIC, encoder::chooseMode("0"));
    assert_eq!(Mode::NUMERIC, encoder::chooseMode("0123456789"));
    // Alphanumeric Mode::
    assert_eq!(Mode::ALPHANUMERIC, encoder::chooseMode("A"));
    assert_eq!(
        Mode::ALPHANUMERIC,
        encoder::chooseMode("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:")
    );
    // 8-bit byte Mode::
    assert_eq!(Mode::BYTE, encoder::chooseMode("a"));
    assert_eq!(Mode::BYTE, encoder::chooseMode("#"));
    assert_eq!(Mode::BYTE, encoder::chooseMode(""));
    // Kanji Mode::  We used to use MODE_KANJI for these, but we stopped
    // doing that as we cannot distinguish Shift_JIS from other encodings
    // from data bytes alone.  See also comments in qrcode_encoder::h.

    // AIUE in Hiragana in Shift_JIS
    assert_eq!(
        Mode::BYTE,
        encoder::chooseMode(&shiftJISString(&[0x8, 0xa, 0x8, 0xa, 0x8, 0xa, 0x8, 0xa6]))
    );

    // Nihon in Kanji in Shift_JIS.
    assert_eq!(
        Mode::BYTE,
        encoder::chooseMode(&shiftJISString(&[0x9, 0xf, 0x9, 0x7b]))
    );

    // Sou-Utsu-Byou in Kanji in Shift_JIS.
    assert_eq!(
        Mode::BYTE,
        encoder::chooseMode(&shiftJISString(&[0xe, 0x4, 0x9, 0x5, 0x9, 0x61]))
    );
}

#[test]
fn testEncode() {
    let qrCode = encoder::encode("ABCDEF", ErrorCorrectionLevel::H).expect("encode");
    let expected = r"<<
 mode: ALPHANUMERIC
 ecLevel: H
 version: 1
 maskPattern: 0
 matrix:
 1 1 1 1 1 1 1 0 1 1 1 1 0 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 0 1 1 1 0 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 1 0 1 1 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 1 1 0 1 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 1 1 1 0 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 1 0 0 0 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0
 0 0 1 0 1 1 1 0 1 1 0 0 1 1 0 0 0 1 0 0 1
 1 0 1 1 1 0 0 1 0 0 0 1 0 1 0 0 0 0 0 0 0
 0 0 1 1 0 0 1 0 1 0 0 0 1 0 1 0 1 0 1 1 0
 1 1 0 1 0 1 0 1 1 1 0 1 0 1 0 0 0 0 0 1 0
 0 0 1 1 0 1 1 1 1 0 0 0 1 0 1 0 1 1 1 1 0
 0 0 0 0 0 0 0 0 1 0 0 1 1 1 0 1 0 1 0 0 0
 1 1 1 1 1 1 1 0 0 0 1 0 1 0 1 1 0 0 0 0 1
 1 0 0 0 0 0 1 0 1 1 1 1 0 1 0 1 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 0 1 1 0 1 0 1 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 1 1 0 1 1 1 1 0 1 0 1 0
 1 0 1 1 1 0 1 0 1 0 0 0 1 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 1 1 0 1 1 0 1 0 0 0 1 1
 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 1 0 1 0 1
>>
";
    assert_eq!(expected, qrCode.to_string());
}

#[test]
fn testEncodeWithVersion() {
    let mut hints = HashMap::new();
    hints.insert(
        EncodeHintType::QR_VERSION,
        EncodeHintValue::QrVersion("7".to_owned()),
    );
    let qrCode =
        encoder::encode_with_hints("ABCDEF", ErrorCorrectionLevel::H, &hints).expect("encode");
    assert!(qrCode.to_string().contains(" version: 7\n"));
}

#[test]
#[should_panic]
fn testEncodeWithVersionTooSmall() {
    let mut hints = HashMap::new();
    hints.insert(
        EncodeHintType::QR_VERSION,
        EncodeHintValue::QrVersion("3".to_owned()),
    );
    encoder::encode_with_hints(
        "THISMESSAGEISTOOLONGFORAQRCODEVERSION3",
        ErrorCorrectionLevel::H,
        &hints,
    )
    .expect("encode");
}

#[test]
fn testSimpleutf8ECI() {
    let mut hints = HashMap::new();
    hints.insert(
        EncodeHintType::CHARACTER_SET,
        EncodeHintValue::CharacterSet("utf8".to_owned()),
    );
    let qrCode =
        encoder::encode_with_hints("hello", ErrorCorrectionLevel::H, &hints).expect("encode");
    let expected = r"<<
 mode: BYTE
 ecLevel: H
 version: 1
 maskPattern: 3
 matrix:
 1 1 1 1 1 1 1 0 0 0 0 0 0 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 0 0 1 0 1 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 1 0 1 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 1 1 0 1 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 0 1 0 1 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 0 0 0 1 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 0 0
 0 0 1 1 0 0 1 1 1 1 0 0 0 1 1 0 1 0 0 0 0
 0 0 1 1 1 0 0 0 0 0 1 1 0 0 0 1 0 1 1 1 0
 0 1 0 1 0 1 1 1 0 1 0 1 0 0 0 0 0 1 1 1 1
 1 1 0 0 1 0 0 1 1 0 0 1 1 1 1 0 1 0 1 1 0
 0 0 0 0 1 0 1 1 1 1 0 0 0 0 0 1 0 0 1 0 0
 0 0 0 0 0 0 0 0 1 1 1 1 0 0 1 1 1 0 0 0 1
 1 1 1 1 1 1 1 0 1 1 1 0 1 0 1 1 0 0 1 0 0
 1 0 0 0 0 0 1 0 0 0 1 0 0 1 1 1 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 1 0 0 0 0 1 1 0 0 0 0 0
 1 0 1 1 1 0 1 0 1 1 1 0 1 0 0 0 1 1 0 0 0
 1 0 1 1 1 0 1 0 1 1 0 0 0 1 0 0 1 0 0 0 0
 1 0 0 0 0 0 1 0 0 0 0 1 1 0 1 0 1 0 1 1 0
 1 1 1 1 1 1 1 0 0 1 0 1 1 1 0 1 1 0 0 0 0
>>
";
    assert_eq!(expected, qrCode.to_string());
}

#[test]
fn testEncodeKanjiMode() {
    let mut hints = HashMap::new();

    hints.insert(
        EncodeHintType::CHARACTER_SET,
        EncodeHintValue::CharacterSet("Shift_JIS".to_owned()),
    );
    // Nihon in Kanji
    let qrCode = encoder::encode_with_hints("\u{65e5}\u{672c}", ErrorCorrectionLevel::M, &hints)
        .expect("encode");
    let expected = r"<<
 mode: KANJI
 ecLevel: M
 version: 1
 maskPattern: 4
 matrix:
 1 1 1 1 1 1 1 0 1 1 1 1 0 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 0 0 0 1 1 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 0 1 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 0 1 0 1 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 1 0 1 1 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 1 0 1 0 1 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0
 1 0 0 0 1 0 1 1 1 0 0 0 1 1 1 1 1 1 0 0 1
 0 1 1 0 0 1 0 1 1 0 1 0 1 1 1 0 0 0 1 0 1
 1 1 1 1 0 1 1 1 0 0 1 0 1 1 0 0 0 0 1 1 1
 1 0 1 0 1 1 0 0 0 0 1 1 1 0 0 1 0 0 1 1 0
 0 0 1 0 1 1 1 1 1 1 1 1 0 0 1 1 1 1 0 1 1
 0 0 0 0 0 0 0 0 1 1 1 1 1 0 0 1 0 1 0 0 0
 1 1 1 1 1 1 1 0 1 1 0 1 0 0 1 1 1 1 1 1 0
 1 0 0 0 0 0 1 0 0 0 0 0 0 1 1 0 1 0 1 0 1
 1 0 1 1 1 0 1 0 1 0 1 0 1 1 1 0 0 0 1 1 1
 1 0 1 1 1 0 1 0 0 1 0 0 1 1 1 0 0 0 1 1 1
 1 0 1 1 1 0 1 0 0 1 1 0 1 1 0 0 0 1 0 0 0
 1 0 0 0 0 0 1 0 0 0 1 1 1 0 0 1 0 1 0 0 0
 1 1 1 1 1 1 1 0 1 1 1 1 0 0 1 1 1 0 1 1 0
>>
";
    assert_eq!(expected, qrCode.to_string());
}

#[test]
fn testEncodeShiftjisNumeric() {
    let mut hints = HashMap::new();

    hints.insert(
        EncodeHintType::CHARACTER_SET,
        EncodeHintValue::CharacterSet("Shift_JIS".to_owned()),
    );

    let qrCode =
        encoder::encode_with_hints("0123", ErrorCorrectionLevel::M, &hints).expect("encode");
    let expected = r"<<
 mode: NUMERIC
 ecLevel: M
 version: 1
 maskPattern: 0
 matrix:
 1 1 1 1 1 1 1 0 0 0 0 0 1 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 1 1 0 1 0 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 1 1 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 0 1 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 0 1 1 1 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 1 0 1 0 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0 0
 1 0 1 0 1 0 1 0 0 0 0 0 1 0 0 0 1 0 0 1 0
 0 0 0 0 0 0 0 1 1 0 1 1 0 1 0 1 0 1 0 1 0
 0 1 0 1 0 1 1 1 1 0 0 1 0 1 1 1 0 1 0 1 0
 0 1 1 1 0 0 0 0 0 0 1 1 1 1 0 1 1 1 0 1 0
 0 0 0 1 1 1 1 1 1 1 1 1 0 1 1 1 0 0 1 0 1
 0 0 0 0 0 0 0 0 1 1 0 0 0 0 1 0 0 0 1 1 0
 1 1 1 1 1 1 1 0 0 1 0 0 1 0 0 0 1 0 0 0 1
 1 0 0 0 0 0 1 0 0 1 0 0 0 0 1 0 0 0 1 0 0
 1 0 1 1 1 0 1 0 1 1 0 0 1 0 1 0 1 0 1 0 1
 1 0 1 1 1 0 1 0 0 1 1 1 0 1 0 1 0 1 0 1 0
 1 0 1 1 1 0 1 0 1 0 1 1 0 1 1 1 0 1 1 0 1
 1 0 0 0 0 0 1 0 0 0 1 1 1 1 0 1 1 1 0 0 0
 1 1 1 1 1 1 1 0 1 0 1 1 0 1 1 1 0 1 1 0 1
>>
";
    assert_eq!(expected, qrCode.to_string());
}

#[test]
fn testEncodeGS1WithStringTypeHint() {
    let mut hints = HashMap::new();

    hints.insert(
        EncodeHintType::GS1_FORMAT,
        EncodeHintValue::Gs1Format("true".to_owned()),
    );
    let qrCode = encoder::encode_with_hints("100001%11171218", ErrorCorrectionLevel::H, &hints)
        .expect("encode");
    verifyGS1EncodedData(&qrCode);
}

#[test]
fn testEncodeGS1WithBooleanTypeHint() {
    let mut hints = HashMap::new();

    hints.insert(
        EncodeHintType::GS1_FORMAT,
        EncodeHintValue::Gs1Format("true".to_owned()),
    );
    let qrCode = encoder::encode_with_hints("100001%11171218", ErrorCorrectionLevel::H, &hints)
        .expect("encode");
    verifyGS1EncodedData(&qrCode);
}

#[test]
fn testDoesNotEncodeGS1WhenBooleanTypeHintExplicitlyFalse() {
    let mut hints = HashMap::new();

    hints.insert(
        EncodeHintType::GS1_FORMAT,
        EncodeHintValue::Gs1Format("false".to_owned()),
    );

    let qrCode =
        encoder::encode_with_hints("ABCDEF", ErrorCorrectionLevel::H, &hints).expect("encode");
    verifyNotGS1EncodedData(&qrCode);
}

#[test]
fn testDoesNotEncodeGS1WhenStringTypeHintExplicitlyFalse() {
    let mut hints = HashMap::new();

    hints.insert(
        EncodeHintType::GS1_FORMAT,
        EncodeHintValue::Gs1Format("false".to_owned()),
    );
    let qrCode =
        encoder::encode_with_hints("ABCDEF", ErrorCorrectionLevel::H, &hints).expect("encode");
    verifyNotGS1EncodedData(&qrCode);
}

#[test]
fn testGS1ModeHeaderWithECI() {
    let mut hints = HashMap::new();

    hints.insert(
        EncodeHintType::CHARACTER_SET,
        EncodeHintValue::CharacterSet("utf8".to_owned()),
    );
    hints.insert(
        EncodeHintType::GS1_FORMAT,
        EncodeHintValue::Gs1Format("true".to_owned()),
    );
    let qrCode =
        encoder::encode_with_hints("hello", ErrorCorrectionLevel::H, &hints).expect("encode");
    let expected = r"<<
 mode: BYTE
 ecLevel: H
 version: 1
 maskPattern: 6
 matrix:
 1 1 1 1 1 1 1 0 0 0 1 1 0 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 0 1 1 0 0 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 1 1 0 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 1 0 1 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 0 1 1 0 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 1 0 0 1 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0
 0 0 0 1 1 0 1 1 0 1 0 0 0 0 0 0 0 1 1 0 0
 0 1 0 1 1 0 0 1 0 1 1 1 1 1 1 0 1 1 1 0 1
 0 1 1 1 1 0 1 0 0 1 0 1 0 1 1 1 0 0 1 0 1
 1 1 1 1 1 0 0 1 0 0 0 1 1 0 0 1 0 0 1 0 0
 1 0 0 1 0 0 1 1 0 1 1 0 1 0 1 0 0 1 0 0 1
 0 0 0 0 0 0 0 0 1 1 1 1 1 1 0 0 1 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 0 1 0 1 1 0 1 0 1 0 0
 1 0 0 0 0 0 1 0 0 1 0 0 0 0 1 0 1 1 1 0 0
 1 0 1 1 1 0 1 0 1 1 0 1 1 0 0 0 1 1 0 0 0
 1 0 1 1 1 0 1 0 1 0 1 1 1 1 1 0 0 0 1 1 0
 1 0 1 1 1 0 1 0 0 0 1 0 0 1 0 0 1 0 1 1 1
 1 0 0 0 0 0 1 0 0 1 0 0 0 0 0 0 0 1 1 0 0
 1 1 1 1 1 1 1 0 0 1 0 1 0 0 1 0 0 0 0 0 0
>>
";
    assert_eq!(expected, qrCode.to_string());
}

#[test]
fn testAppendModeInfo() {
    let mut bits = BitArray::new();
    assert!(encoder::appendModeInfo(Mode::NUMERIC, &mut bits).is_ok());
    assert_eq!(" ...X", bits.to_string());
}

#[test]
fn testAppendLengthInfo() {
    let mut bits = BitArray::new();
    encoder::appendLengthInfo(
        1, // 1 letter (1/1).
        Version::getVersionForNumber(1).unwrap(),
        Mode::NUMERIC,
        &mut bits,
    )
    .expect("ok");
    assert_eq!(" ........ .X", bits.to_string()); // 10 bits.
    let mut bits = BitArray::new();
    encoder::appendLengthInfo(
        2, // 2 letters (2/1).
        Version::getVersionForNumber(10).unwrap(),
        Mode::ALPHANUMERIC,
        &mut bits,
    )
    .expect("ok");
    assert_eq!(" ........ .X.", bits.to_string()); // 11 bits.
    let mut bits = BitArray::new();
    encoder::appendLengthInfo(
        255, // 255 letter (255/1).
        Version::getVersionForNumber(27).unwrap(),
        Mode::BYTE,
        &mut bits,
    )
    .expect("ok");
    assert_eq!(" ........ XXXXXXXX", bits.to_string()); // 16 bits.
    let mut bits = BitArray::new();
    encoder::appendLengthInfo(
        512, // 512 letters (1024/2).
        Version::getVersionForNumber(40).unwrap(),
        Mode::KANJI,
        &mut bits,
    )
    .expect("ok");
    assert_eq!(" ..X..... ....", bits.to_string()); // 12 bits.
}

#[test]
fn testAppendBytes() {
    // Should use appendNumericBytes.
    // 1 = 01 = 0001 in 4 bits.
    let mut bits = BitArray::new();
    encoder::appendBytes(
        "1",
        Mode::NUMERIC,
        &mut bits,
        encoder::DEFAULT_BYTE_MODE_ENCODING,
    )
    .expect("ok");
    assert_eq!(" ...X", bits.to_string());
    // Should use appendAlphanumericBytes.
    // A = 10 = 0xa = 001010 in 6 bits
    let mut bits = BitArray::new();
    encoder::appendBytes(
        "A",
        Mode::ALPHANUMERIC,
        &mut bits,
        encoder::DEFAULT_BYTE_MODE_ENCODING,
    )
    .expect("ok");
    assert_eq!(" ..X.X.", bits.to_string());
    // Lower letters such as 'a' cannot be encoded in MODE_ALPHANUMERIC.
    //try {
    if encoder::appendBytes(
        "a",
        Mode::ALPHANUMERIC,
        &mut bits,
        encoder::DEFAULT_BYTE_MODE_ENCODING,
    )
    .is_ok()
    {
        panic!("should never be ok");
    }
    //} catch (WriterException we) {
    // good
    //}
    // Should use append8BitBytes.
    // 0x61, 0x62, 0x63
    let mut bits = BitArray::new();
    encoder::appendBytes(
        "abc",
        Mode::BYTE,
        &mut bits,
        encoder::DEFAULT_BYTE_MODE_ENCODING,
    )
    .expect("ok");
    assert_eq!(" .XX....X .XX...X. .XX...XX", bits.to_string());
    // Anything can be encoded in QRCode.MODE_8BIT_BYTE.
    encoder::appendBytes(
        "\0",
        Mode::BYTE,
        &mut bits,
        encoder::DEFAULT_BYTE_MODE_ENCODING,
    )
    .expect("ok");
    // Should use appendKanjiBytes.
    // 0x93, 0x5f
    let mut bits = BitArray::new();
    encoder::appendBytes(
        &shiftJISString(&[0x93, 0x5f]),
        Mode::KANJI,
        &mut bits,
        encoder::DEFAULT_BYTE_MODE_ENCODING,
    )
    .expect("ok");
    assert_eq!(" .XX.XX.. XXXXX", bits.to_string());
}

#[test]
fn testTerminateBits() {
    let mut v = BitArray::new();
    encoder::terminateBits(0, &mut v).expect("terminate");
    assert_eq!("", v.to_string());
    let mut v = BitArray::new();
    encoder::terminateBits(1, &mut v).expect("terminate");
    assert_eq!(" ........", v.to_string());
    let mut v = BitArray::new();
    v.appendBits(0, 3).expect("terminate"); // Append 000
    encoder::terminateBits(1, &mut v).expect("terminate");
    assert_eq!(" ........", v.to_string());
    let mut v = BitArray::new();
    v.appendBits(0, 5).expect("terminate"); // Append 00000
    encoder::terminateBits(1, &mut v).expect("terminate");
    assert_eq!(" ........", v.to_string());
    let mut v = BitArray::new();
    v.appendBits(0, 8).expect("terminate"); // Append 00000000
    encoder::terminateBits(1, &mut v).expect("terminate");
    assert_eq!(" ........", v.to_string());
    let mut v = BitArray::new();
    encoder::terminateBits(2, &mut v).expect("terminate");
    assert_eq!(" ........ XXX.XX..", v.to_string());
    let mut v = BitArray::new();
    v.appendBits(0, 1).expect("terminate"); // Append 0
    encoder::terminateBits(3, &mut v).expect("terminate");
    assert_eq!(" ........ XXX.XX.. ...X...X", v.to_string());
}

#[test]
fn testGetNumDataBytesAndNumECBytesForBlockID() {
    // Version 1-H.
    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(26, 9, 1, 0).expect("ok");
    assert_eq!(9, numDataBytes);
    assert_eq!(17, numEcBytes);

    // Version 3-H.  2 blocks.
    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(70, 26, 2, 0).expect("ok");
    assert_eq!(13, numDataBytes);
    assert_eq!(22, numEcBytes);
    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(70, 26, 2, 1).expect("ok");
    assert_eq!(13, numDataBytes);
    assert_eq!(22, numEcBytes);

    // Version 7-H. (4 + 1) blocks.
    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(196, 66, 5, 0).expect("ok");
    assert_eq!(13, numDataBytes);
    assert_eq!(26, numEcBytes);
    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(196, 66, 5, 4).expect("ok");
    assert_eq!(14, numDataBytes);
    assert_eq!(26, numEcBytes);

    // Version 40-H. (20 + 61) blocks.
    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(3706, 1276, 81, 0).expect("ok");
    assert_eq!(15, numDataBytes);
    assert_eq!(30, numEcBytes);

    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(3706, 1276, 81, 20).expect("ok");
    assert_eq!(16, numDataBytes);
    assert_eq!(30, numEcBytes);

    let (numDataBytes, numEcBytes) =
        encoder::getNumDataBytesAndNumECBytesForBlockID(3706, 1276, 81, 80).expect("ok");
    assert_eq!(16, numDataBytes);
    assert_eq!(30, numEcBytes);
}

#[test]
fn testInterleaveWithECBytes() {
    let dataBytes = &[32u32, 65, 205, 69, 41, 220, 46, 128, 236];
    let mut in_ = BitArray::new();
    for dataByte in dataBytes {
        // for (byte dataByte: dataBytes) {
        in_.appendBits(*dataByte, 8).expect("ok");
    }
    let out = encoder::interleaveWithECBytes(&in_, 26, 9, 1).expect("encode");
    let expected = &[
        // Data bytes.
        32, 65, 205, 69, 41, 220, 46, 128, 236, // Error correction bytes.
        42, 159, 74, 221, 244, 169, 239, 150, 138, 70, 237, 85, 224, 96, 74, 219, 61,
    ];
    assert_eq!(expected.len(), out.getSizeInBytes());
    let mut outArray = vec![0u8; expected.len()];
    out.toBytes(0, &mut outArray, 0, expected.len());
    // Can't use Arrays.equals(), because outArray may be longer than out.sizeInBytes()
    for x in 0..expected.len() {
        // for (int x = 0; x < expected.length; x++) {
        assert_eq!(expected[x], outArray[x]);
    }
    // Numbers are from http://www.swetake.com/qr/qr8.html
    let dataBytes = &[
        67u32, 70, 22, 38, 54, 70, 86, 102, 118, 134, 150, 166, 182, 198, 214, 230, 247, 7, 23, 39,
        55, 71, 87, 103, 119, 135, 151, 166, 22, 38, 54, 70, 86, 102, 118, 134, 150, 166, 182, 198,
        214, 230, 247, 7, 23, 39, 55, 71, 87, 103, 119, 135, 151, 160, 236, 17, 236, 17, 236, 17,
        236, 17,
    ];
    in_ = BitArray::new();
    for dataByte in dataBytes {
        // for (byte dataByte: dataBytes) {
        in_.appendBits(*dataByte, 8).expect("ok");
    }

    let out = encoder::interleaveWithECBytes(&in_, 134, 62, 4).expect("interleave ok");
    let expected = &[
        // Data bytes.
        67, 230, 54, 55, 70, 247, 70, 71, 22, 7, 86, 87, 38, 23, 102, 103, 54, 39, 118, 119, 70, 55,
        134, 135, 86, 71, 150, 151, 102, 87, 166, 160, 118, 103, 182, 236, 134, 119, 198, 17, 150,
        135, 214, 236, 166, 151, 230, 17, 182, 166, 247, 236, 198, 22, 7, 17, 214, 38, 23, 236, 39,
        17, // Error correction bytes.
        175, 155, 245, 236, 80, 146, 56, 74, 155, 165, 133, 142, 64, 183, 132, 13, 178, 54, 132,
        108, 45, 113, 53, 50, 214, 98, 193, 152, 233, 147, 50, 71, 65, 190, 82, 51, 209, 199, 171,
        54, 12, 112, 57, 113, 155, 117, 211, 164, 117, 30, 158, 225, 31, 190, 242, 38, 140, 61,
        179, 154, 214, 138, 147, 87, 27, 96, 77, 47, 187, 49, 156, 214,
    ];
    assert_eq!(expected.len(), out.getSizeInBytes());
    outArray = vec![0u8; expected.len()];
    out.toBytes(0, &mut outArray, 0, expected.len());
    for x in 0..expected.len() {
        // for (int x = 0; x < expected.length; x++) {
        assert_eq!(expected[x], outArray[x]);
    }
}

// fn  bytes(int... ints) -> Vec<u8>{
//   byte[] bytes = new byte[ints.length];
//   for (int i = 0; i < ints.length; i++) {
//     bytes[i] = (byte) ints[i];
//   }
//   return bytes;
// }

#[test]
fn testAppendNumericBytes() {
    // 1 = 01 = 0001 in 4 bits.
    let mut bits = BitArray::new();
    encoder::appendNumericBytes("1", &mut bits).expect("append");
    assert_eq!(" ...X", bits.to_string());
    // 12 = 0xc = 0001100 in 7 bits.
    let mut bits = BitArray::new();
    encoder::appendNumericBytes("12", &mut bits).expect("append");
    assert_eq!(" ...XX..", bits.to_string());
    // 123 = 0x7b = 0001111011 in 10 bits.
    let mut bits = BitArray::new();
    encoder::appendNumericBytes("123", &mut bits).expect("append");
    assert_eq!(" ...XXXX. XX", bits.to_string());
    // 1234 = "123" + "4" = 0001111011 + 0100
    let mut bits = BitArray::new();
    encoder::appendNumericBytes("1234", &mut bits).expect("append");
    assert_eq!(" ...XXXX. XX.X..", bits.to_string());
    // Empty.
    let mut bits = BitArray::new();
    encoder::appendNumericBytes("", &mut bits).expect("append");
    assert_eq!("", bits.to_string());
}

#[test]
fn testAppendAlphanumericBytes() {
    // A = 10 = 0xa = 001010 in 6 bits
    let mut bits = BitArray::new();
    encoder::appendAlphanumericBytes("A", &mut bits).expect("append");
    assert_eq!(" ..X.X.", bits.to_string());
    // AB = 10 * 45 + 11 = 461 = 0x1cd = 00111001101 in 11 bits
    let mut bits = BitArray::new();
    encoder::appendAlphanumericBytes("AB", &mut bits).expect("append");
    assert_eq!(" ..XXX..X X.X", bits.to_string());
    // ABC = "AB" + "C" = 00111001101 + 001100
    let mut bits = BitArray::new();
    encoder::appendAlphanumericBytes("ABC", &mut bits).expect("append");
    assert_eq!(" ..XXX..X X.X..XX. .", bits.to_string());
    // Empty.
    let mut bits = BitArray::new();
    encoder::appendAlphanumericBytes("", &mut bits).expect("append");
    assert_eq!("", bits.to_string());
    // Invalid data.
    // try {
    if encoder::appendAlphanumericBytes("abc", &mut BitArray::new()).is_ok() {
        panic!("should not be ok");
    }
    // } catch (WriterException we) {
    // good
    // }
}

#[test]
fn testAppend8BitBytes() {
    // 0x61, 0x62, 0x63
    let mut bits = BitArray::new();
    encoder::append8BitBytes("abc", &mut bits, encoder::DEFAULT_BYTE_MODE_ENCODING)
        .expect("append");
    assert_eq!(" .XX....X .XX...X. .XX...XX", bits.to_string());
    // Empty.
    let mut bits = BitArray::new();
    encoder::append8BitBytes("", &mut bits, encoder::DEFAULT_BYTE_MODE_ENCODING).expect("append");
    assert_eq!("", bits.to_string());
}

// Numbers are from page 21 of JISX0510:2004
#[test]
fn testAppendKanjiBytes() {
    let mut bits = BitArray::new();
    encoder::appendKanjiBytes(&shiftJISString(&[0x93, 0x5f]), &mut bits).expect("append");
    assert_eq!(" .XX.XX.. XXXXX", bits.to_string());
    encoder::appendKanjiBytes(&shiftJISString(&[0xe4, 0xaa]), &mut bits).expect("append");
    assert_eq!(" .XX.XX.. XXXXXXX. X.X.X.X. X.", bits.to_string());
}

// Numbers are from http://www.swetake.com/qr/qr3.html and
// http://www.swetake.com/qr/qr9.html
#[test]
fn testGenerateECBytes() {
    let dataBytes = &[32, 65, 205, 69, 41, 220, 46, 128, 236];
    let ecBytes = encoder::generateECBytes(dataBytes, 17);
    let expected = [
        42, 159, 74, 221, 244, 169, 239, 150, 138, 70, 237, 85, 224, 96, 74, 219, 61,
    ];
    assert_eq!(expected.len(), ecBytes.len());
    for x in 0..expected.len() {
        // for (int x = 0; x < expected.length; x++) {
        assert_eq!(expected[x], ecBytes[x] & 0xFF);
    }
    let dataBytes = &[
        67, 70, 22, 38, 54, 70, 86, 102, 118, 134, 150, 166, 182, 198, 214,
    ];
    let ecBytes = encoder::generateECBytes(dataBytes, 18);
    let expected = &[
        175, 80, 155, 64, 178, 45, 214, 233, 65, 209, 12, 155, 117, 31, 140, 214, 27, 187,
    ];
    assert_eq!(expected.len(), ecBytes.len());
    for x in 0..expected.len() {
        // for (int x = 0; x < expected.length; x++) {
        assert_eq!(expected[x], ecBytes[x] & 0xFF);
    }
    // High-order zero coefficient case.
    let dataBytes = &[32, 49, 205, 69, 42, 20, 0, 236, 17];
    let ecBytes = encoder::generateECBytes(dataBytes, 17);
    let expected = &[
        0, 3, 130, 179, 194, 0, 55, 211, 110, 79, 98, 72, 170, 96, 211, 137, 213,
    ];
    assert_eq!(expected.len(), ecBytes.len());
    for x in 0..expected.len() {
        // for (int x = 0; x < expected.length; x++) {
        assert_eq!(expected[x], ecBytes[x] & 0xFF);
    }
}

#[test]
fn testBugInBitVectorNumBytes() {
    // There was a bug in BitVector.sizeInBytes() that caused it to return a
    // smaller-by-one value (ex. 1465 instead of 1466) if the number of bits
    // in the vector is not 8-bit aligned.  In QRCodeEncoder::InitQRCode(),
    // BitVector::sizeInBytes() is used for finding the smallest QR Code
    // version that can fit the given data.  Hence there were corner cases
    // where we chose a wrong QR Code version that cannot fit the given
    // data.  Note that the issue did not occur with MODE_8BIT_BYTE, as the
    // bits in the bit vector are always 8-bit aligned.
    //
    // Before the bug was fixed, the following test didn't pass, because:
    //
    // - MODE_NUMERIC is chosen as all bytes in the data are '0'
    // - The 3518-byte numeric data needs 1466 bytes
    //   - 3518 / 3 * 10 + 7 = 11727 bits = 1465.875 bytes
    //   - 3 numeric bytes are encoded in 10 bits, hence the first
    //     3516 bytes are encoded in 3516 / 3 * 10 = 11720 bits.
    //   - 2 numeric bytes can be encoded in 7 bits, hence the last
    //     2 bytes are encoded in 7 bits.
    // - The version 27 QR Code with the EC level L has 1468 bytes for data.
    //   - 1828 - 360 = 1468
    // - In InitQRCode(), 3 bytes are reserved for a header.  Hence 1465 bytes
    //   (1468 -3) are left for data.
    // - Because of the bug in BitVector::sizeInBytes(), InitQRCode() determines
    //   the given data can fit in 1465 bytes, despite it needs 1466 bytes.
    // - Hence QRCodeencoder::encode() failed and returned false.
    //   - To be precise, it needs 11727 + 4 (getMode info) + 14 (length info) =
    //     11745 bits = 1468.125 bytes are needed (i.e. cannot fit in 1468
    //     bytes).
    let mut builder = String::with_capacity(3518);
    for _x in 0..3518 {
        // for (int x = 0; x < 3518; x++) {
        builder.push('0');
    }
    assert!(encoder::encode(&builder, ErrorCorrectionLevel::L).is_ok());
}

#[test]
fn testMinimalEncoder1() {
    verifyMinimalEncoding("A", "ALPHANUMERIC(A)", None, false);
}

#[test]
fn testMinimalEncoder2() {
    verifyMinimalEncoding("AB", "ALPHANUMERIC(AB)", None, false);
}

#[test]
fn testMinimalEncoder3() {
    verifyMinimalEncoding("ABC", "ALPHANUMERIC(ABC)", None, false);
}

#[test]
fn testMinimalEncoder4() {
    verifyMinimalEncoding("ABCD", "ALPHANUMERIC(ABCD)", None, false);
}

#[test]
fn testMinimalEncoder5() {
    verifyMinimalEncoding("ABCDE", "ALPHANUMERIC(ABCDE)", None, false);
}

#[test]
fn testMinimalEncoder6() {
    verifyMinimalEncoding("ABCDEF", "ALPHANUMERIC(ABCDEF)", None, false);
}

#[test]
fn testMinimalEncoder7() {
    verifyMinimalEncoding("ABCDEFG", "ALPHANUMERIC(ABCDEFG)", None, false);
}

#[test]
fn testMinimalEncoder8() {
    verifyMinimalEncoding("1", "NUMERIC(1)", None, false);
}

#[test]
fn testMinimalEncoder9() {
    verifyMinimalEncoding("12", "NUMERIC(12)", None, false);
}

#[test]
fn testMinimalEncoder10() {
    verifyMinimalEncoding("123", "NUMERIC(123)", None, false);
}

#[test]
fn testMinimalEncoder11() {
    verifyMinimalEncoding("1234", "NUMERIC(1234)", None, false);
}

#[test]
fn testMinimalEncoder12() {
    verifyMinimalEncoding("12345", "NUMERIC(12345)", None, false);
}

#[test]
fn testMinimalEncoder13() {
    verifyMinimalEncoding("123456", "NUMERIC(123456)", None, false);
}

#[test]
fn testMinimalEncoder14() {
    verifyMinimalEncoding("123A", "ALPHANUMERIC(123A)", None, false);
}

#[test]
fn testMinimalEncoder15() {
    verifyMinimalEncoding("A1", "ALPHANUMERIC(A1)", None, false);
}

#[test]
fn testMinimalEncoder16() {
    verifyMinimalEncoding("A12", "ALPHANUMERIC(A12)", None, false);
}

#[test]
fn testMinimalEncoder17() {
    verifyMinimalEncoding("A123", "ALPHANUMERIC(A123)", None, false);
}

#[test]
fn testMinimalEncoder18() {
    verifyMinimalEncoding("A1234", "ALPHANUMERIC(A1234)", None, false);
}

#[test]
fn testMinimalEncoder19() {
    verifyMinimalEncoding("A12345", "ALPHANUMERIC(A12345)", None, false);
}

#[test]
fn testMinimalEncoder20() {
    verifyMinimalEncoding("A123456", "ALPHANUMERIC(A123456)", None, false);
}

#[test]
fn testMinimalEncoder21() {
    verifyMinimalEncoding("A1234567", "ALPHANUMERIC(A1234567)", None, false);
}

#[test]
fn testMinimalEncoder22() {
    verifyMinimalEncoding("A12345678", "BYTE(A),NUMERIC(12345678)", None, false);
}

#[test]
fn testMinimalEncoder23() {
    verifyMinimalEncoding("A123456789", "BYTE(A),NUMERIC(123456789)", None, false);
}

#[test]
fn testMinimalEncoder24() {
    verifyMinimalEncoding(
        "A1234567890",
        "ALPHANUMERIC(A1),NUMERIC(234567890)",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder25() {
    verifyMinimalEncoding("AB1", "ALPHANUMERIC(AB1)", None, false);
}

#[test]
fn testMinimalEncoder26() {
    verifyMinimalEncoding("AB12", "ALPHANUMERIC(AB12)", None, false);
}

#[test]
fn testMinimalEncoder27() {
    verifyMinimalEncoding("AB123", "ALPHANUMERIC(AB123)", None, false);
}

#[test]
fn testMinimalEncoder28() {
    verifyMinimalEncoding("AB1234", "ALPHANUMERIC(AB1234)", None, false);
}

#[test]
fn testMinimalEncoder29() {
    verifyMinimalEncoding("ABC1", "ALPHANUMERIC(ABC1)", None, false);
}

#[test]
fn testMinimalEncoder30() {
    verifyMinimalEncoding("ABC12", "ALPHANUMERIC(ABC12)", None, false);
}

#[test]
fn testMinimalEncoder31() {
    verifyMinimalEncoding("ABC1234", "ALPHANUMERIC(ABC1234)", None, false);
}

#[test]
fn testMinimalEncoder32() {
    verifyMinimalEncoding(
        "http://foo.com",
        "BYTE(http://foo.com)\
        ",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder33() {
    verifyMinimalEncoding(
        "HTTP://FOO.COM",
        "ALPHANUMERIC(HTTP://FOO.COM\
        )",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder34() {
    verifyMinimalEncoding(
        "1001114670010%01201220%107211220%140045003267781",
        "NUMERIC(1001114670010),ALPHANUMERIC(%01201220%107211220%),NUMERIC(140045003267781)",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder35() {
    verifyMinimalEncoding("\u{0150}", "ECI(iso-8859-2),BYTE(.)", None, false);
}

#[test]
fn testMinimalEncoder36() {
    verifyMinimalEncoding("\u{015C}", "ECI(iso-8859-3),BYTE(.)", None, false);
}

#[test]
fn testMinimalEncoder37() {
    verifyMinimalEncoding("\u{0150}\u{015C}", "ECI(utf-8),BYTE(..)", None, false);
}

#[test]
fn testMinimalEncoder38() {
    verifyMinimalEncoding(
        "\u{0150}\u{0150}\u{015C}\u{015C}",
        "ECI(iso-8859-2),BYTE(.\
        .),ECI(iso-8859-3),BYTE(..)",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder39() {
    verifyMinimalEncoding(
        "abcdef\u{0150}ghij",
        "ECI(iso-8859-2),BYTE(abcde\
        f.ghij)",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder40() {
    verifyMinimalEncoding(
        "2938928329832983\u{0150}2938928329832983\u{015C}2938928329832983",
        "NUMERIC(2938928329832983),ECI(iso-8859-2),BYTE(.),NUMERIC(2938928329832983),ECI(iso-8\
        859-3),BYTE(.),NUMERIC(2938928329832983)",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder41() {
    verifyMinimalEncoding(
        "1001114670010%01201220%107211220%140045003267781",
        "FNC1_FIRST_POSITION(),NUMERIC(100111\
        4670010),ALPHANUMERIC(%01201220%107211220%),NUMERIC(140045003267781)",
        None,
        true,
    );
}

#[test]
fn testMinimalEncoder42() {
    // test halfwidth Katakana character (they are single byte encoded in Shift_JIS)
    // NOTE: Changed to windows-31j because that is what is supported in encoding crate
    verifyMinimalEncoding(
        "Katakana:\u{FF66}\u{FF66}\u{FF66}\u{FF66}\u{FF66}\u{FF66}",
        "ECI(windows-31j),BYTE(Katakana:......)",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder43() {
    // The character \u30A2 encodes as double byte in Shift_JIS so KANJI is more compact in this case
    verifyMinimalEncoding(
        "Katakana:\u{30A2}\u{30A2}\u{30A2}\u{30A2}\u{30A2}\u{30A2}",
        "BYTE(Katakana:),KANJI(......)",
        None,
        false,
    );
}

#[test]
fn testMinimalEncoder44() {
    // The character \u30A2 encodes as double byte in Shift_JIS but KANJI is not more compact in this case because
    // KANJI is only more compact when it encodes pairs of characters. In the case of mixed text it can however be
    // that Shift_JIS encoding is more compact as in this example
    // NOTE: Changed to windows-31j because that is what is supported in encoding crate
    verifyMinimalEncoding(
        "Katakana:\u{30A2}a\u{30A2}a\u{30A2}a\u{30A2}a\u{30A2}a\u{30A2}",
        "ECI(windows-31j),BYTE(Katakana:.a.a.a.a.a.)",
        None,
        false,
    );
}

fn verifyMinimalEncoding(
    input: &str,
    expectedRXingResult: &str,
    priorityCharset: Option<EncodingRef>,
    isGS1: bool,
) {
    let result = MinimalEncoder::encode_with_details(
        input,
        None,
        priorityCharset,
        isGS1,
        ErrorCorrectionLevel::L,
    )
    .expect("encode");
    assert_eq!(result.to_string(), expectedRXingResult);
}

fn verifyGS1EncodedData(qrCode: &QRCode) {
    let expected = r"<<
 mode: ALPHANUMERIC
 ecLevel: H
 version: 2
 maskPattern: 2
 matrix:
 1 1 1 1 1 1 1 0 1 0 1 1 1 1 0 1 1 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 1 0 0 0 0 1 1 0 1 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 1 0 1 1 0 1 1 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 1 1 0 1 0 1 1 1 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 1 1 1 1 1 1 1 1 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 1 0 0 1 1 1 0 0 0 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 1 1 1 0 0 0 1 1 1 0 0 0 0 0 0 0 0
 0 0 1 1 1 0 1 0 1 1 1 1 0 1 1 0 1 1 1 1 0 0 1 1 1
 0 0 0 1 1 1 0 1 0 0 1 0 0 1 0 0 1 1 1 0 0 1 0 0 1
 1 0 1 1 0 0 1 0 1 1 0 0 0 0 1 0 1 1 1 0 0 1 0 0 1
 0 0 1 1 0 1 0 1 1 1 1 0 0 1 1 1 1 0 0 0 1 1 0 1 1
 0 0 1 0 0 0 1 0 0 0 1 1 0 1 0 0 0 1 0 1 1 1 0 1 0
 1 1 1 0 1 1 0 1 0 0 0 0 0 0 0 1 1 0 1 1 0 1 0 0 0
 1 0 1 0 1 0 1 1 0 1 0 1 0 1 1 0 0 0 0 0 1 1 0 0 1
 1 0 0 1 0 1 0 1 0 0 0 1 1 1 1 0 1 0 1 0 0 1 0 0 1
 1 0 1 0 0 1 1 1 0 1 1 0 0 1 0 0 1 1 1 1 1 1 0 0 0
 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 0 1 0 0 0 1 0 0 1 0
 1 1 1 1 1 1 1 0 0 0 0 1 0 0 1 1 1 0 1 0 1 0 1 1 1
 1 0 0 0 0 0 1 0 0 1 1 1 1 1 0 1 1 0 0 0 1 0 0 0 1
 1 0 1 1 1 0 1 0 1 0 1 0 0 1 1 1 1 1 1 1 1 0 0 0 1
 1 0 1 1 1 0 1 0 1 1 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0
 1 0 1 1 1 0 1 0 1 0 0 0 1 1 0 1 0 0 1 1 1 0 1 0 1
 1 0 0 0 0 0 1 0 0 1 0 1 0 1 1 1 0 1 0 0 1 1 1 1 1
 1 1 1 1 1 1 1 0 0 1 1 0 0 1 1 0 1 0 0 0 0 1 0 1 1
>>
";
    assert_eq!(expected, qrCode.to_string());
}

fn verifyNotGS1EncodedData(qrCode: &QRCode) {
    let expected = r"<<
 mode: ALPHANUMERIC
 ecLevel: H
 version: 1
 maskPattern: 0
 matrix:
 1 1 1 1 1 1 1 0 1 1 1 1 0 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 0 1 1 1 0 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 1 0 1 1 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 1 1 0 1 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 1 1 1 0 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 1 0 0 0 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0
 0 0 1 0 1 1 1 0 1 1 0 0 1 1 0 0 0 1 0 0 1
 1 0 1 1 1 0 0 1 0 0 0 1 0 1 0 0 0 0 0 0 0
 0 0 1 1 0 0 1 0 1 0 0 0 1 0 1 0 1 0 1 1 0
 1 1 0 1 0 1 0 1 1 1 0 1 0 1 0 0 0 0 0 1 0
 0 0 1 1 0 1 1 1 1 0 0 0 1 0 1 0 1 1 1 1 0
 0 0 0 0 0 0 0 0 1 0 0 1 1 1 0 1 0 1 0 0 0
 1 1 1 1 1 1 1 0 0 0 1 0 1 0 1 1 0 0 0 0 1
 1 0 0 0 0 0 1 0 1 1 1 1 0 1 0 1 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 0 1 1 0 1 0 1 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 1 1 0 1 1 1 1 0 1 0 1 0
 1 0 1 1 1 0 1 0 1 0 0 0 1 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 1 1 0 1 1 0 1 0 0 0 1 1
 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 1 0 1 0 1
>>
";
    assert_eq!(expected, qrCode.to_string());
}

fn shiftJISString(bytes: &[u8]) -> String {
    SHIFT_JIS_CHARSET
        .decode(bytes, encoding::DecoderTrap::Strict)
        .expect("decode should be ok")
    // return new String(bytes, StringUtils.SHIFT_JIS_CHARSET);
}
