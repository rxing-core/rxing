/*
 * Copyright 2013 ZXing authors
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
// package com::google::zxing::aztec::encoder;

/**
 * This produces nearly optimal encodings of text into the first-level of
 * encoding used by Aztec code.
 *
 * It uses a dynamic algorithm.  For each prefix of the string, it determines
 * a set of encodings that could lead to this prefix.  We repeatedly add a
 * character and generate a new set of optimal encodings until we have read
 * through the entire input.
 *
 * @author Frank Yellin
 * @author Rustam Abdullaev
 */

 const MODE_NAMES: vec![Vec<String>; 5] = vec!["UPPER", "LOWER", "DIGIT", "MIXED", "PUNCT", ]
;

// 5 bits
 const MODE_UPPER: i32 = 0;

// 5 bits
 const MODE_LOWER: i32 = 1;

// 4 bits
 const MODE_DIGIT: i32 = 2;

// 5 bits
 const MODE_MIXED: i32 = 3;

// 5 bits
 const MODE_PUNCT: i32 = 4;

// The Latch Table shows, for each pair of Modes, the optimal method for
// getting from one mode to another.  In the worst possible case, this can
// be up to 14 bits.  In the best possible case, we are already there!
// The high half-word of each entry gives the number of bits.
// The low half-word of each entry are the actual bits necessary to change
 const LATCH_TABLE: vec![vec![Vec<Vec<i32>>; 5]; 5] = vec![vec![0, // UPPER -> LOWER
(5 << 16) + 28, // UPPER -> DIGIT
(5 << 16) + 30, // UPPER -> MIXED
(5 << 16) + 29, // UPPER -> MIXED -> PUNCT
(10 << 16) + (29 << 5) + 30, ]
, vec![// LOWER -> DIGIT -> UPPER
(9 << 16) + (30 << 4) + 14, 0, // LOWER -> DIGIT
(5 << 16) + 30, // LOWER -> MIXED
(5 << 16) + 29, // LOWER -> MIXED -> PUNCT
(10 << 16) + (29 << 5) + 30, ]
, vec![// DIGIT -> UPPER
(4 << 16) + 14, // DIGIT -> UPPER -> LOWER
(9 << 16) + (14 << 5) + 28, 0, // DIGIT -> UPPER -> MIXED
(9 << 16) + (14 << 5) + 29, (14 << 16) + (14 << 10) + (29 << 5) + 30, ]
, vec![// MIXED -> UPPER
(5 << 16) + 29, // MIXED -> LOWER
(5 << 16) + 28, // MIXED -> UPPER -> DIGIT
(10 << 16) + (29 << 5) + 30, 0, // MIXED -> PUNCT
(5 << 16) + 30, ]
, vec![// PUNCT -> UPPER
(5 << 16) + 31, // PUNCT -> UPPER -> LOWER
(10 << 16) + (31 << 5) + 28, // PUNCT -> UPPER -> DIGIT
(10 << 16) + (31 << 5) + 30, // PUNCT -> UPPER -> MIXED
(10 << 16) + (31 << 5) + 29, 0, ]
, ]
;

// A reverse mapping from [mode][char] to the encoding for that character
// in that mode.  An entry of 0 indicates no mapping exists.
 const CHAR_MAP: [[i32; 256]; 5] = [[0; 256]; 5];

// A map showing the available shift codes.  (The shifts to BINARY are not
// shown
// mode shift codes, per table
 const SHIFT_TABLE: [[i32; 6]; 6] = [[0; 6]; 6];
pub struct HighLevelEncoder {

     let text: Vec<i8>;

     let mut charset: Charset;
}

impl HighLevelEncoder {

    static {
        CHAR_MAP[MODE_UPPER][' '] = 1;
         {
             let mut c: i32 = 'A';
            while c <= 'Z' {
                {
                    CHAR_MAP[MODE_UPPER][c] = c - 'A' + 2;
                }
                c += 1;
             }
         }

        CHAR_MAP[MODE_LOWER][' '] = 1;
         {
             let mut c: i32 = 'a';
            while c <= 'z' {
                {
                    CHAR_MAP[MODE_LOWER][c] = c - 'a' + 2;
                }
                c += 1;
             }
         }

        CHAR_MAP[MODE_DIGIT][' '] = 1;
         {
             let mut c: i32 = '0';
            while c <= '9' {
                {
                    CHAR_MAP[MODE_DIGIT][c] = c - '0' + 2;
                }
                c += 1;
             }
         }

        CHAR_MAP[MODE_DIGIT][','] = 12;
        CHAR_MAP[MODE_DIGIT]['.'] = 13;
         let mixed_table: vec![Vec<i32>; 28] = vec!['\0', ' ', '\1', '\2', '\3', '\4', '\5', '\6', '\7', '\b', '\t', '\n', '\13', '\f', '\r', '\33', '\34', '\35', '\36', '\37', '@', '\\', '^', '_', '`', '|', '~', '\177', ]
        ;
         {
             let mut i: i32 = 0;
            while i < mixed_table.len() {
                {
                    CHAR_MAP[MODE_MIXED][mixed_table[i]] = i;
                }
                i += 1;
             }
         }

         let punct_table: vec![Vec<i32>; 31] = vec!['\0', '\r', '\0', '\0', '\0', '\0', '!', '\'', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '[', ']', '{', '}', ]
        ;
         {
             let mut i: i32 = 0;
            while i < punct_table.len() {
                {
                    if punct_table[i] > 0 {
                        CHAR_MAP[MODE_PUNCT][punct_table[i]] = i;
                    }
                }
                i += 1;
             }
         }

    }

    static {
        for  let table: Vec<i32> in SHIFT_TABLE {
            Arrays::fill(&table, -1);
        }
        SHIFT_TABLE[MODE_UPPER][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_LOWER][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_LOWER][MODE_UPPER] = 28;
        SHIFT_TABLE[MODE_MIXED][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_DIGIT][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_DIGIT][MODE_UPPER] = 15;
    }

    pub fn new( text: &Vec<i8>) -> HighLevelEncoder {
        let .text = text;
        let .charset = null;
    }

    pub fn new( text: &Vec<i8>,  charset: &Charset) -> HighLevelEncoder {
        let .text = text;
        let .charset = charset;
    }

    /**
   * @return text represented by this encoder encoded as a {@link BitArray}
   */
    pub fn  encode(&self) -> BitArray  {
         let initial_state: State = State::INITIAL_STATE;
        if self.charset != null {
             let eci: CharacterSetECI = CharacterSetECI::get_character_set_e_c_i(&self.charset);
            if null == eci {
                throw IllegalArgumentException::new(format!("No ECI code for character set {}", self.charset));
            }
            initial_state = initial_state.append_f_l_gn(&eci.get_value());
        }
         let mut states: Collection<State> = Collections::singleton_list(initial_state);
         {
             let mut index: i32 = 0;
            while index < self.text.len() {
                {
                     let pair_code: i32;
                     let next_char: i32 =  if index + 1 < self.text.len() { self.text[index + 1] } else { 0 };
                    match self.text[index] {
                          '\r' => 
                             {
                                pair_code =  if next_char == '\n' { 2 } else { 0 };
                                break;
                            }
                          '.' => 
                             {
                                pair_code =  if next_char == ' ' { 3 } else { 0 };
                                break;
                            }
                          ',' => 
                             {
                                pair_code =  if next_char == ' ' { 4 } else { 0 };
                                break;
                            }
                          ':' => 
                             {
                                pair_code =  if next_char == ' ' { 5 } else { 0 };
                                break;
                            }
                        _ => 
                             {
                                pair_code = 0;
                            }
                    }
                    if pair_code > 0 {
                        // We have one of the four special PUNCT pairs.  Treat them specially.
                        // Get a new set of states for the two new characters.
                        states = ::update_state_list_for_pair(&states, index, pair_code);
                        index += 1;
                    } else {
                        // Get a new set of states for the new character.
                        states = self.update_state_list_for_char(&states, index);
                    }
                }
                index += 1;
             }
         }

        // We are left with a set of states.  Find the shortest one.
         let min_state: State = Collections::min(&states, Comparator<State>::new() {

            pub fn  compare(&self,  a: &State,  b: &State) -> i32  {
                return a.get_bit_count() - b.get_bit_count();
            }
        });
        // Convert it to a bit array, and return.
        return min_state.to_bit_array(&self.text);
    }

    // We update a set of states for a new character by updating each state
    // for the new character, merging the results, and then removing the
    // non-optimal states.
    fn  update_state_list_for_char(&self,  states: &Iterable<State>,  index: i32) -> Collection<State>  {
         let result: Collection<State> = LinkedList<>::new();
        for  let state: State in states {
            self.update_state_for_char(state, index, &result);
        }
        return ::simplify_states(&result);
    }

    // Return a set of states that represent the possible ways of updating this
    // state for the next character.  The resulting set of states are added to
    // the "result" list.
    fn  update_state_for_char(&self,  state: &State,  index: i32,  result: &Collection<State>)   {
         let ch: char = (self.text[index] & 0xFF) as char;
         let char_in_current_table: bool = CHAR_MAP[state.get_mode()][ch] > 0;
         let state_no_binary: State = null;
         {
             let mut mode: i32 = 0;
            while mode <= MODE_PUNCT {
                {
                     let char_in_mode: i32 = CHAR_MAP[mode][ch];
                    if char_in_mode > 0 {
                        if state_no_binary == null {
                            // Only create stateNoBinary the first time it's required.
                            state_no_binary = state.end_binary_shift(index);
                        }
                        // Try generating the character by latching to its mode
                        if !char_in_current_table || mode == state.get_mode() || mode == MODE_DIGIT {
                            // If the character is in the current table, we don't want to latch to
                            // any other mode except possibly digit (which uses only 4 bits).  Any
                            // other latch would be equally successful *after* this character, and
                            // so wouldn't save any bits.
                             let latch_state: State = state_no_binary.latch_and_append(mode, char_in_mode);
                            result.add(latch_state);
                        }
                        // Try generating the character by switching to its mode.
                        if !char_in_current_table && SHIFT_TABLE[state.get_mode()][mode] >= 0 {
                            // It never makes sense to temporarily shift to another mode if the
                            // character exists in the current mode.  That can never save bits.
                             let shift_state: State = state_no_binary.shift_and_append(mode, char_in_mode);
                            result.add(shift_state);
                        }
                    }
                }
                mode += 1;
             }
         }

        if state.get_binary_shift_byte_count() > 0 || CHAR_MAP[state.get_mode()][ch] == 0 {
            // It's never worthwhile to go into binary shift mode if you're not already
            // in binary shift mode, and the character exists in your current mode.
            // That can never save bits over just outputting the char in the current mode.
             let binary_state: State = state.add_binary_shift_char(index);
            result.add(binary_state);
        }
    }

    fn  update_state_list_for_pair( states: &Iterable<State>,  index: i32,  pair_code: i32) -> Collection<State>  {
         let result: Collection<State> = LinkedList<>::new();
        for  let state: State in states {
            ::update_state_for_pair(state, index, pair_code, &result);
        }
        return ::simplify_states(&result);
    }

    fn  update_state_for_pair( state: &State,  index: i32,  pair_code: i32,  result: &Collection<State>)   {
         let state_no_binary: State = state.end_binary_shift(index);
        // Possibility 1.  Latch to MODE_PUNCT, and then append this code
        result.add(&state_no_binary.latch_and_append(MODE_PUNCT, pair_code));
        if state.get_mode() != MODE_PUNCT {
            // Possibility 2.  Shift to MODE_PUNCT, and then append this code.
            // Every state except MODE_PUNCT (handled above) can shift
            result.add(&state_no_binary.shift_and_append(MODE_PUNCT, pair_code));
        }
        if pair_code == 3 || pair_code == 4 {
            // both characters are in DIGITS.  Sometimes better to just add two digits
             let digit_state: State = state_no_binary.latch_and_append(MODE_DIGIT, // period or comma in DIGIT
            16 - pair_code).latch_and_append(MODE_DIGIT, // space in DIGIT
            1);
            result.add(digit_state);
        }
        if state.get_binary_shift_byte_count() > 0 {
            // It only makes sense to do the characters as binary if we're already
            // in binary mode.
             let binary_state: State = state.add_binary_shift_char(index).add_binary_shift_char(index + 1);
            result.add(binary_state);
        }
    }

    fn  simplify_states( states: &Iterable<State>) -> Collection<State>  {
         let result: Deque<State> = LinkedList<>::new();
        for  let new_state: State in states {
             let mut add: bool = true;
             {
                 let iterator: Iterator<State> = result.iterator();
                while iterator.has_next(){
                     let old_state: State = iterator.next();
                    if old_state.is_better_than_or_equal_to(new_state) {
                        add = false;
                        break;
                    }
                    if new_state.is_better_than_or_equal_to(old_state) {
                        iterator.remove();
                    }
                }
             }

            if add {
                result.add_first(new_state);
            }
        }
        return result;
    }
}

