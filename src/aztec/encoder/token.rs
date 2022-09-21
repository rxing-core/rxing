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

use std::rc::Rc;

use crate::common::BitArray;

use super::{SimpleToken, BinaryShiftToken};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
  Simple(SimpleToken), BinaryShift(BinaryShiftToken), Empty
}

pub struct Token {
  tokens: Vec<TokenType>,
  current_pointer: usize,
}

impl Token {
  pub fn new () -> Self {
    Self {
        tokens: vec![TokenType::Empty],
        current_pointer: 0,
    }
  }
  pub fn getPrevious(&mut self) -> &TokenType{
    self.current_pointer -= 1;
    &self.tokens[self.current_pointer]
  }
  pub fn add(&mut self, value:i32, bit_count: u32,) -> &TokenType {
    self.current_pointer+=1;
    self.tokens.push(TokenType::Simple(SimpleToken::new(value,bit_count)));
    &self.tokens[self.current_pointer]
  }
  pub fn addBinaryShift(&mut self, start: u32, byte_count: u32) -> &TokenType {
    self.current_pointer+=1;
    self.tokens.push(TokenType::BinaryShift(BinaryShiftToken::new(start,byte_count)));
    &self.tokens[self.current_pointer]
  }
}

// pub enum Token{
//   Simple(Rc<SimpleToken>),
//   BinaryShift(),
//   Empty,
// }

// pub trait TokenTrait {
//     fn getPrevious(&self)->&Token;

//     fn add(&self, value: i32, bitCount: u32) -> Token{
//       Token::Simple(Rc::new(SimpleToken::new(self, value, bitCount)))
//     }

//     fn addBinaryShift(&self, start: i32, byteCount: u32) -> &Token; //{
//                                                                   //   //int bitCount = (byteCount * 8) + (byteCount <= 31 ? 10 : byteCount <= 62 ? 20 : 21);
//                                                                   //   return new BinaryShiftToken(this, start, byteCount);
//                                                                   // }

//     fn appendTo(&self, bitArray: BitArray, text: &[u8]);
// }
