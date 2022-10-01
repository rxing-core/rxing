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

use crate::Exceptions;

use super::Version;

/**
 * <p>See ISO 18004:2006, 6.4.1, Tables 2 and 3. This enum encapsulates the various modes in which
 * data can be encoded to bits in the QR code standard.</p>
 *
 * @author Sean Owen
 */
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
    pub fn forBits(bits: u8) -> Result<Self, Exceptions> {
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
            _ => Err(Exceptions::IllegalArgumentException(format!(
                "{} is not valid",
                bits
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
