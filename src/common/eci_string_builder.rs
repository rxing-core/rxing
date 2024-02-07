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

use std::{
    collections::{HashMap, HashSet},
    fmt::{self},
};

use super::{CharacterSet, Eci, StringUtils};

/**
 * Class that converts a sequence of ECIs and bytes into a string
 *
 * @author Alex Geller
 */
#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct ECIStringBuilder {
    pub has_eci: bool,
    eci_result: Option<String>,
    bytes: Vec<u8>,
    pub(crate) eci_positions: Vec<(Eci, usize, usize)>, // (Eci, start, end)
    pub symbology: SymbologyIdentifier,
    eci_list: HashSet<Eci>,
}

impl ECIStringBuilder {
    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            eci_result: None,
            bytes: Vec::with_capacity(initial_capacity),
            eci_positions: Vec::default(),
            has_eci: false,
            symbology: SymbologyIdentifier::default(),
            eci_list: HashSet::default(),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value character whose lowest byte is to be appended
     */
    pub fn append_char(&mut self, value: char) {
        self.eci_result = None;
        self.bytes.push(value as u8);
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value byte to append
     */
    pub fn append_byte(&mut self, value: u8) {
        self.eci_result = None;
        self.bytes.push(value)
    }

    pub fn append_bytes(&mut self, value: &[u8]) {
        self.eci_result = None;
        self.bytes.extend_from_slice(value)
    }

    /**
     * Appends the characters in {@code value} as bytes values
     *
     * @param value string to append
     */
    pub fn append_string(&mut self, value: &str) {
        if !value.is_ascii() {
            self.append_eci(Eci::UTF8);
        }
        self.eci_result = None;
        self.bytes.extend_from_slice(value.as_bytes());
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
    pub fn append_eci(&mut self, eci: Eci) {
        self.eci_result = None;

        if !self.has_eci && eci != Eci::ISO8859_1 {
            self.has_eci = true;
        }

        if self.has_eci {
            if let Some(last) = self.eci_positions.last_mut() {
                last.2 = self.bytes.len()
            }

            self.eci_positions.push((eci, self.bytes.len(), 0));

            self.eci_list.insert(eci);

            if self.eci_list.len() == 1 && (self.eci_list.contains(&Eci::Unknown)) {
                self.has_eci = false;
                self.eci_positions.clear();
            }
        }
    }

    /// Change the current encoding characterset, finding an eci to do so
    pub fn switch_encoding(&mut self, charset: CharacterSet, is_eci: bool) {
        //self.append_eci(Eci::from(charset))
        if is_eci && !self.has_eci {
            self.eci_positions.clear();
        }
        if is_eci || !self.has_eci
        //{self.eci_positions.push_back({eci, Size(bytes)});}
        {
            // self.append_eci(Eci::from(charset))
            if let Some(last) = self.eci_positions.last_mut() {
                last.2 = self.bytes.len()
            }

            self.eci_positions
                .push((Eci::from(charset), self.bytes.len(), 0));
        }

        self.has_eci |= is_eci;
    }

    /// Finishes encoding anything in the buffer using the current ECI and resets.
    ///
    /// This function can panic
    pub fn encodeCurrentBytesIfAny(&self) -> String {
        let mut encoded_string = String::with_capacity(self.bytes.len());
        // First encode the first set
        let (_eci, end, _) =
            *self
                .eci_positions
                .first()
                .unwrap_or(&(Eci::ISO8859_1, self.bytes.len(), 0));

        encoded_string.push_str(
            &Self::encode_segment(&self.bytes[0..end], Eci::ISO8859_1).unwrap_or_default(),
        );

        if end == self.bytes.len() {
            return encoded_string;
        }

        // If there are more sets, encode each of them in turn
        for (eci, eci_start, eci_end) in &self.eci_positions {
            // let (_,end) = *self.eci_positions.first().unwrap_or(&(*eci, self.bytes.len()));
            let end = if *eci_end == 0 {
                self.bytes.len()
            } else {
                *eci_end
            };
            encoded_string.push_str(
                &Self::encode_segment(&self.bytes[*eci_start..end], *eci).unwrap_or_default(),
            );
        }

        // Return the result
        encoded_string
    }

    fn encode_segment(bytes: &[u8], eci: Eci) -> Option<String> {
        let mut not_encoded_yet = true;
        let mut encoded_string = String::with_capacity(bytes.len());
        if ![Eci::Binary, Eci::Unknown].contains(&eci) {
            if eci == Eci::UTF8 {
                if !bytes.is_empty() {
                    encoded_string.push_str(&CharacterSet::UTF8.decode(bytes).ok()?);
                    not_encoded_yet = false;
                } else {
                    return None;
                }
            } else if !bytes.is_empty() {
                encoded_string.push_str(&CharacterSet::from(eci).decode(bytes).ok()?);
                not_encoded_yet = false;
            } else {
                return None;
            }
        } else if eci == Eci::Unknown {
            /*  // This probably should never be used, it's here just in case I don't understand what's
                // going on.
            let cs = CharacterSet::from(Eci::ISO8859_1);
            if let Ok(enc_str) = cs.decode(bytes) {
                encoded_string.push_str(&enc_str);
                    not_encoded_yet = false;
            }

            else */
            if let Some(found_encoding) = StringUtils::guessCharset(bytes, &HashMap::default()) {
                if let Ok(found_encoded_str) = found_encoding.decode(bytes) {
                    encoded_string.push_str(&found_encoded_str);
                    not_encoded_yet = false;
                }
            }
        }

        if not_encoded_yet {
            for byte in bytes {
                encoded_string.push(char::from(*byte))
            }
        }

        if encoded_string.is_empty() {
            None
        } else {
            Some(encoded_string)
        }
    }

    /**
     * Appends the characters from {@code value} (unlike all other append methods of this class who append bytes)
     *
     * @param value characters to append
     */
    pub fn appendCharacters(&mut self, value: &str) {
        self.append_string(value);
    }

    /**
     * Short for {@code toString().length()} (if possible, use {@link #isEmpty()} instead)
     *
     * @return length of string representation in characters
     */
    pub fn len(&mut self) -> usize {
        self.bytes.len()
    }

    /// Reserve an additional number of bytes for storage
    pub fn reserve(&mut self, additional: usize) {
        self.bytes.reserve(additional);
    }

    /**
     * @return true iff nothing has been appended
     */
    pub fn is_empty(&mut self) -> bool {
        self.bytes.is_empty()
    }

    pub fn build_result(mut self) -> Self {
        self.eci_result = Some(self.encodeCurrentBytesIfAny());

        self
    }

    // pub fn list_ecis(&self) -> HashSet<Eci> {
    //     let mut hs = HashSet::new();
    //     self.eci_positions.iter().for_each(|pos| {
    //         hs.insert(pos.0);
    //     });
    //     hs
    // }
}

impl fmt::Display for ECIStringBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(res) = &self.eci_result {
            write!(f, "{res}")
        } else {
            write!(f, "{}", self.encodeCurrentBytesIfAny())
        }
    }
}

impl std::ops::AddAssign<u8> for ECIStringBuilder {
    fn add_assign(&mut self, rhs: u8) {
        self.append_byte(rhs)
    }
}

impl std::ops::AddAssign<String> for ECIStringBuilder {
    fn add_assign(&mut self, rhs: String) {
        self.append_string(&rhs)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContentType {
    Text,
    Binary,
    Mixed,
    GS1,
    ISO15434,
    UnknownECI,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AIFlag {
    None,
    GS1,
    AIM,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SymbologyIdentifier {
    //char code = 0, modifier = 0, eciModifierOffset = 0;
    pub code: u8,
    pub modifier: u8,
    pub eciModifierOffset: u8,
    pub aiFlag: AIFlag,
    // AIFlag aiFlag = AIFlag::None;

    // std::string toString(bool hasECI = false) const
    // {
    // 	return code ? ']' + std::string(1, code) + static_cast<char>(modifier + eciModifierOffset * hasECI) : std::string();
    // }
}

impl Default for SymbologyIdentifier {
    fn default() -> Self {
        Self {
            code: 0,
            modifier: 0,
            eciModifierOffset: 0,
            aiFlag: AIFlag::None,
        }
    }
}

impl std::io::Write for ECIStringBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() == 1 {
            self.append_byte(buf[0]);
        } else {
            self.append_bytes(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
