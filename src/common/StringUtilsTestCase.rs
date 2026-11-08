/*
 * Copyright 2012 ZXing authors
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

// package com.google.zxing.common;

// import org.junit.Assert;
// import org.junit.Test;

// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;
// import java.util.Random;

use rand::RngExt;

use crate::{DecodeHintValue, DecodeHints, common::string_utils};

use super::CharacterSet;

#[test]
fn test_random() {
    let mut r = rand::rng();
    let mut bytes: Vec<u8> = vec![0; 1000];
    bytes.fill_with(|| r.random());
    // for byte in &mut bytes {
    //     *byte = r.gen();
    // }
    // Neither UTF-8 nor Shift_JIS, so the ISO/IEC 18004 default
    assert_eq!(
        CharacterSet::ISO8859_1,
        string_utils::guessCharset(&bytes, &DecodeHints::default()).unwrap()
    );
}

#[test]
fn test_short_shift_jis1() {
    // 金魚
    do_test(&[0x8b, 0xe0, 0x8b, 0x9b], CharacterSet::Shift_JIS, "SJIS");
}

#[test]
fn test_short_iso885911() {
    // båd
    do_test(&[0x62, 0xe5, 0x64], CharacterSet::ISO8859_1, "ISO8859_1");
}

#[test]
fn test_short_utf8() {
    // Español
    do_test(
        &[0x45, 0x73, 0x70, 0x61, 0xc3, 0xb1, 0x6f, 0x6c],
        CharacterSet::UTF8,
        "UTF8",
    );
}

#[test]
fn test_mixed_shift_jis1() {
    // Hello 金!
    do_test(
        &[0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x8b, 0xe0, 0x21],
        CharacterSet::Shift_JIS,
        "SJIS",
    );
}

#[test]
fn test_utf16_be() {
    // 调压柜
    do_test(
        &[0xFE, 0xFF, 0x8c, 0x03, 0x53, 0x8b, 0x67, 0xdc],
        CharacterSet::UTF16BE,
        CharacterSet::UTF16BE.get_charset_name(),
    );
}

#[test]
fn test_utf16_le() {
    // 调压柜
    do_test(
        &[0xFF, 0xFE, 0x03, 0x8c, 0x8b, 0x53, 0xdc, 0x67],
        CharacterSet::UTF16LE,
        CharacterSet::UTF16LE.get_charset_name(),
    );
}

#[test]
fn test_iso88591_symbols() {
    // Größe ±5%
    do_test(
        &[0x47, 0x72, 0xf6, 0xdf, 0x65, 0x20, 0xb1, 0x35, 0x25],
        CharacterSet::ISO8859_1,
        "ISO8859_1",
    );
}

#[test]
fn test_iso88591_guillemets() {
    // «Bonjour»
    do_test(
        &[0xab, 0x42, 0x6f, 0x6e, 0x6a, 0x6f, 0x75, 0x72, 0xbb],
        CharacterSet::ISO8859_1,
        "ISO8859_1",
    );
}

#[test]
fn test_iso88591_superscript() {
    // 3 m²
    do_test(
        &[0x33, 0x20, 0x6d, 0xb2],
        CharacterSet::ISO8859_1,
        "ISO8859_1",
    );
}

#[test]
fn test_shift_jis_ideographic_space() {
    // 東京　駅
    do_test(
        &[0x93, 0x8c, 0x8b, 0x9e, 0x81, 0x40, 0x89, 0x77],
        CharacterSet::Shift_JIS,
        "SJIS",
    );
}

#[test]
fn test_shift_jis_half_width_katakana_and_kanji() {
    // ｺｰﾋｰ 120円
    do_test(
        &[0xba, 0xb0, 0xcb, 0xb0, 0x20, 0x31, 0x32, 0x30, 0x89, 0x7e],
        CharacterSet::Shift_JIS,
        "SJIS",
    );
}

#[test]
fn test_shift_jis_half_width_katakana() {
    // ﾃｽﾄ: 3 half-width katakana in a row
    do_test(&[0xc3, 0xbd, 0xc4], CharacterSet::Shift_JIS, "SJIS");
}

#[test]
fn test_two_half_width_katakana() {
    // ｱｲ, or ±² in ISO-8859-1: fewer than 3 half-width katakana are no evidence of Shift_JIS
    let bytes = [0xb1, 0xb2];
    do_test(&bytes, CharacterSet::ISO8859_1, "ISO8859_1");
    let hints = DecodeHints::default().with(DecodeHintValue::CharacterSet("Shift_JIS".to_owned()));
    assert_eq!(
        Some(CharacterSet::Shift_JIS),
        string_utils::guessCharset(&bytes, &hints)
    );
}

#[test]
fn test_shift_jis_double_byte_run() {
    // 鰊饂飩: 3 double-byte characters in a row, none with a byte in 0x80-0x9F
    do_test(
        &[0xe9, 0xd7, 0xe9, 0x5a, 0xe9, 0x4a],
        CharacterSet::Shift_JIS,
        "SJIS",
    );
}

#[test]
fn test_shift_jis_unfinished_double_byte() {
    // A lead byte at the end: not valid Shift_JIS
    do_test(&[0x81], CharacterSet::ISO8859_1, "ISO8859_1");
}

#[test]
fn test_shift_jis_trail_byte_0x7f() {
    // 0x7F is not a trail byte
    do_test(&[0x81, 0x7f], CharacterSet::ISO8859_1, "ISO8859_1");
}

#[test]
fn test_shift_jis_trail_byte_0xfc() {
    // 0xFC is the last trail byte
    do_test(&[0x81, 0xfc], CharacterSet::Shift_JIS, "SJIS");
}

fn do_test(bytes: &[u8], charset: CharacterSet, encoding: &str) {
    let guessedCharset = string_utils::guessCharset(bytes, &DecodeHints::default()).unwrap();
    let guessedEncoding = string_utils::guessEncoding(bytes, &DecodeHints::default()).unwrap();
    assert_eq!(charset, guessedCharset);
    assert_eq!(encoding, guessedEncoding);
}

// /**
//  * Utility for printing out a string in given encoding as a Java statement, since it's better
//  * to write that into the Java source file rather than risk character encoding issues in the
//  * source file itself.
//  *
//  * @param args command line arguments
//  */
// fn main(String[] args) {
//   String text = args[0];
//   Charset charset = Charset.forName(args[1]);
//   StringBuilder declaration = new StringBuilder();
//   declaration.append("new byte[] { ");
//   for (byte b : text.getBytes(charset)) {
//     declaration.append("(byte) 0x");
//     int value = b & 0xFF;
//     if (value < 0x10) {
//       declaration.append('0');
//     }
//     declaration.append(Integer.toHexString(value));
//     declaration.append(", ");
//   }
//   declaration.append('}');
//   System.out.println(declaration);
// }

// }
