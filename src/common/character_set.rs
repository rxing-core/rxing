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

use encoding::EncodingRef;

use crate::common::Result;
use crate::Exceptions;

/**
 * Encapsulates a Character Set ECI, according to "Extended Channel Interpretations" 5.3.1.1
 * of ISO 18004.
 *
 * @author Sean Owen
 */
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CharacterSet {
    // Enum name is a Java encoding valid for java.lang and java.io
    Cp437,      //(new int[]{0,2}),
    ISO8859_1,  //(new int[]{1,3}, "ISO-8859-1"),
    ISO8859_2,  //(4, "ISO-8859-2"),
    ISO8859_3,  //(5, "ISO-8859-3"),
    ISO8859_4,  //(6, "ISO-8859-4"),
    ISO8859_5,  //(7, "ISO-8859-5"),
    // ISO8859_6,  //(8, "ISO-8859-6"),
    ISO8859_7,  //(9, "ISO-8859-7"),
    // ISO8859_8,  //(10, "ISO-8859-8"),
    ISO8859_9,  //(11, "ISO-8859-9"),
    // ISO8859_10, //(12, "ISO-8859-10"),
    // ISO8859_11, //(13, "ISO-8859-11"),
    ISO8859_13, //(15, "ISO-8859-13"),
    // ISO8859_14, //(16, "ISO-8859-14"),
    ISO8859_15, //(17, "ISO-8859-15"),
    ISO8859_16, //(18, "ISO-8859-16"),
    Shift_JIS,  //(20, "Shift_JIS"),
    Cp1250,     //(21, "windows-1250"),
    Cp1251,     //(22, "windows-1251"),
    Cp1252,     //(23, "windows-1252"),
    Cp1256,     //(24, "windows-1256"),
    UTF16BE,    //(25, "UTF-16BE", "UnicodeBig"),
    UTF8,       //(26, "UTF-8"),
    ASCII,      //(new int[] {27, 170}, "US-ASCII"),
    Big5,       //(28),
    GB2312,
    GB18030, //(29, "GB2312", "EUC_CN", "GBK"),
    EUC_KR,  //(30, "EUC-KR");
    UTF16LE,
    UTF32BE,
    UTF32LE,
    Binary,
    Unknown,
}
impl CharacterSet {
    // pub fn get_eci_value(&self) -> u32 {
    //     match self {
    //         CharacterSet::Cp437 => 0,
    //         CharacterSet::ISO8859_1 => 1,
    //         CharacterSet::ISO8859_2 => 4,
    //         CharacterSet::ISO8859_3 => 5,
    //         CharacterSet::ISO8859_4 => 6,
    //         CharacterSet::ISO8859_5 => 7,
    //         // CharacterSetECI::ISO8859_6 => 8,
    //         CharacterSet::ISO8859_7 => 9,
    //         // CharacterSetECI::ISO8859_8 => 10,
    //         CharacterSet::ISO8859_9 => 11,
    //         // CharacterSetECI::ISO8859_10 => 12,
    //         // CharacterSetECI::ISO8859_11 => 13,
    //         CharacterSet::ISO8859_13 => 15,
    //         // CharacterSetECI::ISO8859_14 => 16,
    //         CharacterSet::ISO8859_15 => 17,
    //         CharacterSet::ISO8859_16 => 18,
    //         CharacterSet::Shift_JIS => 20,
    //         CharacterSet::Cp1250 => 21,
    //         CharacterSet::Cp1251 => 22,
    //         CharacterSet::Cp1252 => 23,
    //         CharacterSet::Cp1256 => 24,
    //         CharacterSet::UTF16BE => 25,
    //         CharacterSet::UTF8 => 26,
    //         CharacterSet::ASCII => 27,
    //         CharacterSet::Big5 => 28,
    //         CharacterSet::GB2312 => 29,
    //         CharacterSet::GB18030 => 32,
    //         CharacterSet::EUC_KR => 30,
    //         CharacterSet::UTF16LE => 33,
    //         CharacterSet::UTF32BE => 34,
    //         CharacterSet::UTF32LE => 35,
    //         CharacterSet::Binary => 899,
    //         _=>1000,
    //     }
    // }

    fn get_base_encoder(&self) -> EncodingRef {
        let name = match self {
            CharacterSet::Cp437 => "cp437",
            CharacterSet::ISO8859_1 => return encoding::all::ISO_8859_1,
            CharacterSet::ISO8859_2 => "ISO-8859-2",
            CharacterSet::ISO8859_3 => "ISO-8859-3",
            CharacterSet::ISO8859_4 => "ISO-8859-4",
            CharacterSet::ISO8859_5 => "ISO-8859-5",
            // CharacterSet::ISO8859_6 => "ISO-8859-6",
            CharacterSet::ISO8859_7 => "ISO-8859-7",
            // CharacterSet::ISO8859_8 => "ISO-8859-8",
            CharacterSet::ISO8859_9 => "ISO-8859-9",
            // CharacterSet::ISO8859_10 => "ISO-8859-10",
            // CharacterSet::ISO8859_11 => "ISO-8859-11",
            CharacterSet::ISO8859_13 => "ISO-8859-13",
            // CharacterSet::ISO8859_14 => "ISO-8859-14",
            CharacterSet::ISO8859_15 => "ISO-8859-15",
            CharacterSet::ISO8859_16 => "ISO-8859-16",
            CharacterSet::Shift_JIS => "shift_jis",
            CharacterSet::Cp1250 => "windows-1250",
            CharacterSet::Cp1251 => "windows-1251",
            CharacterSet::Cp1252 => "windows-1252",
            CharacterSet::Cp1256 => "windows-1256",
            CharacterSet::UTF16BE => "UTF-16BE",
            CharacterSet::UTF16LE => "UTF-16LE",
            CharacterSet::UTF8 => "UTF-8",
            CharacterSet::ASCII => "US-ASCII",
            CharacterSet::Big5 => "Big5",
            CharacterSet::GB18030 => "GB18030",
            CharacterSet::GB2312 => "GB2312",
            CharacterSet::EUC_KR => "EUC-KR",
            CharacterSet::UTF32BE => "utf-32be",
            CharacterSet::UTF32LE => "utf-32le",
            CharacterSet::Binary => "binary",
            CharacterSet::Unknown => "unknown",
        };
        encoding::label::encoding_from_whatwg_label(name).unwrap()
    }

    pub fn get_charset_name(&self) -> &'static str {
        match self {
            CharacterSet::Cp437 => "cp437",
            CharacterSet::ISO8859_1 => "iso-8859-1",
            CharacterSet::ISO8859_2 => "iso-8859-2",
            CharacterSet::ISO8859_3 => "iso-8859-3",
            CharacterSet::ISO8859_4 => "iso-8859-4",
            CharacterSet::ISO8859_5 => "iso-8859-5",
            // CharacterSet::ISO8859_6 => "ISO-8859-6",
            CharacterSet::ISO8859_7 => "iso-8859-7",
            // CharacterSet::ISO8859_8 => "ISO-8859-8",
            CharacterSet::ISO8859_9 => "iso-8859-9",
            // CharacterSet::ISO8859_10 => "ISO-8859-10",
            // CharacterSet::ISO8859_11 => "ISO-8859-11",
            CharacterSet::ISO8859_13 => "iso-8859-13",
            // CharacterSet::ISO8859_14 => "ISO-8859-14",
            CharacterSet::ISO8859_15 => "iso-8859-15",
            CharacterSet::ISO8859_16 => "iso-8859-16",
            CharacterSet::Shift_JIS => "shift_jis",
            CharacterSet::Cp1250 => "windows-1250",
            CharacterSet::Cp1251 => "windows-1251",
            CharacterSet::Cp1252 => "windows-1252",
            CharacterSet::Cp1256 => "windows-1256",
            CharacterSet::UTF16BE => "utf-16be",
            CharacterSet::UTF16LE => "utf-16le",
            CharacterSet::UTF8 => "utf-8",
            CharacterSet::ASCII => "us-ascii",
            CharacterSet::Big5 => "big5",
            CharacterSet::GB18030 => "gb18030",
            CharacterSet::GB2312 => "gb2312",
            CharacterSet::EUC_KR => "euc-kr",
            CharacterSet::UTF32BE => "utf-32be",
            CharacterSet::UTF32LE => "utf-32le",
            CharacterSet::Binary => "binary",
            CharacterSet::Unknown => "unknown",
        }
    }

    // /**
    //  * @param charset Java character set object
    //  * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
    //  *   but unsupported
    //  */
    // fn get_character_set_eci(charset: EncodingRef) -> Option<CharacterSetECI> {
    //     let name = if let Some(nm) = charset.whatwg_name() {
    //         nm
    //     } else {
    //         charset.name()
    //     };
    //     match name {
    //         "cp437" => Some(CharacterSetECI::Cp437),
    //         "iso-8859-1" => Some(CharacterSetECI::ISO8859_1),
    //         "iso-8859-2" => Some(CharacterSetECI::ISO8859_2),
    //         "iso-8859-3" => Some(CharacterSetECI::ISO8859_3),
    //         "iso-8859-4" => Some(CharacterSetECI::ISO8859_4),
    //         "iso-8859-5" => Some(CharacterSetECI::ISO8859_5),
    //         // "iso-8859-6" => Some(CharacterSetECI::ISO8859_6),
    //         "iso-8859-7" => Some(CharacterSetECI::ISO8859_7),
    //         // "iso-8859-8" => Some(CharacterSetECI::ISO8859_8),
    //         "iso-8859-9" => Some(CharacterSetECI::ISO8859_9),
    //         // "iso-8859-10" => Some(CharacterSetECI::ISO8859_10),
    //         // "iso-8859-11" => Some(CharacterSetECI::ISO8859_11),
    //         "iso-8859-13" => Some(CharacterSetECI::ISO8859_13),
    //         // "iso-8859-14" => Some(CharacterSetECI::ISO8859_14),
    //         "iso-8859-15" => Some(CharacterSetECI::ISO8859_15),
    //         "iso-8859-16" => Some(CharacterSetECI::ISO8859_16),
    //         "shift_jis" => Some(CharacterSetECI::SJIS),
    //         "windows-1250" => Some(CharacterSetECI::Cp1250),
    //         "windows-1251" => Some(CharacterSetECI::Cp1251),
    //         "windows-1252" => Some(CharacterSetECI::Cp1252),
    //         "windows-1256" => Some(CharacterSetECI::Cp1256),
    //         "utf-16be" => Some(CharacterSetECI::UnicodeBigUnmarked),
    //         "utf-8" | "utf8" => Some(CharacterSetECI::UTF8),
    //         "us-ascii" => Some(CharacterSetECI::ASCII),
    //         "big5" => Some(CharacterSetECI::Big5),
    //         "gb2312" => Some(CharacterSetECI::GB18030),
    //         "euc-kr" => Some(CharacterSetECI::EUC_KR),
    //         _ => None,
    //     }
    // }

    // /**
    //  * @param value character set ECI value
    //  * @return {@code CharacterSetECI} representing ECI of given value, or null if it is legal but
    //  *   unsupported
    //  * @throws FormatException if ECI value is invalid
    //  */
    // pub fn get_character_set_by_eci(value: u32) -> Result<CharacterSet> {
    //     match value {
    //         0 | 2 => Ok(CharacterSet::Cp437),
    //         1 | 3 => Ok(CharacterSet::ISO8859_1),
    //         4 => Ok(CharacterSet::ISO8859_2),
    //         5 => Ok(CharacterSet::ISO8859_3),
    //         6 => Ok(CharacterSet::ISO8859_4),
    //         7 => Ok(CharacterSet::ISO8859_5),
    //         // 8 => Ok(CharacterSetECI::ISO8859_6),
    //         9 => Ok(CharacterSet::ISO8859_7),
    //         // 10 => Ok(CharacterSetECI::ISO8859_8),
    //         11 => Ok(CharacterSet::ISO8859_9),
    //         // 12 => Ok(CharacterSetECI::ISO8859_10),
    //         // 13 => Ok(CharacterSetECI::ISO8859_11),
    //         15 => Ok(CharacterSet::ISO8859_13),
    //         // 16 => Ok(CharacterSetECI::ISO8859_14),
    //         17 => Ok(CharacterSet::ISO8859_15),
    //         18 => Ok(CharacterSet::ISO8859_16),
    //         20 => Ok(CharacterSet::Shift_JIS),
    //         21 => Ok(CharacterSet::Cp1250),
    //         22 => Ok(CharacterSet::Cp1251),
    //         23 => Ok(CharacterSet::Cp1252),
    //         24 => Ok(CharacterSet::Cp1256),
    //         25 => Ok(CharacterSet::UTF16BE),
    //         26 => Ok(CharacterSet::UTF8),
    //         27 => Ok(CharacterSet::ASCII),
    //         28 => Ok(CharacterSet::Big5),
    //         32 => Ok(CharacterSet::GB18030),
    //         29 => Ok(CharacterSet::GB2312),
    //         30 => Ok(CharacterSet::EUC_KR),
    //         33 => Ok(CharacterSet::UTF16LE),
    //         34 => Ok(CharacterSet::UTF32BE),
    //         35 => Ok(CharacterSet::UTF32LE),
    //         899 => Ok(CharacterSet::Binary),
    //         _ => Err(Exceptions::not_found_with("Bad ECI Value")),
    //     }
    // }

    /**
     * @param name character set ECI encoding name
     * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
     *   but unsupported
     */
    pub fn get_character_set_by_name(name: &str) -> Option<CharacterSet> {
        match name.to_lowercase().as_str() {
            "cp437" => Some(CharacterSet::Cp437),
            "iso-8859-1" => Some(CharacterSet::ISO8859_1),
            "iso-8859-2" => Some(CharacterSet::ISO8859_2),
            "iso-8859-3" => Some(CharacterSet::ISO8859_3),
            "iso-8859-4" => Some(CharacterSet::ISO8859_4),
            "iso-8859-5" => Some(CharacterSet::ISO8859_5),
            // "iso-8859-6" => Some(CharacterSet::ISO8859_6),
            "iso-8859-7" => Some(CharacterSet::ISO8859_7),
            // "iso-8859-8" => Some(CharacterSet::ISO8859_8),
            "iso-8859-9" => Some(CharacterSet::ISO8859_9),
            // "ISO-8859-10" => Some(CharacterSet::ISO8859_10),
            // "ISO-8859-11" => Some(CharacterSet::ISO8859_11),
            "iso-8859-13" => Some(CharacterSet::ISO8859_13),
            // "ISO-8859-14" => Some(CharacterSet::ISO8859_14),
            "iso-8859-15" => Some(CharacterSet::ISO8859_15),
            "iso-8859-16" => Some(CharacterSet::ISO8859_16),
            "shift_jis" => Some(CharacterSet::Shift_JIS),
            "windows-1250" => Some(CharacterSet::Cp1250),
            "windows-1251" => Some(CharacterSet::Cp1251),
            "windows-1252" => Some(CharacterSet::Cp1252),
            "windows-1256" => Some(CharacterSet::Cp1256),
            "utf-16be" => Some(CharacterSet::UTF16BE),
            "utf-8" | "utf8" => Some(CharacterSet::UTF8),
            "us-ascii" => Some(CharacterSet::ASCII),
            "big5" => Some(CharacterSet::Big5),
            "gb2312" => Some(CharacterSet::GB2312),
            "gb18030" => Some(CharacterSet::GB18030),
            "euc-kr" => Some(CharacterSet::EUC_KR),
            "utf-32be" => Some(CharacterSet::UTF32BE),
            "utf-32le" => Some(CharacterSet::UTF32LE),
            "binary" => Some(CharacterSet::Binary),
            "unknown" => Some(CharacterSet::Unknown),
            _ => None,
        }
    }

    pub fn encode(&self, input: &str) -> Result<Vec<u8>> {
        if self == &CharacterSet::Cp437 {
            use codepage_437::ToCp437;
            use codepage_437::CP437_CONTROL;

            input
                .to_cp437(&CP437_CONTROL)
                .map(|data| data.to_vec())
                .map_err(|e| Exceptions::format_with(format!("{e:?}")))
        } else {
            self.get_base_encoder()
                .encode(input, encoding::EncoderTrap::Strict)
                .map_err(|e| Exceptions::format_with(e.to_string()))
        }
    }

    pub fn encode_replace(&self, input: &str) -> Result<Vec<u8>> {
        self.get_base_encoder()
            .encode(input, encoding::EncoderTrap::Replace)
            .map_err(|e| Exceptions::format_with(e.to_string()))
    }

    pub fn decode(&self, input: &[u8]) -> Result<String> {
        if self == &CharacterSet::Cp437 {
            use codepage_437::BorrowFromCp437;
            use codepage_437::CP437_CONTROL;

            Ok(String::borrow_from_cp437(&input, &CP437_CONTROL))
        } else {
            self.get_base_encoder()
                .decode(input, encoding::DecoderTrap::Strict)
                .map_err(|e| Exceptions::format_with(e.to_string()))
        }
    }

    pub fn decode_replace(&self, input: &[u8]) -> Result<String> {
        self.get_base_encoder()
            .decode(input, encoding::DecoderTrap::Replace)
            .map_err(|e| Exceptions::format_with(e.to_string()))
    }
}
