/*
 * Copyright 2006-2007 Jeremias Maerki.
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
// package com::google::zxing::datamatrix::encoder;

#[derive(Encoder)]
struct EdifactEncoder {
}

impl EdifactEncoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::EDIFACT_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step F
         let buffer: StringBuilder = StringBuilder::new();
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            ::encode_char(c, &buffer);
            context.pos += 1;
             let count: i32 = buffer.length();
            if count >= 4 {
                context.write_codewords(&::encode_to_codewords(&buffer));
                buffer.delete(0, 4);
                 let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
                if new_mode != self.get_encoding_mode() {
                    // Return to ASCII encodation, which will actually handle latch to new mode
                    context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
                    break;
                }
            }
        }
        //Unlatch
        buffer.append(31 as char);
        ::handle_e_o_d(context, &buffer);
    }

    /**
   * Handle "end of data" situations
   *
   * @param context the encoder context
   * @param buffer  the buffer with the remaining encoded characters
   */
    fn  handle_e_o_d( context: &EncoderContext,  buffer: &CharSequence)   {
        let tryResult1 = 0;
        'try1: loop {
        {
             let count: i32 = buffer.length();
            if count == 0 {
                //Already finished
                return;
            }
            if count == 1 {
                //Only an unlatch at the end
                context.update_symbol_info();
                 let mut available: i32 = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
                 let remaining: i32 = context.get_remaining_characters();
                // The following two lines are a hack inspired by the 'fix' from https://sourceforge.net/p/barcode4j/svn/221/
                if remaining > available {
                    context.update_symbol_info(context.get_codeword_count() + 1);
                    available = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
                }
                if remaining <= available && available <= 2 {
                    //No unlatch
                    return;
                }
            }
            if count > 4 {
                throw IllegalStateException::new("Count must not exceed 4");
            }
             let rest_chars: i32 = count - 1;
             let encoded: String = ::encode_to_codewords(&buffer);
             let end_of_symbol_reached: bool = !context.has_more_characters();
             let rest_in_ascii: bool = end_of_symbol_reached && rest_chars <= 2;
            if rest_chars <= 2 {
                context.update_symbol_info(context.get_codeword_count() + rest_chars);
                 let available: i32 = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
                if available >= 3 {
                    rest_in_ascii = false;
                    context.update_symbol_info(context.get_codeword_count() + encoded.length());
                //available = context.symbolInfo.dataCapacity - context.getCodewordCount();
                }
            }
            if rest_in_ascii {
                context.reset_symbol_info();
                context.pos -= rest_chars;
            } else {
                context.write_codewords(&encoded);
            }
        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }
         finally {
            context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
        }
    }

    fn  encode_char( c: char,  sb: &StringBuilder)   {
        if c >= ' ' && c <= '?' {
            sb.append(c);
        } else if c >= '@' && c <= '^' {
            sb.append((c - 64) as char);
        } else {
            HighLevelEncoder::illegal_character(c);
        }
    }

    fn  encode_to_codewords( sb: &CharSequence) -> String  {
         let len: i32 = sb.length();
        if len == 0 {
            throw IllegalStateException::new("StringBuilder must not be empty");
        }
         let c1: char = sb.char_at(0);
         let c2: char =  if len >= 2 { sb.char_at(1) } else { 0 };
         let c3: char =  if len >= 3 { sb.char_at(2) } else { 0 };
         let c4: char =  if len >= 4 { sb.char_at(3) } else { 0 };
         let v: i32 = (c1 << 18) + (c2 << 12) + (c3 << 6) + c4;
         let cw1: char = ((v >> 16) & 255) as char;
         let cw2: char = ((v >> 8) & 255) as char;
         let cw3: char = (v & 255) as char;
         let res: StringBuilder = StringBuilder::new(3);
        res.append(cw1);
        if len >= 2 {
            res.append(cw2);
        }
        if len >= 3 {
            res.append(cw3);
        }
        return res.to_string();
    }
}

