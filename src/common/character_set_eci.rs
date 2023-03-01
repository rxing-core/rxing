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

// package com.google.zxing.common;

// import com.google.zxing.FormatException;

// import java.nio.charset.Charset;

// import java.util.HashMap;
// import java.util.Map;

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
pub enum CharacterSetECI {
    // Enum name is a Java encoding valid for java.lang and java.io
    Cp437,     //(new int[]{0,2}),
    ISO8859_1, //(new int[]{1,3}, "ISO-8859-1"),
    ISO8859_2, //(4, "ISO-8859-2"),
    ISO8859_3, //(5, "ISO-8859-3"),
    ISO8859_4, //(6, "ISO-8859-4"),
    ISO8859_5, //(7, "ISO-8859-5"),
    //ISO8859_6,          //(8, "ISO-8859-6"),
    ISO8859_7, //(9, "ISO-8859-7"),
    //ISO8859_8,          //(10, "ISO-8859-8"),
    ISO8859_9, //(11, "ISO-8859-9"),
    // ISO8859_10,         //(12, "ISO-8859-10"),
    // ISO8859_11,         //(13, "ISO-8859-11"),
    ISO8859_13, //(15, "ISO-8859-13"),
    // ISO8859_14,         //(16, "ISO-8859-14"),
    ISO8859_15,         //(17, "ISO-8859-15"),
    ISO8859_16,         //(18, "ISO-8859-16"),
    SJIS,               //(20, "Shift_JIS"),
    Cp1250,             //(21, "windows-1250"),
    Cp1251,             //(22, "windows-1251"),
    Cp1252,             //(23, "windows-1252"),
    Cp1256,             //(24, "windows-1256"),
    UnicodeBigUnmarked, //(25, "UTF-16BE", "UnicodeBig"),
    UTF8,               //(26, "UTF-8"),
    ASCII,              //(new int[] {27, 170}, "US-ASCII"),
    Big5,               //(28),
    GB18030,            //(29, "GB2312", "EUC_CN", "GBK"),
    EUC_KR,             //(30, "EUC-KR");
}
impl CharacterSetECI {
    //   private static final Map<Integer,CharacterSetECI> VALUE_TO_ECI = new HashMap<>();
    //   private static final Map<String,CharacterSetECI> NAME_TO_ECI = new HashMap<>();
    //   static {
    //     for (CharacterSetECI eci : values()) {
    //       for (int value : eci.values) {
    //         VALUE_TO_ECI.put(value, eci);
    //       }
    //       NAME_TO_ECI.put(eci.name(), eci);
    //       for (String name : eci.otherEncodingNames) {
    //         NAME_TO_ECI.put(name, eci);
    //       }
    //     }
    //   }

    //   private final int[] values;
    //   private final String[] otherEncodingNames;

    //   CharacterSetECI(int value) {
    //     this(new int[] {value});
    //   }

    //   CharacterSetECI(int value, String... otherEncodingNames) {
    //     this.values = new int[] {value};
    //     this.otherEncodingNames = otherEncodingNames;
    //   }

    //   CharacterSetECI(int[] values, String... otherEncodingNames) {
    //     this.values = values;
    //     this.otherEncodingNames = otherEncodingNames;
    //   }

    pub fn getValueSelf(&self) -> u32 {
        Self::getValue(self)
    }

    pub fn getValue(cs_eci: &CharacterSetECI) -> u32 {
        match cs_eci {
            CharacterSetECI::Cp437 => 0,
            CharacterSetECI::ISO8859_1 => 1,
            CharacterSetECI::ISO8859_2 => 4,
            CharacterSetECI::ISO8859_3 => 5,
            CharacterSetECI::ISO8859_4 => 6,
            CharacterSetECI::ISO8859_5 => 7,
            // CharacterSetECI::ISO8859_6 => 8,
            CharacterSetECI::ISO8859_7 => 9,
            // CharacterSetECI::ISO8859_8 => 10,
            CharacterSetECI::ISO8859_9 => 11,
            // CharacterSetECI::ISO8859_10 => 12,
            // CharacterSetECI::ISO8859_11 => 13,
            CharacterSetECI::ISO8859_13 => 15,
            // CharacterSetECI::ISO8859_14 => 16,
            CharacterSetECI::ISO8859_15 => 17,
            CharacterSetECI::ISO8859_16 => 18,
            CharacterSetECI::SJIS => 20,
            CharacterSetECI::Cp1250 => 21,
            CharacterSetECI::Cp1251 => 22,
            CharacterSetECI::Cp1252 => 23,
            CharacterSetECI::Cp1256 => 24,
            CharacterSetECI::UnicodeBigUnmarked => 25,
            CharacterSetECI::UTF8 => 26,
            CharacterSetECI::ASCII => 27,
            CharacterSetECI::Big5 => 28,
            CharacterSetECI::GB18030 => 29,
            CharacterSetECI::EUC_KR => 30,
        }
    }

    pub fn getCharset(cs_eci: &CharacterSetECI) -> EncodingRef {
        let name = match cs_eci {
            // CharacterSetECI::Cp437 => "CP437",
            CharacterSetECI::Cp437 => "UTF-8",
            CharacterSetECI::ISO8859_1 => "ISO-8859-1",
            CharacterSetECI::ISO8859_2 => "ISO-8859-2",
            CharacterSetECI::ISO8859_3 => "ISO-8859-3",
            CharacterSetECI::ISO8859_4 => "ISO-8859-4",
            CharacterSetECI::ISO8859_5 => "ISO-8859-5",
            // CharacterSetECI::ISO8859_6 => "ISO-8859-6",
            CharacterSetECI::ISO8859_7 => "ISO-8859-7",
            // CharacterSetECI::ISO8859_8 => "ISO-8859-8",
            CharacterSetECI::ISO8859_9 => "ISO-8859-9",
            // CharacterSetECI::ISO8859_10 => "ISO-8859-10",
            // CharacterSetECI::ISO8859_11 => "ISO-8859-11",
            CharacterSetECI::ISO8859_13 => "ISO-8859-13",
            // CharacterSetECI::ISO8859_14 => "ISO-8859-14",
            CharacterSetECI::ISO8859_15 => "ISO-8859-15",
            CharacterSetECI::ISO8859_16 => "ISO-8859-16",
            CharacterSetECI::SJIS => "Shift_JIS",
            CharacterSetECI::Cp1250 => "windows-1250",
            CharacterSetECI::Cp1251 => "windows-1251",
            CharacterSetECI::Cp1252 => "windows-1252",
            CharacterSetECI::Cp1256 => "windows-1256",
            CharacterSetECI::UnicodeBigUnmarked => "UTF-16BE",
            CharacterSetECI::UTF8 => "UTF-8",
            CharacterSetECI::ASCII => "US-ASCII",
            CharacterSetECI::Big5 => "Big5",
            CharacterSetECI::GB18030 => "GB2312",
            CharacterSetECI::EUC_KR => "EUC-KR",
        };
        encoding::label::encoding_from_whatwg_label(name).unwrap()
    }

    /**
     * @param charset Java character set object
     * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
     *   but unsupported
     */
    pub fn getCharacterSetECI(charset: EncodingRef) -> Option<CharacterSetECI> {
        let name = if let Some(nm) = charset.whatwg_name() {
            nm
        } else {
            charset.name()
        };
        match name {
            "CP437" => Some(CharacterSetECI::Cp437),
            "iso-8859-1" => Some(CharacterSetECI::ISO8859_1),
            "iso-8859-2" => Some(CharacterSetECI::ISO8859_2),
            "iso-8859-3" => Some(CharacterSetECI::ISO8859_3),
            "iso-8859-4" => Some(CharacterSetECI::ISO8859_4),
            "iso-8859-5" => Some(CharacterSetECI::ISO8859_5),
            // "iso-8859-6" => Some(CharacterSetECI::ISO8859_6),
            "iso-8859-7" => Some(CharacterSetECI::ISO8859_7),
            // "iso-8859-8" => Some(CharacterSetECI::ISO8859_8),
            "iso-8859-9" => Some(CharacterSetECI::ISO8859_9),
            // "iso-8859-10" => Some(CharacterSetECI::ISO8859_10),
            // "iso-8859-11" => Some(CharacterSetECI::ISO8859_11),
            "iso-8859-13" => Some(CharacterSetECI::ISO8859_13),
            // "iso-8859-14" => Some(CharacterSetECI::ISO8859_14),
            "iso-8859-15" => Some(CharacterSetECI::ISO8859_15),
            "iso-8859-16" => Some(CharacterSetECI::ISO8859_16),
            "shift_jis" => Some(CharacterSetECI::SJIS),
            "windows-1250" => Some(CharacterSetECI::Cp1250),
            "windows-1251" => Some(CharacterSetECI::Cp1251),
            "windows-1252" => Some(CharacterSetECI::Cp1252),
            "windows-1256" => Some(CharacterSetECI::Cp1256),
            "utf-16be" => Some(CharacterSetECI::UnicodeBigUnmarked),
            "utf-8" => Some(CharacterSetECI::UTF8),
            "us-ascii" => Some(CharacterSetECI::ASCII),
            "big5" => Some(CharacterSetECI::Big5),
            "gb2312" => Some(CharacterSetECI::GB18030),
            "euc-kr" => Some(CharacterSetECI::EUC_KR),
            _ => None,
        }
    }

    /**
     * @param value character set ECI value
     * @return {@code CharacterSetECI} representing ECI of given value, or null if it is legal but
     *   unsupported
     * @throws FormatException if ECI value is invalid
     */
    pub fn getCharacterSetECIByValue(value: u32) -> Result<CharacterSetECI> {
        match value {
            0 | 2 => Ok(CharacterSetECI::Cp437),
            1 | 3 => Ok(CharacterSetECI::ISO8859_1),
            4 => Ok(CharacterSetECI::ISO8859_2),
            5 => Ok(CharacterSetECI::ISO8859_3),
            6 => Ok(CharacterSetECI::ISO8859_4),
            7 => Ok(CharacterSetECI::ISO8859_5),
            // 8 => Ok(CharacterSetECI::ISO8859_6),
            9 => Ok(CharacterSetECI::ISO8859_7),
            // 10 => Ok(CharacterSetECI::ISO8859_8),
            11 => Ok(CharacterSetECI::ISO8859_9),
            // 12 => Ok(CharacterSetECI::ISO8859_10),
            // 13 => Ok(CharacterSetECI::ISO8859_11),
            15 => Ok(CharacterSetECI::ISO8859_13),
            // 16 => Ok(CharacterSetECI::ISO8859_14),
            17 => Ok(CharacterSetECI::ISO8859_15),
            18 => Ok(CharacterSetECI::ISO8859_16),
            20 => Ok(CharacterSetECI::SJIS),
            21 => Ok(CharacterSetECI::Cp1250),
            22 => Ok(CharacterSetECI::Cp1251),
            23 => Ok(CharacterSetECI::Cp1252),
            24 => Ok(CharacterSetECI::Cp1256),
            25 => Ok(CharacterSetECI::UnicodeBigUnmarked),
            26 => Ok(CharacterSetECI::UTF8),
            27 | 170 => Ok(CharacterSetECI::ASCII),
            28 => Ok(CharacterSetECI::Big5),
            29 => Ok(CharacterSetECI::GB18030),
            30 => Ok(CharacterSetECI::EUC_KR),
            _ => Err(Exceptions::not_found_with("Bad ECI Value")),
        }
    }

    /**
     * @param name character set ECI encoding name
     * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
     *   but unsupported
     */
    pub fn getCharacterSetECIByName(name: &str) -> Option<CharacterSetECI> {
        match name {
            "CP437" => Some(CharacterSetECI::Cp437),
            "ISO-8859-1" => Some(CharacterSetECI::ISO8859_1),
            "ISO-8859-2" => Some(CharacterSetECI::ISO8859_2),
            "ISO-8859-3" => Some(CharacterSetECI::ISO8859_3),
            "ISO-8859-4" => Some(CharacterSetECI::ISO8859_4),
            "ISO-8859-5" => Some(CharacterSetECI::ISO8859_5),
            // "ISO-8859-6" => Some(CharacterSetECI::ISO8859_6),
            "ISO-8859-7" => Some(CharacterSetECI::ISO8859_7),
            // "ISO-8859-8" => Some(CharacterSetECI::ISO8859_8),
            "ISO-8859-9" => Some(CharacterSetECI::ISO8859_9),
            // "ISO-8859-10" => Some(CharacterSetECI::ISO8859_10),
            // "ISO-8859-11" => Some(CharacterSetECI::ISO8859_11),
            "ISO-8859-13" => Some(CharacterSetECI::ISO8859_13),
            // "ISO-8859-14" => Some(CharacterSetECI::ISO8859_14),
            "ISO-8859-15" => Some(CharacterSetECI::ISO8859_15),
            "ISO-8859-16" => Some(CharacterSetECI::ISO8859_16),
            "Shift_JIS" => Some(CharacterSetECI::SJIS),
            "windows-1250" => Some(CharacterSetECI::Cp1250),
            "windows-1251" => Some(CharacterSetECI::Cp1251),
            "windows-1252" => Some(CharacterSetECI::Cp1252),
            "windows-1256" => Some(CharacterSetECI::Cp1256),
            "UTF-16BE" => Some(CharacterSetECI::UnicodeBigUnmarked),
            "UTF-8" => Some(CharacterSetECI::UTF8),
            "US-ASCII" => Some(CharacterSetECI::ASCII),
            "Big5" => Some(CharacterSetECI::Big5),
            "GB2312" => Some(CharacterSetECI::GB18030),
            "EUC-KR" => Some(CharacterSetECI::EUC_KR),
            _ => None,
        }
    }
}
