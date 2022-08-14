use crate::{BinaryBitmap,NotFoundException,DecodeHintType,ResultPoint};
use crate::common::BitMatrix;

// NEW FILE: detector.rs
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
// package com::google::zxing::pdf417::detector;

/**
 * <p>Encapsulates logic that can detect a PDF417 Code in an image, even if the
 * PDF417 Code is rotated or skewed, or partially obscured.</p>
 *
 * @author SITA Lab (kevin.osullivan@sita.aero)
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Guenther Grau
 */

 const INDEXES_START_PATTERN: vec![Vec<i32>; 4] = vec![0, 4, 1, 5, ]
;

 const INDEXES_STOP_PATTERN: vec![Vec<i32>; 4] = vec![6, 2, 7, 3, ]
;

 const MAX_AVG_VARIANCE: f32 = 0.42f;

 const MAX_INDIVIDUAL_VARIANCE: f32 = 0.8f;

// B S B S B S B S Bar/Space pattern
// 11111111 0 1 0 1 0 1 000
 const START_PATTERN: vec![Vec<i32>; 8] = vec![8, 1, 1, 1, 1, 1, 1, 3, ]
;

// 1111111 0 1 000 1 0 1 00 1
 const STOP_PATTERN: vec![Vec<i32>; 9] = vec![7, 1, 1, 3, 1, 1, 1, 2, 1, ]
;

 const MAX_PIXEL_DRIFT: i32 = 3;

 const MAX_PATTERN_DRIFT: i32 = 5;

// if we set the value too low, then we don't detect the correct height of the bar if the start patterns are damaged.
// if we set the value too high, then we might detect the start pattern from a neighbor barcode.
 const SKIPPED_ROW_COUNT_MAX: i32 = 25;

// A PDF471 barcode should have at least 3 rows, with each row being >= 3 times the module width.
// Therefore it should be at least 9 pixels tall. To be conservative, we use about half the size to
// ensure we don't miss it.
 const ROW_STEP: i32 = 5;

 const BARCODE_MIN_HEIGHT: i32 = 10;

 const ROTATIONS: vec![Vec<i32>; 4] = vec![0, 180, 270, 90, ]
;
pub struct Detector {
}

impl Detector {

    fn new() -> Detector {
    }

    /**
   * <p>Detects a PDF417 Code in an image. Checks 0, 90, 180, and 270 degree rotations.</p>
   *
   * @param image barcode image to decode
   * @param hints optional hints to detector
   * @param multiple if true, then the image is searched for multiple codes. If false, then at most one code will
   * be found and returned
   * @return {@link PDF417DetectorResult} encapsulating results of detecting a PDF417 code
   * @throws NotFoundException if no PDF417 Code can be found
   */
    pub fn  detect( image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>,  multiple: bool) -> /*  throws NotFoundException */Result<PDF417DetectorResult, Rc<Exception>>   {
        // TODO detection improvement, tryHarder could try several different luminance thresholds/blackpoints or even
        // different binarizers
        //boolean tryHarder = hints != null && hints.containsKey(DecodeHintType.TRY_HARDER);
         let original_matrix: BitMatrix = image.get_black_matrix();
        for  let rotation: i32 in ROTATIONS {
             let bit_matrix: BitMatrix = ::apply_rotation(original_matrix, rotation);
             let barcode_coordinates: List<Vec<ResultPoint>> = ::detect(multiple, bit_matrix);
            if !barcode_coordinates.is_empty() {
                return Ok(PDF417DetectorResult::new(bit_matrix, &barcode_coordinates, rotation));
            }
        }
        return Ok(PDF417DetectorResult::new(original_matrix, ArrayList<>::new(), 0));
    }

    /**
   * Applies a rotation to the supplied BitMatrix.
   * @param matrix bit matrix to apply rotation to
   * @param rotation the degrees of rotation to apply
   * @return BitMatrix with applied rotation
   */
    fn  apply_rotation( matrix: &BitMatrix,  rotation: i32) -> BitMatrix  {
        if rotation % 360 == 0 {
            return matrix;
        }
         let new_matrix: BitMatrix = matrix.clone();
        new_matrix.rotate(rotation);
        return new_matrix;
    }

    /**
   * Detects PDF417 codes in an image. Only checks 0 degree rotation
   * @param multiple if true, then the image is searched for multiple codes. If false, then at most one code will
   * be found and returned
   * @param bitMatrix bit matrix to detect barcodes in
   * @return List of ResultPoint arrays containing the coordinates of found barcodes
   */
    fn  detect( multiple: bool,  bit_matrix: &BitMatrix) -> List<Vec<ResultPoint>>  {
         let barcode_coordinates: List<Vec<ResultPoint>> = ArrayList<>::new();
         let mut row: i32 = 0;
         let mut column: i32 = 0;
         let found_barcode_in_row: bool = false;
        while row < bit_matrix.get_height() {
             let vertices: Vec<ResultPoint> = ::find_vertices(bit_matrix, row, column);
            if vertices[0] == null && vertices[3] == null {
                if !found_barcode_in_row {
                    // we didn't find any barcode so that's the end of searching
                    break;
                }
                // we didn't find a barcode starting at the given column and row. Try again from the first column and slightly
                // below the lowest barcode we found so far.
                found_barcode_in_row = false;
                column = 0;
                for  let barcode_coordinate: Vec<ResultPoint> in barcode_coordinates {
                    if barcode_coordinate[1] != null {
                        row = Math::max(row, &barcode_coordinate[1].get_y()) as i32;
                    }
                    if barcode_coordinate[3] != null {
                        row = Math::max(row, barcode_coordinate[3].get_y() as i32);
                    }
                }
                row += ROW_STEP;
                continue;
            }
            found_barcode_in_row = true;
            barcode_coordinates.add(vertices);
            if !multiple {
                break;
            }
            // start pattern of the barcode just found.
            if vertices[2] != null {
                column = vertices[2].get_x() as i32;
                row = vertices[2].get_y() as i32;
            } else {
                column = vertices[4].get_x() as i32;
                row = vertices[4].get_y() as i32;
            }
        }
        return Ok(barcode_coordinates);
    }

    /**
   * Locate the vertices and the codewords area of a black blob using the Start
   * and Stop patterns as locators.
   *
   * @param matrix the scanned barcode image.
   * @return an array containing the vertices:
   *           vertices[0] x, y top left barcode
   *           vertices[1] x, y bottom left barcode
   *           vertices[2] x, y top right barcode
   *           vertices[3] x, y bottom right barcode
   *           vertices[4] x, y top left codeword area
   *           vertices[5] x, y bottom left codeword area
   *           vertices[6] x, y top right codeword area
   *           vertices[7] x, y bottom right codeword area
   */
    fn  find_vertices( matrix: &BitMatrix,  start_row: i32,  start_column: i32) -> Vec<ResultPoint>  {
         let height: i32 = matrix.get_height();
         let width: i32 = matrix.get_width();
         let result: [Option<ResultPoint>; 8] = [None; 8];
        ::copy_to_result(result, &::find_rows_with_pattern(matrix, height, width, start_row, start_column, &START_PATTERN), &INDEXES_START_PATTERN);
        if result[4] != null {
            start_column = result[4].get_x() as i32;
            start_row = result[4].get_y() as i32;
        }
        ::copy_to_result(result, &::find_rows_with_pattern(matrix, height, width, start_row, start_column, &STOP_PATTERN), &INDEXES_STOP_PATTERN);
        return result;
    }

    fn  copy_to_result( result: &Vec<ResultPoint>,  tmp_result: &Vec<ResultPoint>,  destination_indexes: &Vec<i32>)   {
         {
             let mut i: i32 = 0;
            while i < destination_indexes.len() {
                {
                    result[destination_indexes[i]] = tmp_result[i];
                }
                i += 1;
             }
         }

    }

    fn  find_rows_with_pattern( matrix: &BitMatrix,  height: i32,  width: i32,  start_row: i32,  start_column: i32,  pattern: &Vec<i32>) -> Vec<ResultPoint>  {
         let mut result: [Option<ResultPoint>; 4] = [None; 4];
         let mut found: bool = false;
         let counters: [i32; pattern.len()] = [0; pattern.len()];
        while start_row < height {
            {
                 let mut loc: Vec<i32> = ::find_guard_pattern(matrix, start_column, start_row, width, &pattern, &counters);
                if loc != null {
                    while start_row > 0 {
                         let previous_row_loc: Vec<i32> = ::find_guard_pattern(matrix, start_column, start_row -= 1, width, &pattern, &counters);
                        if previous_row_loc != null {
                            loc = previous_row_loc;
                        } else {
                            start_row += 1;
                            break;
                        }
                    }
                    result[0] = ResultPoint::new(loc[0], start_row);
                    result[1] = ResultPoint::new(loc[1], start_row);
                    found = true;
                    break;
                }
            }
            start_row += ROW_STEP;
         }

         let stop_row: i32 = start_row + 1;
        // Last row of the current symbol that contains pattern
        if found {
             let skipped_row_count: i32 = 0;
             let previous_row_loc: vec![Vec<i32>; 2] = vec![result[0].get_x() as i32, result[1].get_x() as i32, ]
            ;
            while stop_row < height {
                {
                     let loc: Vec<i32> = ::find_guard_pattern(matrix, previous_row_loc[0], stop_row, width, &pattern, &counters);
                    // larger drift and don't check for skipped rows.
                    if loc != null && Math::abs(previous_row_loc[0] - loc[0]) < MAX_PATTERN_DRIFT && Math::abs(previous_row_loc[1] - loc[1]) < MAX_PATTERN_DRIFT {
                        previous_row_loc = loc;
                        skipped_row_count = 0;
                    } else {
                        if skipped_row_count > SKIPPED_ROW_COUNT_MAX {
                            break;
                        } else {
                            skipped_row_count += 1;
                        }
                    }
                }
                stop_row += 1;
             }

            stop_row -= skipped_row_count + 1;
            result[2] = ResultPoint::new(previous_row_loc[0], stop_row);
            result[3] = ResultPoint::new(previous_row_loc[1], stop_row);
        }
        if stop_row - start_row < BARCODE_MIN_HEIGHT {
            Arrays::fill(result, null);
        }
        return result;
    }

    /**
   * @param matrix row of black/white values to search
   * @param column x position to start search
   * @param row y position to start search
   * @param width the number of pixels to search on this row
   * @param pattern pattern of counts of number of black and white pixels that are
   *                 being searched for as a pattern
   * @param counters array of counters, as long as pattern, to re-use
   * @return start/end horizontal offset of guard pattern, as an array of two ints.
   */
    fn  find_guard_pattern( matrix: &BitMatrix,  column: i32,  row: i32,  width: i32,  pattern: &Vec<i32>,  counters: &Vec<i32>) -> Vec<i32>  {
        Arrays::fill(&counters, 0, counters.len(), 0);
         let pattern_start: i32 = column;
         let pixel_drift: i32 = 0;
        // if there are black pixels left of the current pixel shift to the left, but only for MAX_PIXEL_DRIFT pixels
        while matrix.get(pattern_start, row) && pattern_start > 0 && pixel_drift += 1 !!!check!!! post increment < MAX_PIXEL_DRIFT {
            pattern_start -= 1;
        }
         let mut x: i32 = pattern_start;
         let counter_position: i32 = 0;
         let pattern_length: i32 = pattern.len();
         {
             let is_white: bool = false;
            while x < width {
                {
                     let pixel: bool = matrix.get(x, row);
                    if pixel != is_white {
                        counters[counter_position] += 1;
                    } else {
                        if counter_position == pattern_length - 1 {
                            if ::pattern_match_variance(&counters, &pattern) < MAX_AVG_VARIANCE {
                                return  : vec![i32; 2] = vec![pattern_start, x, ]
                                ;
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

        if counter_position == pattern_length - 1 && ::pattern_match_variance(&counters, &pattern) < MAX_AVG_VARIANCE {
            return  : vec![i32; 2] = vec![pattern_start, x - 1, ]
            ;
        }
        return null;
    }

    /**
   * Determines how closely a set of observed counts of runs of black/white
   * values matches a given target pattern. This is reported as the ratio of
   * the total variance from the expected pattern proportions across all
   * pattern elements, to the length of the pattern.
   *
   * @param counters observed counters
   * @param pattern expected pattern
   * @return ratio of total variance between counters and pattern compared to total pattern size
   */
    fn  pattern_match_variance( counters: &Vec<i32>,  pattern: &Vec<i32>) -> f32  {
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
            // is too small to reliably match, so fail:
            return Float::POSITIVE_INFINITY;
        }
        // We're going to fake floating-point math in integers. We just need to use more bits.
        // Scale up patternLength so that intermediate values below like scaledCounter will have
        // more "significant digits".
         let unit_bar_width: f32 = total as f32 / pattern_length;
         let max_individual_variance: f32 = MAX_INDIVIDUAL_VARIANCE * unit_bar_width;
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
}

// NEW FILE: p_d_f417_detector_result.rs
/*
 * Copyright 2007 ZXing authors
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
// package com::google::zxing::pdf417::detector;

/**
 * @author Guenther Grau
 */
pub struct PDF417DetectorResult {

     let bits: BitMatrix;

     let points: List<Vec<ResultPoint>>;

     let rotation: i32;
}

impl PDF417DetectorResult {

    pub fn new( bits: &BitMatrix,  points: &List<Vec<ResultPoint>>,  rotation: i32) -> PDF417DetectorResult {
        let .bits = bits;
        let .points = points;
        let .rotation = rotation;
    }

    pub fn new( bits: &BitMatrix,  points: &List<Vec<ResultPoint>>) -> PDF417DetectorResult {
        this(bits, &points, 0);
    }

    pub fn  get_bits(&self) -> BitMatrix  {
        return self.bits;
    }

    pub fn  get_points(&self) -> List<Vec<ResultPoint>>  {
        return self.points;
    }

    pub fn  get_rotation(&self) -> i32  {
        return self.rotation;
    }
}

