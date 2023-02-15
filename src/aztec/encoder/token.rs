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

use crate::{common::BitArray, Exceptions};

use super::{BinaryShiftToken, SimpleToken};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Simple(SimpleToken),
    BinaryShift(BinaryShiftToken),
    Empty,
}

impl TokenType {
    pub fn appendTo(&self, bit_array: &mut BitArray, text: &[u8]) -> Result<(), Exceptions> {
        // let token = &self.tokens[self.current_pointer];
        match self {
            TokenType::Simple(a) => a.appendTo(bit_array, text),
            TokenType::BinaryShift(a) => a.appendTo(bit_array, text),
            TokenType::Empty => Err(Exceptions::illegalState(String::from(
                "cannot appendTo on Empty final item",
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    tokens: Vec<TokenType>,
    //current_pointer: usize,
}

impl Token {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            //current_pointer: 0,
        }
    }
    // pub fn getPrevious(&mut self) -> &TokenType {
    //     self.current_pointer -= 1;
    //     &self.tokens[self.current_pointer]
    // }
    pub fn add(&mut self, value: i32, bit_count: u32) {
        //self.current_pointer += 1;
        self.tokens
            .push(TokenType::Simple(SimpleToken::new(value, bit_count)));
        // &self.tokens[self.current_pointer]
    }
    pub fn addBinaryShift(&mut self, start: u32, byte_count: u32) {
        //self.current_pointer += 1;
        self.tokens
            .push(TokenType::BinaryShift(BinaryShiftToken::new(
                start, byte_count,
            )));
        // &self.tokens[self.current_pointer]
    }
}

pub struct TokenIter {
    src: Vec<TokenType>,
    // pos: usize,
}

impl Iterator for TokenIter {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        self.src.pop()
    }
}

impl IntoIterator for Token {
    type Item = TokenType;

    type IntoIter = TokenIter;

    fn into_iter(self) -> Self::IntoIter {
        TokenIter {
            src: self.tokens,
            // pos: self.current_pointer,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self::new()
    }
}
