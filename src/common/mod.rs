pub mod detector;
pub mod reedsolomon;

use std::{any::Any, collections::HashMap};

use crate::DecodeHintType;
use encoding::Encoding;

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

const PLATFORM_DEFAULT_ENCODING: Encoding = encoding::all::UTF_8;
const SHIFT_JIS_CHARSET: Encoding = encoding::label::encoding_from_whatwg_label("SJIS");
const GB2312_CHARSET: Encoding = encoding::label::encoding_from_whatwg_label("GB2312");
const EUC_JP: Encoding = encoding::label::encoding_from_whatwg_label("EUC_JP");
const ASSUME_SHIFT_JIS: bool = false;
static SHIFT_JIS: &'static str = "SJIS";
static GB2312: &'static str = "GB2312";

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
    pub fn guessEncoding(bytes: &[u8], hints: HashMap<DecodeHintType, &dyn Any>) -> &str {
        let c = StringUtils::guessCharset(bytes, hints);
        if c == SHIFT_JIS_CHARSET {
            return "SJIS";
        } else if c == encoding::all::UTF_8 {
            return "UTF8";
        } else if c == encoding::all::ISO_8859_1 {
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
    pub fn guessCharset(
        bytes: &[u8],
        hints: HashMap<DecodeHintType, &dyn Any>,
    ) -> Box<&dyn Encoding> {
        match hints.get(&DecodeHintType::CHARACTER_SET) {
            Some(hint) => {
                if hint.is::<String>() {
                    return encoding::label::encoding_from_whatwg_label(hint).unwrap();
                }
            }
            _ => {}
        };
        // if hints.contains_key(&DecodeHintType::CHARACTER_SET) {
        //   return Charset.forName(hints.get(DecodeHintType.CHARACTER_SET).toString());
        // }

        // First try UTF-16, assuming anything with its BOM is UTF-16
        if bytes.len() > 2
            && ((bytes[0] == 0xFE && bytes[1] == 0xFF) || (bytes[0] == 0xFF && bytes[1] == 0xFE))
        {
            return encoding::all::UTF_16BE;
        }

        // For now, merely tries to distinguish ISO-8859-1, UTF-8 and Shift_JIS,
        // which should be by far the most common encodings.
        let length = bytes.len();
        let canBeISO88591 = true;
        let canBeShiftJIS = true;
        let canBeUTF8 = true;
        let utf8BytesLeft = 0;
        let utf2BytesChars = 0;
        let utf3BytesChars = 0;
        let utf4BytesChars = 0;
        let sjisBytesLeft = 0;
        let sjisKatakanaChars = 0;
        let sjisCurKatakanaWordLength = 0;
        let sjisCurDoubleBytesWordLength = 0;
        let sjisMaxKatakanaWordLength = 0;
        let sjisMaxDoubleBytesWordLength = 0;
        let isoHighOther = 0;

        let utf8bom = bytes.len() > 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF;

        for i in 0..length {
            // for (int i = 0;
            //      i < length && (canBeISO88591 || canBeShiftJIS || canBeUTF8);
            //      i++) {
            if canBeISO88591 || canBeShiftJIS || canBeUTF8 {
                break;
            }

            let value = bytes[i] & 0xFF;

            // UTF-8 stuff
            if canBeUTF8 {
                if utf8BytesLeft > 0 {
                    if (value & 0x80) == 0 {
                        canBeUTF8 = false;
                    } else {
                        utf8BytesLeft -= 1;
                    }
                } else if (value & 0x80) != 0 {
                    if (value & 0x40) == 0 {
                        canBeUTF8 = false;
                    } else {
                        utf8BytesLeft += 1;
                        if (value & 0x20) == 0 {
                            utf2BytesChars += 1;
                        } else {
                            utf8BytesLeft += 1;
                            if (value & 0x10) == 0 {
                                utf3BytesChars += 1;
                            } else {
                                utf8BytesLeft += 1;
                                if (value & 0x08) == 0 {
                                    utf4BytesChars += 1;
                                } else {
                                    canBeUTF8 = false;
                                }
                            }
                        }
                    }
                }
            }

            // ISO-8859-1 stuff
            if canBeISO88591 {
                if value > 0x7F && value < 0xA0 {
                    canBeISO88591 = false;
                } else if value > 0x9F && (value < 0xC0 || value == 0xD7 || value == 0xF7) {
                    isoHighOther += 1;
                }
            }

            // Shift_JIS stuff
            if canBeShiftJIS {
                if sjisBytesLeft > 0 {
                    if value < 0x40 || value == 0x7F || value > 0xFC {
                        canBeShiftJIS = false;
                    } else {
                        sjisBytesLeft -= 1;
                    }
                } else if value == 0x80 || value == 0xA0 || value > 0xEF {
                    canBeShiftJIS = false;
                } else if value > 0xA0 && value < 0xE0 {
                    sjisKatakanaChars += 1;
                    sjisCurDoubleBytesWordLength = 0;
                    sjisCurKatakanaWordLength += 1;
                    if sjisCurKatakanaWordLength > sjisMaxKatakanaWordLength {
                        sjisMaxKatakanaWordLength = sjisCurKatakanaWordLength;
                    }
                } else if value > 0x7F {
                    sjisBytesLeft += 1;
                    //sjisDoubleBytesChars++;
                    sjisCurKatakanaWordLength = 0;
                    sjisCurDoubleBytesWordLength += 1;
                    if sjisCurDoubleBytesWordLength > sjisMaxDoubleBytesWordLength {
                        sjisMaxDoubleBytesWordLength = sjisCurDoubleBytesWordLength;
                    }
                } else {
                    //sjisLowChars++;
                    sjisCurKatakanaWordLength = 0;
                    sjisCurDoubleBytesWordLength = 0;
                }
            }
        }

        if canBeUTF8 && utf8BytesLeft > 0 {
            canBeUTF8 = false;
        }
        if canBeShiftJIS && sjisBytesLeft > 0 {
            canBeShiftJIS = false;
        }

        // Easy -- if there is BOM or at least 1 valid not-single byte character (and no evidence it can't be UTF-8), done
        if canBeUTF8 && (utf8bom || utf2BytesChars + utf3BytesChars + utf4BytesChars > 0) {
            return encoding::all::UTF_8;
        }
        // Easy -- if assuming Shift_JIS or >= 3 valid consecutive not-ascii characters (and no evidence it can't be), done
        if canBeShiftJIS
            && (ASSUME_SHIFT_JIS
                || sjisMaxKatakanaWordLength >= 3
                || sjisMaxDoubleBytesWordLength >= 3)
        {
            return SHIFT_JIS_CHARSET;
        }
        // Distinguishing Shift_JIS and ISO-8859-1 can be a little tough for short words. The crude heuristic is:
        // - If we saw
        //   - only two consecutive katakana chars in the whole text, or
        //   - at least 10% of bytes that could be "upper" not-alphanumeric Latin1,
        // - then we conclude Shift_JIS, else ISO-8859-1
        if canBeISO88591 && canBeShiftJIS {
            return if (sjisMaxKatakanaWordLength == 2 && sjisKatakanaChars == 2)
                || isoHighOther * 10 >= length
            {
                SHIFT_JIS_CHARSET
            } else {
                encoding::all::ISO_8859_1
            };
        }

        // Otherwise, try in order ISO-8859-1, Shift JIS, UTF-8 and fall back to default platform encoding
        if canBeISO88591 {
            return encoding::all::ISO_8859_1;
        }
        if canBeShiftJIS {
            return SHIFT_JIS_CHARSET;
        }
        if canBeUTF8 {
            return encoding::all::UTF_8;
        }
        // Otherwise, we take a wild guess with platform encoding
        return PLATFORM_DEFAULT_ENCODING;
    }
}
