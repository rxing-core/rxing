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

/**
 * DataMatrix ECC 200 data encoder following the algorithm described in ISO/IEC 16022:200(E) in
 * annex S.
 */

/**
   * Padding character
   */
 const PAD: char = 129;

/**
   * mode latch to C40 encodation mode
   */
 const LATCH_TO_C40: char = 230;

/**
   * mode latch to Base 256 encodation mode
   */
 const LATCH_TO_BASE256: char = 231;

/**
   * FNC1 Codeword
   */
//private static final char FNC1 = 232;
/**
   * Structured Append Codeword
   */
//private static final char STRUCTURED_APPEND = 233;
/**
   * Reader Programming
   */
//private static final char READER_PROGRAMMING = 234;
/**
   * Upper Shift
   */
 const UPPER_SHIFT: char = 235;

/**
   * 05 Macro
   */
 const MACRO_05: char = 236;

/**
   * 06 Macro
   */
 const MACRO_06: char = 237;

/**
   * mode latch to ANSI X.12 encodation mode
   */
 const LATCH_TO_ANSIX12: char = 238;

/**
   * mode latch to Text encodation mode
   */
 const LATCH_TO_TEXT: char = 239;

/**
   * mode latch to EDIFACT encodation mode
   */
 const LATCH_TO_EDIFACT: char = 240;

/**
   * ECI character (Extended Channel Interpretation)
   */
//private static final char ECI = 241;
/**
   * Unlatch from C40 encodation
   */
 const C40_UNLATCH: char = 254;

/**
   * Unlatch from X12 encodation
   */
 const X12_UNLATCH: char = 254;

/**
   * 05 Macro header
   */
 const MACRO_05_HEADER: &'static str = "[)>05";

/**
   * 06 Macro header
   */
 const MACRO_06_HEADER: &'static str = "[)>06";

/**
   * Macro trailer
   */
 const MACRO_TRAILER: &'static str = "";

 const ASCII_ENCODATION: i32 = 0;

 const C40_ENCODATION: i32 = 1;

 const TEXT_ENCODATION: i32 = 2;

 const X12_ENCODATION: i32 = 3;

 const EDIFACT_ENCODATION: i32 = 4;

 const BASE256_ENCODATION: i32 = 5;
pub struct HighLevelEncoder {
}

impl HighLevelEncoder {

    fn new() -> HighLevelEncoder {
    }

    fn  randomize253_state( codeword_position: i32) -> char  {
         let pseudo_random: i32 = ((149 * codeword_position) % 253) + 1;
         let temp_variable: i32 = PAD + pseudo_random;
        return ( if temp_variable <= 254 { temp_variable } else { temp_variable - 254 }) as char;
    }

    /**
   * Performs message encoding of a DataMatrix message using the algorithm described in annex P
   * of ISO/IEC 16022:2000(E).
   *
   * @param msg the message
   * @return the encoded message (the char values range from 0 to 255)
   */
    pub fn  encode_high_level( msg: &String) -> String  {
        return ::encode_high_level(&msg, SymbolShapeHint::FORCE_NONE, null, null, false);
    }

    /**
   * Performs message encoding of a DataMatrix message using the algorithm described in annex P
   * of ISO/IEC 16022:2000(E).
   *
   * @param msg     the message
   * @param shape   requested shape. May be {@code SymbolShapeHint.FORCE_NONE},
   *                {@code SymbolShapeHint.FORCE_SQUARE} or {@code SymbolShapeHint.FORCE_RECTANGLE}.
   * @param minSize the minimum symbol size constraint or null for no constraint
   * @param maxSize the maximum symbol size constraint or null for no constraint
   * @return the encoded message (the char values range from 0 to 255)
   */
    pub fn  encode_high_level( msg: &String,  shape: &SymbolShapeHint,  min_size: &Dimension,  max_size: &Dimension) -> String  {
        return ::encode_high_level(&msg, shape, min_size, max_size, false);
    }

    /**
   * Performs message encoding of a DataMatrix message using the algorithm described in annex P
   * of ISO/IEC 16022:2000(E).
   *
   * @param msg     the message
   * @param shape   requested shape. May be {@code SymbolShapeHint.FORCE_NONE},
   *                {@code SymbolShapeHint.FORCE_SQUARE} or {@code SymbolShapeHint.FORCE_RECTANGLE}.
   * @param minSize the minimum symbol size constraint or null for no constraint
   * @param maxSize the maximum symbol size constraint or null for no constraint
   * @param forceC40 enforce C40 encoding
   * @return the encoded message (the char values range from 0 to 255)
   */
    pub fn  encode_high_level( msg: &String,  shape: &SymbolShapeHint,  min_size: &Dimension,  max_size: &Dimension,  force_c40: bool) -> String  {
        //the codewords 0..255 are encoded as Unicode characters
         let c40_encoder: C40Encoder = C40Encoder::new();
         let encoders: vec![Vec<Encoder>; 6] = vec![ASCIIEncoder::new(), c40_encoder, TextEncoder::new(), X12Encoder::new(), EdifactEncoder::new(), Base256Encoder::new(), ]
        ;
         let mut context: EncoderContext = EncoderContext::new(&msg);
        context.set_symbol_shape(shape);
        context.set_size_constraints(min_size, max_size);
        if msg.starts_with(&MACRO_05_HEADER) && msg.ends_with(&MACRO_TRAILER) {
            context.write_codeword(MACRO_05);
            context.set_skip_at_end(2);
            context.pos += MACRO_05_HEADER::length();
        } else if msg.starts_with(&MACRO_06_HEADER) && msg.ends_with(&MACRO_TRAILER) {
            context.write_codeword(MACRO_06);
            context.set_skip_at_end(2);
            context.pos += MACRO_06_HEADER::length();
        }
        //Default mode
         let encoding_mode: i32 = ASCII_ENCODATION;
        if force_c40 {
            c40_encoder.encode_maximal(context);
            encoding_mode = context.get_new_encoding();
            context.reset_encoder_signal();
        }
        while context.has_more_characters() {
            encoders[encoding_mode].encode(context);
            if context.get_new_encoding() >= 0 {
                encoding_mode = context.get_new_encoding();
                context.reset_encoder_signal();
            }
        }
         let len: i32 = context.get_codeword_count();
        context.update_symbol_info();
         let capacity: i32 = context.get_symbol_info().get_data_capacity();
        if len < capacity && encoding_mode != ASCII_ENCODATION && encoding_mode != BASE256_ENCODATION && encoding_mode != EDIFACT_ENCODATION {
            //Unlatch (254)
            context.write_codeword('þ');
        }
        //Padding
         let codewords: StringBuilder = context.get_codewords();
        if codewords.length() < capacity {
            codewords.append(PAD);
        }
        while codewords.length() < capacity {
            codewords.append(&::randomize253_state(codewords.length() + 1));
        }
        return context.get_codewords().to_string();
    }

    fn  look_ahead_test( msg: &CharSequence,  startpos: i32,  current_mode: i32) -> i32  {
         let new_mode: i32 = ::look_ahead_test_intern(&msg, startpos, current_mode);
        if current_mode == X12_ENCODATION && new_mode == X12_ENCODATION {
             let endpos: i32 = Math::min(startpos + 3, &msg.length());
             {
                 let mut i: i32 = startpos;
                while i < endpos {
                    {
                        if !::is_native_x12(&msg.char_at(i)) {
                            return ASCII_ENCODATION;
                        }
                    }
                    i += 1;
                 }
             }

        } else if current_mode == EDIFACT_ENCODATION && new_mode == EDIFACT_ENCODATION {
             let endpos: i32 = Math::min(startpos + 4, &msg.length());
             {
                 let mut i: i32 = startpos;
                while i < endpos {
                    {
                        if !::is_native_e_d_i_f_a_c_t(&msg.char_at(i)) {
                            return ASCII_ENCODATION;
                        }
                    }
                    i += 1;
                 }
             }

        }
        return new_mode;
    }

    fn  look_ahead_test_intern( msg: &CharSequence,  startpos: i32,  current_mode: i32) -> i32  {
        if startpos >= msg.length() {
            return current_mode;
        }
         let char_counts: Vec<f32>;
        //step J
        if current_mode == ASCII_ENCODATION {
            char_counts =  : vec![f32; 6] = vec![0.0, 1.0, 1.0, 1.0, 1.0, 1.25f, ]
            ;
        } else {
            char_counts =  : vec![f32; 6] = vec![1.0, 2.0, 2.0, 2.0, 2.0, 2.25f, ]
            ;
            char_counts[current_mode] = 0.0;
        }
         let chars_processed: i32 = 0;
         let mins: [i8; 6] = [0; 6];
         let int_char_counts: [i32; 6] = [0; 6];
        while true {
            //step K
            if (startpos + chars_processed) == msg.length() {
                Arrays::fill(&mins, 0 as i8);
                Arrays::fill(&int_char_counts, 0);
                 let min: i32 = ::find_minimums(&char_counts, &int_char_counts, Integer::MAX_VALUE, &mins);
                 let min_count: i32 = ::get_minimum_count(&mins);
                if int_char_counts[ASCII_ENCODATION] == min {
                    return ASCII_ENCODATION;
                }
                if min_count == 1 {
                    if mins[BASE256_ENCODATION] > 0 {
                        return BASE256_ENCODATION;
                    }
                    if mins[EDIFACT_ENCODATION] > 0 {
                        return EDIFACT_ENCODATION;
                    }
                    if mins[TEXT_ENCODATION] > 0 {
                        return TEXT_ENCODATION;
                    }
                    if mins[X12_ENCODATION] > 0 {
                        return X12_ENCODATION;
                    }
                }
                return C40_ENCODATION;
            }
             let c: char = msg.char_at(startpos + chars_processed);
            chars_processed += 1;
            //step L
            if ::is_digit(c) {
                char_counts[ASCII_ENCODATION] += 0.5f;
            } else if ::is_extended_a_s_c_i_i(c) {
                char_counts[ASCII_ENCODATION] = Math::ceil(char_counts[ASCII_ENCODATION]) as f32;
                char_counts[ASCII_ENCODATION] += 2.0f;
            } else {
                char_counts[ASCII_ENCODATION] = Math::ceil(char_counts[ASCII_ENCODATION]) as f32;
                char_counts[ASCII_ENCODATION] += 1;
            }
            //step M
            if ::is_native_c40(c) {
                char_counts[C40_ENCODATION] += 2.0f / 3.0f;
            } else if ::is_extended_a_s_c_i_i(c) {
                char_counts[C40_ENCODATION] += 8.0f / 3.0f;
            } else {
                char_counts[C40_ENCODATION] += 4.0f / 3.0f;
            }
            //step N
            if ::is_native_text(c) {
                char_counts[TEXT_ENCODATION] += 2.0f / 3.0f;
            } else if ::is_extended_a_s_c_i_i(c) {
                char_counts[TEXT_ENCODATION] += 8.0f / 3.0f;
            } else {
                char_counts[TEXT_ENCODATION] += 4.0f / 3.0f;
            }
            //step O
            if ::is_native_x12(c) {
                char_counts[X12_ENCODATION] += 2.0f / 3.0f;
            } else if ::is_extended_a_s_c_i_i(c) {
                char_counts[X12_ENCODATION] += 13.0f / 3.0f;
            } else {
                char_counts[X12_ENCODATION] += 10.0f / 3.0f;
            }
            //step P
            if ::is_native_e_d_i_f_a_c_t(c) {
                char_counts[EDIFACT_ENCODATION] += 3.0f / 4.0f;
            } else if ::is_extended_a_s_c_i_i(c) {
                char_counts[EDIFACT_ENCODATION] += 17.0f / 4.0f;
            } else {
                char_counts[EDIFACT_ENCODATION] += 13.0f / 4.0f;
            }
            // step Q
            if ::is_special_b256(c) {
                char_counts[BASE256_ENCODATION] += 4.0f;
            } else {
                char_counts[BASE256_ENCODATION] += 1;
            }
            //step R
            if chars_processed >= 4 {
                Arrays::fill(&mins, 0 as i8);
                Arrays::fill(&int_char_counts, 0);
                ::find_minimums(&char_counts, &int_char_counts, Integer::MAX_VALUE, &mins);
                if int_char_counts[ASCII_ENCODATION] < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[EDIFACT_ENCODATION]) {
                    return ASCII_ENCODATION;
                }
                if int_char_counts[BASE256_ENCODATION] < int_char_counts[ASCII_ENCODATION] || int_char_counts[BASE256_ENCODATION] + 1 < ::min(int_char_counts[C40_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[EDIFACT_ENCODATION]) {
                    return BASE256_ENCODATION;
                }
                if int_char_counts[EDIFACT_ENCODATION] + 1 < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[ASCII_ENCODATION]) {
                    return EDIFACT_ENCODATION;
                }
                if int_char_counts[TEXT_ENCODATION] + 1 < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[EDIFACT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[ASCII_ENCODATION]) {
                    return TEXT_ENCODATION;
                }
                if int_char_counts[X12_ENCODATION] + 1 < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[EDIFACT_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[ASCII_ENCODATION]) {
                    return X12_ENCODATION;
                }
                if int_char_counts[C40_ENCODATION] + 1 < ::min(int_char_counts[ASCII_ENCODATION], int_char_counts[BASE256_ENCODATION], int_char_counts[EDIFACT_ENCODATION], int_char_counts[TEXT_ENCODATION]) {
                    if int_char_counts[C40_ENCODATION] < int_char_counts[X12_ENCODATION] {
                        return C40_ENCODATION;
                    }
                    if int_char_counts[C40_ENCODATION] == int_char_counts[X12_ENCODATION] {
                         let mut p: i32 = startpos + chars_processed + 1;
                        while p < msg.length() {
                             let tc: char = msg.char_at(p);
                            if ::is_x12_term_sep(tc) {
                                return X12_ENCODATION;
                            }
                            if !::is_native_x12(tc) {
                                break;
                            }
                            p += 1;
                        }
                        return C40_ENCODATION;
                    }
                }
            }
        }
    }

    fn  min( f1: i32,  f2: i32,  f3: i32,  f4: i32,  f5: i32) -> i32  {
        return Math::min(&::min(f1, f2, f3, f4), f5);
    }

    fn  min( f1: i32,  f2: i32,  f3: i32,  f4: i32) -> i32  {
        return Math::min(f1, &Math::min(f2, &Math::min(f3, f4)));
    }

    fn  find_minimums( char_counts: &Vec<f32>,  int_char_counts: &Vec<i32>,  min: i32,  mins: &Vec<i8>) -> i32  {
         {
             let mut i: i32 = 0;
            while i < 6 {
                {
                     let current: i32 = (int_char_counts[i] = Math::ceil(char_counts[i]) as i32);
                    if min > current {
                        min = current;
                        Arrays::fill(&mins, 0 as i8);
                    }
                    if min == current {
                        mins[i] += 1;
                    }
                }
                i += 1;
             }
         }

        return min;
    }

    fn  get_minimum_count( mins: &Vec<i8>) -> i32  {
         let min_count: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 6 {
                {
                    min_count += mins[i];
                }
                i += 1;
             }
         }

        return min_count;
    }

    fn  is_digit( ch: char) -> bool  {
        return ch >= '0' && ch <= '9';
    }

    fn  is_extended_a_s_c_i_i( ch: char) -> bool  {
        return ch >= 128 && ch <= 255;
    }

    fn  is_native_c40( ch: char) -> bool  {
        return (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'A' && ch <= 'Z');
    }

    fn  is_native_text( ch: char) -> bool  {
        return (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'z');
    }

    fn  is_native_x12( ch: char) -> bool  {
        return ::is_x12_term_sep(ch) || (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'A' && ch <= 'Z');
    }

    fn  is_x12_term_sep( ch: char) -> bool  {
        return //CR
        (ch == '\r') || (ch == '*') || (ch == '>');
    }

    fn  is_native_e_d_i_f_a_c_t( ch: char) -> bool  {
        return ch >= ' ' && ch <= '^';
    }

    fn  is_special_b256( ch: char) -> bool  {
        //TODO NOT IMPLEMENTED YET!!!
        return false;
    }

    /**
   * Determines the number of consecutive characters that are encodable using numeric compaction.
   *
   * @param msg      the message
   * @param startpos the start position within the message
   * @return the requested character count
   */
    pub fn  determine_consecutive_digit_count( msg: &CharSequence,  startpos: i32) -> i32  {
         let len: i32 = msg.length();
         let mut idx: i32 = startpos;
        while idx < len && ::is_digit(&msg.char_at(idx)) {
            idx += 1;
        }
        return idx - startpos;
    }

    fn  illegal_character( c: char)   {
         let mut hex: String = Integer::to_hex_string(c);
        hex = format!("{}{}", "0000".substring(0, 4 - hex.length()), hex);
        throw IllegalArgumentException::new(format!("Illegal character: {} (0x{})", c, hex));
    }
}

