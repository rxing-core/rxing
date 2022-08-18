use crate::{BarcodeFormat,DecodeHintType,NotFoundException,XRingResult,ResultMetadataType,ResultPoint,ChecksumException,FormatException,Reader,ReaderException,EncodeHintType,Writer,BinaryBitmap,ResultPointCallback};
use crate::common::{BitArray,BitMatrix};
use crate::oned::rss::{RSS14Reader};
use crate::oned::rss::expanded::{RSSExpandedReader};

// NEW FILE: coda_bar_reader.rs
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
 * <p>Decodes Codabar barcodes.</p>
 *
 * @author Bas Vijfwinkel
 * @author David Walker
 */

// These values are critical for determining how permissive the decoding
// will be. All stripe sizes must be within the window these define, as
// compared to the average stripe size.
 const MAX_ACCEPTABLE: f32 = 2.0f;

 const PADDING: f32 = 1.5f;

 const ALPHABET_STRING: &'static str = "0123456789-$:/.+ABCD";

 const ALPHABET: Vec<char> = ALPHABET_STRING::to_char_array();

/**
   * These represent the encodings of characters, as patterns of wide and narrow bars. The 7 least-significant bits of
   * each int correspond to the pattern of wide and narrow, with 1s representing "wide" and 0s representing narrow.
   */
 const CHARACTER_ENCODINGS: vec![Vec<i32>; 20] = vec![// 0-9
0x003, // 0-9
0x006, // 0-9
0x009, // 0-9
0x060, // 0-9
0x012, // 0-9
0x042, // 0-9
0x021, // 0-9
0x024, // 0-9
0x030, // 0-9
0x048, // -$:/.+ABCD
0x00c, // -$:/.+ABCD
0x018, // -$:/.+ABCD
0x045, // -$:/.+ABCD
0x051, // -$:/.+ABCD
0x054, // -$:/.+ABCD
0x015, // -$:/.+ABCD
0x01A, // -$:/.+ABCD
0x029, // -$:/.+ABCD
0x00B, // -$:/.+ABCD
0x00E, ]
;

// minimal number of characters that should be present (including start and stop characters)
// under normal circumstances this should be set to 3, but can be set higher
// as a last-ditch attempt to reduce false positives.
 const MIN_CHARACTER_LENGTH: i32 = 3;

// official start and end patterns
 const STARTEND_ENCODING: vec![Vec<char>; 4] = vec!['A', 'B', 'C', 'D', ]
;
pub struct CodaBarReader {
    super: OneDReader;

    // some Codabar generator allow the Codabar string to be closed by every
    // character. This will cause lots of false positives!
    // some industries use a checksum standard but this is not part of the original Codabar standard
    // for more information see : http://www.mecsw.com/specs/codabar.html
    // Keep some instance variables to avoid reallocations
     let decode_row_result: StringBuilder;

     let mut counters: Vec<i32>;

     let counter_length: i32;
}

impl CodaBarReader {

    pub fn new() -> CodaBarReader {
        decode_row_result = StringBuilder::new(20);
        counters = : [i32; 80] = [0; 80];
        counter_length = 0;
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        Arrays::fill(&self.counters, 0);
        self.set_counters(row);
         let start_offset: i32 = self.find_start_pattern();
         let next_start: i32 = start_offset;
        self.decode_row_result.set_length(0);
        loop { {
             let char_offset: i32 = self.to_narrow_wide_pattern(next_start);
            if char_offset == -1 {
                throw NotFoundException::get_not_found_instance();
            }
            // Hack: We store the position in the alphabet table into a
            // StringBuilder, so that we can access the decoded patterns in
            // validatePattern. We'll translate to the actual characters later.
            self.decode_row_result.append(char_offset as char);
            next_start += 8;
            // Stop as soon as we see the end character.
            if self.decode_row_result.length() > 1 && ::array_contains(&STARTEND_ENCODING, ALPHABET[char_offset]) {
                break;
            }
        }if !(// no fixed end pattern so keep on reading while data is available
        next_start < self.counter_length) break;}
        // Look for whitespace after pattern:
         let trailing_whitespace: i32 = self.counters[next_start - 1];
         let last_pattern_size: i32 = 0;
         {
             let mut i: i32 = -8;
            while i < -1 {
                {
                    last_pattern_size += self.counters[next_start + i];
                }
                i += 1;
             }
         }

        // at the end of the row. (I.e. the barcode barely fits.)
        if next_start < self.counter_length && trailing_whitespace < last_pattern_size / 2 {
            throw NotFoundException::get_not_found_instance();
        }
        self.validate_pattern(start_offset);
        // Translate character table offsets to actual characters.
         {
             let mut i: i32 = 0;
            while i < self.decode_row_result.length() {
                {
                    self.decode_row_result.set_char_at(i, ALPHABET[self.decode_row_result.char_at(i)]);
                }
                i += 1;
             }
         }

        // Ensure a valid start and end character
         let startchar: char = self.decode_row_result.char_at(0);
        if !::array_contains(&STARTEND_ENCODING, startchar) {
            throw NotFoundException::get_not_found_instance();
        }
         let endchar: char = self.decode_row_result.char_at(self.decode_row_result.length() - 1);
        if !::array_contains(&STARTEND_ENCODING, endchar) {
            throw NotFoundException::get_not_found_instance();
        }
        // remove stop/start characters character and check if a long enough string is contained
        if self.decode_row_result.length() <= MIN_CHARACTER_LENGTH {
            // Almost surely a false positive ( start + stop + at least 1 character)
            throw NotFoundException::get_not_found_instance();
        }
        if hints == null || !hints.contains_key(DecodeHintType::RETURN_CODABAR_START_END) {
            self.decode_row_result.delete_char_at(self.decode_row_result.length() - 1);
            self.decode_row_result.delete_char_at(0);
        }
         let running_count: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < start_offset {
                {
                    running_count += self.counters[i];
                }
                i += 1;
             }
         }

         let left: f32 = running_count;
         {
             let mut i: i32 = start_offset;
            while i < next_start - 1 {
                {
                    running_count += self.counters[i];
                }
                i += 1;
             }
         }

         let right: f32 = running_count;
         let result: Result = Result::new(&self.decode_row_result.to_string(), null,  : vec![ResultPoint; 2] = vec![ResultPoint::new(left, row_number), ResultPoint::new(right, row_number), ]
        , BarcodeFormat::CODABAR);
        result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, "]F0");
        return Ok(result);
    }

    fn  validate_pattern(&self,  start: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
        // First, sum up the total size of our four categories of stripe sizes;
         let mut sizes: vec![Vec<i32>; 4] = vec![0, 0, 0, 0, ]
        ;
         let mut counts: vec![Vec<i32>; 4] = vec![0, 0, 0, 0, ]
        ;
         let end: i32 = self.decode_row_result.length() - 1;
        // We break out of this loop in the middle, in order to handle
        // inter-character spaces properly.
         let mut pos: i32 = start;
         {
             let mut i: i32 = 0;
            while i <= end {
                {
                     let mut pattern: i32 = CHARACTER_ENCODINGS[self.decode_row_result.char_at(i)];
                     {
                         let mut j: i32 = 6;
                        while j >= 0 {
                            {
                                // Even j = bars, while odd j = spaces. Categories 2 and 3 are for
                                // long stripes, while 0 and 1 are for short stripes.
                                 let mut category: i32 = (j & 1) + (pattern & 1) * 2;
                                sizes[category] += self.counters[pos + j];
                                counts[category] += 1;
                                pattern >>= 1;
                            }
                            j -= 1;
                         }
                     }

                    // We ignore the inter-character space - it could be of any size.
                    pos += 8;
                }
                i += 1;
             }
         }

        // Calculate our allowable size thresholds using fixed-point math.
         let mut maxes: [f32; 4.0] = [0.0; 4.0];
         let mut mins: [f32; 4.0] = [0.0; 4.0];
        // should be on the "wrong" side of that line.
         {
             let mut i: i32 = 0;
            while i < 2 {
                {
                    // Accept arbitrarily small "short" stripes.
                    mins[i] = 0.0f;
                    mins[i + 2] = (sizes[i] as f32 / counts[i] + sizes[i + 2] as f32 / counts[i + 2]) / 2.0f;
                    maxes[i] = mins[i + 2];
                    maxes[i + 2] = (sizes[i + 2] * MAX_ACCEPTABLE + PADDING) / counts[i + 2];
                }
                i += 1;
             }
         }

        // Now verify that all of the stripes are within the thresholds.
        pos = start;
         {
             let mut i: i32 = 0;
            while i <= end {
                {
                     let mut pattern: i32 = CHARACTER_ENCODINGS[self.decode_row_result.char_at(i)];
                     {
                         let mut j: i32 = 6;
                        while j >= 0 {
                            {
                                // Even j = bars, while odd j = spaces. Categories 2 and 3 are for
                                // long stripes, while 0 and 1 are for short stripes.
                                 let category: i32 = (j & 1) + (pattern & 1) * 2;
                                 let size: i32 = self.counters[pos + j];
                                if size < mins[category] || size > maxes[category] {
                                    throw NotFoundException::get_not_found_instance();
                                }
                                pattern >>= 1;
                            }
                            j -= 1;
                         }
                     }

                    pos += 8;
                }
                i += 1;
             }
         }

    }

    /**
   * Records the size of all runs of white and black pixels, starting with white.
   * This is just like recordPattern, except it records all the counters, and
   * uses our builtin "counters" member for storage.
   * @param row row to count from
   */
    fn  set_counters(&self,  row: &BitArray)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
        self.counter_length = 0;
        // Start from the first white bit.
         let mut i: i32 = row.get_next_unset(0);
         let end: i32 = row.get_size();
        if i >= end {
            throw NotFoundException::get_not_found_instance();
        }
         let is_white: bool = true;
         let mut count: i32 = 0;
        while i < end {
            if row.get(i) != is_white {
                count += 1;
            } else {
                self.counter_append(count);
                count = 1;
                is_white = !is_white;
            }
            i += 1;
        }
        self.counter_append(count);
    }

    fn  counter_append(&self,  e: i32)   {
        self.counters[self.counter_length] = e;
        self.counter_length += 1;
        if self.counter_length >= self.counters.len() {
             let temp: [i32; self.counter_length * 2] = [0; self.counter_length * 2];
            System::arraycopy(&self.counters, 0, &temp, 0, self.counter_length);
            self.counters = temp;
        }
    }

    fn  find_start_pattern(&self) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         {
             let mut i: i32 = 1;
            while i < self.counter_length {
                {
                     let char_offset: i32 = self.to_narrow_wide_pattern(i);
                    if char_offset != -1 && ::array_contains(&STARTEND_ENCODING, ALPHABET[char_offset]) {
                        // Look for whitespace before start pattern, >= 50% of width of start pattern
                        // We make an exception if the whitespace is the first element.
                         let pattern_size: i32 = 0;
                         {
                             let mut j: i32 = i;
                            while j < i + 7 {
                                {
                                    pattern_size += self.counters[j];
                                }
                                j += 1;
                             }
                         }

                        if i == 1 || self.counters[i - 1] >= pattern_size / 2 {
                            return Ok(i);
                        }
                    }
                }
                i += 2;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    fn  array_contains( array: &Vec<char>,  key: char) -> bool  {
        if array != null {
            for  let c: char in array {
                if c == key {
                    return true;
                }
            }
        }
        return false;
    }

    // Assumes that counters[position] is a bar.
    fn  to_narrow_wide_pattern(&self,  position: i32) -> i32  {
         let end: i32 = position + 7;
        if end >= self.counter_length {
            return -1;
        }
         let the_counters: Vec<i32> = self.counters;
         let max_bar: i32 = 0;
         let min_bar: i32 = Integer::MAX_VALUE;
         {
             let mut j: i32 = position;
            while j < end {
                {
                     let current_counter: i32 = the_counters[j];
                    if current_counter < min_bar {
                        min_bar = current_counter;
                    }
                    if current_counter > max_bar {
                        max_bar = current_counter;
                    }
                }
                j += 2;
             }
         }

         let threshold_bar: i32 = (min_bar + max_bar) / 2;
         let max_space: i32 = 0;
         let min_space: i32 = Integer::MAX_VALUE;
         {
             let mut j: i32 = position + 1;
            while j < end {
                {
                     let current_counter: i32 = the_counters[j];
                    if current_counter < min_space {
                        min_space = current_counter;
                    }
                    if current_counter > max_space {
                        max_space = current_counter;
                    }
                }
                j += 2;
             }
         }

         let threshold_space: i32 = (min_space + max_space) / 2;
         let mut bitmask: i32 = 1 << 7;
         let mut pattern: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 7 {
                {
                     let threshold: i32 =  if (i & 1) == 0 { threshold_bar } else { threshold_space };
                    bitmask >>= 1;
                    if the_counters[position + i] > threshold {
                        pattern |= bitmask;
                    }
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 0;
            while i < CHARACTER_ENCODINGS.len() {
                {
                    if CHARACTER_ENCODINGS[i] == pattern {
                        return i;
                    }
                }
                i += 1;
             }
         }

        return -1;
    }
}

// NEW FILE: coda_bar_writer.rs
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

// NEW FILE: code128_reader.rs
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
 * <p>Decodes Code 128 barcodes.</p>
 *
 * @author Sean Owen
 */

 const CODE_PATTERNS: vec![vec![Vec<Vec<i32>>; 7]; 107] = vec![// 0
vec![2, 1, 2, 2, 2, 2, ]
, vec![2, 2, 2, 1, 2, 2, ]
, vec![2, 2, 2, 2, 2, 1, ]
, vec![1, 2, 1, 2, 2, 3, ]
, vec![1, 2, 1, 3, 2, 2, ]
, // 5
vec![1, 3, 1, 2, 2, 2, ]
, vec![1, 2, 2, 2, 1, 3, ]
, vec![1, 2, 2, 3, 1, 2, ]
, vec![1, 3, 2, 2, 1, 2, ]
, vec![2, 2, 1, 2, 1, 3, ]
, // 10
vec![2, 2, 1, 3, 1, 2, ]
, vec![2, 3, 1, 2, 1, 2, ]
, vec![1, 1, 2, 2, 3, 2, ]
, vec![1, 2, 2, 1, 3, 2, ]
, vec![1, 2, 2, 2, 3, 1, ]
, // 15
vec![1, 1, 3, 2, 2, 2, ]
, vec![1, 2, 3, 1, 2, 2, ]
, vec![1, 2, 3, 2, 2, 1, ]
, vec![2, 2, 3, 2, 1, 1, ]
, vec![2, 2, 1, 1, 3, 2, ]
, // 20
vec![2, 2, 1, 2, 3, 1, ]
, vec![2, 1, 3, 2, 1, 2, ]
, vec![2, 2, 3, 1, 1, 2, ]
, vec![3, 1, 2, 1, 3, 1, ]
, vec![3, 1, 1, 2, 2, 2, ]
, // 25
vec![3, 2, 1, 1, 2, 2, ]
, vec![3, 2, 1, 2, 2, 1, ]
, vec![3, 1, 2, 2, 1, 2, ]
, vec![3, 2, 2, 1, 1, 2, ]
, vec![3, 2, 2, 2, 1, 1, ]
, // 30
vec![2, 1, 2, 1, 2, 3, ]
, vec![2, 1, 2, 3, 2, 1, ]
, vec![2, 3, 2, 1, 2, 1, ]
, vec![1, 1, 1, 3, 2, 3, ]
, vec![1, 3, 1, 1, 2, 3, ]
, // 35
vec![1, 3, 1, 3, 2, 1, ]
, vec![1, 1, 2, 3, 1, 3, ]
, vec![1, 3, 2, 1, 1, 3, ]
, vec![1, 3, 2, 3, 1, 1, ]
, vec![2, 1, 1, 3, 1, 3, ]
, // 40
vec![2, 3, 1, 1, 1, 3, ]
, vec![2, 3, 1, 3, 1, 1, ]
, vec![1, 1, 2, 1, 3, 3, ]
, vec![1, 1, 2, 3, 3, 1, ]
, vec![1, 3, 2, 1, 3, 1, ]
, // 45
vec![1, 1, 3, 1, 2, 3, ]
, vec![1, 1, 3, 3, 2, 1, ]
, vec![1, 3, 3, 1, 2, 1, ]
, vec![3, 1, 3, 1, 2, 1, ]
, vec![2, 1, 1, 3, 3, 1, ]
, // 50
vec![2, 3, 1, 1, 3, 1, ]
, vec![2, 1, 3, 1, 1, 3, ]
, vec![2, 1, 3, 3, 1, 1, ]
, vec![2, 1, 3, 1, 3, 1, ]
, vec![3, 1, 1, 1, 2, 3, ]
, // 55
vec![3, 1, 1, 3, 2, 1, ]
, vec![3, 3, 1, 1, 2, 1, ]
, vec![3, 1, 2, 1, 1, 3, ]
, vec![3, 1, 2, 3, 1, 1, ]
, vec![3, 3, 2, 1, 1, 1, ]
, // 60
vec![3, 1, 4, 1, 1, 1, ]
, vec![2, 2, 1, 4, 1, 1, ]
, vec![4, 3, 1, 1, 1, 1, ]
, vec![1, 1, 1, 2, 2, 4, ]
, vec![1, 1, 1, 4, 2, 2, ]
, // 65
vec![1, 2, 1, 1, 2, 4, ]
, vec![1, 2, 1, 4, 2, 1, ]
, vec![1, 4, 1, 1, 2, 2, ]
, vec![1, 4, 1, 2, 2, 1, ]
, vec![1, 1, 2, 2, 1, 4, ]
, // 70
vec![1, 1, 2, 4, 1, 2, ]
, vec![1, 2, 2, 1, 1, 4, ]
, vec![1, 2, 2, 4, 1, 1, ]
, vec![1, 4, 2, 1, 1, 2, ]
, vec![1, 4, 2, 2, 1, 1, ]
, // 75
vec![2, 4, 1, 2, 1, 1, ]
, vec![2, 2, 1, 1, 1, 4, ]
, vec![4, 1, 3, 1, 1, 1, ]
, vec![2, 4, 1, 1, 1, 2, ]
, vec![1, 3, 4, 1, 1, 1, ]
, // 80
vec![1, 1, 1, 2, 4, 2, ]
, vec![1, 2, 1, 1, 4, 2, ]
, vec![1, 2, 1, 2, 4, 1, ]
, vec![1, 1, 4, 2, 1, 2, ]
, vec![1, 2, 4, 1, 1, 2, ]
, // 85
vec![1, 2, 4, 2, 1, 1, ]
, vec![4, 1, 1, 2, 1, 2, ]
, vec![4, 2, 1, 1, 1, 2, ]
, vec![4, 2, 1, 2, 1, 1, ]
, vec![2, 1, 2, 1, 4, 1, ]
, // 90
vec![2, 1, 4, 1, 2, 1, ]
, vec![4, 1, 2, 1, 2, 1, ]
, vec![1, 1, 1, 1, 4, 3, ]
, vec![1, 1, 1, 3, 4, 1, ]
, vec![1, 3, 1, 1, 4, 1, ]
, // 95
vec![1, 1, 4, 1, 1, 3, ]
, vec![1, 1, 4, 3, 1, 1, ]
, vec![4, 1, 1, 1, 1, 3, ]
, vec![4, 1, 1, 3, 1, 1, ]
, vec![1, 1, 3, 1, 4, 1, ]
, // 100
vec![1, 1, 4, 1, 3, 1, ]
, vec![3, 1, 1, 1, 4, 1, ]
, vec![4, 1, 1, 1, 3, 1, ]
, vec![2, 1, 1, 4, 1, 2, ]
, vec![2, 1, 1, 2, 1, 4, ]
, // 105
vec![2, 1, 1, 2, 3, 2, ]
, vec![2, 3, 3, 1, 1, 1, 2, ]
, ]
;

 const MAX_AVG_VARIANCE: f32 = 0.25f;

 const MAX_INDIVIDUAL_VARIANCE: f32 = 0.7f;

 const CODE_SHIFT: i32 = 98;

 const CODE_CODE_C: i32 = 99;

 const CODE_CODE_B: i32 = 100;

 const CODE_CODE_A: i32 = 101;

 const CODE_FNC_1: i32 = 102;

 const CODE_FNC_2: i32 = 97;

 const CODE_FNC_3: i32 = 96;

 const CODE_FNC_4_A: i32 = 101;

 const CODE_FNC_4_B: i32 = 100;

 const CODE_START_A: i32 = 103;

 const CODE_START_B: i32 = 104;

 const CODE_START_C: i32 = 105;

 const CODE_STOP: i32 = 106;
pub struct Code128Reader {
    super: OneDReader;
}

impl Code128Reader {

    fn  find_start_pattern( row: &BitArray) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let width: i32 = row.get_size();
         let row_offset: i32 = row.get_next_set(0);
         let counter_position: i32 = 0;
         let mut counters: [i32; 6] = [0; 6];
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
                             let best_variance: f32 = MAX_AVG_VARIANCE;
                             let best_match: i32 = -1;
                             {
                                 let start_code: i32 = CODE_START_A;
                                while start_code <= CODE_START_C {
                                    {
                                         let variance: f32 = pattern_match_variance(&counters, CODE_PATTERNS[start_code], MAX_INDIVIDUAL_VARIANCE);
                                        if variance < best_variance {
                                            best_variance = variance;
                                            best_match = start_code;
                                        }
                                    }
                                    start_code += 1;
                                 }
                             }

                            // Look for whitespace before start pattern, >= 50% of width of start pattern
                            if best_match >= 0 && row.is_range(&Math::max(0, pattern_start - (i - pattern_start) / 2), pattern_start, false) {
                                return Ok( : vec![i32; 3] = vec![pattern_start, i, best_match, ]
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

    fn  decode_code( row: &BitArray,  counters: &Vec<i32>,  row_offset: i32) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
        record_pattern(row, row_offset, &counters);
        // worst variance we'll accept
         let best_variance: f32 = MAX_AVG_VARIANCE;
         let best_match: i32 = -1;
         {
             let mut d: i32 = 0;
            while d < CODE_PATTERNS.len() {
                {
                     let pattern: Vec<i32> = CODE_PATTERNS[d];
                     let variance: f32 = pattern_match_variance(&counters, &pattern, MAX_INDIVIDUAL_VARIANCE);
                    if variance < best_variance {
                        best_variance = variance;
                        best_match = d;
                    }
                }
                d += 1;
             }
         }

        // TODO We're overlooking the fact that the STOP pattern has 7 values, not 6.
        if best_match >= 0 {
            return Ok(best_match);
        } else {
            throw NotFoundException::get_not_found_instance();
        }
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Result, Rc<Exception>>   {
         let convert_f_n_c1: bool = hints != null && hints.contains_key(DecodeHintType::ASSUME_GS1);
         let symbology_modifier: i32 = 0;
         let start_pattern_info: Vec<i32> = ::find_start_pattern(row);
         let start_code: i32 = start_pattern_info[2];
         let raw_codes: List<Byte> = ArrayList<>::new(20);
        raw_codes.add(start_code as i8);
         let code_set: i32;
        match start_code {
              CODE_START_A => 
                 {
                    code_set = CODE_CODE_A;
                    break;
                }
              CODE_START_B => 
                 {
                    code_set = CODE_CODE_B;
                    break;
                }
              CODE_START_C => 
                 {
                    code_set = CODE_CODE_C;
                    break;
                }
            _ => 
                 {
                    throw FormatException::get_format_instance();
                }
        }
         let mut done: bool = false;
         let is_next_shifted: bool = false;
         let result: StringBuilder = StringBuilder::new(20);
         let last_start: i32 = start_pattern_info[0];
         let next_start: i32 = start_pattern_info[1];
         let counters: [i32; 6] = [0; 6];
         let last_code: i32 = 0;
         let mut code: i32 = 0;
         let checksum_total: i32 = start_code;
         let mut multiplier: i32 = 0;
         let last_character_was_printable: bool = true;
         let upper_mode: bool = false;
         let shift_upper_mode: bool = false;
        while !done {
             let unshift: bool = is_next_shifted;
            is_next_shifted = false;
            // Save off last code
            last_code = code;
            // Decode another code from image
            code = ::decode_code(row, &counters, next_start);
            raw_codes.add(code as i8);
            // Remember whether the last code was printable or not (excluding CODE_STOP)
            if code != CODE_STOP {
                last_character_was_printable = true;
            }
            // Add to checksum computation (if not CODE_STOP of course)
            if code != CODE_STOP {
                multiplier += 1;
                checksum_total += multiplier * code;
            }
            // Advance to where the next code will to start
            last_start = next_start;
            for  let counter: i32 in counters {
                next_start += counter;
            }
            // Take care of illegal start codes
            match code {
                  CODE_START_A => 
                     {
                    }
                  CODE_START_B => 
                     {
                    }
                  CODE_START_C => 
                     {
                        throw FormatException::get_format_instance();
                    }
            }
            match code_set {
                  CODE_CODE_A => 
                     {
                        if code < 64 {
                            if shift_upper_mode == upper_mode {
                                result.append((' ' + code) as char);
                            } else {
                                result.append((' ' + code + 128) as char);
                            }
                            shift_upper_mode = false;
                        } else if code < 96 {
                            if shift_upper_mode == upper_mode {
                                result.append((code - 64) as char);
                            } else {
                                result.append((code + 64) as char);
                            }
                            shift_upper_mode = false;
                        } else {
                            // code was printable or not.
                            if code != CODE_STOP {
                                last_character_was_printable = false;
                            }
                            match code {
                                  CODE_FNC_1 => 
                                     {
                                        if result.length() == 0 {
                                            // FNC1 at first or second character determines the symbology
                                            symbology_modifier = 1;
                                        } else if result.length() == 1 {
                                            symbology_modifier = 2;
                                        }
                                        if convert_f_n_c1 {
                                            if result.length() == 0 {
                                                // GS1 specification 5.4.3.7. and 5.4.6.4. If the first char after the start code
                                                // is FNC1 then this is GS1-128. We add the symbology identifier.
                                                result.append("]C1");
                                            } else {
                                                // GS1 specification 5.4.7.5. Every subsequent FNC1 is returned as ASCII 29 (GS)
                                                result.append(29 as char);
                                            }
                                        }
                                        break;
                                    }
                                  CODE_FNC_2 => 
                                     {
                                        symbology_modifier = 4;
                                        break;
                                    }
                                  CODE_FNC_3 => 
                                     {
                                        // do nothing?
                                        break;
                                    }
                                  CODE_FNC_4_A => 
                                     {
                                        if !upper_mode && shift_upper_mode {
                                            upper_mode = true;
                                            shift_upper_mode = false;
                                        } else if upper_mode && shift_upper_mode {
                                            upper_mode = false;
                                            shift_upper_mode = false;
                                        } else {
                                            shift_upper_mode = true;
                                        }
                                        break;
                                    }
                                  CODE_SHIFT => 
                                     {
                                        is_next_shifted = true;
                                        code_set = CODE_CODE_B;
                                        break;
                                    }
                                  CODE_CODE_B => 
                                     {
                                        code_set = CODE_CODE_B;
                                        break;
                                    }
                                  CODE_CODE_C => 
                                     {
                                        code_set = CODE_CODE_C;
                                        break;
                                    }
                                  CODE_STOP => 
                                     {
                                        done = true;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  CODE_CODE_B => 
                     {
                        if code < 96 {
                            if shift_upper_mode == upper_mode {
                                result.append((' ' + code) as char);
                            } else {
                                result.append((' ' + code + 128) as char);
                            }
                            shift_upper_mode = false;
                        } else {
                            if code != CODE_STOP {
                                last_character_was_printable = false;
                            }
                            match code {
                                  CODE_FNC_1 => 
                                     {
                                        if result.length() == 0 {
                                            // FNC1 at first or second character determines the symbology
                                            symbology_modifier = 1;
                                        } else if result.length() == 1 {
                                            symbology_modifier = 2;
                                        }
                                        if convert_f_n_c1 {
                                            if result.length() == 0 {
                                                // GS1 specification 5.4.3.7. and 5.4.6.4. If the first char after the start code
                                                // is FNC1 then this is GS1-128. We add the symbology identifier.
                                                result.append("]C1");
                                            } else {
                                                // GS1 specification 5.4.7.5. Every subsequent FNC1 is returned as ASCII 29 (GS)
                                                result.append(29 as char);
                                            }
                                        }
                                        break;
                                    }
                                  CODE_FNC_2 => 
                                     {
                                        symbology_modifier = 4;
                                        break;
                                    }
                                  CODE_FNC_3 => 
                                     {
                                        // do nothing?
                                        break;
                                    }
                                  CODE_FNC_4_B => 
                                     {
                                        if !upper_mode && shift_upper_mode {
                                            upper_mode = true;
                                            shift_upper_mode = false;
                                        } else if upper_mode && shift_upper_mode {
                                            upper_mode = false;
                                            shift_upper_mode = false;
                                        } else {
                                            shift_upper_mode = true;
                                        }
                                        break;
                                    }
                                  CODE_SHIFT => 
                                     {
                                        is_next_shifted = true;
                                        code_set = CODE_CODE_A;
                                        break;
                                    }
                                  CODE_CODE_A => 
                                     {
                                        code_set = CODE_CODE_A;
                                        break;
                                    }
                                  CODE_CODE_C => 
                                     {
                                        code_set = CODE_CODE_C;
                                        break;
                                    }
                                  CODE_STOP => 
                                     {
                                        done = true;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  CODE_CODE_C => 
                     {
                        if code < 100 {
                            if code < 10 {
                                result.append('0');
                            }
                            result.append(code);
                        } else {
                            if code != CODE_STOP {
                                last_character_was_printable = false;
                            }
                            match code {
                                  CODE_FNC_1 => 
                                     {
                                        if result.length() == 0 {
                                            // FNC1 at first or second character determines the symbology
                                            symbology_modifier = 1;
                                        } else if result.length() == 1 {
                                            symbology_modifier = 2;
                                        }
                                        if convert_f_n_c1 {
                                            if result.length() == 0 {
                                                // GS1 specification 5.4.3.7. and 5.4.6.4. If the first char after the start code
                                                // is FNC1 then this is GS1-128. We add the symbology identifier.
                                                result.append("]C1");
                                            } else {
                                                // GS1 specification 5.4.7.5. Every subsequent FNC1 is returned as ASCII 29 (GS)
                                                result.append(29 as char);
                                            }
                                        }
                                        break;
                                    }
                                  CODE_CODE_A => 
                                     {
                                        code_set = CODE_CODE_A;
                                        break;
                                    }
                                  CODE_CODE_B => 
                                     {
                                        code_set = CODE_CODE_B;
                                        break;
                                    }
                                  CODE_STOP => 
                                     {
                                        done = true;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
            }
            // Unshift back to another code set if we were shifted
            if unshift {
                code_set =  if code_set == CODE_CODE_A { CODE_CODE_B } else { CODE_CODE_A };
            }
        }
         let last_pattern_size: i32 = next_start - last_start;
        // Check for ample whitespace following pattern, but, to do this we first need to remember that
        // we fudged decoding CODE_STOP since it actually has 7 bars, not 6. There is a black bar left
        // to read off. Would be slightly better to properly read. Here we just skip it:
        next_start = row.get_next_unset(next_start);
        if !row.is_range(next_start, &Math::min(&row.get_size(), next_start + (next_start - last_start) / 2), false) {
            throw NotFoundException::get_not_found_instance();
        }
        // Pull out from sum the value of the penultimate check code
        checksum_total -= multiplier * last_code;
        // lastCode is the checksum then:
        if checksum_total % 103 != last_code {
            throw ChecksumException::get_checksum_instance();
        }
        // Need to pull out the check digits from string
         let result_length: i32 = result.length();
        if result_length == 0 {
            // false positive
            throw NotFoundException::get_not_found_instance();
        }
        // be a printable character. If it was just interpreted as a control code, nothing to remove.
        if result_length > 0 && last_character_was_printable {
            if code_set == CODE_CODE_C {
                result.delete(result_length - 2, result_length);
            } else {
                result.delete(result_length - 1, result_length);
            }
        }
         let left: f32 = (start_pattern_info[1] + start_pattern_info[0]) / 2.0f;
         let right: f32 = last_start + last_pattern_size / 2.0f;
         let raw_codes_size: i32 = raw_codes.size();
         let raw_bytes: [i8; raw_codes_size] = [0; raw_codes_size];
         {
             let mut i: i32 = 0;
            while i < raw_codes_size {
                {
                    raw_bytes[i] = raw_codes.get(i);
                }
                i += 1;
             }
         }

         let result_object: Result = Result::new(&result.to_string(), &raw_bytes,  : vec![ResultPoint; 2] = vec![ResultPoint::new(left, row_number), ResultPoint::new(right, row_number), ]
        , BarcodeFormat::CODE_128);
        result_object.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, format!("]C{}", symbology_modifier));
        return Ok(result_object);
    }
}

// NEW FILE: code128_writer.rs
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
 * This object renders a CODE128 code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */

 const CODE_START_A: i32 = 103;

 const CODE_START_B: i32 = 104;

 const CODE_START_C: i32 = 105;

 const CODE_CODE_A: i32 = 101;

 const CODE_CODE_B: i32 = 100;

 const CODE_CODE_C: i32 = 99;

 const CODE_STOP: i32 = 106;

// Dummy characters used to specify control characters in input
 const ESCAPE_FNC_1: char = 'ñ';

 const ESCAPE_FNC_2: char = 'ò';

 const ESCAPE_FNC_3: char = 'ó';

 const ESCAPE_FNC_4: char = 'ô';

// Code A, Code B, Code C
 const CODE_FNC_1: i32 = 102;

// Code A, Code B
 const CODE_FNC_2: i32 = 97;

// Code A, Code B
 const CODE_FNC_3: i32 = 96;

// Code A
 const CODE_FNC_4_A: i32 = 101;

// Code B
 const CODE_FNC_4_B: i32 = 100;
pub struct Code128Writer {
    super: OneDimensionalCodeWriter;
}

impl Code128Writer {

    // Results of minimal lookahead for code C
    enum CType {

        UNCODABLE(), ONE_DIGIT(), TWO_DIGITS(), FNC_1()
    }

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::CODE_128);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
        return self.encode(&contents, null);
    }

    pub fn  encode(&self,  contents: &String,  hints: &Map<EncodeHintType, ?>) -> Vec<bool>  {
         let forced_code_set: i32 = ::check(&contents, &hints);
         let has_compaction_hint: bool = hints != null && hints.contains_key(EncodeHintType::CODE128_COMPACT) && Boolean::parse_boolean(&hints.get(EncodeHintType::CODE128_COMPACT).to_string());
        return  if has_compaction_hint { MinimalEncoder::new().encode(&contents) } else { ::encode_fast(&contents, forced_code_set) };
    }

    fn  check( contents: &String,  hints: &Map<EncodeHintType, ?>) -> i32  {
         let length: i32 = contents.length();
        // Check length
        if length < 1 || length > 80 {
            throw IllegalArgumentException::new(format!("Contents length should be between 1 and 80 characters, but got {}", length));
        }
        // Check for forced code set hint.
         let forced_code_set: i32 = -1;
        if hints != null && hints.contains_key(EncodeHintType::FORCE_CODE_SET) {
             let code_set_hint: String = hints.get(EncodeHintType::FORCE_CODE_SET).to_string();
            match code_set_hint {
                  "A" => 
                     {
                        forced_code_set = CODE_CODE_A;
                        break;
                    }
                  "B" => 
                     {
                        forced_code_set = CODE_CODE_B;
                        break;
                    }
                  "C" => 
                     {
                        forced_code_set = CODE_CODE_C;
                        break;
                    }
                _ => 
                     {
                        throw IllegalArgumentException::new(format!("Unsupported code set hint: {}", code_set_hint));
                    }
            }
        }
        // Check content
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let c: char = contents.char_at(i);
                    // check for non ascii characters that are not special GS1 characters
                    match c {
                        // special function characters
                          ESCAPE_FNC_1 => 
                             {
                            }
                          ESCAPE_FNC_2 => 
                             {
                            }
                          ESCAPE_FNC_3 => 
                             {
                            }
                          ESCAPE_FNC_4 => 
                             {
                                break;
                            }
                        // non ascii characters
                        _ => 
                             {
                                if c > 127 {
                                    // shift and manual code change are not supported
                                    throw IllegalArgumentException::new(format!("Bad character in input: ASCII value={}", c as i32));
                                }
                            }
                    }
                    // check characters for compatibility with forced code set
                    match forced_code_set {
                          CODE_CODE_A => 
                             {
                                // allows no ascii above 95 (no lower caps, no special symbols)
                                if c > 95 && c <= 127 {
                                    throw IllegalArgumentException::new(format!("Bad character in input for forced code set A: ASCII value={}", c as i32));
                                }
                                break;
                            }
                          CODE_CODE_B => 
                             {
                                // allows no ascii below 32 (terminal symbols)
                                if c <= 32 {
                                    throw IllegalArgumentException::new(format!("Bad character in input for forced code set B: ASCII value={}", c as i32));
                                }
                                break;
                            }
                          CODE_CODE_C => 
                             {
                                // allows only numbers and no FNC 2/3/4
                                if c < 48 || (c > 57 && c <= 127) || c == ESCAPE_FNC_2 || c == ESCAPE_FNC_3 || c == ESCAPE_FNC_4 {
                                    throw IllegalArgumentException::new(format!("Bad character in input for forced code set C: ASCII value={}", c as i32));
                                }
                                break;
                            }
                    }
                }
                i += 1;
             }
         }

        return forced_code_set;
    }

    fn  encode_fast( contents: &String,  forced_code_set: i32) -> Vec<bool>  {
         let length: i32 = contents.length();
        // temporary storage for patterns
         let patterns: Collection<Vec<i32>> = ArrayList<>::new();
         let check_sum: i32 = 0;
         let check_weight: i32 = 1;
        // selected code (CODE_CODE_B or CODE_CODE_C)
         let code_set: i32 = 0;
        // position in contents
         let mut position: i32 = 0;
        while position < length {
            //Select code to use
             let new_code_set: i32;
            if forced_code_set == -1 {
                new_code_set = ::choose_code(&contents, position, code_set);
            } else {
                new_code_set = forced_code_set;
            }
            //Get the pattern index
             let pattern_index: i32;
            if new_code_set == code_set {
                // First handle escapes
                match contents.char_at(position) {
                      ESCAPE_FNC_1 => 
                         {
                            pattern_index = CODE_FNC_1;
                            break;
                        }
                      ESCAPE_FNC_2 => 
                         {
                            pattern_index = CODE_FNC_2;
                            break;
                        }
                      ESCAPE_FNC_3 => 
                         {
                            pattern_index = CODE_FNC_3;
                            break;
                        }
                      ESCAPE_FNC_4 => 
                         {
                            if code_set == CODE_CODE_A {
                                pattern_index = CODE_FNC_4_A;
                            } else {
                                pattern_index = CODE_FNC_4_B;
                            }
                            break;
                        }
                    _ => 
                         {
                            // Then handle normal characters otherwise
                            match code_set {
                                  CODE_CODE_A => 
                                     {
                                        pattern_index = contents.char_at(position) - ' ';
                                        if pattern_index < 0 {
                                            // everything below a space character comes behind the underscore in the code patterns table
                                            pattern_index += '`';
                                        }
                                        break;
                                    }
                                  CODE_CODE_B => 
                                     {
                                        pattern_index = contents.char_at(position) - ' ';
                                        break;
                                    }
                                _ => 
                                     {
                                        // CODE_CODE_C
                                        if position + 1 == length {
                                            // this is the last character, but the encoding is C, which always encodes two characers
                                            throw IllegalArgumentException::new("Bad number of characters for digit only encoding.");
                                        }
                                        pattern_index = Integer::parse_int(&contents.substring(position, position + 2));
                                        // Also incremented below
                                        position += 1;
                                        break;
                                    }
                            }
                        }
                }
                position += 1;
            } else {
                // Do we have a code set?
                if code_set == 0 {
                    // No, we don't have a code set
                    match new_code_set {
                          CODE_CODE_A => 
                             {
                                pattern_index = CODE_START_A;
                                break;
                            }
                          CODE_CODE_B => 
                             {
                                pattern_index = CODE_START_B;
                                break;
                            }
                        _ => 
                             {
                                pattern_index = CODE_START_C;
                                break;
                            }
                    }
                } else {
                    // Yes, we have a code set
                    pattern_index = new_code_set;
                }
                code_set = new_code_set;
            }
            // Get the pattern
            patterns.add(Code128Reader::CODE_PATTERNS[pattern_index]);
            // Compute checksum
            check_sum += pattern_index * check_weight;
            if position != 0 {
                check_weight += 1;
            }
        }
        return ::produce_result(&patterns, check_sum);
    }

    fn  produce_result( patterns: &Collection<Vec<i32>>,  check_sum: i32) -> Vec<bool>  {
        // Compute and append checksum
        check_sum %= 103;
        patterns.add(Code128Reader::CODE_PATTERNS[check_sum]);
        // Append stop code
        patterns.add(Code128Reader::CODE_PATTERNS[CODE_STOP]);
        // Compute code width
         let code_width: i32 = 0;
        for  let pattern: Vec<i32> in patterns {
            for  let width: i32 in pattern {
                code_width += width;
            }
        }
        // Compute result
         let result: [bool; code_width] = [false; code_width];
         let mut pos: i32 = 0;
        for  let pattern: Vec<i32> in patterns {
            pos += append_pattern(&result, pos, &pattern, true);
        }
        return result;
    }

    fn  find_c_type( value: &CharSequence,  start: i32) -> CType  {
         let last: i32 = value.length();
        if start >= last {
            return CType.UNCODABLE;
        }
         let mut c: char = value.char_at(start);
        if c == ESCAPE_FNC_1 {
            return CType.FNC_1;
        }
        if c < '0' || c > '9' {
            return CType.UNCODABLE;
        }
        if start + 1 >= last {
            return CType.ONE_DIGIT;
        }
        c = value.char_at(start + 1);
        if c < '0' || c > '9' {
            return CType.ONE_DIGIT;
        }
        return CType.TWO_DIGITS;
    }

    fn  choose_code( value: &CharSequence,  start: i32,  old_code: i32) -> i32  {
         let mut lookahead: CType = ::find_c_type(&value, start);
        if lookahead == CType.ONE_DIGIT {
            if old_code == CODE_CODE_A {
                return CODE_CODE_A;
            }
            return CODE_CODE_B;
        }
        if lookahead == CType.UNCODABLE {
            if start < value.length() {
                 let c: char = value.char_at(start);
                if c < ' ' || (old_code == CODE_CODE_A && (c < '`' || (c >= ESCAPE_FNC_1 && c <= ESCAPE_FNC_4))) {
                    // can continue in code A, encodes ASCII 0 to 95 or FNC1 to FNC4
                    return CODE_CODE_A;
                }
            }
            // no choice
            return CODE_CODE_B;
        }
        if old_code == CODE_CODE_A && lookahead == CType.FNC_1 {
            return CODE_CODE_A;
        }
        if old_code == CODE_CODE_C {
            // can continue in code C
            return CODE_CODE_C;
        }
        if old_code == CODE_CODE_B {
            if lookahead == CType.FNC_1 {
                // can continue in code B
                return CODE_CODE_B;
            }
            // Seen two consecutive digits, see what follows
            lookahead = ::find_c_type(&value, start + 2);
            if lookahead == CType.UNCODABLE || lookahead == CType.ONE_DIGIT {
                // not worth switching now
                return CODE_CODE_B;
            }
            if lookahead == CType.FNC_1 {
                // two digits, then FNC_1...
                lookahead = ::find_c_type(&value, start + 3);
                if lookahead == CType.TWO_DIGITS {
                    // then two more digits, switch
                    return CODE_CODE_C;
                } else {
                    // otherwise not worth switching
                    return CODE_CODE_B;
                }
            }
            // At this point, there are at least 4 consecutive digits.
            // Look ahead to choose whether to switch now or on the next round.
             let mut index: i32 = start + 4;
            while (lookahead = ::find_c_type(&value, index)) == CType.TWO_DIGITS {
                index += 2;
            }
            if lookahead == CType.ONE_DIGIT {
                // odd number of digits, switch later
                return CODE_CODE_B;
            }
            // even number of digits, switch now
            return CODE_CODE_C;
        }
        // Here oldCode == 0, which means we are choosing the initial code
        if lookahead == CType.FNC_1 {
            // ignore FNC_1
            lookahead = ::find_c_type(&value, start + 1);
        }
        if lookahead == CType.TWO_DIGITS {
            // at least two digits, start in code C
            return CODE_CODE_C;
        }
        return CODE_CODE_B;
    }

    /** 
   * Encodes minimally using Divide-And-Conquer with Memoization
   **/

     const A: &'static str = format!(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_ 	\n\rÿ");

     const B: &'static str = format!(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ÿ");

     const CODE_SHIFT: i32 = 98;
    struct MinimalEncoder {

         let memoized_cost: Vec<Vec<i32>>;

         let min_path: Vec<Vec<Latch>>;
    }
    
    impl MinimalEncoder {

        enum Charset {

            A(), B(), C(), NONE()
        }

        enum Latch {

            A(), B(), C(), SHIFT(), NONE()
        }

        fn  encode(&self,  contents: &String) -> Vec<bool>  {
            self.memoized_cost = : [[i32; contents.length()]; 4] = [[0; contents.length()]; 4];
            self.min_path = : [[Option<Latch>; contents.length()]; 4] = [[None; contents.length()]; 4];
            self.encode(&contents, Charset::NONE, 0);
             let patterns: Collection<Vec<i32>> = ArrayList<>::new();
             let check_sum : vec![i32; 1] = vec![0, ]
            ;
             let check_weight : vec![i32; 1] = vec![1, ]
            ;
             let length: i32 = contents.length();
             let mut charset: Charset = Charset::NONE;
             {
                 let mut i: i32 = 0;
                while i < length {
                    {
                         let latch: Latch = self.min_path[charset.ordinal()][i];
                        match latch {
                              A => 
                                 {
                                    charset = Charset::A;
                                    ::add_pattern(&patterns,  if i == 0 { CODE_START_A } else { CODE_CODE_A }, &check_sum, &check_weight, i);
                                    break;
                                }
                              B => 
                                 {
                                    charset = Charset::B;
                                    ::add_pattern(&patterns,  if i == 0 { CODE_START_B } else { CODE_CODE_B }, &check_sum, &check_weight, i);
                                    break;
                                }
                              C => 
                                 {
                                    charset = Charset::C;
                                    ::add_pattern(&patterns,  if i == 0 { CODE_START_C } else { CODE_CODE_C }, &check_sum, &check_weight, i);
                                    break;
                                }
                              SHIFT => 
                                 {
                                    ::add_pattern(&patterns, CODE_SHIFT, &check_sum, &check_weight, i);
                                    break;
                                }
                        }
                        if charset == Charset::C {
                            if contents.char_at(i) == ESCAPE_FNC_1 {
                                ::add_pattern(&patterns, CODE_FNC_1, &check_sum, &check_weight, i);
                            } else {
                                ::add_pattern(&patterns, &Integer::parse_int(&contents.substring(i, i + 2)), &check_sum, &check_weight, i);
                                //the algorithm never leads to a single trailing digit in character set C
                                assert!( i + 1 < length);
                                if i + 1 < length {
                                    i += 1;
                                }
                            }
                        } else {
                            // charset A or B
                             let pattern_index: i32;
                            match contents.char_at(i) {
                                  ESCAPE_FNC_1 => 
                                     {
                                        pattern_index = CODE_FNC_1;
                                        break;
                                    }
                                  ESCAPE_FNC_2 => 
                                     {
                                        pattern_index = CODE_FNC_2;
                                        break;
                                    }
                                  ESCAPE_FNC_3 => 
                                     {
                                        pattern_index = CODE_FNC_3;
                                        break;
                                    }
                                  ESCAPE_FNC_4 => 
                                     {
                                        if (charset == Charset::A && latch != Latch::SHIFT) || (charset == Charset::B && latch == Latch::SHIFT) {
                                            pattern_index = CODE_FNC_4_A;
                                        } else {
                                            pattern_index = CODE_FNC_4_B;
                                        }
                                        break;
                                    }
                                _ => 
                                     {
                                        pattern_index = contents.char_at(i) - ' ';
                                    }
                            }
                            if (charset == Charset::A && latch != Latch::SHIFT) || (charset == Charset::B && latch == Latch::SHIFT) {
                                if pattern_index < 0 {
                                    pattern_index += '`';
                                }
                            }
                            ::add_pattern(&patterns, pattern_index, &check_sum, &check_weight, i);
                        }
                    }
                    i += 1;
                 }
             }

            self.memoized_cost = null;
            self.min_path = null;
            return ::produce_result(&patterns, check_sum[0]);
        }

        fn  add_pattern( patterns: &Collection<Vec<i32>>,  pattern_index: i32,  check_sum: &Vec<i32>,  check_weight: &Vec<i32>,  position: i32)   {
            patterns.add(Code128Reader::CODE_PATTERNS[pattern_index]);
            if position != 0 {
                check_weight[0] += 1;
            }
            check_sum[0] += pattern_index * check_weight[0];
        }

        fn  is_digit( c: char) -> bool  {
            return c >= '0' && c <= '9';
        }

        fn  can_encode(&self,  contents: &CharSequence,  charset: &Charset,  position: i32) -> bool  {
             let c: char = contents.char_at(position);
            match charset {
                  A => 
                     {
                        return c == ESCAPE_FNC_1 || c == ESCAPE_FNC_2 || c == ESCAPE_FNC_3 || c == ESCAPE_FNC_4 || A::index_of(c) >= 0;
                    }
                  B => 
                     {
                        return c == ESCAPE_FNC_1 || c == ESCAPE_FNC_2 || c == ESCAPE_FNC_3 || c == ESCAPE_FNC_4 || B::index_of(c) >= 0;
                    }
                  C => 
                     {
                        return c == ESCAPE_FNC_1 || (position + 1 < contents.length() && ::is_digit(c) && ::is_digit(&contents.char_at(position + 1)));
                    }
                _ => 
                     {
                        return false;
                    }
            }
        }

        /**
     * Encode the string starting at position position starting with the character set charset
     **/
        fn  encode(&self,  contents: &CharSequence,  charset: &Charset,  position: i32) -> i32  {
            assert!( position < contents.length());
             let m_cost: i32 = self.memoized_cost[charset.ordinal()][position];
            if m_cost > 0 {
                return m_cost;
            }
             let min_cost: i32 = Integer::MAX_VALUE;
             let min_latch: Latch = Latch::NONE;
             let at_end: bool = position + 1 >= contents.length();
             let sets : vec![Charset; 2] = vec![Charset::A, Charset::B, ]
            ;
             {
                 let mut i: i32 = 0;
                while i <= 1 {
                    {
                        if self.can_encode(&contents, sets[i], position) {
                             let mut cost: i32 = 1;
                             let mut latch: Latch = Latch::NONE;
                            if charset != sets[i] {
                                cost += 1;
                                latch = Latch::value_of(&sets[i].to_string());
                            }
                            if !at_end {
                                cost += self.encode(&contents, sets[i], position + 1);
                            }
                            if cost < min_cost {
                                min_cost = cost;
                                min_latch = latch;
                            }
                            cost = 1;
                            if charset == sets[(i + 1) % 2] {
                                cost += 1;
                                latch = Latch::SHIFT;
                                if !at_end {
                                    cost += self.encode(&contents, charset, position + 1);
                                }
                                if cost < min_cost {
                                    min_cost = cost;
                                    min_latch = latch;
                                }
                            }
                        }
                    }
                    i += 1;
                 }
             }

            if self.can_encode(&contents, Charset::C, position) {
                 let mut cost: i32 = 1;
                 let mut latch: Latch = Latch::NONE;
                if charset != Charset::C {
                    cost += 1;
                    latch = Latch::C;
                }
                 let advance: i32 =  if contents.char_at(position) == ESCAPE_FNC_1 { 1 } else { 2 };
                if position + advance < contents.length() {
                    cost += self.encode(&contents, Charset::C, position + advance);
                }
                if cost < min_cost {
                    min_cost = cost;
                    min_latch = latch;
                }
            }
            if min_cost == Integer::MAX_VALUE {
                throw IllegalArgumentException::new(format!("Bad character in input: ASCII value={}", contents.char_at(position) as i32));
            }
            self.memoized_cost[charset.ordinal()][position] = min_cost;
            self.min_path[charset.ordinal()][position] = min_latch;
            return min_cost;
        }
    }

}

// NEW FILE: code39_reader.rs
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

// NEW FILE: code39_writer.rs
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
 * This object renders a CODE39 code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */
pub struct Code39Writer {
    super: OneDimensionalCodeWriter;
}

impl Code39Writer {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::CODE_39);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let mut length: i32 = contents.length();
        if length > 80 {
            throw IllegalArgumentException::new(format!("Requested contents should be less than 80 digits long, but got {}", length));
        }
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let index_in_string: i32 = Code39Reader::ALPHABET_STRING::index_of(&contents.char_at(i));
                    if index_in_string < 0 {
                        contents = ::try_to_convert_to_extended_mode(&contents);
                        length = contents.length();
                        if length > 80 {
                            throw IllegalArgumentException::new(format!("Requested contents should be less than 80 digits long, but got {} (extended full ASCII mode)", length));
                        }
                        break;
                    }
                }
                i += 1;
             }
         }

         let widths: [i32; 9] = [0; 9];
         let code_width: i32 = 24 + 1 + (13 * length);
         let result: [bool; code_width] = [false; code_width];
        ::to_int_array(Code39Reader::ASTERISK_ENCODING, &widths);
         let mut pos: i32 = append_pattern(&result, 0, &widths, true);
         let narrow_white: vec![Vec<i32>; 1] = vec![1, ]
        ;
        pos += append_pattern(&result, pos, &narrow_white, false);
        //append next character to byte matrix
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let index_in_string: i32 = Code39Reader::ALPHABET_STRING::index_of(&contents.char_at(i));
                    ::to_int_array(Code39Reader::CHARACTER_ENCODINGS[index_in_string], &widths);
                    pos += append_pattern(&result, pos, &widths, true);
                    pos += append_pattern(&result, pos, &narrow_white, false);
                }
                i += 1;
             }
         }

        ::to_int_array(Code39Reader::ASTERISK_ENCODING, &widths);
        append_pattern(&result, pos, &widths, true);
        return result;
    }

    fn  to_int_array( a: i32,  to_return: &Vec<i32>)   {
         {
             let mut i: i32 = 0;
            while i < 9 {
                {
                     let temp: i32 = a & (1 << (8 - i));
                    to_return[i] =  if temp == 0 { 1 } else { 2 };
                }
                i += 1;
             }
         }

    }

    fn  try_to_convert_to_extended_mode( contents: &String) -> String  {
         let length: i32 = contents.length();
         let extended_content: StringBuilder = StringBuilder::new();
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let character: char = contents.char_at(i);
                    match character {
                          ' ' => 
                             {
                                extended_content.append("%U");
                                break;
                            }
                          ' ' => 
                             {
                            }
                          '-' => 
                             {
                            }
                          '.' => 
                             {
                                extended_content.append(character);
                                break;
                            }
                          '@' => 
                             {
                                extended_content.append("%V");
                                break;
                            }
                          '`' => 
                             {
                                extended_content.append("%W");
                                break;
                            }
                        _ => 
                             {
                                if character <= 26 {
                                    extended_content.append('$');
                                    extended_content.append(('A' + (character - 1)) as char);
                                } else if character < ' ' {
                                    extended_content.append('%');
                                    extended_content.append(('A' + (character - 27)) as char);
                                } else if character <= ',' || character == '/' || character == ':' {
                                    extended_content.append('/');
                                    extended_content.append(('A' + (character - 33)) as char);
                                } else if character <= '9' {
                                    extended_content.append(('0' + (character - 48)) as char);
                                } else if character <= '?' {
                                    extended_content.append('%');
                                    extended_content.append(('F' + (character - 59)) as char);
                                } else if character <= 'Z' {
                                    extended_content.append(('A' + (character - 65)) as char);
                                } else if character <= '_' {
                                    extended_content.append('%');
                                    extended_content.append(('K' + (character - 91)) as char);
                                } else if character <= 'z' {
                                    extended_content.append('+');
                                    extended_content.append(('A' + (character - 97)) as char);
                                } else if character <= 127 {
                                    extended_content.append('%');
                                    extended_content.append(('P' + (character - 123)) as char);
                                } else {
                                    throw IllegalArgumentException::new(format!("Requested content contains a non-encodable character: '{}'", contents.char_at(i)));
                                }
                                break;
                            }
                    }
                }
                i += 1;
             }
         }

        return extended_content.to_string();
    }
}

// NEW FILE: code93_reader.rs
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

// NEW FILE: code93_writer.rs
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

// NEW FILE: e_a_n13_reader.rs
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
 * <p>Implements decoding of the EAN-13 format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */

// For an EAN-13 barcode, the first digit is represented by the parities used
// to encode the next six digits, according to the table below. For example,
// if the barcode is 5 123456 789012 then the value of the first digit is
// signified by using odd for '1', even for '2', even for '3', odd for '4',
// odd for '5', and even for '6'. See http://en.wikipedia.org/wiki/EAN-13
//
//                Parity of next 6 digits
//    Digit   0     1     2     3     4     5
//       0    Odd   Odd   Odd   Odd   Odd   Odd
//       1    Odd   Odd   Even  Odd   Even  Even
//       2    Odd   Odd   Even  Even  Odd   Even
//       3    Odd   Odd   Even  Even  Even  Odd
//       4    Odd   Even  Odd   Odd   Even  Even
//       5    Odd   Even  Even  Odd   Odd   Even
//       6    Odd   Even  Even  Even  Odd   Odd
//       7    Odd   Even  Odd   Even  Odd   Even
//       8    Odd   Even  Odd   Even  Even  Odd
//       9    Odd   Even  Even  Odd   Even  Odd
//
// Note that the encoding for '0' uses the same parity as a UPC barcode. Hence
// a UPC barcode can be converted to an EAN-13 barcode by prepending a 0.
//
// The encoding is represented by the following array, which is a bit pattern
// using Odd = 0 and Even = 1. For example, 5 is represented by:
//
//              Odd Even Even Odd Odd Even
// in binary:
//                0    1    1   0   0    1   == 0x19
//
 const FIRST_DIGIT_ENCODINGS: vec![Vec<i32>; 10] = vec![0x00, 0x0B, 0x0D, 0xE, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A, ]
;
pub struct EAN13Reader {
    super: UPCEANReader;

     let decode_middle_counters: Vec<i32>;
}

impl EAN13Reader {

    pub fn new() -> EAN13Reader {
        decode_middle_counters = : [i32; 4] = [0; 4];
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.decode_middle_counters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let end: i32 = row.get_size();
         let row_offset: i32 = start_range[1];
         let lg_pattern_found: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < 6 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_AND_G_PATTERNS);
                    result_string.append(('0' + best_match % 10) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                    if best_match >= 10 {
                        lg_pattern_found |= 1 << (5 - x);
                    }
                }
                x += 1;
             }
         }

        ::determine_first_digit(&result_string, lg_pattern_found);
         let middle_range: Vec<i32> = find_guard_pattern(row, row_offset, true, MIDDLE_PATTERN);
        row_offset = middle_range[1];
         {
             let mut x: i32 = 0;
            while x < 6 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_PATTERNS);
                    result_string.append(('0' + best_match) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                }
                x += 1;
             }
         }

        return Ok(row_offset);
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::EAN_13;
    }

    /**
   * Based on pattern of odd-even ('L' and 'G') patterns used to encoded the explicitly-encoded
   * digits in a barcode, determines the implicitly encoded first digit and adds it to the
   * result string.
   *
   * @param resultString string to insert decoded first digit into
   * @param lgPatternFound int whose bits indicates the pattern of odd/even L/G patterns used to
   *  encode digits
   * @throws NotFoundException if first digit cannot be determined
   */
    fn  determine_first_digit( result_string: &StringBuilder,  lg_pattern_found: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         {
             let mut d: i32 = 0;
            while d < 10 {
                {
                    if lg_pattern_found == FIRST_DIGIT_ENCODINGS[d] {
                        result_string.insert(0, ('0' + d) as char);
                        return;
                    }
                }
                d += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }
}

// NEW FILE: e_a_n13_writer.rs
/*
 * Copyright 2009 ZXing authors
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
 * This object renders an EAN13 code as a {@link BitMatrix}.
 *
 * @author aripollak@gmail.com (Ari Pollak)
 */

 const CODE_WIDTH: i32 = // start guard
3 + // left bars
(7 * 6) + // middle guard
5 + // right bars
(7 * 6) + // end guard
3;
pub struct EAN13Writer {
    super: UPCEANWriter;
}

impl EAN13Writer {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::EAN_13);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let length: i32 = contents.length();
        match length {
              12 => 
                 {
                    // No check digit present, calculate it and add it
                     let mut check: i32;
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        check = UPCEANReader::get_standard_u_p_c_e_a_n_checksum(&contents);
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( fe: &FormatException) {
                            throw IllegalArgumentException::new(fe);
                        }  0 => break
                    }

                    contents += check;
                    break;
                }
              13 => 
                 {
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        if !UPCEANReader::check_standard_u_p_c_e_a_n_checksum(&contents) {
                            throw IllegalArgumentException::new("Contents do not pass checksum");
                        }
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( ignored: &FormatException) {
                            throw IllegalArgumentException::new("Illegal contents");
                        }  0 => break
                    }

                    break;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("Requested contents should be 12 or 13 digits long, but got {}", length));
                }
        }
        check_numeric(&contents);
         let first_digit: i32 = Character::digit(&contents.char_at(0), 10);
         let parities: i32 = EAN13Reader.FIRST_DIGIT_ENCODINGS[first_digit];
         let result: [bool; CODE_WIDTH] = [false; CODE_WIDTH];
         let mut pos: i32 = 0;
        pos += append_pattern(&result, pos, UPCEANReader.START_END_PATTERN, true);
        // See EAN13Reader for a description of how the first digit & left bars are encoded
         {
             let mut i: i32 = 1;
            while i <= 6 {
                {
                     let mut digit: i32 = Character::digit(&contents.char_at(i), 10);
                    if (parities >> (6 - i) & 1) == 1 {
                        digit += 10;
                    }
                    pos += append_pattern(&result, pos, UPCEANReader.L_AND_G_PATTERNS[digit], false);
                }
                i += 1;
             }
         }

        pos += append_pattern(&result, pos, UPCEANReader.MIDDLE_PATTERN, false);
         {
             let mut i: i32 = 7;
            while i <= 12 {
                {
                     let digit: i32 = Character::digit(&contents.char_at(i), 10);
                    pos += append_pattern(&result, pos, UPCEANReader.L_PATTERNS[digit], true);
                }
                i += 1;
             }
         }

        append_pattern(&result, pos, UPCEANReader.START_END_PATTERN, true);
        return result;
    }
}

// NEW FILE: e_a_n8_reader.rs
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
 * <p>Implements decoding of the EAN-8 format.</p>
 *
 * @author Sean Owen
 */
pub struct EAN8Reader {
    super: UPCEANReader;

     let decode_middle_counters: Vec<i32>;
}

impl EAN8Reader {

    pub fn new() -> EAN8Reader {
        decode_middle_counters = : [i32; 4] = [0; 4];
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.decode_middle_counters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let end: i32 = row.get_size();
         let row_offset: i32 = start_range[1];
         {
             let mut x: i32 = 0;
            while x < 4 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_PATTERNS);
                    result.append(('0' + best_match) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                }
                x += 1;
             }
         }

         let middle_range: Vec<i32> = find_guard_pattern(row, row_offset, true, MIDDLE_PATTERN);
        row_offset = middle_range[1];
         {
             let mut x: i32 = 0;
            while x < 4 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_PATTERNS);
                    result.append(('0' + best_match) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                }
                x += 1;
             }
         }

        return Ok(row_offset);
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::EAN_8;
    }
}

// NEW FILE: e_a_n8_writer.rs
/*
 * Copyright 2009 ZXing authors
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
 * This object renders an EAN8 code as a {@link BitMatrix}.
 *
 * @author aripollak@gmail.com (Ari Pollak)
 */

 const CODE_WIDTH: i32 = // start guard
3 + // left bars
(7 * 4) + // middle guard
5 + // right bars
(7 * 4) + // end guard
3;
pub struct EAN8Writer {
    super: UPCEANWriter;
}

impl EAN8Writer {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::EAN_8);
    }

    /**
   * @return a byte array of horizontal pixels (false = white, true = black)
   */
    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let length: i32 = contents.length();
        match length {
              7 => 
                 {
                    // No check digit present, calculate it and add it
                     let mut check: i32;
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        check = UPCEANReader::get_standard_u_p_c_e_a_n_checksum(&contents);
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( fe: &FormatException) {
                            throw IllegalArgumentException::new(fe);
                        }  0 => break
                    }

                    contents += check;
                    break;
                }
              8 => 
                 {
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        if !UPCEANReader::check_standard_u_p_c_e_a_n_checksum(&contents) {
                            throw IllegalArgumentException::new("Contents do not pass checksum");
                        }
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( ignored: &FormatException) {
                            throw IllegalArgumentException::new("Illegal contents");
                        }  0 => break
                    }

                    break;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("Requested contents should be 7 or 8 digits long, but got {}", length));
                }
        }
        check_numeric(&contents);
         let result: [bool; CODE_WIDTH] = [false; CODE_WIDTH];
         let mut pos: i32 = 0;
        pos += append_pattern(&result, pos, UPCEANReader.START_END_PATTERN, true);
         {
             let mut i: i32 = 0;
            while i <= 3 {
                {
                     let digit: i32 = Character::digit(&contents.char_at(i), 10);
                    pos += append_pattern(&result, pos, UPCEANReader.L_PATTERNS[digit], false);
                }
                i += 1;
             }
         }

        pos += append_pattern(&result, pos, UPCEANReader.MIDDLE_PATTERN, false);
         {
             let mut i: i32 = 4;
            while i <= 7 {
                {
                     let digit: i32 = Character::digit(&contents.char_at(i), 10);
                    pos += append_pattern(&result, pos, UPCEANReader.L_PATTERNS[digit], true);
                }
                i += 1;
             }
         }

        append_pattern(&result, pos, UPCEANReader.START_END_PATTERN, true);
        return result;
    }
}

// NEW FILE: e_a_n_manufacturer_org_support.rs
/*
 * Copyright (C) 2010 ZXing authors
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
 * Records EAN prefix to GS1 Member Organization, where the member organization
 * correlates strongly with a country. This is an imperfect means of identifying
 * a country of origin by EAN-13 barcode value. See
 * <a href="http://en.wikipedia.org/wiki/List_of_GS1_country_codes">
 * http://en.wikipedia.org/wiki/List_of_GS1_country_codes</a>.
 *
 * @author Sean Owen
 */
struct EANManufacturerOrgSupport {

     let ranges: List<Vec<i32>> = ArrayList<>::new();

     let country_identifiers: List<String> = ArrayList<>::new();
}

impl EANManufacturerOrgSupport {

    fn  lookup_country_identifier(&self,  product_code: &String) -> String  {
        self.init_if_needed();
         let prefix: i32 = Integer::parse_int(&product_code.substring(0, 3));
         let max: i32 = self.ranges.size();
         {
             let mut i: i32 = 0;
            while i < max {
                {
                     let range: Vec<i32> = self.ranges.get(i);
                     let start: i32 = range[0];
                    if prefix < start {
                        return null;
                    }
                     let end: i32 =  if range.len() == 1 { start } else { range[1] };
                    if prefix <= end {
                        return self.country_identifiers.get(i);
                    }
                }
                i += 1;
             }
         }

        return null;
    }

    fn  add(&self,  range: &Vec<i32>,  id: &String)   {
        self.ranges.add(&range);
        self.country_identifiers.add(&id);
    }

    fn  init_if_needed(&self)   {
        if !self.ranges.is_empty() {
            return;
        }
        self.add( : vec![i32; 2] = vec![0, 19, ]
        , "US/CA");
        self.add( : vec![i32; 2] = vec![30, 39, ]
        , "US");
        self.add( : vec![i32; 2] = vec![60, 139, ]
        , "US/CA");
        self.add( : vec![i32; 2] = vec![300, 379, ]
        , "FR");
        self.add( : vec![i32; 1] = vec![380, ]
        , "BG");
        self.add( : vec![i32; 1] = vec![383, ]
        , "SI");
        self.add( : vec![i32; 1] = vec![385, ]
        , "HR");
        self.add( : vec![i32; 1] = vec![387, ]
        , "BA");
        self.add( : vec![i32; 2] = vec![400, 440, ]
        , "DE");
        self.add( : vec![i32; 2] = vec![450, 459, ]
        , "JP");
        self.add( : vec![i32; 2] = vec![460, 469, ]
        , "RU");
        self.add( : vec![i32; 1] = vec![471, ]
        , "TW");
        self.add( : vec![i32; 1] = vec![474, ]
        , "EE");
        self.add( : vec![i32; 1] = vec![475, ]
        , "LV");
        self.add( : vec![i32; 1] = vec![476, ]
        , "AZ");
        self.add( : vec![i32; 1] = vec![477, ]
        , "LT");
        self.add( : vec![i32; 1] = vec![478, ]
        , "UZ");
        self.add( : vec![i32; 1] = vec![479, ]
        , "LK");
        self.add( : vec![i32; 1] = vec![480, ]
        , "PH");
        self.add( : vec![i32; 1] = vec![481, ]
        , "BY");
        self.add( : vec![i32; 1] = vec![482, ]
        , "UA");
        self.add( : vec![i32; 1] = vec![484, ]
        , "MD");
        self.add( : vec![i32; 1] = vec![485, ]
        , "AM");
        self.add( : vec![i32; 1] = vec![486, ]
        , "GE");
        self.add( : vec![i32; 1] = vec![487, ]
        , "KZ");
        self.add( : vec![i32; 1] = vec![489, ]
        , "HK");
        self.add( : vec![i32; 2] = vec![490, 499, ]
        , "JP");
        self.add( : vec![i32; 2] = vec![500, 509, ]
        , "GB");
        self.add( : vec![i32; 1] = vec![520, ]
        , "GR");
        self.add( : vec![i32; 1] = vec![528, ]
        , "LB");
        self.add( : vec![i32; 1] = vec![529, ]
        , "CY");
        self.add( : vec![i32; 1] = vec![531, ]
        , "MK");
        self.add( : vec![i32; 1] = vec![535, ]
        , "MT");
        self.add( : vec![i32; 1] = vec![539, ]
        , "IE");
        self.add( : vec![i32; 2] = vec![540, 549, ]
        , "BE/LU");
        self.add( : vec![i32; 1] = vec![560, ]
        , "PT");
        self.add( : vec![i32; 1] = vec![569, ]
        , "IS");
        self.add( : vec![i32; 2] = vec![570, 579, ]
        , "DK");
        self.add( : vec![i32; 1] = vec![590, ]
        , "PL");
        self.add( : vec![i32; 1] = vec![594, ]
        , "RO");
        self.add( : vec![i32; 1] = vec![599, ]
        , "HU");
        self.add( : vec![i32; 2] = vec![600, 601, ]
        , "ZA");
        self.add( : vec![i32; 1] = vec![603, ]
        , "GH");
        self.add( : vec![i32; 1] = vec![608, ]
        , "BH");
        self.add( : vec![i32; 1] = vec![609, ]
        , "MU");
        self.add( : vec![i32; 1] = vec![611, ]
        , "MA");
        self.add( : vec![i32; 1] = vec![613, ]
        , "DZ");
        self.add( : vec![i32; 1] = vec![616, ]
        , "KE");
        self.add( : vec![i32; 1] = vec![618, ]
        , "CI");
        self.add( : vec![i32; 1] = vec![619, ]
        , "TN");
        self.add( : vec![i32; 1] = vec![621, ]
        , "SY");
        self.add( : vec![i32; 1] = vec![622, ]
        , "EG");
        self.add( : vec![i32; 1] = vec![624, ]
        , "LY");
        self.add( : vec![i32; 1] = vec![625, ]
        , "JO");
        self.add( : vec![i32; 1] = vec![626, ]
        , "IR");
        self.add( : vec![i32; 1] = vec![627, ]
        , "KW");
        self.add( : vec![i32; 1] = vec![628, ]
        , "SA");
        self.add( : vec![i32; 1] = vec![629, ]
        , "AE");
        self.add( : vec![i32; 2] = vec![640, 649, ]
        , "FI");
        self.add( : vec![i32; 2] = vec![690, 695, ]
        , "CN");
        self.add( : vec![i32; 2] = vec![700, 709, ]
        , "NO");
        self.add( : vec![i32; 1] = vec![729, ]
        , "IL");
        self.add( : vec![i32; 2] = vec![730, 739, ]
        , "SE");
        self.add( : vec![i32; 1] = vec![740, ]
        , "GT");
        self.add( : vec![i32; 1] = vec![741, ]
        , "SV");
        self.add( : vec![i32; 1] = vec![742, ]
        , "HN");
        self.add( : vec![i32; 1] = vec![743, ]
        , "NI");
        self.add( : vec![i32; 1] = vec![744, ]
        , "CR");
        self.add( : vec![i32; 1] = vec![745, ]
        , "PA");
        self.add( : vec![i32; 1] = vec![746, ]
        , "DO");
        self.add( : vec![i32; 1] = vec![750, ]
        , "MX");
        self.add( : vec![i32; 2] = vec![754, 755, ]
        , "CA");
        self.add( : vec![i32; 1] = vec![759, ]
        , "VE");
        self.add( : vec![i32; 2] = vec![760, 769, ]
        , "CH");
        self.add( : vec![i32; 1] = vec![770, ]
        , "CO");
        self.add( : vec![i32; 1] = vec![773, ]
        , "UY");
        self.add( : vec![i32; 1] = vec![775, ]
        , "PE");
        self.add( : vec![i32; 1] = vec![777, ]
        , "BO");
        self.add( : vec![i32; 1] = vec![779, ]
        , "AR");
        self.add( : vec![i32; 1] = vec![780, ]
        , "CL");
        self.add( : vec![i32; 1] = vec![784, ]
        , "PY");
        self.add( : vec![i32; 1] = vec![785, ]
        , "PE");
        self.add( : vec![i32; 1] = vec![786, ]
        , "EC");
        self.add( : vec![i32; 2] = vec![789, 790, ]
        , "BR");
        self.add( : vec![i32; 2] = vec![800, 839, ]
        , "IT");
        self.add( : vec![i32; 2] = vec![840, 849, ]
        , "ES");
        self.add( : vec![i32; 1] = vec![850, ]
        , "CU");
        self.add( : vec![i32; 1] = vec![858, ]
        , "SK");
        self.add( : vec![i32; 1] = vec![859, ]
        , "CZ");
        self.add( : vec![i32; 1] = vec![860, ]
        , "YU");
        self.add( : vec![i32; 1] = vec![865, ]
        , "MN");
        self.add( : vec![i32; 1] = vec![867, ]
        , "KP");
        self.add( : vec![i32; 2] = vec![868, 869, ]
        , "TR");
        self.add( : vec![i32; 2] = vec![870, 879, ]
        , "NL");
        self.add( : vec![i32; 1] = vec![880, ]
        , "KR");
        self.add( : vec![i32; 1] = vec![885, ]
        , "TH");
        self.add( : vec![i32; 1] = vec![888, ]
        , "SG");
        self.add( : vec![i32; 1] = vec![890, ]
        , "IN");
        self.add( : vec![i32; 1] = vec![893, ]
        , "VN");
        self.add( : vec![i32; 1] = vec![896, ]
        , "PK");
        self.add( : vec![i32; 1] = vec![899, ]
        , "ID");
        self.add( : vec![i32; 2] = vec![900, 919, ]
        , "AT");
        self.add( : vec![i32; 2] = vec![930, 939, ]
        , "AU");
        self.add( : vec![i32; 2] = vec![940, 949, ]
        , "AZ");
        self.add( : vec![i32; 1] = vec![955, ]
        , "MY");
        self.add( : vec![i32; 1] = vec![958, ]
        , "MO");
    }
}

// NEW FILE: i_t_f_reader.rs
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
 * <p>Implements decoding of the ITF format, or Interleaved Two of Five.</p>
 *
 * <p>This Reader will scan ITF barcodes of certain lengths only.
 * At the moment it reads length 6, 8, 10, 12, 14, 16, 18, 20, 24, and 44 as these have appeared "in the wild". Not all
 * lengths are scanned, especially shorter ones, to avoid false positives. This in turn is due to a lack of
 * required checksum function.</p>
 *
 * <p>The checksum is optional and is not applied by this Reader. The consumer of the decoded
 * value will have to apply a checksum if required.</p>
 *
 * <p><a href="http://en.wikipedia.org/wiki/Interleaved_2_of_5">http://en.wikipedia.org/wiki/Interleaved_2_of_5</a>
 * is a great reference for Interleaved 2 of 5 information.</p>
 *
 * @author kevin.osullivan@sita.aero, SITA Lab.
 */

 const MAX_AVG_VARIANCE: f32 = 0.38f;

 const MAX_INDIVIDUAL_VARIANCE: f32 = 0.5f;

// Pixel width of a 3x wide line
 const W: i32 = 3;

// Pixel width of a 2x wide line
 let w: i32 = 2;

// Pixed width of a narrow line
 const N: i32 = 1;

/** Valid ITF lengths. Anything longer than the largest value is also allowed. */
 const DEFAULT_ALLOWED_LENGTHS: vec![Vec<i32>; 5] = vec![6, 8, 10, 12, 14, ]
;

/**
   * Start/end guard pattern.
   *
   * Note: The end pattern is reversed because the row is reversed before
   * searching for the END_PATTERN
   */
 const START_PATTERN: vec![Vec<i32>; 4] = vec![N, N, N, N, ]
;

 const END_PATTERN_REVERSED: vec![vec![Vec<Vec<i32>>; 3]; 2] = vec![// 2x
vec![N, N, w, ]
, // 3x
vec![N, N, W, ]
, ]
;

// See ITFWriter.PATTERNS
/**
   * Patterns of Wide / Narrow lines to indicate each digit
   */
 const PATTERNS: vec![vec![Vec<Vec<i32>>; 5]; 20] = vec![// 0
vec![N, N, w, w, N, ]
, // 1
vec![w, N, N, N, w, ]
, // 2
vec![N, w, N, N, w, ]
, // 3
vec![w, w, N, N, N, ]
, // 4
vec![N, N, w, N, w, ]
, // 5
vec![w, N, w, N, N, ]
, // 6
vec![N, w, w, N, N, ]
, // 7
vec![N, N, N, w, w, ]
, // 8
vec![w, N, N, w, N, ]
, // 9
vec![N, w, N, w, N, ]
, // 0
vec![N, N, W, W, N, ]
, // 1
vec![W, N, N, N, W, ]
, // 2
vec![N, W, N, N, W, ]
, // 3
vec![W, W, N, N, N, ]
, // 4
vec![N, N, W, N, W, ]
, // 5
vec![W, N, W, N, N, ]
, // 6
vec![N, W, W, N, N, ]
, // 7
vec![N, N, N, W, W, ]
, // 8
vec![W, N, N, W, N, ]
, // 9
vec![N, W, N, W, N, ]
, ]
;
pub struct ITFReader {
    super: OneDReader;

    // Stores the actual narrow line width of the image being decoded.
     let narrow_line_width: i32 = -1;
}

impl ITFReader {

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException, NotFoundException */Result<Result, Rc<Exception>>   {
        // Find out where the Middle section (payload) starts & ends
         let start_range: Vec<i32> = self.decode_start(row);
         let end_range: Vec<i32> = self.decode_end(row);
         let result: StringBuilder = StringBuilder::new(20);
        ::decode_middle(row, start_range[1], end_range[0], &result);
         let result_string: String = result.to_string();
         let allowed_lengths: Vec<i32> = null;
        if hints != null {
            allowed_lengths = hints.get(DecodeHintType::ALLOWED_LENGTHS) as Vec<i32>;
        }
        if allowed_lengths == null {
            allowed_lengths = DEFAULT_ALLOWED_LENGTHS;
        }
        // To avoid false positives with 2D barcodes (and other patterns), make
        // an assumption that the decoded string must be a 'standard' length if it's short
         let length: i32 = result_string.length();
         let length_o_k: bool = false;
         let max_allowed_length: i32 = 0;
        for  let allowed_length: i32 in allowed_lengths {
            if length == allowed_length {
                length_o_k = true;
                break;
            }
            if allowed_length > max_allowed_length {
                max_allowed_length = allowed_length;
            }
        }
        if !length_o_k && length > max_allowed_length {
            length_o_k = true;
        }
        if !length_o_k {
            throw FormatException::get_format_instance();
        }
         let result_object: Result = Result::new(&result_string, // no natural byte representation for these barcodes
        null,  : vec![ResultPoint; 2] = vec![ResultPoint::new(start_range[1], row_number), ResultPoint::new(end_range[0], row_number), ]
        , BarcodeFormat::ITF);
        result_object.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, "]I0");
        return Ok(result_object);
    }

    /**
   * @param row          row of black/white values to search
   * @param payloadStart offset of start pattern
   * @param resultString {@link StringBuilder} to append decoded chars to
   * @throws NotFoundException if decoding could not complete successfully
   */
    fn  decode_middle( row: &BitArray,  payload_start: i32,  payload_end: i32,  result_string: &StringBuilder)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
        // Digits are interleaved in pairs - 5 black lines for one digit, and the
        // 5
        // interleaved white lines for the second digit.
        // Therefore, need to scan 10 lines and then
        // split these into two arrays
         let counter_digit_pair: [i32; 10] = [0; 10];
         let counter_black: [i32; 5] = [0; 5];
         let counter_white: [i32; 5] = [0; 5];
        while payload_start < payload_end {
            // Get 10 runs of black/white.
            record_pattern(row, payload_start, &counter_digit_pair);
            // Split them into each array
             {
                 let mut k: i32 = 0;
                while k < 5 {
                    {
                         let two_k: i32 = 2 * k;
                        counter_black[k] = counter_digit_pair[two_k];
                        counter_white[k] = counter_digit_pair[two_k + 1];
                    }
                    k += 1;
                 }
             }

             let best_match: i32 = ::decode_digit(&counter_black);
            result_string.append(('0' + best_match) as char);
            best_match = ::decode_digit(&counter_white);
            result_string.append(('0' + best_match) as char);
            for  let counter_digit: i32 in counter_digit_pair {
                payload_start += counter_digit;
            }
        }
    }

    /**
   * Identify where the start of the middle / payload section starts.
   *
   * @param row row of black/white values to search
   * @return Array, containing index of start of 'start block' and end of
   *         'start block'
   */
    fn  decode_start(&self,  row: &BitArray) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let end_start: i32 = ::skip_white_space(row);
         let start_pattern: Vec<i32> = ::find_guard_pattern(row, end_start, &START_PATTERN);
        // Determine the width of a narrow line in pixels. We can do this by
        // getting the width of the start pattern and dividing by 4 because its
        // made up of 4 narrow lines.
        self.narrowLineWidth = (start_pattern[1] - start_pattern[0]) / 4;
        self.validate_quiet_zone(row, start_pattern[0]);
        return Ok(start_pattern);
    }

    /**
   * The start & end patterns must be pre/post fixed by a quiet zone. This
   * zone must be at least 10 times the width of a narrow line.  Scan back until
   * we either get to the start of the barcode or match the necessary number of
   * quiet zone pixels.
   *
   * Note: Its assumed the row is reversed when using this method to find
   * quiet zone after the end pattern.
   *
   * ref: http://www.barcode-1.net/i25code.html
   *
   * @param row bit array representing the scanned barcode.
   * @param startPattern index into row of the start or end pattern.
   * @throws NotFoundException if the quiet zone cannot be found
   */
    fn  validate_quiet_zone(&self,  row: &BitArray,  start_pattern: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
        // expect to find this many pixels of quiet zone
         let quiet_count: i32 = self.narrowLineWidth * 10;
        // if there are not so many pixel at all let's try as many as possible
        quiet_count = Math::min(quiet_count, start_pattern);
         {
             let mut i: i32 = start_pattern - 1;
            while quiet_count > 0 && i >= 0 {
                {
                    if row.get(i) {
                        break;
                    }
                    quiet_count -= 1;
                }
                i -= 1;
             }
         }

        if quiet_count != 0 {
            // Unable to find the necessary number of quiet zone pixels.
            throw NotFoundException::get_not_found_instance();
        }
    }

    /**
   * Skip all whitespace until we get to the first black line.
   *
   * @param row row of black/white values to search
   * @return index of the first black line.
   * @throws NotFoundException Throws exception if no black lines are found in the row
   */
    fn  skip_white_space( row: &BitArray) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let width: i32 = row.get_size();
         let end_start: i32 = row.get_next_set(0);
        if end_start == width {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(end_start);
    }

    /**
   * Identify where the end of the middle / payload section ends.
   *
   * @param row row of black/white values to search
   * @return Array, containing index of start of 'end block' and end of 'end
   *         block'
   */
    fn  decode_end(&self,  row: &BitArray) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
        // For convenience, reverse the row and then
        // search from 'the start' for the end block
        row.reverse();
        let tryResult1 = 0;
        'try1: loop {
        {
             let end_start: i32 = ::skip_white_space(row);
             let end_pattern: Vec<i32>;
            let tryResult2 = 0;
            'try2: loop {
            {
                end_pattern = ::find_guard_pattern(row, end_start, END_PATTERN_REVERSED[0]);
            }
            break 'try2
            }
            match tryResult2 {
                 catch ( nfe: &NotFoundException) {
                    end_pattern = ::find_guard_pattern(row, end_start, END_PATTERN_REVERSED[1]);
                }  0 => break
            }

            // The start & end patterns must be pre/post fixed by a quiet zone. This
            // zone must be at least 10 times the width of a narrow line.
            // ref: http://www.barcode-1.net/i25code.html
            self.validate_quiet_zone(row, end_pattern[0]);
            // Now recalculate the indices of where the 'endblock' starts & stops to
            // accommodate
            // the reversed nature of the search
             let temp: i32 = end_pattern[0];
            end_pattern[0] = row.get_size() - end_pattern[1];
            end_pattern[1] = row.get_size() - temp;
            return Ok(end_pattern);
        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }
         finally {
            // Put the row back the right way.
            row.reverse();
        }
    }

    /**
   * @param row       row of black/white values to search
   * @param rowOffset position to start search
   * @param pattern   pattern of counts of number of black and white pixels that are
   *                  being searched for as a pattern
   * @return start/end horizontal offset of guard pattern, as an array of two
   *         ints
   * @throws NotFoundException if pattern is not found
   */
    fn  find_guard_pattern( row: &BitArray,  row_offset: i32,  pattern: &Vec<i32>) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let pattern_length: i32 = pattern.len();
         let mut counters: [i32; pattern_length] = [0; pattern_length];
         let width: i32 = row.get_size();
         let is_white: bool = false;
         let counter_position: i32 = 0;
         let pattern_start: i32 = row_offset;
         {
             let mut x: i32 = row_offset;
            while x < width {
                {
                    if row.get(x) != is_white {
                        counters[counter_position] += 1;
                    } else {
                        if counter_position == pattern_length - 1 {
                            if pattern_match_variance(&counters, &pattern, MAX_INDIVIDUAL_VARIANCE) < MAX_AVG_VARIANCE {
                                return Ok( : vec![i32; 2] = vec![pattern_start, x, ]
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
                x += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    /**
   * Attempts to decode a sequence of ITF black/white lines into single
   * digit.
   *
   * @param counters the counts of runs of observed black/white/black/... values
   * @return The decoded digit
   * @throws NotFoundException if digit cannot be decoded
   */
    fn  decode_digit( counters: &Vec<i32>) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
        // worst variance we'll accept
         let best_variance: f32 = MAX_AVG_VARIANCE;
         let best_match: i32 = -1;
         let max: i32 = PATTERNS.len();
         {
             let mut i: i32 = 0;
            while i < max {
                {
                     let pattern: Vec<i32> = PATTERNS[i];
                     let variance: f32 = pattern_match_variance(&counters, &pattern, MAX_INDIVIDUAL_VARIANCE);
                    if variance < best_variance {
                        best_variance = variance;
                        best_match = i;
                    } else if variance == best_variance {
                        // if we find a second 'best match' with the same variance, we can not reliably report to have a suitable match
                        best_match = -1;
                    }
                }
                i += 1;
             }
         }

        if best_match >= 0 {
            return Ok(best_match % 10);
        } else {
            throw NotFoundException::get_not_found_instance();
        }
    }
}

// NEW FILE: i_t_f_writer.rs
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
 * This object renders a ITF code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */

 const START_PATTERN: vec![Vec<i32>; 4] = vec![1, 1, 1, 1, ]
;

 const END_PATTERN: vec![Vec<i32>; 3] = vec![3, 1, 1, ]
;

// Pixel width of a 3x wide line
 const W: i32 = 3;

// Pixed width of a narrow line
 const N: i32 = 1;

// See ITFReader.PATTERNS
 const PATTERNS: vec![vec![Vec<Vec<i32>>; 5]; 10] = vec![// 0
vec![N, N, W, W, N, ]
, // 1
vec![W, N, N, N, W, ]
, // 2
vec![N, W, N, N, W, ]
, // 3
vec![W, W, N, N, N, ]
, // 4
vec![N, N, W, N, W, ]
, // 5
vec![W, N, W, N, N, ]
, // 6
vec![N, W, W, N, N, ]
, // 7
vec![N, N, N, W, W, ]
, // 8
vec![W, N, N, W, N, ]
, // 9
vec![N, W, N, W, N, ]
, ]
;
pub struct ITFWriter {
    super: OneDimensionalCodeWriter;
}

impl ITFWriter {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::ITF);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let length: i32 = contents.length();
        if length % 2 != 0 {
            throw IllegalArgumentException::new("The length of the input should be even");
        }
        if length > 80 {
            throw IllegalArgumentException::new(format!("Requested contents should be less than 80 digits long, but got {}", length));
        }
        check_numeric(&contents);
         let result: [bool; 9 + 9 * length] = [false; 9 + 9 * length];
         let mut pos: i32 = append_pattern(&result, 0, &START_PATTERN, true);
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let one: i32 = Character::digit(&contents.char_at(i), 10);
                     let two: i32 = Character::digit(&contents.char_at(i + 1), 10);
                     let mut encoding: [i32; 10] = [0; 10];
                     {
                         let mut j: i32 = 0;
                        while j < 5 {
                            {
                                encoding[2 * j] = PATTERNS[one][j];
                                encoding[2 * j + 1] = PATTERNS[two][j];
                            }
                            j += 1;
                         }
                     }

                    pos += append_pattern(&result, pos, &encoding, true);
                }
                i += 2;
             }
         }

        append_pattern(&result, pos, &END_PATTERN, true);
        return result;
    }
}

// NEW FILE: multi_format_one_d_reader.rs
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
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */

 const EMPTY_ONED_ARRAY: [Option<OneDReader>; 0] = [None; 0];
pub struct MultiFormatOneDReader {
    super: OneDReader;

     let readers: Vec<OneDReader>;
}

impl MultiFormatOneDReader {

    pub fn new( hints: &HashMap<DecodeHintType, _>) -> MultiFormatOneDReader {
         let possible_formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
         let use_code39_check_digit: bool = hints != null && hints.get(DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT) != null;
         let mut readers: Collection<OneDReader> = ArrayList<>::new();
        if possible_formats != null {
            if possible_formats.contains(BarcodeFormat::EAN_13) || possible_formats.contains(BarcodeFormat::UPC_A) || possible_formats.contains(BarcodeFormat::EAN_8) || possible_formats.contains(BarcodeFormat::UPC_E) {
                readers.add(MultiFormatUPCEANReader::new(&hints));
            }
            if possible_formats.contains(BarcodeFormat::CODE_39) {
                readers.add(Code39Reader::new(use_code39_check_digit));
            }
            if possible_formats.contains(BarcodeFormat::CODE_93) {
                readers.add(Code93Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::CODE_128) {
                readers.add(Code128Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::ITF) {
                readers.add(ITFReader::new());
            }
            if possible_formats.contains(BarcodeFormat::CODABAR) {
                readers.add(CodaBarReader::new());
            }
            if possible_formats.contains(BarcodeFormat::RSS_14) {
                readers.add(RSS14Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::RSS_EXPANDED) {
                readers.add(RSSExpandedReader::new());
            }
        }
        if readers.is_empty() {
            readers.add(MultiFormatUPCEANReader::new(&hints));
            readers.add(Code39Reader::new());
            readers.add(CodaBarReader::new());
            readers.add(Code93Reader::new());
            readers.add(Code128Reader::new());
            readers.add(ITFReader::new());
            readers.add(RSS14Reader::new());
            readers.add(RSSExpandedReader::new());
        }
        let .readers = readers.to_array(EMPTY_ONED_ARRAY);
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        for  let reader: OneDReader in self.readers {
            let tryResult1 = 0;
            'try1: loop {
            {
                return Ok(reader.decode_row(row_number, row, &hints));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( re: &ReaderException) {
                }  0 => break
            }

        }
        throw NotFoundException::get_not_found_instance();
    }

    pub fn  reset(&self)   {
        for  let reader: Reader in self.readers {
            reader.reset();
        }
    }
}

// NEW FILE: multi_format_u_p_c_e_a_n_reader.rs
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
 * <p>A reader that can read all available UPC/EAN formats. If a caller wants to try to
 * read all such formats, it is most efficient to use this implementation rather than invoke
 * individual readers.</p>
 *
 * @author Sean Owen
 */

 const EMPTY_READER_ARRAY: [Option<UPCEANReader>; 0] = [None; 0];
pub struct MultiFormatUPCEANReader {
    super: OneDReader;

     let readers: Vec<UPCEANReader>;
}

impl MultiFormatUPCEANReader {

    pub fn new( hints: &Map<DecodeHintType, ?>) -> MultiFormatUPCEANReader {
         let possible_formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
         let mut readers: Collection<UPCEANReader> = ArrayList<>::new();
        if possible_formats != null {
            if possible_formats.contains(BarcodeFormat::EAN_13) {
                readers.add(EAN13Reader::new());
            } else if possible_formats.contains(BarcodeFormat::UPC_A) {
                readers.add(UPCAReader::new());
            }
            if possible_formats.contains(BarcodeFormat::EAN_8) {
                readers.add(EAN8Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::UPC_E) {
                readers.add(UPCEReader::new());
            }
        }
        if readers.is_empty() {
            readers.add(EAN13Reader::new());
            // UPC-A is covered by EAN-13
            readers.add(EAN8Reader::new());
            readers.add(UPCEReader::new());
        }
        let .readers = readers.to_array(EMPTY_READER_ARRAY);
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        // Compute this location once and reuse it on multiple implementations
         let start_guard_pattern: Vec<i32> = UPCEANReader::find_start_guard_pattern(row);
        for  let reader: UPCEANReader in self.readers {
            let tryResult1 = 0;
            'try1: loop {
            {
                 let result: Result = reader.decode_row(row_number, row, &start_guard_pattern, &hints);
                // Special case: a 12-digit code encoded in UPC-A is identical to a "0"
                // followed by those 12 digits encoded as EAN-13. Each will recognize such a code,
                // UPC-A as a 12-digit string and EAN-13 as a 13-digit string starting with "0".
                // Individually these are correct and their readers will both read such a code
                // and correctly call it EAN-13, or UPC-A, respectively.
                //
                // In this case, if we've been looking for both types, we'd like to call it
                // a UPC-A code. But for efficiency we only run the EAN-13 decoder to also read
                // UPC-A. So we special case it here, and convert an EAN-13 result to a UPC-A
                // result if appropriate.
                //
                // But, don't return UPC-A if UPC-A was not a requested format!
                 let ean13_may_be_u_p_c_a: bool = result.get_barcode_format() == BarcodeFormat::EAN_13 && result.get_text().char_at(0) == '0';
                 let possible_formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
                 let can_return_u_p_c_a: bool = possible_formats == null || possible_formats.contains(BarcodeFormat::UPC_A);
                if ean13_may_be_u_p_c_a && can_return_u_p_c_a {
                    // Transfer the metadata across
                     let result_u_p_c_a: Result = Result::new(&result.get_text().substring(1), &result.get_raw_bytes(), &result.get_result_points(), BarcodeFormat::UPC_A);
                    result_u_p_c_a.put_all_metadata(&result.get_result_metadata());
                    return Ok(result_u_p_c_a);
                }
                return Ok(result);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( ignored: &ReaderException) {
                }  0 => break
            }

        }
        throw NotFoundException::get_not_found_instance();
    }

    pub fn  reset(&self)   {
        for  let reader: Reader in self.readers {
            reader.reset();
        }
    }
}

// NEW FILE: one_d_reader.rs
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
 * Encapsulates functionality and implementation that is common to all families
 * of one-dimensional barcodes.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
#[derive(Reader)]
pub struct OneDReader {
}

impl OneDReader {

    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode(image, null));
    }

    // Note that we don't try rotation without the try harder flag, even if rotation was supported.
    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(self.do_decode(image, &hints));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( nfe: &NotFoundException) {
                 let try_harder: bool = hints != null && hints.contains_key(DecodeHintType::TRY_HARDER);
                if try_harder && image.is_rotate_supported() {
                     let rotated_image: BinaryBitmap = image.rotate_counter_clockwise();
                     let result: Result = self.do_decode(rotated_image, &hints);
                     let metadata: Map<ResultMetadataType, ?> = result.get_result_metadata();
                     let mut orientation: i32 = 270;
                    if metadata != null && metadata.contains_key(ResultMetadataType::ORIENTATION) {
                        orientation = (orientation + metadata.get(ResultMetadataType::ORIENTATION) as Integer) % 360;
                    }
                    result.put_metadata(ResultMetadataType::ORIENTATION, orientation);
                     let mut points: Vec<ResultPoint> = result.get_result_points();
                    if points != null {
                         let height: i32 = rotated_image.get_height();
                         {
                             let mut i: i32 = 0;
                            while i < points.len() {
                                {
                                    points[i] = ResultPoint::new(height - points[i].get_y() - 1, &points[i].get_x());
                                }
                                i += 1;
                             }
                         }

                    }
                    return Ok(result);
                } else {
                    throw nfe;
                }
            }  0 => break
        }

    }

    pub fn  reset(&self)   {
    // do nothing
    }

    /**
   * We're going to examine rows from the middle outward, searching alternately above and below the
   * middle, and farther out each time. rowStep is the number of rows between each successive
   * attempt above and below the middle. So we'd scan row middle, then middle - rowStep, then
   * middle + rowStep, then middle - (2 * rowStep), etc.
   * rowStep is bigger as the image is taller, but is always at least 1. We've somewhat arbitrarily
   * decided that moving up and down by about 1/16 of the image is pretty good; we try more of the
   * image if "trying harder".
   *
   * @param image The image to decode
   * @param hints Any hints that were requested
   * @return The contents of the decoded barcode
   * @throws NotFoundException Any spontaneous errors which occur
   */
    fn  do_decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
         let width: i32 = image.get_width();
         let height: i32 = image.get_height();
         let mut row: BitArray = BitArray::new(width);
         let try_harder: bool = hints != null && hints.contains_key(DecodeHintType::TRY_HARDER);
         let row_step: i32 = Math::max(1, height >> ( if try_harder { 8 } else { 5 }));
         let max_lines: i32;
        if try_harder {
            // Look at the whole image, not just the center
            max_lines = height;
        } else {
            // 15 rows spaced 1/32 apart is roughly the middle half of the image
            max_lines = 15;
        }
         let middle: i32 = height / 2;
         {
             let mut x: i32 = 0;
            while x < max_lines {
                {
                    // Scanning from the middle out. Determine which row we're looking at next:
                     let row_steps_above_or_below: i32 = (x + 1) / 2;
                    // i.e. is x even?
                     let is_above: bool = (x & 0x01) == 0;
                     let row_number: i32 = middle + row_step * ( if is_above { row_steps_above_or_below } else { -row_steps_above_or_below });
                    if row_number < 0 || row_number >= height {
                        // Oops, if we run off the top or bottom, stop
                        break;
                    }
                    // Estimate black point for this row and load it:
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        row = image.get_black_row(row_number, row);
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( ignored: &NotFoundException) {
                            continue;
                        }  0 => break
                    }

                    // handle decoding upside down barcodes.
                     {
                         let mut attempt: i32 = 0;
                        while attempt < 2 {
                            {
                                if attempt == 1 {
                                    // trying again?
                                    // reverse the row and continue
                                    row.reverse();
                                    // that start on the center line.
                                    if hints != null && hints.contains_key(DecodeHintType::NEED_RESULT_POINT_CALLBACK) {
                                         let new_hints: Map<DecodeHintType, Object> = EnumMap<>::new(DecodeHintType.class);
                                        new_hints.put_all(&hints);
                                        new_hints.remove(DecodeHintType::NEED_RESULT_POINT_CALLBACK);
                                        hints = new_hints;
                                    }
                                }
                                let tryResult1 = 0;
                                'try1: loop {
                                {
                                    // Look for a barcode
                                     let result: Result = self.decode_row(row_number, row, &hints);
                                    // We found our barcode
                                    if attempt == 1 {
                                        // But it was upside down, so note that
                                        result.put_metadata(ResultMetadataType::ORIENTATION, 180);
                                        // And remember to flip the result points horizontally.
                                         let mut points: Vec<ResultPoint> = result.get_result_points();
                                        if points != null {
                                            points[0] = ResultPoint::new(width - points[0].get_x() - 1, &points[0].get_y());
                                            points[1] = ResultPoint::new(width - points[1].get_x() - 1, &points[1].get_y());
                                        }
                                    }
                                    return Ok(result);
                                }
                                break 'try1
                                }
                                match tryResult1 {
                                     catch ( re: &ReaderException) {
                                    }  0 => break
                                }

                            }
                            attempt += 1;
                         }
                     }

                }
                x += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    /**
   * Records the size of successive runs of white and black pixels in a row, starting at a given point.
   * The values are recorded in the given array, and the number of runs recorded is equal to the size
   * of the array. If the row starts on a white pixel at the given start point, then the first count
   * recorded is the run of white pixels starting from that point; likewise it is the count of a run
   * of black pixels if the row begin on a black pixels at that point.
   *
   * @param row row to count from
   * @param start offset into row to start at
   * @param counters array into which to record counts
   * @throws NotFoundException if counters cannot be filled entirely from row before running out
   *  of pixels
   */
    pub fn  record_pattern( row: &BitArray,  start: i32,  counters: &Vec<i32>)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         let num_counters: i32 = counters.len();
        Arrays::fill(&counters, 0, num_counters, 0);
         let end: i32 = row.get_size();
        if start >= end {
            throw NotFoundException::get_not_found_instance();
        }
         let is_white: bool = !row.get(start);
         let counter_position: i32 = 0;
         let mut i: i32 = start;
        while i < end {
            if row.get(i) != is_white {
                counters[counter_position] += 1;
            } else {
                if counter_position += 1 == num_counters {
                    break;
                } else {
                    counters[counter_position] = 1;
                    is_white = !is_white;
                }
            }
            i += 1;
        }
        // the last counter but ran off the side of the image, OK. Otherwise, a problem.
        if !(counter_position == num_counters || (counter_position == num_counters - 1 && i == end)) {
            throw NotFoundException::get_not_found_instance();
        }
    }

    pub fn  record_pattern_in_reverse( row: &BitArray,  start: i32,  counters: &Vec<i32>)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
        // This could be more efficient I guess
         let num_transitions_left: i32 = counters.len();
         let mut last: bool = row.get(start);
        while start > 0 && num_transitions_left >= 0 {
            if row.get(start -= 1) != last {
                num_transitions_left -= 1;
                last = !last;
            }
        }
        if num_transitions_left >= 0 {
            throw NotFoundException::get_not_found_instance();
        }
        ::record_pattern(row, start + 1, &counters);
    }

    /**
   * Determines how closely a set of observed counts of runs of black/white values matches a given
   * target pattern. This is reported as the ratio of the total variance from the expected pattern
   * proportions across all pattern elements, to the length of the pattern.
   *
   * @param counters observed counters
   * @param pattern expected pattern
   * @param maxIndividualVariance The most any counter can differ before we give up
   * @return ratio of total variance between counters and pattern compared to total pattern size
   */
    pub fn  pattern_match_variance( counters: &Vec<i32>,  pattern: &Vec<i32>,  max_individual_variance: f32) -> f32  {
         let num_counters: i32 = counters.len();
         let mut total: i32 = 0;
         let pattern_length: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < num_counters {
                {
                    total += counters[i];
                    pattern_length += pattern[i];
                }
                i += 1;
             }
         }

        if total < pattern_length {
            // to reliably match, so fail:
            return Float::POSITIVE_INFINITY;
        }
         let unit_bar_width: f32 = total as f32 / pattern_length;
        max_individual_variance *= unit_bar_width;
         let total_variance: f32 = 0.0f;
         {
             let mut x: i32 = 0;
            while x < num_counters {
                {
                     let counter: i32 = counters[x];
                     let scaled_pattern: f32 = pattern[x] * unit_bar_width;
                     let variance: f32 =  if counter > scaled_pattern { counter - scaled_pattern } else { scaled_pattern - counter };
                    if variance > max_individual_variance {
                        return Float::POSITIVE_INFINITY;
                    }
                    total_variance += variance;
                }
                x += 1;
             }
         }

        return total_variance / total;
    }

    /**
   * <p>Attempts to decode a one-dimensional barcode format given a single row of
   * an image.</p>
   *
   * @param rowNumber row number from top of the row
   * @param row the black/white pixel data of the row
   * @param hints decode hints
   * @return {@link Result} containing encoded string and start/end of barcode
   * @throws NotFoundException if no potential barcode is found
   * @throws ChecksumException if a potential barcode is found but does not pass its checksum
   * @throws FormatException if a potential barcode is found but format is invalid
   */
    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>  ;
}

// NEW FILE: one_dimensional_code_writer.rs
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
 * <p>Encapsulates functionality and implementation that is common to one-dimensional barcodes.</p>
 *
 * @author dsbnatut@gmail.com (Kazuki Nishiura)
 */

 const NUMERIC: Pattern = Pattern::compile("[0-9]+");
#[derive(Writer)]
pub struct OneDimensionalCodeWriter {
}

impl OneDimensionalCodeWriter {

    /**
   * Encode the contents to boolean array expression of one-dimensional barcode.
   * Start code and end code should be included in result, and side margins should not be included.
   *
   * @param contents barcode contents to encode
   * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
   */
    pub fn  encode(&self,  contents: &String) -> Vec<bool> ;

    /**
   * Can be overwritten if the encode requires to read the hints map. Otherwise it defaults to {@code encode}.
   * @param contents barcode contents to encode
   * @param hints encoding hints
   * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
   */
    pub fn  encode(&self,  contents: &String,  hints: &Map<EncodeHintType, ?>) -> Vec<bool>  {
        return self.encode(&contents);
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> BitMatrix  {
        return self.encode(&contents, format, width, height, null);
    }

    /**
   * Encode the contents following specified format.
   * {@code width} and {@code height} are required size. This method may return bigger size
   * {@code BitMatrix} when specified size is too small. The user can set both {@code width} and
   * {@code height} to zero to get minimum size barcode. If negative value is set to {@code width}
   * or {@code height}, {@code IllegalArgumentException} is thrown.
   */
    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> BitMatrix  {
        if contents.is_empty() {
            throw IllegalArgumentException::new("Found empty contents");
        }
        if width < 0 || height < 0 {
            throw IllegalArgumentException::new(format!("Negative size is not allowed. Input: {}x{}", width, height));
        }
         let supported_formats: Collection<BarcodeFormat> = self.get_supported_write_formats();
        if supported_formats != null && !supported_formats.contains(format) {
            throw IllegalArgumentException::new(format!("Can only encode {}, but got {}", supported_formats, format));
        }
         let sides_margin: i32 = self.get_default_margin();
        if hints != null && hints.contains_key(EncodeHintType::MARGIN) {
            sides_margin = Integer::parse_int(&hints.get(EncodeHintType::MARGIN).to_string());
        }
         let code: Vec<bool> = self.encode(&contents, &hints);
        return ::render_result(&code, width, height, sides_margin);
    }

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return null;
    }

    /**
   * @return a byte array of horizontal pixels (0 = white, 1 = black)
   */
    fn  render_result( code: &Vec<bool>,  width: i32,  height: i32,  sides_margin: i32) -> BitMatrix  {
         let input_width: i32 = code.len();
        // Add quiet zone on both sides.
         let full_width: i32 = input_width + sides_margin;
         let output_width: i32 = Math::max(width, full_width);
         let output_height: i32 = Math::max(1, height);
         let multiple: i32 = output_width / full_width;
         let left_padding: i32 = (output_width - (input_width * multiple)) / 2;
         let output: BitMatrix = BitMatrix::new(output_width, output_height);
         {
             let input_x: i32 = 0, let output_x: i32 = left_padding;
            while input_x < input_width {
                {
                    if code[input_x] {
                        output.set_region(output_x, 0, multiple, output_height);
                    }
                }
                input_x += 1;
                output_x += multiple;
             }
         }

        return output;
    }

    /**
   * @param contents string to check for numeric characters
   * @throws IllegalArgumentException if input contains characters other than digits 0-9.
   */
    pub fn  check_numeric( contents: &String)   {
        if !NUMERIC::matcher(&contents)::matches() {
            throw IllegalArgumentException::new("Input should only contain digits 0-9");
        }
    }

    /**
   * @param target encode black/white pattern into this array
   * @param pos position to start encoding at in {@code target}
   * @param pattern lengths of black/white runs to encode
   * @param startColor starting color - false for white, true for black
   * @return the number of elements added to target.
   */
    pub fn  append_pattern( target: &Vec<bool>,  pos: i32,  pattern: &Vec<i32>,  start_color: bool) -> i32  {
         let mut color: bool = start_color;
         let num_added: i32 = 0;
        for  let len: i32 in pattern {
             {
                 let mut j: i32 = 0;
                while j < len {
                    {
                        target[pos += 1 !!!check!!! post increment] = color;
                    }
                    j += 1;
                 }
             }

            num_added += len;
            // flip color after each segment
            color = !color;
        }
        return num_added;
    }

    pub fn  get_default_margin(&self) -> i32  {
        // This seems like a decent idea for a default for all formats.
        return 10;
    }
}

// NEW FILE: u_p_c_a_reader.rs
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
 * <p>Implements decoding of the UPC-A format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
pub struct UPCAReader {
    super: UPCEANReader;

     let ean13_reader: UPCEANReader = EAN13Reader::new();
}

impl UPCAReader {

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  start_guard_range: &Vec<i32>,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode_row(row_number, row, &start_guard_range, &hints)));
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode_row(row_number, row, &hints)));
    }

    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode(image)));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode(image, &hints)));
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::UPC_A;
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
        return Ok(self.ean13_reader.decode_middle(row, &start_range, &result_string));
    }

    fn  maybe_return_result( result: &Result) -> /*  throws FormatException */Result<Result, Rc<Exception>>   {
         let text: String = result.get_text();
        if text.char_at(0) == '0' {
             let upca_result: Result = Result::new(&text.substring(1), null, &result.get_result_points(), BarcodeFormat::UPC_A);
            if result.get_result_metadata() != null {
                upca_result.put_all_metadata(&result.get_result_metadata());
            }
            return Ok(upca_result);
        } else {
            throw FormatException::get_format_instance();
        }
    }
}

// NEW FILE: u_p_c_a_writer.rs
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
 * This object renders a UPC-A code as a {@link BitMatrix}.
 *
 * @author qwandor@google.com (Andrew Walbran)
 */
#[derive(Writer)]
pub struct UPCAWriter {

     let sub_writer: EAN13Writer = EAN13Writer::new();
}

impl UPCAWriter {

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> BitMatrix  {
        return self.encode(&contents, format, width, height, null);
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> BitMatrix  {
        if format != BarcodeFormat::UPC_A {
            throw IllegalArgumentException::new(format!("Can only encode UPC-A, but got {}", format));
        }
        // Transform a UPC-A code into the equivalent EAN-13 code and write it that way
        return self.sub_writer.encode(format!("0{}", contents), BarcodeFormat::EAN_13, width, height, &hints);
    }
}

// NEW FILE: u_p_c_e_a_n_extension2_support.rs
/*
 * Copyright (C) 2012 ZXing authors
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
 * @see UPCEANExtension5Support
 */
struct UPCEANExtension2Support {

     let decode_middle_counters: [i32; 4] = [0; 4];

     let decode_row_string_buffer: StringBuilder = StringBuilder::new();
}

impl UPCEANExtension2Support {

    fn  decode_row(&self,  row_number: i32,  row: &BitArray,  extension_start_range: &Vec<i32>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
         let result: StringBuilder = self.decode_row_string_buffer;
        result.set_length(0);
         let end: i32 = self.decode_middle(row, &extension_start_range, &result);
         let result_string: String = result.to_string();
         let extension_data: Map<ResultMetadataType, Object> = ::parse_extension_string(&result_string);
         let extension_result: Result = Result::new(&result_string, null,  : vec![ResultPoint; 2] = vec![ResultPoint::new((extension_start_range[0] + extension_start_range[1]) / 2.0f, row_number), ResultPoint::new(end, row_number), ]
        , BarcodeFormat::UPC_EAN_EXTENSION);
        if extension_data != null {
            extension_result.put_all_metadata(&extension_data);
        }
        return Ok(extension_result);
    }

    fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.decode_middle_counters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let end: i32 = row.get_size();
         let row_offset: i32 = start_range[1];
         let check_parity: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < 2 && row_offset < end {
                {
                     let best_match: i32 = UPCEANReader::decode_digit(row, &counters, row_offset, UPCEANReader.L_AND_G_PATTERNS);
                    result_string.append(('0' + best_match % 10) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                    if best_match >= 10 {
                        check_parity |= 1 << (1 - x);
                    }
                    if x != 1 {
                        // Read off separator if not last
                        row_offset = row.get_next_set(row_offset);
                        row_offset = row.get_next_unset(row_offset);
                    }
                }
                x += 1;
             }
         }

        if result_string.length() != 2 {
            throw NotFoundException::get_not_found_instance();
        }
        if Integer::parse_int(&result_string.to_string()) % 4 != check_parity {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(row_offset);
    }

    /**
   * @param raw raw content of extension
   * @return formatted interpretation of raw content as a {@link Map} mapping
   *  one {@link ResultMetadataType} to appropriate value, or {@code null} if not known
   */
    fn  parse_extension_string( raw: &String) -> Map<ResultMetadataType, Object>  {
        if raw.length() != 2 {
            return null;
        }
         let result: Map<ResultMetadataType, Object> = EnumMap<>::new(ResultMetadataType.class);
        result.put(ResultMetadataType::ISSUE_NUMBER, &Integer::value_of(&raw));
        return result;
    }
}

// NEW FILE: u_p_c_e_a_n_extension5_support.rs
/*
 * Copyright (C) 2010 ZXing authors
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
 * @see UPCEANExtension2Support
 */

 const CHECK_DIGIT_ENCODINGS: vec![Vec<i32>; 10] = vec![0x18, 0x14, 0x12, 0x11, 0x0C, 0x06, 0x03, 0x0A, 0x09, 0x05, ]
;
struct UPCEANExtension5Support {

     let decode_middle_counters: [i32; 4] = [0; 4];

     let decode_row_string_buffer: StringBuilder = StringBuilder::new();
}

impl UPCEANExtension5Support {

    fn  decode_row(&self,  row_number: i32,  row: &BitArray,  extension_start_range: &Vec<i32>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
         let result: StringBuilder = self.decode_row_string_buffer;
        result.set_length(0);
         let end: i32 = self.decode_middle(row, &extension_start_range, &result);
         let result_string: String = result.to_string();
         let extension_data: Map<ResultMetadataType, Object> = ::parse_extension_string(&result_string);
         let extension_result: Result = Result::new(&result_string, null,  : vec![ResultPoint; 2] = vec![ResultPoint::new((extension_start_range[0] + extension_start_range[1]) / 2.0f, row_number), ResultPoint::new(end, row_number), ]
        , BarcodeFormat::UPC_EAN_EXTENSION);
        if extension_data != null {
            extension_result.put_all_metadata(&extension_data);
        }
        return Ok(extension_result);
    }

    fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.decode_middle_counters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let end: i32 = row.get_size();
         let row_offset: i32 = start_range[1];
         let lg_pattern_found: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < 5 && row_offset < end {
                {
                     let best_match: i32 = UPCEANReader::decode_digit(row, &counters, row_offset, UPCEANReader.L_AND_G_PATTERNS);
                    result_string.append(('0' + best_match % 10) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                    if best_match >= 10 {
                        lg_pattern_found |= 1 << (4 - x);
                    }
                    if x != 4 {
                        // Read off separator if not last
                        row_offset = row.get_next_set(row_offset);
                        row_offset = row.get_next_unset(row_offset);
                    }
                }
                x += 1;
             }
         }

        if result_string.length() != 5 {
            throw NotFoundException::get_not_found_instance();
        }
         let check_digit: i32 = ::determine_check_digit(lg_pattern_found);
        if ::extension_checksum(&result_string.to_string()) != check_digit {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(row_offset);
    }

    fn  extension_checksum( s: &CharSequence) -> i32  {
         let length: i32 = s.length();
         let mut sum: i32 = 0;
         {
             let mut i: i32 = length - 2;
            while i >= 0 {
                {
                    sum += s.char_at(i) - '0';
                }
                i -= 2;
             }
         }

        sum *= 3;
         {
             let mut i: i32 = length - 1;
            while i >= 0 {
                {
                    sum += s.char_at(i) - '0';
                }
                i -= 2;
             }
         }

        sum *= 3;
        return sum % 10;
    }

    fn  determine_check_digit( lg_pattern_found: i32) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         {
             let mut d: i32 = 0;
            while d < 10 {
                {
                    if lg_pattern_found == CHECK_DIGIT_ENCODINGS[d] {
                        return Ok(d);
                    }
                }
                d += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    /**
   * @param raw raw content of extension
   * @return formatted interpretation of raw content as a {@link Map} mapping
   *  one {@link ResultMetadataType} to appropriate value, or {@code null} if not known
   */
    fn  parse_extension_string( raw: &String) -> Map<ResultMetadataType, Object>  {
        if raw.length() != 5 {
            return null;
        }
         let value: Object = ::parse_extension5_string(&raw);
        if value == null {
            return null;
        }
         let result: Map<ResultMetadataType, Object> = EnumMap<>::new(ResultMetadataType.class);
        result.put(ResultMetadataType::SUGGESTED_PRICE, &value);
        return result;
    }

    fn  parse_extension5_string( raw: &String) -> String  {
         let mut currency: String;
        match raw.char_at(0) {
              '0' => 
                 {
                    currency = "Â£";
                    break;
                }
              '5' => 
                 {
                    currency = "$";
                    break;
                }
              '9' => 
                 {
                    // Reference: http://www.jollytech.com
                    match raw {
                          "90000" => 
                             {
                                // No suggested retail price
                                return null;
                            }
                          "99991" => 
                             {
                                // Complementary
                                return "0.00";
                            }
                          "99990" => 
                             {
                                return "Used";
                            }
                    }
                    // Otherwise... unknown currency?
                    currency = "";
                    break;
                }
            _ => 
                 {
                    currency = "";
                    break;
                }
        }
         let raw_amount: i32 = Integer::parse_int(&raw.substring(1));
         let units_string: String = String::value_of(raw_amount / 100);
         let hundredths: i32 = raw_amount % 100;
         let hundredths_string: String =  if hundredths < 10 { format!("0{}", hundredths) } else { String::value_of(hundredths) };
        return format!("{}{}.{}", currency, units_string, hundredths_string);
    }
}

// NEW FILE: u_p_c_e_a_n_extension_support.rs
/*
 * Copyright (C) 2010 ZXing authors
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


 const EXTENSION_START_PATTERN: vec![Vec<i32>; 3] = vec![1, 1, 2, ]
;
struct UPCEANExtensionSupport {

     let two_support: UPCEANExtension2Support = UPCEANExtension2Support::new();

     let five_support: UPCEANExtension5Support = UPCEANExtension5Support::new();
}

impl UPCEANExtensionSupport {

    fn  decode_row(&self,  row_number: i32,  row: &BitArray,  row_offset: i32) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
         let extension_start_range: Vec<i32> = UPCEANReader::find_guard_pattern(row, row_offset, false, &EXTENSION_START_PATTERN);
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(self.five_support.decode_row(row_number, row, &extension_start_range));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &ReaderException) {
                return Ok(self.two_support.decode_row(row_number, row, &extension_start_range));
            }  0 => break
        }

    }
}

// NEW FILE: u_p_c_e_a_n_reader.rs
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
 * <p>Encapsulates functionality and implementation that is common to UPC and EAN families
 * of one-dimensional barcodes.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */

// These two values are critical for determining how permissive the decoding will be.
// We've arrived at these values through a lot of trial and error. Setting them any higher
// lets false positives creep in quickly.
 const MAX_AVG_VARIANCE: f32 = 0.48f;

 const MAX_INDIVIDUAL_VARIANCE: f32 = 0.7f;

/**
   * Start/end guard pattern.
   */
 const START_END_PATTERN: vec![Vec<i32>; 3] = vec![1, 1, 1, ]
;

/**
   * Pattern marking the middle of a UPC/EAN pattern, separating the two halves.
   */
 const MIDDLE_PATTERN: vec![Vec<i32>; 5] = vec![1, 1, 1, 1, 1, ]
;

/**
   * end guard pattern.
   */
 const END_PATTERN: vec![Vec<i32>; 6] = vec![1, 1, 1, 1, 1, 1, ]
;

/**
   * "Odd", or "L" patterns used to encode UPC/EAN digits.
   */
 const L_PATTERNS: vec![vec![Vec<Vec<i32>>; 4]; 10] = vec![// 0
vec![3, 2, 1, 1, ]
, // 1
vec![2, 2, 2, 1, ]
, // 2
vec![2, 1, 2, 2, ]
, // 3
vec![1, 4, 1, 1, ]
, // 4
vec![1, 1, 3, 2, ]
, // 5
vec![1, 2, 3, 1, ]
, // 6
vec![1, 1, 1, 4, ]
, // 7
vec![1, 3, 1, 2, ]
, // 8
vec![1, 2, 1, 3, ]
, // 9
vec![3, 1, 1, 2, ]
, ]
;

/**
   * As above but also including the "even", or "G" patterns used to encode UPC/EAN digits.
   */
 const L_AND_G_PATTERNS: Vec<Vec<i32>>;
pub struct UPCEANReader {
    super: OneDReader;

     let decode_row_string_buffer: StringBuilder;

     let extension_reader: UPCEANExtensionSupport;

     let ean_man_support: EANManufacturerOrgSupport;
}

impl UPCEANReader {

    static {
        L_AND_G_PATTERNS = : [i32; 20] = [0; 20];
        System::arraycopy(&L_PATTERNS, 0, &L_AND_G_PATTERNS, 0, 10);
         {
             let mut i: i32 = 10;
            while i < 20 {
                {
                     let widths: Vec<i32> = L_PATTERNS[i - 10];
                     let reversed_widths: [i32; widths.len()] = [0; widths.len()];
                     {
                         let mut j: i32 = 0;
                        while j < widths.len() {
                            {
                                reversed_widths[j] = widths[widths.len() - j - 1];
                            }
                            j += 1;
                         }
                     }

                    L_AND_G_PATTERNS[i] = reversed_widths;
                }
                i += 1;
             }
         }

    }

    pub fn new() -> UPCEANReader {
        decode_row_string_buffer = StringBuilder::new(20);
        extension_reader = UPCEANExtensionSupport::new();
        ean_man_support = EANManufacturerOrgSupport::new();
    }

    fn  find_start_guard_pattern( row: &BitArray) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let found_start: bool = false;
         let start_range: Vec<i32> = null;
         let next_start: i32 = 0;
         let counters: [i32; START_END_PATTERN.len()] = [0; START_END_PATTERN.len()];
        while !found_start {
            Arrays::fill(&counters, 0, START_END_PATTERN.len(), 0);
            start_range = ::find_guard_pattern(row, next_start, false, &START_END_PATTERN, &counters);
             let start: i32 = start_range[0];
            next_start = start_range[1];
            // Make sure there is a quiet zone at least as big as the start pattern before the barcode.
            // If this check would run off the left edge of the image, do not accept this barcode,
            // as it is very likely to be a false positive.
             let quiet_start: i32 = start - (next_start - start);
            if quiet_start >= 0 {
                found_start = row.is_range(quiet_start, start, false);
            }
        }
        return Ok(start_range);
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode_row(row_number, row, &::find_start_guard_pattern(row), &hints));
    }

    /**
   * <p>Like {@link #decodeRow(int, BitArray, Map)}, but
   * allows caller to inform method about where the UPC/EAN start pattern is
   * found. This allows this to be computed once and reused across many implementations.</p>
   *
   * @param rowNumber row index into the image
   * @param row encoding of the row of the barcode image
   * @param startGuardRange start/end column where the opening start pattern was found
   * @param hints optional hints that influence decoding
   * @return {@link Result} encapsulating the result of decoding a barcode in the row
   * @throws NotFoundException if no potential barcode is found
   * @throws ChecksumException if a potential barcode is found but does not pass its checksum
   * @throws FormatException if a potential barcode is found but format is invalid
   */
    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  start_guard_range: &Vec<i32>,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
         let result_point_callback: ResultPointCallback =  if hints == null { null } else { hints.get(DecodeHintType::NEED_RESULT_POINT_CALLBACK) as ResultPointCallback };
         let symbology_identifier: i32 = 0;
        if result_point_callback != null {
            result_point_callback.found_possible_result_point(ResultPoint::new((start_guard_range[0] + start_guard_range[1]) / 2.0f, row_number));
        }
         let result: StringBuilder = self.decode_row_string_buffer;
        result.set_length(0);
         let end_start: i32 = self.decode_middle(row, &start_guard_range, &result);
        if result_point_callback != null {
            result_point_callback.found_possible_result_point(ResultPoint::new(end_start, row_number));
        }
         let end_range: Vec<i32> = self.decode_end(row, end_start);
        if result_point_callback != null {
            result_point_callback.found_possible_result_point(ResultPoint::new((end_range[0] + end_range[1]) / 2.0f, row_number));
        }
        // Make sure there is a quiet zone at least as big as the end pattern after the barcode. The
        // spec might want more whitespace, but in practice this is the maximum we can count on.
         let end: i32 = end_range[1];
         let quiet_end: i32 = end + (end - end_range[0]);
        if quiet_end >= row.get_size() || !row.is_range(end, quiet_end, false) {
            throw NotFoundException::get_not_found_instance();
        }
         let result_string: String = result.to_string();
        // UPC/EAN should never be less than 8 chars anyway
        if result_string.length() < 8 {
            throw FormatException::get_format_instance();
        }
        if !self.check_checksum(&result_string) {
            throw ChecksumException::get_checksum_instance();
        }
         let left: f32 = (start_guard_range[1] + start_guard_range[0]) / 2.0f;
         let right: f32 = (end_range[1] + end_range[0]) / 2.0f;
         let format: BarcodeFormat = self.get_barcode_format();
         let decode_result: Result = Result::new(&result_string, // no natural byte representation for these barcodes
        null,  : vec![ResultPoint; 2] = vec![ResultPoint::new(left, row_number), ResultPoint::new(right, row_number), ]
        , format);
         let extension_length: i32 = 0;
        let tryResult1 = 0;
        'try1: loop {
        {
             let extension_result: Result = self.extension_reader.decode_row(row_number, row, end_range[1]);
            decode_result.put_metadata(ResultMetadataType::UPC_EAN_EXTENSION, &extension_result.get_text());
            decode_result.put_all_metadata(&extension_result.get_result_metadata());
            decode_result.add_result_points(&extension_result.get_result_points());
            extension_length = extension_result.get_text().length();
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &ReaderException) {
            }  0 => break
        }

         let allowed_extensions: Vec<i32> =  if hints == null { null } else { hints.get(DecodeHintType::ALLOWED_EAN_EXTENSIONS) as Vec<i32> };
        if allowed_extensions != null {
             let mut valid: bool = false;
            for  let length: i32 in allowed_extensions {
                if extension_length == length {
                    valid = true;
                    break;
                }
            }
            if !valid {
                throw NotFoundException::get_not_found_instance();
            }
        }
        if format == BarcodeFormat::EAN_13 || format == BarcodeFormat::UPC_A {
             let country_i_d: String = self.ean_man_support.lookup_country_identifier(&result_string);
            if country_i_d != null {
                decode_result.put_metadata(ResultMetadataType::POSSIBLE_COUNTRY, &country_i_d);
            }
        }
        if format == BarcodeFormat::EAN_8 {
            symbology_identifier = 4;
        }
        decode_result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, format!("]E{}", symbology_identifier));
        return Ok(decode_result);
    }

    /**
   * @param s string of digits to check
   * @return {@link #checkStandardUPCEANChecksum(CharSequence)}
   * @throws FormatException if the string does not contain only digits
   */
    fn  check_checksum(&self,  s: &String) -> /*  throws FormatException */Result<bool, Rc<Exception>>   {
        return Ok(::check_standard_u_p_c_e_a_n_checksum(&s));
    }

    /**
   * Computes the UPC/EAN checksum on a string of digits, and reports
   * whether the checksum is correct or not.
   *
   * @param s string of digits to check
   * @return true iff string of digits passes the UPC/EAN checksum algorithm
   * @throws FormatException if the string does not contain only digits
   */
    fn  check_standard_u_p_c_e_a_n_checksum( s: &CharSequence) -> /*  throws FormatException */Result<bool, Rc<Exception>>   {
         let length: i32 = s.length();
        if length == 0 {
            return Ok(false);
        }
         let check: i32 = Character::digit(&s.char_at(length - 1), 10);
        return Ok(::get_standard_u_p_c_e_a_n_checksum(&s.sub_sequence(0, length - 1)) == check);
    }

    fn  get_standard_u_p_c_e_a_n_checksum( s: &CharSequence) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
         let length: i32 = s.length();
         let mut sum: i32 = 0;
         {
             let mut i: i32 = length - 1;
            while i >= 0 {
                {
                     let digit: i32 = s.char_at(i) - '0';
                    if digit < 0 || digit > 9 {
                        throw FormatException::get_format_instance();
                    }
                    sum += digit;
                }
                i -= 2;
             }
         }

        sum *= 3;
         {
             let mut i: i32 = length - 2;
            while i >= 0 {
                {
                     let digit: i32 = s.char_at(i) - '0';
                    if digit < 0 || digit > 9 {
                        throw FormatException::get_format_instance();
                    }
                    sum += digit;
                }
                i -= 2;
             }
         }

        return Ok((1000 - sum) % 10);
    }

    fn  decode_end(&self,  row: &BitArray,  end_start: i32) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
        return Ok(::find_guard_pattern(row, end_start, false, &START_END_PATTERN));
    }

    fn  find_guard_pattern( row: &BitArray,  row_offset: i32,  white_first: bool,  pattern: &Vec<i32>) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
        return Ok(::find_guard_pattern(row, row_offset, white_first, &pattern, : [i32; pattern.len()] = [0; pattern.len()]));
    }

    /**
   * @param row row of black/white values to search
   * @param rowOffset position to start search
   * @param whiteFirst if true, indicates that the pattern specifies white/black/white/...
   * pixel counts, otherwise, it is interpreted as black/white/black/...
   * @param pattern pattern of counts of number of black and white pixels that are being
   * searched for as a pattern
   * @param counters array of counters, as long as pattern, to re-use
   * @return start/end horizontal offset of guard pattern, as an array of two ints
   * @throws NotFoundException if pattern is not found
   */
    fn  find_guard_pattern( row: &BitArray,  row_offset: i32,  white_first: bool,  pattern: &Vec<i32>,  counters: &Vec<i32>) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let width: i32 = row.get_size();
        row_offset =  if white_first { row.get_next_unset(row_offset) } else { row.get_next_set(row_offset) };
         let counter_position: i32 = 0;
         let pattern_start: i32 = row_offset;
         let pattern_length: i32 = pattern.len();
         let is_white: bool = white_first;
         {
             let mut x: i32 = row_offset;
            while x < width {
                {
                    if row.get(x) != is_white {
                        counters[counter_position] += 1;
                    } else {
                        if counter_position == pattern_length - 1 {
                            if pattern_match_variance(&counters, &pattern, MAX_INDIVIDUAL_VARIANCE) < MAX_AVG_VARIANCE {
                                return Ok( : vec![i32; 2] = vec![pattern_start, x, ]
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
                x += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    /**
   * Attempts to decode a single UPC/EAN-encoded digit.
   *
   * @param row row of black/white values to decode
   * @param counters the counts of runs of observed black/white/black/... values
   * @param rowOffset horizontal offset to start decoding from
   * @param patterns the set of patterns to use to decode -- sometimes different encodings
   * for the digits 0-9 are used, and this indicates the encodings for 0 to 9 that should
   * be used
   * @return horizontal offset of first pixel beyond the decoded digit
   * @throws NotFoundException if digit cannot be decoded
   */
    fn  decode_digit( row: &BitArray,  counters: &Vec<i32>,  row_offset: i32,  patterns: &Vec<Vec<i32>>) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
        record_pattern(row, row_offset, &counters);
        // worst variance we'll accept
         let best_variance: f32 = MAX_AVG_VARIANCE;
         let best_match: i32 = -1;
         let max: i32 = patterns.len();
         {
             let mut i: i32 = 0;
            while i < max {
                {
                     let pattern: Vec<i32> = patterns[i];
                     let variance: f32 = pattern_match_variance(&counters, &pattern, MAX_INDIVIDUAL_VARIANCE);
                    if variance < best_variance {
                        best_variance = variance;
                        best_match = i;
                    }
                }
                i += 1;
             }
         }

        if best_match >= 0 {
            return Ok(best_match);
        } else {
            throw NotFoundException::get_not_found_instance();
        }
    }

    /**
   * Get the format of this decoder.
   *
   * @return The 1D format.
   */
    fn  get_barcode_format(&self) -> BarcodeFormat ;

    /**
   * Subclasses override this to decode the portion of a barcode between the start
   * and end guard patterns.
   *
   * @param row row of black/white values to search
   * @param startRange start/end offset of start guard pattern
   * @param resultString {@link StringBuilder} to append decoded chars to
   * @return horizontal offset of first pixel after the "middle" that was decoded
   * @throws NotFoundException if decoding could not complete successfully
   */
    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>  ;
}

// NEW FILE: u_p_c_e_a_n_writer.rs
/*
 * Copyright 2009 ZXing authors
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
 * <p>Encapsulates functionality and implementation that is common to UPC and EAN families
 * of one-dimensional barcodes.</p>
 *
 * @author aripollak@gmail.com (Ari Pollak)
 * @author dsbnatut@gmail.com (Kazuki Nishiura)
 */
pub struct UPCEANWriter {
    super: OneDimensionalCodeWriter;
}

impl UPCEANWriter {

    pub fn  get_default_margin(&self) -> i32  {
        // Use a different default more appropriate for UPC/EAN
        return 9;
    }
}

// NEW FILE: u_p_c_e_reader.rs
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
 * <p>Implements decoding of the UPC-E format.</p>
 * <p><a href="http://www.barcodeisland.com/upce.phtml">This</a> is a great reference for
 * UPC-E information.</p>
 *
 * @author Sean Owen
 */

/**
   * The pattern that marks the middle, and end, of a UPC-E pattern.
   * There is no "second half" to a UPC-E barcode.
   */
 const MIDDLE_END_PATTERN: vec![Vec<i32>; 6] = vec![1, 1, 1, 1, 1, 1, ]
;

// For an UPC-E barcode, the final digit is represented by the parities used
// to encode the middle six digits, according to the table below.
//
//                Parity of next 6 digits
//    Digit   0     1     2     3     4     5
//       0    Even   Even  Even Odd  Odd   Odd
//       1    Even   Even  Odd  Even Odd   Odd
//       2    Even   Even  Odd  Odd  Even  Odd
//       3    Even   Even  Odd  Odd  Odd   Even
//       4    Even   Odd   Even Even Odd   Odd
//       5    Even   Odd   Odd  Even Even  Odd
//       6    Even   Odd   Odd  Odd  Even  Even
//       7    Even   Odd   Even Odd  Even  Odd
//       8    Even   Odd   Even Odd  Odd   Even
//       9    Even   Odd   Odd  Even Odd   Even
//
// The encoding is represented by the following array, which is a bit pattern
// using Odd = 0 and Even = 1. For example, 5 is represented by:
//
//              Odd Even Even Odd Odd Even
// in binary:
//                0    1    1   0   0    1   == 0x19
//
/**
   * See {@link #L_AND_G_PATTERNS}; these values similarly represent patterns of
   * even-odd parity encodings of digits that imply both the number system (0 or 1)
   * used, and the check digit.
   */
 const NUMSYS_AND_CHECK_DIGIT_PATTERNS: vec![vec![Vec<Vec<i32>>; 10]; 2] = vec![vec![0x38, 0x34, 0x32, 0x31, 0x2C, 0x26, 0x23, 0x2A, 0x29, 0x25, ]
, vec![0x07, 0x0B, 0x0D, 0x0E, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A, ]
, ]
;
pub struct UPCEReader {
    super: UPCEANReader;

     let decode_middle_counters: Vec<i32>;
}

impl UPCEReader {

    pub fn new() -> UPCEReader {
        decode_middle_counters = : [i32; 4] = [0; 4];
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.decode_middle_counters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let end: i32 = row.get_size();
         let row_offset: i32 = start_range[1];
         let lg_pattern_found: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < 6 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_AND_G_PATTERNS);
                    result.append(('0' + best_match % 10) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                    if best_match >= 10 {
                        lg_pattern_found |= 1 << (5 - x);
                    }
                }
                x += 1;
             }
         }

        ::determine_num_sys_and_check_digit(&result, lg_pattern_found);
        return Ok(row_offset);
    }

    pub fn  decode_end(&self,  row: &BitArray,  end_start: i32) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
        return Ok(find_guard_pattern(row, end_start, true, &MIDDLE_END_PATTERN));
    }

    pub fn  check_checksum(&self,  s: &String) -> /*  throws FormatException */Result<bool, Rc<Exception>>   {
        return Ok(super.check_checksum(&::convert_u_p_c_eto_u_p_c_a(&s)));
    }

    fn  determine_num_sys_and_check_digit( result_string: &StringBuilder,  lg_pattern_found: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         {
             let num_sys: i32 = 0;
            while num_sys <= 1 {
                {
                     {
                         let mut d: i32 = 0;
                        while d < 10 {
                            {
                                if lg_pattern_found == NUMSYS_AND_CHECK_DIGIT_PATTERNS[num_sys][d] {
                                    result_string.insert(0, ('0' + num_sys) as char);
                                    result_string.append(('0' + d) as char);
                                    return;
                                }
                            }
                            d += 1;
                         }
                     }

                }
                num_sys += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::UPC_E;
    }

    /**
   * Expands a UPC-E value back into its full, equivalent UPC-A code value.
   *
   * @param upce UPC-E code as string of digits
   * @return equivalent UPC-A code as string of digits
   */
    pub fn  convert_u_p_c_eto_u_p_c_a( upce: &String) -> String  {
         let upce_chars: [Option<char>; 6] = [None; 6];
        upce.get_chars(1, 7, &upce_chars, 0);
         let result: StringBuilder = StringBuilder::new(12);
        result.append(&upce.char_at(0));
         let last_char: char = upce_chars[5];
        match last_char {
              '0' => 
                 {
                }
              '1' => 
                 {
                }
              '2' => 
                 {
                    result.append(&upce_chars, 0, 2);
                    result.append(last_char);
                    result.append("0000");
                    result.append(&upce_chars, 2, 3);
                    break;
                }
              '3' => 
                 {
                    result.append(&upce_chars, 0, 3);
                    result.append("00000");
                    result.append(&upce_chars, 3, 2);
                    break;
                }
              '4' => 
                 {
                    result.append(&upce_chars, 0, 4);
                    result.append("00000");
                    result.append(upce_chars[4]);
                    break;
                }
            _ => 
                 {
                    result.append(&upce_chars, 0, 5);
                    result.append("0000");
                    result.append(last_char);
                    break;
                }
        }
        // Only append check digit in conversion if supplied
        if upce.length() >= 8 {
            result.append(&upce.char_at(7));
        }
        return result.to_string();
    }
}

// NEW FILE: u_p_c_e_writer.rs
/*
 * Copyright 2009 ZXing authors
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
 * This object renders an UPC-E code as a {@link BitMatrix}.
 *
 * @author 0979097955s@gmail.com (RX)
 */

 const CODE_WIDTH: i32 = // start guard
3 + // bars
(7 * 6) + // end guard
6;
pub struct UPCEWriter {
    super: UPCEANWriter;
}

impl UPCEWriter {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::UPC_E);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let length: i32 = contents.length();
        match length {
              7 => 
                 {
                    // No check digit present, calculate it and add it
                     let mut check: i32;
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        check = UPCEANReader::get_standard_u_p_c_e_a_n_checksum(&UPCEReader::convert_u_p_c_eto_u_p_c_a(&contents));
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( fe: &FormatException) {
                            throw IllegalArgumentException::new(fe);
                        }  0 => break
                    }

                    contents += check;
                    break;
                }
              8 => 
                 {
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        if !UPCEANReader::check_standard_u_p_c_e_a_n_checksum(&UPCEReader::convert_u_p_c_eto_u_p_c_a(&contents)) {
                            throw IllegalArgumentException::new("Contents do not pass checksum");
                        }
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( ignored: &FormatException) {
                            throw IllegalArgumentException::new("Illegal contents");
                        }  0 => break
                    }

                    break;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("Requested contents should be 7 or 8 digits long, but got {}", length));
                }
        }
        check_numeric(&contents);
         let first_digit: i32 = Character::digit(&contents.char_at(0), 10);
        if first_digit != 0 && first_digit != 1 {
            throw IllegalArgumentException::new("Number system must be 0 or 1");
        }
         let check_digit: i32 = Character::digit(&contents.char_at(7), 10);
         let parities: i32 = UPCEReader.NUMSYS_AND_CHECK_DIGIT_PATTERNS[first_digit][check_digit];
         let result: [bool; CODE_WIDTH] = [false; CODE_WIDTH];
         let mut pos: i32 = append_pattern(&result, 0, UPCEANReader.START_END_PATTERN, true);
         {
             let mut i: i32 = 1;
            while i <= 6 {
                {
                     let mut digit: i32 = Character::digit(&contents.char_at(i), 10);
                    if (parities >> (6 - i) & 1) == 1 {
                        digit += 10;
                    }
                    pos += append_pattern(&result, pos, UPCEANReader.L_AND_G_PATTERNS[digit], false);
                }
                i += 1;
             }
         }

        append_pattern(&result, pos, UPCEANReader.END_PATTERN, false);
        return result;
    }
}

