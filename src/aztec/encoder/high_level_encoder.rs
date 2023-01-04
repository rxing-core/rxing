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

use crate::{
    common::{BitArray, CharacterSetECI},
    exceptions::Exceptions,
};

use super::{State, Token};

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
pub struct HighLevelEncoder {
    text: Vec<u8>,
    charset: encoding::EncodingRef,
}

impl HighLevelEncoder {
    pub const MODE_NAMES: [&'static str; 5] = ["UPPER", "LOWER", "DIGIT", "MIXED", "PUNCT"];

    pub const MODE_UPPER: usize = 0; // 5 bits
    pub const MODE_LOWER: usize = 1; // 5 bits
    pub const MODE_DIGIT: usize = 2; // 4 bits
    pub const MODE_MIXED: usize = 3; // 5 bits
    pub const MODE_PUNCT: usize = 4; // 5 bits

    // The Latch Table shows, for each pair of Modes, the optimal method for
    // getting from one mode to another.  In the worst possible case, this can
    // be up to 14 bits.  In the best possible case, we are already there!
    // The high half-word of each entry gives the number of bits.
    // The low half-word of each entry are the actual bits necessary to change
    pub const LATCH_TABLE: [[u32; 5]; 5] = [
        [
            0,
            (5 << 16) + 28,              // UPPER -> LOWER
            (5 << 16) + 30,              // UPPER -> DIGIT
            (5 << 16) + 29,              // UPPER -> MIXED
            (10 << 16) + (29 << 5) + 30, // UPPER -> MIXED -> PUNCT
        ],
        [
            (9 << 16) + (30 << 4) + 14, // LOWER -> DIGIT -> UPPER
            0,
            (5 << 16) + 30,              // LOWER -> DIGIT
            (5 << 16) + 29,              // LOWER -> MIXED
            (10 << 16) + (29 << 5) + 30, // LOWER -> MIXED -> PUNCT
        ],
        [
            (4 << 16) + 14,             // DIGIT -> UPPER
            (9 << 16) + (14 << 5) + 28, // DIGIT -> UPPER -> LOWER
            0,
            (9 << 16) + (14 << 5) + 29, // DIGIT -> UPPER -> MIXED
            (14 << 16) + (14 << 10) + (29 << 5) + 30,
        ], // DIGIT -> UPPER -> MIXED -> PUNCT
        [
            (5 << 16) + 29,              // MIXED -> UPPER
            (5 << 16) + 28,              // MIXED -> LOWER
            (10 << 16) + (29 << 5) + 30, // MIXED -> UPPER -> DIGIT
            0,
            (5 << 16) + 30, // MIXED -> PUNCT
        ],
        [
            (5 << 16) + 31,              // PUNCT -> UPPER
            (10 << 16) + (31 << 5) + 28, // PUNCT -> UPPER -> LOWER
            (10 << 16) + (31 << 5) + 30, // PUNCT -> UPPER -> DIGIT
            (10 << 16) + (31 << 5) + 29, // PUNCT -> UPPER -> MIXED
            0,
        ],
    ];

    // A reverse mapping from [mode][char] to the encoding for that character
    // in that mode.  An entry of 0 indicates no mapping exists.
    pub const CHAR_MAP: [[u8; 256]; 5] = {
        let mut char_map = [[0u8; 256]; 5];
        char_map[Self::MODE_UPPER][b' ' as usize] = 1;
        let mut c = b'A';
        while c <= b'Z' {
            char_map[Self::MODE_UPPER][c as usize] = c - b'A' + 2;
            c += 1;
        }
        // for (int c = 'A'; c <= 'Z'; c++) {
        //   char_map[Self::MODE_UPPER][c] = c - 'A' + 2;
        // }
        char_map[Self::MODE_LOWER][b' ' as usize] = 1;
        let mut c = b'a';
        while c <= b'z' {
            char_map[Self::MODE_LOWER][c as usize] = c - b'a' + 2;
            c += 1;
        }
        // for (int c = 'a'; c <= 'z'; c++) {
        //   char_map[Self::MODE_LOWER][c] = c - 'a' + 2;
        // }
        char_map[Self::MODE_DIGIT][b' ' as usize] = 1;
        let mut c = b'0';
        while c <= b'9' {
            char_map[Self::MODE_DIGIT][c as usize] = c - b'0' + 2;
            c += 1;
        }
        // for (int c = '0'; c <= '9'; c++) {
        //   char_map[Self::MODE_DIGIT][c] = c - '0' + 2;
        // }
        char_map[Self::MODE_DIGIT][b',' as usize] = 12;
        char_map[Self::MODE_DIGIT][b'.' as usize] = 13;
        let mixed_table = [
            '\0', ' ', '\u{1}', '\u{2}', '\u{3}', '\u{4}', '\u{5}', '\u{6}', '\u{7}', '\u{8}',
            '\t', '\n', '\u{000b}', '\u{000c}', '\r', '\u{001b}', '\u{001c}', '\u{001d}',
            '\u{001e}', '\u{001f}', '@', '\\', '^', '_', '`', '|', '~', '\u{007f}',
        ];
        let mut i = 0;
        while i < mixed_table.len() {
            char_map[Self::MODE_MIXED][mixed_table[i] as u8 as usize] = i as u8;
            i += 1;
        }
        // for (int i = 0; i < mixedTable.length; i++) {
        //   CHAR_MAP[MODE_MIXED][mixedTable[i]] = i;
        // }
        let punct_table = [
            b'\0', b'\r', b'\0', b'\0', b'\0', b'\0', b'!', b'\'', b'#', b'$', b'%', b'&', b'\'',
            b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?',
            b'[', b']', b'{', b'}',
        ];
        let mut i = 0;
        while i < punct_table.len() {
            if punct_table[i] > 0u8 {
                char_map[Self::MODE_PUNCT][punct_table[i] as usize] = i as u8;
            }
            i += 1;
        }
        // for (int i = 0; i < punctTable.length; i++) {
        //   if (punctTable[i] > 0) {
        //     CHAR_MAP[MODE_PUNCT][punctTable[i]] = i;
        //   }
        // }

        char_map
    };
    // private static final int[][] CHAR_MAP = new int[5][256];
    // static {
    //   CHAR_MAP[MODE_UPPER][' '] = 1;
    //   for (int c = 'A'; c <= 'Z'; c++) {
    //     CHAR_MAP[MODE_UPPER][c] = c - 'A' + 2;
    //   }
    //   CHAR_MAP[MODE_LOWER][' '] = 1;
    //   for (int c = 'a'; c <= 'z'; c++) {
    //     CHAR_MAP[MODE_LOWER][c] = c - 'a' + 2;
    //   }
    //   CHAR_MAP[MODE_DIGIT][' '] = 1;
    //   for (int c = '0'; c <= '9'; c++) {
    //     CHAR_MAP[MODE_DIGIT][c] = c - '0' + 2;
    //   }
    //   CHAR_MAP[MODE_DIGIT][','] = 12;
    //   CHAR_MAP[MODE_DIGIT]['.'] = 13;
    //   int[] mixedTable = {
    //       '\0', ' ', '\1', '\2', '\3', '\4', '\5', '\6', '\7', '\b', '\t', '\n',
    //       '\13', '\f', '\r', '\33', '\34', '\35', '\36', '\37', '@', '\\', '^',
    //       '_', '`', '|', '~', '\177'
    //   };
    //   for (int i = 0; i < mixedTable.length; i++) {
    //     CHAR_MAP[MODE_MIXED][mixedTable[i]] = i;
    //   }
    //   int[] punctTable = {
    //       '\0', '\r', '\0', '\0', '\0', '\0', '!', '\'', '#', '$', '%', '&', '\'',
    //       '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?',
    //       '[', ']', '{', '}'
    //   };
    //   for (int i = 0; i < punctTable.length; i++) {
    //     if (punctTable[i] > 0) {
    //       CHAR_MAP[MODE_PUNCT][punctTable[i]] = i;
    //     }
    //   }
    // }

    // A map showing the available shift codes.  (The shifts to BINARY are not
    // shown
    pub const SHIFT_TABLE: [[i32; 6]; 6] = {
        // mode shift codes, per table
        let mut shift_table = [[-1i32; 6]; 6];

        shift_table[Self::MODE_UPPER][Self::MODE_PUNCT] = 0;

        shift_table[Self::MODE_LOWER][Self::MODE_PUNCT] = 0;
        shift_table[Self::MODE_LOWER][Self::MODE_UPPER] = 28;

        shift_table[Self::MODE_MIXED][Self::MODE_PUNCT] = 0;

        shift_table[Self::MODE_DIGIT][Self::MODE_PUNCT] = 0;
        shift_table[Self::MODE_DIGIT][Self::MODE_UPPER] = 15;

        shift_table
    };
    // const SHIFT_TABLE : [[u32]]= new int[6][6]; // mode shift codes, per table
    // static {
    //   for (int[] table : SHIFT_TABLE) {
    //     Arrays.fill(table, -1);
    //   }
    //   SHIFT_TABLE[MODE_UPPER][MODE_PUNCT] = 0;

    //   SHIFT_TABLE[MODE_LOWER][MODE_PUNCT] = 0;
    //   SHIFT_TABLE[MODE_LOWER][MODE_UPPER] = 28;

    //   SHIFT_TABLE[MODE_MIXED][MODE_PUNCT] = 0;

    //   SHIFT_TABLE[MODE_DIGIT][MODE_PUNCT] = 0;
    //   SHIFT_TABLE[MODE_DIGIT][MODE_UPPER] = 15;
    // }

    pub fn new(text: Vec<u8>) -> Self {
        Self {
            text,
            charset: encoding::all::ISO_8859_1,
        }
    }

    pub fn with_charset(text: Vec<u8>, charset: encoding::EncodingRef) -> Self {
        Self { text, charset }
    }

    /**
     * @return text represented by this encoder encoded as a {@link BitArray}
     */
    pub fn encode(&self) -> Result<BitArray, Exceptions> {
        let mut initial_state = State::new(Token::new(), Self::MODE_UPPER as u32, 0, 0);
        if let Some(eci) = CharacterSetECI::getCharacterSetECI(self.charset) {
            if eci != CharacterSetECI::ISO8859_1 {
                //} && eci != CharacterSetECI::Cp1252 {
                initial_state = initial_state.appendFLGn(CharacterSetECI::getValue(&eci))?;
            }
        } else {
            return Err(Exceptions::IllegalArgumentException(Some(
                "No ECI code for character set".to_owned(),
            )));
        }
        // if self.charset != null {
        //   CharacterSetECI eci = CharacterSetECI.getCharacterSetECI(charset);
        //   if (null == eci) {
        //     throw new IllegalArgumentException("No ECI code for character set " + charset);
        //   }
        //   initialState = initialState.appendFLGn(eci.getValue());
        // }
        let mut states = vec![initial_state];
        let mut index = 0;
        while index < self.text.len() {
            // for index in 0..self.text.len() {
            // for (int index = 0; index < text.length; index++) {

            let next_char = if index + 1 < self.text.len() {
                self.text[index + 1]
            } else {
                0
            };
            let pair_code = match self.text[index] {
                b'\r' if next_char == b'\n' => 2,
                b'.' if next_char == b' ' => 3,
                b',' if next_char == b' ' => 4,
                b':' if next_char == b' ' => 5,
                _ => 0,
            };

            if pair_code > 0 {
                // We have one of the four special PUNCT pairs.  Treat them specially.
                // Get a new set of states for the two new characters.
                states = Self::update_state_list_for_pair(states, index as u32, pair_code);
                index += 1;
            } else {
                // Get a new set of states for the new character.
                states = self.update_state_list_for_char(states, index as u32);
            }
            index += 1;
        }

        // for state in &states {
        //     dbg!(state.clone().toBitArray(&self.text).to_string());
        // }

        // We are left with a set of states.  Find the shortest one.
        let min_state = states
            .into_iter()
            .min_by(|a, b| {
                let diff: i64 = a.getBitCount() as i64 - b.getBitCount() as i64;
                diff.cmp(&0)
                // match diff {
                //     ..0 => Ordering::Less,
                //     0 => Ordering::Equal,
                //     0.. => Ordering::Greater,
                // }
                // if diff < 0 {
                //     Ordering::Less
                // } else if diff == 0 {
                //     Ordering::Equal
                // } else {
                //     Ordering::Greater
                // }
                //  a.getBitCount() - b.getBitCount()
            })
            .unwrap();
        // let minState = Collections.min(states, new Comparator<State>() {
        //   @Override
        //   public int compare(State a, State b) {
        //     return a.getBitCount() - b.getBitCount();
        //   }
        // });
        // Convert it to a bit array, and return.
        // dbg!(min_state.clone().toBitArray(&self.text).to_string());

        Ok(min_state.toBitArray(&self.text))
    }

    // We update a set of states for a new character by updating each state
    // for the new character, merging the results, and then removing the
    // non-optimal states.
    fn update_state_list_for_char(&self, states: Vec<State>, index: u32) -> Vec<State> {
        let mut result = Vec::new();
        for state in states {
            // for (State state : states) {
            self.update_state_for_char(state, index, &mut result);
        }
        Self::simplify_states(result)
    }

    // Return a set of states that represent the possible ways of updating this
    // state for the next character.  The resulting set of states are added to
    // the "result" list.
    fn update_state_for_char(&self, state: State, index: u32, result: &mut Vec<State>) {
        let ch = self.text[index as usize];
        let char_in_current_table = Self::CHAR_MAP[state.getMode() as usize][ch as usize] > 0;
        let mut state_no_binary = None;
        for mode in 0..=Self::MODE_PUNCT {
            // for (int mode = 0; mode <= MODE_PUNCT; mode++) {
            let char_in_mode = Self::CHAR_MAP[mode][ch as usize];
            if char_in_mode > 0 {
                if state_no_binary.is_none() {
                    // Only create stateNoBinary the first time it's required.
                    state_no_binary = Some(state.clone().endBinaryShift(index));
                }
                // Try generating the character by latching to its mode
                if !char_in_current_table
                    || mode as u32 == state.getMode()
                    || mode == Self::MODE_DIGIT
                {
                    // If the character is in the current table, we don't want to latch to
                    // any other mode except possibly digit (which uses only 4 bits).  Any
                    // other latch would be equally successful *after* this character, and
                    // so wouldn't save any bits.
                    let latch_state = state_no_binary
                        .clone()
                        .unwrap()
                        .latchAndAppend(mode as u32, char_in_mode as u32);
                    result.push(latch_state);
                }
                // Try generating the character by switching to its mode.
                if !char_in_current_table && Self::SHIFT_TABLE[state.getMode() as usize][mode] >= 0
                {
                    // It never makes sense to temporarily shift to another mode if the
                    // character exists in the current mode.  That can never save bits.
                    let shift_state = state_no_binary
                        .clone()
                        .unwrap()
                        .shiftAndAppend(mode as u32, char_in_mode as u32);
                    result.push(shift_state);
                }
            }
        }
        if state.getBinaryShiftByteCount() > 0
            || Self::CHAR_MAP[state.getMode() as usize][ch as usize] == 0
        {
            // It's never worthwhile to go into binary shift mode if you're not already
            // in binary shift mode, and the character exists in your current mode.
            // That can never save bits over just outputting the char in the current mode.
            let binary_state = state.addBinaryShiftChar(index);
            result.push(binary_state);
        }
    }

    fn update_state_list_for_pair(states: Vec<State>, index: u32, pairCode: u32) -> Vec<State> {
        let mut result = Vec::new();
        for state in states {
            // for (State state : states) {
            Self::update_state_for_pair(state, index, pairCode, &mut result);
        }

        Self::simplify_states(result)
    }

    fn update_state_for_pair(state: State, index: u32, pair_code: u32, result: &mut Vec<State>) {
        let state_no_binary = state.clone().endBinaryShift(index);
        // Possibility 1.  Latch to MODE_PUNCT, and then append this code
        result.push(
            state_no_binary
                .clone()
                .latchAndAppend(Self::MODE_PUNCT as u32, pair_code),
        );
        if state.getMode() != Self::MODE_PUNCT as u32 {
            // Possibility 2.  Shift to MODE_PUNCT, and then append this code.
            // Every state except MODE_PUNCT (handled above) can shift
            result.push(
                state_no_binary
                    .clone()
                    .shiftAndAppend(Self::MODE_PUNCT as u32, pair_code),
            );
        }
        if pair_code == 3 || pair_code == 4 {
            // both characters are in DIGITS.  Sometimes better to just add two digits
            let digit_state = state_no_binary
                .latchAndAppend(Self::MODE_DIGIT as u32, 16 - pair_code) // period or comma in DIGIT
                .latchAndAppend(Self::MODE_DIGIT as u32, 1); // space in DIGIT
            result.push(digit_state);
        }
        if state.getBinaryShiftByteCount() > 0 {
            // It only makes sense to do the characters as binary if we're already
            // in binary mode.
            let binary_state = state
                .addBinaryShiftChar(index)
                .addBinaryShiftChar(index + 1);
            result.push(binary_state);
        }
    }

    fn simplify_states(states: Vec<State>) -> Vec<State> {
        let mut result: Vec<State> = Vec::new();
        for newState in states {
            // for (State newState : states) {
            let mut add = true;
            for i in 0..result.len() {
                // for st in result {
                // for (Iterator<State> iterator = result.iterator(); iterator.hasNext();) {
                if let Some(oldState) = result.get(i) {
                    if oldState.isBetterThanOrEqualTo(&newState) {
                        add = false;
                        break;
                    }
                    if newState.isBetterThanOrEqualTo(oldState) {
                        result.remove(i);
                    }
                }
            }
            if add {
                result.push(newState);
            }
        }
        result
    }
}
