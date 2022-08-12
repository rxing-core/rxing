/*
 * Copyright 2015 ZXing authors
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
 * This object renders a CODE93 code as a BitMatrix
 */
pub struct Code93Writer {
    super: OneDimensionalCodeWriter;
}

impl Code93Writer {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::CODE_93);
    }

    /**
   * @param contents barcode contents to encode. It should not be encoded for extended characters.
   * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
   */
    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
        contents = ::convert_to_extended(&contents);
         let length: i32 = contents.length();
        if length > 80 {
            throw IllegalArgumentException::new(format!("Requested contents should be less than 80 digits long after converting to extended encoding, but got {}", length));
        }
        //length of code + 2 start/stop characters + 2 checksums, each of 9 bits, plus a termination bar
         let code_width: i32 = (contents.length() + 2 + 2) * 9 + 1;
         let mut result: [bool; code_width] = [false; code_width];
        //start character (*)
         let mut pos: i32 = ::append_pattern(&result, 0, Code93Reader::ASTERISK_ENCODING);
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let index_in_string: i32 = Code93Reader::ALPHABET_STRING::index_of(&contents.char_at(i));
                    pos += ::append_pattern(&result, pos, Code93Reader::CHARACTER_ENCODINGS[index_in_string]);
                }
                i += 1;
             }
         }

        //add two checksums
         let check1: i32 = ::compute_checksum_index(&contents, 20);
        pos += ::append_pattern(&result, pos, Code93Reader::CHARACTER_ENCODINGS[check1]);
        //append the contents to reflect the first checksum added
        contents += Code93Reader::ALPHABET_STRING::char_at(check1);
         let check2: i32 = ::compute_checksum_index(&contents, 15);
        pos += ::append_pattern(&result, pos, Code93Reader::CHARACTER_ENCODINGS[check2]);
        //end character (*)
        pos += ::append_pattern(&result, pos, Code93Reader::ASTERISK_ENCODING);
        //termination bar (single black bar)
        result[pos] = true;
        return result;
    }

    /**
   * @param target output to append to
   * @param pos start position
   * @param pattern pattern to append
   * @param startColor unused
   * @return 9
   * @deprecated without replacement; intended as an internal-only method
   */
    pub fn  append_pattern( target: &Vec<bool>,  pos: i32,  pattern: &Vec<i32>,  start_color: bool) -> i32  {
        for  let bit: i32 in pattern {
            target[pos += 1 !!!check!!! post increment] = bit != 0;
        }
        return 9;
    }

    fn  append_pattern( target: &Vec<bool>,  pos: i32,  a: i32) -> i32  {
         {
             let mut i: i32 = 0;
            while i < 9 {
                {
                     let temp: i32 = a & (1 << (8 - i));
                    target[pos + i] = temp != 0;
                }
                i += 1;
             }
         }

        return 9;
    }

    fn  compute_checksum_index( contents: &String,  max_weight: i32) -> i32  {
         let mut weight: i32 = 1;
         let mut total: i32 = 0;
         {
             let mut i: i32 = contents.length() - 1;
            while i >= 0 {
                {
                     let index_in_string: i32 = Code93Reader::ALPHABET_STRING::index_of(&contents.char_at(i));
                    total += index_in_string * weight;
                    if weight += 1 > max_weight {
                        weight = 1;
                    }
                }
                i -= 1;
             }
         }

        return total % 47;
    }

    fn  convert_to_extended( contents: &String) -> String  {
         let length: i32 = contents.length();
         let extended_content: StringBuilder = StringBuilder::new(length * 2);
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let character: char = contents.char_at(i);
                    // ($)=a, (%)=b, (/)=c, (+)=d. see Code93Reader.ALPHABET_STRING
                    if character == 0 {
                        // NUL: (%)U
                        extended_content.append("bU");
                    } else if character <= 26 {
                        // SOH - SUB: ($)A - ($)Z
                        extended_content.append('a');
                        extended_content.append(('A' + character - 1) as char);
                    } else if character <= 31 {
                        // ESC - US: (%)A - (%)E
                        extended_content.append('b');
                        extended_content.append(('A' + character - 27) as char);
                    } else if character == ' ' || character == '$' || character == '%' || character == '+' {
                        // space $ % +
                        extended_content.append(character);
                    } else if character <= ',' {
                        // ! " # & ' ( ) * ,: (/)A - (/)L
                        extended_content.append('c');
                        extended_content.append(('A' + character - '!') as char);
                    } else if character <= '9' {
                        extended_content.append(character);
                    } else if character == ':' {
                        // :: (/)Z
                        extended_content.append("cZ");
                    } else if character <= '?' {
                        // ; - ?: (%)F - (%)J
                        extended_content.append('b');
                        extended_content.append(('F' + character - ';') as char);
                    } else if character == '@' {
                        // @: (%)V
                        extended_content.append("bV");
                    } else if character <= 'Z' {
                        // A - Z
                        extended_content.append(character);
                    } else if character <= '_' {
                        // [ - _: (%)K - (%)O
                        extended_content.append('b');
                        extended_content.append(('K' + character - '[') as char);
                    } else if character == '`' {
                        // `: (%)W
                        extended_content.append("bW");
                    } else if character <= 'z' {
                        // a - z: (*)A - (*)Z
                        extended_content.append('d');
                        extended_content.append(('A' + character - 'a') as char);
                    } else if character <= 127 {
                        // { - DEL: (%)P - (%)T
                        extended_content.append('b');
                        extended_content.append(('P' + character - '{') as char);
                    } else {
                        throw IllegalArgumentException::new(format!("Requested content contains a non-encodable character: '{}'", character));
                    }
                }
                i += 1;
             }
         }

        return extended_content.to_string();
    }
}

