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

use crate::{
    common::{
        string_utils, BitSource, CharacterSet, DecoderRXingResult, ECIStringBuilder, Eci, Result,
    },
    DecodeHints, Exceptions,
};

#[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
use crate::{DecodeHintType, DecodeHintValue};

use super::{ErrorCorrectionLevel, Mode, VersionRef};

/**
 * <p>QR Codes can encode text as bits in one of several modes, and can use multiple modes
 * in one QR Code. This class decodes the bits back into text.</p>
 *
 * <p>See ISO 18004:2006, 6.4.3 - 6.4.7</p>
 *
 * @author Sean Owen
 */

/**
 * See ISO 18004:2006, 6.4.4 Table 5
 */
const ALPHANUMERIC_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
const GB2312_SUBSET: u32 = 1;

pub fn decode(
    bytes: &[u8],
    version: VersionRef,
    ecLevel: ErrorCorrectionLevel,
    hints: &DecodeHints,
) -> Result<DecoderRXingResult> {
    let mut bits = BitSource::new(bytes.to_owned());
    let mut result = ECIStringBuilder::with_capacity(50); //String::with_capacity(50);
    let mut byteSegments: std::vec::Vec<std::vec::Vec<u8>> = vec![];
    let mut symbolSequence = -1;
    let mut parityData = -1;

    let mut currentCharacterSetECI = None;
    let mut fc1InEffect = false;
    let mut hasFNC1first = false;
    let mut hasFNC1second = false;
    let mut mode;
    loop {
        // While still another segment to read...
        if bits.available() < 4 {
            // OK, assume we're done. Really, a TERMINATOR mode should have been recorded here
            mode = Mode::TERMINATOR;
        } else {
            mode = Mode::forBits(bits.readBits(4)? as u8)?; // mode is encoded by 4 bits
        }
        match mode {
            Mode::TERMINATOR => {}
            Mode::FNC1_FIRST_POSITION => {
                hasFNC1first = true; // symbology detection
                                     // We do little with FNC1 except alter the parsed result a bit according to the spec
                fc1InEffect = true;
            }
            Mode::FNC1_SECOND_POSITION => {
                hasFNC1second = true; // symbology detection
                                      // We do little with FNC1 except alter the parsed result a bit according to the spec
                fc1InEffect = true;
            }
            Mode::STRUCTURED_APPEND => {
                if bits.available() < 16 {
                    return Err(Exceptions::format_with(format!(
                        "Mode::Structured append expected bits.available() < 16, found bits of {}",
                        bits.available()
                    )));
                }
                // sequence number and parity is added later to the result metadata
                // Read next 8 bits (symbol sequence #) and 8 bits (parity data), then continue
                symbolSequence = bits.readBits(8)? as i32;
                parityData = bits.readBits(8)? as i32;
            }
            Mode::ECI => {
                // Count doesn't apply to ECI
                let value = parseECIValue(&mut bits)?;
                currentCharacterSetECI = CharacterSet::from(value).into(); //CharacterSet::get_character_set_by_eci(value).ok();
                if currentCharacterSetECI.is_none() {
                    return Err(Exceptions::format_with(format!(
                        "Value of {value} not valid"
                    )));
                }
            }
            Mode::HANZI => {
                // First handle Hanzi mode which does not start with character count
                // Chinese mode contains a sub set indicator right after mode indicator
                let subset = bits.readBits(4)?;
                let countHanzi =
                    bits.readBits(mode.getCharacterCountBits(version) as usize)? as usize;
                if subset == GB2312_SUBSET {
                    decodeHanziSegment(&mut bits, &mut result, countHanzi)?;
                }
            }
            _ => {
                // "Normal" QR code modes:
                // How many characters will follow, encoded in this mode?
                let count = bits.readBits(mode.getCharacterCountBits(version) as usize)? as usize;
                match mode {
                    Mode::NUMERIC => decodeNumericSegment(&mut bits, &mut result, count)?,
                    Mode::ALPHANUMERIC => {
                        decodeAlphanumericSegment(&mut bits, &mut result, count, fc1InEffect)?
                    }
                    Mode::BYTE => decodeByteSegment(
                        &mut bits,
                        &mut result,
                        count,
                        currentCharacterSetECI,
                        &mut byteSegments,
                        hints,
                    )?,
                    Mode::KANJI => decodeKanjiSegment(
                        &mut bits,
                        &mut result,
                        count,
                        currentCharacterSetECI,
                        hints,
                    )?,
                    _ => return Err(Exceptions::FORMAT),
                }
            }
        }

        if mode == Mode::TERMINATOR {
            break;
        }
    }

    let symbologyModifier = get_symbology_identifier(
        currentCharacterSetECI.is_some(),
        hasFNC1first,
        hasFNC1second,
    );

    Ok(DecoderRXingResult::with_all(
        bytes.to_owned(),
        result.build_result().to_string(),
        byteSegments.to_vec(),
        format!("{}", u8::from(ecLevel)),
        symbolSequence,
        parityData,
        symbologyModifier,
        String::default(),
        false,
    ))
}

fn get_symbology_identifier(has_charset: bool, hasFNC1first: bool, hasFNC1second: bool) -> u32 {
    if has_charset {
        if hasFNC1first {
            4
        } else if hasFNC1second {
            6
        } else {
            2
        }
    } else if hasFNC1first {
        3
    } else if hasFNC1second {
        5
    } else {
        1
    }
}

/**
 * See specification GBT 18284-2000
 */
fn decodeHanziSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    count: usize,
) -> Result<()> {
    // Don't crash trying to read more bits than we have available.
    if count * 13 > bits.available() {
        return Err(Exceptions::FORMAT);
    }

    // Each character will require 2 bytes. Read the characters as 2-byte pairs
    // and decode as GB2312 afterwards
    let mut buffer = vec![0u8; 2 * count];
    let mut offset = 0;
    let mut count = count;
    while count > 0 {
        // Each 13 bits encodes a 2-byte character
        let twoBytes = bits.readBits(13)?;
        let mut assembledTwoBytes = ((twoBytes / 0x060) << 8) | (twoBytes % 0x060);
        if assembledTwoBytes < 0x00A00 {
            // In the 0xA1A1 to 0xAAFE range
            assembledTwoBytes += 0x0A1A1;
        } else {
            // In the 0xB0A1 to 0xFAFE range
            assembledTwoBytes += 0x0A6A1;
        }

        buffer[offset] = (assembledTwoBytes >> 8) as u8;
        buffer[offset + 1] = assembledTwoBytes as u8;
        offset += 2;
        count -= 1;
    }

    result.append_eci(Eci::GB18030);
    result.append_bytes(&buffer);

    Ok(())
}

fn decodeKanjiSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    count: usize,
    currentCharacterSetECI: Option<CharacterSet>,
    hints: &DecodeHints,
) -> Result<()> {
    // Don't crash trying to read more bits than we have available.
    if count * 13 > bits.available() {
        return Err(Exceptions::FORMAT);
    }

    // Each character will require 2 bytes. Read the characters as 2-byte pairs
    // and decode as Shift_JIS afterwards
    let mut buffer = vec![0u8; 2 * count];
    let mut offset = 0;
    let mut count = count;
    while count > 0 {
        // Each 13 bits encodes a 2-byte character
        let twoBytes = bits.readBits(13)?;
        let mut assembledTwoBytes = ((twoBytes / 0x0C0) << 8) | (twoBytes % 0x0C0);
        if assembledTwoBytes < 0x01F00 {
            // In the 0x8140 to 0x9FFC range
            assembledTwoBytes += 0x08140;
        } else {
            // In the 0xE040 to 0xEBBF range
            assembledTwoBytes += 0x0C140;
        }
        buffer[offset] = (assembledTwoBytes >> 8) as u8;
        buffer[offset + 1] = assembledTwoBytes as u8;
        offset += 2;
        count -= 1;
    }

    #[cfg(not(feature = "allow_forced_iso_ied_18004_compliance"))]
    let encoder = {
        let _ = currentCharacterSetECI;
        let _ = hints;
        CharacterSet::Shift_JIS
    };

    #[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
    let encoder = if let Some(DecodeHintValue::QrAssumeSpecConformInput(true)) =
        hints.get(&DecodeHintType::QR_ASSUME_SPEC_CONFORM_INPUT)
    {
        currentCharacterSetECI.unwrap_or(CharacterSet::ISO8859_1)
    } else {
        CharacterSet::Shift_JIS
    };

    result.append_eci(Eci::from(encoder));
    result.append_bytes(&buffer);

    Ok(())
}

fn decodeByteSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    count: usize,
    currentCharacterSetECI: Option<CharacterSet>,
    byteSegments: &mut Vec<Vec<u8>>,
    hints: &DecodeHints,
) -> Result<()> {
    // Don't crash trying to read more bits than we have available.
    if 8 * count > bits.available() {
        return Err(Exceptions::FORMAT);
    }

    let mut readBytes = vec![0u8; count];

    for byte in readBytes.iter_mut().take(count) {
        *byte = bits.readBits(8)? as u8;
    }
    let encoding = if currentCharacterSetECI.is_none() {
        // The spec isn't clear on this mode; see
        // section 6.4.5: t does not say which encoding to assuming
        // upon decoding. I have seen ISO-8859-1 used as well as
        // Shift_JIS -- without anything like an ECI designator to
        // give a hint.
        {
            #[cfg(not(feature = "allow_forced_iso_ied_18004_compliance"))]
            string_utils::guessCharset(&readBytes, hints).ok_or(Exceptions::ILLEGAL_STATE)?
        }

        #[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
        if let Some(DecodeHintValue::QrAssumeSpecConformInput(true)) =
            hints.get(&DecodeHintType::QR_ASSUME_SPEC_CONFORM_INPUT)
        {
            CharacterSet::ISO8859_1
        } else {
            StringUtils::guessCharset(&readBytes, hints).ok_or(Exceptions::ILLEGAL_STATE)?
        }
    } else {
        currentCharacterSetECI.ok_or(Exceptions::ILLEGAL_STATE)?
        // CharacterSetECI::getCharset(
        //     currentCharacterSetECI
        //         .as_ref()
        //         .ok_or(Exceptions::ILLEGAL_STATE)?,
        // )
    };

    result.append_eci(Eci::from(encoding));
    result.append_bytes(&readBytes);

    byteSegments.push(readBytes);

    Ok(())
}

fn toAlphaNumericChar(value: u32) -> Result<char> {
    if value as usize >= ALPHANUMERIC_CHARS.len() {
        return Err(Exceptions::FORMAT);
    }

    ALPHANUMERIC_CHARS
        .chars()
        .nth(value as usize)
        .ok_or(Exceptions::FORMAT)
}

fn decodeAlphanumericSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    count: usize,
    fc1InEffect: bool,
) -> Result<()> {
    let mut r_hld = String::with_capacity(count);
    // Read two characters at a time
    let start = 0;
    let mut count = count;
    while count > 1 {
        if bits.available() < 11 {
            return Err(Exceptions::FORMAT);
        }
        let nextTwoCharsBits = bits.readBits(11)?;
        r_hld.push(toAlphaNumericChar(nextTwoCharsBits / 45)?);
        r_hld.push(toAlphaNumericChar(nextTwoCharsBits % 45)?);
        count -= 2;
    }
    if count == 1 {
        // special case: one character left
        if bits.available() < 6 {
            return Err(Exceptions::FORMAT);
        }
        r_hld.push(toAlphaNumericChar(bits.readBits(6)?)?);
    }
    // See section 6.4.8.1, 6.4.8.2
    if fc1InEffect {
        // We need to massage the result a bit if in an FNC1 mode:
        for i in start..r_hld.len() {
            if r_hld
                .chars()
                .nth(i)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                == '%'
            {
                if i < result.len() - 1
                    && r_hld
                        .chars()
                        .nth(i + 1)
                        .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                        == '%'
                {
                    // %% is rendered as %
                    r_hld.remove(i + 1);
                } else {
                    // In alpha mode, % should be converted to FNC1 separator 0x1D
                    r_hld.replace_range(i..i + 1, "\u{1D}");
                }
            }
        }
    }

    result.append_eci(Eci::ISO8859_1);
    result.append_string(&r_hld);

    Ok(())
}

fn decodeNumericSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    count: usize,
) -> Result<()> {
    result.append_eci(Eci::ISO8859_1);
    let mut count = count;
    // Read three digits at a time
    while count >= 3 {
        // Each 10 bits encodes three digits
        if bits.available() < 10 {
            return Err(Exceptions::FORMAT);
        }
        let threeDigitsBits = bits.readBits(10)?;
        if threeDigitsBits >= 1000 {
            return Err(Exceptions::FORMAT);
        }
        result.append_char(toAlphaNumericChar(threeDigitsBits / 100)?);
        result.append_char(toAlphaNumericChar((threeDigitsBits / 10) % 10)?);
        result.append_char(toAlphaNumericChar(threeDigitsBits % 10)?);
        count -= 3;
    }
    if count == 2 {
        // Two digits left over to read, encoded in 7 bits
        if bits.available() < 7 {
            return Err(Exceptions::FORMAT);
        }
        let twoDigitsBits = bits.readBits(7)?;
        if twoDigitsBits >= 100 {
            return Err(Exceptions::FORMAT);
        }
        result.append_char(toAlphaNumericChar(twoDigitsBits / 10)?);
        result.append_char(toAlphaNumericChar(twoDigitsBits % 10)?);
    } else if count == 1 {
        // One digit left over to read
        if bits.available() < 4 {
            return Err(Exceptions::FORMAT);
        }
        let digitBits = bits.readBits(4)?;
        if digitBits >= 10 {
            return Err(Exceptions::FORMAT);
        }
        result.append_char(toAlphaNumericChar(digitBits)?);
    }

    Ok(())
}

fn parseECIValue(bits: &mut BitSource) -> Result<Eci> {
    let firstByte = bits.readBits(8)?;
    if (firstByte & 0x80) == 0 {
        // just one byte
        return Ok(Eci::from(firstByte & 0x7F));
    }
    if (firstByte & 0xC0) == 0x80 {
        // two bytes
        let secondByte = bits.readBits(8)?;
        return Ok(Eci::from(((firstByte & 0x3F) << 8) | secondByte));
    }
    if (firstByte & 0xE0) == 0xC0 {
        // three bytes
        let secondThirdBytes = bits.readBits(16)?;
        return Ok(Eci::from(((firstByte & 0x1F) << 16) | secondThirdBytes));
    }

    Err(Exceptions::FORMAT)
}
