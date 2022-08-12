/*
 * Copyright (C) 2010 ZXing authors
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
// package com::google::zxing::oned::rss::expanded::decoders;

/**
 * @author Pablo OrduÃ±a, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
struct GeneralAppIdDecoder {

     let information: BitArray;

     let current: CurrentParsingState = CurrentParsingState::new();

     let buffer: StringBuilder = StringBuilder::new();
}

impl GeneralAppIdDecoder {

    fn new( information: &BitArray) -> GeneralAppIdDecoder {
        let .information = information;
    }

    fn  decode_all_codes(&self,  buff: &StringBuilder,  initial_position: i32) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
         let current_position: i32 = initial_position;
         let mut remaining: String = null;
        loop { {
             let info: DecodedInformation = self.decode_general_purpose_field(current_position, &remaining);
             let parsed_fields: String = FieldParser::parse_fields_in_general_purpose(&info.get_new_string());
            if parsed_fields != null {
                buff.append(&parsed_fields);
            }
            if info.is_remaining() {
                remaining = String::value_of(&info.get_remaining_value());
            } else {
                remaining = null;
            }
            if current_position == info.get_new_position() {
                // No step forward!
                break;
            }
            current_position = info.get_new_position();
        }if !(true) break;}
        return Ok(buff.to_string());
    }

    fn  is_still_numeric(&self,  pos: i32) -> bool  {
        // and one of the first 4 bits is "1".
        if pos + 7 > self.information.get_size() {
            return pos + 4 <= self.information.get_size();
        }
         {
             let mut i: i32 = pos;
            while i < pos + 3 {
                {
                    if self.information.get(i) {
                        return true;
                    }
                }
                i += 1;
             }
         }

        return self.information.get(pos + 3);
    }

    fn  decode_numeric(&self,  pos: i32) -> /*  throws FormatException */Result<DecodedNumeric, Rc<Exception>>   {
        if pos + 7 > self.information.get_size() {
             let numeric: i32 = ::extract_numeric_value_from_bit_array(pos, 4);
            if numeric == 0 {
                return Ok(DecodedNumeric::new(&self.information.get_size(), DecodedNumeric::FNC1, DecodedNumeric::FNC1));
            }
            return Ok(DecodedNumeric::new(&self.information.get_size(), numeric - 1, DecodedNumeric::FNC1));
        }
         let numeric: i32 = ::extract_numeric_value_from_bit_array(pos, 7);
         let digit1: i32 = (numeric - 8) / 11;
         let digit2: i32 = (numeric - 8) % 11;
        return Ok(DecodedNumeric::new(pos + 7, digit1, digit2));
    }

    fn  extract_numeric_value_from_bit_array(&self,  pos: i32,  bits: i32) -> i32  {
        return ::extract_numeric_value_from_bit_array(self.information, pos, bits);
    }

    fn  extract_numeric_value_from_bit_array( information: &BitArray,  pos: i32,  bits: i32) -> i32  {
         let mut value: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < bits {
                {
                    if information.get(pos + i) {
                        value |= 1 << (bits - i - 1);
                    }
                }
                i += 1;
             }
         }

        return value;
    }

    fn  decode_general_purpose_field(&self,  pos: i32,  remaining: &String) -> /*  throws FormatException */Result<DecodedInformation, Rc<Exception>>   {
        self.buffer.set_length(0);
        if remaining != null {
            self.buffer.append(&remaining);
        }
        self.current.set_position(pos);
         let last_decoded: DecodedInformation = self.parse_blocks();
        if last_decoded != null && last_decoded.is_remaining() {
            return Ok(DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string(), &last_decoded.get_remaining_value()));
        }
        return Ok(DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string()));
    }

    fn  parse_blocks(&self) -> /*  throws FormatException */Result<DecodedInformation, Rc<Exception>>   {
         let is_finished: bool;
         let mut result: BlockParsedResult;
        loop { {
             let initial_position: i32 = self.current.get_position();
            if self.current.is_alpha() {
                result = self.parse_alpha_block();
                is_finished = result.is_finished();
            } else if self.current.is_iso_iec646() {
                result = self.parse_iso_iec646_block();
                is_finished = result.is_finished();
            } else {
                // it must be numeric
                result = self.parse_numeric_block();
                is_finished = result.is_finished();
            }
             let position_changed: bool = initial_position != self.current.get_position();
            if !position_changed && !is_finished {
                break;
            }
        }if !(!is_finished) break;}
        return Ok(result.get_decoded_information());
    }

    fn  parse_numeric_block(&self) -> /*  throws FormatException */Result<BlockParsedResult, Rc<Exception>>   {
        while self.is_still_numeric(&self.current.get_position()) {
             let numeric: DecodedNumeric = self.decode_numeric(&self.current.get_position());
            self.current.set_position(&numeric.get_new_position());
            if numeric.is_first_digit_f_n_c1() {
                 let mut information: DecodedInformation;
                if numeric.is_second_digit_f_n_c1() {
                    information = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                } else {
                    information = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string(), &numeric.get_second_digit());
                }
                return Ok(BlockParsedResult::new(information, true));
            }
            self.buffer.append(&numeric.get_first_digit());
            if numeric.is_second_digit_f_n_c1() {
                 let information: DecodedInformation = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                return Ok(BlockParsedResult::new(information, true));
            }
            self.buffer.append(&numeric.get_second_digit());
        }
        if self.is_numeric_to_alpha_numeric_latch(&self.current.get_position()) {
            self.current.set_alpha();
            self.current.increment_position(4);
        }
        return Ok(BlockParsedResult::new());
    }

    fn  parse_iso_iec646_block(&self) -> /*  throws FormatException */Result<BlockParsedResult, Rc<Exception>>   {
        while self.is_still_iso_iec646(&self.current.get_position()) {
             let iso: DecodedChar = self.decode_iso_iec646(&self.current.get_position());
            self.current.set_position(&iso.get_new_position());
            if iso.is_f_n_c1() {
                 let information: DecodedInformation = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                return Ok(BlockParsedResult::new(information, true));
            }
            self.buffer.append(&iso.get_value());
        }
        if self.is_alpha_or646_to_numeric_latch(&self.current.get_position()) {
            self.current.increment_position(3);
            self.current.set_numeric();
        } else if self.is_alpha_to646_to_alpha_latch(&self.current.get_position()) {
            if self.current.get_position() + 5 < self.information.get_size() {
                self.current.increment_position(5);
            } else {
                self.current.set_position(&self.information.get_size());
            }
            self.current.set_alpha();
        }
        return Ok(BlockParsedResult::new());
    }

    fn  parse_alpha_block(&self) -> BlockParsedResult  {
        while self.is_still_alpha(&self.current.get_position()) {
             let alpha: DecodedChar = self.decode_alphanumeric(&self.current.get_position());
            self.current.set_position(&alpha.get_new_position());
            if alpha.is_f_n_c1() {
                 let information: DecodedInformation = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                //end of the char block
                return BlockParsedResult::new(information, true);
            }
            self.buffer.append(&alpha.get_value());
        }
        if self.is_alpha_or646_to_numeric_latch(&self.current.get_position()) {
            self.current.increment_position(3);
            self.current.set_numeric();
        } else if self.is_alpha_to646_to_alpha_latch(&self.current.get_position()) {
            if self.current.get_position() + 5 < self.information.get_size() {
                self.current.increment_position(5);
            } else {
                self.current.set_position(&self.information.get_size());
            }
            self.current.set_iso_iec646();
        }
        return BlockParsedResult::new();
    }

    fn  is_still_iso_iec646(&self,  pos: i32) -> bool  {
        if pos + 5 > self.information.get_size() {
            return false;
        }
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value >= 5 && five_bit_value < 16 {
            return true;
        }
        if pos + 7 > self.information.get_size() {
            return false;
        }
         let seven_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 7);
        if seven_bit_value >= 64 && seven_bit_value < 116 {
            return true;
        }
        if pos + 8 > self.information.get_size() {
            return false;
        }
         let eight_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 8);
        return eight_bit_value >= 232 && eight_bit_value < 253;
    }

    fn  decode_iso_iec646(&self,  pos: i32) -> /*  throws FormatException */Result<DecodedChar, Rc<Exception>>   {
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value == 15 {
            return Ok(DecodedChar::new(pos + 5, DecodedChar::FNC1));
        }
        if five_bit_value >= 5 && five_bit_value < 15 {
            return Ok(DecodedChar::new(pos + 5, ('0' + five_bit_value - 5) as char));
        }
         let seven_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 7);
        if seven_bit_value >= 64 && seven_bit_value < 90 {
            return Ok(DecodedChar::new(pos + 7, (seven_bit_value + 1) as char));
        }
        if seven_bit_value >= 90 && seven_bit_value < 116 {
            return Ok(DecodedChar::new(pos + 7, (seven_bit_value + 7) as char));
        }
         let eight_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 8);
         let mut c: char;
        match eight_bit_value {
              232 => 
                 {
                    c = '!';
                    break;
                }
              233 => 
                 {
                    c = '"';
                    break;
                }
              234 => 
                 {
                    c = '%';
                    break;
                }
              235 => 
                 {
                    c = '&';
                    break;
                }
              236 => 
                 {
                    c = '\'';
                    break;
                }
              237 => 
                 {
                    c = '(';
                    break;
                }
              238 => 
                 {
                    c = ')';
                    break;
                }
              239 => 
                 {
                    c = '*';
                    break;
                }
              240 => 
                 {
                    c = '+';
                    break;
                }
              241 => 
                 {
                    c = ',';
                    break;
                }
              242 => 
                 {
                    c = '-';
                    break;
                }
              243 => 
                 {
                    c = '.';
                    break;
                }
              244 => 
                 {
                    c = '/';
                    break;
                }
              245 => 
                 {
                    c = ':';
                    break;
                }
              246 => 
                 {
                    c = ';';
                    break;
                }
              247 => 
                 {
                    c = '<';
                    break;
                }
              248 => 
                 {
                    c = '=';
                    break;
                }
              249 => 
                 {
                    c = '>';
                    break;
                }
              250 => 
                 {
                    c = '?';
                    break;
                }
              251 => 
                 {
                    c = '_';
                    break;
                }
              252 => 
                 {
                    c = ' ';
                    break;
                }
            _ => 
                 {
                    throw FormatException::get_format_instance();
                }
        }
        return Ok(DecodedChar::new(pos + 8, c));
    }

    fn  is_still_alpha(&self,  pos: i32) -> bool  {
        if pos + 5 > self.information.get_size() {
            return false;
        }
        // We now check if it's a valid 5-bit value (0..9 and FNC1)
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value >= 5 && five_bit_value < 16 {
            return true;
        }
        if pos + 6 > self.information.get_size() {
            return false;
        }
         let six_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 6);
        // 63 not included
        return six_bit_value >= 16 && six_bit_value < 63;
    }

    fn  decode_alphanumeric(&self,  pos: i32) -> DecodedChar  {
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value == 15 {
            return DecodedChar::new(pos + 5, DecodedChar::FNC1);
        }
        if five_bit_value >= 5 && five_bit_value < 15 {
            return DecodedChar::new(pos + 5, ('0' + five_bit_value - 5) as char);
        }
         let six_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 6);
        if six_bit_value >= 32 && six_bit_value < 58 {
            return DecodedChar::new(pos + 6, (six_bit_value + 33) as char);
        }
         let mut c: char;
        match six_bit_value {
              58 => 
                 {
                    c = '*';
                    break;
                }
              59 => 
                 {
                    c = ',';
                    break;
                }
              60 => 
                 {
                    c = '-';
                    break;
                }
              61 => 
                 {
                    c = '.';
                    break;
                }
              62 => 
                 {
                    c = '/';
                    break;
                }
            _ => 
                 {
                    throw IllegalStateException::new(format!("Decoding invalid alphanumeric value: {}", six_bit_value));
                }
        }
        return DecodedChar::new(pos + 6, c);
    }

    fn  is_alpha_to646_to_alpha_latch(&self,  pos: i32) -> bool  {
        if pos + 1 > self.information.get_size() {
            return false;
        }
         {
             let mut i: i32 = 0;
            while i < 5 && i + pos < self.information.get_size() {
                {
                    if i == 2 {
                        if !self.information.get(pos + 2) {
                            return false;
                        }
                    } else if self.information.get(pos + i) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    fn  is_alpha_or646_to_numeric_latch(&self,  pos: i32) -> bool  {
        // Next is alphanumeric if there are 3 positions and they are all zeros
        if pos + 3 > self.information.get_size() {
            return false;
        }
         {
             let mut i: i32 = pos;
            while i < pos + 3 {
                {
                    if self.information.get(i) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    fn  is_numeric_to_alpha_numeric_latch(&self,  pos: i32) -> bool  {
        // if there is a subset of this just before the end of the symbol
        if pos + 1 > self.information.get_size() {
            return false;
        }
         {
             let mut i: i32 = 0;
            while i < 4 && i + pos < self.information.get_size() {
                {
                    if self.information.get(pos + i) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }
}

