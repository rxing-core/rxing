use crate::{NotFoundException,ResultPoint,BarcodeFormat,DecodeHintType,NotFoundException,RXingResult,ResultMetadataType,ResultPoint,ResultPointCallback};
use crate::common::BitArray;
use crate::common::detector::{MathUtils};
use crate::oned::{OneDReader};



// NEW FILE: abstract_r_s_s_reader.rs
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
// package com::google::zxing::oned::rss;

/**
 * Superclass of {@link OneDReader} implementations that read barcodes in the RSS family
 * of formats.
 */

 const MAX_AVG_VARIANCE: f32 = 0.2f;

 const MAX_INDIVIDUAL_VARIANCE: f32 = 0.45f;

 const MIN_FINDER_PATTERN_RATIO: f32 = 9.5f / 12.0f;

 const MAX_FINDER_PATTERN_RATIO: f32 = 12.5f / 14.0f;
pub struct AbstractRSSReader {
    super: OneDReader;

     let decode_finder_counters: Vec<i32>;

     let data_character_counters: Vec<i32>;

     let odd_rounding_errors: Vec<f32>;

     let even_rounding_errors: Vec<f32>;

     let odd_counts: Vec<i32>;

     let even_counts: Vec<i32>;
}

impl AbstractRSSReader {

    pub fn new() -> AbstractRSSReader {
        decode_finder_counters = : [i32; 4] = [0; 4];
        data_character_counters = : [i32; 8] = [0; 8];
        odd_rounding_errors = : [f32; 4.0] = [0.0; 4.0];
        even_rounding_errors = : [f32; 4.0] = [0.0; 4.0];
        odd_counts = : [i32; data_character_counters.len() / 2] = [0; data_character_counters.len() / 2];
        even_counts = : [i32; data_character_counters.len() / 2] = [0; data_character_counters.len() / 2];
    }

    pub fn  get_decode_finder_counters(&self) -> Vec<i32>  {
        return self.decode_finder_counters;
    }

    pub fn  get_data_character_counters(&self) -> Vec<i32>  {
        return self.data_character_counters;
    }

    pub fn  get_odd_rounding_errors(&self) -> Vec<f32>  {
        return self.odd_rounding_errors;
    }

    pub fn  get_even_rounding_errors(&self) -> Vec<f32>  {
        return self.even_rounding_errors;
    }

    pub fn  get_odd_counts(&self) -> Vec<i32>  {
        return self.odd_counts;
    }

    pub fn  get_even_counts(&self) -> Vec<i32>  {
        return self.even_counts;
    }

    pub fn  parse_finder_value( counters: &Vec<i32>,  finder_patterns: &Vec<Vec<i32>>) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         {
             let mut value: i32 = 0;
            while value < finder_patterns.len() {
                {
                    if pattern_match_variance(&counters, finder_patterns[value], MAX_INDIVIDUAL_VARIANCE) < MAX_AVG_VARIANCE {
                        return Ok(value);
                    }
                }
                value += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    /**
   * @param array values to sum
   * @return sum of values
   * @deprecated call {@link MathUtils#sum(int[])}
   */
    pub fn  count( array: &Vec<i32>) -> i32  {
        return MathUtils::sum(&array);
    }

    pub fn  increment( array: &Vec<i32>,  errors: &Vec<f32>)   {
         let mut index: i32 = 0;
         let biggest_error: f32 = errors[0];
         {
             let mut i: i32 = 1;
            while i < array.len() {
                {
                    if errors[i] > biggest_error {
                        biggest_error = errors[i];
                        index = i;
                    }
                }
                i += 1;
             }
         }

        array[index] += 1;
    }

    pub fn  decrement( array: &Vec<i32>,  errors: &Vec<f32>)   {
         let mut index: i32 = 0;
         let biggest_error: f32 = errors[0];
         {
             let mut i: i32 = 1;
            while i < array.len() {
                {
                    if errors[i] < biggest_error {
                        biggest_error = errors[i];
                        index = i;
                    }
                }
                i += 1;
             }
         }

        array[index] -= 1;
    }

    pub fn  is_finder_pattern( counters: &Vec<i32>) -> bool  {
         let first_two_sum: i32 = counters[0] + counters[1];
         let sum: i32 = first_two_sum + counters[2] + counters[3];
         let ratio: f32 = first_two_sum / sum as f32;
        if ratio >= MIN_FINDER_PATTERN_RATIO && ratio <= MAX_FINDER_PATTERN_RATIO {
            // passes ratio test in spec, but see if the counts are unreasonable
             let min_counter: i32 = Integer::MAX_VALUE;
             let max_counter: i32 = Integer::MIN_VALUE;
            for  let counter: i32 in counters {
                if counter > max_counter {
                    max_counter = counter;
                }
                if counter < min_counter {
                    min_counter = counter;
                }
            }
            return max_counter < 10 * min_counter;
        }
        return false;
    }
}

// NEW FILE: data_character.rs
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
// package com::google::zxing::oned::rss;

/**
 * Encapsulates a since character value in an RSS barcode, including its checksum information.
 */
pub struct DataCharacter {

     let value: i32;

     let checksum_portion: i32;
}

impl DataCharacter {

    pub fn new( value: i32,  checksum_portion: i32) -> DataCharacter {
        let .value = value;
        let .checksumPortion = checksum_portion;
    }

    pub fn  get_value(&self) -> i32  {
        return self.value;
    }

    pub fn  get_checksum_portion(&self) -> i32  {
        return self.checksum_portion;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{}({})", self.value, self.checksum_portion);
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof DataCharacter) {
            return false;
        }
         let that: DataCharacter = o as DataCharacter;
        return self.value == that.value && self.checksum_portion == that.checksumPortion;
    }

    pub fn  hash_code(&self) -> i32  {
        return self.value ^ self.checksum_portion;
    }
}

// NEW FILE: finder_pattern.rs
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
// package com::google::zxing::oned::rss;

/**
 * Encapsulates an RSS barcode finder pattern, including its start/end position and row.
 */
pub struct FinderPattern {

     let value: i32;

     let start_end: Vec<i32>;

     let result_points: Vec<ResultPoint>;
}

impl FinderPattern {

    pub fn new( value: i32,  start_end: &Vec<i32>,  start: i32,  end: i32,  row_number: i32) -> FinderPattern {
        let .value = value;
        let .startEnd = start_end;
        let .resultPoints =  : vec![ResultPoint; 2] = vec![ResultPoint::new(start, row_number), ResultPoint::new(end, row_number), ]
        ;
    }

    pub fn  get_value(&self) -> i32  {
        return self.value;
    }

    pub fn  get_start_end(&self) -> Vec<i32>  {
        return self.start_end;
    }

    pub fn  get_result_points(&self) -> Vec<ResultPoint>  {
        return self.result_points;
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof FinderPattern) {
            return false;
        }
         let that: FinderPattern = o as FinderPattern;
        return self.value == that.value;
    }

    pub fn  hash_code(&self) -> i32  {
        return self.value;
    }
}

// NEW FILE: pair.rs
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
// package com::google::zxing::oned::rss;

struct Pair {
    super: DataCharacter;

     let finder_pattern: FinderPattern;

     let mut count: i32;
}

impl Pair {

    fn new( value: i32,  checksum_portion: i32,  finder_pattern: &FinderPattern) -> Pair {
        super(value, checksum_portion);
        let .finderPattern = finder_pattern;
    }

    fn  get_finder_pattern(&self) -> FinderPattern  {
        return self.finder_pattern;
    }

    fn  get_count(&self) -> i32  {
        return self.count;
    }

    fn  increment_count(&self)   {
        self.count += 1;
    }
}

// NEW FILE: r_s_s14_reader.rs
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
// package com::google::zxing::oned::rss;

/**
 * Decodes RSS-14, including truncated and stacked variants. See ISO/IEC 24724:2006.
 */

 const OUTSIDE_EVEN_TOTAL_SUBSET: vec![Vec<i32>; 5] = vec![1, 10, 34, 70, 126, ]
;

 const INSIDE_ODD_TOTAL_SUBSET: vec![Vec<i32>; 4] = vec![4, 20, 48, 81, ]
;

 const OUTSIDE_GSUM: vec![Vec<i32>; 5] = vec![0, 161, 961, 2015, 2715, ]
;

 const INSIDE_GSUM: vec![Vec<i32>; 4] = vec![0, 336, 1036, 1516, ]
;

 const OUTSIDE_ODD_WIDEST: vec![Vec<i32>; 5] = vec![8, 6, 4, 3, 1, ]
;

 const INSIDE_ODD_WIDEST: vec![Vec<i32>; 4] = vec![2, 4, 6, 8, ]
;

 const FINDER_PATTERNS: vec![vec![Vec<Vec<i32>>; 4]; 9] = vec![vec![3, 8, 2, 1, ]
, vec![3, 5, 5, 1, ]
, vec![3, 3, 7, 1, ]
, vec![3, 1, 9, 1, ]
, vec![2, 7, 4, 1, ]
, vec![2, 5, 6, 1, ]
, vec![2, 3, 8, 1, ]
, vec![1, 5, 7, 1, ]
, vec![1, 3, 9, 1, ]
, ]
;
pub struct RSS14Reader {
    super: AbstractRSSReader;

     let possible_left_pairs: List<Pair>;

     let possible_right_pairs: List<Pair>;
}

impl RSS14Reader {

    pub fn new() -> RSS14Reader {
        possible_left_pairs = ArrayList<>::new();
        possible_right_pairs = ArrayList<>::new();
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
         let left_pair: Pair = self.decode_pair(row, false, row_number, &hints);
        ::add_or_tally(&self.possible_left_pairs, left_pair);
        row.reverse();
         let right_pair: Pair = self.decode_pair(row, true, row_number, &hints);
        ::add_or_tally(&self.possible_right_pairs, right_pair);
        row.reverse();
        for  let left: Pair in self.possible_left_pairs {
            if left.get_count() > 1 {
                for  let right: Pair in self.possible_right_pairs {
                    if right.get_count() > 1 && ::check_checksum(left, right) {
                        return Ok(::construct_result(left, right));
                    }
                }
            }
        }
        throw NotFoundException::get_not_found_instance();
    }

    fn  add_or_tally( possible_pairs: &Collection<Pair>,  pair: &Pair)   {
        if pair == null {
            return;
        }
         let mut found: bool = false;
        for  let other: Pair in possible_pairs {
            if other.get_value() == pair.get_value() {
                other.increment_count();
                found = true;
                break;
            }
        }
        if !found {
            possible_pairs.add(pair);
        }
    }

    pub fn  reset(&self)   {
        self.possible_left_pairs.clear();
        self.possible_right_pairs.clear();
    }

    fn  construct_result( left_pair: &Pair,  right_pair: &Pair) -> Result  {
         let symbol_value: i64 = 4537077 * left_pair.get_value() + right_pair.get_value();
         let text: String = String::value_of(symbol_value);
         let buffer: StringBuilder = StringBuilder::new(14);
         {
             let mut i: i32 = 13 - text.length();
            while i > 0 {
                {
                    buffer.append('0');
                }
                i -= 1;
             }
         }

        buffer.append(&text);
         let check_digit: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 13 {
                {
                     let digit: i32 = buffer.char_at(i) - '0';
                    check_digit +=  if (i & 0x01) == 0 { 3 * digit } else { digit };
                }
                i += 1;
             }
         }

        check_digit = 10 - (check_digit % 10);
        if check_digit == 10 {
            check_digit = 0;
        }
        buffer.append(check_digit);
         let left_points: Vec<ResultPoint> = left_pair.get_finder_pattern().get_result_points();
         let right_points: Vec<ResultPoint> = right_pair.get_finder_pattern().get_result_points();
         let result: Result = Result::new(&buffer.to_string(), null,  : vec![ResultPoint; 4] = vec![left_points[0], left_points[1], right_points[0], right_points[1], ]
        , BarcodeFormat::RSS_14);
        result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, "]e0");
        return result;
    }

    fn  check_checksum( left_pair: &Pair,  right_pair: &Pair) -> bool  {
         let check_value: i32 = (left_pair.get_checksum_portion() + 16 * right_pair.get_checksum_portion()) % 79;
         let target_check_value: i32 = 9 * left_pair.get_finder_pattern().get_value() + right_pair.get_finder_pattern().get_value();
        if target_check_value > 72 {
            target_check_value -= 1;
        }
        if target_check_value > 8 {
            target_check_value -= 1;
        }
        return check_value == target_check_value;
    }

    fn  decode_pair(&self,  row: &BitArray,  right: bool,  row_number: i32,  hints: &Map<DecodeHintType, ?>) -> Pair  {
        let tryResult1 = 0;
        'try1: loop {
        {
             let start_end: Vec<i32> = self.find_finder_pattern(row, right);
             let pattern: FinderPattern = self.parse_found_finder_pattern(row, row_number, right, &start_end);
             let result_point_callback: ResultPointCallback =  if hints == null { null } else { hints.get(DecodeHintType::NEED_RESULT_POINT_CALLBACK) as ResultPointCallback };
            if result_point_callback != null {
                start_end = pattern.get_start_end();
                 let mut center: f32 = (start_end[0] + start_end[1] - 1.0) / 2.0f;
                if right {
                    // row is actually reversed
                    center = row.get_size() - 1.0 - center;
                }
                result_point_callback.found_possible_result_point(ResultPoint::new(center, row_number));
            }
             let outside: DataCharacter = self.decode_data_character(row, pattern, true);
             let inside: DataCharacter = self.decode_data_character(row, pattern, false);
            return Pair::new(1597 * outside.get_value() + inside.get_value(), outside.get_checksum_portion() + 4 * inside.get_checksum_portion(), pattern);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &NotFoundException) {
                return null;
            }  0 => break
        }

    }

    fn  decode_data_character(&self,  row: &BitArray,  pattern: &FinderPattern,  outside_char: bool) -> /*  throws NotFoundException */Result<DataCharacter, Rc<Exception>>   {
         let mut counters: Vec<i32> = get_data_character_counters();
        Arrays::fill(&counters, 0);
        if outside_char {
            record_pattern_in_reverse(row, pattern.get_start_end()[0], &counters);
        } else {
            record_pattern(row, pattern.get_start_end()[1], &counters);
            // reverse it
             {
                 let mut i: i32 = 0, let mut j: i32 = counters.len() - 1;
                while i < j {
                    {
                         let temp: i32 = counters[i];
                        counters[i] = counters[j];
                        counters[j] = temp;
                    }
                    i += 1;
                    j -= 1;
                 }
             }

        }
         let num_modules: i32 =  if outside_char { 16 } else { 15 };
         let element_width: f32 = MathUtils::sum(&counters) / num_modules as f32;
         let odd_counts: Vec<i32> = self.get_odd_counts();
         let even_counts: Vec<i32> = self.get_even_counts();
         let odd_rounding_errors: Vec<f32> = self.get_odd_rounding_errors();
         let even_rounding_errors: Vec<f32> = self.get_even_rounding_errors();
         {
             let mut i: i32 = 0;
            while i < counters.len() {
                {
                     let value: f32 = counters[i] / element_width;
                    // Round
                     let mut count: i32 = (value + 0.5f) as i32;
                    if count < 1 {
                        count = 1;
                    } else if count > 8 {
                        count = 8;
                    }
                     let mut offset: i32 = i / 2;
                    if (i & 0x01) == 0 {
                        odd_counts[offset] = count;
                        odd_rounding_errors[offset] = value - count;
                    } else {
                        even_counts[offset] = count;
                        even_rounding_errors[offset] = value - count;
                    }
                }
                i += 1;
             }
         }

        self.adjust_odd_even_counts(outside_char, num_modules);
         let odd_sum: i32 = 0;
         let odd_checksum_portion: i32 = 0;
         {
             let mut i: i32 = odd_counts.len() - 1;
            while i >= 0 {
                {
                    odd_checksum_portion *= 9;
                    odd_checksum_portion += odd_counts[i];
                    odd_sum += odd_counts[i];
                }
                i -= 1;
             }
         }

         let even_checksum_portion: i32 = 0;
         let even_sum: i32 = 0;
         {
             let mut i: i32 = even_counts.len() - 1;
            while i >= 0 {
                {
                    even_checksum_portion *= 9;
                    even_checksum_portion += even_counts[i];
                    even_sum += even_counts[i];
                }
                i -= 1;
             }
         }

         let checksum_portion: i32 = odd_checksum_portion + 3 * even_checksum_portion;
        if outside_char {
            if (odd_sum & 0x01) != 0 || odd_sum > 12 || odd_sum < 4 {
                throw NotFoundException::get_not_found_instance();
            }
             let group: i32 = (12 - odd_sum) / 2;
             let odd_widest: i32 = OUTSIDE_ODD_WIDEST[group];
             let even_widest: i32 = 9 - odd_widest;
             let v_odd: i32 = RSSUtils::get_r_s_svalue(&odd_counts, odd_widest, false);
             let v_even: i32 = RSSUtils::get_r_s_svalue(&even_counts, even_widest, true);
             let t_even: i32 = OUTSIDE_EVEN_TOTAL_SUBSET[group];
             let g_sum: i32 = OUTSIDE_GSUM[group];
            return Ok(DataCharacter::new(v_odd * t_even + v_even + g_sum, checksum_portion));
        } else {
            if (even_sum & 0x01) != 0 || even_sum > 10 || even_sum < 4 {
                throw NotFoundException::get_not_found_instance();
            }
             let group: i32 = (10 - even_sum) / 2;
             let odd_widest: i32 = INSIDE_ODD_WIDEST[group];
             let even_widest: i32 = 9 - odd_widest;
             let v_odd: i32 = RSSUtils::get_r_s_svalue(&odd_counts, odd_widest, true);
             let v_even: i32 = RSSUtils::get_r_s_svalue(&even_counts, even_widest, false);
             let t_odd: i32 = INSIDE_ODD_TOTAL_SUBSET[group];
             let g_sum: i32 = INSIDE_GSUM[group];
            return Ok(DataCharacter::new(v_even * t_odd + v_odd + g_sum, checksum_portion));
        }
    }

    fn  find_finder_pattern(&self,  row: &BitArray,  right_finder_pattern: bool) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
         let mut counters: Vec<i32> = get_decode_finder_counters();
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let width: i32 = row.get_size();
         let is_white: bool = false;
         let row_offset: i32 = 0;
        while row_offset < width {
            is_white = !row.get(row_offset);
            if right_finder_pattern == is_white {
                // Will encounter white first when searching for right finder pattern
                break;
            }
            row_offset += 1;
        }
         let counter_position: i32 = 0;
         let pattern_start: i32 = row_offset;
         {
             let mut x: i32 = row_offset;
            while x < width {
                {
                    if row.get(x) != is_white {
                        counters[counter_position] += 1;
                    } else {
                        if counter_position == 3 {
                            if is_finder_pattern(&counters) {
                                return Ok( : vec![i32; 2] = vec![pattern_start, x, ]
                                );
                            }
                            pattern_start += counters[0] + counters[1];
                            counters[0] = counters[2];
                            counters[1] = counters[3];
                            counters[2] = 0;
                            counters[3] = 0;
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

    fn  parse_found_finder_pattern(&self,  row: &BitArray,  row_number: i32,  right: bool,  start_end: &Vec<i32>) -> /*  throws NotFoundException */Result<FinderPattern, Rc<Exception>>   {
        // Actually we found elements 2-5
         let first_is_black: bool = row.get(start_end[0]);
         let first_element_start: i32 = start_end[0] - 1;
        // Locate element 1
        while first_element_start >= 0 && first_is_black != row.get(first_element_start) {
            first_element_start -= 1;
        }
        first_element_start += 1;
         let first_counter: i32 = start_end[0] - first_element_start;
        // Make 'counters' hold 1-4
         let mut counters: Vec<i32> = get_decode_finder_counters();
        System::arraycopy(&counters, 0, &counters, 1, counters.len() - 1);
        counters[0] = first_counter;
         let value: i32 = parse_finder_value(&counters, &FINDER_PATTERNS);
         let mut start: i32 = first_element_start;
         let mut end: i32 = start_end[1];
        if right {
            // row is actually reversed
            start = row.get_size() - 1 - start;
            end = row.get_size() - 1 - end;
        }
        return Ok(FinderPattern::new(value,  : vec![i32; 2] = vec![first_element_start, start_end[1], ]
        , start, end, row_number));
    }

    fn  adjust_odd_even_counts(&self,  outside_char: bool,  num_modules: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         let odd_sum: i32 = MathUtils::sum(&get_odd_counts());
         let even_sum: i32 = MathUtils::sum(&get_even_counts());
         let increment_odd: bool = false;
         let decrement_odd: bool = false;
         let increment_even: bool = false;
         let decrement_even: bool = false;
        if outside_char {
            if odd_sum > 12 {
                decrement_odd = true;
            } else if odd_sum < 4 {
                increment_odd = true;
            }
            if even_sum > 12 {
                decrement_even = true;
            } else if even_sum < 4 {
                increment_even = true;
            }
        } else {
            if odd_sum > 11 {
                decrement_odd = true;
            } else if odd_sum < 5 {
                increment_odd = true;
            }
            if even_sum > 10 {
                decrement_even = true;
            } else if even_sum < 4 {
                increment_even = true;
            }
        }
         let mismatch: i32 = odd_sum + even_sum - num_modules;
         let odd_parity_bad: bool = (odd_sum & 0x01) == ( if outside_char { 1 } else { 0 });
         let even_parity_bad: bool = (even_sum & 0x01) == 1;
        /*if (mismatch == 2) {
      if (!(oddParityBad && evenParityBad)) {
        throw ReaderException.getInstance();
      }
      decrementOdd = true;
      decrementEven = true;
    } else if (mismatch == -2) {
      if (!(oddParityBad && evenParityBad)) {
        throw ReaderException.getInstance();
      }
      incrementOdd = true;
      incrementEven = true;
    } else */
        match mismatch {
              1 => 
                 {
                    if odd_parity_bad {
                        if even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        decrement_odd = true;
                    } else {
                        if !even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        decrement_even = true;
                    }
                    break;
                }
              -1 => 
                 {
                    if odd_parity_bad {
                        if even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        increment_odd = true;
                    } else {
                        if !even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        increment_even = true;
                    }
                    break;
                }
              0 => 
                 {
                    if odd_parity_bad {
                        if !even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        // Both bad
                        if odd_sum < even_sum {
                            increment_odd = true;
                            decrement_even = true;
                        } else {
                            decrement_odd = true;
                            increment_even = true;
                        }
                    } else {
                        if even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                    // Nothing to do!
                    }
                    break;
                }
            _ => 
                 {
                    throw NotFoundException::get_not_found_instance();
                }
        }
        if increment_odd {
            if decrement_odd {
                throw NotFoundException::get_not_found_instance();
            }
            increment(&get_odd_counts(), &get_odd_rounding_errors());
        }
        if decrement_odd {
            decrement(&get_odd_counts(), &get_odd_rounding_errors());
        }
        if increment_even {
            if decrement_even {
                throw NotFoundException::get_not_found_instance();
            }
            increment(&get_even_counts(), &get_odd_rounding_errors());
        }
        if decrement_even {
            decrement(&get_even_counts(), &get_even_rounding_errors());
        }
    }
}

// NEW FILE: r_s_s_utils.rs
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
// package com::google::zxing::oned::rss;

/** Adapted from listings in ISO/IEC 24724 Appendix B and Appendix G. */
pub struct RSSUtils {
}

impl RSSUtils {

    fn new() -> RSSUtils {
    }

    pub fn  get_r_s_svalue( widths: &Vec<i32>,  max_width: i32,  no_narrow: bool) -> i32  {
         let mut n: i32 = 0;
        for  let width: i32 in widths {
            n += width;
        }
         let mut val: i32 = 0;
         let narrow_mask: i32 = 0;
         let elements: i32 = widths.len();
         {
             let mut bar: i32 = 0;
            while bar < elements - 1 {
                {
                     let elm_width: i32;
                     {
                        elm_width = 1;
                        narrow_mask |= 1 << bar;
                        while elm_width < widths[bar] {
                            {
                                 let sub_val: i32 = ::combins(n - elm_width - 1, elements - bar - 2);
                                if no_narrow && (narrow_mask == 0) && (n - elm_width - (elements - bar - 1) >= elements - bar - 1) {
                                    sub_val -= ::combins(n - elm_width - (elements - bar), elements - bar - 2);
                                }
                                if elements - bar - 1 > 1 {
                                     let less_val: i32 = 0;
                                     {
                                         let mxw_element: i32 = n - elm_width - (elements - bar - 2);
                                        while mxw_element > max_width {
                                            {
                                                less_val += ::combins(n - elm_width - mxw_element - 1, elements - bar - 3);
                                            }
                                            mxw_element -= 1;
                                         }
                                     }

                                    sub_val -= less_val * (elements - 1 - bar);
                                } else if n - elm_width > max_width {
                                    sub_val -= 1;
                                }
                                val += sub_val;
                            }
                            elm_width += 1;
                            narrow_mask &= ~(1 << bar);
                         }
                     }

                    n -= elm_width;
                }
                bar += 1;
             }
         }

        return val;
    }

    fn  combins( n: i32,  r: i32) -> i32  {
         let max_denom: i32;
         let min_denom: i32;
        if n - r > r {
            min_denom = r;
            max_denom = n - r;
        } else {
            min_denom = n - r;
            max_denom = r;
        }
         let mut val: i32 = 1;
         let mut j: i32 = 1;
         {
             let mut i: i32 = n;
            while i > max_denom {
                {
                    val *= i;
                    if j <= min_denom {
                        val /= j;
                        j += 1;
                    }
                }
                i -= 1;
             }
         }

        while j <= min_denom {
            val /= j;
            j += 1;
        }
        return val;
    }
}

