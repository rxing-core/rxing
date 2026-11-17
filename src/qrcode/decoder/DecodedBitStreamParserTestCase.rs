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

fn decode_text(builder: &mut BitSourceBuilder) -> crate::common::Result<String> {
    Ok(decoded_bit_stream_parser::decode(
        builder.asByteArray(),
        Version::getVersionForNumber(1).expect("unwrap"),
        ErrorCorrectionLevel::H,
        &DecodeHints::default(),
    )?
    .getText()
    .to_owned())
}

fn write_alphanumeric(builder: &mut BitSourceBuilder, values: &[u32]) {
    builder.write(0x02, 4); // Alphanumeric mode
    builder.write(values.len() as u32, 9);
    for pair in values.chunks(2) {
        match pair {
            [a, b] => builder.write(a * 45 + b, 11),
            [a] => builder.write(*a, 6),
            _ => unreachable!(),
        }
    }
}

#[test]
fn testFNC1AlphanumericDoublePercent() {
    // 5 % 6 % % A -> "5<GS>6%A"
    let mut builder = BitSourceBuilder::new();
    builder.write(0x05, 4); // FNC1 first position
    write_alphanumeric(&mut builder, &[5, 38, 6, 38, 38, 10]);
    assert_eq!("5\u{1D}6%A", decode_text(&mut builder).expect("decode"));

    // %%% -> "%<GS>", %%%% -> "%%"
    let mut builder = BitSourceBuilder::new();
    builder.write(0x05, 4);
    write_alphanumeric(&mut builder, &[38, 38, 38]);
    assert_eq!("%\u{1D}", decode_text(&mut builder).expect("decode"));

    let mut builder = BitSourceBuilder::new();
    builder.write(0x05, 4);
    write_alphanumeric(&mut builder, &[38, 38, 38, 38]);
    assert_eq!("%%", decode_text(&mut builder).expect("decode"));
}

#[test]
fn testFNC1AlphanumericTrailingPercentAfterOtherSegment() {
    // Byte "XY" then alphanumeric "%": the lone % must become <GS>.
    let mut builder = BitSourceBuilder::new();
    builder.write(0x05, 4); // FNC1 first position
    builder.write(0x04, 4); // Byte mode
    builder.write(0x02, 8);
    builder.write(b'X' as u32, 8);
    builder.write(b'Y' as u32, 8);
    write_alphanumeric(&mut builder, &[38]);
    assert_eq!("XY\u{1D}", decode_text(&mut builder).expect("decode"));
}
