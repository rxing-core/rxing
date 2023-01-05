#![allow(deprecated)]
/*
 * Copyright 2009 ZXing authors
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

use num::{self, bigint::ToBigUint, BigUint};
use std::rc::Rc;

use crate::{
    common::{DecoderRXingResult, ECIStringBuilder},
    pdf417::PDF417RXingResultMetadata,
    Exceptions,
};

/**
 * <p>This class contains the methods for decoding the PDF417 codewords.</p>
 *
 * @author SITA Lab (kevin.osullivan@sita.aero)
 * @author Guenther Grau
 */

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Alpha,
    Lower,
    Mixed,
    Punct,
    AlphaShift,
    PunctShift,
}

const TEXT_COMPACTION_MODE_LATCH: u32 = 900;
const BYTE_COMPACTION_MODE_LATCH: u32 = 901;
const NUMERIC_COMPACTION_MODE_LATCH: u32 = 902;
const BYTE_COMPACTION_MODE_LATCH_6: u32 = 924;
const ECI_USER_DEFINED: u32 = 925;
const ECI_GENERAL_PURPOSE: u32 = 926;
const ECI_CHARSET: u32 = 927;
const BEGIN_MACRO_PDF417_CONTROL_BLOCK: u32 = 928;
const BEGIN_MACRO_PDF417_OPTIONAL_FIELD: u32 = 923;
const MACRO_PDF417_TERMINATOR: u32 = 922;
const MODE_SHIFT_TO_BYTE_COMPACTION_MODE: u32 = 913;
const MAX_NUMERIC_CODEWORDS: usize = 15;

const MACRO_PDF417_OPTIONAL_FIELD_FILE_NAME: u32 = 0;
const MACRO_PDF417_OPTIONAL_FIELD_SEGMENT_COUNT: u32 = 1;
const MACRO_PDF417_OPTIONAL_FIELD_TIME_STAMP: u32 = 2;
const MACRO_PDF417_OPTIONAL_FIELD_SENDER: u32 = 3;
const MACRO_PDF417_OPTIONAL_FIELD_ADDRESSEE: u32 = 4;
const MACRO_PDF417_OPTIONAL_FIELD_FILE_SIZE: u32 = 5;
const MACRO_PDF417_OPTIONAL_FIELD_CHECKSUM: u32 = 6;

const PL: u32 = 25;
const LL: u32 = 27;
const AS: u32 = 27;
const ML: u32 = 28;
const AL: u32 = 28;
const PS: u32 = 29;
const PAL: u32 = 29;

const PUNCT_CHARS: [char; 29] = [
    ';', '<', '>', '@', '[', '\\', ']', '_', '`', '~', '!', '\r', '\t', ',', ':', '\n', '-', '.',
    '$', '/', '"', '|', '*', '(', ')', '?', '{', '}', '\'',
];

const MIXED_CHARS: [char; 25] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '&', '\r', '\t', ',', ':', '#', '-', '.',
    '$', '/', '+', '%', '*', '=', '^',
];

use once_cell::sync::Lazy;

/**
 * Table containing values for the exponent of 900.
 * This is used in the numeric compaction decode algorithm.
 */
static EXP900: Lazy<Vec<BigUint>> = Lazy::new(|| {
    const EXP_LEN: usize = 16;
    let mut exp900 = Vec::with_capacity(EXP_LEN); //[0;16];
    exp900.push(ToBigUint::to_biguint(&1).unwrap());
    let nineHundred = ToBigUint::to_biguint(&900).unwrap();
    exp900.push(nineHundred);
    let mut i = 2;
    while i < EXP_LEN {
        // for (int i = 2; i < EXP900.length; i++) {
        exp900.push(&exp900[i - 1] * 900_u32);

        i += 1;
    }

    exp900
});

// /**
//  * Table containing values for the exponent of 900.
//  * This is used in the numeric compaction decode algorithm.
//  */
// const  EXP900 : [u128;16] =

//  {
//   let mut exp900 = [0;16];
//   exp900[0] = 1;
//   let nineHundred = 900;
//   exp900[1] = nineHundred;
//   let mut i = 2;
//   while i < exp900.len() {
//   // for (int i = 2; i < EXP900.length; i++) {
//     exp900[i] = exp900[i - 1] * (nineHundred);

//     i+=1;
//   }

//   exp900
// };

const NUMBER_OF_SEQUENCE_CODEWORDS: usize = 2;

pub fn decode(codewords: &[u32], ecLevel: &str) -> Result<DecoderRXingResult, Exceptions> {
    let mut result = ECIStringBuilder::with_capacity(codewords.len() * 2);
    let mut codeIndex = textCompaction(codewords, 1, &mut result)?;
    let mut resultMetadata = PDF417RXingResultMetadata::default();
    while codeIndex < codewords[0] as usize {
        let code = codewords[codeIndex];
        codeIndex += 1;
        match code {
            TEXT_COMPACTION_MODE_LATCH => {
                codeIndex = textCompaction(codewords, codeIndex, &mut result)?
            }
            BYTE_COMPACTION_MODE_LATCH | BYTE_COMPACTION_MODE_LATCH_6 => {
                codeIndex = byteCompaction(code, codewords, codeIndex, &mut result)?
            }
            MODE_SHIFT_TO_BYTE_COMPACTION_MODE => {
                result.append_char(char::from_u32(codewords[codeIndex]).unwrap());
                codeIndex += 1;
            }
            NUMERIC_COMPACTION_MODE_LATCH => {
                codeIndex = numericCompaction(codewords, codeIndex, &mut result)?
            }
            ECI_CHARSET => {
                result.appendECI(codewords[codeIndex])?;
                codeIndex += 1;
            }
            ECI_GENERAL_PURPOSE =>
            // Can't do anything with generic ECI; skip its 2 characters
            {
                codeIndex += 2
            }
            ECI_USER_DEFINED =>
            // Can't do anything with user ECI; skip its 1 character
            {
                codeIndex += 1
            }
            BEGIN_MACRO_PDF417_CONTROL_BLOCK => {
                codeIndex = decodeMacroBlock(codewords, codeIndex, &mut resultMetadata)?
            }
            BEGIN_MACRO_PDF417_OPTIONAL_FIELD | MACRO_PDF417_TERMINATOR =>
            // Should not see these outside a macro block
            {
                return Err(Exceptions::FormatException(None))
            }
            _ => {
                // Default to text compaction. During testing numerous barcodes
                // appeared to be missing the starting Mode:: In these cases defaulting
                // to text compaction seems to work.
                codeIndex -= 1;
                codeIndex = textCompaction(codewords, codeIndex, &mut result)?;
            }
        }
    }

    result = result.build_result();

    if result.is_empty() && resultMetadata.getFileId().is_empty() {
        return Err(Exceptions::FormatException(None));
    }

    let mut decoderRXingResult = DecoderRXingResult::new(
        Vec::new(),
        result.to_string(),
        Vec::new(),
        ecLevel.to_owned(),
    );
    decoderRXingResult.setOther(Some(Rc::new(resultMetadata)));

    Ok(decoderRXingResult)
}

pub fn decodeMacroBlock(
    codewords: &[u32],
    codeIndex: usize,
    resultMetadata: &mut PDF417RXingResultMetadata,
) -> Result<usize, Exceptions> {
    let mut codeIndex = codeIndex;
    if codeIndex + NUMBER_OF_SEQUENCE_CODEWORDS > codewords[0] as usize {
        // we must have at least two bytes left for the segment index
        return Err(Exceptions::FormatException(None));
    }
    let mut segmentIndexArray = [0; NUMBER_OF_SEQUENCE_CODEWORDS];
    for seq in segmentIndexArray
        .iter_mut()
        .take(NUMBER_OF_SEQUENCE_CODEWORDS)
    {
        // for (int i = 0; i < NUMBER_OF_SEQUENCE_CODEWORDS; i++, codeIndex++) {
        *seq = codewords[codeIndex];
        codeIndex += 1;
    }
    let segmentIndexString =
        decodeBase900toBase10(&segmentIndexArray, NUMBER_OF_SEQUENCE_CODEWORDS)?;
    if segmentIndexString.is_empty() {
        resultMetadata.setSegmentIndex(0);
    } else if let Ok(parsed_int) = segmentIndexString.parse::<usize>() {
        resultMetadata.setSegmentIndex(parsed_int);
    } else {
        // too large; bad input?
        return Err(Exceptions::FormatException(None));
    }

    // Decoding the fileId codewords as 0-899 numbers, each 0-filled to width 3. This follows the spec
    // (See ISO/IEC 15438:2015 Annex H.6) and preserves all info, but some generators (e.g. TEC-IT) write
    // the fileId using text compaction, so in those cases the fileId will appear mangled.
    let mut fileId = String::new();
    while codeIndex < codewords[0] as usize
        && codeIndex < codewords.len()
        && codewords[codeIndex] != MACRO_PDF417_TERMINATOR
        && codewords[codeIndex] != BEGIN_MACRO_PDF417_OPTIONAL_FIELD
    {
        fileId.push_str(&format!("{:0>3}", codewords[codeIndex])/*String.format("%03d", codewords[codeIndex])*/);
        codeIndex += 1;
    }
    if fileId.chars().count() == 0 {
        // at least one fileId codeword is required (Annex H.2)
        return Err(Exceptions::FormatException(None));
    }
    resultMetadata.setFileId(fileId);

    let mut optionalFieldsStart = -1_isize;
    if codewords[codeIndex] == BEGIN_MACRO_PDF417_OPTIONAL_FIELD {
        optionalFieldsStart = codeIndex as isize + 1;
    }

    while codeIndex < codewords[0] as usize {
        match codewords[codeIndex] {
            BEGIN_MACRO_PDF417_OPTIONAL_FIELD => {
                codeIndex += 1;
                match codewords[codeIndex] {
                    MACRO_PDF417_OPTIONAL_FIELD_FILE_NAME => {
                        let mut fileName = ECIStringBuilder::new();
                        codeIndex = textCompaction(codewords, codeIndex + 1, &mut fileName)?;
                        fileName = fileName.build_result();
                        resultMetadata.setFileName(fileName.to_string());
                    }
                    MACRO_PDF417_OPTIONAL_FIELD_SENDER => {
                        let mut sender = ECIStringBuilder::new();
                        codeIndex = textCompaction(codewords, codeIndex + 1, &mut sender)?;
                        sender = sender.build_result();
                        resultMetadata.setSender(sender.to_string());
                    }
                    MACRO_PDF417_OPTIONAL_FIELD_ADDRESSEE => {
                        let mut addressee = ECIStringBuilder::new();
                        codeIndex = textCompaction(codewords, codeIndex + 1, &mut addressee)?;
                        addressee = addressee.build_result();
                        resultMetadata.setAddressee(addressee.to_string());
                    }
                    MACRO_PDF417_OPTIONAL_FIELD_SEGMENT_COUNT => {
                        let mut segmentCount = ECIStringBuilder::new();
                        codeIndex = numericCompaction(codewords, codeIndex + 1, &mut segmentCount)?;
                        segmentCount = segmentCount.build_result();
                        resultMetadata.setSegmentCount(segmentCount.to_string().parse().unwrap());
                    }
                    MACRO_PDF417_OPTIONAL_FIELD_TIME_STAMP => {
                        let mut timestamp = ECIStringBuilder::new();
                        codeIndex = numericCompaction(codewords, codeIndex + 1, &mut timestamp)?;
                        timestamp = timestamp.build_result();
                        resultMetadata.setTimestamp(timestamp.to_string().parse().unwrap());
                    }
                    MACRO_PDF417_OPTIONAL_FIELD_CHECKSUM => {
                        let mut checksum = ECIStringBuilder::new();
                        codeIndex = numericCompaction(codewords, codeIndex + 1, &mut checksum)?;
                        checksum = checksum.build_result();
                        resultMetadata.setChecksum(checksum.to_string().parse().unwrap());
                    }
                    MACRO_PDF417_OPTIONAL_FIELD_FILE_SIZE => {
                        let mut fileSize = ECIStringBuilder::new();
                        codeIndex = numericCompaction(codewords, codeIndex + 1, &mut fileSize)?;
                        fileSize = fileSize.build_result();
                        resultMetadata.setFileSize(fileSize.to_string().parse().unwrap());
                    }
                    _ => return Err(Exceptions::FormatException(None)),
                }
            }
            MACRO_PDF417_TERMINATOR => {
                codeIndex += 1;
                resultMetadata.setLastSegment(true);
            }
            _ => return Err(Exceptions::FormatException(None)),
        }
    }

    // copy optional fields to additional options
    if optionalFieldsStart != -1 {
        let mut optionalFieldsLength = codeIndex - optionalFieldsStart as usize;
        if resultMetadata.isLastSegment() {
            // do not include terminator
            optionalFieldsLength -= 1;
        }
        // resultMetadata.setOptionalData(
        //     Arrays.copyOfRange(codewords, optionalFieldsStart, optionalFieldsStart + optionalFieldsLength));
        resultMetadata.setOptionalData(
            codewords[optionalFieldsStart as usize
                ..(optionalFieldsStart + optionalFieldsLength as isize) as usize]
                .to_vec(),
        );
    }

    Ok(codeIndex)
}

/**
 * Text Compaction mode (see 5.4.1.5) permits all printable ASCII characters to be
 * encoded, i.e. values 32 - 126 inclusive in accordance with ISO/IEC 646 (IRV), as
 * well as selected control characters.
 *
 * @param codewords The array of codewords (data + error)
 * @param codeIndex The current index into the codeword array.
 * @param result    The decoded data is appended to the result.
 * @return The next index into the codeword array.
 */
fn textCompaction(
    codewords: &[u32],
    codeIndex: usize,
    result: &mut ECIStringBuilder,
) -> Result<usize, Exceptions> {
    let mut codeIndex = codeIndex;
    // 2 character per codeword
    let mut textCompactionData = vec![0; (codewords[0] as usize - codeIndex) * 2];
    // Used to hold the byte compaction value if there is a mode shift
    let mut byteCompactionData = vec![0; (codewords[0] as usize - codeIndex) * 2];

    let mut index = 0;
    let mut end = false;
    let mut subMode = Mode::Alpha;
    while (codeIndex < codewords[0] as usize) && !end {
        let mut code = codewords[codeIndex];
        codeIndex += 1;
        if code < TEXT_COMPACTION_MODE_LATCH {
            textCompactionData[index] = code / 30;
            textCompactionData[index + 1] = code % 30;
            index += 2;
        } else {
            match code {
                TEXT_COMPACTION_MODE_LATCH => {
                    // reinitialize text compaction mode to alpha sub mode
                    textCompactionData[index] = TEXT_COMPACTION_MODE_LATCH;
                    index += 1;
                }
                BYTE_COMPACTION_MODE_LATCH
                | BYTE_COMPACTION_MODE_LATCH_6
                | NUMERIC_COMPACTION_MODE_LATCH
                | BEGIN_MACRO_PDF417_CONTROL_BLOCK
                | BEGIN_MACRO_PDF417_OPTIONAL_FIELD
                | MACRO_PDF417_TERMINATOR => {
                    codeIndex -= 1;
                    end = true;
                }
                MODE_SHIFT_TO_BYTE_COMPACTION_MODE => {
                    // The Mode Shift codeword 913 shall cause a temporary
                    // switch from Text Compaction mode to Byte Compaction Mode::
                    // This switch shall be in effect for only the next codeword,
                    // after which the mode shall revert to the prevailing sub-mode
                    // of the Text Compaction Mode:: Codeword 913 is only available
                    // in Text Compaction mode; its use is described in 5.4.2.4.
                    textCompactionData[index] = MODE_SHIFT_TO_BYTE_COMPACTION_MODE;
                    code = codewords[codeIndex];
                    codeIndex += 1;
                    byteCompactionData[index] = code;
                    index += 1;
                }
                ECI_CHARSET => {
                    subMode = decodeTextCompaction(
                        &textCompactionData,
                        &byteCompactionData,
                        index,
                        result,
                        subMode,
                    );
                    result.appendECI(codewords[codeIndex])?;
                    codeIndex += 1;
                    textCompactionData = vec![0; (codewords[0] as usize - codeIndex) * 2];
                    byteCompactionData = vec![0; (codewords[0] as usize - codeIndex) * 2];
                    index = 0;
                }
                _ => {}
            }
        }
    }
    decodeTextCompaction(
        &textCompactionData,
        &byteCompactionData,
        index,
        result,
        subMode,
    );

    Ok(codeIndex)
}

/**
 * The Text Compaction mode includes all the printable ASCII characters
 * (i.e. values from 32 to 126) and three ASCII control characters: HT or tab
 * (ASCII value 9), LF or line feed (ASCII value 10), and CR or carriage
 * return (ASCII value 13). The Text Compaction mode also includes various latch
 * and shift characters which are used exclusively within the Mode:: The Text
 * Compaction mode encodes up to 2 characters per codeword. The compaction rules
 * for converting data into PDF417 codewords are defined in 5.4.2.2. The sub-mode
 * switches are defined in 5.4.2.3.
 *
 * @param textCompactionData The text compaction data.
 * @param byteCompactionData The byte compaction data if there
 *                           was a mode shift.
 * @param length             The size of the text compaction and byte compaction data.
 * @param result             The decoded data is appended to the result.
 * @param startMode          The mode in which decoding starts
 * @return The mode in which decoding ended
 */
fn decodeTextCompaction(
    textCompactionData: &[u32],
    byteCompactionData: &[u32],
    length: usize,
    result: &mut ECIStringBuilder,
    startMode: Mode,
) -> Mode {
    // Beginning from an initial state
    // The default compaction mode for PDF417 in effect at the start of each symbol shall always be Text
    // Compaction mode Alpha sub-mode (uppercase alphabetic). A latch codeword from another mode to the Text
    // Compaction mode shall always switch to the Text Compaction Alpha sub-Mode::
    let mut subMode = startMode;
    let mut priorToShiftMode = startMode;
    let mut latchedMode = startMode;
    let mut i = 0;
    while i < length {
        let subModeCh = textCompactionData[i];
        let mut ch = 0 as char;
        match subMode {
            Mode::Alpha =>
            // Alpha (uppercase alphabetic)
            {
                if subModeCh < 26 {
                    // Upper case Alpha Character
                    ch = char::from_u32('A' as u32 + subModeCh).unwrap();
                } else {
                    match subModeCh {
                        26 => ch = ' ',
                        LL => {
                            subMode = Mode::Lower;
                            latchedMode = subMode;
                        }
                        ML => {
                            subMode = Mode::Mixed;
                            latchedMode = subMode;
                        }
                        PS => {
                            // Shift to punctuation
                            priorToShiftMode = subMode;
                            subMode = Mode::PunctShift;
                        }
                        MODE_SHIFT_TO_BYTE_COMPACTION_MODE => {
                            result.append_char(char::from_u32(byteCompactionData[i]).unwrap())
                        }
                        TEXT_COMPACTION_MODE_LATCH => {
                            subMode = Mode::Alpha;
                            latchedMode = subMode;
                        }
                        _ => {}
                    }
                }
            }

            Mode::Lower =>
            // Lower (lowercase alphabetic)
            {
                if subModeCh < 26 {
                    ch = char::from_u32('a' as u32 + subModeCh).unwrap();
                } else {
                    match subModeCh {
                        26 => ch = ' ',
                        AS => {
                            // Shift to alpha
                            priorToShiftMode = subMode;
                            subMode = Mode::AlphaShift;
                        }
                        ML => {
                            subMode = Mode::Mixed;
                            latchedMode = subMode;
                        }
                        PS => {
                            // Shift to punctuation
                            priorToShiftMode = subMode;
                            subMode = Mode::PunctShift;
                        }
                        MODE_SHIFT_TO_BYTE_COMPACTION_MODE => {
                            result.append_char(char::from_u32(byteCompactionData[i]).unwrap())
                        }
                        TEXT_COMPACTION_MODE_LATCH => {
                            subMode = Mode::Alpha;
                            latchedMode = subMode;
                        }
                        _ => {}
                    }
                }
            }

            Mode::Mixed =>
            // Mixed (numeric and some punctuation)
            {
                if subModeCh < PL {
                    ch = MIXED_CHARS[subModeCh as usize];
                } else {
                    match subModeCh {
                        PL => {
                            subMode = Mode::Punct;
                            latchedMode = subMode;
                        }
                        26 => ch = ' ',
                        LL => {
                            subMode = Mode::Lower;
                            latchedMode = subMode;
                        }
                        AL | TEXT_COMPACTION_MODE_LATCH => {
                            subMode = Mode::Alpha;
                            latchedMode = subMode;
                        }
                        PS => {
                            // Shift to punctuation
                            priorToShiftMode = subMode;
                            subMode = Mode::PunctShift;
                        }
                        MODE_SHIFT_TO_BYTE_COMPACTION_MODE => {
                            result.append_char(char::from_u32(byteCompactionData[i]).unwrap())
                        }
                        _ => {}
                    }
                }
            }

            Mode::Punct =>
            // Punctuation
            {
                if subModeCh < PAL {
                    ch = PUNCT_CHARS[subModeCh as usize];
                } else {
                    match subModeCh {
                        PAL | TEXT_COMPACTION_MODE_LATCH => {
                            subMode = Mode::Alpha;
                            latchedMode = subMode;
                        }
                        MODE_SHIFT_TO_BYTE_COMPACTION_MODE => {
                            result.append_char(char::from_u32(byteCompactionData[i]).unwrap())
                        }
                        _ => {}
                    }
                }
            }

            Mode::AlphaShift => {
                // Restore sub-mode
                subMode = priorToShiftMode;
                if subModeCh < 26 {
                    ch = char::from_u32('A' as u32 + subModeCh).unwrap();
                } else {
                    match subModeCh {
                        26 => ch = ' ',
                        TEXT_COMPACTION_MODE_LATCH => subMode = Mode::Alpha,
                        _ => {}
                    }
                }
            }

            Mode::PunctShift => {
                // Restore sub-mode
                subMode = priorToShiftMode;
                if subModeCh < PAL {
                    ch = PUNCT_CHARS[subModeCh as usize];
                } else {
                    match subModeCh {
                        PAL | TEXT_COMPACTION_MODE_LATCH => subMode = Mode::Alpha,
                        MODE_SHIFT_TO_BYTE_COMPACTION_MODE =>
                        // PS before Shift-to-Byte is used as a padding character,
                        // see 5.4.2.4 of the specification
                        {
                            result.append_char(char::from_u32(byteCompactionData[i]).unwrap())
                        }
                        _ => {}
                    }
                }
            }
        }
        if ch as u32 != 0 {
            // Append decoded character to result
            result.append_char(ch);
        }
        i += 1;
    }
    latchedMode
}

/**
 * Byte Compaction mode (see 5.4.3) permits all 256 possible 8-bit byte values to be encoded.
 * This includes all ASCII characters value 0 to 127 inclusive and provides for international
 * character set support.
 *
 * @param mode      The byte compaction mode i.e. 901 or 924
 * @param codewords The array of codewords (data + error)
 * @param codeIndex The current index into the codeword array.
 * @param result    The decoded data is appended to the result.
 * @return The next index into the codeword array.
 */
fn byteCompaction(
    mode: u32,
    codewords: &[u32],
    codeIndex: usize,
    result: &mut ECIStringBuilder,
) -> Result<usize, Exceptions> {
    let mut end = false;
    let mut codeIndex = codeIndex;

    while codeIndex < codewords[0] as usize && !end {
        //handle leading ECIs
        while codeIndex < codewords[0] as usize && codewords[codeIndex] == ECI_CHARSET {
            codeIndex += 1;
            result.appendECI(codewords[codeIndex])?;
            codeIndex += 1;
        }

        if codeIndex >= codewords[0] as usize || codewords[codeIndex] >= TEXT_COMPACTION_MODE_LATCH
        {
            end = true;
        } else {
            //decode one block of 5 codewords to 6 bytes
            let mut value: u64 = 0;
            let mut count = 0;
            loop {
                value = 900 * value + codewords[codeIndex] as u64;
                codeIndex += 1;
                count += 1;
                if !(count < 5
                    && codeIndex < codewords[0] as usize
                    && codewords[codeIndex] < TEXT_COMPACTION_MODE_LATCH)
                {
                    break;
                }
            } /*while (count < 5 &&
              codeIndex < codewords[0] &&
              codewords[codeIndex] < TEXT_COMPACTION_MODE_LATCH);*/
            if count == 5
                && (mode == BYTE_COMPACTION_MODE_LATCH_6
                    || codeIndex < codewords[0] as usize
                        && codewords[codeIndex] < TEXT_COMPACTION_MODE_LATCH)
            {
                for i in 0..6 {
                    // for (int i = 0; i < 6; i++) {
                    result.append_byte((value >> (8 * (5 - i))) as u8);
                }
            } else {
                codeIndex -= count;
                while (codeIndex < codewords[0] as usize) && !end {
                    let code = codewords[codeIndex];
                    codeIndex += 1;
                    if code < TEXT_COMPACTION_MODE_LATCH {
                        result.append_byte(code as u8);
                    } else if code == ECI_CHARSET {
                        result.appendECI(codewords[codeIndex])?;
                        codeIndex += 1;
                    } else {
                        codeIndex -= 1;
                        end = true;
                    }
                }
            }
        }
    }
    Ok(codeIndex)
}

/**
 * Numeric Compaction mode (see 5.4.4) permits efficient encoding of numeric data strings.
 *
 * @param codewords The array of codewords (data + error)
 * @param codeIndex The current index into the codeword array.
 * @param result    The decoded data is appended to the result.
 * @return The next index into the codeword array.
 */
fn numericCompaction(
    codewords: &[u32],
    codeIndex: usize,
    result: &mut ECIStringBuilder,
) -> Result<usize, Exceptions> {
    let mut count = 0;
    let mut end = false;
    let mut codeIndex = codeIndex;

    let mut numericCodewords = [0; MAX_NUMERIC_CODEWORDS];

    while codeIndex < codewords[0] as usize && !end {
        let code = codewords[codeIndex];
        codeIndex += 1;
        if codeIndex == codewords[0] as usize {
            end = true;
        }
        if code < TEXT_COMPACTION_MODE_LATCH {
            numericCodewords[count] = code;
            count += 1;
        } else {
            match code {
                TEXT_COMPACTION_MODE_LATCH
                | BYTE_COMPACTION_MODE_LATCH
                | BYTE_COMPACTION_MODE_LATCH_6
                | BEGIN_MACRO_PDF417_CONTROL_BLOCK
                | BEGIN_MACRO_PDF417_OPTIONAL_FIELD
                | MACRO_PDF417_TERMINATOR
                | ECI_CHARSET => {
                    codeIndex -= 1;
                    end = true;
                }
                _ => {}
            }
        }
        if (count % MAX_NUMERIC_CODEWORDS == 0 || code == NUMERIC_COMPACTION_MODE_LATCH || end)
            && count > 0
        {
            // Re-invoking Numeric Compaction mode (by using codeword 902
            // while in Numeric Compaction mode) serves  to terminate the
            // current Numeric Compaction mode grouping as described in 5.4.4.2,
            // and then to start a new one grouping.
            result.append_string(&decodeBase900toBase10(&numericCodewords, count)?);
            count = 0;
        }
    }
    Ok(codeIndex)
}

/**
 * Convert a list of Numeric Compacted codewords from Base 900 to Base 10.
 *
 * @param codewords The array of codewords
 * @param count     The number of codewords
 * @return The decoded string representing the Numeric data.
 */
/*
  EXAMPLE
  Encode the fifteen digit numeric string 000213298174000
  Prefix the numeric string with a 1 and set the initial value of
  t = 1 000 213 298 174 000
  Calculate codeword 0
  d0 = 1 000 213 298 174 000 mod 900 = 200

  t = 1 000 213 298 174 000 div 900 = 1 111 348 109 082
  Calculate codeword 1
  d1 = 1 111 348 109 082 mod 900 = 282

  t = 1 111 348 109 082 div 900 = 1 234 831 232
  Calculate codeword 2
  d2 = 1 234 831 232 mod 900 = 632

  t = 1 234 831 232 div 900 = 1 372 034
  Calculate codeword 3
  d3 = 1 372 034 mod 900 = 434

  t = 1 372 034 div 900 = 1 524
  Calculate codeword 4
  d4 = 1 524 mod 900 = 624

  t = 1 524 div 900 = 1
  Calculate codeword 5
  d5 = 1 mod 900 = 1
  t = 1 div 900 = 0
  Codeword sequence is: 1, 624, 434, 632, 282, 200

  Decode the above codewords involves
    1 x 900 power of 5 + 624 x 900 power of 4 + 434 x 900 power of 3 +
  632 x 900 power of 2 + 282 x 900 power of 1 + 200 x 900 power of 0 = 1000213298174000

  Remove leading 1 =>  RXingResult is 000213298174000
*/
fn decodeBase900toBase10(codewords: &[u32], count: usize) -> Result<String, Exceptions> {
    let mut result = 0.to_biguint().unwrap();
    for i in 0..count {
        // for (int i = 0; i < count; i++) {
        result += &EXP900[count - i - 1] * (codewords[i].to_biguint().unwrap());
        // result = result.add(EXP900[count - i - 1].multiply(BigInteger.valueOf(codewords[i])));
    }
    let resultString = result.to_string();
    if !resultString.starts_with('1') {
        return Err(Exceptions::FormatException(None));
    }
    Ok(resultString[1..].to_owned())
}
