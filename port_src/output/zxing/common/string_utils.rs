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
// package com::google::zxing::common;

/**
 * Common string-related functions.
 *
 * @author Sean Owen
 * @author Alex Dupre
 */

 const PLATFORM_DEFAULT_ENCODING: Charset = Charset::default_charset();

 const SHIFT_JIS_CHARSET: Charset = Charset::for_name("SJIS");

 const GB2312_CHARSET: Charset = Charset::for_name("GB2312");

 const EUC_JP: Charset = Charset::for_name("EUC_JP");

 const ASSUME_SHIFT_JIS: bool = SHIFT_JIS_CHARSET::equals(&PLATFORM_DEFAULT_ENCODING) || EUC_JP::equals(&PLATFORM_DEFAULT_ENCODING);

// Retained for ABI compatibility with earlier versions
 const SHIFT_JIS: &'static str = "SJIS";

 const GB2312: &'static str = "GB2312";
pub struct StringUtils {
}

impl StringUtils {

    fn new() -> StringUtils {
    }

    /**
   * @param bytes bytes encoding a string, whose encoding should be guessed
   * @param hints decode hints if applicable
   * @return name of guessed encoding; at the moment will only guess one of:
   *  "SJIS", "UTF8", "ISO8859_1", or the platform default encoding if none
   *  of these can possibly be correct
   */
    pub fn  guess_encoding( bytes: &Vec<i8>,  hints: &Map<DecodeHintType, ?>) -> String  {
         let c: Charset = ::guess_charset(&bytes, &hints);
        if c == SHIFT_JIS_CHARSET {
            return "SJIS";
        } else if c == StandardCharsets::UTF_8 {
            return "UTF8";
        } else if c == StandardCharsets::ISO_8859_1 {
            return "ISO8859_1";
        }
        return c.name();
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
    pub fn  guess_charset( bytes: &Vec<i8>,  hints: &Map<DecodeHintType, ?>) -> Charset  {
        if hints != null && hints.contains_key(DecodeHintType::CHARACTER_SET) {
            return Charset::for_name(&hints.get(DecodeHintType::CHARACTER_SET).to_string());
        }
        // First try UTF-16, assuming anything with its BOM is UTF-16
        if bytes.len() > 2 && ((bytes[0] == 0xFE as i8 && bytes[1] == 0xFF as i8) || (bytes[0] == 0xFF as i8 && bytes[1] == 0xFE as i8)) {
            return StandardCharsets::UTF_16;
        }
        // For now, merely tries to distinguish ISO-8859-1, UTF-8 and Shift_JIS,
        // which should be by far the most common encodings.
         let length: i32 = bytes.len();
         let can_be_i_s_o88591: bool = true;
         let can_be_shift_j_i_s: bool = true;
         let can_be_u_t_f8: bool = true;
         let utf8_bytes_left: i32 = 0;
         let utf2_bytes_chars: i32 = 0;
         let utf3_bytes_chars: i32 = 0;
         let utf4_bytes_chars: i32 = 0;
         let sjis_bytes_left: i32 = 0;
         let sjis_katakana_chars: i32 = 0;
         let sjis_cur_katakana_word_length: i32 = 0;
         let sjis_cur_double_bytes_word_length: i32 = 0;
         let sjis_max_katakana_word_length: i32 = 0;
         let sjis_max_double_bytes_word_length: i32 = 0;
         let iso_high_other: i32 = 0;
         let utf8bom: bool = bytes.len() > 3 && bytes[0] == 0xEF as i8 && bytes[1] == 0xBB as i8 && bytes[2] == 0xBF as i8;
         {
             let mut i: i32 = 0;
            while i < length && (can_be_i_s_o88591 || can_be_shift_j_i_s || can_be_u_t_f8) {
                {
                     let value: i32 = bytes[i] & 0xFF;
                    // UTF-8 stuff
                    if can_be_u_t_f8 {
                        if utf8_bytes_left > 0 {
                            if (value & 0x80) == 0 {
                                can_be_u_t_f8 = false;
                            } else {
                                utf8_bytes_left -= 1;
                            }
                        } else if (value & 0x80) != 0 {
                            if (value & 0x40) == 0 {
                                can_be_u_t_f8 = false;
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
                                            can_be_u_t_f8 = false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // ISO-8859-1 stuff
                    if can_be_i_s_o88591 {
                        if value > 0x7F && value < 0xA0 {
                            can_be_i_s_o88591 = false;
                        } else if value > 0x9F && (value < 0xC0 || value == 0xD7 || value == 0xF7) {
                            iso_high_other += 1;
                        }
                    }
                    // Shift_JIS stuff
                    if can_be_shift_j_i_s {
                        if sjis_bytes_left > 0 {
                            if value < 0x40 || value == 0x7F || value > 0xFC {
                                can_be_shift_j_i_s = false;
                            } else {
                                sjis_bytes_left -= 1;
                            }
                        } else if value == 0x80 || value == 0xA0 || value > 0xEF {
                            can_be_shift_j_i_s = false;
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
                i += 1;
             }
         }

        if can_be_u_t_f8 && utf8_bytes_left > 0 {
            can_be_u_t_f8 = false;
        }
        if can_be_shift_j_i_s && sjis_bytes_left > 0 {
            can_be_shift_j_i_s = false;
        }
        // Easy -- if there is BOM or at least 1 valid not-single byte character (and no evidence it can't be UTF-8), done
        if can_be_u_t_f8 && (utf8bom || utf2_bytes_chars + utf3_bytes_chars + utf4_bytes_chars > 0) {
            return StandardCharsets::UTF_8;
        }
        // Easy -- if assuming Shift_JIS or >= 3 valid consecutive not-ascii characters (and no evidence it can't be), done
        if can_be_shift_j_i_s && (ASSUME_SHIFT_JIS || sjis_max_katakana_word_length >= 3 || sjis_max_double_bytes_word_length >= 3) {
            return SHIFT_JIS_CHARSET;
        }
        // - then we conclude Shift_JIS, else ISO-8859-1
        if can_be_i_s_o88591 && can_be_shift_j_i_s {
            return  if (sjis_max_katakana_word_length == 2 && sjis_katakana_chars == 2) || iso_high_other * 10 >= length { SHIFT_JIS_CHARSET } else { StandardCharsets::ISO_8859_1 };
        }
        // Otherwise, try in order ISO-8859-1, Shift JIS, UTF-8 and fall back to default platform encoding
        if can_be_i_s_o88591 {
            return StandardCharsets::ISO_8859_1;
        }
        if can_be_shift_j_i_s {
            return SHIFT_JIS_CHARSET;
        }
        if can_be_u_t_f8 {
            return StandardCharsets::UTF_8;
        }
        // Otherwise, we take a wild guess with platform encoding
        return PLATFORM_DEFAULT_ENCODING;
    }
}

