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

use std::rc::Rc;

use crate::{Dimension, Exceptions};

use super::{SymbolInfo, SymbolShapeHint, SymbolInfoLookup};
use encoding::{self, EncodingRef};
use unicode_segmentation::UnicodeSegmentation;

const ISO_8859_1_ENCODER: EncodingRef = encoding::all::ISO_8859_1;

pub struct EncoderContext<'a> {
  symbol_lookup:Rc<SymbolInfoLookup<'a>>,
    msg: String,
    shape: SymbolShapeHint,
    minSize: Option<Dimension>,
    maxSize: Option<Dimension>,
    codewords: String,
    pub(super) pos: u32,
    newEncoding:Option< usize>,
    symbolInfo: Option<&'a SymbolInfo>,
    skipAtEnd: u32,
}

impl EncoderContext<'_> {
    pub fn new(msg: &str) -> Result<Self, Exceptions> {
        //From this point on Strings are not Unicode anymore!
        // let msgBinary = ISO_8859_1_ENCODER.encode(msg, encoding::EncoderTrap::Strict).expect("encode to bytes");//msg.getBytes(StandardCharsets.ISO_8859_1);
        // let sb =  String::with_capacity(msgBinary.len());
        // for (int i = 0, c = msgBinary.length; i < c; i++) {
        //   char ch = (char) (msgBinary[i] & 0xff);
        //   if (ch == '?' && msg.charAt(i) != '?') {
        //     throw new IllegalArgumentException("Message contains characters outside ISO-8859-1 encoding.");
        //   }
        //   sb.append(ch);
        // }
        let sb = if let Ok(encoded_bytes) =
            ISO_8859_1_ENCODER.encode(msg, encoding::EncoderTrap::Strict)
        {
            ISO_8859_1_ENCODER
                .decode(&encoded_bytes, encoding::DecoderTrap::Strict)
                .expect("round trip decode should always work")
        } else {
            return Err(Exceptions::IllegalArgumentException(
                "Message contains characters outside ISO-8859-1 encoding.".to_owned(),
            ));
        };
        Ok(Self {
          symbol_lookup:Rc::new(SymbolInfoLookup::new()),
            msg: sb,
            shape: SymbolShapeHint::FORCE_NONE,
            codewords: String::with_capacity(msg.len()),
            newEncoding: None,
            minSize: None,
            maxSize: None,
            pos: 0,
            symbolInfo: None,
            skipAtEnd: 0,
        })
    }

    pub fn setSymbolShape(&mut self, shape: SymbolShapeHint) {
        self.shape = shape;
    }

    pub fn setSizeConstraints(&mut self, minSize: Option<Dimension>, maxSize: Option<Dimension>) {
        self.minSize = minSize;
        self.maxSize = maxSize;
    }

    pub fn getMessage(&self) -> &str {
        &self.msg
    }

    pub fn setSkipAtEnd(&mut self, count: u32) {
        self.skipAtEnd = count;
    }

    pub fn getCurrentChar(&self) -> char {
        // self.msg.graphemes(true).nth(self.pos as usize).unwrap()
        self.msg.chars().nth(self.pos as usize).unwrap()
    }

    pub fn getCurrent(&self) -> char {
        // self.msg.graphemes(true).nth(self.pos as usize).unwrap()
        self.msg.chars().nth(self.pos as usize).unwrap()
    }

    pub fn getCodewords(&self) -> &str {
        &self.codewords
    }

    pub fn writeCodewords(&mut self, codewords: &str) {
        self.codewords.push_str(codewords);
    }

    pub fn writeCodeword(&mut self, codeword: u8) {
        self.codewords.push(codeword as char);
    }

    pub fn getCodewordCount(&self) -> usize {
        self.codewords.len()
    }

    pub fn getNewEncoding(&self) -> Option<usize> {
        self.newEncoding
    }

    pub fn signalEncoderChange(&mut self, encoding: usize) {
        self.newEncoding = Some(encoding);
    }

    pub fn resetEncoderSignal(&mut self) {
        self.newEncoding = None;
    }

    pub fn hasMoreCharacters(&self) -> bool {
        self.pos < self.getTotalMessageCharCount()
    }

    fn getTotalMessageCharCount(&self) -> u32 {
        self.msg.len() as u32 - self.skipAtEnd
    }

    pub fn getRemainingCharacters(&self) -> u32 {
        self.getTotalMessageCharCount() - self.pos
    }

    pub fn getSymbolInfo(&self) -> &Option<&SymbolInfo> {
        &self.symbolInfo
    }

    pub fn updateSymbolInfo(&mut self) {
        self.updateSymbolInfoWithLength(self.getCodewordCount());
    }

    pub fn updateSymbolInfoWithLength(&mut self, len: usize) {
        if self.symbolInfo.is_none()
            || len > self.symbolInfo.as_ref().unwrap().getDataCapacity() as usize
        {
            self.symbolInfo = Some(
                self.symbol_lookup.lookup_with_codewords_shape_size_fail(
                    len as u32,
                    self.shape,
                    &self.minSize,
                    &self.maxSize,
                    true,
                )
                .unwrap()
                .unwrap(),
            );
        }
    }

    pub fn resetSymbolInfo(&mut self) {
        self.symbolInfo = None;
    }
}
