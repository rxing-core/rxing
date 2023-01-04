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

// package com.google.zxing.common;

// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;
// import java.util.Map;

use encoding::{Encoding, EncodingRef};

use crate::{DecodeHintType, DecodeHintValue, DecodingHintDictionary};

use lazy_static::lazy_static;

/**
 * Common string-related functions.
 *
 * @author Sean Owen
 * @author Alex Dupre
 */
pub struct StringUtils {
    //   private static final Charset PLATFORM_DEFAULT_ENCODING = Charset.defaultCharset();
    //   public static final Charset SHIFT_JIS_CHARSET = Charset.forName("SJIS");
    //   public static final Charset GB2312_CHARSET = Charset.forName("GB2312");
    //   private static final Charset EUC_JP = Charset.forName("EUC_JP");
    //   private static final boolean ASSUME_SHIFT_JIS =
    //       SHIFT_JIS_CHARSET.equals(PLATFORM_DEFAULT_ENCODING) ||
    //       EUC_JP.equals(PLATFORM_DEFAULT_ENCODING);

    //   // Retained for ABI compatibility with earlier versions
    //   public static final String SHIFT_JIS = "SJIS";
    //   public static final String GB2312 = "GB2312";
}

// const PLATFORM_DEFAULT_ENCODING: &dyn Encoding = encoding::all::UTF_8;
// const SHIFT_JIS_CHARSET: &dyn Encoding =
//     encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
// const GB2312_CHARSET: &dyn Encoding =
//     encoding::label::encoding_from_whatwg_label("GB2312").unwrap();
// const EUC_JP: &dyn Encoding = encoding::label::encoding_from_whatwg_label("EUC_JP").unwrap();
const ASSUME_SHIFT_JIS: bool = false;
// static SHIFT_JIS: &'static str = "SJIS";
// static GB2312: &'static str = "GB2312";
lazy_static! {
    pub static ref SHIFT_JIS_CHARSET: EncodingRef =
        encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
}

//    private static final boolean ASSUME_SHIFT_JIS =
//        SHIFT_JIS_CHARSET.equals(PLATFORM_DEFAULT_ENCODING) ||
//        EUC_JP.equals(PLATFORM_DEFAULT_ENCODING);

impl StringUtils {
    /**
     * @param bytes bytes encoding a string, whose encoding should be guessed
     * @param hints decode hints if applicable
     * @return name of guessed encoding; at the moment will only guess one of:
     *  "SJIS", "UTF8", "ISO8859_1", or the platform default encoding if none
     *  of these can possibly be correct
     */
    pub fn guessEncoding(bytes: &[u8], hints: &DecodingHintDictionary) -> String {
        let c = StringUtils::guessCharset(bytes, hints);
        if c.name()
            == encoding::label::encoding_from_whatwg_label("SJIS")
                .unwrap()
                .name()
        {
            return "SJIS".to_owned();
        } else if c.name() == encoding::all::UTF_8.name() {
            return "UTF8".to_owned();
        } else if c.name() == encoding::all::ISO_8859_1.name() {
            return "ISO8859_1".to_owned();
        }
        return c.name().to_owned();
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
    pub fn guessCharset(bytes: &[u8], hints: &DecodingHintDictionary) -> EncodingRef {
        if let Some(DecodeHintValue::CharacterSet(cs_name)) =
            hints.get(&DecodeHintType::CHARACTER_SET)
        {
            // if let DecodeHintValue::CharacterSet(cs_name) = hint {
            return encoding::label::encoding_from_whatwg_label(cs_name).unwrap();
            // }
        }
        // if hints.contains_key(&DecodeHintType::CHARACTER_SET) {
        //   return Charset.forName(hints.get(DecodeHintType.CHARACTER_SET).toString());
        // }

        // First try UTF-16, assuming anything with its BOM is UTF-16
        if bytes.len() > 2
            && ((bytes[0] == 0xFE && bytes[1] == 0xFF) || (bytes[0] == 0xFF && bytes[1] == 0xFE))
        {
            if bytes[0] == 0xFE && bytes[1] == 0xFF {
                return encoding::all::UTF_16BE;
            } else {
                return encoding::all::UTF_16LE;
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

        let utf8bom = bytes.len() > 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF;

        // for i in 0..length {
        for value in bytes.iter().take(length).copied() {
            // for (int i = 0;
            //      i < length && (canBeISO88591 || canBeShiftJIS || canBeUTF8);
            //      i++) {
            if !(can_be_iso88591 || can_be_shift_jis || can_be_utf8) {
                break;
            }

            // let value = bytes[i];

            // UTF-8 stuff
            if can_be_utf8 {
                if utf8_bytes_left > 0 {
                    if (value & 0x80) == 0 {
                        can_be_utf8 = false;
                    } else {
                        utf8_bytes_left -= 1;
                    }
                } else if (value & 0x80) != 0 {
                    if (value & 0x40) == 0 {
                        can_be_utf8 = false;
                    } else {
                        utf8_bytes_left += 1;
                        if (value & 0x20) == 0 {
                            utf2_bytes_chars += 1;
                        } else {
                            utf8_bytes_left += 1;
                            if (value & 0x10) == 0 {
                                utf3_bytes_chars += 1;
                            } else {
                                utf8_bytes_left += 1;
                                if (value & 0x08) == 0 {
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
                if value > 0x7F && value < 0xA0 {
                    can_be_iso88591 = false;
                } else if value > 0x9F && (value < 0xC0 || value == 0xD7 || value == 0xF7) {
                    iso_high_other += 1;
                }
            }

            // Shift_JIS stuff
            if can_be_shift_jis {
                if sjis_bytes_left > 0 {
                    if value < 0x40 || value == 0x7F || value > 0xFC {
                        can_be_shift_jis = false;
                    } else {
                        sjis_bytes_left -= 1;
                    }
                } else if value == 0x80 || value == 0xA0 || value > 0xEF {
                    can_be_shift_jis = false;
                } else if value > 0xA0 && value < 0xE0 {
                    sjis_katakana_chars += 1;
                    sjis_cur_double_bytes_word_length = 0;
                    sjis_cur_katakana_word_length += 1;
                    if sjis_cur_katakana_word_length > sjis_max_katakana_word_length {
                        sjis_max_katakana_word_length = sjis_cur_katakana_word_length;
                    }
                } else if value > 0x7F {
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
            return encoding::all::UTF_8;
        }
        // Easy -- if assuming Shift_JIS or >= 3 valid consecutive not-ascii characters (and no evidence it can't be), done
        if can_be_shift_jis
            && (ASSUME_SHIFT_JIS
                || sjis_max_katakana_word_length >= 3
                || sjis_max_double_bytes_word_length >= 3)
        {
            return encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
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
                encoding::label::encoding_from_whatwg_label("SJIS").unwrap()
            } else {
                encoding::all::ISO_8859_1
            };
        }

        // Otherwise, try in order ISO-8859-1, Shift JIS, UTF-8 and fall back to default platform encoding
        if can_be_iso88591 {
            return encoding::all::ISO_8859_1;
        }
        if can_be_shift_jis {
            return encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
        }
        if can_be_utf8 {
            return encoding::all::UTF_8;
        }
        // Otherwise, we take a wild guess with platform encoding
        encoding::all::UTF_8
    }
}
