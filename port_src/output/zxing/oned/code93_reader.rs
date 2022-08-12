/*
 * Copyright 2010 ZXing authors
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
 * <p>Decodes Code 93 barcodes.</p>
 *
 * @author Sean Owen
 * @see Code39Reader
 */

// Note that 'abcd' are dummy characters in place of control characters.
 const ALPHABET_STRING: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%abcd*";

 const ALPHABET: Vec<char> = ALPHABET_STRING::to_char_array();

/**
   * These represent the encodings of characters, as patterns of wide and narrow bars.
   * The 9 least-significant bits of each int correspond to the pattern of wide and narrow.
   */
 const CHARACTER_ENCODINGS: vec![Vec<i32>; 48] = vec![// 0-9
0x114, // 0-9
0x148, // 0-9
0x144, // 0-9
0x142, // 0-9
0x128, // 0-9
0x124, // 0-9
0x122, // 0-9
0x150, // 0-9
0x112, // 0-9
0x10A, // A-J
0x1A8, // A-J
0x1A4, // A-J
0x1A2, // A-J
0x194, // A-J
0x192, // A-J
0x18A, // A-J
0x168, // A-J
0x164, // A-J
0x162, // A-J
0x134, // K-T
0x11A, // K-T
0x158, // K-T
0x14C, // K-T
0x146, // K-T
0x12C, // K-T
0x116, // K-T
0x1B4, // K-T
0x1B2, // K-T
0x1AC, // K-T
0x1A6, // U-Z
0x196, // U-Z
0x19A, // U-Z
0x16C, // U-Z
0x166, // U-Z
0x136, // U-Z
0x13A, // - - %
0x12E, // - - %
0x1D4, // - - %
0x1D2, // - - %
0x1CA, // - - %
0x16E, // - - %
0x176, // - - %
0x1AE, // Control chars? $-*
0x126, // Control chars? $-*
0x1DA, // Control chars? $-*
0x1D6, // Control chars? $-*
0x132, // Control chars? $-*
0x15E, ]
;

 const ASTERISK_ENCODING: i32 = CHARACTER_ENCODINGS[47];
pub struct Code93Reader {
    super: OneDReader;

     let decode_row_result: StringBuilder;

     let mut counters: Vec<i32>;
}

impl Code93Reader {

    pub fn new() -> Code93Reader {
        decode_row_result = StringBuilder::new(20);
        counters = : [i32; 6] = [0; 6];
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
         let start: Vec<i32> = self.find_asterisk_pattern(row);
        // Read off white space
         let next_start: i32 = row.get_next_set(start[1]);
         let end: i32 = row.get_size();
         let the_counters: Vec<i32> = self.counters;
        Arrays::fill(&the_counters, 0);
         let result: StringBuilder = self.decode_row_result;
        result.set_length(0);
         let decoded_char: char;
         let last_start: i32;
        loop { {
            record_pattern(row, next_start, &the_counters);
             let pattern: i32 = ::to_pattern(&the_counters);
            if pattern < 0 {
                throw NotFoundException::get_not_found_instance();
            }
            decoded_char = ::pattern_to_char(pattern);
            result.append(decoded_char);
            last_start = next_start;
            for  let counter: i32 in the_counters {
                next_start += counter;
            }
            // Read off white space
            next_start = row.get_next_set(next_start);
        }if !(decoded_char != '*') break;}
        // remove asterisk
        result.delete_char_at(result.length() - 1);
         let last_pattern_size: i32 = 0;
        for  let counter: i32 in the_counters {
            last_pattern_size += counter;
        }
        // Should be at least one more black module
        if next_start == end || !row.get(next_start) {
            throw NotFoundException::get_not_found_instance();
        }
        if result.length() < 2 {
            // false positive -- need at least 2 checksum digits
            throw NotFoundException::get_not_found_instance();
        }
        ::check_checksums(&result);
        // Remove checksum digits
        result.set_length(result.length() - 2);
         let result_string: String = ::decode_extended(&result);
         let left: f32 = (start[1] + start[0]) / 2.0f;
         let right: f32 = last_start + last_pattern_size / 2.0f;
         let result_object: Result = Result::new(&result_string, null,  : vec![ResultPoint; 2] = vec![ResultPoint::new(left, row_number), ResultPoint::new(right, row_number), ]
        , BarcodeFormat::CODE_93);
        result_object.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, "]G0");
        return Ok(result_object);
    }

    fn  find_asterisk_pattern(&self,  row: &BitArray) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let width: i32 = row.get_size();
         let row_offset: i32 = row.get_next_set(0);
        Arrays::fill(&self.counters, 0);
         let the_counters: Vec<i32> = self.counters;
         let pattern_start: i32 = row_offset;
         let is_white: bool = false;
         let pattern_length: i32 = the_counters.len();
         let counter_position: i32 = 0;
         {
             let mut i: i32 = row_offset;
            while i < width {
                {
                    if row.get(i) != is_white {
                        the_counters[counter_position] += 1;
                    } else {
                        if counter_position == pattern_length - 1 {
                            if ::to_pattern(&the_counters) == ASTERISK_ENCODING {
                                return Ok( : vec![i32; 2] = vec![pattern_start, i, ]
                                );
                            }
                            pattern_start += the_counters[0] + the_counters[1];
                            System::arraycopy(&the_counters, 2, &the_counters, 0, counter_position - 1);
                            the_counters[counter_position - 1] = 0;
                            the_counters[counter_position] = 0;
                            counter_position -= 1;
                        } else {
                            counter_position += 1;
                        }
                        the_counters[counter_position] = 1;
                        is_white = !is_white;
                    }
                }
                i += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    fn  to_pattern( counters: &Vec<i32>) -> i32  {
         let mut sum: i32 = 0;
        for  let counter: i32 in counters {
            sum += counter;
        }
         let mut pattern: i32 = 0;
         let max: i32 = counters.len();
         {
             let mut i: i32 = 0;
            while i < max {
                {
                     let scaled: i32 = Math::round(counters[i] * 9.0f / sum);
                    if scaled < 1 || scaled > 4 {
                        return -1;
                    }
                    if (i & 0x01) == 0 {
                         {
                             let mut j: i32 = 0;
                            while j < scaled {
                                {
                                    pattern = (pattern << 1) | 0x01;
                                }
                                j += 1;
                             }
                         }

                    } else {
                        pattern <<= scaled;
                    }
                }
                i += 1;
             }
         }

        return pattern;
    }

    fn  pattern_to_char( pattern: i32) -> /*  throws NotFoundException */Result<char, Rc<Exception>>   {
         {
             let mut i: i32 = 0;
            while i < CHARACTER_ENCODINGS.len() {
                {
                    if CHARACTER_ENCODINGS[i] == pattern {
                        return Ok(ALPHABET[i]);
                    }
                }
                i += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    fn  decode_extended( encoded: &CharSequence) -> /*  throws FormatException */Result<String, Rc<Exception>>   {
         let length: i32 = encoded.length();
         let decoded: StringBuilder = StringBuilder::new(length);
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let c: char = encoded.char_at(i);
                    if c >= 'a' && c <= 'd' {
                        if i >= length - 1 {
                            throw FormatException::get_format_instance();
                        }
                         let next: char = encoded.char_at(i + 1);
                         let decoded_char: char = '\0';
                        match c {
                              'd' => 
                                 {
                                    // +A to +Z map to a to z
                                    if next >= 'A' && next <= 'Z' {
                                        decoded_char = (next + 32) as char;
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              'a' => 
                                 {
                                    // $A to $Z map to control codes SH to SB
                                    if next >= 'A' && next <= 'Z' {
                                        decoded_char = (next - 64) as char;
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              'b' => 
                                 {
                                    if next >= 'A' && next <= 'E' {
                                        // %A to %E map to control codes ESC to USep
                                        decoded_char = (next - 38) as char;
                                    } else if next >= 'F' && next <= 'J' {
                                        // %F to %J map to ; < = > ?
                                        decoded_char = (next - 11) as char;
                                    } else if next >= 'K' && next <= 'O' {
                                        // %K to %O map to [ \ ] ^ _
                                        decoded_char = (next + 16) as char;
                                    } else if next >= 'P' && next <= 'T' {
                                        // %P to %T map to { | } ~ DEL
                                        decoded_char = (next + 43) as char;
                                    } else if next == 'U' {
                                        // %U map to NUL
                                        decoded_char = '\0';
                                    } else if next == 'V' {
                                        // %V map to @
                                        decoded_char = '@';
                                    } else if next == 'W' {
                                        // %W map to `
                                        decoded_char = '`';
                                    } else if next >= 'X' && next <= 'Z' {
                                        // %X to %Z all map to DEL (127)
                                        decoded_char = 127;
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              'c' => 
                                 {
                                    // /A to /O map to ! to , and /Z maps to :
                                    if next >= 'A' && next <= 'O' {
                                        decoded_char = (next - 32) as char;
                                    } else if next == 'Z' {
                                        decoded_char = ':';
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                        }
                        decoded.append(decoded_char);
                        // bump up i again since we read two characters
                        i += 1;
                    } else {
                        decoded.append(c);
                    }
                }
                i += 1;
             }
         }

        return Ok(decoded.to_string());
    }

    fn  check_checksums( result: &CharSequence)  -> /*  throws ChecksumException */Result<Void, Rc<Exception>>   {
         let length: i32 = result.length();
        ::check_one_checksum(&result, length - 2, 20);
        ::check_one_checksum(&result, length - 1, 15);
    }

    fn  check_one_checksum( result: &CharSequence,  check_position: i32,  weight_max: i32)  -> /*  throws ChecksumException */Result<Void, Rc<Exception>>   {
         let mut weight: i32 = 1;
         let mut total: i32 = 0;
         {
             let mut i: i32 = check_position - 1;
            while i >= 0 {
                {
                    total += weight * ALPHABET_STRING::index_of(&result.char_at(i));
                    if weight += 1 > weight_max {
                        weight = 1;
                    }
                }
                i -= 1;
             }
         }

        if result.char_at(check_position) != ALPHABET[total % 47] {
            throw ChecksumException::get_checksum_instance();
        }
    }
}

