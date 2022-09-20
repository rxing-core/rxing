/*
 * Copyright 2010 ZXing authors
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

// package com.google.zxing.aztec.decoder;

// import com.google.zxing.FormatException;
// import com.google.zxing.aztec.AztecDetectorRXingResult;
// import com.google.zxing.common.BitMatrix;
// import com.google.zxing.common.CharacterSetECI;
// import com.google.zxing.common.DecoderRXingResult;
// import com.google.zxing.common.reedsolomon.GenericGF;
// import com.google.zxing.common.reedsolomon.ReedSolomonDecoder;
// import com.google.zxing.common.reedsolomon.ReedSolomonException;

// import java.io.ByteArrayOutputStream;
// import java.io.UnsupportedEncodingException;
// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;
// import java.util.Arrays;

use encoding::Encoding;

use crate::{
    common::{
        self,
        reedsolomon::{
            get_predefined_genericgf, GenericGF, PredefinedGenericGF, ReedSolomonDecoder,
        },
        BitMatrix, CharacterSetECI, DecoderRXingResult, DetectorRXingResult,
    },
    exceptions::Exceptions,
};

use super::AztecDetectorResult::AztecDetectorRXingResult;

/**
 * <p>The main class which implements Aztec Code decoding -- as opposed to locating and extracting
 * the Aztec Code from an image.</p>
 *
 * @author David Olivier
 */

#[derive(PartialEq, Eq,Copy,Clone)]
enum Table {
    UPPER,
    LOWER,
    MIXED,
    DIGIT,
    PUNCT,
    BINARY,
}

const UPPER_TABLE: [&str; 32] = [
    "CTRL_PS", " ", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
    "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "CTRL_LL", "CTRL_ML", "CTRL_DL", "CTRL_BS",
];

const LOWER_TABLE: [&str; 32] = [
    "CTRL_PS", " ", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
    "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "CTRL_US", "CTRL_ML", "CTRL_DL", "CTRL_BS",
];

const MIXED_TABLE: [&str; 32] = [
    "CTRL_PS", " ", "\u{1}", "\u{2}", "\u{3}", "\u{4}", "\u{5}", "\u{6}", "\u{7}", "\u{8}", "\t",
    "\n", "\u{13}", "\u{0c}", "\r", "\u{33}", "\u{34}", "\u{35}", "\u{36}", "\u{37}", "@", "\\",
    "^", "_", "`", "|", "~", "\u{177}", "CTRL_LL", "CTRL_UL", "CTRL_PL", "CTRL_BS",
];

const PUNCT_TABLE: [&str; 32] = [
    "FLG(n)", "\r", "\r\n", ". ", ", ", ": ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*",
    "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "[", "]", "{", "}", "CTRL_UL",
];

const DIGIT_TABLE: [&str; 16] = [
    "CTRL_PS", " ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ",", ".", "CTRL_UL",
    "CTRL_US",
];

//   private static final Charset DEFAULT_ENCODING = StandardCharsets.ISO_8859_1;

//   private AztecDetectorRXingResult ddata;

pub fn decode(
    detectorRXingResult: &AztecDetectorRXingResult,
) -> Result<DecoderRXingResult, Exceptions> {
    //let mut detectorRXingResult = detectorRXingResult.clone();
    let matrix = detectorRXingResult.getBits();
    let rawbits = extractBits(&detectorRXingResult, matrix);
    let correctedBits = correctBits(&detectorRXingResult, &rawbits)?;
    let rawBytes = convertBoolArrayToByteArray(&correctedBits.correctBits);
    let result = getEncodedData(&correctedBits.correctBits);
    let mut decoderRXingResult = DecoderRXingResult::new(
        rawBytes,
        result?,
        Vec::new(),
        format!("{}%", correctedBits.ecLevel),
    );
    decoderRXingResult.setNumBits(correctedBits.correctBits.len());

    Ok(decoderRXingResult)
}

/// This method is used for testing the high-level encoder
pub fn highLevelDecode(correctedBits: &[bool]) -> Result<String, Exceptions> {
    getEncodedData(correctedBits)
}

/**
 * Gets the string encoded in the aztec code bits
 *
 * @return the decoded string
 */
fn getEncodedData(corrected_bits: &[bool]) -> Result<String, Exceptions> {
    let endIndex = corrected_bits.len();
    let mut latch_table = Table::UPPER; // table most recently latched to
    let mut shift_table = Table::UPPER; // table to use for the next read

    // Final decoded string result
    // (correctedBits-5) / 4 is an upper bound on the size (all-digit result)
    let mut result = String::with_capacity((corrected_bits.len() - 5) / 4);

    // Intermediary buffer of decoded bytes, which is decoded into a string and flushed
    // when character encoding changes (ECI) or input ends.
    let mut decoded_bytes: Vec<u8> = Vec::new();
    // let mut encdr: &'static dyn encoding::Encoding = encoding::all::UTF_8;
     let mut encdr: &'static dyn encoding::Encoding = encoding::all::ISO_8859_1;

    let mut index = 0;
    while index < endIndex {
        if shift_table == Table::BINARY {
            if endIndex - index < 5 {
                break;
            }
            let mut length = readCode(corrected_bits, index, 5);
            index += 5;
            if length == 0 {
                if endIndex - index < 11 {
                    break;
                }
                length = readCode(corrected_bits, index, 11) + 31;
                index += 11;
            }
            for _charCount in 0..length {
                // for (int charCount = 0; charCount < length; charCount++) {
                if endIndex - index < 8 {
                    index = endIndex; // Force outer loop to exit
                    break;
                }
                let code = readCode(corrected_bits, index, 8);
                decoded_bytes.push(code as u8);
                index += 8;
            }
            // Go back to whatever mode we had been in
            shift_table = latch_table;
        } else {
            let size = if shift_table == Table::DIGIT { 4 } else { 5 };
            if endIndex - index < size {
                break;
            }
            let code = readCode(corrected_bits, index, size);
            index += size;
            let str = getCharacter(shift_table, code)?;
            if "FLG(n)" == str {
                if endIndex - index < 3 {
                    break;
                }
                let mut n = readCode(corrected_bits, index, 3);
                index += 3;
                //  flush bytes, FLG changes state
                //   try {
                result.push_str(&encdr.decode(&decoded_bytes, encoding::DecoderTrap::Strict).unwrap());

                // result.push_str(&String::from_utf8(decoded_bytes.clone()).unwrap());
                // result.append(decodedBytes.toString(encoding.name()));
                //   } catch (UnsupportedEncodingException uee) {
                // throw new IllegalStateException(uee);
                //   }
                decoded_bytes.clear();
                match n {
                    0 => result.push(29 as char), // translate FNC1 as ASCII 29
                    7 => {
                        return Err(Exceptions::FormatException(
                            "FLG(7) is reserved and illegal".to_owned(),
                        ))
                    } // FLG(7) is reserved and illegal
                    _ => {
                        // ECI is decimal integer encoded as 1-6 codes in DIGIT mode
                        let mut eci = 0;
                        if endIndex - index < 4 * (n as usize) {
                            break;
                        }
                        while n > 0 {
                            //while (n-- > 0) {
                            let nextDigit = readCode(corrected_bits, index, 4);
                            index += 4;
                            if nextDigit < 2 || nextDigit > 11 {
                                return Err(Exceptions::FormatException(
                                    "Not a decimal digit".to_owned(),
                                )); // Not a decimal digit
                            }
                            eci = eci * 10 + (nextDigit - 2);
                            n -= 1;
                        }
                        let charsetECI = CharacterSetECI::getCharacterSetECIByValue(eci);
                        if charsetECI.is_err() {
                            return Err(Exceptions::FormatException(
                                "Charset must exist".to_owned(),
                            ));
                        }
                        encdr = CharacterSetECI::getCharset(&charsetECI?);
                        //   encdr = charsetECI?::getCharset();
                    }
                }
                //   switch (n) {
                //     case 0:
                //       result.append(29 as char);  // translate FNC1 as ASCII 29
                //       break;
                //     case 7:
                //       throw FormatException.getFormatInstance(); // FLG(7) is reserved and illegal
                //     default:
                //       // ECI is decimal integer encoded as 1-6 codes in DIGIT mode
                //       int eci = 0;
                //       if (endIndex - index < 4 * n) {
                //         break;
                //       }
                //       while (n-- > 0) {
                //         int nextDigit = readCode(correctedBits, index, 4);
                //         index += 4;
                //         if (nextDigit < 2 || nextDigit > 11) {
                //           throw FormatException.getFormatInstance(); // Not a decimal digit
                //         }
                //         eci = eci * 10 + (nextDigit - 2);
                //       }
                //       CharacterSetECI charsetECI = CharacterSetECI.getCharacterSetECIByValue(eci);
                //       if (charsetECI == null) {
                //         throw FormatException.getFormatInstance();
                //       }
                //       encoding = charsetECI.getCharset();
                //   }
                // Go back to whatever mode we had been in
                shift_table = latch_table;
            } else if str.starts_with("CTRL_") {
                // Table changes
                // ISO/IEC 24778:2008 prescribes ending a shift sequence in the mode from which it was invoked.
                // That's including when that mode is a shift.
                // Our test case dlusbs.png for issue #642 exercises that.
                latch_table = shift_table; // Latch the current mode, so as to return to Upper after U/S B/S
                shift_table = getTable(str.chars().nth(5).unwrap());
                if str.chars().nth(6).unwrap() == 'L' {
                    latch_table = shift_table;
                }
            } else {
                // Though stored as a table of strings for convenience, codes actually represent 1 or 2 *bytes*.
                let b = str.as_bytes();
                //let b = str.getBytes(StandardCharsets.US_ASCII);
                //decodedBytes.write(b, 0, b.length);
                for bt in b {
                    decoded_bytes.push(*bt);
                }
                // Go back to whatever mode we had been in
                shift_table = latch_table;
            }
        }
    }
    //try {
    if let Ok(str) = encdr.decode(&decoded_bytes, encoding::DecoderTrap::Strict) {
        result.push_str(&str);
    } else {
        return Err(Exceptions::IllegalStateException("bad encoding".to_owned()));
    }
    //   result.push_str(decodedBytes.toString(encoding.name()));
    //} catch (UnsupportedEncodingException uee) {
    // can't happen
    //throw new IllegalStateException(uee);
    //}
    Ok(result)
}

/**
 * gets the table corresponding to the char passed
 */
fn getTable(t: char) -> Table {
    match t {
        'L' => Table::LOWER,
        'P' => Table::PUNCT,
        'M' => Table::MIXED,
        'D' => Table::DIGIT,
        'B' => Table::BINARY,
        _ => Table::UPPER,
    }
    // switch (t) {
    //   case 'L':
    //     return Table.LOWER;
    //   case 'P':
    //     return Table.PUNCT;
    //   case 'M':
    //     return Table.MIXED;
    //   case 'D':
    //     return Table.DIGIT;
    //   case 'B':
    //     return Table.BINARY;
    //   case 'U':
    //   default:
    //     return Table.UPPER;
    // }
}

/**
 * Gets the character (or string) corresponding to the passed code in the given table
 *
 * @param table the table used
 * @param code the code of the character
 */
fn getCharacter(table: Table, code: u32) -> Result<&'static str, Exceptions> {
    match table {
        Table::UPPER => Ok(UPPER_TABLE[code as usize]),
        Table::LOWER => Ok(LOWER_TABLE[code as usize]),
        Table::MIXED => Ok(MIXED_TABLE[code as usize]),
        Table::DIGIT => Ok(DIGIT_TABLE[code as usize]),
        Table::PUNCT => Ok(PUNCT_TABLE[code as usize]),
        _ => Err(Exceptions::IllegalStateException("Bad table".to_owned())),
    }
    // switch (table) {
    //   case UPPER:
    //     return UPPER_TABLE[code];
    //   case LOWER:
    //     return LOWER_TABLE[code];
    //   case MIXED:
    //     return MIXED_TABLE[code];
    //   case PUNCT:
    //     return PUNCT_TABLE[code];
    //   case DIGIT:
    //     return DIGIT_TABLE[code];
    //   default:
    //     // Should not reach here.
    //     throw new IllegalStateException("Bad table");
}

struct CorrectedBitsRXingResult {
    correctBits: Vec<bool>,
    ecLevel: u32,
}
impl CorrectedBitsRXingResult {
    pub fn new(correctBits: Vec<bool>, ecLevel: u32) -> Self {
        Self {
            correctBits,
            ecLevel,
        }
    }
}

/**
 * <p>Performs RS error correction on an array of bits.</p>
 *
 * @return the corrected array
 * @throws FormatException if the input contains too many errors
 */
fn correctBits(
    ddata: &&AztecDetectorRXingResult,
    rawbits: &[bool],
) -> Result<CorrectedBitsRXingResult, Exceptions> {
    let gf: GenericGF;
    let codewordSize;

    if ddata.getNbLayers() <= 2 {
        codewordSize = 6;
        gf = get_predefined_genericgf(PredefinedGenericGF::AztecData6); //GenericGF.AZTEC_DATA_6;
    } else if ddata.getNbLayers() <= 8 {
        codewordSize = 8;
        gf = get_predefined_genericgf(PredefinedGenericGF::AztecData8); //GenericGF.AZTEC_DATA_8;
    } else if ddata.getNbLayers() <= 22 {
        codewordSize = 10;
        gf = get_predefined_genericgf(PredefinedGenericGF::AztecData10); //GenericGF.AZTEC_DATA_10;
    } else {
        codewordSize = 12;
        gf = get_predefined_genericgf(PredefinedGenericGF::AztecData12); //GenericGF.AZTEC_DATA_12;
    }

    let numDataCodewords = ddata.getNbDatablocks();
    let numCodewords = rawbits.len() / codewordSize;
    if numCodewords < numDataCodewords as usize {
        return Err(Exceptions::FormatException(format!(
            "numCodewords {}< numDataCodewords{}",
            numCodewords, numDataCodewords
        )));
    }
    let mut offset = rawbits.len() % codewordSize;

    let mut dataWords = vec![0i32; numCodewords];
    for i in 0..numCodewords {
        // for (int i = 0; i < numCodewords; i++, offset += codewordSize) {
        dataWords[i] = readCode(rawbits, offset, codewordSize) as i32;
        offset += codewordSize;
    }

    //try {
    let rs_decoder = ReedSolomonDecoder::new(gf);
    rs_decoder.decode(
        &mut dataWords,
        (numCodewords - numDataCodewords as usize) as i32,
    )?;
    //} catch (ReedSolomonException ex) {
    //throw FormatException.getFormatInstance(ex);
    //}

    // Now perform the unstuffing operation.
    // First, count how many bits are going to be thrown out as stuffing
    let mask = (1 << codewordSize) - 1;
    let mut stuffedBits = 0;
    for i in 0..numDataCodewords as usize {
        // for (int i = 0; i < numDataCodewords; i++) {
        let dataWord = dataWords[i];
        if dataWord == 0 || dataWord == mask {
            return Err(Exceptions::FormatException(
                "dataWord == 0 || dataWord == mask".to_owned(),
            ));
            //throw FormatException.getFormatInstance();
        } else if dataWord == 1 || dataWord == mask - 1 {
            stuffedBits += 1;
        }
    }
    // Now, actually unpack the bits and remove the stuffing
    let mut correctedBits =
        vec![false; (numDataCodewords * codewordSize as u32 - stuffedBits) as usize];
    let mut index = 0;
    for i in 0..numDataCodewords as usize {
        // for (int i = 0; i < numDataCodewords; i++) {
        let dataWord = dataWords[i];
        if dataWord == 1 || dataWord == mask - 1 {
            // next codewordSize-1 bits are all zeros or all ones
            correctedBits.splice(
                index..index + codewordSize - 1,
                vec![dataWord > 1; codewordSize],
            );
            // Arrays.fill(correctedBits, index, index + codewordSize - 1, dataWord > 1);
            index += codewordSize - 1;
        } else {
            for bit in (0..codewordSize).rev() {
                // for (int bit = codewordSize - 1; bit >= 0; --bit) {
                correctedBits[index] = (dataWord & (1 << bit)) != 0;
                index += 1;
            }
        }
    }

    Ok(CorrectedBitsRXingResult::new(
        correctedBits,
        (100 * (numCodewords - numDataCodewords as usize) / numCodewords) as u32,
    ))
}

/**
 * Gets the array of bits from an Aztec Code matrix
 *
 * @return the array of bits
 */
fn extractBits(ddata: &AztecDetectorRXingResult, matrix: &BitMatrix) -> Vec<bool> {
    let compact = ddata.isCompact();
    let layers = ddata.getNbLayers();
    let baseMatrixSize = ((if compact { 11 } else { 14 }) + layers * 4) as usize; // not including alignment lines
    let mut alignmentMap = vec![0u32; baseMatrixSize];
    let mut rawbits = vec![false; totalBitsInLayer(layers as usize, compact)];

    if compact {
        for i in 0..alignmentMap.len() {
            //   for (int i = 0; i < alignmentMap.length; i++) {
            alignmentMap[i] = i as u32;
        }
    } else {
        let matrixSize = baseMatrixSize + 1 + 2 * ((baseMatrixSize / 2 - 1) / 15);
        let origCenter = baseMatrixSize / 2;
        let center = matrixSize / 2;
        for i in 0..origCenter {
            //   for (int i = 0; i < origCenter; i++) {
            let newOffset = i + i / 15;
            alignmentMap[origCenter - i - 1] = (center - newOffset - 1) as u32;
            alignmentMap[origCenter + i] = (center + newOffset + 1) as u32;
        }
    }
    let mut rowOffset = 0;
    for i in 0..layers {
        // for (int i = 0, rowOffset = 0; i < layers; i++) {
        let rowSize = (layers - i) * 4 + (if compact { 9 } else { 12 });
        // The top-left most point of this layer is <low, low> (not including alignment lines)
        let low = i * 2;
        // The bottom-right most point of this layer is <high, high> (not including alignment lines)
        let high = baseMatrixSize as u32 - 1 - low;
        // We pull bits from the two 2 x rowSize columns and two rowSize x 2 rows
        for j in 0..rowSize {
            //   for (int j = 0; j < rowSize; j++) {
            let columnOffset = j * 2;
            for k in 0..2 {
                // for (int k = 0; k < 2; k++) {
                // left column
                rawbits[(rowOffset + columnOffset + k) as usize] = matrix.get(
                    alignmentMap[(low + k) as usize],
                    alignmentMap[(low + j) as usize],
                );
                // bottom row
                rawbits[(rowOffset + 2 * rowSize + columnOffset + k) as usize] = matrix.get(
                    alignmentMap[(low + j) as usize],
                    alignmentMap[(high - k) as usize],
                );
                // right column
                rawbits[(rowOffset + 4 * rowSize + columnOffset + k) as usize] = matrix.get(
                    alignmentMap[(high - k) as usize],
                    alignmentMap[(high - j) as usize],
                );
                // top row
                rawbits[(rowOffset + 6 * rowSize + columnOffset + k) as usize] = matrix.get(
                    alignmentMap[(high - j) as usize],
                    alignmentMap[(low + k) as usize],
                );
            }
        }
        rowOffset += rowSize * 8;
    }
    return rawbits;
}

/**
 * Reads a code of given length and at given index in an array of bits
 */
fn readCode(rawbits: &[bool], start_index: usize, length: usize) -> u32 {
    let mut res = 0;
    for i in start_index..start_index+length {
        // for (int i = startIndex; i < startIndex + length; i++) {
        res <<= 1;
        if rawbits[i] {
            res |= 0x01;
        }
    }
    return res;
}

/**
 * Reads a code of length 8 in an array of bits, padding with zeros
 */
fn readByte(rawbits: &[bool], start_index: usize) -> u8 {
    let n = rawbits.len() - start_index;
    if n >= 8 {
        return readCode(rawbits, start_index, 8) as u8;
    }
    return (readCode(rawbits, start_index, n) << (8 - n)) as u8;
}

/**
 * Packs a bit array into bytes, most significant bit first
 */
pub fn convertBoolArrayToByteArray(bool_arr: &[bool]) -> Vec<u8> {
    let mut byte_arr = vec![0u8; (bool_arr.len() + 7) / 8];
    for i in 0..byte_arr.len() {
        // for (int i = 0; i < byteArr.length; i++) {
        byte_arr[i] = readByte(bool_arr, 8 * i);
    }
    return byte_arr;
}

fn totalBitsInLayer(layers: usize, compact: bool) -> usize {
    (if compact { 88 } else { 112 } + 16 * layers) * layers
    // return ((compact ? 88 : 112) + 16 * layers) * layers;
}
