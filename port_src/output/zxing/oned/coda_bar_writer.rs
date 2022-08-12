/*
 * Copyright 2011 ZXing authors
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
// package com::google::zxing::oned;

/**
 * This class renders CodaBar as {@code boolean[]}.
 *
 * @author dsbnatut@gmail.com (Kazuki Nishiura)
 */

 const START_END_CHARS: vec![Vec<char>; 4] = vec!['A', 'B', 'C', 'D', ]
;

 const ALT_START_END_CHARS: vec![Vec<char>; 4] = vec!['T', 'N', '*', 'E', ]
;

 const CHARS_WHICH_ARE_TEN_LENGTH_EACH_AFTER_DECODED: vec![Vec<char>; 4] = vec!['/', ':', '+', '.', ]
;

 const DEFAULT_GUARD: char = START_END_CHARS[0];
pub struct CodaBarWriter {
    super: OneDimensionalCodeWriter;
}

impl CodaBarWriter {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::CODABAR);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
        if contents.length() < 2 {
            // Can't have a start/end guard, so tentatively add default guards
            contents = format!("{}{}{}", DEFAULT_GUARD, contents, DEFAULT_GUARD);
        } else {
            // Verify input and calculate decoded length.
             let first_char: char = Character::to_upper_case(&contents.char_at(0));
             let last_char: char = Character::to_upper_case(&contents.char_at(contents.length() - 1));
             let starts_normal: bool = CodaBarReader::array_contains(&START_END_CHARS, first_char);
             let ends_normal: bool = CodaBarReader::array_contains(&START_END_CHARS, last_char);
             let starts_alt: bool = CodaBarReader::array_contains(&ALT_START_END_CHARS, first_char);
             let ends_alt: bool = CodaBarReader::array_contains(&ALT_START_END_CHARS, last_char);
            if starts_normal {
                if !ends_normal {
                    throw IllegalArgumentException::new(format!("Invalid start/end guards: {}", contents));
                }
            // else already has valid start/end
            } else if starts_alt {
                if !ends_alt {
                    throw IllegalArgumentException::new(format!("Invalid start/end guards: {}", contents));
                }
            // else already has valid start/end
            } else {
                // Doesn't start with a guard
                if ends_normal || ends_alt {
                    throw IllegalArgumentException::new(format!("Invalid start/end guards: {}", contents));
                }
                // else doesn't end with guard either, so add a default
                contents = format!("{}{}{}", DEFAULT_GUARD, contents, DEFAULT_GUARD);
            }
        }
        // The start character and the end character are decoded to 10 length each.
         let result_length: i32 = 20;
         {
             let mut i: i32 = 1;
            while i < contents.length() - 1 {
                {
                    if Character::is_digit(&contents.char_at(i)) || contents.char_at(i) == '-' || contents.char_at(i) == '$' {
                        result_length += 9;
                    } else if CodaBarReader::array_contains(&CHARS_WHICH_ARE_TEN_LENGTH_EACH_AFTER_DECODED, &contents.char_at(i)) {
                        result_length += 10;
                    } else {
                        throw IllegalArgumentException::new(format!("Cannot encode : '{}\'", contents.char_at(i)));
                    }
                }
                i += 1;
             }
         }

        // A blank is placed between each character.
        result_length += contents.length() - 1;
         let mut result: [bool; result_length] = [false; result_length];
         let mut position: i32 = 0;
         {
             let mut index: i32 = 0;
            while index < contents.length() {
                {
                     let mut c: char = Character::to_upper_case(&contents.char_at(index));
                    if index == 0 || index == contents.length() - 1 {
                        // The start/end chars are not in the CodaBarReader.ALPHABET.
                        match c {
                              'T' => 
                                 {
                                    c = 'A';
                                    break;
                                }
                              'N' => 
                                 {
                                    c = 'B';
                                    break;
                                }
                              '*' => 
                                 {
                                    c = 'C';
                                    break;
                                }
                              'E' => 
                                 {
                                    c = 'D';
                                    break;
                                }
                        }
                    }
                     let mut code: i32 = 0;
                     {
                         let mut i: i32 = 0;
                        while i < CodaBarReader::ALPHABET::len() {
                            {
                                // Found any, because I checked above.
                                if c == CodaBarReader::ALPHABET[i] {
                                    code = CodaBarReader::CHARACTER_ENCODINGS[i];
                                    break;
                                }
                            }
                            i += 1;
                         }
                     }

                     let mut color: bool = true;
                     let mut counter: i32 = 0;
                     let mut bit: i32 = 0;
                    while bit < 7 {
                        // A character consists of 7 digit.
                        result[position] = color;
                        position += 1;
                        if ((code >> (6 - bit)) & 1) == 0 || counter == 1 {
                            // Flip the color.
                            color = !color;
                            bit += 1;
                            counter = 0;
                        } else {
                            counter += 1;
                        }
                    }
                    if index < contents.length() - 1 {
                        result[position] = false;
                        position += 1;
                    }
                }
                index += 1;
             }
         }

        return result;
    }
}

