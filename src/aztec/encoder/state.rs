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

use crate::{exceptions::Exceptions, common::BitArray};

use super::{Token, TokenType};


/**
 * State represents all information about a sequence necessary to generate the current output.
 * Note that a state is immutable.
 */
pub struct State {

  // static final State INITIAL_STATE = new State(Token.EMPTY, HighLevelEncoder.MODE_UPPER, 0, 0);

  // The current mode of the encoding (or the mode to which we'll return if
  // we're in Binary Shift mode.
   mode:u32,
  // The list of tokens that we output.  If we are in Binary Shift mode, this
  // token list does *not* yet included the token for those bytes
   token:Token,
  // If non-zero, the number of most recent bytes that should be output
  // in Binary Shift mode.
   binaryShiftByteCount:u32,
  // The total number of bits generated (including Binary Shift).
   bitCount:u32,
   binaryShiftCost:u32,
}
  impl State {
  pub fn new( token:Token,  mode:u32,  binaryBytes:u32,  bitCount:u32) -> Self{
    Self{
        mode,
        token,
        binaryShiftByteCount: binaryBytes,
        bitCount,
        binaryShiftCost: Self::calculateBinaryShiftCost(binaryBytes),
    }
  }

  pub fn getMode(&self) -> u32{
    self.mode
  }

   pub fn getToken(&self) -> &Token{
    &self.token
  }

  pub fn getBinaryShiftByteCount(&self) -> u32{
    self.binaryShiftByteCount
  }

  pub fn getBitCount(&self) -> u32{
    self.bitCount
  }

  pub fn appendFLGn(&self,  eci:u32) -> Result<Self,Exceptions> {
    let result = self.shiftAndAppend(HighLevelEncoder::MODE_PUNCT, 0); // 0: FLG(n)
    let token = result.token;
    let bitsAdded = 3;
    if eci < 0 {
      token.add(0, 3); // 0: FNC1
    } else if eci > 999999 {
      return Err(Exceptions::IllegalArgumentException("ECI code must be between 0 and 999999".to_owned()));
      // throw new IllegalArgumentException("ECI code must be between 0 and 999999");
    } else {
      let eciDigits = Integer.toString(eci).getBytes(StandardCharsets.ISO_8859_1);
      token.add(eciDigits.length, 3); // 1-6: number of ECI digits
      for eciDigit in eciDigits {
      // for (byte eciDigit : eciDigits) {
        token.add(eciDigit - '0' + 2, 4);
      }
      bitsAdded += eciDigits.length * 4;
    }
    Ok(State::new(token, self.mode, 0, self.bitCount + bitsAdded))
    // return new State(token, mode, 0, bitCount + bitsAdded);
  }

  // Create a new state representing this state with a latch to a (not
  // necessary different) mode, and then a code.
  pub fn latchAndAppend(&self, mode:u32,  value:u32) -> State{
    let bitCount = self.bitCount;
    let token = self.token;
    if mode != self.mode {
      let latch = HighLevelEncoder.LATCH_TABLE[this.mode][mode];
      token.add(latch & 0xFFFF, latch >> 16);
      bitCount += latch >> 16;
    }
    let latchModeBitCount = mode == HighLevelEncoder.MODE_DIGIT ? 4 : 5;
    token.add(value, latchModeBitCount);
    
    State::new(token, mode, 0, bitCount + latchModeBitCount)
  }

  // Create a new state representing this state, with a temporary shift
  // to a different mode to output a single value.
  pub fn shiftAndAppend(&self,  mode:u32,  value:u32) -> State{
    let token = this.token;
    let thisModeBitCount = this.mode == HighLevelEncoder.MODE_DIGIT ? 4 : 5;
    // Shifts exist only to UPPER and PUNCT, both with tokens size 5.
    token = token.add(HighLevelEncoder.SHIFT_TABLE[this.mode][mode], thisModeBitCount);
    token = token.add(value, 5);
    State::new(token, this.mode, 0, this.bitCount + thisModeBitCount + 5)
  }

  // Create a new state representing this state, but an additional character
  // output in Binary Shift mode.
  pub fn addBinaryShiftChar(&self,  index:u32) -> State{
    let token = this.token;
    let mode = this.mode;
    let bitCount = this.bitCount;
    if self.mode == HighLevelEncoder.MODE_PUNCT || self.mode == HighLevelEncoder.MODE_DIGIT {
      let latch = HighLevelEncoder.LATCH_TABLE[mode][HighLevelEncoder.MODE_UPPER];
      token.add(latch & 0xFFFF, latch >> 16);
      bitCount += latch >> 16;
      mode = HighLevelEncoder.MODE_UPPER;
    }
    let deltaBitCount =
        (binaryShiftByteCount == 0 || binaryShiftByteCount == 31) ? 18 :
        (binaryShiftByteCount == 62) ? 9 : 8;
    let result =  State::new(token, mode, binaryShiftByteCount + 1, bitCount + deltaBitCount);
    if (result.binaryShiftByteCount == 2047 + 31) {
      // The string is as long as it's allowed to be.  We should end it.
      result = result.endBinaryShift(index + 1);
    }
    result
  }

  // Create the state identical to this one, but we are no longer in
  // Binary Shift mode.
  pub fn endBinaryShift(self,  index:u32) -> State{
    if self.binaryShiftByteCount == 0 {
      return self;
    }
    let token = self.token;
    self.token.addBinaryShift(index - self.binaryShiftByteCount, self.binaryShiftByteCount);
    
    State::new(token, self.mode, 0, self.bitCount)
  }

  // Returns true if "this" state is better (or equal) to be in than "that"
  // state under all possible circumstances.
  pub fn isBetterThanOrEqualTo(&self, other:&State)->bool {
    let newModeBitCount = self.bitCount + (HighLevelEncoder.LATCH_TABLE[this.mode][other.mode] >> 16);
    if self.binaryShiftByteCount < other.binaryShiftByteCount {
      // add additional B/S encoding cost of other, if any
      newModeBitCount += other.binaryShiftCost - self.binaryShiftCost;
    } else if self.binaryShiftByteCount > other.binaryShiftByteCount && other.binaryShiftByteCount > 0 {
      // maximum possible additional cost (we end up exceeding the 31 byte boundary and other state can stay beneath it)
      newModeBitCount += 10;
    }
     newModeBitCount <= other.bitCount
  }

  pub fn toBitArray(&self, text:&[u8]) -> BitArray{
    let symbols = Vec::new();
    let mut tok = self.endBinaryShift(text.len() as u32).token;
    let mut tkn = tok.getPrevious();
    while tkn != &TokenType::Empty {
    // for (Token token = endBinaryShift(text.length).token; token != null; token = token.getPrevious()) {
      symbols.push(tkn);
      tkn = tok.getPrevious();
    }
    let bitArray =  BitArray::new();
    // Add each token to the result in forward order
    for i in (0..symbols.len()-1).rev() {
    // for (int i = symbols.size() - 1; i >= 0; i--) {
      symbols.get(i).unwrap().appendTo(bitArray, text);
    }
     bitArray
  }

  // @Override
  // public String toString() {
  //   return String.format("%s bits=%d bytes=%d", HighLevelEncoder.MODE_NAMES[mode], bitCount, binaryShiftByteCount);
  // }

  fn calculateBinaryShiftCost( binaryShiftByteCount:u32) -> u32{
    if binaryShiftByteCount > 62 {
      return 21; // B/S with extended length
    }
    if binaryShiftByteCount > 31 {
      return 20; // two B/S
    }
    if binaryShiftByteCount > 0 {
      return 10; // one B/S
    }
    return 0;
  }

}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} bits={} bytes={}", HighLevelEncoder::MODE_NAMES[mode], self.bitCount, self.binaryShiftByteCount)
    }
}