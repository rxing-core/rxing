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

struct X12Encoder {
    super: C40Encoder;
}

impl X12Encoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::X12_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step C
         let buffer: StringBuilder = StringBuilder::new();
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            context.pos += 1;
            self.encode_char(c, &buffer);
             let count: i32 = buffer.length();
            if (count % 3) == 0 {
                write_next_triplet(context, &buffer);
                 let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
                if new_mode != self.get_encoding_mode() {
                    // Return to ASCII encodation, which will actually handle latch to new mode
                    context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
                    break;
                }
            }
        }
        self.handle_e_o_d(context, &buffer);
    }

    fn  encode_char(&self,  c: char,  sb: &StringBuilder) -> i32  {
        match c {
              '\r' => 
                 {
                    sb.append('\0');
                    break;
                }
              '*' => 
                 {
                    sb.append('\1');
                    break;
                }
              '>' => 
                 {
                    sb.append('\2');
                    break;
                }
              ' ' => 
                 {
                    sb.append('\3');
                    break;
                }
            _ => 
                 {
                    if c >= '0' && c <= '9' {
                        sb.append((c - 48 + 4) as char);
                    } else if c >= 'A' && c <= 'Z' {
                        sb.append((c - 65 + 14) as char);
                    } else {
                        HighLevelEncoder::illegal_character(c);
                    }
                    break;
                }
        }
        return 1;
    }

    fn  handle_e_o_d(&self,  context: &EncoderContext,  buffer: &StringBuilder)   {
        context.update_symbol_info();
         let available: i32 = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
         let count: i32 = buffer.length();
        context.pos -= count;
        if context.get_remaining_characters() > 1 || available > 1 || context.get_remaining_characters() != available {
            context.write_codeword(HighLevelEncoder::X12_UNLATCH);
        }
        if context.get_new_encoding() < 0 {
            context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
        }
    }
}

