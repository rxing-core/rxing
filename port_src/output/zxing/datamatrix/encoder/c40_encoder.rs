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
struct C40Encoder {
}

impl C40Encoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::C40_ENCODATION;
    }

    fn  encode_maximal(&self,  context: &EncoderContext)   {
         let buffer: StringBuilder = StringBuilder::new();
         let last_char_size: i32 = 0;
         let backtrack_start_position: i32 = context.pos;
         let backtrack_buffer_length: i32 = 0;
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            context.pos += 1;
            last_char_size = self.encode_char(c, &buffer);
            if buffer.length() % 3 == 0 {
                backtrack_start_position = context.pos;
                backtrack_buffer_length = buffer.length();
            }
        }
        if backtrack_buffer_length != buffer.length() {
             let unwritten: i32 = (buffer.length() / 3) * 2;
            // +1 for the latch to C40
             let cur_codeword_count: i32 = context.get_codeword_count() + unwritten + 1;
            context.update_symbol_info(cur_codeword_count);
             let available: i32 = context.get_symbol_info().get_data_capacity() - cur_codeword_count;
             let rest: i32 = buffer.length() % 3;
            if (rest == 2 && available != 2) || (rest == 1 && (last_char_size > 3 || available != 1)) {
                buffer.set_length(backtrack_buffer_length);
                context.pos = backtrack_start_position;
            }
        }
        if buffer.length() > 0 {
            context.write_codeword(HighLevelEncoder::LATCH_TO_C40);
        }
        self.handle_e_o_d(context, &buffer);
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step C
         let buffer: StringBuilder = StringBuilder::new();
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            context.pos += 1;
             let last_char_size: i32 = self.encode_char(c, &buffer);
             let unwritten: i32 = (buffer.length() / 3) * 2;
             let cur_codeword_count: i32 = context.get_codeword_count() + unwritten;
            context.update_symbol_info(cur_codeword_count);
             let available: i32 = context.get_symbol_info().get_data_capacity() - cur_codeword_count;
            if !context.has_more_characters() {
                //Avoid having a single C40 value in the last triplet
                 let removed: StringBuilder = StringBuilder::new();
                if (buffer.length() % 3) == 2 && available != 2 {
                    last_char_size = self.backtrack_one_character(context, &buffer, &removed, last_char_size);
                }
                while (buffer.length() % 3) == 1 && (last_char_size > 3 || available != 1) {
                    last_char_size = self.backtrack_one_character(context, &buffer, &removed, last_char_size);
                }
                break;
            }
             let count: i32 = buffer.length();
            if (count % 3) == 0 {
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

    fn  backtrack_one_character(&self,  context: &EncoderContext,  buffer: &StringBuilder,  removed: &StringBuilder,  last_char_size: i32) -> i32  {
         let count: i32 = buffer.length();
        buffer.delete(count - last_char_size, count);
        context.pos -= 1;
         let c: char = context.get_current_char();
        last_char_size = self.encode_char(c, &removed);
        //Deal with possible reduction in symbol size
        context.reset_symbol_info();
        return last_char_size;
    }

    fn  write_next_triplet( context: &EncoderContext,  buffer: &StringBuilder)   {
        context.write_codewords(&::encode_to_codewords(&buffer));
        buffer.delete(0, 3);
    }

    /**
   * Handle "end of data" situations
   *
   * @param context the encoder context
   * @param buffer  the buffer with the remaining encoded characters
   */
    fn  handle_e_o_d(&self,  context: &EncoderContext,  buffer: &StringBuilder)   {
         let unwritten: i32 = (buffer.length() / 3) * 2;
         let rest: i32 = buffer.length() % 3;
         let cur_codeword_count: i32 = context.get_codeword_count() + unwritten;
        context.update_symbol_info(cur_codeword_count);
         let available: i32 = context.get_symbol_info().get_data_capacity() - cur_codeword_count;
        if rest == 2 {
            //Shift 1
            buffer.append('\0');
            while buffer.length() >= 3 {
                ::write_next_triplet(context, &buffer);
            }
            if context.has_more_characters() {
                context.write_codeword(HighLevelEncoder::C40_UNLATCH);
            }
        } else if available == 1 && rest == 1 {
            while buffer.length() >= 3 {
                ::write_next_triplet(context, &buffer);
            }
            if context.has_more_characters() {
                context.write_codeword(HighLevelEncoder::C40_UNLATCH);
            }
            // else no unlatch
            context.pos -= 1;
        } else if rest == 0 {
            while buffer.length() >= 3 {
                ::write_next_triplet(context, &buffer);
            }
            if available > 0 || context.has_more_characters() {
                context.write_codeword(HighLevelEncoder::C40_UNLATCH);
            }
        } else {
            throw IllegalStateException::new("Unexpected case. Please report!");
        }
        context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
    }

    fn  encode_char(&self,  c: char,  sb: &StringBuilder) -> i32  {
        if c == ' ' {
            sb.append('\3');
            return 1;
        }
        if c >= '0' && c <= '9' {
            sb.append((c - 48 + 4) as char);
            return 1;
        }
        if c >= 'A' && c <= 'Z' {
            sb.append((c - 65 + 14) as char);
            return 1;
        }
        if c < ' ' {
            //Shift 1 Set
            sb.append('\0');
            sb.append(c);
            return 2;
        }
        if c <= '/' {
            //Shift 2 Set
            sb.append('\1');
            sb.append((c - 33) as char);
            return 2;
        }
        if c <= '@' {
            //Shift 2 Set
            sb.append('\1');
            sb.append((c - 58 + 15) as char);
            return 2;
        }
        if c <= '_' {
            //Shift 2 Set
            sb.append('\1');
            sb.append((c - 91 + 22) as char);
            return 2;
        }
        if c <= 127 {
            //Shift 3 Set
            sb.append('\2');
            sb.append((c - 96) as char);
            return 2;
        }
        //Shift 2, Upper Shift
        sb.append("\1");
         let mut len: i32 = 2;
        len += self.encode_char((c - 128) as char, &sb);
        return len;
    }

    fn  encode_to_codewords( sb: &CharSequence) -> String  {
         let v: i32 = (1600 * sb.char_at(0)) + (40 * sb.char_at(1)) + sb.char_at(2) + 1;
         let cw1: char = (v / 256) as char;
         let cw2: char = (v % 256) as char;
        return String::new( : vec![char; 2] = vec![cw1, cw2, ]
        );
    }
}

