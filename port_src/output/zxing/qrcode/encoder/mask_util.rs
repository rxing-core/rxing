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
// package com::google::zxing::qrcode::encoder;

/**
 * @author Satoru Takabayashi
 * @author Daniel Switkin
 * @author Sean Owen
 */

// Penalty weights from section 6.8.2.1
 const N1: i32 = 3;

 const N2: i32 = 3;

 const N3: i32 = 40;

 const N4: i32 = 10;
struct MaskUtil {
}

impl MaskUtil {

    fn new() -> MaskUtil {
    // do nothing
    }

    /**
   * Apply mask penalty rule 1 and return the penalty. Find repetitive cells with the same color and
   * give penalty to them. Example: 00000 or 11111.
   */
    fn  apply_mask_penalty_rule1( matrix: &ByteMatrix) -> i32  {
        return ::apply_mask_penalty_rule1_internal(matrix, true) + ::apply_mask_penalty_rule1_internal(matrix, false);
    }

    /**
   * Apply mask penalty rule 2 and return the penalty. Find 2x2 blocks with the same color and give
   * penalty to them. This is actually equivalent to the spec's rule, which is to find MxN blocks and give a
   * penalty proportional to (M-1)x(N-1), because this is the number of 2x2 blocks inside such a block.
   */
    fn  apply_mask_penalty_rule2( matrix: &ByteMatrix) -> i32  {
         let mut penalty: i32 = 0;
         let array: Vec<Vec<i8>> = matrix.get_array();
         let width: i32 = matrix.get_width();
         let height: i32 = matrix.get_height();
         {
             let mut y: i32 = 0;
            while y < height - 1 {
                {
                     let array_y: Vec<i8> = array[y];
                     {
                         let mut x: i32 = 0;
                        while x < width - 1 {
                            {
                                 let value: i32 = array_y[x];
                                if value == array_y[x + 1] && value == array[y + 1][x] && value == array[y + 1][x + 1] {
                                    penalty += 1;
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return N2 * penalty;
    }

    /**
   * Apply mask penalty rule 3 and return the penalty. Find consecutive runs of 1:1:3:1:1:4
   * starting with black, or 4:1:1:3:1:1 starting with white, and give penalty to them.  If we
   * find patterns like 000010111010000, we give penalty once.
   */
    fn  apply_mask_penalty_rule3( matrix: &ByteMatrix) -> i32  {
         let num_penalties: i32 = 0;
         let array: Vec<Vec<i8>> = matrix.get_array();
         let width: i32 = matrix.get_width();
         let height: i32 = matrix.get_height();
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                // We can at least optimize this access
                                 let array_y: Vec<i8> = array[y];
                                if x + 6 < width && array_y[x] == 1 && array_y[x + 1] == 0 && array_y[x + 2] == 1 && array_y[x + 3] == 1 && array_y[x + 4] == 1 && array_y[x + 5] == 0 && array_y[x + 6] == 1 && (::is_white_horizontal(&array_y, x - 4, x) || ::is_white_horizontal(&array_y, x + 7, x + 11)) {
                                    num_penalties += 1;
                                }
                                if y + 6 < height && array[y][x] == 1 && array[y + 1][x] == 0 && array[y + 2][x] == 1 && array[y + 3][x] == 1 && array[y + 4][x] == 1 && array[y + 5][x] == 0 && array[y + 6][x] == 1 && (::is_white_vertical(&array, x, y - 4, y) || ::is_white_vertical(&array, x, y + 7, y + 11)) {
                                    num_penalties += 1;
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return num_penalties * N3;
    }

    fn  is_white_horizontal( row_array: &Vec<i8>,  from: i32,  to: i32) -> bool  {
        if from < 0 || row_array.len() < to {
            return false;
        }
         {
             let mut i: i32 = from;
            while i < to {
                {
                    if row_array[i] == 1 {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    fn  is_white_vertical( array: &Vec<Vec<i8>>,  col: i32,  from: i32,  to: i32) -> bool  {
        if from < 0 || array.len() < to {
            return false;
        }
         {
             let mut i: i32 = from;
            while i < to {
                {
                    if array[i][col] == 1 {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    /**
   * Apply mask penalty rule 4 and return the penalty. Calculate the ratio of dark cells and give
   * penalty if the ratio is far from 50%. It gives 10 penalty for 5% distance.
   */
    fn  apply_mask_penalty_rule4( matrix: &ByteMatrix) -> i32  {
         let num_dark_cells: i32 = 0;
         let array: Vec<Vec<i8>> = matrix.get_array();
         let width: i32 = matrix.get_width();
         let height: i32 = matrix.get_height();
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let array_y: Vec<i8> = array[y];
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                if array_y[x] == 1 {
                                    num_dark_cells += 1;
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

         let num_total_cells: i32 = matrix.get_height() * matrix.get_width();
         let five_percent_variances: i32 = Math::abs(num_dark_cells * 2 - num_total_cells) * 10 / num_total_cells;
        return five_percent_variances * N4;
    }

    /**
   * Return the mask bit for "getMaskPattern" at "x" and "y". See 8.8 of JISX0510:2004 for mask
   * pattern conditions.
   */
    fn  get_data_mask_bit( mask_pattern: i32,  x: i32,  y: i32) -> bool  {
         let mut intermediate: i32;
         let mut temp: i32;
        match mask_pattern {
              0 => 
                 {
                    intermediate = (y + x) & 0x1;
                    break;
                }
              1 => 
                 {
                    intermediate = y & 0x1;
                    break;
                }
              2 => 
                 {
                    intermediate = x % 3;
                    break;
                }
              3 => 
                 {
                    intermediate = (y + x) % 3;
                    break;
                }
              4 => 
                 {
                    intermediate = ((y / 2) + (x / 3)) & 0x1;
                    break;
                }
              5 => 
                 {
                    temp = y * x;
                    intermediate = (temp & 0x1) + (temp % 3);
                    break;
                }
              6 => 
                 {
                    temp = y * x;
                    intermediate = ((temp & 0x1) + (temp % 3)) & 0x1;
                    break;
                }
              7 => 
                 {
                    temp = y * x;
                    intermediate = ((temp % 3) + ((y + x) & 0x1)) & 0x1;
                    break;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("Invalid mask pattern: {}", mask_pattern));
                }
        }
        return intermediate == 0;
    }

    /**
   * Helper function for applyMaskPenaltyRule1. We need this for doing this calculation in both
   * vertical and horizontal orders respectively.
   */
    fn  apply_mask_penalty_rule1_internal( matrix: &ByteMatrix,  is_horizontal: bool) -> i32  {
         let mut penalty: i32 = 0;
         let i_limit: i32 =  if is_horizontal { matrix.get_height() } else { matrix.get_width() };
         let j_limit: i32 =  if is_horizontal { matrix.get_width() } else { matrix.get_height() };
         let array: Vec<Vec<i8>> = matrix.get_array();
         {
             let mut i: i32 = 0;
            while i < i_limit {
                {
                     let num_same_bit_cells: i32 = 0;
                     let prev_bit: i32 = -1;
                     {
                         let mut j: i32 = 0;
                        while j < j_limit {
                            {
                                 let bit: i32 =  if is_horizontal { array[i][j] } else { array[j][i] };
                                if bit == prev_bit {
                                    num_same_bit_cells += 1;
                                } else {
                                    if num_same_bit_cells >= 5 {
                                        penalty += N1 + (num_same_bit_cells - 5);
                                    }
                                    // Include the cell itself.
                                    num_same_bit_cells = 1;
                                    prev_bit = bit;
                                }
                            }
                            j += 1;
                         }
                     }

                    if num_same_bit_cells >= 5 {
                        penalty += N1 + (num_same_bit_cells - 5);
                    }
                }
                i += 1;
             }
         }

        return penalty;
    }
}

