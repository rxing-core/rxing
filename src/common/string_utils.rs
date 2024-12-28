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

/**
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
const ASSUME_SHIFT_JIS: bool = false;
// static SHIFT_JIS: &'static str = "SJIS";
// static GB2312: &'static str = "GB2312";

pub const SHIFT_JIS_CHARSET: CharacterSet = CharacterSet::Shift_JIS;

//    private static final boolean ASSUME_SHIFT_JIS =
//        SHIFT_JIS_CHARSET.equals(PLATFORM_DEFAULT_ENCODING) ||
//        EUC_JP.equals(PLATFORM_DEFAULT_ENCODING);

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
 * @return Charset of guessed encoding; at the moment will only guess one of:
 *  {@link #SHIFT_JIS_CHARSET}, {@link StandardCharsets#UTF_8},
 *  {@link StandardCharsets#ISO_8859_1}, {@link StandardCharsets#UTF_16},
 *  or the platform default encoding if
 *  none of these can possibly be correct
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

    // For now, merely tries to distinguish ISO-8859-1, UTF-8 and Shift_JIS,
    // which should be by far the most common encodings.
    let length = bytes.len();
    let mut can_be_iso88591 = true;
    let mut can_be_shift_jis = true;
    let mut can_be_utf8 = true;
    let mut utf8_bytes_left = 0;
    let mut utf2_bytes_chars = 0;
    let mut utf3_bytes_chars = 0;
    let mut utf4_bytes_chars = 0;
    let mut sjis_bytes_left = 0;
    let mut sjis_katakana_chars = 0;
    let mut sjis_cur_katakana_word_length = 0;
    let mut sjis_cur_double_bytes_word_length = 0;
    let mut sjis_max_katakana_word_length = 0;
    let mut sjis_max_double_bytes_word_length = 0;
    let mut iso_high_other = 0;

    let utf8bom = bytes.len() > 3 && bytes[0..=2] == [0xEF, 0xBB, 0xBF];

    // for i in 0..length {
    for &byte in bytes {
        if !(can_be_iso88591 || can_be_shift_jis || can_be_utf8) {
            break;
        }

        // UTF-8 stuff
        if can_be_utf8 {
            if utf8_bytes_left > 0 {
                if (byte & 0x80) == 0 {
                    can_be_utf8 = false;
                } else {
                    utf8_bytes_left -= 1;
                }
            } else if (byte & 0x80) != 0 {
                if (byte & 0x40) == 0 {
                    can_be_utf8 = false;
                } else {
                    utf8_bytes_left += 1;
                    if (byte & 0x20) == 0 {
                        utf2_bytes_chars += 1;
                    } else {
                        utf8_bytes_left += 1;
                        if (byte & 0x10) == 0 {
                            utf3_bytes_chars += 1;
                        } else {
                            utf8_bytes_left += 1;
                            if (byte & 0x08) == 0 {
                                utf4_bytes_chars += 1;
                            } else {
                                can_be_utf8 = false;
                            }
                        }
                    }
                }
            }
        }

        // ISO-8859-1 stuff
        if can_be_iso88591 {
            if matches!(byte, 0x7F..0xA0) {
                // if byte > 0x7F && byte < 0xA0 {
                can_be_iso88591 = false;
            } else if byte > 0x9F && (byte < 0xC0 || byte == 0xD7 || byte == 0xF7) {
                iso_high_other += 1;
            }
        }

        // Shift_JIS stuff
        if can_be_shift_jis {
            if sjis_bytes_left > 0 {
                if matches!(byte, 0x40 | 0x7F | 0xFC) {
                    can_be_shift_jis = false;
                } else {
                    sjis_bytes_left -= 1;
                }
            } else if matches!(byte, 0x80 | 0xA0 | 0xEF) {
                can_be_shift_jis = false;
            } else if matches!(byte, 0xA0 | 0xE0) {
                sjis_katakana_chars += 1;
                sjis_cur_double_bytes_word_length = 0;
                sjis_cur_katakana_word_length += 1;
                if sjis_cur_katakana_word_length > sjis_max_katakana_word_length {
                    sjis_max_katakana_word_length = sjis_cur_katakana_word_length;
                }
            } else if byte > 0x7F {
                sjis_bytes_left += 1;
                //sjisDoubleBytesChars++;
                sjis_cur_katakana_word_length = 0;
                sjis_cur_double_bytes_word_length += 1;
                if sjis_cur_double_bytes_word_length > sjis_max_double_bytes_word_length {
                    sjis_max_double_bytes_word_length = sjis_cur_double_bytes_word_length;
                }
            } else {
                //sjisLowChars++;
                sjis_cur_katakana_word_length = 0;
                sjis_cur_double_bytes_word_length = 0;
            }
        }
    }

    if can_be_utf8 && utf8_bytes_left > 0 {
        can_be_utf8 = false;
    }
    if can_be_shift_jis && sjis_bytes_left > 0 {
        can_be_shift_jis = false;
    }

    // Easy -- if there is BOM or at least 1 valid not-single byte character (and no evidence it can't be UTF-8), done
    if can_be_utf8 && (utf8bom || utf2_bytes_chars + utf3_bytes_chars + utf4_bytes_chars > 0) {
        return Some(CharacterSet::UTF8);
    }
    // Easy -- if assuming Shift_JIS or >= 3 valid consecutive not-ascii characters (and no evidence it can't be), done
    if can_be_shift_jis
        && (ASSUME_SHIFT_JIS
            || sjis_max_katakana_word_length >= 3
            || sjis_max_double_bytes_word_length >= 3)
    {
        return Some(CharacterSet::Shift_JIS); //encoding::label::encoding_from_whatwg_label("SJIS");
    }
    // Distinguishing Shift_JIS and ISO-8859-1 can be a little tough for short words. The crude heuristic is:
    // - If we saw
    //   - only two consecutive katakana chars in the whole text, or
    //   - at least 10% of bytes that could be "upper" not-alphanumeric Latin1,
    // - then we conclude Shift_JIS, else ISO-8859-1
    if can_be_iso88591 && can_be_shift_jis {
        return if (sjis_max_katakana_word_length == 2 && sjis_katakana_chars == 2)
            || iso_high_other * 10 >= length
        {
            Some(CharacterSet::Shift_JIS)
        } else {
            Some(CharacterSet::ISO8859_1)
        };
    }

    // Otherwise, try in order ISO-8859-1, Shift JIS, UTF-8 and fall back to default platform encoding
    if can_be_iso88591 {
        return Some(CharacterSet::ISO8859_1);
    }
    if can_be_shift_jis {
        return Some(CharacterSet::Shift_JIS);
    }
    if can_be_utf8 {
        return Some(CharacterSet::UTF8);
    }
    // Otherwise, we take a wild guess with platform encoding
    Some(CharacterSet::UTF8)
}
