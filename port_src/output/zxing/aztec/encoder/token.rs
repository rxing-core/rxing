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


 const EMPTY: Token = SimpleToken::new(null, 0, 0);
struct Token {

     let previous: Token;
}

impl Token {

    fn new( previous: &Token) -> Token {
        let .previous = previous;
    }

    fn  get_previous(&self) -> Token  {
        return self.previous;
    }

    fn  add(&self,  value: i32,  bit_count: i32) -> Token  {
        return SimpleToken::new(self, value, bit_count);
    }

    fn  add_binary_shift(&self,  start: i32,  byte_count: i32) -> Token  {
        //int bitCount = (byteCount * 8) + (byteCount <= 31 ? 10 : byteCount <= 62 ? 20 : 21);
        return BinaryShiftToken::new(self, start, byte_count);
    }

    fn  append_to(&self,  bit_array: &BitArray,  text: &Vec<i8>)  ;
}

