/*
 * Copyright 2021 ZXing authors
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

// package com.google.zxing.common;

// import java.nio.charset.Charset;
// import java.nio.charset.CharsetEncoder;
// import java.nio.charset.StandardCharsets;
// import java.nio.charset.UnsupportedCharsetException;
// import java.util.ArrayList;
// import java.util.List;

use encoding::{Encoding, EncodingRef};
use unicode_segmentation::UnicodeSegmentation;

use super::CharacterSetECI;

use lazy_static::lazy_static;

lazy_static! {
    static ref ENCODERS : Vec<EncodingRef> = {
        let mut enc_vec = Vec::new();
        for name in NAMES {
            if let Some(enc) = CharacterSetECI::getCharacterSetECIByName(name) {
                // try {
                    enc_vec.push(CharacterSetECI::getCharset(&enc));
                // } catch (UnsupportedCharsetException e) {
                // continue
                // }
            }
        }
        enc_vec
    };
}
const NAMES: [&str; 20] = [
    "IBM437",
    "ISO-8859-2",
    "ISO-8859-3",
    "ISO-8859-4",
    "ISO-8859-5",
    "ISO-8859-6",
    "ISO-8859-7",
    "ISO-8859-8",
    "ISO-8859-9",
    "ISO-8859-10",
    "ISO-8859-11",
    "ISO-8859-13",
    "ISO-8859-14",
    "ISO-8859-15",
    "ISO-8859-16",
    "windows-1250",
    "windows-1251",
    "windows-1252",
    "windows-1256",
    "Shift_JIS",
];

/**
 * Set of CharsetEncoders for a given input string
 *
 * Invariants:
 * - The list contains only encoders from CharacterSetECI (list is shorter then the list of encoders available on
 *   the platform for which ECI values are defined).
 * - The list contains encoders at least one encoder for every character in the input.
 * - The first encoder in the list is always the ISO-8859-1 encoder even of no character in the input can be encoded
 *       by it.
 * - If the input contains a character that is not in ISO-8859-1 then the last two entries in the list will be the
 *   UTF-8 encoder and the UTF-16BE encoder.
 *
 * @author Alex Geller
 */
#[derive(Clone)]
pub struct ECIEncoderSet {
    encoders: Vec<EncodingRef>,
    priorityEncoderIndex: Option<usize>,
}

impl ECIEncoderSet {
    /**
     * Constructs an encoder set
     *
     * @param stringToEncode the string that needs to be encoded
     * @param priorityCharset The preferred {@link Charset} or null.
     * @param fnc1 fnc1 denotes the character in the input that represents the FNC1 character or -1 for a non-GS1 bar
     * code. When specified, it is considered an error to pass it as argument to the methods canEncode() or encode().
     */
    pub fn new(
        stringToEncodeMain: &str,
        priorityCharset: Option<EncodingRef>,
        fnc1: Option<&str>,
    ) -> Self {
        // List of encoders that potentially encode characters not in ISO-8859-1 in one byte.

        let mut encoders: Vec<EncodingRef>;
        let mut priorityEncoderIndexValue = None;

        let mut neededEncoders: Vec<EncodingRef> = Vec::new();

        let stringToEncode = stringToEncodeMain.graphemes(true).collect::<Vec<&str>>();

        //we always need the ISO-8859-1 encoder. It is the default encoding
        neededEncoders.push(encoding::all::ISO_8859_1);
        let mut needUnicodeEncoder = if let Some(pc) = priorityCharset {
            pc.name().starts_with("UTF")
        } else {
            false
        };

        //Walk over the input string and see if all characters can be encoded with the list of encoders
        for i in 0..stringToEncode.len() {
            // for (int i = 0; i < stringToEncode.length(); i++) {
            let mut canEncode = false;
            for encoder in &neededEncoders {
                //   for (CharsetEncoder encoder : neededEncoders) {
                let c = stringToEncode.get(i).unwrap();
                if (fnc1.is_some() && c == fnc1.as_ref().unwrap())
                    || encoder.encode(c, encoding::EncoderTrap::Strict).is_ok()
                {
                    canEncode = true;
                    break;
                }
            }
            if !canEncode {
                //for the character at position i we don't yet have an encoder in the list
                for i_encoder in 0..ENCODERS.len() {
                    // for encoder in ENCODERS {
                    let encoder = ENCODERS.get(i_encoder).unwrap();
                    // for (CharsetEncoder encoder : ENCODERS) {
                    if encoder
                        .encode(
                            &stringToEncode.get(i).unwrap(),
                            encoding::EncoderTrap::Strict,
                        )
                        .is_ok()
                    {
                        //Good, we found an encoder that can encode the character. We add him to the list and continue scanning
                        //the input
                        neededEncoders.push(*encoder);
                        canEncode = true;
                        break;
                    }
                }
            }

            if !canEncode {
                //The character is not encodeable by any of the single byte encoders so we remember that we will need a
                //Unicode encoder.
                needUnicodeEncoder = true;
            }
        }

        if neededEncoders.len() == 1 && !needUnicodeEncoder {
            //the entire input can be encoded by the ISO-8859-1 encoder
            encoders = vec![encoding::all::ISO_8859_1];
        } else {
            // we need more than one single byte encoder or we need a Unicode encoder.
            // In this case we append a UTF-8 and UTF-16 encoder to the list
            //   encoders = [] new CharsetEncoder[neededEncoders.size() + 2];
            encoders = Vec::new();
            // let index = 0;

            for encoder in neededEncoders {
                //   for (CharsetEncoder encoder : neededEncoders) {
                //encoders[index++] = encoder;
                encoders.push(encoder);
            }

            encoders.push(encoding::all::UTF_8);
            encoders.push(encoding::all::UTF_16BE);
        }

        //Compute priorityEncoderIndex by looking up priorityCharset in encoders
        // if priorityCharset != null {
        if priorityCharset.is_some() {
            for i in 0..encoders.len() {
                //   for (int i = 0; i < encoders.length; i++) {
                if priorityCharset.as_ref().unwrap().name() == encoders[i].name() {
                    priorityEncoderIndexValue = Some(i);
                    break;
                }
            }
        }
        // }
        //invariants
        assert_eq!(encoders[0].name(), encoding::all::ISO_8859_1.name());
        Self {
            encoders: encoders,
            priorityEncoderIndex: priorityEncoderIndexValue,
        }
    }

    pub fn len(&self) -> usize {
        return self.encoders.len();
    }

    pub fn getCharsetName(&self, index: usize) -> &'static str {
        assert!(index < self.len());
        return self.encoders[index].name();
    }

    pub fn getCharset(&self, index: usize) -> EncodingRef {
        assert!(index < self.len());
        return self.encoders[index];
    }

    pub fn getECIValue(&self, encoderIndex: usize) -> u32 {
        CharacterSetECI::getValue(
            &CharacterSetECI::getCharacterSetECI(self.encoders[encoderIndex]).unwrap(),
        )
    }

    /*
     *  returns -1 if no priority charset was defined
     */
    pub fn getPriorityEncoderIndex(&self) -> Option<usize> {
        self.priorityEncoderIndex
    }

    pub fn canEncode(&self, c: &str, encoderIndex: usize) -> bool {
        assert!(encoderIndex < self.len());
        let encoder = self.encoders[encoderIndex];
        let enc_data = encoder.encode(c, encoding::EncoderTrap::Strict);

        enc_data.is_ok()
    }

    pub fn encode_char(&self, c: &str, encoderIndex: usize) -> Vec<u8> {
        assert!(encoderIndex < self.len());
        let encoder = self.encoders[encoderIndex];
        let enc_data = encoder.encode(&c.to_string(), encoding::EncoderTrap::Strict);
        assert!(enc_data.is_ok());
        return enc_data.unwrap();
    }

    pub fn encode_string(&self, s: &str, encoderIndex: usize) -> Vec<u8> {
        assert!(encoderIndex < self.len());
        let encoder = self.encoders[encoderIndex];
        encoder.encode(s, encoding::EncoderTrap::Replace).unwrap()
    }
}
