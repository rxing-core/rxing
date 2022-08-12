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

