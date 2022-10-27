/*
 * Copyright 2006-2007 Jeremias Maerki.
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

use std::rc::Rc;

use encoding::{self, EncodingRef};

use crate::{Dimension, Exceptions};

use super::{
    ASCIIEncoder, Base256Encoder, C40Encoder, EdifactEncoder, Encoder, EncoderContext,
    SymbolShapeHint, TextEncoder, X12Encoder,
};
const DEFAULT_ENCODING: EncodingRef = encoding::all::ISO_8859_1;

use unicode_segmentation::UnicodeSegmentation;

/**
 * DataMatrix ECC 200 data encoder following the algorithm described in ISO/IEC 16022:200(E) in
 * annex S.
 */
/**
 * Padding character
 */
const PAD: u8 = 129;
/**
 * mode latch to C40 encodation mode
 */
pub const LATCH_TO_C40: u8 = 230;
/**
 * mode latch to Base 256 encodation mode
 */
pub const LATCH_TO_BASE256: u8 = 231;
/**
 * FNC1 Codeword
 */
//private static final char FNC1 = 232;
/**
 * Structured Append Codeword
 */
//private static final char STRUCTURED_APPEND = 233;
/**
 * Reader Programming
 */
//private static final char READER_PROGRAMMING = 234;
/**
 * Upper Shift
 */
pub const UPPER_SHIFT: u8 = 235;
/**
 * 05 Macro
 */
const MACRO_05: u8 = 236;
/**
 * 06 Macro
 */
const MACRO_06: u8 = 237;
/**
 * mode latch to ANSI X.12 encodation mode
 */
pub const LATCH_TO_ANSIX12: u8 = 238;
/**
 * mode latch to Text encodation mode
 */
pub const LATCH_TO_TEXT: u8 = 239;
/**
 * mode latch to EDIFACT encodation mode
 */
pub const LATCH_TO_EDIFACT: u8 = 240;
/**
 * ECI character (Extended Channel Interpretation)
 */
//private static final char ECI = 241;

/**
 * Unlatch from C40 encodation
 */
pub const C40_UNLATCH: u8 = 254;
/**
 * Unlatch from X12 encodation
 */
pub const X12_UNLATCH: u8 = 254;

/**
 * 05 Macro header
 */
pub const MACRO_05_HEADER: &str = "[)>\u{001E}05\u{001D}"; // THIS MIGHT BE WRONG, CHECK IF IT SHOULD BE a long unicode
/**
 * 06 Macro header
 */
pub const MACRO_06_HEADER: &str = "[)>\u{001E}06\u{001D}"; // THIS MIGHT BE WRONG, CHECK IF IT SHOULD BE a long unicode
/**
 * Macro trailer
 */
pub const MACRO_TRAILER: &str = "\u{001E}\u{0004}";

pub const ASCII_ENCODATION: usize = 0;
pub const C40_ENCODATION: usize = 1;
pub const TEXT_ENCODATION: usize = 2;
pub const X12_ENCODATION: usize = 3;
pub const EDIFACT_ENCODATION: usize = 4;
pub const BASE256_ENCODATION: usize = 5;

fn randomize253State(codewordPosition: u32) -> String {
    let pseudoRandom = ((149 * codewordPosition) % 253) + 1;
    let tempVariable = PAD as u32 + pseudoRandom;
    if tempVariable <= 254 {
        char::from_u32(tempVariable)
    } else {
        char::from_u32(tempVariable - 254)
    }
    .expect("must become a char")
    .to_string()
    // return (char) (tempVariable <= 254 ? tempVariable : tempVariable - 254);
}

/**
 * Performs message encoding of a DataMatrix message using the algorithm described in annex P
 * of ISO/IEC 16022:2000(E).
 *
 * @param msg the message
 * @return the encoded message (the char values range from 0 to 255)
 */
pub fn encodeHighLevel(msg: &str) -> Result<String, Exceptions> {
    encodeHighLevelWithDimensionForceC40(msg, SymbolShapeHint::FORCE_NONE, None, None, false)
}

/**
 * Performs message encoding of a DataMatrix message using the algorithm described in annex P
 * of ISO/IEC 16022:2000(E).
 *
 * @param msg     the message
 * @param shape   requested shape. May be {@code SymbolShapeHint.FORCE_NONE},
 *                {@code SymbolShapeHint.FORCE_SQUARE} or {@code SymbolShapeHint.FORCE_RECTANGLE}.
 * @param minSize the minimum symbol size constraint or null for no constraint
 * @param maxSize the maximum symbol size constraint or null for no constraint
 * @return the encoded message (the char values range from 0 to 255)
 */
pub fn encodeHighLevelWithDimension(
    msg: &str,
    shape: SymbolShapeHint,
    minSize: Option<Dimension>,
    maxSize: Option<Dimension>,
) -> Result<String, Exceptions> {
    encodeHighLevelWithDimensionForceC40(msg, shape, minSize, maxSize, false)
}
/**
 * Performs message encoding of a DataMatrix message using the algorithm described in annex P
 * of ISO/IEC 16022:2000(E).
 *
 * @param msg     the message
 * @param shape   requested shape. May be {@code SymbolShapeHint.FORCE_NONE},
 *                {@code SymbolShapeHint.FORCE_SQUARE} or {@code SymbolShapeHint.FORCE_RECTANGLE}.
 * @param minSize the minimum symbol size constraint or null for no constraint
 * @param maxSize the maximum symbol size constraint or null for no constraint
 * @param forceC40 enforce C40 encoding
 * @return the encoded message (the char values range from 0 to 255)
 */
pub fn encodeHighLevelWithDimensionForceC40(
    msg: &str,
    shape: SymbolShapeHint,
    minSize: Option<Dimension>,
    maxSize: Option<Dimension>,
    forceC40: bool,
) -> Result<String, Exceptions> {
    //the codewords 0..255 are encoded as Unicode characters
    let c40Encoder = Rc::new(C40Encoder::new());
    let encoders: [Rc<dyn Encoder>; 6] = [
        Rc::new(ASCIIEncoder::new()),
        c40Encoder.clone(),
        Rc::new(TextEncoder::new()),
        Rc::new(X12Encoder::new()),
        Rc::new(EdifactEncoder::new()),
        Rc::new(Base256Encoder::new()),
    ];

    let mut context = EncoderContext::new(msg)?;
    context.setSymbolShape(shape);
    context.setSizeConstraints(minSize, maxSize);

    if msg.starts_with(MACRO_05_HEADER) && msg.ends_with(MACRO_TRAILER) {
        context.writeCodeword(MACRO_05);
        context.setSkipAtEnd(2);
        context.pos += MACRO_05_HEADER.len() as u32;
    } else if msg.starts_with(MACRO_06_HEADER) && msg.ends_with(MACRO_TRAILER) {
        context.writeCodeword(MACRO_06);
        context.setSkipAtEnd(2);
        context.pos += MACRO_06_HEADER.len() as u32;
    }

    let mut encodingMode = ASCII_ENCODATION; //Default mode

    if forceC40 {
        c40Encoder.encodeMaximalC40(&mut context);
        encodingMode = context.getNewEncoding().unwrap();
        context.resetEncoderSignal();
    }

    while context.hasMoreCharacters() {
        encoders[encodingMode].encode(&mut context);
        if context.getNewEncoding().is_some() {
            encodingMode = context.getNewEncoding().unwrap();
            context.resetEncoderSignal();
        }
    }
    let len = context.getCodewordCount();
    context.updateSymbolInfo();
    let capacity = context.getSymbolInfo().unwrap().getDataCapacity();
    if len < capacity as usize
        && encodingMode != ASCII_ENCODATION
        && encodingMode != BASE256_ENCODATION
        && encodingMode != EDIFACT_ENCODATION
    {
        context.writeCodeword(0xfe); //Unlatch (254)
        // context.writeCodeword("\u{00fe}"); //Unlatch (254)
    }
    //Padding
    // let codewords = context.getCodewords();
    if context.getCodewords().len() < capacity as usize{
        // codewords.push(PAD as char);
        context.writeCodeword(PAD)
    }
    while context.getCodewords().len() < capacity as usize {
        // codewords.append(randomize253State(codewords.len() + 1));
        context.writeCodewords(&randomize253State(context.getCodewords().len() as u32 + 1))
    }

     Ok(context.getCodewords().to_owned())
}

pub fn lookAheadTest(msg: &str, startpos: u32, currentMode: u32) -> usize {
    let newMode = lookAheadTestIntern(msg, startpos, currentMode);
    if currentMode as usize == X12_ENCODATION && newMode as usize == X12_ENCODATION {
        // let msg_graphemes = msg.graphemes(true);
        let endpos = (startpos + 3).min(msg.chars().count() as u32);
        for i in startpos..endpos {
            // for (int i = startpos; i < endpos; i++) {
            if !isNativeX12(msg.chars().nth(i as usize).unwrap()) {
                return ASCII_ENCODATION;
            }
        }
    } else if currentMode as usize == EDIFACT_ENCODATION && newMode == EDIFACT_ENCODATION {
        // let msg_graphemes = msg.graphemes(true);
        let endpos = (startpos + 4).min(msg.chars().count() as u32);
        for i in startpos..endpos {
            // for (int i = startpos; i < endpos; i++) {
            if !isNativeEDIFACT(msg.chars().nth(i as usize).unwrap()) {
                return ASCII_ENCODATION;
            }
        }
    }
    newMode
}

fn lookAheadTestIntern(msg: &str, startpos: u32, currentMode: u32) -> usize {
    if startpos as usize >= msg.len() {
        return currentMode as usize;
    }
    let mut charCounts: [f32; 6];
    //step J
    if currentMode == ASCII_ENCODATION as u32 {
        charCounts = [0.0, 1.0, 1.0, 1.0, 1.0, 1.25];
    } else {
        charCounts = [1.0, 2.0, 2.0, 2.0, 2.0, 2.25];
        charCounts[currentMode as usize] = 0.0;
    }

    let mut charsProcessed = 0;
    let mut mins = [0u8; 6];
    let mut intCharCounts = [0u32; 6];
    loop {
        //step K
        if (startpos + charsProcessed) == msg.len() as u32 {
            mins.fill(0);
            intCharCounts.fill(0);
            // Arrays.fill(mins, (byte) 0);
            // Arrays.fill(intCharCounts, 0);
            let min = findMinimums(&charCounts, &mut intCharCounts, u32::MAX, &mut mins);
            let minCount = getMinimumCount(&mins);

            if intCharCounts[ASCII_ENCODATION] == min {
                return ASCII_ENCODATION;
            }
            if minCount == 1 {
                if mins[BASE256_ENCODATION] > 0 {
                    return BASE256_ENCODATION;
                }
                if mins[EDIFACT_ENCODATION] > 0 {
                    return EDIFACT_ENCODATION;
                }
                if mins[TEXT_ENCODATION] > 0 {
                    return TEXT_ENCODATION;
                }
                if mins[X12_ENCODATION] > 0 {
                    return X12_ENCODATION;
                }
            }
            return C40_ENCODATION;
        }

        // let c = msg
        //     .graphemes(true)
        //     .nth((startpos + charsProcessed) as usize)
        //     .unwrap();
        // let c = msg.charAt(startpos + charsProcessed);
        let c = msg.chars().nth((startpos + charsProcessed) as usize).unwrap();
        charsProcessed += 1;

        //step L
        if isDigit(c) {
            charCounts[ASCII_ENCODATION] += 0.5;
        } else if isExtendedASCII(c) {
            charCounts[ASCII_ENCODATION] = charCounts[ASCII_ENCODATION].ceil();
            charCounts[ASCII_ENCODATION] += 2.0;
        } else {
            charCounts[ASCII_ENCODATION] = charCounts[ASCII_ENCODATION].ceil();
            charCounts[ASCII_ENCODATION] += 1.0;
        }

        //step M
        if isNativeC40(c) {
            charCounts[C40_ENCODATION] += 2.0 / 3.0;
        } else if isExtendedASCII(c) {
            charCounts[C40_ENCODATION] += 8.0 / 3.0;
        } else {
            charCounts[C40_ENCODATION] += 4.0 / 3.0;
        }

        //step N
        if isNativeText(c) {
            charCounts[TEXT_ENCODATION] += 2.0 / 3.0;
        } else if isExtendedASCII(c) {
            charCounts[TEXT_ENCODATION] += 8.0 / 3.0;
        } else {
            charCounts[TEXT_ENCODATION] += 4.0 / 3.0;
        }

        //step O
        if isNativeX12(c) {
            charCounts[X12_ENCODATION] += 2.0 / 3.0;
        } else if isExtendedASCII(c) {
            charCounts[X12_ENCODATION] += 13.0 / 3.0;
        } else {
            charCounts[X12_ENCODATION] += 10.0 / 3.0;
        }

        //step P
        if isNativeEDIFACT(c) {
            charCounts[EDIFACT_ENCODATION] += 3.0 / 4.0;
        } else if isExtendedASCII(c) {
            charCounts[EDIFACT_ENCODATION] += 17.0 / 4.0;
        } else {
            charCounts[EDIFACT_ENCODATION] += 13.0 / 4.0;
        }

        // step Q
        if isSpecialB256(c) {
            charCounts[BASE256_ENCODATION] += 4.0;
        } else {
            charCounts[BASE256_ENCODATION] += 1.0;
        }

        //step R
        if charsProcessed >= 4 {
            mins.fill(0);
            intCharCounts.fill(0);
            // Arrays.fill(mins, (byte) 0);
            // Arrays.fill(intCharCounts, 0);
            findMinimums(&charCounts, &mut intCharCounts, u32::MAX, &mut mins);

            if intCharCounts[ASCII_ENCODATION]
                < min5(
                    intCharCounts[BASE256_ENCODATION],
                    intCharCounts[C40_ENCODATION],
                    intCharCounts[TEXT_ENCODATION],
                    intCharCounts[X12_ENCODATION],
                    intCharCounts[EDIFACT_ENCODATION],
                )
            {
                return ASCII_ENCODATION;
            }
            if intCharCounts[BASE256_ENCODATION] < intCharCounts[ASCII_ENCODATION]
                || intCharCounts[BASE256_ENCODATION] + 1
                    < min4(
                        intCharCounts[C40_ENCODATION],
                        intCharCounts[TEXT_ENCODATION],
                        intCharCounts[X12_ENCODATION],
                        intCharCounts[EDIFACT_ENCODATION],
                    )
            {
                return BASE256_ENCODATION;
            }
            if intCharCounts[EDIFACT_ENCODATION] + 1
                < min5(
                    intCharCounts[BASE256_ENCODATION],
                    intCharCounts[C40_ENCODATION],
                    intCharCounts[TEXT_ENCODATION],
                    intCharCounts[X12_ENCODATION],
                    intCharCounts[ASCII_ENCODATION],
                )
            {
                return EDIFACT_ENCODATION;
            }
            if intCharCounts[TEXT_ENCODATION] + 1
                < min5(
                    intCharCounts[BASE256_ENCODATION],
                    intCharCounts[C40_ENCODATION],
                    intCharCounts[EDIFACT_ENCODATION],
                    intCharCounts[X12_ENCODATION],
                    intCharCounts[ASCII_ENCODATION],
                )
            {
                return TEXT_ENCODATION;
            }
            if intCharCounts[X12_ENCODATION] + 1
                < min5(
                    intCharCounts[BASE256_ENCODATION],
                    intCharCounts[C40_ENCODATION],
                    intCharCounts[EDIFACT_ENCODATION],
                    intCharCounts[TEXT_ENCODATION],
                    intCharCounts[ASCII_ENCODATION],
                )
            {
                return X12_ENCODATION;
            }
            if intCharCounts[C40_ENCODATION] + 1
                < min4(
                    intCharCounts[ASCII_ENCODATION],
                    intCharCounts[BASE256_ENCODATION],
                    intCharCounts[EDIFACT_ENCODATION],
                    intCharCounts[TEXT_ENCODATION],
                )
            {
                if intCharCounts[C40_ENCODATION] < intCharCounts[X12_ENCODATION] {
                    return C40_ENCODATION;
                }
                if intCharCounts[C40_ENCODATION] == intCharCounts[X12_ENCODATION] {
                    let mut _p = startpos + charsProcessed + 1;
                    for tc in msg.chars() {
                        // while (p as usize) < msg.len() {
                        // let tc = msg.charAt(p);
                        if isX12TermSep(tc) {
                            return X12_ENCODATION;
                        }
                        if !isNativeX12(tc) {
                            break;
                        }
                        _p += 1;
                    }
                    return C40_ENCODATION;
                }
            }
        }
    }
}

fn min5(f1: u32, f2: u32, f3: u32, f4: u32, f5: u32) -> u32 {
    min4(f1, f2, f3, f4).min(f5)
}

fn min4(f1: u32, f2: u32, f3: u32, f4: u32) -> u32 {
    f1.min(f2.min(f3.min(f4)))
    //  Math.min(f1, Math.min(f2, Math.min(f3, f4)))
}

fn findMinimums(charCounts: &[f32; 6], intCharCounts: &mut [u32; 6], min: u32, mins: &mut [u8]) -> u32 {
    let mut min = min;
    for i in 0..6 {
        // for (int i = 0; i < 6; i++) {
        intCharCounts[i] = charCounts[i].ceil() as u32;
        let current = intCharCounts[i]; // = (int) Math.ceil(charCounts[i]));
        if min > current {
            min = current;
            mins.fill(0);
            // Arrays.fill(mins, (byte) 0);
        }
        if min == current {
            mins[i] += 1;
        }
    }
    return min;
}

fn getMinimumCount(mins: &[u8]) -> u32 {
    let mut minCount = 0;
    for i in 0..6 {
        // for (int i = 0; i < 6; i++) {
        minCount += mins[i] as u32;
    }
    minCount
}

pub fn isDigit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

pub fn isExtendedASCII(ch: char) -> bool {
    (ch as u8) >= 128 && (ch as u8) <= 255
}

fn isNativeC40(ch: char) -> bool {
    (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'A' && ch <= 'Z')
}

fn isNativeText(ch: char) -> bool {
    (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'z')
}

fn isNativeX12(ch: char) -> bool {
    return isX12TermSep(ch)
        || (ch == ' ')
        || (ch >= '0' && ch <= '9')
        || (ch >= 'A' && ch <= 'Z');
}

fn isX12TermSep(ch: char) -> bool {
    (ch == '\r') //CR
        || (ch == '*')
        || (ch == '>')
}

fn isNativeEDIFACT(ch: char) -> bool {
    ch >= ' ' && ch <= '^'
}

fn isSpecialB256(ch: char) -> bool {
    unimplemented!();
    return false; //TODO NOT IMPLEMENTED YET!!!
}

/**
 * Determines the number of consecutive characters that are encodable using numeric compaction.
 *
 * @param msg      the message
 * @param startpos the start position within the message
 * @return the requested character count
 */
pub fn determineConsecutiveDigitCount(msg: &str, startpos: u32) -> u32 {
    let len = msg.chars().count();//len();
    let mut idx = startpos;
    // let graphemes = msg.graphemes(true);
    while (idx as usize) < len && isDigit(msg.chars().nth(idx as usize).unwrap()) {
        idx += 1;
    }
    idx - startpos
}

pub fn illegalCharacter(c: char) -> Result<(), Exceptions> {
    // let hex = Integer.toHexString(c);
    // hex = "0000".substring(0, 4 - hex.length()) + hex;
    Err(Exceptions::IllegalArgumentException(format!(
        "Illegal character: {} (0x{})",
        c, c
    )))
}
