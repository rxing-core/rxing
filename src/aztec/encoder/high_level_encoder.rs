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

use std::cmp::Ordering;

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
    charset: &'static dyn encoding::Encoding,
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
        let mixedTable = [
            '\0', ' ', '\u{1}', '\u{2}', '\u{3}', '\u{4}', '\u{5}', '\u{6}', '\u{7}', '\u{8}',
            '\t', '\n', '\u{13}', '\u{f}', '\r', '\u{33}', '\u{34}', '\u{35}', '\u{36}', '\u{37}',
            '@', '\\', '^', '_', '`', '|', '~', '\u{177}',
        ];
        let mut i = 0;
        while i < mixedTable.len() {
            char_map[Self::MODE_MIXED][mixedTable[i] as u8 as usize] = i as u8;
            i += 1;
        }
        // for (int i = 0; i < mixedTable.length; i++) {
        //   CHAR_MAP[MODE_MIXED][mixedTable[i]] = i;
        // }
        let punctTable = [
            '\0', '\r', '\0', '\0', '\0', '\0', '!', '\'', '#', '$', '%', '&', '\'', '(', ')', '*',
            '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '[', ']', '{', '}',
        ];
        let mut i = 0;
        while i < punctTable.len() {
            if punctTable[i] as u8 > 0u8 {
                char_map[Self::MODE_PUNCT][punctTable[i] as u8 as usize] = i as u8;
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
            charset: encoding::all::UTF_8,
        }
    }

    pub fn with_charset(text: Vec<u8>, charset: &'static dyn encoding::Encoding) -> Self {
        Self { text, charset }
    }

    /**
     * @return text represented by this encoder encoded as a {@link BitArray}
     */
    pub fn encode(&self) -> Result<BitArray, Exceptions> {
        let mut initialState = State::new(Token::new(), Self::MODE_UPPER as u32, 0, 0);
        if let Some(eci) = CharacterSetECI::getCharacterSetECI(self.charset) {
            initialState = initialState.appendFLGn(CharacterSetECI::getValue(&eci))?;
        } else {
            return Err(Exceptions::IllegalArgumentException(
                "No ECI code for character set".to_owned(),
            ));
        }
        // if self.charset != null {
        //   CharacterSetECI eci = CharacterSetECI.getCharacterSetECI(charset);
        //   if (null == eci) {
        //     throw new IllegalArgumentException("No ECI code for character set " + charset);
        //   }
        //   initialState = initialState.appendFLGn(eci.getValue());
        // }
        let mut states = vec![initialState];
        let mut index = 0;
        while index < self.text.len() {
            // for index in 0..self.text.len() {
            // for (int index = 0; index < text.length; index++) {
            let pairCode;
            let nextChar = if index + 1 < self.text.len() {
                self.text[index + 1]
            } else {
                0
            };
            pairCode = match self.text[index] {
                b'\r' if nextChar == b'\n' => 2,
                b'.' if nextChar == b' ' => 3,
                b',' if nextChar == b' ' => 4,
                b':' if nextChar == b' ' => 5,
                _ => 0,
            };
            // switch (text[index]) {
            //   case '\r':
            //     pairCode = nextChar == '\n' ? 2 : 0;
            //     break;
            //   case '.' :
            //     pairCode = nextChar == ' ' ? 3 : 0;
            //     break;
            //   case ',' :
            //     pairCode = nextChar == ' ' ? 4 : 0;
            //     break;
            //   case ':' :
            //     pairCode = nextChar == ' ' ? 5 : 0;
            //     break;
            //   default:
            //     pairCode = 0;
            // }
            if pairCode > 0 {
                // We have one of the four special PUNCT pairs.  Treat them specially.
                // Get a new set of states for the two new characters.
                states = Self::updateStateListForPair(states, index as u32, pairCode);
                index += 1;
            } else {
                // Get a new set of states for the new character.
                states = self.updateStateListForChar(states, index as u32);
            }
            index += 1;
        }
        // We are left with a set of states.  Find the shortest one.
        let minState = states
            .into_iter()
            .min_by(|a, b| {
                let diff: i64 = a.getBitCount() as i64 - b.getBitCount() as i64;
                if diff < 0 {
                    Ordering::Less
                } else if diff == 0 {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
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
        Ok(minState.toBitArray(&self.text))
    }

    // We update a set of states for a new character by updating each state
    // for the new character, merging the results, and then removing the
    // non-optimal states.
    fn updateStateListForChar(&self, states: Vec<State>, index: u32) -> Vec<State> {
        let mut result = Vec::new();
        for state in states {
            // for (State state : states) {
            self.updateStateForChar(state, index, &mut result);
        }
        Self::simplifyStates(result)
    }

    // Return a set of states that represent the possible ways of updating this
    // state for the next character.  The resulting set of states are added to
    // the "result" list.
    fn updateStateForChar(&self, state: State, index: u32, result: &mut Vec<State>) {
        let ch = self.text[index as usize];
        let charInCurrentTable = Self::CHAR_MAP[state.getMode() as usize][ch as usize] > 0;
        let mut stateNoBinary = None;
        for mode in 0..Self::MODE_PUNCT {
            // for (int mode = 0; mode <= MODE_PUNCT; mode++) {
            let charInMode = Self::CHAR_MAP[mode as usize][ch as usize];
            if charInMode > 0 {
                if stateNoBinary.is_none() {
                    // Only create stateNoBinary the first time it's required.
                    stateNoBinary = Some(state.clone().endBinaryShift(index));
                }
                // Try generating the character by latching to its mode
                if !charInCurrentTable || mode as u32 == state.getMode() || mode == Self::MODE_DIGIT
                {
                    // If the character is in the current table, we don't want to latch to
                    // any other mode except possibly digit (which uses only 4 bits).  Any
                    // other latch would be equally successful *after* this character, and
                    // so wouldn't save any bits.
                    let latchState = stateNoBinary
                        .clone()
                        .unwrap()
                        .latchAndAppend(mode as u32, charInMode as u32);
                    result.push(latchState);
                }
                // Try generating the character by switching to its mode.
                if !charInCurrentTable && Self::SHIFT_TABLE[state.getMode() as usize][mode] >= 0 {
                    // It never makes sense to temporarily shift to another mode if the
                    // character exists in the current mode.  That can never save bits.
                    let shiftState = stateNoBinary
                        .clone()
                        .unwrap()
                        .shiftAndAppend(mode as u32, charInMode as u32);
                    result.push(shiftState);
                }
            }
        }
        if state.getBinaryShiftByteCount() > 0
            || Self::CHAR_MAP[state.getMode() as usize][ch as usize] == 0
        {
            // It's never worthwhile to go into binary shift mode if you're not already
            // in binary shift mode, and the character exists in your current mode.
            // That can never save bits over just outputting the char in the current mode.
            let binaryState = state.addBinaryShiftChar(index);
            result.push(binaryState);
        }
    }

    fn updateStateListForPair(states: Vec<State>, index: u32, pairCode: u32) -> Vec<State> {
        let mut result = Vec::new();
        for state in states {
            // for (State state : states) {
            Self::updateStateForPair(state, index, pairCode, &mut result);
        }

        Self::simplifyStates(result)
    }

    fn updateStateForPair(state: State, index: u32, pairCode: u32, result: &mut Vec<State>) {
        let stateNoBinary = state.clone().endBinaryShift(index);
        // Possibility 1.  Latch to MODE_PUNCT, and then append this code
        result.push(
            stateNoBinary
                .clone()
                .latchAndAppend(Self::MODE_PUNCT as u32, pairCode),
        );
        if state.getMode() != Self::MODE_PUNCT as u32 {
            // Possibility 2.  Shift to MODE_PUNCT, and then append this code.
            // Every state except MODE_PUNCT (handled above) can shift
            result.push(
                stateNoBinary
                    .clone()
                    .shiftAndAppend(Self::MODE_PUNCT as u32, pairCode),
            );
        }
        if pairCode == 3 || pairCode == 4 {
            // both characters are in DIGITS.  Sometimes better to just add two digits
            let digitState = stateNoBinary
                .latchAndAppend(Self::MODE_DIGIT as u32, 16 - pairCode) // period or comma in DIGIT
                .latchAndAppend(Self::MODE_DIGIT as u32, 1); // space in DIGIT
            result.push(digitState);
        }
        if state.getBinaryShiftByteCount() > 0 {
            // It only makes sense to do the characters as binary if we're already
            // in binary mode.
            let binaryState = state
                .addBinaryShiftChar(index)
                .addBinaryShiftChar(index + 1);
            result.push(binaryState);
        }
    }

    fn simplifyStates(states: Vec<State>) -> Vec<State> {
        let mut result: Vec<State> = Vec::new();
        for newState in states {
            // for (State newState : states) {
            let mut add = true;
            for i in 0..result.len() {
                // for st in result {
                // for (Iterator<State> iterator = result.iterator(); iterator.hasNext();) {
                let oldState = result.get(i).unwrap();
                if oldState.isBetterThanOrEqualTo(&newState) {
                    add = false;
                    break;
                }
                if newState.isBetterThanOrEqualTo(&oldState) {
                    result.remove(i);
                }
            }
            if add {
                result.push(newState);
            }
        }
        return result;
    }
}
