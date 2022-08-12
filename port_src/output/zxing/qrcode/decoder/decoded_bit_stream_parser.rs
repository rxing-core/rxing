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
// package com::google::zxing::qrcode::decoder;

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
 const ALPHANUMERIC_CHARS: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:".to_char_array();

 const GB2312_SUBSET: i32 = 1;
struct DecodedBitStreamParser {
}

impl DecodedBitStreamParser {

    fn new() -> DecodedBitStreamParser {
    }

    fn  decode( bytes: &Vec<i8>,  version: &Version,  ec_level: &ErrorCorrectionLevel,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
         let bits: BitSource = BitSource::new(&bytes);
         let result: StringBuilder = StringBuilder::new(50);
         let byte_segments: List<Vec<i8>> = ArrayList<>::new(1);
         let symbol_sequence: i32 = -1;
         let parity_data: i32 = -1;
         let symbology_modifier: i32;
        let tryResult1 = 0;
        'try1: loop {
        {
             let current_character_set_e_c_i: CharacterSetECI = null;
             let fc1_in_effect: bool = false;
             let has_f_n_c1first: bool = false;
             let has_f_n_c1second: bool = false;
             let mut mode: Mode;
            loop { {
                // While still another segment to read...
                if bits.available() < 4 {
                    // OK, assume we're done. Really, a TERMINATOR mode should have been recorded here
                    mode = Mode::TERMINATOR;
                } else {
                    // mode is encoded by 4 bits
                    mode = Mode::for_bits(&bits.read_bits(4));
                }
                match mode {
                      TERMINATOR => 
                         {
                            break;
                        }
                      FNC1_FIRST_POSITION => 
                         {
                            // symbology detection
                            has_f_n_c1first = true;
                            // We do little with FNC1 except alter the parsed result a bit according to the spec
                            fc1_in_effect = true;
                            break;
                        }
                      FNC1_SECOND_POSITION => 
                         {
                            // symbology detection
                            has_f_n_c1second = true;
                            // We do little with FNC1 except alter the parsed result a bit according to the spec
                            fc1_in_effect = true;
                            break;
                        }
                      STRUCTURED_APPEND => 
                         {
                            if bits.available() < 16 {
                                throw FormatException::get_format_instance();
                            }
                            // sequence number and parity is added later to the result metadata
                            // Read next 8 bits (symbol sequence #) and 8 bits (parity data), then continue
                            symbol_sequence = bits.read_bits(8);
                            parity_data = bits.read_bits(8);
                            break;
                        }
                      ECI => 
                         {
                            // Count doesn't apply to ECI
                             let value: i32 = ::parse_e_c_i_value(bits);
                            current_character_set_e_c_i = CharacterSetECI::get_character_set_e_c_i_by_value(value);
                            if current_character_set_e_c_i == null {
                                throw FormatException::get_format_instance();
                            }
                            break;
                        }
                      HANZI => 
                         {
                            // First handle Hanzi mode which does not start with character count
                            // Chinese mode contains a sub set indicator right after mode indicator
                             let subset: i32 = bits.read_bits(4);
                             let count_hanzi: i32 = bits.read_bits(&mode.get_character_count_bits(version));
                            if subset == GB2312_SUBSET {
                                ::decode_hanzi_segment(bits, &result, count_hanzi);
                            }
                            break;
                        }
                    _ => 
                         {
                            // "Normal" QR code modes:
                            // How many characters will follow, encoded in this mode?
                             let count: i32 = bits.read_bits(&mode.get_character_count_bits(version));
                            match mode {
                                  NUMERIC => 
                                     {
                                        ::decode_numeric_segment(bits, &result, count);
                                        break;
                                    }
                                  ALPHANUMERIC => 
                                     {
                                        ::decode_alphanumeric_segment(bits, &result, count, fc1_in_effect);
                                        break;
                                    }
                                  BYTE => 
                                     {
                                        ::decode_byte_segment(bits, &result, count, current_character_set_e_c_i, &byte_segments, &hints);
                                        break;
                                    }
                                  KANJI => 
                                     {
                                        ::decode_kanji_segment(bits, &result, count);
                                        break;
                                    }
                                _ => 
                                     {
                                        throw FormatException::get_format_instance();
                                    }
                            }
                            break;
                        }
                }
            }if !(mode != Mode::TERMINATOR) break;}
            if current_character_set_e_c_i != null {
                if has_f_n_c1first {
                    symbology_modifier = 4;
                } else if has_f_n_c1second {
                    symbology_modifier = 6;
                } else {
                    symbology_modifier = 2;
                }
            } else {
                if has_f_n_c1first {
                    symbology_modifier = 3;
                } else if has_f_n_c1second {
                    symbology_modifier = 5;
                } else {
                    symbology_modifier = 1;
                }
            }
        }
        break 'try1
        }
        match tryResult1 {
             catch ( iae: &IllegalArgumentException) {
                throw FormatException::get_format_instance();
            }  0 => break
        }

        return Ok(DecoderResult::new(&bytes, &result.to_string(),  if byte_segments.is_empty() { null } else { byte_segments },  if ec_level == null { null } else { ec_level.to_string() }, symbol_sequence, parity_data, symbology_modifier));
    }

    /**
   * See specification GBT 18284-2000
   */
    fn  decode_hanzi_segment( bits: &BitSource,  result: &StringBuilder,  count: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Don't crash trying to read more bits than we have available.
        if count * 13 > bits.available() {
            throw FormatException::get_format_instance();
        }
        // Each character will require 2 bytes. Read the characters as 2-byte pairs
        // and decode as GB2312 afterwards
         let mut buffer: [i8; 2 * count] = [0; 2 * count];
         let mut offset: i32 = 0;
        while count > 0 {
            // Each 13 bits encodes a 2-byte character
             let two_bytes: i32 = bits.read_bits(13);
             let assembled_two_bytes: i32 = ((two_bytes / 0x060) << 8) | (two_bytes % 0x060);
            if assembled_two_bytes < 0x00A00 {
                // In the 0xA1A1 to 0xAAFE range
                assembled_two_bytes += 0x0A1A1;
            } else {
                // In the 0xB0A1 to 0xFAFE range
                assembled_two_bytes += 0x0A6A1;
            }
            buffer[offset] = ((assembled_two_bytes >> 8) & 0xFF) as i8;
            buffer[offset + 1] = (assembled_two_bytes & 0xFF) as i8;
            offset += 2;
            count -= 1;
        }
        result.append(String::new(&buffer, StringUtils::GB2312_CHARSET));
    }

    fn  decode_kanji_segment( bits: &BitSource,  result: &StringBuilder,  count: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Don't crash trying to read more bits than we have available.
        if count * 13 > bits.available() {
            throw FormatException::get_format_instance();
        }
        // Each character will require 2 bytes. Read the characters as 2-byte pairs
        // and decode as Shift_JIS afterwards
         let mut buffer: [i8; 2 * count] = [0; 2 * count];
         let mut offset: i32 = 0;
        while count > 0 {
            // Each 13 bits encodes a 2-byte character
             let two_bytes: i32 = bits.read_bits(13);
             let assembled_two_bytes: i32 = ((two_bytes / 0x0C0) << 8) | (two_bytes % 0x0C0);
            if assembled_two_bytes < 0x01F00 {
                // In the 0x8140 to 0x9FFC range
                assembled_two_bytes += 0x08140;
            } else {
                // In the 0xE040 to 0xEBBF range
                assembled_two_bytes += 0x0C140;
            }
            buffer[offset] = (assembled_two_bytes >> 8) as i8;
            buffer[offset + 1] = assembled_two_bytes as i8;
            offset += 2;
            count -= 1;
        }
        result.append(String::new(&buffer, StringUtils::SHIFT_JIS_CHARSET));
    }

    fn  decode_byte_segment( bits: &BitSource,  result: &StringBuilder,  count: i32,  current_character_set_e_c_i: &CharacterSetECI,  byte_segments: &Collection<Vec<i8>>,  hints: &Map<DecodeHintType, ?>)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Don't crash trying to read more bits than we have available.
        if 8 * count > bits.available() {
            throw FormatException::get_format_instance();
        }
         let read_bytes: [i8; count] = [0; count];
         {
             let mut i: i32 = 0;
            while i < count {
                {
                    read_bytes[i] = bits.read_bits(8) as i8;
                }
                i += 1;
             }
         }

         let mut encoding: Charset;
        if current_character_set_e_c_i == null {
            // The spec isn't clear on this mode; see
            // section 6.4.5: t does not say which encoding to assuming
            // upon decoding. I have seen ISO-8859-1 used as well as
            // Shift_JIS -- without anything like an ECI designator to
            // give a hint.
            encoding = StringUtils::guess_charset(&read_bytes, &hints);
        } else {
            encoding = current_character_set_e_c_i.get_charset();
        }
        result.append(String::new(&read_bytes, &encoding));
        byte_segments.add(&read_bytes);
    }

    fn  to_alpha_numeric_char( value: i32) -> /*  throws FormatException */Result<char, Rc<Exception>>   {
        if value >= ALPHANUMERIC_CHARS.len() {
            throw FormatException::get_format_instance();
        }
        return Ok(ALPHANUMERIC_CHARS[value]);
    }

    fn  decode_alphanumeric_segment( bits: &BitSource,  result: &StringBuilder,  count: i32,  fc1_in_effect: bool)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Read two characters at a time
         let start: i32 = result.length();
        while count > 1 {
            if bits.available() < 11 {
                throw FormatException::get_format_instance();
            }
             let next_two_chars_bits: i32 = bits.read_bits(11);
            result.append(&::to_alpha_numeric_char(next_two_chars_bits / 45));
            result.append(&::to_alpha_numeric_char(next_two_chars_bits % 45));
            count -= 2;
        }
        if count == 1 {
            // special case: one character left
            if bits.available() < 6 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(&bits.read_bits(6)));
        }
        // See section 6.4.8.1, 6.4.8.2
        if fc1_in_effect {
            // We need to massage the result a bit if in an FNC1 mode:
             {
                 let mut i: i32 = start;
                while i < result.length() {
                    {
                        if result.char_at(i) == '%' {
                            if i < result.length() - 1 && result.char_at(i + 1) == '%' {
                                // %% is rendered as %
                                result.delete_char_at(i + 1);
                            } else {
                                // In alpha mode, % should be converted to FNC1 separator 0x1D
                                result.set_char_at(i, 0x1D as char);
                            }
                        }
                    }
                    i += 1;
                 }
             }

        }
    }

    fn  decode_numeric_segment( bits: &BitSource,  result: &StringBuilder,  count: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Read three digits at a time
        while count >= 3 {
            // Each 10 bits encodes three digits
            if bits.available() < 10 {
                throw FormatException::get_format_instance();
            }
             let three_digits_bits: i32 = bits.read_bits(10);
            if three_digits_bits >= 1000 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(three_digits_bits / 100));
            result.append(&::to_alpha_numeric_char((three_digits_bits / 10) % 10));
            result.append(&::to_alpha_numeric_char(three_digits_bits % 10));
            count -= 3;
        }
        if count == 2 {
            // Two digits left over to read, encoded in 7 bits
            if bits.available() < 7 {
                throw FormatException::get_format_instance();
            }
             let two_digits_bits: i32 = bits.read_bits(7);
            if two_digits_bits >= 100 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(two_digits_bits / 10));
            result.append(&::to_alpha_numeric_char(two_digits_bits % 10));
        } else if count == 1 {
            // One digit left over to read
            if bits.available() < 4 {
                throw FormatException::get_format_instance();
            }
             let digit_bits: i32 = bits.read_bits(4);
            if digit_bits >= 10 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(digit_bits));
        }
    }

    fn  parse_e_c_i_value( bits: &BitSource) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
         let first_byte: i32 = bits.read_bits(8);
        if (first_byte & 0x80) == 0 {
            // just one byte
            return Ok(first_byte & 0x7F);
        }
        if (first_byte & 0xC0) == 0x80 {
            // two bytes
             let second_byte: i32 = bits.read_bits(8);
            return Ok(((first_byte & 0x3F) << 8) | second_byte);
        }
        if (first_byte & 0xE0) == 0xC0 {
            // three bytes
             let second_third_bytes: i32 = bits.read_bits(16);
            return Ok(((first_byte & 0x1F) << 16) | second_third_bytes);
        }
        throw FormatException::get_format_instance();
    }
}

