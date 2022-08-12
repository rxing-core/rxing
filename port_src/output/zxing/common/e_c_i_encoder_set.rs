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
// package com::google::zxing::common;

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

// List of encoders that potentially encode characters not in ISO-8859-1 in one byte.
 const ENCODERS: List<CharsetEncoder> = ArrayList<>::new();
pub struct ECIEncoderSet {

     let mut encoders: Vec<CharsetEncoder>;

     let priority_encoder_index: i32;
}

impl ECIEncoderSet {

    static {
         let names: vec![Vec<String>; 20] = vec!["IBM437", "ISO-8859-2", "ISO-8859-3", "ISO-8859-4", "ISO-8859-5", "ISO-8859-6", "ISO-8859-7", "ISO-8859-8", "ISO-8859-9", "ISO-8859-10", "ISO-8859-11", "ISO-8859-13", "ISO-8859-14", "ISO-8859-15", "ISO-8859-16", "windows-1250", "windows-1251", "windows-1252", "windows-1256", "Shift_JIS", ]
        ;
        for  let name: String in names {
            if CharacterSetECI::get_character_set_e_c_i_by_name(&name) != null {
                let tryResult1 = 0;
                'try1: loop {
                {
                    ENCODERS::add(&Charset::for_name(&name)::new_encoder());
                }
                break 'try1
                }
                match tryResult1 {
                     catch ( e: &UnsupportedCharsetException) {
                    }  0 => break
                }

            }
        }
    }

    /**
   * Constructs an encoder set
   *
   * @param stringToEncode the string that needs to be encoded
   * @param priorityCharset The preferred {@link Charset} or null.
   * @param fnc1 fnc1 denotes the character in the input that represents the FNC1 character or -1 for a non-GS1 bar
   * code. When specified, it is considered an error to pass it as argument to the methods canEncode() or encode().
   */
    pub fn new( string_to_encode: &String,  priority_charset: &Charset,  fnc1: i32) -> ECIEncoderSet {
         let needed_encoders: List<CharsetEncoder> = ArrayList<>::new();
        //we always need the ISO-8859-1 encoder. It is the default encoding
        needed_encoders.add(&StandardCharsets::ISO_8859_1::new_encoder());
         let need_unicode_encoder: bool = priority_charset != null && priority_charset.name().starts_with("UTF");
        //Walk over the input string and see if all characters can be encoded with the list of encoders 
         {
             let mut i: i32 = 0;
            while i < string_to_encode.length() {
                {
                     let can_encode: bool = false;
                    for  let encoder: CharsetEncoder in needed_encoders {
                         let c: char = string_to_encode.char_at(i);
                        if c == fnc1 || encoder.can_encode(c) {
                            can_encode = true;
                            break;
                        }
                    }
                    if !can_encode {
                        //for the character at position i we don't yet have an encoder in the list
                        for  let encoder: CharsetEncoder in ENCODERS {
                            if encoder.can_encode(&string_to_encode.char_at(i)) {
                                //Good, we found an encoder that can encode the character. We add him to the list and continue scanning
                                //the input
                                needed_encoders.add(&encoder);
                                can_encode = true;
                                break;
                            }
                        }
                    }
                    if !can_encode {
                        //The character is not encodeable by any of the single byte encoders so we remember that we will need a
                        //Unicode encoder.
                        need_unicode_encoder = true;
                    }
                }
                i += 1;
             }
         }

        if needed_encoders.size() == 1 && !need_unicode_encoder {
            //the entire input can be encoded by the ISO-8859-1 encoder
            encoders =  : vec![CharsetEncoder; 1] = vec![needed_encoders.get(0), ]
            ;
        } else {
            // we need more than one single byte encoder or we need a Unicode encoder.
            // In this case we append a UTF-8 and UTF-16 encoder to the list
            encoders = : [Option<CharsetEncoder>; needed_encoders.size() + 2] = [None; needed_encoders.size() + 2];
             let mut index: i32 = 0;
            for  let encoder: CharsetEncoder in needed_encoders {
                encoders[index += 1 !!!check!!! post increment] = encoder;
            }
            encoders[index] = StandardCharsets::UTF_8::new_encoder();
            encoders[index + 1] = StandardCharsets::UTF_16BE::new_encoder();
        }
        //Compute priorityEncoderIndex by looking up priorityCharset in encoders
         let priority_encoder_index_value: i32 = -1;
        if priority_charset != null {
             {
                 let mut i: i32 = 0;
                while i < encoders.len() {
                    {
                        if encoders[i] != null && priority_charset.name().equals(&encoders[i].charset().name()) {
                            priority_encoder_index_value = i;
                            break;
                        }
                    }
                    i += 1;
                 }
             }

        }
        priority_encoder_index = priority_encoder_index_value;
        //invariants
        assert!( encoders[0].charset().equals(StandardCharsets::ISO_8859_1));
    }

    pub fn  length(&self) -> i32  {
        return self.encoders.len();
    }

    pub fn  get_charset_name(&self,  index: i32) -> String  {
        assert!( index < self.length());
        return self.encoders[index].charset().name();
    }

    pub fn  get_charset(&self,  index: i32) -> Charset  {
        assert!( index < self.length());
        return self.encoders[index].charset();
    }

    pub fn  get_e_c_i_value(&self,  encoder_index: i32) -> i32  {
        return CharacterSetECI::get_character_set_e_c_i(&self.encoders[encoder_index].charset())::get_value();
    }

    /*
   *  returns -1 if no priority charset was defined
   */
    pub fn  get_priority_encoder_index(&self) -> i32  {
        return self.priority_encoder_index;
    }

    pub fn  can_encode(&self,  c: char,  encoder_index: i32) -> bool  {
        assert!( encoder_index < self.length());
         let encoder: CharsetEncoder = self.encoders[encoder_index];
        return encoder.can_encode(format!("{}", c));
    }

    pub fn  encode(&self,  c: char,  encoder_index: i32) -> Vec<i8>  {
        assert!( encoder_index < self.length());
         let encoder: CharsetEncoder = self.encoders[encoder_index];
        assert!( encoder.can_encode(format!("{}", c)));
        return (format!("{}", c)).get_bytes(&encoder.charset());
    }

    pub fn  encode(&self,  s: &String,  encoder_index: i32) -> Vec<i8>  {
        assert!( encoder_index < self.length());
         let encoder: CharsetEncoder = self.encoders[encoder_index];
        return s.get_bytes(&encoder.charset());
    }
}

