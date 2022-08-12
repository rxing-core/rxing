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
// package com::google::zxing::datamatrix::decoder;

/**
 * <p>Data Matrix Codes can encode text as bits in one of several modes, and can use multiple modes
 * in one Data Matrix Code. This class decodes the bits back into text.</p>
 *
 * <p>See ISO 16022:2006, 5.2.1 - 5.2.9.2</p>
 *
 * @author bbrown@google.com (Brian Brown)
 * @author Sean Owen
 */

/**
   * See ISO 16022:2006, Annex C Table C.1
   * The C40 Basic Character Set (*'s used for placeholders for the shift values)
   */
 const C40_BASIC_SET_CHARS: vec![Vec<char>; 40] = vec!['*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ]
;

 const C40_SHIFT2_SET_CHARS: vec![Vec<char>; 27] = vec!['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', ]
;

 const TEXT_BASIC_SET_CHARS: vec![Vec<char>; 40] = vec!['*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ]
;

// Shift 2 for Text is the same encoding as C40
 const TEXT_SHIFT2_SET_CHARS: Vec<char> = C40_SHIFT2_SET_CHARS;

 const TEXT_SHIFT3_SET_CHARS: vec![Vec<char>; 32] = vec!['`', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '{', '|', '}', '~', 127 as char, ]
;
struct DecodedBitStreamParser {
}

impl DecodedBitStreamParser {

    enum Mode {

        // Not really a mode
        PAD_ENCODE(), ASCII_ENCODE(), C40_ENCODE(), TEXT_ENCODE(), ANSIX12_ENCODE(), EDIFACT_ENCODE(), BASE256_ENCODE(), ECI_ENCODE()
    }

    fn new() -> DecodedBitStreamParser {
    }

    fn  decode( bytes: &Vec<i8>) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
         let bits: BitSource = BitSource::new(&bytes);
         let result: ECIStringBuilder = ECIStringBuilder::new(100);
         let result_trailer: StringBuilder = StringBuilder::new(0);
         let byte_segments: List<Vec<i8>> = ArrayList<>::new(1);
         let mut mode: Mode = Mode::ASCII_ENCODE;
        // Could look directly at 'bytes', if we're sure of not having to account for multi byte values
         let fnc1_positions: Set<Integer> = HashSet<>::new();
         let symbology_modifier: i32;
         let is_e_c_iencoded: bool = false;
        loop { {
            if mode == Mode::ASCII_ENCODE {
                mode = ::decode_ascii_segment(bits, result, &result_trailer, &fnc1_positions);
            } else {
                match mode {
                      C40_ENCODE => 
                         {
                            ::decode_c40_segment(bits, result, &fnc1_positions);
                            break;
                        }
                      TEXT_ENCODE => 
                         {
                            ::decode_text_segment(bits, result, &fnc1_positions);
                            break;
                        }
                      ANSIX12_ENCODE => 
                         {
                            ::decode_ansi_x12_segment(bits, result);
                            break;
                        }
                      EDIFACT_ENCODE => 
                         {
                            ::decode_edifact_segment(bits, result);
                            break;
                        }
                      BASE256_ENCODE => 
                         {
                            ::decode_base256_segment(bits, result, &byte_segments);
                            break;
                        }
                      ECI_ENCODE => 
                         {
                            ::decode_e_c_i_segment(bits, result);
                            // ECI detection only, atm continue decoding as ASCII
                            is_e_c_iencoded = true;
                            break;
                        }
                    _ => 
                         {
                            throw FormatException::get_format_instance();
                        }
                }
                mode = Mode::ASCII_ENCODE;
            }
        }if !(mode != Mode::PAD_ENCODE && bits.available() > 0) break;}
        if result_trailer.length() > 0 {
            result.append_characters(&result_trailer);
        }
        if is_e_c_iencoded {
            // https://honeywellaidc.force.com/supportppr/s/article/List-of-barcode-symbology-AIM-Identifiers
            if fnc1_positions.contains(0) || fnc1_positions.contains(4) {
                symbology_modifier = 5;
            } else if fnc1_positions.contains(1) || fnc1_positions.contains(5) {
                symbology_modifier = 6;
            } else {
                symbology_modifier = 4;
            }
        } else {
            if fnc1_positions.contains(0) || fnc1_positions.contains(4) {
                symbology_modifier = 2;
            } else if fnc1_positions.contains(1) || fnc1_positions.contains(5) {
                symbology_modifier = 3;
            } else {
                symbology_modifier = 1;
            }
        }
        return Ok(DecoderResult::new(&bytes, &result.to_string(),  if byte_segments.is_empty() { null } else { byte_segments }, null, symbology_modifier));
    }

    /**
   * See ISO 16022:2006, 5.2.3 and Annex C, Table C.2
   */
    fn  decode_ascii_segment( bits: &BitSource,  result: &ECIStringBuilder,  result_trailer: &StringBuilder,  fnc1positions: &Set<Integer>) -> /*  throws FormatException */Result<Mode, Rc<Exception>>   {
         let upper_shift: bool = false;
        loop { {
             let one_byte: i32 = bits.read_bits(8);
            if one_byte == 0 {
                throw FormatException::get_format_instance();
            } else if one_byte <= 128 {
                // ASCII data (ASCII value + 1)
                if upper_shift {
                    one_byte += 128;
                //upperShift = false;
                }
                result.append((one_byte - 1) as char);
                return Ok(Mode::ASCII_ENCODE);
            } else if one_byte == 129 {
                // Pad
                return Ok(Mode::PAD_ENCODE);
            } else if one_byte <= 229 {
                // 2-digit data 00-99 (Numeric Value + 130)
                 let value: i32 = one_byte - 130;
                if value < 10 {
                    // pad with '0' for single digit values
                    result.append('0');
                }
                result.append(value);
            } else {
                match one_byte {
                      // Latch to C40 encodation
                    230 => 
                         {
                            return Ok(Mode::C40_ENCODE);
                        }
                      // Latch to Base 256 encodation
                    231 => 
                         {
                            return Ok(Mode::BASE256_ENCODE);
                        }
                      // FNC1
                    232 => 
                         {
                            fnc1positions.add(&result.length());
                            // translate as ASCII 29
                            result.append(29 as char);
                            break;
                        }
                    // Structured Append
                      233 => 
                         {
                        }
                      // Reader Programming
                    234 => 
                         {
                            //throw ReaderException.getInstance();
                            break;
                        }
                      // Upper Shift (shift to Extended ASCII)
                    235 => 
                         {
                            upper_shift = true;
                            break;
                        }
                      // 05 Macro
                    236 => 
                         {
                            result.append("[)>05");
                            result_trailer.insert(0, "");
                            break;
                        }
                      // 06 Macro
                    237 => 
                         {
                            result.append("[)>06");
                            result_trailer.insert(0, "");
                            break;
                        }
                      // Latch to ANSI X12 encodation
                    238 => 
                         {
                            return Ok(Mode::ANSIX12_ENCODE);
                        }
                      // Latch to Text encodation
                    239 => 
                         {
                            return Ok(Mode::TEXT_ENCODE);
                        }
                      // Latch to EDIFACT encodation
                    240 => 
                         {
                            return Ok(Mode::EDIFACT_ENCODE);
                        }
                      // ECI Character
                    241 => 
                         {
                            return Ok(Mode::ECI_ENCODE);
                        }
                    _ => 
                         {
                            // but work around encoders that end with 254, latch back to ASCII
                            if one_byte != 254 || bits.available() != 0 {
                                throw FormatException::get_format_instance();
                            }
                            break;
                        }
                }
            }
        }if !(bits.available() > 0) break;}
        return Ok(Mode::ASCII_ENCODE);
    }

    /**
   * See ISO 16022:2006, 5.2.5 and Annex C, Table C.1
   */
    fn  decode_c40_segment( bits: &BitSource,  result: &ECIStringBuilder,  fnc1positions: &Set<Integer>)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Three C40 values are encoded in a 16-bit value as
        // (1600 * C1) + (40 * C2) + C3 + 1
        // TODO(bbrown): The Upper Shift with C40 doesn't work in the 4 value scenario all the time
         let upper_shift: bool = false;
         let c_values: [i32; 3] = [0; 3];
         let mut shift: i32 = 0;
        loop { {
            // If there is only one byte left then it will be encoded as ASCII
            if bits.available() == 8 {
                return;
            }
             let first_byte: i32 = bits.read_bits(8);
            if first_byte == 254 {
                // Unlatch codeword
                return;
            }
            ::parse_two_bytes(first_byte, &bits.read_bits(8), &c_values);
             {
                 let mut i: i32 = 0;
                while i < 3 {
                    {
                         let c_value: i32 = c_values[i];
                        match shift {
                              0 => 
                                 {
                                    if c_value < 3 {
                                        shift = c_value + 1;
                                    } else if c_value < C40_BASIC_SET_CHARS.len() {
                                         let c40char: char = C40_BASIC_SET_CHARS[c_value];
                                        if upper_shift {
                                            result.append((c40char + 128) as char);
                                            upper_shift = false;
                                        } else {
                                            result.append(c40char);
                                        }
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              1 => 
                                 {
                                    if upper_shift {
                                        result.append((c_value + 128) as char);
                                        upper_shift = false;
                                    } else {
                                        result.append(c_value as char);
                                    }
                                    shift = 0;
                                    break;
                                }
                              2 => 
                                 {
                                    if c_value < C40_SHIFT2_SET_CHARS.len() {
                                         let c40char: char = C40_SHIFT2_SET_CHARS[c_value];
                                        if upper_shift {
                                            result.append((c40char + 128) as char);
                                            upper_shift = false;
                                        } else {
                                            result.append(c40char);
                                        }
                                    } else {
                                        match c_value {
                                              // FNC1
                                            27 => 
                                                 {
                                                    fnc1positions.add(&result.length());
                                                    // translate as ASCII 29
                                                    result.append(29 as char);
                                                    break;
                                                }
                                              // Upper Shift
                                            30 => 
                                                 {
                                                    upper_shift = true;
                                                    break;
                                                }
                                            _ => 
                                                 {
                                                    throw FormatException::get_format_instance();
                                                }
                                        }
                                    }
                                    shift = 0;
                                    break;
                                }
                              3 => 
                                 {
                                    if upper_shift {
                                        result.append((c_value + 224) as char);
                                        upper_shift = false;
                                    } else {
                                        result.append((c_value + 96) as char);
                                    }
                                    shift = 0;
                                    break;
                                }
                            _ => 
                                 {
                                    throw FormatException::get_format_instance();
                                }
                        }
                    }
                    i += 1;
                 }
             }

        }if !(bits.available() > 0) break;}
    }

    /**
   * See ISO 16022:2006, 5.2.6 and Annex C, Table C.2
   */
    fn  decode_text_segment( bits: &BitSource,  result: &ECIStringBuilder,  fnc1positions: &Set<Integer>)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Three Text values are encoded in a 16-bit value as
        // (1600 * C1) + (40 * C2) + C3 + 1
        // TODO(bbrown): The Upper Shift with Text doesn't work in the 4 value scenario all the time
         let upper_shift: bool = false;
         let c_values: [i32; 3] = [0; 3];
         let mut shift: i32 = 0;
        loop { {
            // If there is only one byte left then it will be encoded as ASCII
            if bits.available() == 8 {
                return;
            }
             let first_byte: i32 = bits.read_bits(8);
            if first_byte == 254 {
                // Unlatch codeword
                return;
            }
            ::parse_two_bytes(first_byte, &bits.read_bits(8), &c_values);
             {
                 let mut i: i32 = 0;
                while i < 3 {
                    {
                         let c_value: i32 = c_values[i];
                        match shift {
                              0 => 
                                 {
                                    if c_value < 3 {
                                        shift = c_value + 1;
                                    } else if c_value < TEXT_BASIC_SET_CHARS.len() {
                                         let text_char: char = TEXT_BASIC_SET_CHARS[c_value];
                                        if upper_shift {
                                            result.append((text_char + 128) as char);
                                            upper_shift = false;
                                        } else {
                                            result.append(text_char);
                                        }
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              1 => 
                                 {
                                    if upper_shift {
                                        result.append((c_value + 128) as char);
                                        upper_shift = false;
                                    } else {
                                        result.append(c_value as char);
                                    }
                                    shift = 0;
                                    break;
                                }
                              2 => 
                                 {
                                    // Shift 2 for Text is the same encoding as C40
                                    if c_value < TEXT_SHIFT2_SET_CHARS.len() {
                                         let text_char: char = TEXT_SHIFT2_SET_CHARS[c_value];
                                        if upper_shift {
                                            result.append((text_char + 128) as char);
                                            upper_shift = false;
                                        } else {
                                            result.append(text_char);
                                        }
                                    } else {
                                        match c_value {
                                              // FNC1
                                            27 => 
                                                 {
                                                    fnc1positions.add(&result.length());
                                                    // translate as ASCII 29
                                                    result.append(29 as char);
                                                    break;
                                                }
                                              // Upper Shift
                                            30 => 
                                                 {
                                                    upper_shift = true;
                                                    break;
                                                }
                                            _ => 
                                                 {
                                                    throw FormatException::get_format_instance();
                                                }
                                        }
                                    }
                                    shift = 0;
                                    break;
                                }
                              3 => 
                                 {
                                    if c_value < TEXT_SHIFT3_SET_CHARS.len() {
                                         let text_char: char = TEXT_SHIFT3_SET_CHARS[c_value];
                                        if upper_shift {
                                            result.append((text_char + 128) as char);
                                            upper_shift = false;
                                        } else {
                                            result.append(text_char);
                                        }
                                        shift = 0;
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                            _ => 
                                 {
                                    throw FormatException::get_format_instance();
                                }
                        }
                    }
                    i += 1;
                 }
             }

        }if !(bits.available() > 0) break;}
    }

    /**
   * See ISO 16022:2006, 5.2.7
   */
    fn  decode_ansi_x12_segment( bits: &BitSource,  result: &ECIStringBuilder)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Three ANSI X12 values are encoded in a 16-bit value as
        // (1600 * C1) + (40 * C2) + C3 + 1
         let c_values: [i32; 3] = [0; 3];
        loop { {
            // If there is only one byte left then it will be encoded as ASCII
            if bits.available() == 8 {
                return;
            }
             let first_byte: i32 = bits.read_bits(8);
            if first_byte == 254 {
                // Unlatch codeword
                return;
            }
            ::parse_two_bytes(first_byte, &bits.read_bits(8), &c_values);
             {
                 let mut i: i32 = 0;
                while i < 3 {
                    {
                         let c_value: i32 = c_values[i];
                        match c_value {
                              // X12 segment terminator <CR>
                            0 => 
                                 {
                                    result.append('\r');
                                    break;
                                }
                              // X12 segment separator *
                            1 => 
                                 {
                                    result.append('*');
                                    break;
                                }
                              // X12 sub-element separator >
                            2 => 
                                 {
                                    result.append('>');
                                    break;
                                }
                              // space
                            3 => 
                                 {
                                    result.append(' ');
                                    break;
                                }
                            _ => 
                                 {
                                    if c_value < 14 {
                                        // 0 - 9
                                        result.append((c_value + 44) as char);
                                    } else if c_value < 40 {
                                        // A - Z
                                        result.append((c_value + 51) as char);
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                        }
                    }
                    i += 1;
                 }
             }

        }if !(bits.available() > 0) break;}
    }

    fn  parse_two_bytes( first_byte: i32,  second_byte: i32,  result: &Vec<i32>)   {
         let full_bit_value: i32 = (first_byte << 8) + second_byte - 1;
         let mut temp: i32 = full_bit_value / 1600;
        result[0] = temp;
        full_bit_value -= temp * 1600;
        temp = full_bit_value / 40;
        result[1] = temp;
        result[2] = full_bit_value - temp * 40;
    }

    /**
   * See ISO 16022:2006, 5.2.8 and Annex C Table C.3
   */
    fn  decode_edifact_segment( bits: &BitSource,  result: &ECIStringBuilder)   {
        loop { {
            // If there is only two or less bytes left then it will be encoded as ASCII
            if bits.available() <= 16 {
                return;
            }
             {
                 let mut i: i32 = 0;
                while i < 4 {
                    {
                         let edifact_value: i32 = bits.read_bits(6);
                        // Check for the unlatch character
                        if edifact_value == 0x1F {
                            // 011111
                            // Read rest of byte, which should be 0, and stop
                             let bits_left: i32 = 8 - bits.get_bit_offset();
                            if bits_left != 8 {
                                bits.read_bits(bits_left);
                            }
                            return;
                        }
                        if (edifact_value & 0x20) == 0 {
                            // no 1 in the leading (6th) bit
                            // Add a leading 01 to the 6 bit binary value
                            edifact_value |= 0x40;
                        }
                        result.append(edifact_value as char);
                    }
                    i += 1;
                 }
             }

        }if !(bits.available() > 0) break;}
    }

    /**
   * See ISO 16022:2006, 5.2.9 and Annex B, B.2
   */
    fn  decode_base256_segment( bits: &BitSource,  result: &ECIStringBuilder,  byte_segments: &Collection<Vec<i8>>)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Figure out how long the Base 256 Segment is.
        // position is 1-indexed
         let codeword_position: i32 = 1 + bits.get_byte_offset();
         let d1: i32 = ::unrandomize255_state(&bits.read_bits(8), codeword_position += 1 !!!check!!! post increment);
         let mut count: i32;
        if d1 == 0 {
            // Read the remainder of the symbol
            count = bits.available() / 8;
        } else if d1 < 250 {
            count = d1;
        } else {
            count = 250 * (d1 - 249) + ::unrandomize255_state(&bits.read_bits(8), codeword_position += 1 !!!check!!! post increment);
        }
        // We're seeing NegativeArraySizeException errors from users.
        if count < 0 {
            throw FormatException::get_format_instance();
        }
         let mut bytes: [i8; count] = [0; count];
         {
             let mut i: i32 = 0;
            while i < count {
                {
                    // http://www.bcgen.com/demo/IDAutomationStreamingDataMatrix.aspx?MODE=3&D=Fred&PFMT=3&PT=F&X=0.3&O=0&LM=0.2
                    if bits.available() < 8 {
                        throw FormatException::get_format_instance();
                    }
                    bytes[i] = ::unrandomize255_state(&bits.read_bits(8), codeword_position += 1 !!!check!!! post increment) as i8;
                }
                i += 1;
             }
         }

        byte_segments.add(&bytes);
        result.append(String::new(&bytes, StandardCharsets::ISO_8859_1));
    }

    /**
   * See ISO 16022:2007, 5.4.1
   */
    fn  decode_e_c_i_segment( bits: &BitSource,  result: &ECIStringBuilder)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        if bits.available() < 8 {
            throw FormatException::get_format_instance();
        }
         let c1: i32 = bits.read_bits(8);
        if c1 <= 127 {
            result.append_e_c_i(c1 - 1);
        }
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
    fn  unrandomize255_state( randomized_base256_codeword: i32,  base256_codeword_position: i32) -> i32  {
         let pseudo_random_number: i32 = ((149 * base256_codeword_position) % 255) + 1;
         let temp_variable: i32 = randomized_base256_codeword - pseudo_random_number;
        return  if temp_variable >= 0 { temp_variable } else { temp_variable + 256 };
    }
}

