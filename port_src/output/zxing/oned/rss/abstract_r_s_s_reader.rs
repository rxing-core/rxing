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

