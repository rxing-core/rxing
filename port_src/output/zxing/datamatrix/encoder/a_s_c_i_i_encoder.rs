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
struct ASCIIEncoder {
}

impl ASCIIEncoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::ASCII_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step B
         let n: i32 = HighLevelEncoder::determine_consecutive_digit_count(&context.get_message(), context.pos);
        if n >= 2 {
            context.write_codeword(&::encode_a_s_c_i_i_digits(&context.get_message().char_at(context.pos), &context.get_message().char_at(context.pos + 1)));
            context.pos += 2;
        } else {
             let c: char = context.get_current_char();
             let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
            if new_mode != self.get_encoding_mode() {
                match new_mode {
                      HighLevelEncoder::BASE256_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_BASE256);
                            context.signal_encoder_change(HighLevelEncoder::BASE256_ENCODATION);
                            return;
                        }
                      HighLevelEncoder::C40_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_C40);
                            context.signal_encoder_change(HighLevelEncoder::C40_ENCODATION);
                            return;
                        }
                      HighLevelEncoder::X12_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_ANSIX12);
                            context.signal_encoder_change(HighLevelEncoder::X12_ENCODATION);
                            break;
                        }
                      HighLevelEncoder::TEXT_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_TEXT);
                            context.signal_encoder_change(HighLevelEncoder::TEXT_ENCODATION);
                            break;
                        }
                      HighLevelEncoder::EDIFACT_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_EDIFACT);
                            context.signal_encoder_change(HighLevelEncoder::EDIFACT_ENCODATION);
                            break;
                        }
                    _ => 
                         {
                            throw IllegalStateException::new(format!("Illegal mode: {}", new_mode));
                        }
                }
            } else if HighLevelEncoder::is_extended_a_s_c_i_i(c) {
                context.write_codeword(HighLevelEncoder::UPPER_SHIFT);
                context.write_codeword((c - 128 + 1) as char);
                context.pos += 1;
            } else {
                context.write_codeword((c + 1) as char);
                context.pos += 1;
            }
        }
    }

    fn  encode_a_s_c_i_i_digits( digit1: char,  digit2: char) -> char  {
        if HighLevelEncoder::is_digit(digit1) && HighLevelEncoder::is_digit(digit2) {
             let num: i32 = (digit1 - 48) * 10 + (digit2 - 48);
            return (num + 130) as char;
        }
        throw IllegalArgumentException::new(format!("not digits: {}{}", digit1, digit2));
    }
}

