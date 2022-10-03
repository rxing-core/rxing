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

// package com.google.zxing.qrcode.encoder;

// import com.google.zxing.EncodeHintType;
// import com.google.zxing.WriterException;
// import com.google.zxing.common.BitArray;
// import com.google.zxing.common.StringUtils;
// import com.google.zxing.common.CharacterSetECI;
// import com.google.zxing.common.reedsolomon.GenericGF;
// import com.google.zxing.common.reedsolomon.ReedSolomonEncoder;
// import com.google.zxing.qrcode.decoder.ErrorCorrectionLevel;
// import com.google.zxing.qrcode.decoder.Mode;
// import com.google.zxing.qrcode.decoder.Version;

// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;
// import java.util.ArrayList;
// import java.util.Collection;
// import java.util.Map;

use std::collections::HashMap;

use encoding::EncodingRef;

use lazy_static::lazy_static;

use crate::{EncodingHintDictionary, common::{BitArray, CharacterSetECI, reedsolomon::{ReedSolomonEncoder, get_predefined_genericgf, PredefinedGenericGF}, StringUtils}, Exceptions, qrcode::decoder::{ErrorCorrectionLevel, Mode, VersionRef, Version}, EncodeHintType, EncodeHintValue};

use super::{mask_util, ByteMatrix, QRCode, matrix_util, MinimalEncoder, BlockPair};

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

 lazy_static !{
static ref SHIFT_JIS_CHARSET : EncodingRef = encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
 }

  // The original table is defined in the table 5 of JISX0510:2004 (p.19).
  const ALPHANUMERIC_TABLE : [i8;96]= [
      -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  // 0x00-0x0f
      -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  // 0x10-0x1f
      36, -1, -1, -1, 37, 38, -1, -1, -1, -1, 39, 40, -1, 41, 42, 43,  // 0x20-0x2f
      0,   1,  2,  3,  4,  5,  6,  7,  8,  9, 44, -1, -1, -1, -1, -1,  // 0x30-0x3f
      -1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,  // 0x40-0x4f
      25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, -1,  // 0x50-0x5f
  ];

  const DEFAULT_BYTE_MODE_ENCODING : EncodingRef = encoding::all::ISO_8859_1;


  // The mask penalty calculation is complicated.  See Table 21 of JISX0510:2004 (p.45) for details.
  // Basically it applies four rules and summate all penalties.
  pub fn calculateMaskPenalty( matrix:&ByteMatrix) -> u32{
    return mask_util::applyMaskPenaltyRule1(matrix)
        + mask_util::applyMaskPenaltyRule2(matrix)
        + mask_util::applyMaskPenaltyRule3(matrix)
        + mask_util::applyMaskPenaltyRule4(matrix);
  }

  /**
   * @param content text to encode
   * @param ecLevel error correction level to use
   * @return {@link QRCode} representing the encoded QR code
   * @throws WriterException if encoding can't succeed, because of for example invalid content
   *   or configuration
   */
  pub fn encode( content:&str,  ecLevel:ErrorCorrectionLevel) -> Result<QRCode, Exceptions> {
    return encode_with_hints(content, ecLevel, HashMap::new());
  }

  pub fn encode_with_hints( content:&str,
                               ecLevel:ErrorCorrectionLevel,
                               hints:EncodingHintDictionary) -> Result<QRCode, Exceptions> {

    let version;
    let headerAndDataBits;
    let mode;

    let hasGS1FormatHint =  hints.contains_key(&EncodeHintType::GS1_FORMAT) &&
    if let EncodeHintValue::Gs1Format(v) = hints.get(&EncodeHintType::GS1_FORMAT).unwrap() {
      if let Ok(vb) = v.parse::<bool>() {
        vb
      }else {
        false
      }
    }else {
      false
    };
    let hasCompactionHint =  hints.contains_key(&EncodeHintType::QR_COMPACT) &&
    if let EncodeHintValue::QrCompact(v) = hints.get(&EncodeHintType::QR_COMPACT).unwrap() {
      if let Ok(vb) = v.parse::<bool>() {
        vb
      }else {
        false
      }
    }else {
      false
    };

    // Determine what character encoding has been specified by the caller, if any
    let encoding = DEFAULT_BYTE_MODE_ENCODING;
    let hasEncodingHint = hints.contains_key(&EncodeHintType::CHARACTER_SET);
    if hasEncodingHint {
       if let EncodeHintValue::CharacterSet(v) = hints.get(&EncodeHintType::CHARACTER_SET).unwrap() {
        encoding = encoding::label::encoding_from_whatwg_label(v).unwrap()
      }
      // encoding = encoding::label::encoding_from_whatwg_label(hints.get(&EncodeHintType::CHARACTER_SET).unwrap());
    }

    if hasCompactionHint {
      mode = Mode::BYTE;

      let priorityEncoding = encoding; //if encoding.name() == DEFAULT_BYTE_MODE_ENCODING.name()  {None} else {Some(encoding)};
      let rn = MinimalEncoder::encode_with_details(content, None, priorityEncoding, hasGS1FormatHint, ecLevel)?;

      headerAndDataBits =  BitArray::new();
      rn.getBits(&headerAndDataBits);
      version = rn.getVersion();

    } else {
    
      // Pick an encoding mode appropriate for the content. Note that this will not attempt to use
      // multiple modes / segments even if that were more efficient.
      mode = chooseModeWithEncoding(content, encoding);
  
      // This will store the header information, like mode and
      // length, as well as "header" segments like an ECI segment.
      let headerBits =  BitArray::new();
  
      // Append ECI segment if applicable
      if mode == Mode::BYTE && hasEncodingHint {
        let eci = CharacterSetECI::getCharacterSetECI(encoding);
        if eci.is_some() {
          appendECI(&eci.unwrap(), &headerBits);
        }
      }
  
      // Append the FNC1 mode header for GS1 formatted data if applicable
      if hasGS1FormatHint {
        // GS1 formatted codes are prefixed with a FNC1 in first position mode header
        appendModeInfo(Mode::FNC1_FIRST_POSITION, &headerBits);
      }
    
      // (With ECI in place,) Write the mode marker
      appendModeInfo(mode, &headerBits);
  
      // Collect data within the main segment, separately, to count its size if needed. Don't add it to
      // main payload yet.
      let dataBits =  BitArray::new();
      appendBytes(content, mode, &dataBits, encoding);
  
      if  hints.contains_key(&EncodeHintType::QR_VERSION) {
        let versionNumber = if let EncodeHintValue::QrVersion(v) = hints.get(&EncodeHintType::QR_VERSION).unwrap() {
          if let Ok(vb) = v.parse::<u32>() {
            vb
          }else {
            0
          }
        }else {
          0
        };
        // let versionNumber = Integer.parseInt(hints.get(&EncodeHintType::QR_VERSION).unwrap()());
        version = Version::getVersionForNumber(versionNumber)?;
        let bitsNeeded = calculateBitsNeeded(mode, &headerBits, &dataBits, version);
        if !willFit(bitsNeeded, version, &ecLevel) {
          return Err(Exceptions::WriterException("Data too big for requested version".to_owned()))
        }
      } else {
        version = recommendVersion(&ecLevel, mode, &headerBits, &dataBits)?;
      }
    
      headerAndDataBits =  BitArray::new();
      headerAndDataBits.appendBitArray(headerBits);
      // Find "length" of main segment and write it
      let numLetters = if mode == Mode::BYTE  {dataBits.getSizeInBytes() }else {content.len()};
      appendLengthInfo(numLetters as u32, version, mode, &headerAndDataBits);
      // Put data together into the overall payload
      headerAndDataBits.appendBitArray(dataBits);
    }

    let ecBlocks = version.getECBlocksForLevel(ecLevel);
    let numDataBytes = version.getTotalCodewords() - ecBlocks.getTotalECCodewords();

    // Terminate the bits properly.
    terminateBits(numDataBytes, &headerAndDataBits);

    // Interleave data bits with error correction code.
    let finalBits = interleaveWithECBytes(&headerAndDataBits,
                                               version.getTotalCodewords(),
                                               numDataBytes,
                                               ecBlocks.getNumBlocks())?;

                                               let qrCode =  QRCode::new();

    qrCode.setECLevel(ecLevel);
    qrCode.setMode(mode);
    qrCode.setVersion(version);

    //  Choose the mask pattern and set to "qrCode".
    let dimension = version.getDimensionForVersion();
    let matrix =  ByteMatrix::new(dimension, dimension);

    // Enable manual selection of the pattern to be used via hint
    let maskPattern = -1;
    if hints.contains_key(&EncodeHintType::QR_MASK_PATTERN) {
      let hintMaskPattern =if let EncodeHintValue::QrMaskPattern(v) = hints.get(&EncodeHintType::QR_MASK_PATTERN).unwrap() {
        if let Ok(vb) = v.parse::<i32>() {
          vb
        }else {
          -1
        }
      }else {
        -1
      };
      // let hintMaskPattern = Integer.parseInt(hints.get(&EncodeHintType::QR_MASK_PATTERN).unwrap());
      maskPattern = if QRCode::isValidMaskPattern(hintMaskPattern)  {hintMaskPattern} else {-1};
    }

    if maskPattern == -1 {
      maskPattern = chooseMaskPattern(&finalBits, &ecLevel, version, &matrix)? as i32;
    }
    qrCode.setMaskPattern(maskPattern);

    // Build the matrix and set it to "qrCode".
    matrix_util::buildMatrix(&finalBits, &ecLevel, version, maskPattern, &mut matrix);
    qrCode.setMatrix(matrix);

    Ok( qrCode)
  }

  /**
   * Decides the smallest version of QR code that will contain all of the provided data.
   *
   * @throws WriterException if the data cannot fit in any version
   */
  fn recommendVersion( ecLevel:&ErrorCorrectionLevel,
                                           mode:Mode,
                                           headerBits:&BitArray,
                                           dataBits:&BitArray) -> Result<VersionRef,Exceptions> {
    // Hard part: need to know version to know how many bits length takes. But need to know how many
    // bits it takes to know version. First we take a guess at version by assuming version will be
    // the minimum, 1:
    let provisionalBitsNeeded = calculateBitsNeeded(mode, headerBits, dataBits, Version::getVersionForNumber(1)?);
    let provisionalVersion = chooseVersion(provisionalBitsNeeded, ecLevel)?;

    // Use that guess to calculate the right version. I am still not sure this works in 100% of cases.
    let bitsNeeded = calculateBitsNeeded(mode, headerBits, dataBits, provisionalVersion);
    return chooseVersion(bitsNeeded, ecLevel);
  }

  fn  calculateBitsNeeded( mode:Mode,
                                          headerBits:&BitArray,
                                          dataBits:&BitArray,
                                          version:VersionRef) -> u32 {
    (headerBits.getSize() + mode.getCharacterCountBits(version) as usize + dataBits.getSize()) as u32
  }

  /**
   * @return the code point of the table used in alphanumeric mode or
   *  -1 if there is no corresponding code in the table.
   */
  pub fn getAlphanumericCode( code:u32) -> i8{
    let code = code as usize;
    if code < ALPHANUMERIC_TABLE.len() {
       ALPHANUMERIC_TABLE[code]
    }else{
      -1
    }
  }

  pub fn chooseMode( content:&str) -> Mode{
    return chooseModeWithEncoding(content, encoding::all::ISO_8859_1);
  }

  /**
   * Choose the best mode by examining the content. Note that 'encoding' is used as a hint;
   * if it is Shift_JIS, and the input is only double-byte Kanji, then we return {@link Mode#KANJI}.
   */
  fn chooseModeWithEncoding( content:&str,  encoding:EncodingRef) -> Mode{
    if SHIFT_JIS_CHARSET.name() == encoding.name() && isOnlyDoubleByteKanji(content) {
    // if (StringUtils.SHIFT_JIS_CHARSET.equals(encoding) && isOnlyDoubleByteKanji(content)) {
      // Choose Kanji mode if all input are double-byte characters
      return Mode::KANJI;
    }
    let hasNumeric = false;
    let hasAlphanumeric = false;
    for i in 0..content.len() {
    // for (int i = 0; i < content.length(); ++i) {
      let c = content.chars().nth(i).unwrap();
      if c >= '0' && c <= '9' {
        hasNumeric = true;
      } else if (getAlphanumericCode(c as u32) != -1) {
        hasAlphanumeric = true;
      } else {
        return Mode::BYTE;
      }
    }
    if (hasAlphanumeric) {
      return Mode::ALPHANUMERIC;
    }
    if (hasNumeric) {
      return Mode::NUMERIC;
    }
    return Mode::BYTE;
  }

  pub fn isOnlyDoubleByteKanji( content:&str) -> bool{
    let bytes = SHIFT_JIS_CHARSET.encode(content,encoding::EncoderTrap::Strict).expect("encode");
    let length = bytes.len();
    if length % 2 != 0 {
      return false;
    }
    let mut i = 0;
    while i < length {
    // for (int i = 0; i < length; i += 2) {
      let byte1 = bytes[i] & 0xFF;
      if (byte1 < 0x81 || byte1 > 0x9F) && (byte1 < 0xE0 || byte1 > 0xEB) {
        return false;
      }
      i+=2;
    }
    return true;
  }

  fn chooseMaskPattern( bits:&BitArray,
                                        ecLevel:&ErrorCorrectionLevel,
                                        version:VersionRef,
                                        matrix:&ByteMatrix) -> Result<u32, Exceptions> {

    let minPenalty = u32::MAX;  // Lower penalty is better.
    let bestMaskPattern = -1;
    // We try all mask patterns to choose the best one.
    for maskPattern in 0..QRCode::NUM_MASK_PATTERNS {
    // for (int maskPattern = 0; maskPattern < QRCode.NUM_MASK_PATTERNS; maskPattern++) {
      matrix_util::buildMatrix(bits, ecLevel, version, maskPattern, &mut matrix);
      let penalty = calculateMaskPenalty(matrix);
      if penalty < minPenalty {
        minPenalty = penalty;
        bestMaskPattern = maskPattern;
      }
    }
    Ok( bestMaskPattern as u32 )
  }

  fn chooseVersion( numInputBits:u32,  ecLevel:&ErrorCorrectionLevel) -> Result<VersionRef,Exceptions> {
    for versionNum in 1..=40 {
    // for (int versionNum = 1; versionNum <= 40; versionNum++) {
      let version = Version::getVersionForNumber(versionNum)?;
      if willFit(numInputBits, version, ecLevel) {
        return Ok(version);
      }
    }
    Err(Exceptions::WriterException("Data too big".to_owned()))
  }

  /**
   * @return true if the number of input bits will fit in a code with the specified version and
   * error correction level.
   */
  pub fn willFit( numInputBits:u32,  version:VersionRef,  ecLevel:&ErrorCorrectionLevel) -> bool {
    // In the following comments, we use numbers of Version 7-H.
    // numBytes = 196
    let numBytes = version.getTotalCodewords();
    // getNumECBytes = 130
    let ecBlocks = version.getECBlocksForLevel(*ecLevel);
    let numEcBytes = ecBlocks.getTotalECCodewords();
    // getNumDataBytes = 196 - 130 = 66
    let numDataBytes = numBytes - numEcBytes;
    let totalInputBytes = (numInputBits + 7) / 8;
    return numDataBytes >= totalInputBytes;
  }

  /**
   * Terminate bits as described in 8.4.8 and 8.4.9 of JISX0510:2004 (p.24).
   */
  pub fn terminateBits( numDataBytes:u32,  bits:&BitArray) -> Result<(),Exceptions> {
    let capacity = numDataBytes * 8;
    if bits.getSize() > capacity as usize {
      return Err(Exceptions::WriterException(format!("data bits cannot fit in the QR Code{} > " ,
      capacity)))
      // throw new WriterException("data bits cannot fit in the QR Code" + bits.getSize() + " > " +
      //     capacity);
    }
    // Append Mode.TERMINATE if there is enough space (value is 0000)
    for i in 0..4 {
      if bits.getSize() > 0 { break }
    // }
    // for (int i = 0; i < 4 && bits.getSize() < capacity; ++i) {
      bits.appendBit(false);
    }
    // Append termination bits. See 8.4.8 of JISX0510:2004 (p.24) for details.
    // If the last byte isn't 8-bit aligned, we'll add padding bits.
    let numBitsInLastByte = bits.getSize() & 0x07;
    if numBitsInLastByte > 0 {
      for i in numBitsInLastByte..8 {
      // for (int i = numBitsInLastByte; i < 8; i++) {
        bits.appendBit(false);
      }
    }
    // If we have more space, we'll fill the space with padding patterns defined in 8.4.9 (p.24).
    let numPaddingBytes = numDataBytes as usize - bits.getSizeInBytes();
    for i in 0..numPaddingBytes {
    // for (int i = 0; i < numPaddingBytes; ++i) {
      bits.appendBits(if (i & 0x01) == 0  {0xEC} else {0x11}, 8);
    }
    if bits.getSize() != capacity as usize {
      return Err(Exceptions::WriterException("Bits size does not equal capacity".to_owned()))
      // throw new WriterException("Bits size does not equal capacity");
    }
    Ok(())
  }

  /**
   * Get number of data bytes and number of error correction bytes for block id "blockID". Store
   * the result in "numDataBytesInBlock", and "numECBytesInBlock". See table 12 in 8.5.1 of
   * JISX0510:2004 (p.30)
   */
  pub fn getNumDataBytesAndNumECBytesForBlockID( numTotalBytes:u32,
                                                      numDataBytes:u32,
                                                      numRSBlocks:u32,
                                                      blockID:u32,
                                                      numDataBytesInBlock:&[u32],
                                                      numECBytesInBlock:&[u32]) -> Result<(),Exceptions> {
    if blockID >= numRSBlocks {
      return Err(Exceptions::WriterException("Block ID too large".to_owned()))
      // throw new WriterException("Block ID too large");
    }
    // numRsBlocksInGroup2 = 196 % 5 = 1
    let numRsBlocksInGroup2 = numTotalBytes % numRSBlocks;
    // numRsBlocksInGroup1 = 5 - 1 = 4
    let numRsBlocksInGroup1 = numRSBlocks - numRsBlocksInGroup2;
    // numTotalBytesInGroup1 = 196 / 5 = 39
    let numTotalBytesInGroup1 = numTotalBytes / numRSBlocks;
    // numTotalBytesInGroup2 = 39 + 1 = 40
    let numTotalBytesInGroup2 = numTotalBytesInGroup1 + 1;
    // numDataBytesInGroup1 = 66 / 5 = 13
    let numDataBytesInGroup1 = numDataBytes / numRSBlocks;
    // numDataBytesInGroup2 = 13 + 1 = 14
    let numDataBytesInGroup2 = numDataBytesInGroup1 + 1;
    // numEcBytesInGroup1 = 39 - 13 = 26
    let numEcBytesInGroup1 = numTotalBytesInGroup1 - numDataBytesInGroup1;
    // numEcBytesInGroup2 = 40 - 14 = 26
    let numEcBytesInGroup2 = numTotalBytesInGroup2 - numDataBytesInGroup2;
    // Sanity checks.
    // 26 = 26
    if (numEcBytesInGroup1 != numEcBytesInGroup2) {
      return Err(Exceptions::WriterException("EC bytes mismatch".to_owned()))
      // throw new WriterException("EC bytes mismatch");
    }
    // 5 = 4 + 1.
    if (numRSBlocks != numRsBlocksInGroup1 + numRsBlocksInGroup2) {
      return Err(Exceptions::WriterException("RS blocks mismatch".to_owned()))

      // throw new WriterException("RS blocks mismatch");
    }
    // 196 = (13 + 26) * 4 + (14 + 26) * 1
    if (numTotalBytes !=
        ((numDataBytesInGroup1 + numEcBytesInGroup1) *
            numRsBlocksInGroup1) +
            ((numDataBytesInGroup2 + numEcBytesInGroup2) *
                numRsBlocksInGroup2)) {
      return Err(Exceptions::WriterException("Total bytes mismatch".to_owned()))
                  
      // throw new WriterException("Total bytes mismatch");
    }

    if (blockID < numRsBlocksInGroup1) {
      numDataBytesInBlock[0] = numDataBytesInGroup1;
      numECBytesInBlock[0] = numEcBytesInGroup1;
    } else {
      numDataBytesInBlock[0] = numDataBytesInGroup2;
      numECBytesInBlock[0] = numEcBytesInGroup2;
    }
    Ok(())
  }

  /**
   * Interleave "bits" with corresponding error correction bytes. On success, store the result in
   * "result". The interleave rule is complicated. See 8.6 of JISX0510:2004 (p.37) for details.
   */
  pub fn interleaveWithECBytes( bits:&BitArray,
                                         numTotalBytes:u32,
                                         numDataBytes:u32,
                                         numRSBlocks:u32) -> Result<BitArray,Exceptions> {

    // "bits" must have "getNumDataBytes" bytes of data.
    if bits.getSizeInBytes() as u32 != numDataBytes {
      return Err(Exceptions::WriterException("Number of bits and data bytes does not match".to_owned()))
    }

    // Step 1.  Divide data bytes into blocks and generate error correction bytes for them. We'll
    // store the divided data bytes blocks and error correction bytes blocks into "blocks".
    let dataBytesOffset = 0;
    let maxNumDataBytes = 0;
    let maxNumEcBytes = 0;

    // Since, we know the number of reedsolmon blocks, we can initialize the vector with the number.
    let blocks = Vec::new();

    for i in 0..numRSBlocks {
    // for (int i = 0; i < numRSBlocks; ++i) {
      let numDataBytesInBlock = vec![0;1];//new int[1];
      let numEcBytesInBlock = vec![0;1];//new int[1];
      getNumDataBytesAndNumECBytesForBlockID(
          numTotalBytes, numDataBytes, numRSBlocks, i,
          &numDataBytesInBlock, &numEcBytesInBlock);

      let size = numDataBytesInBlock[0];
      let dataBytes = vec![0u8;size as usize];
      bits.toBytes(8 * dataBytesOffset, &mut dataBytes, 0, size as usize);
      let ecBytes = generateECBytes(&dataBytes, numEcBytesInBlock[0] as usize);
      blocks.push( BlockPair::new(dataBytes, ecBytes));

      maxNumDataBytes = maxNumDataBytes.max( size);
      maxNumEcBytes = maxNumEcBytes.max( ecBytes.len());
      dataBytesOffset += numDataBytesInBlock[0] as usize;
    }
    if numDataBytes != dataBytesOffset as u32 {
      return Err(Exceptions::WriterException("Data bytes does not match offset".to_owned()))
    }

    let result =  BitArray::new();

    // First, place data blocks.
    for i in 0..maxNumDataBytes as usize{
    // for (int i = 0; i < maxNumDataBytes; ++i) {
      for block in blocks {
      // for (BlockPair block : blocks) {
        let dataBytes = block.getDataBytes();
        if i < dataBytes.len() {
          result.appendBits(dataBytes[i] as u32, 8);
        }
      }
    }
    // Then, place error correction blocks.
    for i in 0..maxNumEcBytes {
    // for (int i = 0; i < maxNumEcBytes; ++i) {
      for block in blocks {
      // for (BlockPair block : blocks) {
        let ecBytes = block.getErrorCorrectionBytes();
        if (i < ecBytes.len()) {
          result.appendBits(ecBytes[i] as u32, 8);
        }
      }
    }
    if (numTotalBytes != result.getSizeInBytes() as u32) {  // Should be same.
      return Err(
        Exceptions::WriterException(
          format!(
            "Interleaving error: {} and {} differ.",
            numTotalBytes,result.getSizeInBytes()
          )
        )
      )
      // throw new WriterException("Interleaving error: " + numTotalBytes + " and " +
      //     result.getSizeInBytes() + " differ.");
    }

    Ok(result)
  }

  pub fn generateECBytes( dataBytes:&[u8],  numEcBytesInBlock:usize) -> Vec<u8> {
    let numDataBytes = dataBytes.len();
    let toEncode = vec![0;numDataBytes + numEcBytesInBlock];
    for i in 0..numDataBytes {
    // for (int i = 0; i < numDataBytes; i++) {
      toEncode[i] = dataBytes[i] as i32;
    }
  
     ReedSolomonEncoder::new(get_predefined_genericgf(PredefinedGenericGF::QrCodeField256)).encode(&mut toEncode, numEcBytesInBlock);

    let ecBytes = vec![0u8;numEcBytesInBlock];
    for i in 0..numEcBytesInBlock {
    // for (int i = 0; i < numEcBytesInBlock; i++) {
      ecBytes[i] = toEncode[numDataBytes + i] as u8;
    }
    return ecBytes;
  }

  /**
   * Append mode info. On success, store the result in "bits".
   */
  pub fn appendModeInfo( mode:Mode,  bits:&BitArray) {
    bits.appendBits(mode.getBits() as u32, 4);
  }


  /**
   * Append length info. On success, store the result in "bits".
   */
  pub fn appendLengthInfo( numLetters:u32,  version:VersionRef,  mode:Mode,  bits:&BitArray) -> Result<(),Exceptions> {
    let numBits = mode.getCharacterCountBits(version);
    if numLetters >= (1 << numBits) {
      return Err(Exceptions::WriterException(format!("{} is bigger than {}" ,numLetters , ((1 << numBits) - 1))))
    }
    bits.appendBits(numLetters, numBits as usize);
    Ok(())
  }

  /**
   * Append "bytes" in "mode" mode (encoding) into "bits". On success, store the result in "bits".
   */
  pub fn appendBytes( content:&str,
                           mode:Mode,
                           bits:&BitArray,
                           encoding:EncodingRef) -> Result<(),Exceptions>{
                            match mode {
                                Mode::NUMERIC => appendNumericBytes(content, bits),
                                Mode::ALPHANUMERIC => appendAlphanumericBytes(content, bits)?,
                                Mode::BYTE => append8BitBytes(content, bits, encoding),
                                Mode::KANJI => appendKanjiBytes(content, bits)?,
                                _=> return Err(Exceptions::WriterException(format!("Invalid mode: {:?}" , mode)))
                            };
Ok(())
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

  pub fn appendNumericBytes( content:&str,  bits:&BitArray) {
    let length = content.len();
    let i = 0;
    while i < length {
      let num1 = content.chars().nth(i).unwrap() as u8 - b'0';
      if i + 2 < length {
        // Encode three numeric letters in ten bits.
        let num2 = content.chars().nth(i + 1).unwrap() as u8 - b'0';
        let num3 = content.chars().nth(i + 2).unwrap() as u8 - b'0';
        bits.appendBits((num1 * 100 + num2 * 10 + num3) as u32, 10);
        i += 3;
      } else if i + 1 < length {
        // Encode two numeric letters in seven bits.
        let num2 = content.chars().nth(i + 1).unwrap() as u8 - b'0';
        bits.appendBits((num1 * 10 + num2) as u32, 7);
        i += 2;
      } else {
        // Encode one numeric letter in four bits.
        bits.appendBits(num1 as u32, 4);
        i+=1;
      }
    }
  }

  pub fn appendAlphanumericBytes( content:&str,  bits:&BitArray) -> Result<(),Exceptions> {
    let length = content.len();
    let i = 0;
    while i < length {
      let code1 = getAlphanumericCode(content.chars().nth(i).unwrap() as u32);
      if code1 == -1 {
        return Err(Exceptions::WriterException("".to_owned()));
      }
      if i + 1 < length {
        let code2 = getAlphanumericCode(content.chars().nth(i + 1).unwrap() as u32);
        if (code2 == -1) {
          return Err(Exceptions::WriterException("".to_owned()));

        }
        // Encode two alphanumeric letters in 11 bits.
        bits.appendBits((code1 * 45 + code2) as u32, 11);
        i += 2;
      } else {
        // Encode one alphanumeric letter in six bits.
        bits.appendBits(code1 as u32, 6);
        i+=1;
      }
    }
    Ok(())
  }

  fn append8BitBytes( content:&str,  bits:&BitArray,  encoding:EncodingRef) {
    let bytes = encoding.encode(content, encoding::EncoderTrap::Strict).expect("should encode");
    // let bytes = content.getBytes(encoding);
    for b in bytes {
    // for (byte b : bytes) {
      bits.appendBits(b as u32, 8);
    }
  }

  fn appendKanjiBytes( content:&str,  bits:&BitArray) -> Result<(),Exceptions> {
    let sjis = SHIFT_JIS_CHARSET; //encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
    
    let bytes = sjis.encode(content, encoding::EncoderTrap::Strict).expect("should encode fine");
    // let bytes = content.getBytes(StringUtils::SHIFT_JIS_CHARSET);
    if bytes.len() % 2 != 0 {
      return Err(Exceptions::WriterException("Kanji byte size not even".to_owned()))
    }
    let maxI = bytes.len() - 1; // bytes.length must be even
    let mut i = 0;
    while i < maxI {
    // for (int i = 0; i < maxI; i += 2) {
      let byte1 = bytes[i] & 0xFF;
      let byte2 = bytes[i + 1] & 0xFF;
      let code = (byte1 << 8) | byte2;
      let subtracted:i32 = -1;
      if code >= 0x8140 && code <= 0x9ffc {
        subtracted = code as i32 - 0x8140;
      } else if code >= 0xe040 && code <= 0xebbf {
        subtracted = code as i32 - 0xc140;
      }
      if subtracted == -1 {
        return Err(Exceptions::WriterException("Invalid byte sequence".to_owned()))
      }
      let encoded = ((subtracted >> 8) * 0xc0) + (subtracted & 0xff);
      bits.appendBits(encoded as u32, 13);

      i+=2;
    }
    Ok(())
  }

  fn appendECI( eci:&CharacterSetECI,  bits:&BitArray) {
    bits.appendBits(Mode::ECI.getBits() as u32, 4);
    // This is correct for values up to 127, which is all we need now.
    bits.appendBits(eci.getValueSelf(), 8);
  }

