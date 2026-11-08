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

use crate::Exceptions;
use crate::common::Result;

#[cfg(all(
    not(feature = "encoding_rs"),
    not(feature = "legacy_encoding"),
    not(feature = "no_character_set_support")
))]
compile_error!(
    "Either feature 'encoding_rs' or 'legacy_encoding' must be enabled for CharacterSet support. If your intention is to not include CharacterSet support, please enable the 'no_character_set_support' feature."
);

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
    ISO8859_6,  //(8, "ISO-8859-6"),
    ISO8859_7,  //(9, "ISO-8859-7"),
    ISO8859_8,  //(10, "ISO-8859-8"),
    ISO8859_9,  //(11, "ISO-8859-9"),
    ISO8859_10, //(12, "ISO-8859-10"),
    ISO8859_11, //(13, "ISO-8859-11"),
    ISO8859_13, //(15, "ISO-8859-13"),
    ISO8859_14, //(16, "ISO-8859-14"),
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
    pub const fn get_charset_name(&self) -> &'static str {
        match self {
            CharacterSet::Cp437 => "cp437",
            CharacterSet::ISO8859_1 => "iso-8859-1",
            CharacterSet::ISO8859_2 => "iso-8859-2",
            CharacterSet::ISO8859_3 => "iso-8859-3",
            CharacterSet::ISO8859_4 => "iso-8859-4",
            CharacterSet::ISO8859_5 => "iso-8859-5",
            CharacterSet::ISO8859_6 => "iso-8859-6",
            CharacterSet::ISO8859_7 => "iso-8859-7",
            CharacterSet::ISO8859_8 => "iso-8859-8",
            CharacterSet::ISO8859_9 => "iso-8859-9",
            CharacterSet::ISO8859_10 => "iso-8859-10",
            CharacterSet::ISO8859_11 => "iso-8859-11",
            CharacterSet::ISO8859_13 => "iso-8859-13",
            CharacterSet::ISO8859_14 => "iso-8859-14",
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

    pub fn get_character_set_by_name(name: &str) -> Option<CharacterSet> {
        match name.to_lowercase().as_str() {
            "cp437" => Some(CharacterSet::Cp437),
            "iso-8859-1" => Some(CharacterSet::ISO8859_1),
            "iso-8859-2" => Some(CharacterSet::ISO8859_2),
            "iso-8859-3" => Some(CharacterSet::ISO8859_3),
            "iso-8859-4" => Some(CharacterSet::ISO8859_4),
            "iso-8859-5" => Some(CharacterSet::ISO8859_5),
            "iso-8859-6" => Some(CharacterSet::ISO8859_6),
            "iso-8859-7" => Some(CharacterSet::ISO8859_7),
            "iso-8859-8" => Some(CharacterSet::ISO8859_8),
            "iso-8859-9" => Some(CharacterSet::ISO8859_9),
            "iso-8859-10" => Some(CharacterSet::ISO8859_10),
            "iso-8859-11" => Some(CharacterSet::ISO8859_11),
            "iso-8859-13" => Some(CharacterSet::ISO8859_13),
            "iso-8859-14" => Some(CharacterSet::ISO8859_14),
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
}

// MODERN IMPLEMENTATION (encoding_rs)
#[cfg(all(feature = "encoding_rs", not(feature = "no_character_set_support")))]
impl CharacterSet {
    fn get_encoding(&self) -> Option<&'static encoding_rs::Encoding> {
        match self {
            CharacterSet::ISO8859_2 => Some(encoding_rs::ISO_8859_2),
            CharacterSet::ISO8859_3 => Some(encoding_rs::ISO_8859_3),
            CharacterSet::ISO8859_4 => Some(encoding_rs::ISO_8859_4),
            CharacterSet::ISO8859_5 => Some(encoding_rs::ISO_8859_5),
            CharacterSet::ISO8859_6 => Some(encoding_rs::ISO_8859_6),
            CharacterSet::ISO8859_7 => Some(encoding_rs::ISO_8859_7),
            CharacterSet::ISO8859_8 => Some(encoding_rs::ISO_8859_8),
            CharacterSet::ISO8859_9 => Some(encoding_rs::WINDOWS_1254),
            CharacterSet::ISO8859_10 => Some(encoding_rs::ISO_8859_10),
            CharacterSet::ISO8859_11 => Some(encoding_rs::WINDOWS_874),
            CharacterSet::ISO8859_13 => Some(encoding_rs::ISO_8859_13),
            CharacterSet::ISO8859_14 => Some(encoding_rs::ISO_8859_14),
            CharacterSet::ISO8859_15 => Some(encoding_rs::ISO_8859_15),
            CharacterSet::ISO8859_16 => Some(encoding_rs::ISO_8859_16),
            CharacterSet::Shift_JIS => Some(encoding_rs::SHIFT_JIS),
            CharacterSet::Cp1250 => Some(encoding_rs::WINDOWS_1250),
            CharacterSet::Cp1251 => Some(encoding_rs::WINDOWS_1251),
            CharacterSet::Cp1252 => Some(encoding_rs::WINDOWS_1252),
            CharacterSet::Cp1256 => Some(encoding_rs::WINDOWS_1256),
            CharacterSet::UTF8 => Some(encoding_rs::UTF_8),
            CharacterSet::ASCII => encoding_rs::Encoding::for_label(b"ascii"),
            CharacterSet::Big5 => Some(encoding_rs::BIG5),
            CharacterSet::GB18030 => Some(encoding_rs::GB18030),
            CharacterSet::GB2312 => Some(encoding_rs::GBK),
            CharacterSet::EUC_KR => Some(encoding_rs::EUC_KR),
            CharacterSet::UTF16BE => Some(encoding_rs::UTF_16BE),
            CharacterSet::UTF16LE => Some(encoding_rs::UTF_16LE),
            _ => None,
        }
    }

    pub fn encode(&self, input: &str) -> Result<Vec<u8>> {
        match self {
            CharacterSet::Cp437 => {
                use codepage_437::CP437_CONTROL;
                use codepage_437::ToCp437;

                input
                    .to_cp437(&CP437_CONTROL)
                    .map(|data| data.to_vec())
                    .map_err(|e| Exceptions::format_with(format!("{e:?}")))
            }
            CharacterSet::UTF16BE => Ok(table_free::encode_utf16(input, true)),
            CharacterSet::UTF16LE => Ok(table_free::encode_utf16(input, false)),
            CharacterSet::UTF32BE => Ok(table_free::encode_utf32(input, true)),
            CharacterSet::UTF32LE => Ok(table_free::encode_utf32(input, false)),
            CharacterSet::Binary | CharacterSet::ISO8859_1 => {
                table_free::encode_latin1(input, false)
            }
            CharacterSet::ASCII => table_free::encode_ascii(input, false),
            _ => {
                if let Some(enc) = self.get_encoding() {
                    let (res, _, had_errors) = enc.encode(input);
                    if had_errors {
                        return Err(Exceptions::format_with("Could not encode character"));
                    }
                    Ok(res.into_owned())
                } else {
                    Err(Exceptions::format_with("Unsupported encoding"))
                }
            }
        }
    }

    pub fn encode_replace(&self, input: &str) -> Result<Vec<u8>> {
        match self {
            CharacterSet::Cp437
            | CharacterSet::UTF16BE
            | CharacterSet::UTF16LE
            | CharacterSet::UTF32BE
            | CharacterSet::UTF32LE => self.encode(input),
            CharacterSet::Binary | CharacterSet::ISO8859_1 => {
                table_free::encode_latin1(input, true)
            }
            CharacterSet::ASCII => table_free::encode_ascii(input, true),
            _ => {
                if let Some(enc) = self.get_encoding() {
                    let (res, _, _) = enc.encode(input);
                    Ok(res.into_owned())
                } else {
                    Err(Exceptions::format_with("Unsupported encoding"))
                }
            }
        }
    }

    pub fn decode(&self, input: &[u8]) -> Result<String> {
        match self {
            CharacterSet::Cp437 => {
                use codepage_437::BorrowFromCp437;
                use codepage_437::CP437_CONTROL;

                Ok(String::borrow_from_cp437(input, &CP437_CONTROL))
            }
            CharacterSet::UTF32BE => table_free::decode_utf32(input, true, false),
            CharacterSet::UTF32LE => table_free::decode_utf32(input, false, false),
            CharacterSet::Binary | CharacterSet::ISO8859_1 => Ok(table_free::decode_latin1(input)),
            CharacterSet::ASCII => table_free::decode_ascii(input, false),
            _ => {
                if let Some(enc) = self.get_encoding() {
                    let (res, _, had_errors) = enc.decode(input);
                    if had_errors {
                        return Err(Exceptions::format_with("Could not decode character"));
                    }
                    Ok(res.into_owned())
                } else {
                    Err(Exceptions::format_with("Unsupported encoding"))
                }
            }
        }
    }

    pub fn decode_replace(&self, input: &[u8]) -> Result<String> {
        match self {
            CharacterSet::Cp437 | CharacterSet::Binary | CharacterSet::ISO8859_1 => {
                self.decode(input)
            }
            CharacterSet::ASCII => table_free::decode_ascii(input, true),
            CharacterSet::UTF32BE => table_free::decode_utf32(input, true, true),
            CharacterSet::UTF32LE => table_free::decode_utf32(input, false, true),
            _ => {
                if let Some(enc) = self.get_encoding() {
                    let (res, _, _) = enc.decode(input);
                    Ok(res.into_owned())
                } else {
                    Err(Exceptions::format_with("Unsupported encoding"))
                }
            }
        }
    }
}

// LEGACY IMPLEMENTATION (encoding)
#[cfg(all(
    not(feature = "encoding_rs"),
    feature = "legacy_encoding",
    not(feature = "no_character_set_support")
))]
impl CharacterSet {
    fn get_base_encoder(&self) -> encoding::EncodingRef {
        let name = match self {
            CharacterSet::Cp437 => "cp437",
            CharacterSet::ISO8859_1 => return encoding::all::ISO_8859_1,
            CharacterSet::ISO8859_2 => "ISO-8859-2",
            CharacterSet::ISO8859_3 => "ISO-8859-3",
            CharacterSet::ISO8859_4 => "ISO-8859-4",
            CharacterSet::ISO8859_5 => "ISO-8859-5",
            CharacterSet::ISO8859_6 => "ISO-8859-6",
            CharacterSet::ISO8859_7 => "ISO-8859-7",
            CharacterSet::ISO8859_8 => "ISO-8859-8",
            CharacterSet::ISO8859_9 => "ISO-8859-9",
            CharacterSet::ISO8859_10 => "ISO-8859-10",
            CharacterSet::ISO8859_11 => "ISO-8859-11",
            CharacterSet::ISO8859_13 => "ISO-8859-13",
            CharacterSet::ISO8859_14 => "ISO-8859-14",
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

    pub fn encode(&self, input: &str) -> Result<Vec<u8>> {
        if self == &CharacterSet::Cp437 {
            use codepage_437::CP437_CONTROL;
            use codepage_437::ToCp437;

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

            Ok(String::borrow_from_cp437(input, &CP437_CONTROL))
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

// NO CHARACTER SET SUPPORT
// Only the sets the standard library converts without lookup tables: UTF-8,
// UTF-16, UTF-32, US-ASCII and ISO-8859-1 (also used for Binary). Every other
// set returns an error, which a reader treats like any other undecodable
// symbol, rather than panicking.
#[cfg(feature = "no_character_set_support")]
impl CharacterSet {
    pub fn encode(&self, input: &str) -> Result<Vec<u8>> {
        self.encode_table_free(input, false)
    }

    pub fn encode_replace(&self, input: &str) -> Result<Vec<u8>> {
        self.encode_table_free(input, true)
    }

    pub fn decode(&self, input: &[u8]) -> Result<String> {
        self.decode_table_free(input, false)
    }

    pub fn decode_replace(&self, input: &[u8]) -> Result<String> {
        self.decode_table_free(input, true)
    }

    fn encode_table_free(&self, input: &str, replace: bool) -> Result<Vec<u8>> {
        match self {
            CharacterSet::UTF8 => Ok(input.as_bytes().to_vec()),
            CharacterSet::UTF16BE => Ok(table_free::encode_utf16(input, true)),
            CharacterSet::UTF16LE => Ok(table_free::encode_utf16(input, false)),
            CharacterSet::UTF32BE => Ok(table_free::encode_utf32(input, true)),
            CharacterSet::UTF32LE => Ok(table_free::encode_utf32(input, false)),
            CharacterSet::Binary | CharacterSet::ISO8859_1 => {
                table_free::encode_latin1(input, replace)
            }
            CharacterSet::ASCII => table_free::encode_ascii(input, replace),
            _ => Err(self.needs_tables()),
        }
    }

    fn decode_table_free(&self, input: &[u8], replace: bool) -> Result<String> {
        match self {
            CharacterSet::UTF8 => table_free::decode_utf8(input, replace),
            CharacterSet::UTF16BE => table_free::decode_utf16(input, true, replace),
            CharacterSet::UTF16LE => table_free::decode_utf16(input, false, replace),
            CharacterSet::UTF32BE => table_free::decode_utf32(input, true, replace),
            CharacterSet::UTF32LE => table_free::decode_utf32(input, false, replace),
            CharacterSet::Binary | CharacterSet::ISO8859_1 => Ok(table_free::decode_latin1(input)),
            CharacterSet::ASCII => table_free::decode_ascii(input, replace),
            _ => Err(self.needs_tables()),
        }
    }

    fn needs_tables(&self) -> Exceptions {
        Exceptions::unsupported_operation_with(format!(
            "{} needs the encoding_rs or legacy_encoding feature",
            self.get_charset_name()
        ))
    }
}

/// Conversions that need no lookup tables. The `encoding_rs` backend uses
/// them for these sets, and they are all `no_character_set_support` offers.
#[cfg(any(feature = "encoding_rs", feature = "no_character_set_support"))]
mod table_free {
    use crate::Exceptions;
    use crate::common::Result;

    const REPLACEMENT_BYTE: u8 = b'?';

    pub(super) fn encode_latin1(input: &str, replace: bool) -> Result<Vec<u8>> {
        encode_single_byte(
            input,
            0xFF,
            replace,
            "Binary/ISO-8859-1 encoding only supports characters up to U+00FF",
        )
    }

    pub(super) fn encode_ascii(input: &str, replace: bool) -> Result<Vec<u8>> {
        encode_single_byte(
            input,
            0x7F,
            replace,
            "ASCII encoding only supports characters up to U+007F",
        )
    }

    fn encode_single_byte(input: &str, max: u8, replace: bool, error: &str) -> Result<Vec<u8>> {
        input
            .chars()
            .map(|c| match u8::try_from(c) {
                Ok(b) if b <= max => Ok(b),
                _ if replace => Ok(REPLACEMENT_BYTE),
                _ => Err(Exceptions::format_with(error)),
            })
            .collect()
    }

    pub(super) fn encode_utf16(input: &str, big_endian: bool) -> Vec<u8> {
        input
            .encode_utf16()
            .flat_map(|u| {
                if big_endian {
                    u.to_be_bytes()
                } else {
                    u.to_le_bytes()
                }
            })
            .collect()
    }

    pub(super) fn encode_utf32(input: &str, big_endian: bool) -> Vec<u8> {
        input
            .chars()
            .map(u32::from)
            .flat_map(|u| {
                if big_endian {
                    u.to_be_bytes()
                } else {
                    u.to_le_bytes()
                }
            })
            .collect()
    }

    pub(super) fn decode_latin1(input: &[u8]) -> String {
        input.iter().map(|&b| char::from(b)).collect()
    }

    pub(super) fn decode_ascii(input: &[u8], replace: bool) -> Result<String> {
        input
            .iter()
            .map(|&b| match b {
                0..=0x7F => Ok(char::from(b)),
                _ if replace => Ok(char::REPLACEMENT_CHARACTER),
                _ => Err(Exceptions::format_with("Invalid ASCII")),
            })
            .collect()
    }

    /// Like `encoding_rs`, a leading byte order mark is not part of the text.
    #[cfg(feature = "no_character_set_support")]
    pub(super) fn decode_utf8(input: &[u8], replace: bool) -> Result<String> {
        let input = input.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(input);
        if replace {
            return Ok(String::from_utf8_lossy(input).into_owned());
        }
        String::from_utf8(input.to_vec()).map_err(|_| Exceptions::format_with("Invalid UTF-8"))
    }

    /// A trailing odd byte is malformed, like an unpaired surrogate.
    #[cfg(feature = "no_character_set_support")]
    pub(super) fn decode_utf16(input: &[u8], big_endian: bool, replace: bool) -> Result<String> {
        let pairs = input.chunks_exact(2);
        let odd_byte = !pairs.remainder().is_empty();
        let units = pairs.map(|pair| {
            let pair = [pair[0], pair[1]];
            if big_endian {
                u16::from_be_bytes(pair)
            } else {
                u16::from_le_bytes(pair)
            }
        });
        let mut text = char::decode_utf16(units)
            .map(|unit| match unit {
                Ok(c) => Ok(c),
                Err(_) if replace => Ok(char::REPLACEMENT_CHARACTER),
                Err(_) => Err(Exceptions::format_with("Invalid UTF-16")),
            })
            .collect::<Result<String>>()?;
        if odd_byte {
            if !replace {
                return Err(Exceptions::format_with("Invalid UTF-16"));
            }
            text.push(char::REPLACEMENT_CHARACTER);
        }
        Ok(text)
    }

    /// Trailing bytes short of a whole code unit are ignored.
    pub(super) fn decode_utf32(input: &[u8], big_endian: bool, replace: bool) -> Result<String> {
        input
            .chunks_exact(4)
            .map(|quad| {
                let quad = [quad[0], quad[1], quad[2], quad[3]];
                let value = if big_endian {
                    u32::from_be_bytes(quad)
                } else {
                    u32::from_le_bytes(quad)
                };
                match char::from_u32(value) {
                    Some(c) => Ok(c),
                    None if replace => Ok(char::REPLACEMENT_CHARACTER),
                    None => Err(Exceptions::format_with(if big_endian {
                        "Invalid UTF-32BE"
                    } else {
                        "Invalid UTF-32LE"
                    })),
                }
            })
            .collect()
    }
}
