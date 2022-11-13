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

use crate::{Exceptions, common::{BitSource, ECIStringBuilder, DecoderRXingResult}};


/**
 * <p>Data Matrix Codes can encode text as bits in one of several modes, and can use multiple modes
 * in one Data Matrix Code. This class decodes the bits back into text.</p>
 *
 * <p>See ISO 16022:2006, 5.2.1 - 5.2.9.2</p>
 *
 * @author bbrown@google.com (Brian Brown)
 * @author Sean Owen
 */

 #[derive(Debug,PartialEq, Eq,Clone, Copy)]
   enum Mode {
    PAD_ENCODE, // Not really a mode
    ASCII_ENCODE,
    C40_ENCODE,
    TEXT_ENCODE,
    ANSIX12_ENCODE,
    EDIFACT_ENCODE,
    BASE256_ENCODE,
    ECI_ENCODE
  }

  /**
   * See ISO 16022:2006, Annex C Table C.1
   * The C40 Basic Character Set (*'s used for placeholders for the shift values)
   */
  const C40_BASIC_SET_CHARS : [char;40]= [
    '*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
  ];

   const C40_SHIFT2_SET_CHARS :[char;27] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*',  '+', ',', '-', '.',
    '/', ':', ';', '<', '=', '>', '?',  '@', '[', '\\', ']', '^', '_'
   ];

  /**
   * See ISO 16022:2006, Annex C Table C.2
   * The Text Basic Character Set (*'s used for placeholders for the shift values)
   */
  const TEXT_BASIC_SET_CHARS : [char;40]= 
    ['*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
  ];

  // Shift 2 for Text is the same encoding as C40
  const TEXT_SHIFT2_SET_CHARS : [char;27]= C40_SHIFT2_SET_CHARS;

  const TEXT_SHIFT3_SET_CHARS : [char;32]= [
    '`', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O',  'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '{', '|', '}', '~',  127 as char
  ];

  pub fn decode( bytes: &[u8]) -> Result<DecoderRXingResult,Exceptions> {
    let bits =  BitSource::new(bytes);
    let result =  ECIStringBuilder::with_capacity(100);
    let resultTrailer = String::new();
    let byteSegments = Vec::new();//new ArrayList<>(1);
    let mode = Mode::ASCII_ENCODE;
    // Could look directly at 'bytes', if we're sure of not having to account for multi byte values
    let fnc1Positions = Vec::new();
    let symbologyModifier;
    let isECIencoded = false;
    loop {
      if mode == Mode::ASCII_ENCODE {
        mode = decodeAsciiSegment(bits, result, resultTrailer, fnc1Positions);
      } else {
        match mode {
          Mode::C40_ENCODE=>
            decodeC40Segment(bits, result, fnc1Positions),
            Mode::TEXT_ENCODE=>
            decodeTextSegment(bits, result, fnc1Positions),
            Mode::ANSIX12_ENCODE=>
            decodeAnsiX12Segment(bits, result),
            Mode::EDIFACT_ENCODE=>
            decodeEdifactSegment(bits, result),
            Mode::BASE256_ENCODE=>
            decodeBase256Segment(bits, result, byteSegments),
            Mode::ECI_ENCODE=>{
            decodeECISegment(bits, result);
            isECIencoded = true; // ECI detection only, atm continue decoding as ASCII
            },
          _=>
            return Err(Exceptions::FormatException("".to_owned())),
        }
        mode = Mode::ASCII_ENCODE;
      }
      if ! (mode != Mode::PAD_ENCODE && bits.available() > 0) { break }
    } //while (mode != Mode.PAD_ENCODE && bits.available() > 0);
    if (resultTrailer.length() > 0) {
      result.appendCharacters(resultTrailer);
    }
    if (isECIencoded) {
      // Examples for this numbers can be found in this documentation of a hardware barcode scanner:
      // https://honeywellaidc.force.com/supportppr/s/article/List-of-barcode-symbology-AIM-Identifiers
      if (fnc1Positions.contains(0) || fnc1Positions.contains(4)) {
        symbologyModifier = 5;
      } else if (fnc1Positions.contains(1) || fnc1Positions.contains(5)) {
        symbologyModifier = 6;
      } else {
        symbologyModifier = 4;
      }
    } else {
      if (fnc1Positions.contains(0) || fnc1Positions.contains(4)) {
        symbologyModifier = 2;
      } else if (fnc1Positions.contains(1) || fnc1Positions.contains(5)) {
        symbologyModifier = 3;
      } else {
        symbologyModifier = 1;
      }
    }

    return new DecoderRXingResult(bytes,
                             result.toString(),
                             byteSegments.isEmpty() ? null : byteSegments,
                             null,
                             symbologyModifier);
  }

  /**
   * See ISO 16022:2006, 5.2.3 and Annex C, Table C.2
   */
  fn decodeAsciiSegment( bits:&BitSource,
                                          result:&ECIStringBuilder,
                                          resultTrailer:&String,
                                          fnc1positions:&[usize]) -> Result<Mode,Exceptions> {
    boolean upperShift = false;
    do {
      int oneByte = bits.readBits(8);
      if (oneByte == 0) {
        throw FormatException.getFormatInstance();
      } else if (oneByte <= 128) {  // ASCII data (ASCII value + 1)
        if (upperShift) {
          oneByte += 128;
          //upperShift = false;
        }
        result.append((char) (oneByte - 1));
        return Mode.ASCII_ENCODE;
      } else if (oneByte == 129) {  // Pad
        return Mode.PAD_ENCODE;
      } else if (oneByte <= 229) {  // 2-digit data 00-99 (Numeric Value + 130)
        int value = oneByte - 130;
        if (value < 10) { // pad with '0' for single digit values
          result.append('0');
        }
        result.append(value);
      } else {
        switch (oneByte) {
          case 230: // Latch to C40 encodation
            return Mode.C40_ENCODE;
          case 231: // Latch to Base 256 encodation
            return Mode.BASE256_ENCODE;
          case 232: // FNC1
            fnc1positions.add(result.length());
            result.append((char) 29); // translate as ASCII 29
            break;
          case 233: // Structured Append
          case 234: // Reader Programming
            // Ignore these symbols for now
            //throw ReaderException.getInstance();
            break;
          case 235: // Upper Shift (shift to Extended ASCII)
            upperShift = true;
            break;
          case 236: // 05 Macro
            result.append("[)>\u001E05\u001D");
            resultTrailer.insert(0, "\u001E\u0004");
            break;
          case 237: // 06 Macro
            result.append("[)>\u001E06\u001D");
            resultTrailer.insert(0, "\u001E\u0004");
            break;
          case 238: // Latch to ANSI X12 encodation
            return Mode.ANSIX12_ENCODE;
          case 239: // Latch to Text encodation
            return Mode.TEXT_ENCODE;
          case 240: // Latch to EDIFACT encodation
            return Mode.EDIFACT_ENCODE;
          case 241: // ECI Character
            return Mode.ECI_ENCODE;
          default:
            // Not to be used in ASCII encodation
            // but work around encoders that end with 254, latch back to ASCII
            if (oneByte != 254 || bits.available() != 0) {
              throw FormatException.getFormatInstance();
            }
            break;
        }
      }
    } while (bits.available() > 0);
    return Mode.ASCII_ENCODE;
  }

  /**
   * See ISO 16022:2006, 5.2.5 and Annex C, Table C.1
   */
  fn decodeC40Segment( bits:&BitSource,  result:&ECIStringBuilder, fnc1positions:&[usize])
      -> Result<(),Exceptions> {
    // Three C40 values are encoded in a 16-bit value as
    // (1600 * C1) + (40 * C2) + C3 + 1
    // TODO(bbrown): The Upper Shift with C40 doesn't work in the 4 value scenario all the time
    boolean upperShift = false;

    int[] cValues = new int[3];
    int shift = 0;

    do {
      // If there is only one byte left then it will be encoded as ASCII
      if (bits.available() == 8) {
        return;
      }
      int firstByte = bits.readBits(8);
      if (firstByte == 254) {  // Unlatch codeword
        return;
      }

      parseTwoBytes(firstByte, bits.readBits(8), cValues);

      for (int i = 0; i < 3; i++) {
        int cValue = cValues[i];
        switch (shift) {
          case 0:
            if (cValue < 3) {
              shift = cValue + 1;
            } else if (cValue < C40_BASIC_SET_CHARS.length) {
              char c40char = C40_BASIC_SET_CHARS[cValue];
              if (upperShift) {
                result.append((char) (c40char + 128));
                upperShift = false;
              } else {
                result.append(c40char);
              }
            } else {
              throw FormatException.getFormatInstance();
            }
            break;
          case 1:
            if (upperShift) {
              result.append((char) (cValue + 128));
              upperShift = false;
            } else {
              result.append((char) cValue);
            }
            shift = 0;
            break;
          case 2:
            if (cValue < C40_SHIFT2_SET_CHARS.length) {
              char c40char = C40_SHIFT2_SET_CHARS[cValue];
              if (upperShift) {
                result.append((char) (c40char + 128));
                upperShift = false;
              } else {
                result.append(c40char);
              }
            } else {
              switch (cValue) {
                case 27: // FNC1
                  fnc1positions.add(result.length());
                  result.append((char) 29); // translate as ASCII 29
                  break;
                case 30: // Upper Shift
                  upperShift = true;
                  break;
                default:
                  throw FormatException.getFormatInstance();
              }
            }
            shift = 0;
            break;
          case 3:
            if (upperShift) {
              result.append((char) (cValue + 224));
              upperShift = false;
            } else {
              result.append((char) (cValue + 96));
            }
            shift = 0;
            break;
          default:
            throw FormatException.getFormatInstance();
        }
      }
    } while (bits.available() > 0);
  }

  /**
   * See ISO 16022:2006, 5.2.6 and Annex C, Table C.2
   */
  fn decodeTextSegment( bits:&BitSource,  result:&ECIStringBuilder,  fnc1positions:&[usize])
      -> Result<(),Exceptions> {
    // Three Text values are encoded in a 16-bit value as
    // (1600 * C1) + (40 * C2) + C3 + 1
    // TODO(bbrown): The Upper Shift with Text doesn't work in the 4 value scenario all the time
    let upperShift = false;

    let cValues = [0;3];//new int[3];
    let shift = 0;
    loop {
      // If there is only one byte left then it will be encoded as ASCII
      if bits.available() == 8 {
        return Ok(())
      }
      let firstByte = bits.readBits(8)?;
      if firstByte == 254 {  // Unlatch codeword
        return Ok(())
      }

      parseTwoBytes(firstByte, bits.readBits(8)?, &cValues);

      for cValue in cValues {
      // for (int i = 0; i < 3; i++) {
        // int cValue = cValues[i];
        match shift {
           0=>
            if cValue < 3 {
              shift = cValue + 1;
            } else if cValue < TEXT_BASIC_SET_CHARS.len() {
              let textChar = TEXT_BASIC_SET_CHARS[cValue];
              if upperShift {
                result.append_char( (textChar + 128));
                upperShift = false;
              } else {
                result.append(textChar);
              }
            } else {
              return Err(Exceptions::FormatException("".to_owned()));
            },
           1=>
            {if upperShift {
              result.append_char( (cValue + 128));
              upperShift = false;
            } else {
              result.append_char( cValue);
            }
            shift = 0;},
            
           2=>
            {// Shift 2 for Text is the same encoding as C40
            if cValue < TEXT_SHIFT2_SET_CHARS.len() {
              let textChar = TEXT_SHIFT2_SET_CHARS[cValue];
              if upperShift {
                result.append_char( (textChar + 128));
                upperShift = false;
              } else {
                result.append_char(textChar);
              }
            } else {
              match cValue {
                27=>{ // FNC1
                  fnc1positions.push(result.length());
                  result.append_char( 29); // translate as ASCII 29
                  },
                 30=> // Upper Shift
                  upperShift = true,
                  
                _=>
                return Err(Exceptions::FormatException("".to_owned())),
              }
            }
            shift = 0;},
           3=>
            if cValue < TEXT_SHIFT3_SET_CHARS.len() {
              let textChar = TEXT_SHIFT3_SET_CHARS[cValue];
              if upperShift {
                result.append_char( (textChar + 128));
                upperShift = false;
              } else {
                result.append_char(textChar);
              }
              shift = 0;
            } else {
              return Err(Exceptions::FormatException("".to_owned()));
            },
            
          _=>
            return Err(Exceptions::FormatException("".to_owned())),
        }
      }
      if !(bits.available() > 0){break}
    } //while (bits.available() > 0);

    Ok(())
  }

  /**
   * See ISO 16022:2006, 5.2.7
   */
  fn decodeAnsiX12Segment( bits:&BitSource,
                                            result:&ECIStringBuilder) -> Result<(),Exceptions> {
    // Three ANSI X12 values are encoded in a 16-bit value as
    // (1600 * C1) + (40 * C2) + C3 + 1

    let cValues = [0;3];//new int[3];
    loop {
      // If there is only one byte left then it will be encoded as ASCII
      if bits.available() == 8 {
        return Ok(())
      }
      let firstByte = bits.readBits(8)?;
      if firstByte == 254 {  // Unlatch codeword
        return Ok(())
      }

      parseTwoBytes(firstByte, bits.readBits(8)?, &cValues);

      for cValue in cValues {
      // for (int i = 0; i < 3; i++) {
      //   int cValue = cValues[i];
        match cValue {
           0=> // X12 segment terminator <CR>
            result.append_char('\r'),
            
           1=> // X12 segment separator *
            result.append_char('*'),
            
           2=> // X12 sub-element separator >
            result.append_char('>'),
            
           3=> // space
            result.append_char(' '),
            
          _=>
            if cValue < 14 {  // 0 - 9
              result.append_char( char::from_u32(cValue + 44).unwrap());
            } else if cValue < 40 {  // A - Z
              result.append_char( char::from_u32(cValue + 51).unwrap());
            } else {
              return Err(Exceptions::FormatException("".to_owned()))
            },
        }
      }
      if ! (bits.available() > 0) { break }
    } //while (bits.available() > 0);

    Ok(())
  }

  fn parseTwoBytes( firstByte:u32,  secondByte:u32, result:&[u32]) {
    let fullBitValue = (firstByte << 8) + secondByte - 1;
    let temp = fullBitValue / 1600;
    result[0] = temp;
    fullBitValue -= temp * 1600;
    temp = fullBitValue / 40;
    result[1] = temp;
    result[2] = fullBitValue - temp * 40;
  }

  /**
   * See ISO 16022:2006, 5.2.8 and Annex C Table C.3
   */
  fn decodeEdifactSegment( bits:&BitSource,  result:&ECIStringBuilder) -> Result<(),Exceptions>{
    loop {
      // If there is only two or less bytes left then it will be encoded as ASCII
      if bits.available() <= 16 {
        return Ok(());
      }

      for i in 0..4 {
      // for (int i = 0; i < 4; i++) {
        let edifactValue = bits.readBits(6)?;

        // Check for the unlatch character
        if edifactValue == 0x1F {  // 011111
          // Read rest of byte, which should be 0, and stop
          let bitsLeft = 8 - bits.getBitOffset();
          if bitsLeft != 8 {
            bits.readBits(bitsLeft);
          }
          return Ok(());
        }

        if (edifactValue & 0x20) == 0 {  // no 1 in the leading (6th) bit
          edifactValue |= 0x40;  // Add a leading 01 to the 6 bit binary value
        }
        result.append_char( char::from_u32(edifactValue).unwrap());
      }

      if ! (bits.available() > 0) { break }
    } 

    Ok(())
  }

  /**
   * See ISO 16022:2006, 5.2.9 and Annex B, B.2
   */
  fn decodeBase256Segment( bits:&BitSource,
                                            result:&ECIStringBuilder,
                                            byteSegments:&Vec<Vec<u8>>)
      -> Result<(),Exceptions> {
    // Figure out how long the Base 256 Segment is.
    let codewordPosition = 1 + bits.getByteOffset(); // position is 1-indexed
    let d1 = unrandomize255State(bits.readBits(8)?, codewordPosition);
    codewordPosition +=1;
    let count;
    if d1 == 0 {  // Read the remainder of the symbol
      count = bits.available() as u32 / 8;
    } else if d1 < 250 {
      count = d1;
    } else {
      count = 250 * (d1 - 249) + unrandomize255State(bits.readBits(8)?, codewordPosition);
      codewordPosition +=1;
    }

    // We're seeing NegativeArraySizeException errors from users.
    if (count < 0) {
      return Err(Exceptions::FormatException("".to_owned()))
    }

    let bytes = vec![0u8;count as usize];
    for i in 0..count as usize {
    // for (int i = 0; i < count; i++) {
      // Have seen this particular error in the wild, such as at
      // http://www.bcgen.com/demo/IDAutomationStreamingDataMatrix.aspx?MODE=3&D=Fred&PFMT=3&PT=F&X=0.3&O=0&LM=0.2
      if bits.available() < 8 {
        return Err(Exceptions::FormatException("".to_owned()))
      }
      bytes[i] =  unrandomize255State(bits.readBits(8)?, codewordPosition) as u8;
      codewordPosition+=1;
    }
    byteSegments.push(bytes);
    result.append_string(&encoding::all::ISO_8859_1.decode(&bytes, encoding::DecoderTrap::Strict).expect("decode"));
  
    Ok(())
  }

  /**
   * See ISO 16022:2007, 5.4.1
   */
  fn decodeECISegment( bits:&BitSource,
                                            result:&ECIStringBuilder)
      -> Result<(),Exceptions> {
    if bits.available() < 8 {
      return Err(Exceptions::FormatException("".to_owned()))
    }
    let c1 = bits.readBits(8)?;
    if c1 <= 127 {
      result.appendECI(c1 - 1)?;
    }

    Ok(())
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
   * See ISO 16022:2006, Annex B, B.2
   */
  fn unrandomize255State( randomizedBase256Codeword:u32,
                                           base256CodewordPosition:usize) -> u32{
    let pseudoRandomNumber = ((149 * base256CodewordPosition as u32) % 255) + 1;
    let tempVariable = randomizedBase256Codeword - pseudoRandomNumber;
     
    if tempVariable >= 0  {tempVariable} else  {tempVariable + 256}
  }

