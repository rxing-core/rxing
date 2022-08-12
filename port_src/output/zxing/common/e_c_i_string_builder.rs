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
// package com::google::zxing::common;

/**
 * Class that converts a sequence of ECIs and bytes into a string
 *
 * @author Alex Geller
 */
pub struct ECIStringBuilder {

     let current_bytes: StringBuilder;

     let mut result: StringBuilder;

     let current_charset: Charset = StandardCharsets::ISO_8859_1;
}

impl ECIStringBuilder {

    pub fn new() -> ECIStringBuilder {
        current_bytes = StringBuilder::new();
    }

    pub fn new( initial_capacity: i32) -> ECIStringBuilder {
        current_bytes = StringBuilder::new(initial_capacity);
    }

    /**
   * Appends {@code value} as a byte value
   *
   * @param value character whose lowest byte is to be appended
   */
    pub fn  append(&self,  value: char)   {
        self.current_bytes.append((value & 0xff) as char);
    }

    /**
   * Appends {@code value} as a byte value
   *
   * @param value byte to append
   */
    pub fn  append(&self,  value: i8)   {
        self.current_bytes.append((value & 0xff) as char);
    }

    /**
   * Appends the characters in {@code value} as bytes values
   *
   * @param value string to append
   */
    pub fn  append(&self,  value: &String)   {
        self.current_bytes.append(&value);
    }

    /**
   * Append the string repesentation of {@code value} (short for {@code append(String.valueOf(value))})
   *
   * @param value int to append as a string
   */
    pub fn  append(&self,  value: i32)   {
        self.append(&String::value_of(value));
    }

    /**
   * Appends ECI value to output.
   *
   * @param value ECI value to append, as an int
   * @throws FormatException on invalid ECI value
   */
    pub fn  append_e_c_i(&self,  value: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        self.encode_current_bytes_if_any();
         let character_set_e_c_i: CharacterSetECI = CharacterSetECI::get_character_set_e_c_i_by_value(value);
        if character_set_e_c_i == null {
            throw FormatException::get_format_instance();
        }
        self.current_charset = character_set_e_c_i.get_charset();
    }

    fn  encode_current_bytes_if_any(&self)   {
        if self.current_charset.equals(StandardCharsets::ISO_8859_1) {
            if self.current_bytes.length() > 0 {
                if self.result == null {
                    self.result = self.current_bytes;
                    self.current_bytes = StringBuilder::new();
                } else {
                    self.result.append(&self.current_bytes);
                    self.current_bytes = StringBuilder::new();
                }
            }
        } else if self.current_bytes.length() > 0 {
             let bytes: Vec<i8> = self.current_bytes.to_string().get_bytes(StandardCharsets::ISO_8859_1);
            self.current_bytes = StringBuilder::new();
            if self.result == null {
                self.result = StringBuilder::new(String::new(&bytes, &self.current_charset));
            } else {
                self.result.append(String::new(&bytes, &self.current_charset));
            }
        }
    }

    /**
   * Appends the characters from {@code value} (unlike all other append methods of this class who append bytes)
   *
   * @param value characters to append
   */
    pub fn  append_characters(&self,  value: &StringBuilder)   {
        self.encode_current_bytes_if_any();
        self.result.append(&value);
    }

    /**
   * Short for {@code toString().length()} (if possible, use {@link #isEmpty()} instead)
   *
   * @return length of string representation in characters
   */
    pub fn  length(&self) -> i32  {
        return self.to_string().length();
    }

    /**
   * @return true iff nothing has been appended
   */
    pub fn  is_empty(&self) -> bool  {
        return self.current_bytes.length() == 0 && (self.result == null || self.result.length() == 0);
    }

    pub fn  to_string(&self) -> String  {
        self.encode_current_bytes_if_any();
        return  if self.result == null { "" } else { self.result.to_string() };
    }
}

