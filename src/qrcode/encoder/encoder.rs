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
/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */
use std::collections::HashMap;

use encoding::EncodingRef;

use once_cell::sync::Lazy;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    common::{
        reedsolomon::{get_predefined_genericgf, PredefinedGenericGF, ReedSolomonEncoder},
        BitArray, CharacterSetECI,
    },
    qrcode::decoder::{ErrorCorrectionLevel, Mode, Version, VersionRef},
    EncodeHintType, EncodeHintValue, EncodingHintDictionary, Exceptions,
};

use super::{mask_util, matrix_util, BlockPair, ByteMatrix, MinimalEncoder, QRCode};

static SHIFT_JIS_CHARSET: Lazy<EncodingRef> =
    Lazy::new(|| encoding::label::encoding_from_whatwg_label("SJIS").unwrap());

// The original table is defined in the table 5 of JISX0510:2004 (p.19).
const ALPHANUMERIC_TABLE: [i8; 96] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, // 0x00-0x0f
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, // 0x10-0x1f
    36, -1, -1, -1, 37, 38, -1, -1, -1, -1, 39, 40, -1, 41, 42, 43, // 0x20-0x2f
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 44, -1, -1, -1, -1, -1, // 0x30-0x3f
    -1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, // 0x40-0x4f
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, -1, // 0x50-0x5f
];

pub const DEFAULT_BYTE_MODE_ENCODING: EncodingRef = encoding::all::ISO_8859_1;

// The mask penalty calculation is complicated.  See Table 21 of JISX0510:2004 (p.45) for details.
// Basically it applies four rules and summate all penalties.
pub fn calculateMaskPenalty(matrix: &ByteMatrix) -> u32 {
    mask_util::applyMaskPenaltyRule1(matrix)
        + mask_util::applyMaskPenaltyRule2(matrix)
        + mask_util::applyMaskPenaltyRule3(matrix)
        + mask_util::applyMaskPenaltyRule4(matrix)
}

/**
 * @param content text to encode
 * @param ecLevel error correction level to use
 * @return {@link QRCode} representing the encoded QR code
 * @throws WriterException if encoding can't succeed, because of for example invalid content
 *   or configuration
 */
pub fn encode(content: &str, ecLevel: ErrorCorrectionLevel) -> Result<QRCode, Exceptions> {
    encode_with_hints(content, ecLevel, &HashMap::new())
}

pub fn encode_with_hints(
    content: &str,
    ec_level: ErrorCorrectionLevel,
    hints: &EncodingHintDictionary,
) -> Result<QRCode, Exceptions> {
    let version;
    let mut header_and_data_bits;
    let mode;

    let has_gs1_format_hint = hints.contains_key(&EncodeHintType::GS1_FORMAT)
        && if let EncodeHintValue::Gs1Format(v) = hints.get(&EncodeHintType::GS1_FORMAT).unwrap() {
            *v
        } else {
            false
        };
    let has_compaction_hint = hints.contains_key(&EncodeHintType::QR_COMPACT)
        && if let EncodeHintValue::QrCompact(v) = hints.get(&EncodeHintType::QR_COMPACT).unwrap() {
            if let Ok(vb) = v.parse::<bool>() {
                vb
            } else {
                false
            }
        } else {
            false
        };

    // Determine what character encoding has been specified by the caller, if any
    let mut encoding = None; //DEFAULT_BYTE_MODE_ENCODING;
    let mut has_encoding_hint = hints.contains_key(&EncodeHintType::CHARACTER_SET);
    if has_encoding_hint {
        if let EncodeHintValue::CharacterSet(v) = hints.get(&EncodeHintType::CHARACTER_SET).unwrap()
        {
            encoding = Some(encoding::label::encoding_from_whatwg_label(v).unwrap())
        }
        // encoding = encoding::label::encoding_from_whatwg_label(hints.get(&EncodeHintType::CHARACTER_SET).unwrap());
    }

    if has_compaction_hint {
        mode = Mode::BYTE;

        // dbg!("consider this a huge risk, not sure if it should be defaulting to default");
        let priority_encoding = encoding; //if encoding.name() == DEFAULT_BYTE_MODE_ENCODING.name()  {None} else {Some(encoding)};
        let rn = MinimalEncoder::encode_with_details(
            content,
            None,
            priority_encoding,
            has_gs1_format_hint,
            ec_level,
        )?;

        header_and_data_bits = BitArray::new();
        rn.getBits(&mut header_and_data_bits)?;
        version = rn.getVersion();
    } else {
        //Switch to default encoding
        let encoding = if let Some(encoding) = encoding {
            encoding
        } else if let Ok(_encs) =
            DEFAULT_BYTE_MODE_ENCODING.encode(content, encoding::EncoderTrap::Strict)
        {
            DEFAULT_BYTE_MODE_ENCODING
        } else {
            has_encoding_hint = true;
            encoding::all::UTF_8
        };

        // Pick an encoding mode appropriate for the content. Note that this will not attempt to use
        // multiple modes / segments even if that were more efficient.
        mode = chooseModeWithEncoding(content, encoding);

        // This will store the header information, like mode and
        // length, as well as "header" segments like an ECI segment.
        let mut header_bits = BitArray::new();

        // Append ECI segment if applicable
        if mode == Mode::BYTE && has_encoding_hint {
            let eci = CharacterSetECI::getCharacterSetECI(encoding);
            if eci.is_some() {
                appendECI(&eci.unwrap(), &mut header_bits)?;
            }
        }

        // Append the FNC1 mode header for GS1 formatted data if applicable
        if has_gs1_format_hint {
            // GS1 formatted codes are prefixed with a FNC1 in first position mode header
            appendModeInfo(Mode::FNC1_FIRST_POSITION, &mut header_bits)?;
        }

        // (With ECI in place,) Write the mode marker
        appendModeInfo(mode, &mut header_bits)?;

        // Collect data within the main segment, separately, to count its size if needed. Don't add it to
        // main payload yet.
        let mut data_bits = BitArray::new();
        appendBytes(content, mode, &mut data_bits, encoding)?;

        if hints.contains_key(&EncodeHintType::QR_VERSION) {
            let versionNumber = if let EncodeHintValue::QrVersion(v) =
                hints.get(&EncodeHintType::QR_VERSION).unwrap()
            {
                if let Ok(vb) = v.parse::<u32>() {
                    vb
                } else {
                    0
                }
            } else {
                0
            };
            // let versionNumber = Integer.parseInt(hints.get(&EncodeHintType::QR_VERSION).unwrap()());
            version = Version::getVersionForNumber(versionNumber)?;
            let bitsNeeded = calculateBitsNeeded(mode, &header_bits, &data_bits, version);
            if !willFit(bitsNeeded, version, &ec_level) {
                return Err(Exceptions::WriterException(Some(
                    "Data too big for requested version".to_owned(),
                )));
            }
        } else {
            version = recommendVersion(&ec_level, mode, &header_bits, &data_bits)?;
        }

        header_and_data_bits = BitArray::new();
        header_and_data_bits.appendBitArray(header_bits);
        // Find "length" of main segment and write it
        let num_letters = if mode == Mode::BYTE {
            data_bits.getSizeInBytes()
        } else {
            content.graphemes(true).count()
        };
        appendLengthInfo(num_letters as u32, version, mode, &mut header_and_data_bits)?;
        // Put data together into the overall payload
        header_and_data_bits.appendBitArray(data_bits);
    }

    let ec_blocks = version.getECBlocksForLevel(ec_level);
    let num_data_bytes = version.getTotalCodewords() - ec_blocks.getTotalECCodewords();

    // Terminate the bits properly.
    terminateBits(num_data_bytes, &mut header_and_data_bits)?;

    // Interleave data bits with error correction code.
    let final_bits = interleaveWithECBytes(
        &header_and_data_bits,
        version.getTotalCodewords(),
        num_data_bytes,
        ec_blocks.getNumBlocks(),
    )?;

    let mut qrCode = QRCode::new();

    qrCode.setECLevel(ec_level);
    qrCode.setMode(mode);
    qrCode.setVersion(version);

    //  Choose the mask pattern and set to "qrCode".
    let dimension = version.getDimensionForVersion();
    let mut matrix = ByteMatrix::new(dimension, dimension);

    // Enable manual selection of the pattern to be used via hint
    let mut mask_pattern = -1;
    if hints.contains_key(&EncodeHintType::QR_MASK_PATTERN) {
        let hint_mask_pattern = if let EncodeHintValue::QrMaskPattern(v) =
            hints.get(&EncodeHintType::QR_MASK_PATTERN).unwrap()
        {
            if let Ok(vb) = v.parse::<i32>() {
                vb
            } else {
                -1
            }
        } else {
            -1
        };
        // let hintMaskPattern = Integer.parseInt(hints.get(&EncodeHintType::QR_MASK_PATTERN).unwrap());
        mask_pattern = if QRCode::isValidMaskPattern(hint_mask_pattern) {
            hint_mask_pattern
        } else {
            -1
        };
    }

    if mask_pattern == -1 {
        mask_pattern = chooseMaskPattern(&final_bits, &ec_level, version, &mut matrix)? as i32;
    }
    qrCode.setMaskPattern(mask_pattern);

    // Build the matrix and set it to "qrCode".
    matrix_util::buildMatrix(&final_bits, &ec_level, version, mask_pattern, &mut matrix)?;
    qrCode.setMatrix(matrix);

    Ok(qrCode)
}

/**
 * Decides the smallest version of QR code that will contain all of the provided data.
 *
 * @throws WriterException if the data cannot fit in any version
 */
fn recommendVersion(
    ec_level: &ErrorCorrectionLevel,
    mode: Mode,
    header_bits: &BitArray,
    data_bits: &BitArray,
) -> Result<VersionRef, Exceptions> {
    // Hard part: need to know version to know how many bits length takes. But need to know how many
    // bits it takes to know version. First we take a guess at version by assuming version will be
    // the minimum, 1:
    let provisional_bits_needed = calculateBitsNeeded(
        mode,
        header_bits,
        data_bits,
        Version::getVersionForNumber(1)?,
    );
    let provisional_version = chooseVersion(provisional_bits_needed, ec_level)?;

    // Use that guess to calculate the right version. I am still not sure this works in 100% of cases.
    let bits_needed = calculateBitsNeeded(mode, header_bits, data_bits, provisional_version);

    chooseVersion(bits_needed, ec_level)
}

fn calculateBitsNeeded(
    mode: Mode,
    header_bits: &BitArray,
    data_bits: &BitArray,
    version: VersionRef,
) -> u32 {
    (header_bits.getSize() + mode.getCharacterCountBits(version) as usize + data_bits.getSize())
        as u32
}

/**
 * @return the code point of the table used in alphanumeric mode or
 *  -1 if there is no corresponding code in the table.
 */
pub fn getAlphanumericCode(code: u32) -> i8 {
    let code = code as usize;
    if code < ALPHANUMERIC_TABLE.len() {
        ALPHANUMERIC_TABLE[code]
    } else {
        -1
    }
}

pub fn chooseMode(content: &str) -> Mode {
    chooseModeWithEncoding(content, encoding::all::ISO_8859_1)
}

/**
 * Choose the best mode by examining the content. Note that 'encoding' is used as a hint;
 * if it is Shift_JIS, and the input is only double-byte Kanji, then we return {@link Mode#KANJI}.
 */
fn chooseModeWithEncoding(content: &str, encoding: EncodingRef) -> Mode {
    if SHIFT_JIS_CHARSET.name() == encoding.name() && isOnlyDoubleByteKanji(content) {
        // if (StringUtils.SHIFT_JIS_CHARSET.equals(encoding) && isOnlyDoubleByteKanji(content)) {
        // Choose Kanji mode if all input are double-byte characters
        return Mode::KANJI;
    }
    let mut has_numeric = false;
    let mut has_alphanumeric = false;
    for i in 0..content.len() {
        // for (int i = 0; i < content.length(); ++i) {
        let c = content.chars().nth(i).unwrap();
        if ('0'..='9').contains(&c) {
            has_numeric = true;
        } else if getAlphanumericCode(c as u32) != -1 {
            has_alphanumeric = true;
        } else {
            return Mode::BYTE;
        }
    }
    if has_alphanumeric {
        return Mode::ALPHANUMERIC;
    }
    if has_numeric {
        return Mode::NUMERIC;
    }
    Mode::BYTE
}

pub fn isOnlyDoubleByteKanji(content: &str) -> bool {
    let bytes = if let Ok(byt) = SHIFT_JIS_CHARSET.encode(content, encoding::EncoderTrap::Strict) {
        byt
    } else {
        return false;
    };
    let length = bytes.len();
    if length % 2 != 0 {
        return false;
    }
    let mut i = 0;
    while i < length {
        // for (int i = 0; i < length; i += 2) {
        let byte1 = bytes[i];
        if !(0x81..=0x9F).contains(&byte1) && !(0xE0..=0xEB).contains(&byte1) {
            return false;
        }
        i += 2;
    }
    true
}

fn chooseMaskPattern(
    bits: &BitArray,
    ec_level: &ErrorCorrectionLevel,
    version: VersionRef,
    matrix: &mut ByteMatrix,
) -> Result<u32, Exceptions> {
    let mut min_penalty = u32::MAX; // Lower penalty is better.
    let mut best_mask_pattern = -1;
    // We try all mask patterns to choose the best one.
    for maskPattern in 0..QRCode::NUM_MASK_PATTERNS {
        // for (int maskPattern = 0; maskPattern < QRCode.NUM_MASK_PATTERNS; maskPattern++) {
        let mut matrix = matrix.clone();
        matrix_util::buildMatrix(bits, ec_level, version, maskPattern, &mut matrix)?;
        let penalty = calculateMaskPenalty(&matrix);
        if penalty < min_penalty {
            min_penalty = penalty;
            best_mask_pattern = maskPattern;
        }
    }
    Ok(best_mask_pattern as u32)
}

fn chooseVersion(
    numInputBits: u32,
    ecLevel: &ErrorCorrectionLevel,
) -> Result<VersionRef, Exceptions> {
    for versionNum in 1..=40 {
        // for (int versionNum = 1; versionNum <= 40; versionNum++) {
        let version = Version::getVersionForNumber(versionNum)?;
        if willFit(numInputBits, version, ecLevel) {
            return Ok(version);
        }
    }
    Err(Exceptions::WriterException(Some("Data too big".to_owned())))
}

/**
 * @return true if the number of input bits will fit in a code with the specified version and
 * error correction level.
 */
pub fn willFit(numInputBits: u32, version: VersionRef, ecLevel: &ErrorCorrectionLevel) -> bool {
    // In the following comments, we use numbers of Version 7-H.
    // numBytes = 196
    let num_bytes = version.getTotalCodewords();
    // getNumECBytes = 130
    let ec_blocks = version.getECBlocksForLevel(*ecLevel);
    let num_ec_bytes = ec_blocks.getTotalECCodewords();
    // getNumDataBytes = 196 - 130 = 66
    let num_data_bytes = num_bytes - num_ec_bytes;
    let total_input_bytes = (numInputBits + 7) / 8;
    num_data_bytes >= total_input_bytes
}

/**
 * Terminate bits as described in 8.4.8 and 8.4.9 of JISX0510:2004 (p.24).
 */
pub fn terminateBits(num_data_bytes: u32, bits: &mut BitArray) -> Result<(), Exceptions> {
    let capacity = num_data_bytes * 8;
    if bits.getSize() > capacity as usize {
        return Err(Exceptions::WriterException(Some(format!(
            "data bits cannot fit in the QR Code{} > ",
            capacity
        ))));
        // throw new WriterException("data bits cannot fit in the QR Code" + bits.getSize() + " > " +
        //     capacity);
    }
    // Append Mode.TERMINATE if there is enough space (value is 0000)
    for _i in 0..4 {
        if bits.getSize() >= capacity as usize {
            break;
        }
        // }
        // for (int i = 0; i < 4 && bits.getSize() < capacity; ++i) {
        bits.appendBit(false);
    }
    // Append termination bits. See 8.4.8 of JISX0510:2004 (p.24) for details.
    // If the last byte isn't 8-bit aligned, we'll add padding bits.
    let num_bits_in_last_byte = bits.getSize() & 0x07;
    if num_bits_in_last_byte > 0 {
        for _i in num_bits_in_last_byte..8 {
            // for (int i = numBitsInLastByte; i < 8; i++) {
            bits.appendBit(false);
        }
    }
    // If we have more space, we'll fill the space with padding patterns defined in 8.4.9 (p.24).
    let num_padding_bytes = num_data_bytes as isize - bits.getSizeInBytes() as isize;
    for i in 0..num_padding_bytes {
        if i >= num_padding_bytes {
            break;
        }
        // for (int i = 0; i < numPaddingBytes; ++i) {
        bits.appendBits(if (i & 0x01) == 0 { 0xEC } else { 0x11 }, 8)?;
    }
    if bits.getSize() != capacity as usize {
        return Err(Exceptions::WriterException(Some(
            "Bits size does not equal capacity".to_owned(),
        )));
        // throw new WriterException("Bits size does not equal capacity");
    }
    Ok(())
}

/**
 * Get number of data bytes and number of error correction bytes for block id "blockID". Store
 * the result in "numDataBytesInBlock", and "numECBytesInBlock". See table 12 in 8.5.1 of
 * JISX0510:2004 (p.30)
 */
pub fn getNumDataBytesAndNumECBytesForBlockID(
    num_total_bytes: u32,
    num_data_bytes: u32,
    num_rsblocks: u32,
    block_id: u32,
    // numDataBytesInBlock: &mut [u32],
    // numECBytesInBlock: &mut [u32],
) -> Result<(u32, u32), Exceptions> {
    if block_id >= num_rsblocks {
        return Err(Exceptions::WriterException(Some(
            "Block ID too large".to_owned(),
        )));
        // throw new WriterException("Block ID too large");
    }
    // numRsBlocksInGroup2 = 196 % 5 = 1
    let num_rs_blocks_in_group2 = num_total_bytes % num_rsblocks;
    // numRsBlocksInGroup1 = 5 - 1 = 4
    let num_rs_blocks_in_group1 = num_rsblocks - num_rs_blocks_in_group2;
    // numTotalBytesInGroup1 = 196 / 5 = 39
    let num_total_bytes_in_group1 = num_total_bytes / num_rsblocks;
    // numTotalBytesInGroup2 = 39 + 1 = 40
    let num_total_bytes_in_group2 = num_total_bytes_in_group1 + 1;
    // numDataBytesInGroup1 = 66 / 5 = 13
    let num_data_bytes_in_group1 = num_data_bytes / num_rsblocks;
    // numDataBytesInGroup2 = 13 + 1 = 14
    let num_data_bytes_in_group2 = num_data_bytes_in_group1 + 1;
    // numEcBytesInGroup1 = 39 - 13 = 26
    let num_ec_bytes_in_group1 = num_total_bytes_in_group1 - num_data_bytes_in_group1;
    // numEcBytesInGroup2 = 40 - 14 = 26
    let numEcBytesInGroup2 = num_total_bytes_in_group2 - num_data_bytes_in_group2;
    // Sanity checks.
    // 26 = 26
    if num_ec_bytes_in_group1 != numEcBytesInGroup2 {
        return Err(Exceptions::WriterException(Some(
            "EC bytes mismatch".to_owned(),
        )));
        // throw new WriterException("EC bytes mismatch");
    }
    // 5 = 4 + 1.
    if num_rsblocks != num_rs_blocks_in_group1 + num_rs_blocks_in_group2 {
        return Err(Exceptions::WriterException(Some(
            "RS blocks mismatch".to_owned(),
        )));

        // throw new WriterException("RS blocks mismatch");
    }
    // 196 = (13 + 26) * 4 + (14 + 26) * 1
    if num_total_bytes
        != ((num_data_bytes_in_group1 + num_ec_bytes_in_group1) * num_rs_blocks_in_group1)
            + ((num_data_bytes_in_group2 + numEcBytesInGroup2) * num_rs_blocks_in_group2)
    {
        return Err(Exceptions::WriterException(Some(
            "Total bytes mismatch".to_owned(),
        )));

        // throw new WriterException("Total bytes mismatch");
    }

    Ok(if block_id < num_rs_blocks_in_group1 {
        (num_data_bytes_in_group1, num_ec_bytes_in_group1)
    } else {
        (num_data_bytes_in_group2, numEcBytesInGroup2)
    })
}

/**
 * Interleave "bits" with corresponding error correction bytes. On success, store the result in
 * "result". The interleave rule is complicated. See 8.6 of JISX0510:2004 (p.37) for details.
 */
pub fn interleaveWithECBytes(
    bits: &BitArray,
    num_total_bytes: u32,
    num_data_bytes: u32,
    num_rsblocks: u32,
) -> Result<BitArray, Exceptions> {
    // "bits" must have "getNumDataBytes" bytes of data.
    if bits.getSizeInBytes() as u32 != num_data_bytes {
        return Err(Exceptions::WriterException(Some(
            "Number of bits and data bytes does not match".to_owned(),
        )));
    }

    // Step 1.  Divide data bytes into blocks and generate error correction bytes for them. We'll
    // store the divided data bytes blocks and error correction bytes blocks into "blocks".
    let mut data_bytes_offset = 0;
    let mut max_num_data_bytes = 0;
    let mut max_num_ec_bytes = 0;

    // Since, we know the number of reedsolmon blocks, we can initialize the vector with the number.
    let mut blocks = Vec::new();

    for i in 0..num_rsblocks {
        // for (int i = 0; i < numRSBlocks; ++i) {
        // let mut numDataBytesInBlock = vec![0; 1]; //new int[1];
        // let mut numEcBytesInBlock = vec![0; 1]; //new int[1];
        let (numDataBytesInBlock, numEcBytesInBlock) = getNumDataBytesAndNumECBytesForBlockID(
            num_total_bytes,
            num_data_bytes,
            num_rsblocks,
            i,
            // &mut numDataBytesInBlock,
            // &mut numEcBytesInBlock,
        )?;

        let size = numDataBytesInBlock;
        let mut dataBytes = vec![0u8; size as usize];
        bits.toBytes(8 * data_bytes_offset, &mut dataBytes, 0, size as usize);
        let ec_bytes = generateECBytes(&dataBytes, numEcBytesInBlock as usize);
        blocks.push(BlockPair::new(dataBytes, ec_bytes.clone()));

        max_num_data_bytes = max_num_data_bytes.max(size);
        max_num_ec_bytes = max_num_ec_bytes.max(ec_bytes.len());
        data_bytes_offset += numDataBytesInBlock as usize;
    }
    if num_data_bytes != data_bytes_offset as u32 {
        return Err(Exceptions::WriterException(Some(
            "Data bytes does not match offset".to_owned(),
        )));
    }

    let mut result = BitArray::new();

    // First, place data blocks.
    for i in 0..max_num_data_bytes as usize {
        // for (int i = 0; i < maxNumDataBytes; ++i) {
        for block in &blocks {
            // for (BlockPair block : blocks) {
            let data_bytes = block.getDataBytes();
            if i < data_bytes.len() {
                result.appendBits(data_bytes[i] as u32, 8)?;
            }
        }
    }
    // Then, place error correction blocks.
    for i in 0..max_num_ec_bytes {
        // for (int i = 0; i < maxNumEcBytes; ++i) {
        for block in &blocks {
            // for (BlockPair block : blocks) {
            let ec_bytes = block.getErrorCorrectionBytes();
            if i < ec_bytes.len() {
                result.appendBits(ec_bytes[i] as u32, 8)?;
            }
        }
    }
    if num_total_bytes != result.getSizeInBytes() as u32 {
        // Should be same.
        return Err(Exceptions::WriterException(Some(format!(
            "Interleaving error: {} and {} differ.",
            num_total_bytes,
            result.getSizeInBytes()
        ))));
        // throw new WriterException("Interleaving error: " + numTotalBytes + " and " +
        //     result.getSizeInBytes() + " differ.");
    }

    Ok(result)
}

pub fn generateECBytes(dataBytes: &[u8], num_ec_bytes_in_block: usize) -> Vec<u8> {
    let num_data_bytes = dataBytes.len();
    let mut to_encode = vec![0; num_data_bytes + num_ec_bytes_in_block];
    for i in 0..num_data_bytes {
        // for (int i = 0; i < numDataBytes; i++) {
        to_encode[i] = dataBytes[i] as i32;
    }

    ReedSolomonEncoder::new(get_predefined_genericgf(
        PredefinedGenericGF::QrCodeField256,
    ))
    .encode(&mut to_encode, num_ec_bytes_in_block)
    .expect("rs encode must complete");

    let mut ecBytes = vec![0u8; num_ec_bytes_in_block];
    for i in 0..num_ec_bytes_in_block {
        // for (int i = 0; i < numEcBytesInBlock; i++) {
        ecBytes[i] = to_encode[num_data_bytes + i] as u8;
    }
    ecBytes
}

/**
 * Append mode info. On success, store the result in "bits".
 */
pub fn appendModeInfo(mode: Mode, bits: &mut BitArray) -> Result<(), Exceptions> {
    bits.appendBits(mode.getBits() as u32, 4)?;
    Ok(())
}

/**
 * Append length info. On success, store the result in "bits".
 */
pub fn appendLengthInfo(
    num_letters: u32,
    version: VersionRef,
    mode: Mode,
    bits: &mut BitArray,
) -> Result<(), Exceptions> {
    let numBits = mode.getCharacterCountBits(version);
    if num_letters >= (1 << numBits) {
        return Err(Exceptions::WriterException(Some(format!(
            "{} is bigger than {}",
            num_letters,
            ((1 << numBits) - 1)
        ))));
    }
    bits.appendBits(num_letters, numBits as usize)?;
    Ok(())
}

/**
 * Append "bytes" in "mode" mode (encoding) into "bits". On success, store the result in "bits".
 */
pub fn appendBytes(
    content: &str,
    mode: Mode,
    bits: &mut BitArray,
    encoding: EncodingRef,
) -> Result<(), Exceptions> {
    match mode {
        Mode::NUMERIC => appendNumericBytes(content, bits),
        Mode::ALPHANUMERIC => appendAlphanumericBytes(content, bits),
        Mode::BYTE => append8BitBytes(content, bits, encoding),
        Mode::KANJI => appendKanjiBytes(content, bits),
        _ => Err(Exceptions::WriterException(Some(format!(
            "Invalid mode: {:?}",
            mode
        )))),
    }
    // switch (mode) {
    //   case NUMERIC:
    //     appendNumericBytes(content, bits);
    //     break;
    //   case ALPHANUMERIC:
    //     appendAlphanumericBytes(content, bits);
    //     break;
    //   case BYTE:
    //     append8BitBytes(content, bits, encoding);
    //     break;
    //   case KANJI:
    //     appendKanjiBytes(content, bits);
    //     break;
    //   default:
    //     throw new WriterException("Invalid mode: " + mode);
    // }
}

pub fn appendNumericBytes(content: &str, bits: &mut BitArray) -> Result<(), Exceptions> {
    let length = content.len();
    let mut i = 0;
    while i < length {
        let num1 = content.chars().nth(i).unwrap() as u8 - b'0';
        if i + 2 < length {
            // Encode three numeric letters in ten bits.
            let num2 = content.chars().nth(i + 1).unwrap() as u8 - b'0';
            let num3 = content.chars().nth(i + 2).unwrap() as u8 - b'0';
            bits.appendBits(num1 as u32 * 100 + num2 as u32 * 10 + num3 as u32, 10)?;
            i += 3;
        } else if i + 1 < length {
            // Encode two numeric letters in seven bits.
            let num2 = content.chars().nth(i + 1).unwrap() as u8 - b'0';
            bits.appendBits(num1 as u32 * 10 + num2 as u32, 7)?;
            i += 2;
        } else {
            // Encode one numeric letter in four bits.
            bits.appendBits(num1 as u32, 4)?;
            i += 1;
        }
    }
    Ok(())
}

pub fn appendAlphanumericBytes(content: &str, bits: &mut BitArray) -> Result<(), Exceptions> {
    let length = content.len();
    let mut i = 0;
    while i < length {
        let code1 = getAlphanumericCode(content.chars().nth(i).unwrap() as u32);
        if code1 == -1 {
            return Err(Exceptions::WriterException(None));
        }
        if i + 1 < length {
            let code2 = getAlphanumericCode(content.chars().nth(i + 1).unwrap() as u32);
            if code2 == -1 {
                return Err(Exceptions::WriterException(None));
            }
            // Encode two alphanumeric letters in 11 bits.
            bits.appendBits((code1 as i16 * 45 + code2 as i16) as u32, 11)?;
            i += 2;
        } else {
            // Encode one alphanumeric letter in six bits.
            bits.appendBits(code1 as u32, 6)?;
            i += 1;
        }
    }
    Ok(())
}

pub fn append8BitBytes(
    content: &str,
    bits: &mut BitArray,
    encoding: EncodingRef,
) -> Result<(), Exceptions> {
    let bytes = encoding
        .encode(content, encoding::EncoderTrap::Strict)
        .expect("should encode");
    // let bytes = content.getBytes(encoding);
    for b in bytes {
        // for (byte b : bytes) {
        bits.appendBits(b as u32, 8)?;
    }
    Ok(())
}

pub fn appendKanjiBytes(content: &str, bits: &mut BitArray) -> Result<(), Exceptions> {
    let sjis = &SHIFT_JIS_CHARSET; //encoding::label::encoding_from_whatwg_label("SJIS").unwrap();

    let bytes = sjis
        .encode(content, encoding::EncoderTrap::Strict)
        .expect("should encode fine");
    // let bytes = content.getBytes(StringUtils::SHIFT_JIS_CHARSET);
    if bytes.len() % 2 != 0 {
        return Err(Exceptions::WriterException(Some(
            "Kanji byte size not even".to_owned(),
        )));
    }
    let max_i = bytes.len() - 1; // bytes.length must be even
    let mut i = 0;
    while i < max_i {
        // for (int i = 0; i < maxI; i += 2) {
        let byte1 = bytes[i]; // & 0xFF;
        let byte2 = bytes[i + 1]; // & 0xFF;
        let code: u16 = ((byte1 as u16) << 8u16) | byte2 as u16;
        let mut subtracted: i32 = -1;
        if (0x8140..=0x9ffc).contains(&code) {
            subtracted = code as i32 - 0x8140;
        } else if (0xe040..=0xebbf).contains(&code) {
            subtracted = code as i32 - 0xc140;
        }
        if subtracted == -1 {
            return Err(Exceptions::WriterException(Some(
                "Invalid byte sequence".to_owned(),
            )));
        }
        let encoded = ((subtracted >> 8) * 0xc0) + (subtracted & 0xff);
        bits.appendBits(encoded as u32, 13)?;

        i += 2;
    }
    Ok(())
}

fn appendECI(eci: &CharacterSetECI, bits: &mut BitArray) -> Result<(), Exceptions> {
    bits.appendBits(Mode::ECI.getBits() as u32, 4)?;
    // This is correct for values up to 127, which is all we need now.
    bits.appendBits(eci.getValueSelf(), 8)?;
    Ok(())
}
