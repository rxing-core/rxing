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

