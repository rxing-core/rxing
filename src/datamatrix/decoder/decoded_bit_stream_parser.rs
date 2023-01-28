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

use encoding::Encoding;

use crate::{
    common::{BitSource, DecoderRXingResult, ECIStringBuilder},
    Exceptions,
};

/**
 * <p>Data Matrix Codes can encode text as bits in one of several modes, and can use multiple modes
 * in one Data Matrix Code. This class decodes the bits back into text.</p>
 *
 * <p>See ISO 16022:2006, 5.2.1 - 5.2.9.2</p>
 *
 * @author bbrown@google.com (Brian Brown)
 * @author Sean Owen
 */

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mode {
    PAD_ENCODE, // Not really a mode
    ASCII_ENCODE,
    C40_ENCODE,
    TEXT_ENCODE,
    ANSIX12_ENCODE,
    EDIFACT_ENCODE,
    BASE256_ENCODE,
    ECI_ENCODE,
}

/**
 * See ISO 16022:2006, Annex C Table C.1
 * The C40 Basic Character Set (*'s used for placeholders for the shift values)
 */
const C40_BASIC_SET_CHARS: [char; 40] = [
    '*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z',
];

const C40_SHIFT2_SET_CHARS: [char; 27] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_',
];

/**
 * See ISO 16022:2006, Annex C Table C.2
 * The Text Basic Character Set (*'s used for placeholders for the shift values)
 */
const TEXT_BASIC_SET_CHARS: [char; 40] = [
    '*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z',
];

// Shift 2 for Text is the same encoding as C40
const TEXT_SHIFT2_SET_CHARS: [char; 27] = C40_SHIFT2_SET_CHARS;

const TEXT_SHIFT3_SET_CHARS: [char; 32] = [
    '`',
    'A',
    'B',
    'C',
    'D',
    'E',
    'F',
    'G',
    'H',
    'I',
    'J',
    'K',
    'L',
    'M',
    'N',
    'O',
    'P',
    'Q',
    'R',
    'S',
    'T',
    'U',
    'V',
    'W',
    'X',
    'Y',
    'Z',
    '{',
    '|',
    '}',
    '~',
    127 as char,
];

const INSERT_STRING_CONST: &str = "\u{001E}\u{0004}";
const VALUE_236: &str = "[)>\u{001E}05\u{001D}";
const VALUE_237: &str = "[)>\u{001E}06\u{001D}";

pub fn decode(bytes: &[u8]) -> Result<DecoderRXingResult, Exceptions> {
    let mut bits = BitSource::new(bytes.to_vec());
    let mut result = ECIStringBuilder::with_capacity(100);
    let mut resultTrailer = String::new();
    let mut byteSegments = Vec::new(); //new ArrayList<>(1);
    let mut mode = Mode::ASCII_ENCODE;
    // Could look directly at 'bytes', if we're sure of not having to account for multi byte values
    let mut fnc1Positions = Vec::new();
    let symbologyModifier;
    let mut isECIencoded = false;
    loop {
        if mode == Mode::ASCII_ENCODE {
            mode = decodeAsciiSegment(
                &mut bits,
                &mut result,
                &mut resultTrailer,
                &mut fnc1Positions,
            )?;
        } else {
            match mode {
                Mode::C40_ENCODE => decodeC40Segment(&mut bits, &mut result, &mut fnc1Positions)?,
                Mode::TEXT_ENCODE => decodeTextSegment(&mut bits, &mut result, &mut fnc1Positions)?,
                Mode::ANSIX12_ENCODE => decodeAnsiX12Segment(&mut bits, &mut result)?,
                Mode::EDIFACT_ENCODE => decodeEdifactSegment(&mut bits, &mut result)?,
                Mode::BASE256_ENCODE => {
                    decodeBase256Segment(&mut bits, &mut result, &mut byteSegments)?
                }
                Mode::ECI_ENCODE => {
                    decodeECISegment(&mut bits, &mut result)?;
                    isECIencoded = true; // ECI detection only, atm continue decoding as ASCII
                }
                _ => return Err(Exceptions::FormatException(None)),
            };
            mode = Mode::ASCII_ENCODE;
        }
        if !(mode != Mode::PAD_ENCODE && bits.available() > 0) {
            break;
        }
    } //while (mode != Mode.PAD_ENCODE && bits.available() > 0);
    if !resultTrailer.is_empty() {
        result.appendCharacters(&resultTrailer);
    }
    if isECIencoded {
        // Examples for this numbers can be found in this documentation of a hardware barcode scanner:
        // https://honeywellaidc.force.com/supportppr/s/article/List-of-barcode-symbology-AIM-Identifiers
        if fnc1Positions.contains(&0) || fnc1Positions.contains(&4) {
            symbologyModifier = 5;
        } else if fnc1Positions.contains(&1) || fnc1Positions.contains(&5) {
            symbologyModifier = 6;
        } else {
            symbologyModifier = 4;
        }
    } else if fnc1Positions.contains(&0) || fnc1Positions.contains(&4) {
        symbologyModifier = 2;
    } else if fnc1Positions.contains(&1) || fnc1Positions.contains(&5) {
        symbologyModifier = 3;
    } else {
        symbologyModifier = 1;
    }

    Ok(DecoderRXingResult::with_symbology(
        bytes.to_vec(),
        result.build_result().to_string(),
        byteSegments,
        String::new(),
        symbologyModifier,
    ))

    // return new DecoderRXingResult(bytes,
    //                          result.toString(),
    //                          byteSegments.isEmpty() ? null : byteSegments,
    //                          null,
    //                          symbologyModifier);
}

/**
 * See ISO 16022:2006, 5.2.3 and Annex C, Table C.2
 */
fn decodeAsciiSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    resultTrailer: &mut String,
    fnc1positions: &mut Vec<usize>,
) -> Result<Mode, Exceptions> {
    let mut upperShift = false;
    let mut firstFNC1Position = 1;
    let mut firstCodeword = true;
    let mut sai = StructuredAppendInfo::default();
    loop {
        let mut oneByte = bits.readBits(8)?;
        if oneByte == 0 {
            return Err(Exceptions::FormatException(None));
        } else if oneByte <= 128 {
            // ASCII data (ASCII value + 1)
            if upperShift {
                oneByte += 128;
                //upperShift = false;
            }
            result.append_char(char::from_u32(oneByte - 1).unwrap());
            return Ok(Mode::ASCII_ENCODE);
        } else if oneByte == 129 {
            // Pad
            return Ok(Mode::PAD_ENCODE);
        } else if oneByte <= 229 {
            // 2-digit data 00-99 (Numeric Value + 130)
            let value = oneByte - 130;
            if value < 10 {
                // pad with '0' for single digit values
                result.append_char('0');
            }
            //result.append_char(char::from_u32(value).unwrap());
            result.append_string(&format!("{value}"));
        } else {
            match oneByte {
           230=> // Latch to C40 encodation
            return Ok(Mode::C40_ENCODE),
           231=> // Latch to Base 256 encodation
            return Ok(Mode::BASE256_ENCODE),
           232=> {// FNC1
            if bits.getByteOffset() == firstFNC1Position
					{/*result.symbology.modifier = '2';*/} // GS1
				else if bits.getByteOffset() == firstFNC1Position + 1
					{/*result.symbology.modifier = '3';*/} // AIM, note no AIM Application Indicator format defined, ISO 16022:2006 11.2
				else
					{fnc1positions.push(result.len());
                        result.append_char( 29 as char); }// translate as ASCII 29
            },
           233| // Structured Append
           234=> // Reader Programming
            // Ignore these symbols for now
            //throw ReaderException.getInstance();
            {
                if !firstCodeword // Must be first ISO 16022:2006 5.6.1
					{return Err(Exceptions::FormatException(Some("structured append tag must be first code word".to_owned())));}
				parse_structured_append(bits, &mut sai)?;
				firstFNC1Position = 5;
            },
           235=> // Upper Shift (shift to Extended ASCII)
            upperShift = true,
           236=> {// 05 Macro
            result.append_string(VALUE_236);
            resultTrailer.replace_range(0..0, INSERT_STRING_CONST);
            // resultTrailer.insert(0, "\u{001E}\u{0004}");
            },
           237=>{ // 06 Macro
            result.append_string(VALUE_237);
            resultTrailer.replace_range(0..0, INSERT_STRING_CONST);
            // resultTrailer.insert(0, "\u{001E}\u{0004}");
            },
           238=> // Latch to ANSI X12 encodation
            return Ok(Mode::ANSIX12_ENCODE),
           239=> // Latch to Text encodation
            return Ok(Mode::TEXT_ENCODE),
           240=> // Latch to EDIFACT encodation
            return Ok(Mode::EDIFACT_ENCODE),
           241=> // ECI Character
            return Ok(Mode::ECI_ENCODE),
          _=>{
            // Not to be used in ASCII encodation
            // but work around encoders that end with 254, latch back to ASCII
            if oneByte != 254 || bits.available() != 0 {
              return Err(Exceptions::FormatException(None))
            }},
        }
        }
        if bits.available() == 0 {
            break;
        }
        firstCodeword = false;
    } //while (bits.available() > 0);
    Ok(Mode::ASCII_ENCODE)
}

/**
 * See ISO 16022:2006, 5.2.5 and Annex C, Table C.1
 */
fn decodeC40Segment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    fnc1positions: &mut Vec<usize>,
) -> Result<(), Exceptions> {
    // Three C40 values are encoded in a 16-bit value as
    // (1600 * C1) + (40 * C2) + C3 + 1
    // TODO(bbrown): The Upper Shift with C40 doesn't work in the 4 value scenario all the time
    let mut upperShift = false;

    let mut cValues = [0; 3];
    let mut shift = 0;

    loop {
        // If there is only one byte left then it will be encoded as ASCII
        if bits.available() == 8 {
            return Ok(());
        }
        let firstByte = bits.readBits(8)?;
        if firstByte == 254 {
            // Unlatch codeword
            return Ok(());
        }

        parseTwoBytes(firstByte, bits.readBits(8)?, &mut cValues);

        for cValue in cValues {
            // for i in 0..3 {
            // for (int i = 0; i < 3; i++) {
            // let cValue = cValues[i];
            match shift {
                0 => {
                    if cValue < 3 {
                        shift = cValue + 1;
                    } else if cValue < C40_BASIC_SET_CHARS.len() as u32 {
                        let c40char = C40_BASIC_SET_CHARS[cValue as usize];
                        if upperShift {
                            result.append_char(char::from_u32(c40char as u32 + 128).unwrap());
                            upperShift = false;
                        } else {
                            result.append_char(c40char);
                        }
                    } else {
                        return Err(Exceptions::FormatException(None));
                    }
                }
                1 => {
                    if upperShift {
                        result.append_char(char::from_u32(cValue + 128).unwrap());
                        upperShift = false;
                    } else {
                        result.append_char(char::from_u32(cValue).unwrap());
                    }
                    shift = 0;
                }
                2 => {
                    if cValue < C40_SHIFT2_SET_CHARS.len() as u32 {
                        let c40char = C40_SHIFT2_SET_CHARS[cValue as usize];
                        if upperShift {
                            result.append_char(char::from_u32(c40char as u32 + 128).unwrap());
                            upperShift = false;
                        } else {
                            result.append_char(c40char);
                        }
                    } else {
                        match cValue {
                            27 => {
                                // FNC1
                                fnc1positions.push(result.len());
                                result.append_char(29 as char); // translate as ASCII 29
                            }
                            30 =>
                            // Upper Shift
                            {
                                upperShift = true
                            }

                            _ => return Err(Exceptions::FormatException(None)),
                        }
                    }
                    shift = 0;
                }
                3 => {
                    if upperShift {
                        result.append_char(char::from_u32(cValue + 224).unwrap());
                        upperShift = false;
                    } else {
                        result.append_char(char::from_u32(cValue + 96).unwrap());
                    }
                    shift = 0;
                }

                _ => return Err(Exceptions::FormatException(None)),
            }
        }
        if bits.available() == 0 {
            break;
        }
    } //while (bits.available() > 0);
    Ok(())
}

/**
 * See ISO 16022:2006, 5.2.6 and Annex C, Table C.2
 */
fn decodeTextSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    fnc1positions: &mut Vec<usize>,
) -> Result<(), Exceptions> {
    // Three Text values are encoded in a 16-bit value as
    // (1600 * C1) + (40 * C2) + C3 + 1
    // TODO(bbrown): The Upper Shift with Text doesn't work in the 4 value scenario all the time
    let mut upperShift = false;

    let mut cValues = [0; 3]; //new int[3];
    let mut shift = 0;
    loop {
        // If there is only one byte left then it will be encoded as ASCII
        if bits.available() == 8 {
            return Ok(());
        }
        let firstByte = bits.readBits(8)?;
        if firstByte == 254 {
            // Unlatch codeword
            return Ok(());
        }

        parseTwoBytes(firstByte, bits.readBits(8)?, &mut cValues);

        for cValue in cValues {
            // for (int i = 0; i < 3; i++) {
            // int cValue = cValues[i];
            match shift {
                0 => {
                    if cValue < 3 {
                        shift = cValue + 1;
                    } else if cValue < TEXT_BASIC_SET_CHARS.len() as u32 {
                        let textChar = TEXT_BASIC_SET_CHARS[cValue as usize];
                        if upperShift {
                            result.append_char(char::from_u32(textChar as u32 + 128).unwrap());
                            upperShift = false;
                        } else {
                            result.append_char(textChar);
                        }
                    } else {
                        return Err(Exceptions::FormatException(None));
                    }
                }
                1 => {
                    if upperShift {
                        result.append_char(char::from_u32(cValue + 128).unwrap());
                        upperShift = false;
                    } else {
                        result.append_char(char::from_u32(cValue).unwrap());
                    }
                    shift = 0;
                }

                2 => {
                    // Shift 2 for Text is the same encoding as C40
                    if cValue < TEXT_SHIFT2_SET_CHARS.len() as u32 {
                        let textChar = TEXT_SHIFT2_SET_CHARS[cValue as usize];
                        if upperShift {
                            result.append_char(char::from_u32(textChar as u32 + 128).unwrap());
                            upperShift = false;
                        } else {
                            result.append_char(textChar);
                        }
                    } else {
                        match cValue {
                            27 => {
                                // FNC1
                                fnc1positions.push(result.len());
                                result.append_char(29 as char); // translate as ASCII 29
                            }
                            30 =>
                            // Upper Shift
                            {
                                upperShift = true
                            }

                            _ => return Err(Exceptions::FormatException(None)),
                        }
                    }
                    shift = 0;
                }
                3 => {
                    if cValue < TEXT_SHIFT3_SET_CHARS.len() as u32 {
                        let textChar = TEXT_SHIFT3_SET_CHARS[cValue as usize];
                        if upperShift {
                            result.append_char(char::from_u32(textChar as u32 + 128).unwrap());
                            upperShift = false;
                        } else {
                            result.append_char(textChar);
                        }
                        shift = 0;
                    } else {
                        return Err(Exceptions::FormatException(None));
                    }
                }

                _ => return Err(Exceptions::FormatException(None)),
            }
        }
        if bits.available() == 0 {
            break;
        }
    } //while (bits.available() > 0);

    Ok(())
}

/**
 * See ISO 16022:2006, 5.2.7
 */
fn decodeAnsiX12Segment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
) -> Result<(), Exceptions> {
    // Three ANSI X12 values are encoded in a 16-bit value as
    // (1600 * C1) + (40 * C2) + C3 + 1

    let mut cValues = [0; 3]; //new int[3];
    loop {
        // If there is only one byte left then it will be encoded as ASCII
        if bits.available() == 8 {
            return Ok(());
        }
        let firstByte = bits.readBits(8)?;
        if firstByte == 254 {
            // Unlatch codeword
            return Ok(());
        }

        parseTwoBytes(firstByte, bits.readBits(8)?, &mut cValues);

        for cValue in cValues {
            // for (int i = 0; i < 3; i++) {
            //   int cValue = cValues[i];
            match cValue {
                0 =>
                // X12 segment terminator <CR>
                {
                    result.append_char('\r')
                }

                1 =>
                // X12 segment separator *
                {
                    result.append_char('*')
                }

                2 =>
                // X12 sub-element separator >
                {
                    result.append_char('>')
                }

                3 =>
                // space
                {
                    result.append_char(' ')
                }

                _ => {
                    if cValue < 14 {
                        // 0 - 9
                        result.append_char(char::from_u32(cValue + 44).unwrap());
                    } else if cValue < 40 {
                        // A - Z
                        result.append_char(char::from_u32(cValue + 51).unwrap());
                    } else {
                        return Err(Exceptions::FormatException(None));
                    }
                }
            }
        }
        if bits.available() == 0 {
            break;
        }
    } //while (bits.available() > 0);

    Ok(())
}

fn parseTwoBytes(firstByte: u32, secondByte: u32, result: &mut [u32]) {
    let mut fullBitValue = (firstByte << 8) + secondByte - 1;
    let mut temp = fullBitValue / 1600;
    result[0] = temp;
    fullBitValue -= temp * 1600;
    temp = fullBitValue / 40;
    result[1] = temp;
    result[2] = fullBitValue - temp * 40;
}

/**
 * See ISO 16022:2006, 5.2.8 and Annex C Table C.3
 */
fn decodeEdifactSegment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
) -> Result<(), Exceptions> {
    loop {
        // If there is only two or less bytes left then it will be encoded as ASCII
        if bits.available() <= 16 {
            return Ok(());
        }

        for _i in 0..4 {
            // for (int i = 0; i < 4; i++) {
            let mut edifactValue = bits.readBits(6)?;

            // Check for the unlatch character
            if edifactValue == 0x1F {
                // 011111
                // Read rest of byte, which should be 0, and stop
                let bitsLeft = 8 - bits.getBitOffset();
                if bitsLeft != 8 {
                    bits.readBits(bitsLeft)?;
                }
                return Ok(());
            }

            if (edifactValue & 0x20) == 0 {
                // no 1 in the leading (6th) bit
                edifactValue |= 0x40; // Add a leading 01 to the 6 bit binary value
            }
            result.append_char(char::from_u32(edifactValue).unwrap());
        }

        if bits.available() == 0 {
            break;
        }
    }

    Ok(())
}

/**
 * See ISO 16022:2006, 5.2.9 and Annex B, B.2
 */
fn decodeBase256Segment(
    bits: &mut BitSource,
    result: &mut ECIStringBuilder,
    byteSegments: &mut Vec<Vec<u8>>,
) -> Result<(), Exceptions> {
    // Figure out how long the Base 256 Segment is.
    let mut codewordPosition = 1 + bits.getByteOffset(); // position is 1-indexed
    let d1 = unrandomize255State(bits.readBits(8)?, codewordPosition);
    codewordPosition += 1;
    let count;
    if d1 == 0 {
        // Read the remainder of the symbol
        count = bits.available() as u32 / 8;
    } else if d1 < 250 {
        count = d1;
    } else {
        count = 250 * (d1 - 249) + unrandomize255State(bits.readBits(8)?, codewordPosition);
        codewordPosition += 1;
    }

    // We're seeing NegativeArraySizeException errors from users.
    // but we shouldn't in rust because it's unsigned
    // if count < 0 {
    //     return Err(Exceptions::FormatException(None));
    // }

    let mut bytes = vec![0u8; count as usize];
    for byte in bytes.iter_mut().take(count as usize) {
        // for i in 0..count as usize {
        // for (int i = 0; i < count; i++) {
        // Have seen this particular error in the wild, such as at
        // http://www.bcgen.com/demo/IDAutomationStreamingDataMatrix.aspx?MODE=3&D=Fred&PFMT=3&PT=F&X=0.3&O=0&LM=0.2
        if bits.available() < 8 {
            return Err(Exceptions::FormatException(None));
        }
        *byte = unrandomize255State(bits.readBits(8)?, codewordPosition) as u8;
        codewordPosition += 1;
    }
    result.append_string(
        &encoding::all::ISO_8859_1
            .decode(&bytes, encoding::DecoderTrap::Strict)
            .expect("decode"),
    );
    byteSegments.push(bytes);
    // result.append_string(&encoding::all::ISO_8859_1.decode(&bytes, encoding::DecoderTrap::Strict).expect("decode"));

    Ok(())
}

/**
 * See ISO 16022:2007, 5.4.1
 */
fn decodeECISegment(bits: &mut BitSource, result: &mut ECIStringBuilder) -> Result<(), Exceptions> {
    let firstByte = bits.readBits(8)?;
    if firstByte <= 127 {
        return result.appendECI(firstByte - 1);
    }

    let secondByte = bits.readBits(8)?;
    if firstByte <= 191 {
        return result.appendECI(firstByte - 1);
    }

    let thirdByte = bits.readBits(8)?;

    result
        .appendECI((firstByte - 192) * 64516 + 16383 + (secondByte - 1) * 254 + thirdByte - 1)

    // if bits.available() < 8 {
    //     return Err(Exceptions::FormatException(None));
    // }
    // let c1 = bits.readBits(8)?;
    // if c1 <= 127 {
    //     result.appendECI(c1 - 1)?;
    // }

    // Ok(())
    //currently we only support character set ECIs
    /*} else {
      if (bits.available() < 8) {
        throw FormatException.getFormatInstance();
      }
      int c2 = bits.readBits(8);
      if (c1 >= 128 && c1 <= 191) {
      } else {
        if (bits.available() < 8) {
          throw FormatException.getFormatInstance();
        }
        int c3 = bits.readBits(8);
      }
    }*/
}

/**
* See ISO 16022:2006, 5.6
*/
fn parse_structured_append(
    bits: &mut BitSource,
    sai: &mut StructuredAppendInfo,
) -> Result<(), Exceptions> {
    // 5.6.2 Table 8
    let symbolSequenceIndicator = bits.readBits(8)?;
    sai.index = (symbolSequenceIndicator >> 4) as i32;
    sai.count = (17 - (symbolSequenceIndicator & 0x0F)) as i32; // 2-16 permitted, 17 invalid

    if sai.count == 17 || sai.count <= sai.index
    // If info doesn't make sense
    {
        sai.count = 0; // Choose to mark count as unknown
    }

    let fileId1 = bits.readBits(8)?; // File identification 1
    let fileId2 = bits.readBits(8)?; // File identification 2

    // There's no conversion method or meaning given to the 2 file id codewords in Section 5.6.3, apart from
    // saying that each value should be 1-254. Choosing here to represent them as base 256.
    sai.id = ((fileId1 << 8) | fileId2).to_string();
    Ok(())
}
struct StructuredAppendInfo {
    index: i32, //= -1;
    count: i32, // = -1;
    id: String,
}

impl Default for StructuredAppendInfo {
    fn default() -> Self {
        Self {
            index: -1,
            count: -1,
            id: Default::default(),
        }
    }
}

/**
 * See ISO 16022:2006, Annex B, B.2
 */
fn unrandomize255State(randomizedBase256Codeword: u32, base256CodewordPosition: usize) -> u32 {
    let pseudoRandomNumber = ((149 * base256CodewordPosition as u32) % 255) + 1;
    let tempVariable = randomizedBase256Codeword as i32 - pseudoRandomNumber as i32;

    if tempVariable >= 0 {
        tempVariable as u32
    } else {
        (tempVariable + 256) as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::datamatrix::decoder::decoded_bit_stream_parser;

    #[test]
    fn testAsciiStandardDecode() {
        // ASCII characters 0-127 are encoded as the value + 1
        let bytes = [
            (b'a' + 1),
            (b'b' + 1),
            (b'c' + 1),
            (b'A' + 1),
            (b'B' + 1),
            (b'C' + 1),
        ];
        let decodedString = String::from(
            decoded_bit_stream_parser::decode(&bytes)
                .expect("decode")
                .getText(),
        );
        assert_eq!("abcABC", decodedString);
    }

    #[test]
    fn testAsciiDoubleDigitDecode() {
        // ASCII double digit (00 - 99) Numeric Value + 130
        let bytes = [130, (1 + 130), (98 + 130), (99 + 130)];
        let decodedString = String::from(
            decoded_bit_stream_parser::decode(&bytes)
                .expect("decode")
                .getText(),
        );
        assert_eq!("00019899", decodedString);
    }

    // TODO(bbrown): Add test cases for each encoding type
    // TODO(bbrown): Add test cases for switching encoding types
}
