/*
 * Copyright 2007 ZXing authors
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

use crate::common::Result;
use crate::qrcode::cpp_port::Type;
use crate::Exceptions;

use super::Version;

/**
 * <p>See ISO 18004:2006, 6.4.1, Tables 2 and 3. This enum encapsulates the various modes in which
 * data can be encoded to bits in the QR code standard.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    TERMINATOR,           //(new int[]{0, 0, 0}, 0x00), // Not really a mode...
    NUMERIC,              //(new int[]{10, 12, 14}, 0x01),
    ALPHANUMERIC,         //(new int[]{9, 11, 13}, 0x02),
    STRUCTURED_APPEND,    //(new int[]{0, 0, 0}, 0x03), // Not supported
    BYTE,                 //(new int[]{8, 16, 16}, 0x04),
    ECI,                  //(new int[]{0, 0, 0}, 0x07), // character counts don't apply
    KANJI,                //(new int[]{8, 10, 12}, 0x08),
    FNC1_FIRST_POSITION,  //(new int[]{0, 0, 0}, 0x05),
    FNC1_SECOND_POSITION, //(new int[]{0, 0, 0}, 0x09),
    /** See GBT 18284-2000; "Hanzi" is a transliteration of this mode name. */
    HANZI, //(new int[]{8, 10, 12}, 0x0D);
}
// private final int[] characterCountBitsForVersions;
// private final int bits;

// Mode(int[] characterCountBitsForVersions, int bits) {
//   this.characterCountBitsForVersions = characterCountBitsForVersions;
//   this.bits = bits;
// }

impl Mode {
    /**
     * @param bits four bits encoding a QR Code data mode
     * @return Mode encoded by these bits
     * @throws IllegalArgumentException if bits do not correspond to a known mode
     */
    pub fn forBits(bits: u8) -> Result<Self> {
        match bits {
            0x0 => Ok(Self::TERMINATOR),
            0x1 => Ok(Self::NUMERIC),
            0x2 => Ok(Self::ALPHANUMERIC),
            0x3 => Ok(Self::STRUCTURED_APPEND),
            0x4 => Ok(Self::BYTE),
            0x5 => Ok(Self::FNC1_FIRST_POSITION),
            0x7 => Ok(Self::ECI),
            0x8 => Ok(Self::KANJI),
            0x9 => Ok(Self::FNC1_SECOND_POSITION),
            0xD =>
            // 0xD is defined in GBT 18284-2000, may not be supported in foreign country
            {
                Ok(Self::HANZI)
            }
            _ => Err(Exceptions::illegal_argument_with(format!(
                "{bits} is not valid"
            ))),
        }
    }

    /**
     * @param version version in question
     * @return number of bits used, in this QR Code symbol {@link Version}, to encode the
     *         count of characters that will follow encoded in this Mode
     */
    pub fn getCharacterCountBits(&self, version: &Version) -> u8 {
        let number = version.getVersionNumber();

        let offset = if number <= 9 {
            0
        } else if number <= 26 {
            1
        } else {
            2
        };
        self.get_character_counts()[offset]
    }

    fn get_character_counts(&self) -> &[u8] {
        match self {
            Mode::TERMINATOR => &[0, 0, 0],
            Mode::NUMERIC => &[10, 12, 14],
            Mode::ALPHANUMERIC => &[9, 11, 13],
            Mode::STRUCTURED_APPEND => &[0, 0, 0],
            Mode::BYTE => &[8, 16, 16],
            Mode::ECI => &[0, 0, 0],
            Mode::KANJI => &[8, 10, 12],
            Mode::FNC1_FIRST_POSITION => &[0, 0, 0],
            Mode::FNC1_SECOND_POSITION => &[0, 0, 0],
            Mode::HANZI => &[8, 10, 12],
        }
    }

    pub fn getBits(&self) -> u8 {
        match self {
            Mode::TERMINATOR => 0x00,
            Mode::NUMERIC => 0x01,
            Mode::ALPHANUMERIC => 0x02,
            Mode::STRUCTURED_APPEND => 0x03,
            Mode::BYTE => 0x04,
            Mode::ECI => 0x07,
            Mode::KANJI => 0x08,
            Mode::FNC1_FIRST_POSITION => 0x05,
            Mode::FNC1_SECOND_POSITION => 0x09,
            Mode::HANZI => 0x0D,
        }
    }

    pub fn get_terminator_bit_length(version: &Version) -> u8 {
        (if version.isMicro() {
            version.getVersionNumber() * 2 + 1
        } else {
            4 - u32::from(version.isRMQR())
        }) as u8
    }

    pub fn get_codec_mode_bits_length(version: &Version) -> u8 {
        (if version.isMicro() {
            version.getVersionNumber() - 1
        } else {
            4 - u32::from(version.isRMQR())
        }) as u8
    }
    /**
     * @param bits variable number of bits encoding a QR Code data mode
     * @param isMicro is this a MicroQRCode
     * @return Mode encoded by these bits
     * @throws FormatError if bits do not correspond to a known mode
     */
    pub fn CodecModeForBits(bits: u32, qr_type: Option<Type>) -> Result<Self> {
        let qr_type = qr_type.unwrap_or(Type::Model2);
        let bits = bits as usize;

        if qr_type == Type::Micro {
            const BITS2MODE: [Mode; 4] =
                [Mode::NUMERIC, Mode::ALPHANUMERIC, Mode::BYTE, Mode::KANJI];
            if bits < (BITS2MODE.len()) {
                return Ok(BITS2MODE[bits]);
            }
        } else if qr_type == Type::RectMicro {
            const BITS2MODE: [Mode; 8] = [
                Mode::TERMINATOR,
                Mode::NUMERIC,
                Mode::ALPHANUMERIC,
                Mode::BYTE,
                Mode::KANJI,
                Mode::FNC1_FIRST_POSITION,
                Mode::FNC1_SECOND_POSITION,
                Mode::ECI,
            ];
            if bits < (BITS2MODE.len()) {
                return Ok(BITS2MODE[bits]);
            }
        } else if (0x00..=0x05).contains(&bits) || (0x07..=0x09).contains(&bits) || bits == 0x0d {
            return Mode::try_from(bits as u32);
        }

        Err(Exceptions::format_with("Invalid codec mode"))
    }

    /**
     * @param version version in question
     * @return number of bits used, in this QR Code symbol {@link Version}, to encode the
     *         count of characters that will follow encoded in this Mode
     */
    pub fn CharacterCountBits(&self, version: &Version) -> u32 {
        let number = version.getVersionNumber() as usize;
        if version.isMicro() {
            match self {
		 Mode::NUMERIC=>      return [3, 4, 5, 6][number - 1],
		 Mode::ALPHANUMERIC=> return [3, 4, 5][number - 2],
		 Mode::BYTE=>         return [4, 5][number - 3],
		 Mode::KANJI | //=>        [[fallthrough]],
		 Mode::HANZI=>        return [3, 4][number - 3],
		_=> return 0,
		}
        }

        if version.isRMQR() {
            // See ISO/IEC 23941:2022 7.4.1, Table 3 - Number of bits of character count indicator
            const NUMERIC: [u32; 32] = [
                4, 5, 6, 7, 7, 5, 6, 7, 7, 8, 4, 6, 7, 7, 8, 8, 5, 6, 7, 7, 8, 8, 7, 7, 8, 8, 9, 7,
                8, 8, 8, 9,
            ];
            const ALPHANUM: [u32; 32] = [
                3, 5, 5, 6, 6, 5, 5, 6, 6, 7, 4, 5, 6, 6, 7, 7, 5, 6, 6, 7, 7, 8, 6, 7, 7, 7, 8, 6,
                7, 7, 8, 8,
            ];
            const BYTE: [u32; 32] = [
                3, 4, 5, 5, 6, 4, 5, 5, 6, 6, 3, 5, 5, 6, 6, 7, 4, 5, 6, 6, 7, 7, 6, 6, 7, 7, 7, 6,
                6, 7, 7, 8,
            ];
            const KANJI: [u32; 32] = [
                2, 3, 4, 5, 5, 3, 4, 5, 5, 6, 2, 4, 5, 5, 6, 6, 3, 5, 5, 6, 6, 7, 5, 5, 6, 6, 7, 5,
                6, 6, 6, 7,
            ];
            match self {
                Mode::NUMERIC => return NUMERIC[number - 1],
                Mode::ALPHANUMERIC => return ALPHANUM[number - 1],
                Mode::BYTE => return BYTE[number - 1],
                Mode::KANJI => return KANJI[number - 1],
                _ => return 0,
            }
        }

        let i = if number <= 9 {
            0
        } else if number <= 26 {
            1
        } else {
            2
        };

        match self {
	 Mode::NUMERIC=>      [10, 12, 14][i],
	 Mode::ALPHANUMERIC=> [9, 11, 13][i],
	 Mode::BYTE=>         [8, 16, 16][i],
	 Mode::KANJI|    //    [[fallthrough]];
	 Mode::HANZI=>        [8, 10, 12][i],
	_=>                     0,
	}
    }
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        value.getBits()
    }
}

impl TryFrom<u8> for Mode {
    type Error = Exceptions;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::forBits(value)
    }
}

impl TryFrom<u32> for Mode {
    type Error = Exceptions;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::forBits(value as u8)
    }
}
