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

use crate::{Exceptions, common::{BitSource, StringUtils, DecoderRXingResult, CharacterSetECI}, DecodingHintDictionary};

use super::{VersionRef, Mode, ErrorCorrectionLevel};


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
  const ALPHANUMERIC_CHARS : &str=
      "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
  const GB2312_SUBSET :u8 = 1;


  pub fn decode(bytes : &[u8],
                               version:VersionRef,
                               ecLevel:ErrorCorrectionLevel,
                              hints:DecodingHintDictionary) -> Result<DecoderRXingResult,Exceptions> {
    let bits =  BitSource::new(bytes);
    let result = String::with_capacity(50);
    let byteSegments = vec![vec![0u8;0];0];
    let symbolSequence = -1;
    let parityData = -1;
    let symbologyModifier;

    // try {
      let currentCharacterSetECI = None;
      let fc1InEffect = false;
      let hasFNC1first = false;
      let hasFNC1second = false;
      let mode;
      loop {
        // While still another segment to read...
        if (bits.available() < 4) {
          // OK, assume we're done. Really, a TERMINATOR mode should have been recorded here
          mode = Mode::TERMINATOR;
        } else {
          mode = Mode::forBits(bits.readBits(4)); // mode is encoded by 4 bits
        }
        match mode {
          Mode::TERMINATOR => {},
            Mode::FNC1_FIRST_POSITION=> {
            hasFNC1first = true; // symbology detection
            // We do little with FNC1 except alter the parsed result a bit according to the spec
            fc1InEffect = true;},
            Mode::FNC1_SECOND_POSITION=> {
            hasFNC1second = true; // symbology detection
            // We do little with FNC1 except alter the parsed result a bit according to the spec
            fc1InEffect = true;},
            Mode::STRUCTURED_APPEND=> {
            if (bits.available() < 16) {
              return Err(Exceptions::FormatException(format!("Mode::Structured append expected bits.available() < 16, found bits of {}", bits.available())))
            }
            // sequence number and parity is added later to the result metadata
            // Read next 8 bits (symbol sequence #) and 8 bits (parity data), then continue
            symbolSequence = bits.readBits(8);
            parityData = bits.readBits(8);},
            Mode::ECI=> {
            // Count doesn't apply to ECI
            let value = parseECIValue(&bits)?;
            currentCharacterSetECI = CharacterSetECI::getCharacterSetECIByValue(value);
            if (currentCharacterSetECI.is_none()) {
              return Err(Exceptions::FormatException(format!("Value of {} not valid", value)))

            }},
            Mode::HANZI=> {
            // First handle Hanzi mode which does not start with character count
            // Chinese mode contains a sub set indicator right after mode indicator
            let subset = bits.readBits(4);
            let countHanzi = bits.readBits(mode.getCharacterCountBits(version));
            if (subset == GB2312_SUBSET) {
              decodeHanziSegment(&bits, &result, countHanzi)?;
            }},
          _=>{
            // "Normal" QR code modes:
            // How many characters will follow, encoded in this mode?
            let count = bits.readBits(mode.getCharacterCountBits(version));
            match mode {
              Mode::NUMERIC=>
                decodeNumericSegment(&bits, &result, count)?,
                Mode::ALPHANUMERIC=>decodeAlphanumericSegment(&bits, &result, count, fc1InEffect)?,
                Mode::BYTE=>decodeByteSegment(&bits, &result, count, currentCharacterSetECI, &byteSegments, hints)?,
                Mode::KANJI=>decodeKanjiSegment(&bits, &result, count)?,
              _=>
                return Err(Exceptions::FormatException("".to_owned()))
            }},
        }

        if mode != Mode::TERMINATOR {break}
      } 

      if (currentCharacterSetECI != null) {
        if (hasFNC1first) {
          symbologyModifier = 4;
        } else if (hasFNC1second) {
          symbologyModifier = 6;
        } else {
          symbologyModifier = 2;
        }
      } else {
        if (hasFNC1first) {
          symbologyModifier = 3;
        } else if (hasFNC1second) {
          symbologyModifier = 5;
        } else {
          symbologyModifier = 1;
        }
      }

    // } catch (IllegalArgumentException iae) {
    //   // from readBits() calls
    //   throw FormatException.getFormatInstance();
    // }

    return  DecoderRXingResult::new(bytes,
                             result.toString(),
                             if byteSegments.isEmpty()  {None} else  {byteSegments},
                             if ecLevel.is_none()  {None} else {ecLevel.toString()},
                             symbolSequence,
                             parityData,
                             symbologyModifier);
  }

  /**
   * See specification GBT 18284-2000
   */
  fn decodeHanziSegment( bits:&BitSource,
                                          result:&String,
                                          count:u32) -> Result<(),Exceptions> {
    // Don't crash trying to read more bits than we have available.
    if (count * 13 > bits.available()) {
      return Err(Exceptions::FormatException("".to_owned()))
    }

    // Each character will require 2 bytes. Read the characters as 2-byte pairs
    // and decode as GB2312 afterwards
    let buffer = vec![0u8;2 * count];
    let offset = 0;
    while (count > 0) {
      // Each 13 bits encodes a 2-byte character
      let twoBytes = bits.readBits(13)?;
      let assembledTwoBytes = ((twoBytes / 0x060) << 8) | (twoBytes % 0x060);
      if (assembledTwoBytes < 0x00A00) {
        // In the 0xA1A1 to 0xAAFE range
        assembledTwoBytes += 0x0A1A1;
      } else {
        // In the 0xB0A1 to 0xFAFE range
        assembledTwoBytes += 0x0A6A1;
      }
      buffer[offset] =  ((assembledTwoBytes >> 8) & 0xFF);
      buffer[offset + 1] =  (assembledTwoBytes & 0xFF);
      offset += 2;
      count-=1;
    }

    let gb_encoder = encoding::label::encoding_from_whatwg_label("GB2312").unwrap();
    let encode_string = gb_encoder.decode(&buffer, encoding::DecoderTrap::Strict).unwrap();
    result.append(encode_string);
    Ok(())
  }

  fn decodeKanjiSegment( bits:&BitSource,
                                          result:&String,
                                         count:u32) -> Result<(),Exceptions> {
    // Don't crash trying to read more bits than we have available.
    if (count * 13 > bits.available()) {
      return Err(Exceptions::FormatException("".to_owned()))
    }

    // Each character will require 2 bytes. Read the characters as 2-byte pairs
    // and decode as Shift_JIS afterwards
    let buffer = vec![0u8;2 * count];
    let offset = 0;
    while (count > 0) {
      // Each 13 bits encodes a 2-byte character
      let twoBytes = bits.readBits(13);
      let assembledTwoBytes = ((twoBytes / 0x0C0) << 8) | (twoBytes % 0x0C0);
      if (assembledTwoBytes < 0x01F00) {
        // In the 0x8140 to 0x9FFC range
        assembledTwoBytes += 0x08140;
      } else {
        // In the 0xE040 to 0xEBBF range
        assembledTwoBytes += 0x0C140;
      }
      buffer[offset] = (assembledTwoBytes >> 8);
      buffer[offset + 1] = assembledTwoBytes;
      offset += 2;
      count-=1;
    }
    
    let sjs_encoder = encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
    let encode_string = sjs_encoder.decode(&buffer, encoding::DecoderTrap::Strict).unwrap();
    result.append(encode_string);

    Ok(())
  }

  fn decodeByteSegment( bits:&BitSource,
                                         result:&String,
                                         count:u32,
                                         currentCharacterSetECI:Option<&CharacterSetECI>,
                                         byteSegments:&Vec<Vec<u8>>,
                                         hints:DecodingHintDictionary) -> Result<(),Exceptions> {
    // Don't crash trying to read more bits than we have available.
    if (8 * count > bits.available()) {
      return Err(Exceptions::FormatException("".to_owned()))
    }

    let readBytes = vec![0u8;count];
    for i in 0..count {
    // for (int i = 0; i < count; i++) {
      readBytes[i] = bits.readBits(8);
    }
    let encoding;
    if (currentCharacterSetECI.is_none()) {
      // The spec isn't clear on this mode; see
      // section 6.4.5: t does not say which encoding to assuming
      // upon decoding. I have seen ISO-8859-1 used as well as
      // Shift_JIS -- without anything like an ECI designator to
      // give a hint.
      encoding = StringUtils::guessCharset(&readBytes, hints);
    } else {
      encoding = currentCharacterSetECI.getCharset();
    }
    let encode_string = encoding.decode(&readBytes, encoding::DecoderTrap::Strict).unwrap();
    result.append(encode_string);
    byteSegments.add(readBytes);

    Ok(())
  }

  fn toAlphaNumericChar( value:u32) -> Result<char,Exceptions> {
    if (value >= ALPHANUMERIC_CHARS.len()) {
      return Err(Exceptions::FormatException("".to_owned()))
    }
    Ok( ALPHANUMERIC_CHARS[value])
  }

  fn decodeAlphanumericSegment( bits:&BitSource,
                                                 result:&String,
                                                 count:u32,
                                                 fc1InEffect:bool) -> Result<(),Exceptions> {
    // Read two characters at a time
    let start = result.len();
    while count > 1 {
      if bits.available() < 11 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      let nextTwoCharsBits = bits.readBits(11);
      result.append(toAlphaNumericChar(nextTwoCharsBits / 45));
      result.append(toAlphaNumericChar(nextTwoCharsBits % 45));
      count -= 2;
    }
    if (count == 1) {
      // special case: one character left
      if (bits.available() < 6) {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      result.append(toAlphaNumericChar(bits.readBits(6)));
    }
    // See section 6.4.8.1, 6.4.8.2
    if fc1InEffect {
      // We need to massage the result a bit if in an FNC1 mode:
      for i in start..result.len() {
      // for (int i = start; i < result.length(); i++) {
        if (result.charAt(i) == '%') {
          if (i < result.length() - 1 && result.charAt(i + 1) == '%') {
            // %% is rendered as %
            result.deleteCharAt(i + 1);
          } else {
            // In alpha mode, % should be converted to FNC1 separator 0x1D
            result.setCharAt(i,  0x1D);
          }
        }
      }
    }

    Ok(())
  }

  fn decodeNumericSegment( bits:&BitSource,
                                            result:&String,
                                            count:u32) -> Result<(),Exceptions> {
    // Read three digits at a time
    while count >= 3 {
      // Each 10 bits encodes three digits
      if bits.available() < 10 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      let threeDigitsBits = bits.readBits(10)?;
      if threeDigitsBits >= 1000 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      result.append(toAlphaNumericChar(threeDigitsBits / 100));
      result.append(toAlphaNumericChar((threeDigitsBits / 10) % 10));
      result.append(toAlphaNumericChar(threeDigitsBits % 10));
      count -= 3;
    }
    if count == 2 {
      // Two digits left over to read, encoded in 7 bits
      if bits.available() < 7 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      let twoDigitsBits = bits.readBits(7)?;
      if twoDigitsBits >= 100 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      result.append(toAlphaNumericChar(twoDigitsBits / 10));
      result.append(toAlphaNumericChar(twoDigitsBits % 10));
    } else if count == 1 {
      // One digit left over to read
      if bits.available() < 4 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      let digitBits = bits.readBits(4)?;
      if digitBits >= 10 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      result.append(toAlphaNumericChar(digitBits));
    }

    Ok(())
  }

  fn parseECIValue( bits:&BitSource) -> Result<u32,Exceptions> {
    let firstByte = bits.readBits(8)?;
    if (firstByte & 0x80) == 0 {
      // just one byte
      return Ok(firstByte & 0x7F);
    }
    if (firstByte & 0xC0) == 0x80 {
      // two bytes
      let secondByte = bits.readBits(8);
      return Ok(((firstByte & 0x3F) << 8) | secondByte);
    }
    if (firstByte & 0xE0) == 0xC0 {
      // three bytes
      let secondThirdBytes = bits.readBits(16);
      return Ok(((firstByte & 0x1F) << 16) | secondThirdBytes);
    }
    
    Err(Exceptions::FormatException("".to_owned()))
  }

