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

use std::fmt;

use encoding::Encoding;

use crate::{common::BitArray, exceptions::Exceptions};

use super::{HighLevelEncoder, Token};

/**
 * State represents all information about a sequence necessary to generate the current output.
 * Note that a state is immutable.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    // static final State INITIAL_STATE = new State(Token.EMPTY, HighLevelEncoder.MODE_UPPER, 0, 0);

    // The current mode of the encoding (or the mode to which we'll return if
    // we're in Binary Shift mode.
    mode: u32,
    // The list of tokens that we output.  If we are in Binary Shift mode, this
    // token list does *not* yet included the token for those bytes
    token: Token,
    // If non-zero, the number of most recent bytes that should be output
    // in Binary Shift mode.
    binary_shift_byte_count: u32,
    // The total number of bits generated (including Binary Shift).
    bit_count: u32,
    binary_shift_cost: u32,
}
impl State {
    pub fn new(token: Token, mode: u32, binary_bytes: u32, bit_count: u32) -> Self {
        Self {
            mode,
            token,
            binary_shift_byte_count: binary_bytes,
            bit_count,
            binary_shift_cost: Self::calculate_binary_shift_cost(binary_bytes),
        }
    }

    pub fn getMode(&self) -> u32 {
        self.mode
    }

    pub fn getToken(&self) -> &Token {
        &self.token
    }

    pub fn getBinaryShiftByteCount(&self) -> u32 {
        self.binary_shift_byte_count
    }

    pub fn getBitCount(&self) -> u32 {
        self.bit_count
    }

    pub fn appendFLGn(self, eci: u32) -> Result<Self, Exceptions> {
        let bit_count = self.bit_count;
        let mode = self.mode;
        let result = self.shiftAndAppend(HighLevelEncoder::MODE_PUNCT as u32, 0); // 0: FLG(n)
        let mut token = result.token;
        let mut bits_added = 3;
        /*if eci < 0 {
            token.add(0, 3); // 0: FNC1
        } else */
        if eci > 999999 {
            return Err(Exceptions::IllegalArgumentException(Some(
                "ECI code must be between 0 and 999999".to_owned(),
            )));
            // throw new IllegalArgumentException("ECI code must be between 0 and 999999");
        } else {
            let Ok(eci_digits) = encoding::all::ISO_8859_1
                .encode(&format!("{eci}"), encoding::EncoderTrap::Strict)
                 else {
                    return Err(Exceptions::IllegalArgumentException(None))
                 };
            // let eciDigits = Integer.toString(eci).getBytes(StandardCharsets.ISO_8859_1);
            token.add(eci_digits.len() as i32, 3); // 1-6: number of ECI digits
            for eci_digit in &eci_digits {
                // for (byte eciDigit : eciDigits) {
                token.add((eci_digit - b'0' + 2) as i32, 4);
            }
            bits_added += eci_digits.len() * 4;
        }
        Ok(State::new(token, mode, 0, bit_count + bits_added as u32))
        // return new State(token, mode, 0, bitCount + bitsAdded);
    }

    // Create a new state representing this state with a latch to a (not
    // necessary different) mode, and then a code.
    pub fn latchAndAppend(self, mode: u32, value: u32) -> State {
        let mut bitCount = self.bit_count;
        let mut token = self.token;
        if mode != self.mode {
            let latch = HighLevelEncoder::LATCH_TABLE[self.mode as usize][mode as usize];
            token.add(latch as i32 & 0xFFFF, latch >> 16);
            bitCount += latch >> 16;
        }
        let latchModeBitCount = if mode == HighLevelEncoder::MODE_DIGIT as u32 {
            4
        } else {
            5
        };
        token.add(value as i32, latchModeBitCount);

        State::new(token, mode, 0, bitCount + latchModeBitCount)
    }

    // Create a new state representing this state, with a temporary shift
    // to a different mode to output a single value.
    pub fn shiftAndAppend(self, mode: u32, value: u32) -> State {
        let mut token = self.token;
        let thisModeBitCount = if self.mode == HighLevelEncoder::MODE_DIGIT as u32 {
            4
        } else {
            5
        };
        // Shifts exist only to UPPER and PUNCT, both with tokens size 5.
        token.add(
            HighLevelEncoder::SHIFT_TABLE[self.mode as usize][mode as usize],
            thisModeBitCount,
        );
        token.add(value as i32, 5);
        State::new(token, self.mode, 0, self.bit_count + thisModeBitCount + 5)
    }

    // Create a new state representing this state, but an additional character
    // output in Binary Shift mode.
    pub fn addBinaryShiftChar(self, index: u32) -> State {
        let mut token = self.token;
        let mut mode = self.mode;
        let mut bitCount = self.bit_count;
        if self.mode == HighLevelEncoder::MODE_PUNCT as u32
            || self.mode == HighLevelEncoder::MODE_DIGIT as u32
        {
            let latch = HighLevelEncoder::LATCH_TABLE[mode as usize][HighLevelEncoder::MODE_UPPER];
            token.add(latch as i32 & 0xFFFF, latch >> 16);
            bitCount += latch >> 16;
            mode = HighLevelEncoder::MODE_UPPER as u32;
        }
        let deltaBitCount =
            if self.binary_shift_byte_count == 0 || self.binary_shift_byte_count == 31 {
                18
            } else if self.binary_shift_byte_count == 62 {
                9
            } else {
                8
            };
        let mut result = State::new(
            token,
            mode,
            self.binary_shift_byte_count + 1,
            bitCount + deltaBitCount,
        );
        if result.binary_shift_byte_count == 2047 + 31 {
            // The string is as long as it's allowed to be.  We should end it.
            result = result.endBinaryShift(index + 1);
        }
        result
    }

    // Create the state identical to this one, but we are no longer in
    // Binary Shift mode.
    pub fn endBinaryShift(self, index: u32) -> State {
        if self.binary_shift_byte_count == 0 {
            return self;
        }
        let mut token = self.token;
        token.addBinaryShift(
            index - self.binary_shift_byte_count,
            self.binary_shift_byte_count,
        );

        State::new(token, self.mode, 0, self.bit_count)
    }

    // Returns true if "this" state is better (or equal) to be in than "that"
    // state under all possible circumstances.
    pub fn isBetterThanOrEqualTo(&self, other: &State) -> bool {
        let mut new_mode_bit_count = self.bit_count
            + (HighLevelEncoder::LATCH_TABLE[self.mode as usize][other.mode as usize] >> 16);
        if self.binary_shift_byte_count < other.binary_shift_byte_count {
            // add additional B/S encoding cost of other, if any
            new_mode_bit_count += other.binary_shift_cost - self.binary_shift_cost;
        } else if self.binary_shift_byte_count > other.binary_shift_byte_count
            && other.binary_shift_byte_count > 0
        {
            // maximum possible additional cost (we end up exceeding the 31 byte boundary and other state can stay beneath it)
            new_mode_bit_count += 10;
        }
        new_mode_bit_count <= other.bit_count
    }

    pub fn toBitArray(self, text: &[u8]) -> Result<BitArray, Exceptions> {
        let mut symbols = Vec::new();
        let tok = self.endBinaryShift(text.len() as u32).token;
        for tkn in tok.into_iter() {
            // for (Token token = endBinaryShift(text.length).token; token != null; token = token.getPrevious()) {
            symbols.push(tkn);
        }
        // let mut tkn = tok.getPrevious();
        // while tkn != &TokenType::Empty {
        //     // for (Token token = endBinaryShift(text.length).token; token != null; token = token.getPrevious()) {
        //     symbols.push(tkn);
        //     tkn = tok.getPrevious();
        // }
        let mut bit_array = BitArray::new();
        // Add each token to the result in forward order
        for symbol in symbols.into_iter().rev() {
            // for i in (0..symbols.len()).rev() {
            // for (int i = symbols.size() - 1; i >= 0; i--) {
            symbol.appendTo(&mut bit_array, text)?;
        }
        Ok(bit_array)
    }

    #[inline(always)]
    fn calculate_binary_shift_cost(binary_shift_byte_count: u32) -> u32 {
        if binary_shift_byte_count > 62 {
            21 // B/S with extended length
        } else if binary_shift_byte_count > 31 {
            20 // two B/S
        } else if binary_shift_byte_count > 0 {
            10 // one B/S
        } else {
            0
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} bits={} bytes={}",
            HighLevelEncoder::MODE_NAMES[self.mode as usize],
            self.bit_count,
            self.binary_shift_byte_count
        )
    }
}
