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

use std::str::FromStr;

use crate::Exceptions;

/**
 * <p>See ISO 18004:2006, 6.5.1. This enum encapsulates the four error correction levels
 * defined by the QR code standard.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ErrorCorrectionLevel {
    /** L = ~7% correction */
    L, //0x01
    /** M = ~15% correction */
    M, //0x00
    /** Q = ~25% correction */
    Q, //0x03
    /** H = ~30% correction */
    H, //0x02
}

impl ErrorCorrectionLevel {
    /**
     * @param bits int containing the two bits encoding a QR Code's error correction level
     * @return ErrorCorrectionLevel representing the encoded error correction level
     */
    pub fn forBits(bits: u8) -> Result<Self, Exceptions> {
        match bits {
            0 => Ok(Self::M),
            1 => Ok(Self::L),
            2 => Ok(Self::H),
            3 => Ok(Self::Q),
            _ => Err(Exceptions::illegalArgumentWith(format!(
                "{bits} is not a valid bit selection"
            ))),
        }
    }

    pub fn get_value(&self) -> u8 {
        match self {
            ErrorCorrectionLevel::L => 0x01,
            ErrorCorrectionLevel::M => 0x00,
            ErrorCorrectionLevel::Q => 0x03,
            ErrorCorrectionLevel::H => 0x02,
        }
    }

    pub fn get_ordinal(&self) -> u8 {
        match self {
            ErrorCorrectionLevel::L => 0,
            ErrorCorrectionLevel::M => 1,
            ErrorCorrectionLevel::Q => 2,
            ErrorCorrectionLevel::H => 3,
        }
    }
}

impl TryFrom<u8> for ErrorCorrectionLevel {
    type Error = Exceptions;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        ErrorCorrectionLevel::forBits(value)
    }
}

impl From<ErrorCorrectionLevel> for u8 {
    fn from(value: ErrorCorrectionLevel) -> Self {
        value.get_value()
    }
}

impl FromStr for ErrorCorrectionLevel {
    type Err = Exceptions;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First try to see if the string is just the name of the value
        let as_str = match s.to_uppercase().as_str() {
            "L" => Some(ErrorCorrectionLevel::L),
            "M" => Some(ErrorCorrectionLevel::M),
            "Q" => Some(ErrorCorrectionLevel::Q),
            "H" => Some(ErrorCorrectionLevel::H),
            _ => None,
        };

        // If we find something, cool, return it, otherwise keep trying as numbers
        if let Some(as_str) = as_str {
            return Ok(as_str);
        }

        let number_possible = s.parse::<u8>();
        if let Ok(number_possible) = number_possible {
            return number_possible.try_into();
        }

        return Err(Exceptions::illegalArgumentWith(format!(
            "could not parse {s} into an ec level"
        )));
    }
}
