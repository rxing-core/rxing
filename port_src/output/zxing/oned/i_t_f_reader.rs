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

