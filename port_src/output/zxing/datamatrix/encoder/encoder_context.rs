/*
 * Copyright 2006-2007 Jeremias Maerki.
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

struct EncoderContext {

     let msg: String;

     let mut shape: SymbolShapeHint;

     let min_size: Dimension;

     let max_size: Dimension;

     let mut codewords: StringBuilder;

     let pos: i32;

     let new_encoding: i32;

     let symbol_info: SymbolInfo;

     let skip_at_end: i32;
}

impl EncoderContext {

    fn new( msg: &String) -> EncoderContext {
        //From this point on Strings are not Unicode anymore!
         let msg_binary: Vec<i8> = msg.get_bytes(StandardCharsets::ISO_8859_1);
         let sb: StringBuilder = StringBuilder::new(msg_binary.len());
         {
             let mut i: i32 = 0, let c: i32 = msg_binary.len();
            while i < c {
                {
                     let ch: char = (msg_binary[i] & 0xff) as char;
                    if ch == '?' && msg.char_at(i) != '?' {
                        throw IllegalArgumentException::new("Message contains characters outside ISO-8859-1 encoding.");
                    }
                    sb.append(ch);
                }
                i += 1;
             }
         }

        //Not Unicode here!
        let .msg = sb.to_string();
        shape = SymbolShapeHint::FORCE_NONE;
        let .codewords = StringBuilder::new(&msg.length());
        new_encoding = -1;
    }

    pub fn  set_symbol_shape(&self,  shape: &SymbolShapeHint)   {
        self.shape = shape;
    }

    pub fn  set_size_constraints(&self,  min_size: &Dimension,  max_size: &Dimension)   {
        self.minSize = min_size;
        self.maxSize = max_size;
    }

    pub fn  get_message(&self) -> String  {
        return self.msg;
    }

    pub fn  set_skip_at_end(&self,  count: i32)   {
        self.skipAtEnd = count;
    }

    pub fn  get_current_char(&self) -> char  {
        return self.msg.char_at(self.pos);
    }

    pub fn  get_current(&self) -> char  {
        return self.msg.char_at(self.pos);
    }

    pub fn  get_codewords(&self) -> StringBuilder  {
        return self.codewords;
    }

    pub fn  write_codewords(&self,  codewords: &String)   {
        self.codewords.append(&codewords);
    }

    pub fn  write_codeword(&self,  codeword: char)   {
        self.codewords.append(codeword);
    }

    pub fn  get_codeword_count(&self) -> i32  {
        return self.codewords.length();
    }

    pub fn  get_new_encoding(&self) -> i32  {
        return self.new_encoding;
    }

    pub fn  signal_encoder_change(&self,  encoding: i32)   {
        self.newEncoding = encoding;
    }

    pub fn  reset_encoder_signal(&self)   {
        self.newEncoding = -1;
    }

    pub fn  has_more_characters(&self) -> bool  {
        return self.pos < self.get_total_message_char_count();
    }

    fn  get_total_message_char_count(&self) -> i32  {
        return self.msg.length() - self.skip_at_end;
    }

    pub fn  get_remaining_characters(&self) -> i32  {
        return self.get_total_message_char_count() - self.pos;
    }

    pub fn  get_symbol_info(&self) -> SymbolInfo  {
        return self.symbol_info;
    }

    pub fn  update_symbol_info(&self)   {
        self.update_symbol_info(&self.get_codeword_count());
    }

    pub fn  update_symbol_info(&self,  len: i32)   {
        if self.symbolInfo == null || len > self.symbolInfo.get_data_capacity() {
            self.symbolInfo = SymbolInfo::lookup(len, self.shape, self.min_size, self.max_size, true);
        }
    }

    pub fn  reset_symbol_info(&self)   {
        self.symbolInfo = null;
    }
}

