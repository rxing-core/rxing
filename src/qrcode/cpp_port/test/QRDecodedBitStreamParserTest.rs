/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2008 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

// #include "BitArray.h"
// #include "ByteArray.h"
// #include "DecoderResult.h"
// #include "qrcode/QRDataMask.h"
// #include "qrcode/QRDecoder.h"
// #include "qrcode/QRErrorCorrectionLevel.h"
// #include "qrcode/QRVersion.h"

// #include "gtest/gtest.h"

// namespace ZXing {
// 	namespace QRCode {
// 		DecoderResult DecodeBitStream(ByteArray&& bytes, const Version& version, ErrorCorrectionLevel ecLevel);
// 	}
// }

// using namespace ZXing;
// using namespace ZXing::QRCode;

use crate::{
    common::BitArray,
    qrcode::{
        cpp_port::decoder::DecodeBitStream,
        decoder::{ErrorCorrectionLevel, Version},
    },
};

#[test]
fn SimpleByteMode() {
    let mut ba = BitArray::new();
    ba.appendBits(0x04, 4).unwrap(); // Byte mode
    ba.appendBits(0x03, 8).unwrap(); // 3 bytes
    ba.appendBits(0xF1, 8).unwrap();
    ba.appendBits(0xF2, 8).unwrap();
    ba.appendBits(0xF3, 8).unwrap();
    let bytes: Vec<u8> = ba.into();
    let result = DecodeBitStream(
        &bytes,
        Version::Model2(1).expect("find_version"),
        ErrorCorrectionLevel::M,
    )
    .expect("Decode")
    .content()
    .to_string();
    let str = String::from_utf16(&[0xF1, 0xF2, 0xF3]).unwrap();
    assert_eq!(str, result);
}

#[test]
fn SimpleSJIS() {
    let mut ba = BitArray::new();
    ba.appendBits(0x04, 4).expect("append"); // Byte mode
    ba.appendBits(0x04, 8).expect("append"); // 4 bytes
    ba.appendBits(0xA1, 8).expect("append");
    ba.appendBits(0xA2, 8).expect("append");
    ba.appendBits(0xA3, 8).expect("append");
    ba.appendBits(0xD0, 8).expect("append");
    let bytes: Vec<u8> = ba.into();
    let result = DecodeBitStream(&bytes, Version::Model2(1).unwrap(), ErrorCorrectionLevel::M)
        .unwrap()
        .content()
        .to_string();
    assert_eq!("\u{ff61}\u{ff62}\u{ff63}\u{ff90}", result);
}

#[test]
fn ECI() {
    let mut ba = BitArray::new();
    ba.appendBits(0x07, 4).expect("append"); // ECI mode
    ba.appendBits(0x02, 8).expect("append"); // ECI 2 = CP437 encoding
    ba.appendBits(0x04, 4).expect("append"); // Byte mode
    ba.appendBits(0x03, 8).expect("append"); // 3 bytes
    ba.appendBits(0xA1, 8).expect("append");
    ba.appendBits(0xA2, 8).expect("append");
    ba.appendBits(0xA3, 8).expect("append");
    let bytes: Vec<u8> = ba.into();
    let result = DecodeBitStream(&bytes, Version::Model2(1).unwrap(), ErrorCorrectionLevel::M)
        .unwrap()
        .content()
        .to_string();
    assert_eq!("\u{ED}\u{F3}\u{FA}", result);
}

#[test]
fn Hanzi() {
    let mut ba = BitArray::new();
    ba.appendBits(0x0D, 4).expect("append"); // Hanzi mode
    ba.appendBits(0x01, 4).expect("append"); // Subset 1 = GB2312 encoding
    ba.appendBits(0x01, 8).expect("append"); // 1 characters
    ba.appendBits(0x03C1, 13).expect("append");
    let bytes: Vec<u8> = ba.into();
    let result = DecodeBitStream(&bytes, Version::Model2(1).unwrap(), ErrorCorrectionLevel::M)
        .unwrap()
        .content()
        .to_string();
    assert_eq!("\u{963f}", result);
}

#[test]
fn HanziLevel1() {
    let mut ba = BitArray::new();
    ba.appendBits(0x0D, 4).expect("append"); // Hanzi mode
    ba.appendBits(0x01, 4).expect("append"); // Subset 1 = GB2312 encoding
    ba.appendBits(0x01, 8).expect("append"); // 1 characters
                                             // A5A2 (U+30A2) => A5A2 - A1A1 = 401, 4*60 + 01 = 0181
    ba.appendBits(0x0181, 13).expect("append");

    let bytes: Vec<u8> = ba.into();

    let result = DecodeBitStream(&bytes, Version::Model2(1).unwrap(), ErrorCorrectionLevel::M)
        .unwrap()
        .content()
        .to_string();
    assert_eq!("\u{30a2}", result);
}

#[test]
fn SymbologyIdentifier() {
    let version = Version::Model2(1).unwrap();
    let ecLevel = ErrorCorrectionLevel::M;

    // Plain "ANUM(1) A"
    let result = DecodeBitStream(&[0x20, 0x09, 0x40], version, ecLevel).unwrap();
    assert_eq!(result.symbologyIdentifier(), "]Q1");
    assert_eq!(result.text(), "A");

    // GS1 "FNC1(1st) NUM(4) 2001"
    let result = DecodeBitStream(&[0x51, 0x01, 0x0C, 0x81, 0x00], version, ecLevel).unwrap();
    assert_eq!(result.symbologyIdentifier(), "]Q3");
    assert_eq!(result.text(), "2001"); // "(20)01"

    // GS1 "NUM(4) 2001 FNC1(1st) 301" - FNC1(1st) can occur anywhere (this actually violates the specification)
    let result = DecodeBitStream(
        &[0x10, 0x10, 0xC8, 0x15, 0x10, 0x0D, 0x2D, 0x00],
        version,
        ecLevel,
    )
    .unwrap();
    assert_eq!(result.symbologyIdentifier(), "]Q3");
    assert_eq!(result.text(), "2001301"); // "(20)01(30)1"

    // AIM "FNC1(2nd) 99 (0x63) ANUM(1) A"
    let result = DecodeBitStream(&[0x96, 0x32, 0x00, 0x94, 0x00], version, ecLevel).unwrap();
    assert_eq!(result.symbologyIdentifier(), "]Q5");
    assert_eq!(result.text(), "99A");

    // AIM "BYTE(1) A FNC1(2nd) 99 (0x63) BYTE(1) B" - FNC1(2nd) can occur anywhere
    // Disabled this test, since this violates the specification and the code does support it anymore
    //	result = DecodeBitStream({0x40, 0x14, 0x19, 0x63, 0x40, 0x14, 0x20, 0x00}, version, ecLevel, "");
    //	assert_eq!(result.symbologyIdentifier(), "]Q5");
    //	assert_eq!(result.text(), L"99AB"); // Application Indicator prefixed to data

    // AIM "FNC1(2nd) A (100 + 61 = 0xA5) ANUM(1) B"
    let result = DecodeBitStream(&[0x9A, 0x52, 0x00, 0x96, 0x00], version, ecLevel).unwrap();
    assert_eq!(result.symbologyIdentifier(), "]Q5");
    assert_eq!(result.text(), "AB");

    // AIM "FNC1(2nd) a (100 + 97 = 0xC5) ANUM(1) B"
    let result = DecodeBitStream(&[0x9C, 0x52, 0x00, 0x96, 0x00], version, ecLevel).unwrap();
    assert_eq!(result.symbologyIdentifier(), "]Q5");
    assert_eq!(result.text(), "aB");

    // Bad AIM Application Indicator "FNC1(2nd) @ (0xA4) ANUM(1) B"
    let result = DecodeBitStream(&[0x9A, 0x42, 0x00, 0x96, 0x00], version, ecLevel).unwrap();
    assert!(!result.isValid());
}
