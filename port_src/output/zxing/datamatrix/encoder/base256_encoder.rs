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
struct Base256Encoder {
}

impl Base256Encoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::BASE256_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
         let buffer: StringBuilder = StringBuilder::new();
        //Initialize length field
        buffer.append('\0');
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            buffer.append(c);
            context.pos += 1;
             let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
            if new_mode != self.get_encoding_mode() {
                // Return to ASCII encodation, which will actually handle latch to new mode
                context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
                break;
            }
        }
         let data_count: i32 = buffer.length() - 1;
         let length_field_size: i32 = 1;
         let current_size: i32 = context.get_codeword_count() + data_count + length_field_size;
        context.update_symbol_info(current_size);
         let must_pad: bool = (context.get_symbol_info().get_data_capacity() - current_size) > 0;
        if context.has_more_characters() || must_pad {
            if data_count <= 249 {
                buffer.set_char_at(0, data_count as char);
            } else if data_count <= 1555 {
                buffer.set_char_at(0, ((data_count / 250) + 249) as char);
                buffer.insert(1, (data_count % 250) as char);
            } else {
                throw IllegalStateException::new(format!("Message length not in valid ranges: {}", data_count));
            }
        }
         {
             let mut i: i32 = 0, let c: i32 = buffer.length();
            while i < c {
                {
                    context.write_codeword(&::randomize255_state(&buffer.char_at(i), context.get_codeword_count() + 1));
                }
                i += 1;
             }
         }

    }

    fn  randomize255_state( ch: char,  codeword_position: i32) -> char  {
         let pseudo_random: i32 = ((149 * codeword_position) % 255) + 1;
         let temp_variable: i32 = ch + pseudo_random;
        if temp_variable <= 255 {
            return temp_variable as char;
        } else {
            return (temp_variable - 256) as char;
        }
    }
}

