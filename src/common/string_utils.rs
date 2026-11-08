/*
 * Copyright (C) 2010 ZXing authors
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

use crate::DecodeHints;

use super::CharacterSet;

/*
 * Common string-related functions.
 *
 * @author Sean Owen
 * @author Alex Dupre
 */

// const PLATFORM_DEFAULT_ENCODING: &dyn Encoding = encoding::all::UTF_8;
// const SHIFT_JIS_CHARSET: &dyn Encoding =
//     encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
// const GB2312_CHARSET: &dyn Encoding =
//     encoding::label::encoding_from_whatwg_label("GB2312").unwrap();
// const EUC_JP: &dyn Encoding = encoding::label::encoding_from_whatwg_label("EUC_JP").unwrap();
// static SHIFT_JIS: &'static str = "SJIS";
// static GB2312: &'static str = "GB2312";

pub const SHIFT_JIS_CHARSET: CharacterSet = CharacterSet::Shift_JIS;

/**
 * @param bytes bytes encoding a string, whose encoding should be guessed
 * @param hints decode hints if applicable
 * @return name of guessed encoding; at the moment will only guess one of:
 *  "SJIS", "UTF8", "ISO8859_1", or the platform default encoding if none
 *  of these can possibly be correct
 */
pub fn guessEncoding(bytes: &[u8], hints: &DecodeHints) -> Option<&'static str> {
    let c = guessCharset(bytes, hints)?;
    if c == CharacterSet::Shift_JIS {
        Some("SJIS")
    } else if c == CharacterSet::UTF8 {
        Some("UTF8")
    } else if c == CharacterSet::ISO8859_1 {
        Some("ISO8859_1")
    } else {
        Some(c.get_charset_name())
    }
}

/**
 * @param bytes bytes encoding a string, whose encoding should be guessed
 * @param hints decode hints if applicable
 * @return Charset of guessed encoding: the {@code CharacterSet} hint if given; else
 *  UTF-16 if the bytes start with its byte order mark; else UTF-8 if they are valid UTF-8
 *  with at least one multi-byte character; else Shift_JIS if they are valid Shift_JIS with
 *  a byte in 0x80-0x9F, 3 half-width katakana in a row or 3 double-byte characters in a row;
 *  else ISO-8859-1
 */
pub fn guessCharset(bytes: &[u8], hints: &DecodeHints) -> Option<CharacterSet> {
    if let Some(cs_name) = &hints.CharacterSet {
        return CharacterSet::get_character_set_by_name(cs_name);
    }

    // First try UTF-16, assuming anything with its BOM is UTF-16

    if bytes.len() > 2 {
        match bytes[0..2] {
            [0xFE, 0xFF] => return Some(CharacterSet::UTF16BE),
            [0xFF, 0xFE] => return Some(CharacterSet::UTF16LE),
            _ => {}
        }
    }

    // ISO/IEC 18004 reads byte mode without an ECI as ISO-8859-1 (2006 6.3.1, 2015 7.3.2,
    // 2024 5.1). Only guess otherwise when the bytes carry evidence of another encoding.

    // Valid UTF-8 with a multi-byte character (a UTF-8 BOM is one)
    if !bytes.is_ascii() && std::str::from_utf8(bytes).is_ok() {
        return Some(CharacterSet::UTF8);
    }

    // Shift_JIS byte structure, with the ranges of the WHATWG Encoding Standard's decoder: single
    // bytes 0x00-0x7F and 0xA1-0xDF (half-width katakana), lead bytes 0x81-0x9F and 0xE0-0xFC
    // followed by a trail byte 0x40-0x7E or 0x80-0xFC. Unlike that decoder, which maps a single
    // 0x80 to U+0080, this does not accept 0x80 alone: it is a C1 control code either way, not
    // Shift_JIS text. Evidence for Shift_JIS: a byte in 0x80-0x9F (C1 control codes in
    // ISO-8859-1, but the lead bytes of most double-byte characters), or a run of 3 half-width
    // katakana or of 3 double-byte characters.
    let mut sjis_trail_byte_next = false;
    let mut sjis_katakana_run = 0;
    let mut sjis_double_byte_run = 0;
    let mut sjis_evidence = false;
    for &byte in bytes {
        if sjis_trail_byte_next {
            sjis_trail_byte_next = false;
            if !matches!(byte, 0x40..=0x7E | 0x80..=0xFC) {
                return Some(CharacterSet::ISO8859_1);
            }
            sjis_evidence |= matches!(byte, 0x80..=0x9F);
        } else {
            match byte {
                0x00..=0x7F => {
                    sjis_katakana_run = 0;
                    sjis_double_byte_run = 0;
                }
                0xA1..=0xDF => {
                    sjis_katakana_run += 1;
                    sjis_double_byte_run = 0;
                    sjis_evidence |= sjis_katakana_run >= 3;
                }
                0x81..=0x9F | 0xE0..=0xFC => {
                    sjis_trail_byte_next = true;
                    sjis_katakana_run = 0;
                    sjis_double_byte_run += 1;
                    sjis_evidence |= byte <= 0x9F || sjis_double_byte_run >= 3;
                }
                _ => return Some(CharacterSet::ISO8859_1),
            }
        }
    }
    if !sjis_trail_byte_next && sjis_evidence {
        return Some(CharacterSet::Shift_JIS);
    }
    Some(CharacterSet::ISO8859_1)
}
