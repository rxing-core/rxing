/*
 * Copyright 2021 ZXing authors
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
 * Encoder that encodes minimally
 *
 * Algorithm:
 *
 * Uses Dijkstra to produce mathematically minimal encodings that are in some cases smaller than the results produced
 * by the algorithm described in annex S in the specification ISO/IEC 16022:200(E). The biggest improvment of this
 * algorithm over that one is the case when the algorithm enters the most inefficient mode, the B256 mode. The 
 * algorithm from the specification algorithm will exit this mode only if it encounters digits so that arbitrarily
 * inefficient results can be produced if the postfix contains no digits.
 *
 * Multi ECI support and ECI switching:
 *
 * For multi language content the algorithm selects the most compact representation using ECI modes. Note that unlike
 * the compaction algorithm used for QR-Codes, this implementation operates in two stages and therfore is not
 * mathematically optimal. In the first stage, the input string is encoded minimally as a stream of ECI character set
 * selectors and bytes encoded in the selected encoding. In this stage the algorithm might for example decide to
 * encode ocurrences of the characters "\u0150\u015C" (O-double-acute, S-circumflex) in UTF-8 by a single ECI or
 * alternatively by multiple ECIs that switch between IS0-8859-2 and ISO-8859-3 (e.g. in the case that the input
 * contains many * characters from ISO-8859-2 (Latin 2) and few from ISO-8859-3 (Latin 3)).
 * In a second stage this stream of ECIs and bytes is minimally encoded using the various Data Matrix encoding modes.
 * While both stages encode mathematically minimally it is not ensured that the result is mathematically minimal since
 * the size growth for inserting an ECI in the first stage can only be approximated as the first stage does not know 
 * in which mode the ECI will occur in the second stage (may, or may not require an extra latch to ASCII depending on
 * the current mode). The reason for this shortcoming are difficulties in implementing it in a straightforward and
 * readable manner.
 *
 * GS1 support
 *
 * FNC1 delimiters can be encoded in the input string by using the FNC1 character specified in the encoding function.
 * When a FNC1 character is specified then a leading FNC1 will be encoded and all ocurrences of delimiter characters
 * while result in FNC1 codewords in the symbol.
 *
 * @author Alex Geller
 */

 const C40_SHIFT2_CHARS: vec![Vec<char>; 27] = vec!['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', ]
;
pub struct MinimalEncoder {
}

impl MinimalEncoder {

    enum Mode {

        ASCII(), C40(), TEXT(), X12(), EDF(), B256()
    }

    fn new() -> MinimalEncoder {
    }

    fn  is_extended_a_s_c_i_i( ch: char,  fnc1: i32) -> bool  {
        return ch != fnc1 && ch >= 128 && ch <= 255;
    }

    fn  is_in_c40_shift1_set( ch: char) -> bool  {
        return ch <= 31;
    }

    fn  is_in_c40_shift2_set( ch: char,  fnc1: i32) -> bool  {
        for  let c40_shift2_char: char in C40_SHIFT2_CHARS {
            if c40_shift2_char == ch {
                return true;
            }
        }
        return ch == fnc1;
    }

    fn  is_in_text_shift1_set( ch: char) -> bool  {
        return ::is_in_c40_shift1_set(ch);
    }

    fn  is_in_text_shift2_set( ch: char,  fnc1: i32) -> bool  {
        return ::is_in_c40_shift2_set(ch, fnc1);
    }

    pub fn  encode_high_level( msg: &String) -> String  {
        return ::encode_high_level(&msg, null, -1, SymbolShapeHint::FORCE_NONE);
    }

    pub fn  encode_high_level( msg: &String,  priority_charset: &Charset,  fnc1: i32,  shape: &SymbolShapeHint) -> String  {
         let macro_id: i32 = 0;
        if msg.starts_with(HighLevelEncoder::MACRO_05_HEADER) && msg.ends_with(HighLevelEncoder::MACRO_TRAILER) {
            macro_id = 5;
            msg = msg.substring(&HighLevelEncoder::MACRO_05_HEADER::length(), msg.length() - 2);
        } else if msg.starts_with(HighLevelEncoder::MACRO_06_HEADER) && msg.ends_with(HighLevelEncoder::MACRO_TRAILER) {
            macro_id = 6;
            msg = msg.substring(&HighLevelEncoder::MACRO_06_HEADER::length(), msg.length() - 2);
        }
        return String::new(&::encode(&msg, &priority_charset, fnc1, shape, macro_id), StandardCharsets::ISO_8859_1);
    }

    fn  encode( input: &String,  priority_charset: &Charset,  fnc1: i32,  shape: &SymbolShapeHint,  macro_id: i32) -> Vec<i8>  {
        return ::encode_minimally(Input::new(&input, &priority_charset, fnc1, shape, macro_id)).get_bytes();
    }

    fn  add_edge( edges: &Vec<Vec<Edge>>,  edge: &Edge)   {
         let vertex_index: i32 = edge.fromPosition + edge.characterLength;
        if edges[vertex_index][edge.get_end_mode().ordinal()] == null || edges[vertex_index][edge.get_end_mode().ordinal()].cachedTotalSize > edge.cachedTotalSize {
            edges[vertex_index][edge.get_end_mode().ordinal()] = edge;
        }
    }

    fn  get_number_of_c40_words( input: &Input,  from: i32,  c40: bool,  character_length: &Vec<i32>) -> i32  {
         let thirds_count: i32 = 0;
         {
             let mut i: i32 = from;
            while i < input.length() {
                {
                    if input.is_e_c_i(i) {
                        character_length[0] = 0;
                        return 0;
                    }
                     let ci: char = input.char_at(i);
                    if c40 && HighLevelEncoder::is_native_c40(ci) || !c40 && HighLevelEncoder::is_native_text(ci) {
                        thirds_count += 1;
                    } else if !::is_extended_a_s_c_i_i(ci, &input.get_f_n_c1_character()) {
                        thirds_count += 2;
                    } else {
                         let ascii_value: i32 = ci & 0xff;
                        if ascii_value >= 128 && (c40 && HighLevelEncoder::is_native_c40((ascii_value - 128) as char) || !c40 && HighLevelEncoder::is_native_text((ascii_value - 128) as char)) {
                            thirds_count += 3;
                        } else {
                            thirds_count += 4;
                        }
                    }
                    if thirds_count % 3 == 0 || ((thirds_count - 2) % 3 == 0 && i + 1 == input.length()) {
                        character_length[0] = i - from + 1;
                        return Math::ceil((thirds_count as f64) / 3.0) as i32;
                    }
                }
                i += 1;
             }
         }

        character_length[0] = 0;
        return 0;
    }

    fn  add_edges( input: &Input,  edges: &Vec<Vec<Edge>>,  from: i32,  previous: &Edge)   {
        if input.is_e_c_i(from) {
            ::add_edge(edges, Edge::new(input, Mode::ASCII, from, 1, previous));
            return;
        }
         let ch: char = input.char_at(from);
        if previous == null || previous.get_end_mode() != Mode::EDF {
            if HighLevelEncoder::is_digit(ch) && input.have_n_characters(from, 2) && HighLevelEncoder::is_digit(&input.char_at(from + 1)) {
                ::add_edge(edges, Edge::new(input, Mode::ASCII, from, 2, previous));
            } else {
                ::add_edge(edges, Edge::new(input, Mode::ASCII, from, 1, previous));
            }
             let modes: vec![Vec<Mode>; 2] = vec![Mode::C40, Mode::TEXT, ]
            ;
            for  let mode: Mode in modes {
                 let character_length: [i32; 1] = [0; 1];
                if ::get_number_of_c40_words(input, from, mode == Mode::C40, &character_length) > 0 {
                    ::add_edge(edges, Edge::new(input, mode, from, character_length[0], previous));
                }
            }
            if input.have_n_characters(from, 3) && HighLevelEncoder::is_native_x12(&input.char_at(from)) && HighLevelEncoder::is_native_x12(&input.char_at(from + 1)) && HighLevelEncoder::is_native_x12(&input.char_at(from + 2)) {
                ::add_edge(edges, Edge::new(input, Mode::X12, from, 3, previous));
            }
            ::add_edge(edges, Edge::new(input, Mode::B256, from, 1, previous));
        }
        //unless it is 2 characters away from the end of the input.
         let mut i: i32;
         {
            i = 0;
            while i < 3 {
                {
                     let pos: i32 = from + i;
                    if input.have_n_characters(pos, 1) && HighLevelEncoder::is_native_e_d_i_f_a_c_t(&input.char_at(pos)) {
                        ::add_edge(edges, Edge::new(input, Mode::EDF, from, i + 1, previous));
                    } else {
                        break;
                    }
                }
                i += 1;
             }
         }

        if i == 3 && input.have_n_characters(from, 4) && HighLevelEncoder::is_native_e_d_i_f_a_c_t(&input.char_at(from + 3)) {
            ::add_edge(edges, Edge::new(input, Mode::EDF, from, 4, previous));
        }
    }

    fn  encode_minimally( input: &Input) -> Result  {
         let input_length: i32 = input.length();
        // Array that represents vertices. There is a vertex for every character and mode.
        // The last dimension in the array below encodes the 6 modes ASCII, C40, TEXT, X12, EDF and B256
         let mut edges: [[Option<Edge>; 6]; input_length + 1] = [[None; 6]; input_length + 1];
        ::add_edges(input, edges, 0, null);
         {
             let mut i: i32 = 1;
            while i <= input_length {
                {
                     {
                         let mut j: i32 = 0;
                        while j < 6 {
                            {
                                if edges[i][j] != null && i < input_length {
                                    ::add_edges(input, edges, i, edges[i][j]);
                                }
                            }
                            j += 1;
                         }
                     }

                    //optimize memory by removing edges that have been passed.
                     {
                         let mut j: i32 = 0;
                        while j < 6 {
                            {
                                edges[i - 1][j] = null;
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

         let minimal_j: i32 = -1;
         let minimal_size: i32 = Integer::MAX_VALUE;
         {
             let mut j: i32 = 0;
            while j < 6 {
                {
                    if edges[input_length][j] != null {
                         let edge: Edge = edges[input_length][j];
                        //C40, TEXT and X12 need an
                         let size: i32 =  if j >= 1 && j <= 3 { edge.cachedTotalSize + 1 } else { edge.cachedTotalSize };
                        // extra unlatch at the end
                        if size < minimal_size {
                            minimal_size = size;
                            minimal_j = j;
                        }
                    }
                }
                j += 1;
             }
         }

        if minimal_j < 0 {
            throw RuntimeException::new(format!("Internal error: failed to encode \"{}\"", input));
        }
        return Result::new(edges[input_length][minimal_j]);
    }


     let all_codeword_capacities: vec![Vec<i32>; 28] = vec![3, 5, 8, 10, 12, 16, 18, 22, 30, 32, 36, 44, 49, 62, 86, 114, 144, 174, 204, 280, 368, 456, 576, 696, 816, 1050, 1304, 1558, ]
    ;

     let square_codeword_capacities: vec![Vec<i32>; 24] = vec![3, 5, 8, 12, 18, 22, 30, 36, 44, 62, 86, 114, 144, 174, 204, 280, 368, 456, 576, 696, 816, 1050, 1304, 1558, ]
    ;

     let rectangular_codeword_capacities: vec![Vec<i32>; 6] = vec![5, 10, 16, 33, 32, 49, ]
    ;
    struct Edge {

         let input: Input;

        //the mode at the start of this edge.
         let mode: Mode;

         let from_position: i32;

         let character_length: i32;

         let previous: Edge;

         let cached_total_size: i32;
    }
    
    impl Edge {

        fn new( input: &Input,  mode: &Mode,  from_position: i32,  character_length: i32,  previous: &Edge) -> Edge {
            let .input = input;
            let .mode = mode;
            let .fromPosition = from_position;
            let .characterLength = character_length;
            let .previous = previous;
            assert!( from_position + character_length <= input.length());
             let mut size: i32 =  if previous != null { previous.cachedTotalSize } else { 0 };
             let previous_mode: Mode = self.get_previous_mode();
            /*
      * Switching modes
      * ASCII -> C40: latch 230
      * ASCII -> TEXT: latch 239
      * ASCII -> X12: latch 238
      * ASCII -> EDF: latch 240
      * ASCII -> B256: latch 231
      * C40 -> ASCII: word(c1,c2,c3), 254
      * TEXT -> ASCII: word(c1,c2,c3), 254
      * X12 -> ASCII: word(c1,c2,c3), 254
      * EDIFACT -> ASCII: Unlatch character,0,0,0 or c1,Unlatch character,0,0 or c1,c2,Unlatch character,0 or 
      * c1,c2,c3,Unlatch character
      * B256 -> ASCII: without latch after n bytes
      */
            match mode {
                  ASCII => 
                     {
                        size += 1;
                        if input.is_e_c_i(from_position) || ::is_extended_a_s_c_i_i(&input.char_at(from_position), &input.get_f_n_c1_character()) {
                            size += 1;
                        }
                        if previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12 {
                            // unlatch 254 to ASCII
                            size += 1;
                        }
                        break;
                    }
                  B256 => 
                     {
                        size += 1;
                        if previous_mode != Mode::B256 {
                            //byte count
                            size += 1;
                        } else if self.get_b256_size() == 250 {
                            //extra byte count
                            size += 1;
                        }
                        if previous_mode == Mode::ASCII {
                            //latch to B256
                            size += 1;
                        } else if previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12 {
                            //unlatch to ASCII, latch to B256
                            size += 2;
                        }
                        break;
                    }
                  C40 => 
                     {
                    }
                  TEXT => 
                     {
                    }
                  X12 => 
                     {
                        if mode == Mode::X12 {
                            size += 2;
                        } else {
                             let char_len: [i32; 1] = [0; 1];
                            size += ::get_number_of_c40_words(input, from_position, mode == Mode::C40, &char_len) * 2;
                        }
                        if previous_mode == Mode::ASCII || previous_mode == Mode::B256 {
                            //additional byte for latch from ASCII to this mode
                            size += 1;
                        } else if previous_mode != mode && (previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12) {
                            //unlatch 254 to ASCII followed by latch to this mode
                            size += 2;
                        }
                        break;
                    }
                  EDF => 
                     {
                        size += 3;
                        if previous_mode == Mode::ASCII || previous_mode == Mode::B256 {
                            //additional byte for latch from ASCII to this mode
                            size += 1;
                        } else if previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12 {
                            //unlatch 254 to ASCII followed by latch to this mode
                            size += 2;
                        }
                        break;
                    }
            }
            cached_total_size = size;
        }

        // does not count beyond 250
        fn  get_b256_size(&self) -> i32  {
             let mut cnt: i32 = 0;
             let mut current: Edge = self;
            while current != null && current.mode == Mode::B256 && cnt <= 250 {
                cnt += 1;
                current = current.previous;
            }
            return cnt;
        }

        fn  get_previous_start_mode(&self) -> Mode  {
            return  if self.previous == null { Mode::ASCII } else { self.previous.mode };
        }

        fn  get_previous_mode(&self) -> Mode  {
            return  if self.previous == null { Mode::ASCII } else { self.previous.get_end_mode() };
        }

        /** Returns Mode.ASCII in case that:
     *  - Mode is EDIFACT and characterLength is less than 4 or the remaining characters can be encoded in at most 2
     *    ASCII bytes.
     *  - Mode is C40, TEXT or X12 and the remaining characters can be encoded in at most 1 ASCII byte.
     *  Returns mode in all other cases.
     * */
        fn  get_end_mode(&self) -> Mode  {
            if self.mode == Mode::EDF {
                if self.character_length < 4 {
                    return Mode::ASCII;
                }
                // see 5.2.8.2 EDIFACT encodation Rules
                 let last_a_s_c_i_i: i32 = self.get_last_a_s_c_i_i();
                if last_a_s_c_i_i > 0 && self.get_codewords_remaining(self.cached_total_size + last_a_s_c_i_i) <= 2 - last_a_s_c_i_i {
                    return Mode::ASCII;
                }
            }
            if self.mode == Mode::C40 || self.mode == Mode::TEXT || self.mode == Mode::X12 {
                // see 5.2.5.2 C40 encodation rules and 5.2.7.2 ANSI X12 encodation rules
                if self.from_position + self.character_length >= self.input.length() && self.get_codewords_remaining(self.cached_total_size) == 0 {
                    return Mode::ASCII;
                }
                 let last_a_s_c_i_i: i32 = self.get_last_a_s_c_i_i();
                if last_a_s_c_i_i == 1 && self.get_codewords_remaining(self.cached_total_size + 1) == 0 {
                    return Mode::ASCII;
                }
            }
            return self.mode;
        }

        fn  get_mode(&self) -> Mode  {
            return self.mode;
        }

        /** Peeks ahead and returns 1 if the postfix consists of exactly two digits, 2 if the postfix consists of exactly
     *  two consecutive digits and a non extended character or of 4 digits. 
     *  Returns 0 in any other case
     **/
        fn  get_last_a_s_c_i_i(&self) -> i32  {
             let length: i32 = self.input.length();
             let from: i32 = self.from_position + self.character_length;
            if length - from > 4 || from >= length {
                return 0;
            }
            if length - from == 1 {
                if ::is_extended_a_s_c_i_i(&self.input.char_at(from), &self.input.get_f_n_c1_character()) {
                    return 0;
                }
                return 1;
            }
            if length - from == 2 {
                if ::is_extended_a_s_c_i_i(&self.input.char_at(from), &self.input.get_f_n_c1_character()) || ::is_extended_a_s_c_i_i(&self.input.char_at(from + 1), &self.input.get_f_n_c1_character()) {
                    return 0;
                }
                if HighLevelEncoder::is_digit(&self.input.char_at(from)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) {
                    return 1;
                }
                return 2;
            }
            if length - from == 3 {
                if HighLevelEncoder::is_digit(&self.input.char_at(from)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) && !::is_extended_a_s_c_i_i(&self.input.char_at(from + 2), &self.input.get_f_n_c1_character()) {
                    return 2;
                }
                if HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 2)) && !::is_extended_a_s_c_i_i(&self.input.char_at(from), &self.input.get_f_n_c1_character()) {
                    return 2;
                }
                return 0;
            }
            if HighLevelEncoder::is_digit(&self.input.char_at(from)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 2)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 3)) {
                return 2;
            }
            return 0;
        }

        /** Returns the capacity in codewords of the smallest symbol that has enough capacity to fit the given minimal
     * number of codewords.
     **/
        fn  get_min_symbol_size(&self,  minimum: i32) -> i32  {
            match self.input.get_shape_hint() {
                  FORCE_SQUARE => 
                     {
                        for  let capacity: i32 in square_codeword_capacities {
                            if capacity >= minimum {
                                return capacity;
                            }
                        }
                        break;
                    }
                  FORCE_RECTANGLE => 
                     {
                        for  let capacity: i32 in rectangular_codeword_capacities {
                            if capacity >= minimum {
                                return capacity;
                            }
                        }
                        break;
                    }
            }
            for  let capacity: i32 in all_codeword_capacities {
                if capacity >= minimum {
                    return capacity;
                }
            }
            return all_codeword_capacities[all_codeword_capacities.len() - 1];
        }

        /** Returns the remaining capacity in codewords of the smallest symbol that has enough capacity to fit the given
     * minimal number of codewords.
     **/
        fn  get_codewords_remaining(&self,  minimum: i32) -> i32  {
            return self.get_min_symbol_size(minimum) - minimum;
        }

        fn  get_bytes( c: i32) -> Vec<i8>  {
             let mut result: [i8; 1] = [0; 1];
            result[0] = c as i8;
            return result;
        }

        fn  get_bytes( c1: i32,  c2: i32) -> Vec<i8>  {
             let mut result: [i8; 2] = [0; 2];
            result[0] = c1 as i8;
            result[1] = c2 as i8;
            return result;
        }

        fn  set_c40_word( bytes: &Vec<i8>,  offset: i32,  c1: i32,  c2: i32,  c3: i32)   {
             let val16: i32 = (1600 * (c1 & 0xff)) + (40 * (c2 & 0xff)) + (c3 & 0xff) + 1;
            bytes[offset] = (val16 / 256) as i8;
            bytes[offset + 1] = (val16 % 256) as i8;
        }

        fn  get_x12_value( c: char) -> i32  {
            return  if c == 13 { 0 } else {  if c == 42 { 1 } else {  if c == 62 { 2 } else {  if c == 32 { 3 } else {  if c >= 48 && c <= 57 { c - 44 } else {  if c >= 65 && c <= 90 { c - 51 } else { c } } } } } };
        }

        fn  get_x12_words(&self) -> Vec<i8>  {
            assert!( self.character_length % 3 == 0);
             let result: [i8; self.character_length / 3 * 2] = [0; self.character_length / 3 * 2];
             {
                 let mut i: i32 = 0;
                while i < result.len() {
                    {
                        ::set_c40_word(&result, i, &::get_x12_value(&self.input.char_at(self.from_position + i / 2 * 3)), &::get_x12_value(&self.input.char_at(self.from_position + i / 2 * 3 + 1)), &::get_x12_value(&self.input.char_at(self.from_position + i / 2 * 3 + 2)));
                    }
                    i += 2;
                 }
             }

            return result;
        }

        fn  get_shift_value( c: char,  c40: bool,  fnc1: i32) -> i32  {
            return  if (c40 && ::is_in_c40_shift1_set(c) || !c40 && ::is_in_text_shift1_set(c)) { 0 } else {  if (c40 && ::is_in_c40_shift2_set(c, fnc1) || !c40 && ::is_in_text_shift2_set(c, fnc1)) { 1 } else { 2 } };
        }

        fn  get_c40_value( c40: bool,  set_index: i32,  c: char,  fnc1: i32) -> i32  {
            if c == fnc1 {
                assert!( set_index == 2);
                return 27;
            }
            if c40 {
                return  if c <= 31 { c } else {  if c == 32 { 3 } else {  if c <= 47 { c - 33 } else {  if c <= 57 { c - 44 } else {  if c <= 64 { c - 43 } else {  if c <= 90 { c - 51 } else {  if c <= 95 { c - 69 } else {  if c <= 127 { c - 96 } else { c } } } } } } } };
            } else {
                return  if c == 0 { 0 } else {  if //is this a bug in the spec?
                set_index == 0 && c <= 3 { //is this a bug in the spec?
                c - 1 } else {  if set_index == 1 && c <= 31 { c } else {  if c == 32 { 3 } else {  if c >= 33 && c <= 47 { c - 33 } else {  if c >= 48 && c <= 57 { c - 44 } else {  if c >= 58 && c <= 64 { c - 43 } else {  if c >= 65 && c <= 90 { c - 64 } else {  if c >= 91 && c <= 95 { c - 69 } else {  if c == 96 { 0 } else {  if c >= 97 && c <= 122 { c - 83 } else {  if c >= 123 && c <= 127 { c - 96 } else { c } } } } } } } } } } } };
            }
        }

        fn  get_c40_words(&self,  c40: bool,  fnc1: i32) -> Vec<i8>  {
             let c40_values: List<Byte> = ArrayList<>::new();
             {
                 let mut i: i32 = 0;
                while i < self.character_length {
                    {
                         let ci: char = self.input.char_at(self.from_position + i);
                        if c40 && HighLevelEncoder::is_native_c40(ci) || !c40 && HighLevelEncoder::is_native_text(ci) {
                            c40_values.add(::get_c40_value(c40, 0, ci, fnc1) as i8);
                        } else if !::is_extended_a_s_c_i_i(ci, fnc1) {
                             let shift_value: i32 = ::get_shift_value(ci, c40, fnc1);
                            //Shift[123]
                            c40_values.add(shift_value as i8);
                            c40_values.add(::get_c40_value(c40, shift_value, ci, fnc1) as i8);
                        } else {
                             let ascii_value: char = ((ci & 0xff) - 128) as char;
                            if c40 && HighLevelEncoder::is_native_c40(ascii_value) || !c40 && HighLevelEncoder::is_native_text(ascii_value) {
                                //Shift 2
                                c40_values.add(1 as i8);
                                //Upper Shift
                                c40_values.add(30 as i8);
                                c40_values.add(::get_c40_value(c40, 0, ascii_value, fnc1) as i8);
                            } else {
                                //Shift 2
                                c40_values.add(1 as i8);
                                //Upper Shift
                                c40_values.add(30 as i8);
                                 let shift_value: i32 = ::get_shift_value(ascii_value, c40, fnc1);
                                // Shift[123]
                                c40_values.add(shift_value as i8);
                                c40_values.add(::get_c40_value(c40, shift_value, ascii_value, fnc1) as i8);
                            }
                        }
                    }
                    i += 1;
                 }
             }

            if (c40_values.size() % 3) != 0 {
                assert!( (c40_values.size() - 2) % 3 == 0 && self.from_position + self.character_length == self.input.length());
                // pad with 0 (Shift 1)
                c40_values.add(0 as i8);
            }
             let result: [i8; c40_values.size() / 3 * 2] = [0; c40_values.size() / 3 * 2];
             let byte_index: i32 = 0;
             {
                 let mut i: i32 = 0;
                while i < c40_values.size() {
                    {
                        ::set_c40_word(&result, byte_index, c40_values.get(i) & 0xff, c40_values.get(i + 1) & 0xff, c40_values.get(i + 2) & 0xff);
                        byte_index += 2;
                    }
                    i += 3;
                 }
             }

            return result;
        }

        fn  get_e_d_f_bytes(&self) -> Vec<i8>  {
             let number_of_thirds: i32 = Math::ceil(self.character_length / 4.0) as i32;
             let mut result: [i8; number_of_thirds * 3] = [0; number_of_thirds * 3];
             let mut pos: i32 = self.from_position;
             let end_pos: i32 = Math::min(self.from_position + self.character_length - 1, self.input.length() - 1);
             {
                 let mut i: i32 = 0;
                while i < number_of_thirds {
                    {
                         let edf_values: [i32; 4] = [0; 4];
                         {
                             let mut j: i32 = 0;
                            while j < 4 {
                                {
                                    if pos <= end_pos {
                                        edf_values[j] = self.input.char_at(pos += 1 !!!check!!! post increment) & 0x3f;
                                    } else {
                                        edf_values[j] =  if pos == end_pos + 1 { 0x1f } else { 0 };
                                    }
                                }
                                j += 1;
                             }
                         }

                         let mut val24: i32 = edf_values[0] << 18;
                        val24 |= edf_values[1] << 12;
                        val24 |= edf_values[2] << 6;
                        val24 |= edf_values[3];
                        result[i] = ((val24 >> 16) & 0xff) as i8;
                        result[i + 1] = ((val24 >> 8) & 0xff) as i8;
                        result[i + 2] = (val24 & 0xff) as i8;
                    }
                    i += 3;
                 }
             }

            return result;
        }

        fn  get_latch_bytes(&self) -> Vec<i8>  {
            match self.get_previous_mode() {
                  ASCII => 
                     {
                    }
                  //after B256 ends (via length) we are back to ASCII
                B256 => 
                     {
                        match self.mode {
                              B256 => 
                                 {
                                    return ::get_bytes(231);
                                }
                              C40 => 
                                 {
                                    return ::get_bytes(230);
                                }
                              TEXT => 
                                 {
                                    return ::get_bytes(239);
                                }
                              X12 => 
                                 {
                                    return ::get_bytes(238);
                                }
                              EDF => 
                                 {
                                    return ::get_bytes(240);
                                }
                        }
                        break;
                    }
                  C40 => 
                     {
                    }
                  TEXT => 
                     {
                    }
                  X12 => 
                     {
                        if self.mode != self.get_previous_mode() {
                            match self.mode {
                                  ASCII => 
                                     {
                                        return ::get_bytes(254);
                                    }
                                  B256 => 
                                     {
                                        return ::get_bytes(254, 231);
                                    }
                                  C40 => 
                                     {
                                        return ::get_bytes(254, 230);
                                    }
                                  TEXT => 
                                     {
                                        return ::get_bytes(254, 239);
                                    }
                                  X12 => 
                                     {
                                        return ::get_bytes(254, 238);
                                    }
                                  EDF => 
                                     {
                                        return ::get_bytes(254, 240);
                                    }
                            }
                        }
                        break;
                    }
                  EDF => 
                     {
                        //The rightmost EDIFACT edge always contains an unlatch character
                        assert!( self.mode == Mode::EDF);
                        break;
                    }
            }
            return : [i8; 0] = [0; 0];
        }

        // Important: The function does not return the length bytes (one or two) in case of B256 encoding
        fn  get_data_bytes(&self) -> Vec<i8>  {
            match self.mode {
                  ASCII => 
                     {
                        if self.input.is_e_c_i(self.from_position) {
                            return ::get_bytes(241, self.input.get_e_c_i_value(self.from_position) + 1);
                        } else if ::is_extended_a_s_c_i_i(&self.input.char_at(self.from_position), &self.input.get_f_n_c1_character()) {
                            return ::get_bytes(235, self.input.char_at(self.from_position) - 127);
                        } else if self.character_length == 2 {
                            return ::get_bytes((self.input.char_at(self.from_position) - '0') * 10 + self.input.char_at(self.from_position + 1) - '0' + 130);
                        } else if self.input.is_f_n_c1(self.from_position) {
                            return ::get_bytes(232);
                        } else {
                            return ::get_bytes(self.input.char_at(self.from_position) + 1);
                        }
                    }
                  B256 => 
                     {
                        return ::get_bytes(&self.input.char_at(self.from_position));
                    }
                  C40 => 
                     {
                        return self.get_c40_words(true, &self.input.get_f_n_c1_character());
                    }
                  TEXT => 
                     {
                        return self.get_c40_words(false, &self.input.get_f_n_c1_character());
                    }
                  X12 => 
                     {
                        return self.get_x12_words();
                    }
                  EDF => 
                     {
                        return self.get_e_d_f_bytes();
                    }
            }
            assert!( false);
            return : [i8; 0] = [0; 0];
        }
    }


    struct Result {

         let mut bytes: Vec<i8>;
    }
    
    impl Result {

        fn new( solution: &Edge) -> Result {
             let input: Input = solution.input;
             let mut size: i32 = 0;
             let bytes_a_l: List<Byte> = ArrayList<>::new();
             let randomize_postfix_length: List<Integer> = ArrayList<>::new();
             let randomize_lengths: List<Integer> = ArrayList<>::new();
            if (solution.mode == Mode::C40 || solution.mode == Mode::TEXT || solution.mode == Mode::X12) && solution.get_end_mode() != Mode::ASCII {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(254), &bytes_a_l);
            }
             let mut current: Edge = solution;
            while current != null {
                size += ::prepend(&current.get_data_bytes(), &bytes_a_l);
                if current.previous == null || current.get_previous_start_mode() != current.get_mode() {
                    if current.get_mode() == Mode::B256 {
                        if size <= 249 {
                            bytes_a_l.add(0, size as i8);
                            size += 1;
                        } else {
                            bytes_a_l.add(0, (size % 250) as i8);
                            bytes_a_l.add(0, (size / 250 + 249) as i8);
                            size += 2;
                        }
                        randomize_postfix_length.add(&bytes_a_l.size());
                        randomize_lengths.add(size);
                    }
                    ::prepend(&current.get_latch_bytes(), &bytes_a_l);
                    size = 0;
                }
                current = current.previous;
            }
            if input.get_macro_id() == 5 {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(236), &bytes_a_l);
            } else if input.get_macro_id() == 6 {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(237), &bytes_a_l);
            }
            if input.get_f_n_c1_character() > 0 {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(232), &bytes_a_l);
            }
             {
                 let mut i: i32 = 0;
                while i < randomize_postfix_length.size() {
                    {
                        ::apply_random_pattern(&bytes_a_l, bytes_a_l.size() - randomize_postfix_length.get(i), &randomize_lengths.get(i));
                    }
                    i += 1;
                 }
             }

            //add padding
             let capacity: i32 = solution.get_min_symbol_size(&bytes_a_l.size());
            if bytes_a_l.size() < capacity {
                bytes_a_l.add(129 as i8);
            }
            while bytes_a_l.size() < capacity {
                bytes_a_l.add(::randomize253_state(bytes_a_l.size() + 1) as i8);
            }
            bytes = : [i8; bytes_a_l.size()] = [0; bytes_a_l.size()];
             {
                 let mut i: i32 = 0;
                while i < bytes.len() {
                    {
                        bytes[i] = bytes_a_l.get(i);
                    }
                    i += 1;
                 }
             }

        }

        fn  prepend( bytes: &Vec<i8>,  into: &List<Byte>) -> i32  {
             {
                 let mut i: i32 = bytes.len() - 1;
                while i >= 0 {
                    {
                        into.add(0, bytes[i]);
                    }
                    i -= 1;
                 }
             }

            return bytes.len();
        }

        fn  randomize253_state( codeword_position: i32) -> i32  {
             let pseudo_random: i32 = ((149 * codeword_position) % 253) + 1;
             let temp_variable: i32 = 129 + pseudo_random;
            return  if temp_variable <= 254 { temp_variable } else { temp_variable - 254 };
        }

        fn  apply_random_pattern( bytes_a_l: &List<Byte>,  start_position: i32,  length: i32)   {
             {
                 let mut i: i32 = 0;
                while i < length {
                    {
                        //See "B.1 253-state algorithm
                         const Pad_codeword_position: i32 = start_position + i;
                         const Pad_codeword_value: i32 = bytes_a_l.get(Pad_codeword_position) & 0xff;
                         let pseudo_random_number: i32 = ((149 * (Pad_codeword_position + 1)) % 255) + 1;
                         let temp_variable: i32 = Pad_codeword_value + pseudo_random_number;
                        bytes_a_l.set(Pad_codeword_position, ( if temp_variable <= 255 { temp_variable } else { temp_variable - 256 }) as i8);
                    }
                    i += 1;
                 }
             }

        }

        pub fn  get_bytes(&self) -> Vec<i8>  {
            return self.bytes;
        }
    }


    struct Input {
        super: MinimalECIInput;

         let shape: SymbolShapeHint;

         let macro_id: i32;
    }
    
    impl Input {

        fn new( string_to_encode: &String,  priority_charset: &Charset,  fnc1: i32,  shape: &SymbolShapeHint,  macro_id: i32) -> Input {
            super(&string_to_encode, &priority_charset, fnc1);
            let .shape = shape;
            let .macroId = macro_id;
        }

        fn  get_macro_id(&self) -> i32  {
            return self.macro_id;
        }

        fn  get_shape_hint(&self) -> SymbolShapeHint  {
            return self.shape;
        }
    }

}

