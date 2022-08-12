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
 * State represents all information about a sequence necessary to generate the current output.
 * Note that a state is immutable.
 */

 const INITIAL_STATE: State = State::new(Token::EMPTY, HighLevelEncoder::MODE_UPPER, 0, 0);
struct State {

    // The current mode of the encoding (or the mode to which we'll return if
    // we're in Binary Shift mode.
     let mode: i32;

    // The list of tokens that we output.  If we are in Binary Shift mode, this
    // token list does *not* yet included the token for those bytes
     let token: Token;

    // If non-zero, the number of most recent bytes that should be output
    // in Binary Shift mode.
     let binary_shift_byte_count: i32;

    // The total number of bits generated (including Binary Shift).
     let bit_count: i32;

     let binary_shift_cost: i32;
}

impl State {

    fn new( token: &Token,  mode: i32,  binary_bytes: i32,  bit_count: i32) -> State {
        let .token = token;
        let .mode = mode;
        let .binaryShiftByteCount = binary_bytes;
        let .bitCount = bit_count;
        let .binaryShiftCost = ::calculate_binary_shift_cost(binary_bytes);
    }

    fn  get_mode(&self) -> i32  {
        return self.mode;
    }

    fn  get_token(&self) -> Token  {
        return self.token;
    }

    fn  get_binary_shift_byte_count(&self) -> i32  {
        return self.binary_shift_byte_count;
    }

    fn  get_bit_count(&self) -> i32  {
        return self.bit_count;
    }

    fn  append_f_l_gn(&self,  eci: i32) -> State  {
        // 0: FLG(n)
         let result: State = self.shift_and_append(HighLevelEncoder::MODE_PUNCT, 0);
         let mut token: Token = result.token;
         let bits_added: i32 = 3;
        if eci < 0 {
            // 0: FNC1
            token = token.add(0, 3);
        } else if eci > 999999 {
            throw IllegalArgumentException::new("ECI code must be between 0 and 999999");
        } else {
             let eci_digits: Vec<i8> = Integer::to_string(eci)::get_bytes(StandardCharsets::ISO_8859_1);
            // 1-6: number of ECI digits
            token = token.add(eci_digits.len(), 3);
            for  let eci_digit: i8 in eci_digits {
                token = token.add(eci_digit - '0' + 2, 4);
            }
            bits_added += eci_digits.len() * 4;
        }
        return State::new(token, self.mode, 0, self.bit_count + bits_added);
    }

    // Create a new state representing this state with a latch to a (not
    // necessary different) mode, and then a code.
    fn  latch_and_append(&self,  mode: i32,  value: i32) -> State  {
         let bit_count: i32 = self.bitCount;
         let mut token: Token = self.token;
        if mode != self.mode {
             let latch: i32 = HighLevelEncoder::LATCH_TABLE[self.mode][mode];
            token = token.add(latch & 0xFFFF, latch >> 16);
            bit_count += latch >> 16;
        }
         let latch_mode_bit_count: i32 =  if mode == HighLevelEncoder::MODE_DIGIT { 4 } else { 5 };
        token = token.add(value, latch_mode_bit_count);
        return State::new(token, mode, 0, bit_count + latch_mode_bit_count);
    }

    // Create a new state representing this state, with a temporary shift
    // to a different mode to output a single value.
    fn  shift_and_append(&self,  mode: i32,  value: i32) -> State  {
         let mut token: Token = self.token;
         let this_mode_bit_count: i32 =  if self.mode == HighLevelEncoder::MODE_DIGIT { 4 } else { 5 };
        // Shifts exist only to UPPER and PUNCT, both with tokens size 5.
        token = token.add(HighLevelEncoder::SHIFT_TABLE[self.mode][mode], this_mode_bit_count);
        token = token.add(value, 5);
        return State::new(token, self.mode, 0, self.bitCount + this_mode_bit_count + 5);
    }

    // Create a new state representing this state, but an additional character
    // output in Binary Shift mode.
    fn  add_binary_shift_char(&self,  index: i32) -> State  {
         let mut token: Token = self.token;
         let mut mode: i32 = self.mode;
         let bit_count: i32 = self.bitCount;
        if self.mode == HighLevelEncoder::MODE_PUNCT || self.mode == HighLevelEncoder::MODE_DIGIT {
             let latch: i32 = HighLevelEncoder::LATCH_TABLE[mode][HighLevelEncoder::MODE_UPPER];
            token = token.add(latch & 0xFFFF, latch >> 16);
            bit_count += latch >> 16;
            mode = HighLevelEncoder::MODE_UPPER;
        }
         let delta_bit_count: i32 =  if (self.binary_shift_byte_count == 0 || self.binary_shift_byte_count == 31) { 18 } else {  if (self.binary_shift_byte_count == 62) { 9 } else { 8 } };
         let mut result: State = State::new(token, mode, self.binary_shift_byte_count + 1, bit_count + delta_bit_count);
        if result.binaryShiftByteCount == 2047 + 31 {
            // The string is as long as it's allowed to be.  We should end it.
            result = result.end_binary_shift(index + 1);
        }
        return result;
    }

    // Create the state identical to this one, but we are no longer in
    // Binary Shift mode.
    fn  end_binary_shift(&self,  index: i32) -> State  {
        if self.binary_shift_byte_count == 0 {
            return self;
        }
         let mut token: Token = self.token;
        token = token.add_binary_shift(index - self.binary_shift_byte_count, self.binary_shift_byte_count);
        return State::new(token, self.mode, 0, self.bitCount);
    }

    // Returns true if "this" state is better (or equal) to be in than "that"
    // state under all possible circumstances.
    fn  is_better_than_or_equal_to(&self,  other: &State) -> bool  {
         let new_mode_bit_count: i32 = self.bitCount + (HighLevelEncoder::LATCH_TABLE[self.mode][other.mode] >> 16);
        if self.binaryShiftByteCount < other.binaryShiftByteCount {
            // add additional B/S encoding cost of other, if any
            new_mode_bit_count += other.binaryShiftCost - self.binaryShiftCost;
        } else if self.binaryShiftByteCount > other.binaryShiftByteCount && other.binaryShiftByteCount > 0 {
            // maximum possible additional cost (we end up exceeding the 31 byte boundary and other state can stay beneath it)
            new_mode_bit_count += 10;
        }
        return new_mode_bit_count <= other.bitCount;
    }

    fn  to_bit_array(&self,  text: &Vec<i8>) -> BitArray  {
         let symbols: List<Token> = ArrayList<>::new();
         {
             let mut token: Token = self.end_binary_shift(text.len()).token;
            while token != null {
                {
                    symbols.add(token);
                }
                token = token.get_previous();
             }
         }

         let bit_array: BitArray = BitArray::new();
        // Add each token to the result in forward order
         {
             let mut i: i32 = symbols.size() - 1;
            while i >= 0 {
                {
                    symbols.get(i).append_to(bit_array, &text);
                }
                i -= 1;
             }
         }

        return bit_array;
    }

    pub fn  to_string(&self) -> String  {
        return String::format("%s bits=%d bytes=%d", HighLevelEncoder::MODE_NAMES[self.mode], self.bit_count, self.binary_shift_byte_count);
    }

    fn  calculate_binary_shift_cost( binary_shift_byte_count: i32) -> i32  {
        if binary_shift_byte_count > 62 {
            // B/S with extended length
            return 21;
        }
        if binary_shift_byte_count > 31 {
            // two B/S
            return 20;
        }
        if binary_shift_byte_count > 0 {
            // one B/S
            return 10;
        }
        return 0;
    }
}

