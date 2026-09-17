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

use crate::{
    DecodeHints,
    common::BitSourceBuilder,
    qrcode::common::{ErrorCorrectionLevel, Version},
    qrcode::decoder::decoded_bit_stream_parser,
};

/**
 * Tests {@link DecodedBitStreamParser}.
 *
 * @author Sean Owen
 */

#[test]
fn testSimpleByteMode() {
    let mut builder = BitSourceBuilder::new();
    builder.write(0x04, 4); // Byte mode
    builder.write(0x03, 8); // 3 bytes
    builder.write(0xF1, 8);
    builder.write(0xF2, 8);
    builder.write(0xF3, 8);
    let result = decoded_bit_stream_parser::decode(
        builder.asByteArray(),
        Version::getVersionForNumber(1).expect("unwrap"),
        ErrorCorrectionLevel::H,
        &DecodeHints::default(),
    )
    .expect("unwrap")
    .getText()
    .to_string();
    assert_eq!("\u{00f1}\u{00f2}\u{00f3}", result);
}

#[test]
fn testSimpleSJIS() {
    let mut builder = BitSourceBuilder::new();
    builder.write(0x04, 4); // Byte mode
    builder.write(0x04, 8); // 4 bytes
    builder.write(0xA1, 8);
    builder.write(0xA2, 8);
    builder.write(0xA3, 8);
    builder.write(0xD0, 8);
    let result = decoded_bit_stream_parser::decode(
        builder.asByteArray(),
        Version::getVersionForNumber(1).expect("unwrap"),
        ErrorCorrectionLevel::H,
        &DecodeHints::default(),
    )
    .expect("unwrap")
    .getText()
    .to_owned();
    assert_eq!("\u{ff61}\u{ff62}\u{ff63}\u{ff90}", result);
}

#[test]
fn testECI() {
    let mut builder = BitSourceBuilder::new();

    builder.write(0x07, 4); // ECI mode
    builder.write(0x02, 8); // ECI 2 = CP437 encoding
    builder.write(0x04, 4); // Byte mode
    builder.write(0x03, 8); // 3 bytes
    builder.write(0xA1, 8);
    builder.write(0xA2, 8);
    builder.write(0xA3, 8);
    let result = decoded_bit_stream_parser::decode(
        builder.asByteArray(),
        Version::getVersionForNumber(1).expect("unwrap"),
        ErrorCorrectionLevel::H,
        &DecodeHints::default(),
    )
    .expect("unwrap")
    .getText()
    .to_owned();
    assert_eq!("\u{00ed}\u{00f3}\u{00fa}", result);
}

#[test]
fn testHanzi() {
    let mut builder = BitSourceBuilder::new();

    builder.write(0x0D, 4); // Hanzi mode
    builder.write(0x01, 4); // Subset 1 = GB2312 encoding
    builder.write(0x01, 8); // 1 characters
    builder.write(0x03C1, 13);
    let result = decoded_bit_stream_parser::decode(
        builder.asByteArray(),
        Version::getVersionForNumber(1).expect("unwrap"),
        ErrorCorrectionLevel::H,
        &DecodeHints::default(),
    )
    .expect("unwrap")
    .getText()
    .to_owned();
    assert_eq!("\u{963f}", result);
}

#[test]
fn testHanziLevel1() {
    let mut builder = BitSourceBuilder::new();

    builder.write(0x0D, 4); // Hanzi mode
    builder.write(0x01, 4); // Subset 1 = GB2312 encoding
    builder.write(0x01, 8); // 1 characters
    // A5A2 (U+30A2) => A5A2 - A1A1 = 401, 4*60 + 01 = 0181
    builder.write(0x0181, 13);
    let result = decoded_bit_stream_parser::decode(
        builder.asByteArray(),
        Version::getVersionForNumber(1).expect("unwrap"),
        ErrorCorrectionLevel::H,
        &DecodeHints::default(),
    )
    .expect("unwrap")
    .getText()
    .to_owned();
    assert_eq!("\u{30a2}", result);
}

// TODO definitely need more tests here

// Regression tests for FNC1 '%'/'%%' handling in alphanumeric mode (issue #102).
// See ISO/IEC 18004 7.4.8.1-7.4.8.2: a single '%' is the GS1 separator (0x1D);
// a doubled '%%' is one literal '%'. Percent signs are paired left to right.

const AN: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";

fn idx(c: char) -> u32 {
    AN.find(c).unwrap() as u32
}

fn writeAlphanumericSegment(b: &mut BitSourceBuilder, s: &str) {
    b.write(0x02, 4); // alphanumeric mode
    b.write(s.len() as u32, 9); // char count (versions 1-9)
    let c: Vec<char> = s.chars().collect();
    for p in c.chunks(2) {
        if p.len() == 2 {
            b.write(idx(p[0]) * 45 + idx(p[1]), 11);
        } else {
            b.write(idx(p[0]), 6);
        }
    }
}

fn fnc1FirstAlnum(prefixNumeric: Option<&str>, s: &str) -> crate::common::Result<String> {
    let mut b = BitSourceBuilder::new();
    b.write(0x05, 4); // FNC1 first position
    if let Some(n) = prefixNumeric {
        // 2-digit numeric segment
        b.write(0x01, 4);
        b.write(2, 10);
        b.write(n.parse::<u32>().unwrap(), 7);
    }
    writeAlphanumericSegment(&mut b, s);
    b.write(0, 4); // terminator
    decoded_bit_stream_parser::decode(
        b.asByteArray(),
        Version::getVersionForNumber(2).unwrap(),
        ErrorCorrectionLevel::L,
        &DecodeHints::default(),
    )
    .map(|r| r.getText().to_string())
}

fn fnc1SecondAlnum(appIndicator: u32, s: &str) -> crate::common::Result<String> {
    let mut b = BitSourceBuilder::new();
    b.write(0x09, 4); // FNC1 second position
    b.write(appIndicator, 8); // AIM application indicator
    writeAlphanumericSegment(&mut b, s);
    b.write(0, 4); // terminator
    decoded_bit_stream_parser::decode(
        b.asByteArray(),
        Version::getVersionForNumber(2).unwrap(),
        ErrorCorrectionLevel::L,
        &DecodeHints::default(),
    )
    .map(|r| r.getText().to_string())
}

fn plainAlnum(s: &str) -> crate::common::Result<String> {
    let mut b = BitSourceBuilder::new();
    writeAlphanumericSegment(&mut b, s);
    b.write(0, 4); // terminator
    decoded_bit_stream_parser::decode(
        b.asByteArray(),
        Version::getVersionForNumber(2).unwrap(),
        ErrorCorrectionLevel::L,
        &DecodeHints::default(),
    )
    .map(|r| r.getText().to_string())
}

#[test]
fn testFnc1AlphanumericNoPrefixSinglePercent() {
    assert_eq!("A\u{1D}B", fnc1FirstAlnum(None, "A%B").expect("decode"));
}

#[test]
fn testFnc1AlphanumericNoPrefixDoublePercent() {
    assert_eq!("A%B", fnc1FirstAlnum(None, "A%%B").expect("decode"));
}

#[test]
fn testFnc1AlphanumericNumericPrefixSinglePercent() {
    assert_eq!(
        "01A\u{1D}B",
        fnc1FirstAlnum(Some("01"), "A%B").expect("decode")
    );
}

#[test]
fn testFnc1AlphanumericNumericPrefixDoublePercent() {
    assert_eq!("01A%B", fnc1FirstAlnum(Some("01"), "A%%B").expect("decode"));
}

#[test]
fn testFnc1AlphanumericTrailingDoublePercent() {
    assert_eq!("01AB%", fnc1FirstAlnum(Some("01"), "AB%%").expect("decode"));
}

#[test]
fn testFnc1AlphanumericLeadingDoublePercent() {
    assert_eq!("01%AB", fnc1FirstAlnum(Some("01"), "%%AB").expect("decode"));
}

#[test]
fn testFnc1AlphanumericFourPercent() {
    assert_eq!(
        "01A%%B",
        fnc1FirstAlnum(Some("01"), "A%%%%B").expect("decode")
    );
}

#[test]
fn testFnc1AlphanumericDigitsThenDoublePercent() {
    assert_eq!(
        "01100%",
        fnc1FirstAlnum(Some("01"), "100%%").expect("decode")
    );
}

#[test]
fn testFnc1AlphanumericTriplePercent() {
    // %%% -> paired left to right: (%,%) -> '%', then lone '%' -> GS
    assert_eq!("%\u{1D}", fnc1FirstAlnum(None, "%%%").expect("decode"));
}

#[test]
fn testFnc1AlphanumericLoneTrailingPercent() {
    assert_eq!("A\u{1D}", fnc1FirstAlnum(None, "A%").expect("decode"));
}

#[test]
fn testFnc1AlphanumericOddLengthSinglePercent() {
    // Single '%' is odd-length, so it goes through the 6-bit decode path.
    assert_eq!("\u{1D}", fnc1FirstAlnum(None, "%").expect("decode"));
}

#[test]
fn testFnc1AlphanumericOddLengthPairThenPercent() {
    // "AB%" is odd-length: 'A','B' decode as a pair, '%' via the 6-bit path.
    assert_eq!("AB\u{1D}", fnc1FirstAlnum(None, "AB%").expect("decode"));
}

#[test]
fn testFnc1SecondPositionAlphanumericPercent() {
    assert_eq!("01A\u{1D}B", fnc1SecondAlnum(1, "A%B").expect("decode"));
}

#[test]
fn testAlphanumericPercentWithoutFnc1IsUnchanged() {
    assert_eq!("A%B", plainAlnum("A%B").expect("decode"));
    assert_eq!("A%%B", plainAlnum("A%%B").expect("decode"));
}
