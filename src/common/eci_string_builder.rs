/*
 * Copyright 2022 ZXing authors
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

// import com.google.zxing.FormatException;

// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;

use std::fmt;

use crate::common::Result;

use super::CharacterSet;

/**
 * Class that converts a sequence of ECIs and bytes into a string
 *
 * @author Alex Geller
 */
pub struct ECIStringBuilder {
    current_bytes: Vec<u8>,
    result: String,
    current_charset: Option<CharacterSet>, //= StandardCharsets.ISO_8859_1;
}

impl ECIStringBuilder {
    pub fn new() -> Self {
        Self {
            current_bytes: Vec::new(),
            result: String::new(),
            current_charset: Some(CharacterSet::ISO8859_1),
        }
    }
    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            current_bytes: Vec::with_capacity(initial_capacity),
            result: String::with_capacity(initial_capacity),
            current_charset: Some(CharacterSet::ISO8859_1),
        }
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value character whose lowest byte is to be appended
     */
    pub fn append_char(&mut self, value: char) {
        self.current_bytes.push(value as u8);
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value byte to append
     */
    pub fn append_byte(&mut self, value: u8) {
        self.current_bytes.push(value);
    }

    /**
     * Appends the characters in {@code value} as bytes values
     *
     * @param value string to append
     */
    pub fn append_string(&mut self, value: &str) {
        value
            .as_bytes()
            .iter()
            .map(|b| self.current_bytes.push(*b))
            .count();
        // self.current_bytes.push(value.as_bytes());
    }

    /**
     * Append the string repesentation of {@code value} (short for {@code append(String.valueOf(value))})
     *
     * @param value int to append as a string
     */
    pub fn append(&mut self, value: i32) {
        self.append_string(&format!("{value}"));
    }

    /**
     * Appends ECI value to output.
     *
     * @param value ECI value to append, as an int
     * @throws FormatException on invalid ECI value
     */
    pub fn appendECI(&mut self, value: u32) -> Result<()> {
        self.encodeCurrentBytesIfAny();

        self.current_charset = CharacterSet::get_character_set_by_eci(value).ok();


        // if let Ok(character_set_eci) = CharacterSetECI::getCharacterSetECIByValue(value) {
        //     // dbg!(
        //     //     character_set_eci,
        //     //     CharacterSetECI::getCharset(&character_set_eci).name(),
        //     //     CharacterSetECI::getCharset(&character_set_eci).whatwg_name()
        //     // );
        //     self.current_charset = Some(character_set_eci);
        // } else {
        //     self.current_charset = None
        // }

        // self.current_charset = CharacterSetECI::getCharset(&character_set_eci);
        Ok(())
    }

    /// Finishes encoding anything in the buffer using the current ECI and resets.
    ///
    /// This function can panic
    pub fn encodeCurrentBytesIfAny(&mut self) {
        if let Some(encoder) = self.current_charset {
            if encoder == CharacterSet::UTF8 {
                if !self.current_bytes.is_empty() {
                    self.result.push_str(
                        &String::from_utf8(std::mem::take(&mut self.current_bytes)).unwrap(),
                    );
                    self.current_bytes.clear();
                }
            } else if !self.current_bytes.is_empty() {
                let bytes = std::mem::take(&mut self.current_bytes);
                self.current_bytes.clear();
                let encoded_value = encoder.decode(&bytes).unwrap();
                self.result.push_str(&encoded_value);
            }
        } else {
            for byte in &self.current_bytes {
                self.result.push(char::from(*byte))
            }
            self.current_bytes.clear();
        }
    }

    /**
     * Appends the characters from {@code value} (unlike all other append methods of this class who append bytes)
     *
     * @param value characters to append
     */
    pub fn appendCharacters(&mut self, value: &str) {
        self.encodeCurrentBytesIfAny();
        self.result.push_str(value);
    }

    /**
     * Short for {@code toString().length()} (if possible, use {@link #isEmpty()} instead)
     *
     * @return length of string representation in characters
     */
    pub fn len(&mut self) -> usize {
        self.encodeCurrentBytesIfAny(); //return toString().length();
        self.result.chars().count()
    }

    /**
     * @return true iff nothing has been appended
     */
    pub fn is_empty(&mut self) -> bool {
        self.current_bytes.is_empty() && self.result.is_empty()
    }

    pub fn build_result(mut self) -> Self {
        self.encodeCurrentBytesIfAny();

        self
    }
}

impl fmt::Display for ECIStringBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //self.encodeCurrentBytesIfAny();
        write!(f, "{}", self.result)
    }
}

impl Default for ECIStringBuilder {
    fn default() -> Self {
        Self::new()
    }
}
