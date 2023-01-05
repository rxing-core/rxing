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

use encoding::{Encoding, EncodingRef};
use rand::Rng;
use std::collections::HashMap;

use crate::common::StringUtils;

#[test]
fn test_random() {
    let mut r = rand::thread_rng();
    let mut bytes: Vec<u8> = vec![0; 1000];
    bytes.fill_with(|| r.gen());
    // for byte in &mut bytes {
    //     *byte = r.gen();
    // }
    assert_eq!(
        encoding::all::UTF_8.name(),
        StringUtils::guessCharset(&bytes, &HashMap::new()).name()
    );
}

#[test]
fn test_short_shift_jis1() {
    // 金魚
    do_test(
        &[0x8b, 0xe0, 0x8b, 0x9b],
        encoding::label::encoding_from_whatwg_label("SJIS").unwrap(),
        "SJIS",
    );
}

#[test]
fn test_short_iso885911() {
    // båd
    do_test(&[0x62, 0xe5, 0x64], encoding::all::ISO_8859_1, "ISO8859_1");
}

#[test]
fn test_short_utf8() {
    // Español
    do_test(
        &[0x45, 0x73, 0x70, 0x61, 0xc3, 0xb1, 0x6f, 0x6c],
        encoding::all::UTF_8,
        "UTF8",
    );
}

#[test]
fn test_mixed_shift_jis1() {
    // Hello 金!
    do_test(
        &[0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x8b, 0xe0, 0x21],
        encoding::label::encoding_from_whatwg_label("SJIS").unwrap(),
        "SJIS",
    );
}

#[test]
fn test_utf16_be() {
    // 调压柜
    do_test(
        &[0xFE, 0xFF, 0x8c, 0x03, 0x53, 0x8b, 0x67, 0xdc],
        encoding::all::UTF_16BE,
        encoding::all::UTF_16BE.name(),
    );
}

#[test]
fn test_utf16_le() {
    // 调压柜
    do_test(
        &[0xFF, 0xFE, 0x03, 0x8c, 0x8b, 0x53, 0xdc, 0x67],
        encoding::all::UTF_16LE,
        encoding::all::UTF_16LE.name(),
    );
}

fn do_test(bytes: &[u8], charset: EncodingRef, encoding: &str) {
    let guessedCharset = StringUtils::guessCharset(bytes, &HashMap::new());
    let guessedEncoding = StringUtils::guessEncoding(bytes, &HashMap::new());
    assert_eq!(charset.name(), guessedCharset.name());
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
