/*
 * Copyright 2008 ZXing authors
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
 * <p>Decodes Code 39 barcodes. Supports "Full ASCII Code 39" if USE_CODE_39_EXTENDED_MODE is set.</p>
 *
 * @author Sean Owen
 * @see Code93Reader
 */

 const ALPHABET_STRING: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";

/**
   * These represent the encodings of characters, as patterns of wide and narrow bars.
   * The 9 least-significant bits of each int correspond to the pattern of wide and narrow,
   * with 1s representing "wide" and 0s representing narrow.
   */
 const CHARACTER_ENCODINGS: vec![Vec<i32>; 43] = vec![// 0-9
0x034, // 0-9
0x121, // 0-9
0x061, // 0-9
0x160, // 0-9
0x031, // 0-9
0x130, // 0-9
0x070, // 0-9
0x025, // 0-9
0x124, // 0-9
0x064, // A-J
0x109, // A-J
0x049, // A-J
0x148, // A-J
0x019, // A-J
0x118, // A-J
0x058, // A-J
0x00D, // A-J
0x10C, // A-J
0x04C, // A-J
0x01C, // K-T
0x103, // K-T
0x043, // K-T
0x142, // K-T
0x013, // K-T
0x112, // K-T
0x052, // K-T
0x007, // K-T
0x106, // K-T
0x046, // K-T
0x016, // U-$
0x181, // U-$
0x0C1, // U-$
0x1C0, // U-$
0x091, // U-$
0x190, // U-$
0x0D0, // U-$
0x085, // U-$
0x184, // U-$
0x0C4, // U-$
0x0A8, // /-%
0x0A2, // /-%
0x08A, // /-%
0x02A, ]
;

 const ASTERISK_ENCODING: i32 = 0x094;
pub struct Code39Reader {
    super: OneDReader;

     let using_check_digit: bool;

     let extended_mode: bool;

     let decode_row_result: StringBuilder;

     let mut counters: Vec<i32>;
}

impl Code39Reader {

    /**
   * Creates a reader that assumes all encoded data is data, and does not treat the final
   * character as a check digit. It will not decoded "extended Code 39" sequences.
   */
    pub fn new() -> Code39Reader {
        this(false);
    }

    /**
   * Creates a reader that can be configured to check the last character as a check digit.
   * It will not decoded "extended Code 39" sequences.
   *
   * @param usingCheckDigit if true, treat the last data character as a check digit, not
   * data, and verify that the checksum passes.
   */
    pub fn new( using_check_digit: bool) -> Code39Reader {
        this(using_check_digit, false);
    }

    /**
   * Creates a reader that can be configured to check the last character as a check digit,
   * or optionally attempt to decode "extended Code 39" sequences that are used to encode
   * the full ASCII character set.
   *
   * @param usingCheckDigit if true, treat the last data character as a check digit, not
   * data, and verify that the checksum passes.
   * @param extendedMode if true, will attempt to decode extended Code 39 sequences in the
   * text.
   */
    pub fn new( using_check_digit: bool,  extended_mode: bool) -> Code39Reader {
        let .usingCheckDigit = using_check_digit;
        let .extendedMode = extended_mode;
        decode_row_result = StringBuilder::new(20);
        counters = : [i32; 9] = [0; 9];
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
         let the_counters: Vec<i32> = self.counters;
        Arrays::fill(&the_counters, 0);
         let result: StringBuilder = self.decode_row_result;
        result.set_length(0);
         let start: Vec<i32> = ::find_asterisk_pattern(row, &the_counters);
        // Read off white space
         let next_start: i32 = row.get_next_set(start[1]);
         let end: i32 = row.get_size();
         let decoded_char: char;
         let last_start: i32;
        loop { {
            record_pattern(row, next_start, &the_counters);
             let pattern: i32 = ::to_narrow_wide_pattern(&the_counters);
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
        result.set_length(result.length() - 1);
        // Look for whitespace after pattern:
         let last_pattern_size: i32 = 0;
        for  let counter: i32 in the_counters {
            last_pattern_size += counter;
        }
         let white_space_after_end: i32 = next_start - last_start - last_pattern_size;
        // (but if it's whitespace to the very end of the image, that's OK)
        if next_start != end && (white_space_after_end * 2) < last_pattern_size {
            throw NotFoundException::get_not_found_instance();
        }
        if self.using_check_digit {
             let max: i32 = result.length() - 1;
             let mut total: i32 = 0;
             {
                 let mut i: i32 = 0;
                while i < max {
                    {
                        total += ALPHABET_STRING::index_of(&self.decode_row_result.char_at(i));
                    }
                    i += 1;
                 }
             }

            if result.char_at(max) != ALPHABET_STRING::char_at(total % 43) {
                throw ChecksumException::get_checksum_instance();
            }
            result.set_length(max);
        }
        if result.length() == 0 {
            // false positive
            throw NotFoundException::get_not_found_instance();
        }
         let result_string: String;
        if self.extended_mode {
            result_string = ::decode_extended(&result);
        } else {
            result_string = result.to_string();
        }
         let left: f32 = (start[1] + start[0]) / 2.0f;
         let right: f32 = last_start + last_pattern_size / 2.0f;
         let result_object: Result = Result::new(&result_string, null,  : vec![ResultPoint; 2] = vec![ResultPoint::new(left, row_number), ResultPoint::new(right, row_number), ]
        , BarcodeFormat::CODE_39);
        result_object.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, "]A0");
        return Ok(result_object);
    }

    fn  find_asterisk_pattern( row: &BitArray,  counters: &Vec<i32>) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let width: i32 = row.get_size();
         let row_offset: i32 = row.get_next_set(0);
         let counter_position: i32 = 0;
         let pattern_start: i32 = row_offset;
         let is_white: bool = false;
         let pattern_length: i32 = counters.len();
         {
             let mut i: i32 = row_offset;
            while i < width {
                {
                    if row.get(i) != is_white {
                        counters[counter_position] += 1;
                    } else {
                        if counter_position == pattern_length - 1 {
                            // Look for whitespace before start pattern, >= 50% of width of start pattern
                            if ::to_narrow_wide_pattern(&counters) == ASTERISK_ENCODING && row.is_range(&Math::max(0, pattern_start - ((i - pattern_start) / 2)), pattern_start, false) {
                                return Ok( : vec![i32; 2] = vec![pattern_start, i, ]
                                );
                            }
                            pattern_start += counters[0] + counters[1];
                            System::arraycopy(&counters, 2, &counters, 0, counter_position - 1);
                            counters[counter_position - 1] = 0;
                            counters[counter_position] = 0;
                            counter_position -= 1;
                        } else {
                            counter_position += 1;
                        }
                        counters[counter_position] = 1;
                        is_white = !is_white;
                    }
                }
                i += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    // For efficiency, returns -1 on failure. Not throwing here saved as many as 700 exceptions
    // per image when using some of our blackbox images.
    fn  to_narrow_wide_pattern( counters: &Vec<i32>) -> i32  {
         let num_counters: i32 = counters.len();
         let max_narrow_counter: i32 = 0;
         let wide_counters: i32;
        loop { {
             let min_counter: i32 = Integer::MAX_VALUE;
            for  let counter: i32 in counters {
                if counter < min_counter && counter > max_narrow_counter {
                    min_counter = counter;
                }
            }
            max_narrow_counter = min_counter;
            wide_counters = 0;
             let total_wide_counters_width: i32 = 0;
             let mut pattern: i32 = 0;
             {
                 let mut i: i32 = 0;
                while i < num_counters {
                    {
                         let counter: i32 = counters[i];
                        if counter > max_narrow_counter {
                            pattern |= 1 << (num_counters - 1 - i);
                            wide_counters += 1;
                            total_wide_counters_width += counter;
                        }
                    }
                    i += 1;
                 }
             }

            if wide_counters == 3 {
                // counter is more than 1.5 times the average:
                 {
                     let mut i: i32 = 0;
                    while i < num_counters && wide_counters > 0 {
                        {
                             let counter: i32 = counters[i];
                            if counter > max_narrow_counter {
                                wide_counters -= 1;
                                // totalWideCountersWidth = 3 * average, so this checks if counter >= 3/2 * average
                                if (counter * 2) >= total_wide_counters_width {
                                    return -1;
                                }
                            }
                        }
                        i += 1;
                     }
                 }

                return pattern;
            }
        }if !(wide_counters > 3) break;}
        return -1;
    }

    fn  pattern_to_char( pattern: i32) -> /*  throws NotFoundException */Result<char, Rc<Exception>>   {
         {
             let mut i: i32 = 0;
            while i < CHARACTER_ENCODINGS.len() {
                {
                    if CHARACTER_ENCODINGS[i] == pattern {
                        return Ok(ALPHABET_STRING::char_at(i));
                    }
                }
                i += 1;
             }
         }

        if pattern == ASTERISK_ENCODING {
            return Ok('*');
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
                    if c == '+' || c == '$' || c == '%' || c == '/' {
                         let next: char = encoded.char_at(i + 1);
                         let decoded_char: char = '\0';
                        match c {
                              '+' => 
                                 {
                                    // +A to +Z map to a to z
                                    if next >= 'A' && next <= 'Z' {
                                        decoded_char = (next + 32) as char;
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              '$' => 
                                 {
                                    // $A to $Z map to control codes SH to SB
                                    if next >= 'A' && next <= 'Z' {
                                        decoded_char = (next - 64) as char;
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              '%' => 
                                 {
                                    // %A to %E map to control codes ESC to US
                                    if next >= 'A' && next <= 'E' {
                                        decoded_char = (next - 38) as char;
                                    } else if next >= 'F' && next <= 'J' {
                                        decoded_char = (next - 11) as char;
                                    } else if next >= 'K' && next <= 'O' {
                                        decoded_char = (next + 16) as char;
                                    } else if next >= 'P' && next <= 'T' {
                                        decoded_char = (next + 43) as char;
                                    } else if next == 'U' {
                                        decoded_char = 0 as char;
                                    } else if next == 'V' {
                                        decoded_char = '@';
                                    } else if next == 'W' {
                                        decoded_char = '`';
                                    } else if next == 'X' || next == 'Y' || next == 'Z' {
                                        decoded_char = 127 as char;
                                    } else {
                                        throw FormatException::get_format_instance();
                                    }
                                    break;
                                }
                              '/' => 
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
}

