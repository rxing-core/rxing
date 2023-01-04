/*
 * Copyright 2013 ZXing authors
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
    common::{
        reedsolomon::{
            get_predefined_genericgf, GenericGFRef, PredefinedGenericGF, ReedSolomonEncoder,
        },
        BitArray, BitMatrix,
    },
    exceptions::Exceptions,
};

use super::{AztecCode, HighLevelEncoder};

/**
 * Generates Aztec 2D barcodes.
 *
 * @author Rustam Abdullaev
 */

pub const DEFAULT_EC_PERCENT: u32 = 33; // default minimal percentage of error check words
pub const DEFAULT_AZTEC_LAYERS: i32 = 0;
pub const MAX_NB_BITS: u32 = 32;
pub const MAX_NB_BITS_COMPACT: u32 = 4;

pub const WORD_SIZE: [u32; 33] = [
    4, 6, 6, 8, 8, 8, 8, 8, 8, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12,
];

/**
 * Encodes the given string content as an Aztec symbol (without ECI code)
 *
 * @param data input data string; must be encodable as ISO/IEC 8859-1 (Latin-1)
 * @return Aztec symbol matrix with metadata
 */
pub fn encode_simple(data: &str) -> Result<AztecCode, Exceptions> {
    let bytes = encoding::all::ISO_8859_1
        .encode(data, encoding::EncoderTrap::Replace)
        .unwrap();
    encode_bytes_simple(&bytes)
}

/**
 * Encodes the given string content as an Aztec symbol (without ECI code)
 *
 * @param data input data string; must be encodable as ISO/IEC 8859-1 (Latin-1)
 * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
 *                      a minimum of 23% + 3 words is recommended)
 * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
 * @return Aztec symbol matrix with metadata
 */
pub fn encode(
    data: &str,
    minECCPercent: u32,
    userSpecifiedLayers: i32,
) -> Result<AztecCode, Exceptions> {
    let bytes = encoding::all::ISO_8859_1
        .encode(data, encoding::EncoderTrap::Strict)
        .expect("must encode cleanly in ISO_8859_1");
    encode_bytes(&bytes, minECCPercent, userSpecifiedLayers)
}

/**
 * Encodes the given string content as an Aztec symbol
 *
 * @param data input data string
 * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
 *                      a minimum of 23% + 3 words is recommended)
 * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
 * @param charset character set in which to encode string using ECI; if null, no ECI code
 *                will be inserted, and the string must be encodable as ISO/IEC 8859-1
 *                (Latin-1), the default encoding of the symbol.
 * @return Aztec symbol matrix with metadata
 */
pub fn encode_with_charset(
    data: &str,
    minECCPercent: u32,
    userSpecifiedLayers: i32,
    charset: encoding::EncodingRef,
) -> Result<AztecCode, Exceptions> {
    let bytes = charset
        .encode(data, encoding::EncoderTrap::Strict)
        .expect("must be encodeable"); //data.getBytes(null != charset ? charset : StandardCharsets.ISO_8859_1);
    encode_bytes_with_charset(&bytes, minECCPercent, userSpecifiedLayers, charset)
}

/**
 * Encodes the given binary content as an Aztec symbol (without ECI code)
 *
 * @param data input data string
 * @return Aztec symbol matrix with metadata
 */
pub fn encode_bytes_simple(data: &[u8]) -> Result<AztecCode, Exceptions> {
    encode_bytes(data, DEFAULT_EC_PERCENT, DEFAULT_AZTEC_LAYERS)
}

/**
 * Encodes the given binary content as an Aztec symbol (without ECI code)
 *
 * @param data input data string
 * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
 *                      a minimum of 23% + 3 words is recommended)
 * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
 * @return Aztec symbol matrix with metadata
 */
pub fn encode_bytes(
    data: &[u8],
    minECCPercent: u32,
    userSpecifiedLayers: i32,
) -> Result<AztecCode, Exceptions> {
    encode_bytes_with_charset(
        data,
        minECCPercent,
        userSpecifiedLayers,
        encoding::all::ISO_8859_1,
    )
}

/**
 * Encodes the given binary content as an Aztec symbol
 *
 * @param data input data string
 * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
 *                      a minimum of 23% + 3 words is recommended)
 * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
 * @param charset character set to mark using ECI; if null, no ECI code will be inserted, and the
 *                default encoding of ISO/IEC 8859-1 will be assuming by readers.
 * @return Aztec symbol matrix with metadata
 */
pub fn encode_bytes_with_charset(
    data: &[u8],
    min_eccpercent: u32,
    user_specified_layers: i32,
    charset: encoding::EncodingRef,
) -> Result<AztecCode, Exceptions> {
    // High-level encode
    let bits = HighLevelEncoder::with_charset(data.into(), charset).encode()?;

    // stuff bits and choose symbol size
    let ecc_bits = bits.getSize() as u32 * min_eccpercent / 100 + 11;
    let total_size_bits = bits.getSize() as u32 + ecc_bits;
    let mut compact;
    let mut layers: u32;
    let mut total_bits_in_layer_var;
    let mut word_size;
    let mut stuffed_bits;
    if user_specified_layers != DEFAULT_AZTEC_LAYERS {
        compact = user_specified_layers < 0;
        layers = i32::abs(user_specified_layers) as u32;
        if layers
            > (if compact {
                MAX_NB_BITS_COMPACT
            } else {
                MAX_NB_BITS
            })
        {
            return Err(Exceptions::IllegalArgumentException(Some(format!(
                "Illegal value {} for layers",
                user_specified_layers
            ))));
        }
        total_bits_in_layer_var = total_bits_in_layer(layers, compact);
        word_size = WORD_SIZE[layers as usize];
        let usable_bits_in_layers = total_bits_in_layer_var - (total_bits_in_layer_var % word_size);
        stuffed_bits = stuffBits(&bits, word_size as usize);
        if stuffed_bits.getSize() as u32 + ecc_bits > usable_bits_in_layers {
            return Err(Exceptions::IllegalArgumentException(Some(
                "Data to large for user specified layer".to_owned(),
            )));
        }
        if compact && stuffed_bits.getSize() as u32 > word_size * 64 {
            // Compact format only allows 64 data words, though C4 can hold more words than that
            return Err(Exceptions::IllegalArgumentException(Some(
                "Data to large for user specified layer".to_owned(),
            )));
        }
    } else {
        word_size = 0;
        stuffed_bits = BitArray::new();
        // We look at the possible table sizes in the order Compact1, Compact2, Compact3,
        // Compact4, Normal4,...  Normal(i) for i < 4 isn't typically used since Compact(i+1)
        // is the same size, but has more data.
        let mut i = 0;
        loop {
            // for (int i = 0; ; i++) {
            if i > MAX_NB_BITS {
                return Err(Exceptions::IllegalArgumentException(Some(
                    "Data too large for an Aztec code".to_owned(),
                )));
            }
            compact = i <= 3;
            layers = if compact { i + 1 } else { i };
            total_bits_in_layer_var = total_bits_in_layer(layers, compact);
            if total_size_bits > total_bits_in_layer_var {
                i += 1;
                continue;
            }
            // [Re]stuff the bits if this is the first opportunity, or if the
            // wordSize has changed
            if stuffed_bits.getSize() == 0 || word_size != WORD_SIZE[layers as usize] {
                word_size = WORD_SIZE[layers as usize];
                stuffed_bits = stuffBits(&bits, word_size as usize);
            }
            let usable_bits_in_layers =
                total_bits_in_layer_var - (total_bits_in_layer_var % word_size);
            if compact && stuffed_bits.getSize() as u32 > word_size * 64 {
                // Compact format only allows 64 data words, though C4 can hold more words than that
                i += 1;
                continue;
            }
            if stuffed_bits.getSize() as u32 + ecc_bits <= usable_bits_in_layers {
                break;
            }
            i += 1;
        }
    }
    let message_bits = generateCheckWords(
        &stuffed_bits,
        total_bits_in_layer_var as usize,
        word_size as usize,
    );

    // generate mode message
    let messageSizeInWords = stuffed_bits.getSize() as u32 / word_size;
    let modeMessage = generateModeMessage(compact, layers, messageSizeInWords);

    // allocate symbol
    let baseMatrixSize = (if compact { 11 } else { 14 }) + layers * 4; // not including alignment lines
    let mut alignmentMap = vec![0u32; baseMatrixSize as usize];
    let matrixSize;
    if compact {
        // no alignment marks in compact mode, alignmentMap is a no-op
        matrixSize = baseMatrixSize;
        // for i in 0..alignmentMap.len() {
        alignmentMap[..].copy_from_slice(&(0..baseMatrixSize).collect::<Vec<u32>>()[..]);
    } else {
        matrixSize = baseMatrixSize + 1 + 2 * ((baseMatrixSize / 2 - 1) / 15);
        let origCenter = (baseMatrixSize / 2) as usize;
        let center = matrixSize / 2;
        for i in 0..origCenter {
            // for (int i = 0; i < origCenter; i++) {
            let newOffset = (i + i / 15) as u32;
            alignmentMap[origCenter - i - 1] = center - newOffset - 1;
            alignmentMap[origCenter + i] = center + newOffset + 1;
        }
    }
    let mut matrix = BitMatrix::with_single_dimension(matrixSize);

    // dbg!(matrix.to_string());

    // draw data bits
    let mut rowOffset = 0;
    for i in 0..layers as usize {
        // for (int i = 0, rowOffset = 0; i < layers; i++) {
        let rowSize = (layers as usize - i) * 4 + (if compact { 9 } else { 12 });
        for j in 0..rowSize {
            // for (int j = 0; j < rowSize; j++) {
            let columnOffset = j * 2;
            for k in 0..2 {
                // for (int k = 0; k < 2; k++) {
                if message_bits.get(rowOffset + columnOffset + k) {
                    matrix.set(alignmentMap[i * 2 + k], alignmentMap[i * 2 + j]);
                }
                if message_bits.get(rowOffset + rowSize * 2 + columnOffset + k) {
                    matrix.set(
                        alignmentMap[i * 2 + j],
                        alignmentMap[baseMatrixSize as usize - 1 - i * 2 - k],
                    );
                }
                if message_bits.get(rowOffset + rowSize * 4 + columnOffset + k) {
                    matrix.set(
                        alignmentMap[baseMatrixSize as usize - 1 - i * 2 - k],
                        alignmentMap[baseMatrixSize as usize - 1 - i * 2 - j],
                    );
                }
                if message_bits.get(rowOffset + rowSize * 6 + columnOffset + k) {
                    matrix.set(
                        alignmentMap[baseMatrixSize as usize - 1 - i * 2 - j],
                        alignmentMap[i * 2 + k],
                    );
                }
            }
        }
        rowOffset += rowSize * 8;
    }

    // dbg!(matrix.to_string());

    // draw mode message
    drawModeMessage(&mut matrix, compact, matrixSize, modeMessage);

    // dbg!(matrix.to_string());

    // draw alignment marks
    if compact {
        drawBullsEye(&mut matrix, matrixSize / 2, 5);
    } else {
        drawBullsEye(&mut matrix, matrixSize / 2, 7);
        let mut i = 0;
        let mut j = 0;
        while i < baseMatrixSize / 2 - 1 {
            let mut k = (matrixSize / 2) & 1;
            while k < matrixSize {
                // for (int k = (matrixSize / 2) & 1; k < matrixSize; k += 2) {
                matrix.set(matrixSize / 2 - j, k);
                matrix.set(matrixSize / 2 + j, k);
                matrix.set(k, matrixSize / 2 - j);
                matrix.set(k, matrixSize / 2 + j);

                k += 2;
            }

            i += 15;
            j += 16;
        }
        // for (int i = 0, j = 0; i < baseMatrixSize / 2 - 1; i += 15, j += 16) {
        //   for (int k = (matrixSize / 2) & 1; k < matrixSize; k += 2) {
        //     matrix.set(matrixSize / 2 - j, k);
        //     matrix.set(matrixSize / 2 + j, k);
        //     matrix.set(k, matrixSize / 2 - j);
        //     matrix.set(k, matrixSize / 2 + j);
        //   }
        // }
    }

    // dbg!(matrix.to_string());

    let aztec = AztecCode::new(compact, matrixSize, layers, messageSizeInWords, matrix);
    // aztec.setCompact(compact);
    // aztec.setSize(matrixSize);
    // aztec.setLayers(layers);
    // aztec.setCodeWords(messageSizeInWords);
    // aztec.setMatrix(matrix);
    Ok(aztec)
}

fn drawBullsEye(matrix: &mut BitMatrix, center: u32, size: u32) {
    let mut i = 0;
    while i < size {
        // for (int i = 0; i < size; i += 2) {
        for j in (center - i)..=(center + i) {
            // for (int j = center - i; j <= center + i; j++) {
            matrix.set(j, center - i);
            matrix.set(j, center + i);
            matrix.set(center - i, j);
            matrix.set(center + i, j);
        }
        i += 2;
    }
    matrix.set(center - size, center - size);
    matrix.set(center - size + 1, center - size);
    matrix.set(center - size, center - size + 1);
    matrix.set(center + size, center - size);
    matrix.set(center + size, center - size + 1);
    matrix.set(center + size, center + size - 1);
}

pub fn generateModeMessage(compact: bool, layers: u32, messageSizeInWords: u32) -> BitArray {
    let mut mode_message = BitArray::new();
    if compact {
        mode_message
            .appendBits(layers - 1, 2)
            .expect("should append");
        mode_message
            .appendBits(messageSizeInWords - 1, 6)
            .expect("should append");
        mode_message = generateCheckWords(&mode_message, 28, 4);
    } else {
        mode_message
            .appendBits(layers - 1, 5)
            .expect("should append");
        mode_message
            .appendBits(messageSizeInWords - 1, 11)
            .expect("should append");
        mode_message = generateCheckWords(&mode_message, 40, 4);
    }
    mode_message
}

fn drawModeMessage(matrix: &mut BitMatrix, compact: bool, matrixSize: u32, modeMessage: BitArray) {
    let center = matrixSize / 2;
    if compact {
        for i in 0..7usize {
            // for (int i = 0; i < 7; i++) {
            let offset = (center as usize - 3 + i) as u32;
            if modeMessage.get(i) {
                matrix.set(offset, center - 5);
            }
            if modeMessage.get(i + 7) {
                matrix.set(center + 5, offset);
            }
            if modeMessage.get(20 - i) {
                matrix.set(offset, center + 5);
            }
            if modeMessage.get(27 - i) {
                matrix.set(center - 5, offset);
            }
        }
    } else {
        for i in 0..10usize {
            // for (int i = 0; i < 10; i++) {
            let offset = (center as usize - 5 + i + i / 5) as u32;
            if modeMessage.get(i) {
                matrix.set(offset, center - 7);
            }
            if modeMessage.get(i + 10) {
                matrix.set(center + 7, offset);
            }
            if modeMessage.get(29 - i) {
                matrix.set(offset, center + 7);
            }
            if modeMessage.get(39 - i) {
                matrix.set(center - 7, offset);
            }
        }
    }
}

fn generateCheckWords(bitArray: &BitArray, totalBits: usize, wordSize: usize) -> BitArray {
    // bitArray is guaranteed to be a multiple of the wordSize, so no padding needed
    let message_size_in_words = bitArray.getSize() / wordSize;
    let mut rs = ReedSolomonEncoder::new(getGF(wordSize).expect("Should never have bad value"));
    let total_words = totalBits / wordSize;
    let mut message_words = bitsToWords(bitArray, wordSize, total_words);
    rs.encode(&mut message_words, total_words - message_size_in_words)
        .expect("must encode ok");
    let start_pad = totalBits % wordSize;
    let mut message_bits = BitArray::new();
    message_bits.appendBits(0, start_pad).expect("must append");
    for message_word in message_words {
        // for (int messageWord : messageWords) {
        message_bits
            .appendBits(message_word as u32, wordSize)
            .expect("must append");
    }
    // dbg!(message_bits.to_string());
    message_bits
}

fn bitsToWords(stuffedBits: &BitArray, wordSize: usize, totalWords: usize) -> Vec<i32> {
    let mut message = vec![0i32; totalWords];
    let mut i = 0;
    let n = stuffedBits.getSize() / wordSize;
    while i < n {
        // for (i = 0, n = stuffedBits.getSize() / wordSize; i < n; i++) {
        let mut value = 0;
        for j in 0..wordSize {
            //   for (int j = 0; j < wordSize; j++) {
            value |= if stuffedBits.get(i * wordSize + j) {
                1 << (wordSize - j - 1)
            } else {
                0
            };
        }
        message[i] = value;

        i += 1;
    }
    message
}

fn getGF(wordSize: usize) -> Result<GenericGFRef, Exceptions> {
    match wordSize {
        4 => Ok(get_predefined_genericgf(PredefinedGenericGF::AztecParam)),
        6 => Ok(get_predefined_genericgf(PredefinedGenericGF::AztecData6)),
        8 => Ok(get_predefined_genericgf(PredefinedGenericGF::AztecData8)),
        10 => Ok(get_predefined_genericgf(PredefinedGenericGF::AztecData10)),
        12 => Ok(get_predefined_genericgf(PredefinedGenericGF::AztecData12)),
        _ => Err(Exceptions::IllegalArgumentException(Some(format!(
            "Unsupported word size {}",
            wordSize
        )))),
    }
    // switch (wordSize) {
    //   case 4:
    //     return GenericGF.AZTEC_PARAM;
    //   case 6:
    //     return GenericGF.AZTEC_DATA_6;
    //   case 8:
    //     return GenericGF.AZTEC_DATA_8;
    //   case 10:
    //     return GenericGF.AZTEC_DATA_10;
    //   case 12:
    //     return GenericGF.AZTEC_DATA_12;
    //   default:
    //     throw new IllegalArgumentException("Unsupported word size " + wordSize);
    // }
}

pub fn stuffBits(bits: &BitArray, word_size: usize) -> BitArray {
    let mut out = BitArray::new();

    let n = bits.getSize() as isize;
    let mask = (1 << word_size) - 2;
    let mut i: isize = 0;
    while i < n {
        // for (int i = 0; i < n; i += wordSize) {
        let mut word = 0;
        for j in 0..word_size as isize {
            // for (int j = 0; j < wordSize; j++) {
            if i + j >= n || bits.get((i + j) as usize) {
                word |= 1 << (word_size as isize - 1 - j);
            }
        }
        if (word & mask) == mask {
            out.appendBits(word & mask, word_size).unwrap();
            i -= 1;
        } else if (word & mask) == 0 {
            out.appendBits(word | 1, word_size).unwrap();
            i -= 1;
        } else {
            out.appendBits(word, word_size).unwrap();
        }

        i += word_size as isize;
    }
    out
}

fn total_bits_in_layer(layers: u32, compact: bool) -> u32 {
    ((if compact { 88 } else { 112 }) + 16 * layers) * layers
    // return ((compact ? 88 : 112) + 16 * layers) * layers;
}
