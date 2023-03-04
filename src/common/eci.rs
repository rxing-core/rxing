use std::fmt::Display;

use super::CharacterSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Eci {
    Unknown = -1,
    Cp437 = 2, // obsolete
    ISO8859_1 = 3,
    ISO8859_2 = 4,
    ISO8859_3 = 5,
    ISO8859_4 = 6,
    ISO8859_5 = 7,
    ISO8859_6 = 8,
    ISO8859_7 = 9,
    ISO8859_8 = 10,
    ISO8859_9 = 11,
    ISO8859_10 = 12,
    ISO8859_11 = 13,
    ISO8859_13 = 15,
    ISO8859_14 = 16,
    ISO8859_15 = 17,
    ISO8859_16 = 18,
    Shift_JIS = 20,
    Cp1250 = 21,
    Cp1251 = 22,
    Cp1252 = 23,
    Cp1256 = 24,
    UTF16BE = 25,
    UTF8 = 26,
    ASCII = 27,
    Big5 = 28,
    GB2312 = 29,
    EUC_KR = 30,
    GB18030 = 32,
    UTF16LE = 33,
    UTF32BE = 34,
    UTF32LE = 35,
    ISO646_Inv = 170,
    Binary = 899,
}

impl Eci {
    pub fn can_encode(self) -> bool {
        (self as i32) >= 899
    }
}

impl From<u32> for Eci {
    fn from(value: u32) -> Self {
        (value as i32).into()
    }
}

impl From<i32> for Eci {
    fn from(value: i32) -> Self {
        match value {
            0 | 2 => Eci::Cp437,
            1 | 3 => Eci::ISO8859_1,
            4 => Eci::ISO8859_2,
            5 => Eci::ISO8859_3,
            6 => Eci::ISO8859_4,
            7 => Eci::ISO8859_5,
            8 => Eci::ISO8859_6,
            9 => Eci::ISO8859_7,
            10 => Eci::ISO8859_8,
            11 => Eci::ISO8859_9,
            12 => Eci::ISO8859_10,
            13 => Eci::ISO8859_11,
            15 => Eci::ISO8859_13,
            16 => Eci::ISO8859_14,
            17 => Eci::ISO8859_15,
            18 => Eci::ISO8859_16,
            20 => Eci::Shift_JIS,
            21 => Eci::Cp1250,
            22 => Eci::Cp1251,
            23 => Eci::Cp1252,
            24 => Eci::Cp1256,
            25 => Eci::UTF16BE,
            26 => Eci::UTF8,
            27 => Eci::ASCII,
            28 => Eci::Big5,
            29 => Eci::GB18030,
            30 => Eci::EUC_KR,
            32 => Eci::GB18030,
            33 => Eci::UTF16LE,
            34 => Eci::UTF32BE,
            35 => Eci::UTF32LE,
            170 => Eci::ASCII,
            898 => Eci::Binary,
            _ => Eci::Unknown,
        }
    }
}

impl From<CharacterSet> for Eci {
    fn from(value: CharacterSet) -> Self {
        match value {
            CharacterSet::Cp437 => Eci::Cp437,
            CharacterSet::ISO8859_1 => Eci::ISO8859_1,
            CharacterSet::ISO8859_2 => Eci::ISO8859_2,
            CharacterSet::ISO8859_3 => Eci::ISO8859_3,
            CharacterSet::ISO8859_4 => Eci::ISO8859_4,
            CharacterSet::ISO8859_5 => Eci::ISO8859_5,
            CharacterSet::ISO8859_7 => Eci::ISO8859_7,
            CharacterSet::ISO8859_9 => Eci::ISO8859_9,
            CharacterSet::ISO8859_13 => Eci::ISO8859_13,
            CharacterSet::ISO8859_15 => Eci::ISO8859_15,
            CharacterSet::ISO8859_16 => Eci::ISO8859_16,
            CharacterSet::Shift_JIS => Eci::Shift_JIS,
            CharacterSet::Cp1250 => Eci::Cp1250,
            CharacterSet::Cp1251 => Eci::Cp1251,
            CharacterSet::Cp1252 => Eci::Cp1252,
            CharacterSet::Cp1256 => Eci::Cp1256,
            CharacterSet::UTF16BE => Eci::UTF16BE,
            CharacterSet::UTF8 => Eci::UTF8,
            CharacterSet::ASCII => Eci::ASCII,
            CharacterSet::Big5 => Eci::Big5,
            CharacterSet::GB2312 => Eci::GB2312,
            CharacterSet::GB18030 => Eci::GB18030,
            CharacterSet::EUC_KR => Eci::EUC_KR,
            CharacterSet::UTF16LE => Eci::UTF16LE,
            CharacterSet::UTF32BE => Eci::UTF32BE,
            CharacterSet::UTF32LE => Eci::UTF32LE,
            CharacterSet::Binary => Eci::Binary,
            CharacterSet::ISO8859_6 => Eci::ISO8859_6,
            CharacterSet::ISO8859_8 => Eci::ISO8859_8,
            CharacterSet::ISO8859_10 => Eci::ISO8859_10,
            CharacterSet::ISO8859_11 => Eci::ISO8859_11,
            CharacterSet::ISO8859_14 => Eci::ISO8859_14,
            _ => Eci::Unknown,
        }
    }
}

impl From<Eci> for CharacterSet {
    fn from(value: Eci) -> Self {
        match value {
            Eci::Cp437 => CharacterSet::Cp437,
            Eci::ISO8859_1 => CharacterSet::ISO8859_1,
            Eci::ISO8859_2 => CharacterSet::ISO8859_2,
            Eci::ISO8859_3 => CharacterSet::ISO8859_3,
            Eci::ISO8859_4 => CharacterSet::ISO8859_4,
            Eci::ISO8859_5 => CharacterSet::ISO8859_5,
            Eci::ISO8859_6 => CharacterSet::ISO8859_6,
            Eci::ISO8859_7 => CharacterSet::ISO8859_7,
            Eci::ISO8859_8 => CharacterSet::ISO8859_8,
            Eci::ISO8859_9 => CharacterSet::ISO8859_9,
            Eci::ISO8859_10 => CharacterSet::ISO8859_10,
            Eci::ISO8859_11 => CharacterSet::ISO8859_11,
            Eci::ISO8859_13 => CharacterSet::ISO8859_13,
            Eci::ISO8859_14 => CharacterSet::ISO8859_14,
            Eci::ISO8859_15 => CharacterSet::ISO8859_15,
            Eci::ISO8859_16 => CharacterSet::ISO8859_16,
            Eci::Shift_JIS => CharacterSet::Shift_JIS,
            Eci::Cp1250 => CharacterSet::Cp1250,
            Eci::Cp1251 => CharacterSet::Cp1251,
            Eci::Cp1252 => CharacterSet::Cp1252,
            Eci::Cp1256 => CharacterSet::Cp1256,
            Eci::UTF16BE => CharacterSet::UTF16BE,
            Eci::UTF8 => CharacterSet::UTF8,
            Eci::ASCII => CharacterSet::ASCII,
            Eci::Big5 => CharacterSet::Big5,
            Eci::GB2312 => CharacterSet::GB2312,
            Eci::EUC_KR => CharacterSet::EUC_KR,
            Eci::GB18030 => CharacterSet::GB18030,
            Eci::UTF16LE => CharacterSet::UTF16LE,
            Eci::UTF32BE => CharacterSet::UTF32BE,
            Eci::UTF32LE => CharacterSet::UTF32LE,
            Eci::ISO646_Inv => CharacterSet::ASCII,
            Eci::Binary => CharacterSet::Binary,
            _ => CharacterSet::Unknown,
        }
    }
}

impl Display for Eci {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}
