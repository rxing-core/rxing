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

