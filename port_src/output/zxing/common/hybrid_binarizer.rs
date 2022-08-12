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
// package com::google::zxing::common;

/**
 * This class implements a local thresholding algorithm, which while slower than the
 * GlobalHistogramBinarizer, is fairly efficient for what it does. It is designed for
 * high frequency images of barcodes with black data on white backgrounds. For this application,
 * it does a much better job than a global blackpoint with severe shadows and gradients.
 * However it tends to produce artifacts on lower frequency images and is therefore not
 * a good general purpose binarizer for uses outside ZXing.
 *
 * This class extends GlobalHistogramBinarizer, using the older histogram approach for 1D readers,
 * and the newer local approach for 2D readers. 1D decoding using a per-row histogram is already
 * inherently local, and only fails for horizontal gradients. We can revisit that problem later,
 * but for now it was not a win to use local blocks for 1D.
 *
 * This Binarizer is the default for the unit tests and the recommended class for library users.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

// This class uses 5x5 blocks to compute local luminance, where each block is 8x8 pixels.
// So this is the smallest dimension in each axis we can accept.
 const BLOCK_SIZE_POWER: i32 = 3;

// ...0100...00
 const BLOCK_SIZE: i32 = 1 << BLOCK_SIZE_POWER;

// ...0011...11
 const BLOCK_SIZE_MASK: i32 = BLOCK_SIZE - 1;

 const MINIMUM_DIMENSION: i32 = BLOCK_SIZE * 5;

 const MIN_DYNAMIC_RANGE: i32 = 24;
pub struct HybridBinarizer {
    super: GlobalHistogramBinarizer;

     let mut matrix: BitMatrix;
}

impl HybridBinarizer {

    pub fn new( source: &LuminanceSource) -> HybridBinarizer {
        super(source);
    }

    /**
   * Calculates the final BitMatrix once for all requests. This could be called once from the
   * constructor instead, but there are some advantages to doing it lazily, such as making
   * profiling easier, and not doing heavy lifting when callers don't expect it.
   */
    pub fn  get_black_matrix(&self) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
        if self.matrix != null {
            return Ok(self.matrix);
        }
         let source: LuminanceSource = get_luminance_source();
         let width: i32 = source.get_width();
         let height: i32 = source.get_height();
        if width >= MINIMUM_DIMENSION && height >= MINIMUM_DIMENSION {
             let luminances: Vec<i8> = source.get_matrix();
             let sub_width: i32 = width >> BLOCK_SIZE_POWER;
            if (width & BLOCK_SIZE_MASK) != 0 {
                sub_width += 1;
            }
             let sub_height: i32 = height >> BLOCK_SIZE_POWER;
            if (height & BLOCK_SIZE_MASK) != 0 {
                sub_height += 1;
            }
             let black_points: Vec<Vec<i32>> = ::calculate_black_points(&luminances, sub_width, sub_height, width, height);
             let new_matrix: BitMatrix = BitMatrix::new(width, height);
            ::calculate_threshold_for_block(&luminances, sub_width, sub_height, width, height, &black_points, new_matrix);
            self.matrix = new_matrix;
        } else {
            // If the image is too small, fall back to the global histogram approach.
            self.matrix = super.get_black_matrix();
        }
        return Ok(self.matrix);
    }

    pub fn  create_binarizer(&self,  source: &LuminanceSource) -> Binarizer  {
        return HybridBinarizer::new(source);
    }

    /**
   * For each block in the image, calculate the average black point using a 5x5 grid
   * of the blocks around it. Also handles the corner cases (fractional blocks are computed based
   * on the last pixels in the row/column which are also used in the previous block).
   */
    fn  calculate_threshold_for_block( luminances: &Vec<i8>,  sub_width: i32,  sub_height: i32,  width: i32,  height: i32,  black_points: &Vec<Vec<i32>>,  matrix: &BitMatrix)   {
         let max_y_offset: i32 = height - BLOCK_SIZE;
         let max_x_offset: i32 = width - BLOCK_SIZE;
         {
             let mut y: i32 = 0;
            while y < sub_height {
                {
                     let mut yoffset: i32 = y << BLOCK_SIZE_POWER;
                    if yoffset > max_y_offset {
                        yoffset = max_y_offset;
                    }
                     let top: i32 = ::cap(y, sub_height - 3);
                     {
                         let mut x: i32 = 0;
                        while x < sub_width {
                            {
                                 let mut xoffset: i32 = x << BLOCK_SIZE_POWER;
                                if xoffset > max_x_offset {
                                    xoffset = max_x_offset;
                                }
                                 let left: i32 = ::cap(x, sub_width - 3);
                                 let mut sum: i32 = 0;
                                 {
                                     let mut z: i32 = -2;
                                    while z <= 2 {
                                        {
                                             let black_row: Vec<i32> = black_points[top + z];
                                            sum += black_row[left - 2] + black_row[left - 1] + black_row[left] + black_row[left + 1] + black_row[left + 2];
                                        }
                                        z += 1;
                                     }
                                 }

                                 let average: i32 = sum / 25;
                                ::threshold_block(&luminances, xoffset, yoffset, average, width, matrix);
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

    }

    fn  cap( value: i32,  max: i32) -> i32  {
        return  if value < 2 { 2 } else { Math::min(value, max) };
    }

    /**
   * Applies a single threshold to a block of pixels.
   */
    fn  threshold_block( luminances: &Vec<i8>,  xoffset: i32,  yoffset: i32,  threshold: i32,  stride: i32,  matrix: &BitMatrix)   {
         {
             let mut y: i32 = 0, let mut offset: i32 = yoffset * stride + xoffset;
            while y < BLOCK_SIZE {
                {
                     {
                         let mut x: i32 = 0;
                        while x < BLOCK_SIZE {
                            {
                                // Comparison needs to be <= so that black == 0 pixels are black even if the threshold is 0.
                                if (luminances[offset + x] & 0xFF) <= threshold {
                                    matrix.set(xoffset + x, yoffset + y);
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
                offset += stride;
             }
         }

    }

    /**
   * Calculates a single black point for each block of pixels and saves it away.
   * See the following thread for a discussion of this algorithm:
   *  http://groups.google.com/group/zxing/browse_thread/thread/d06efa2c35a7ddc0
   */
    fn  calculate_black_points( luminances: &Vec<i8>,  sub_width: i32,  sub_height: i32,  width: i32,  height: i32) -> Vec<Vec<i32>>  {
         let max_y_offset: i32 = height - BLOCK_SIZE;
         let max_x_offset: i32 = width - BLOCK_SIZE;
         let black_points: [[i32; sub_width]; sub_height] = [[0; sub_width]; sub_height];
         {
             let mut y: i32 = 0;
            while y < sub_height {
                {
                     let mut yoffset: i32 = y << BLOCK_SIZE_POWER;
                    if yoffset > max_y_offset {
                        yoffset = max_y_offset;
                    }
                     {
                         let mut x: i32 = 0;
                        while x < sub_width {
                            {
                                 let mut xoffset: i32 = x << BLOCK_SIZE_POWER;
                                if xoffset > max_x_offset {
                                    xoffset = max_x_offset;
                                }
                                 let mut sum: i32 = 0;
                                 let mut min: i32 = 0xFF;
                                 let mut max: i32 = 0;
                                 {
                                     let mut yy: i32 = 0, let mut offset: i32 = yoffset * width + xoffset;
                                    while yy < BLOCK_SIZE {
                                        {
                                             {
                                                 let mut xx: i32 = 0;
                                                while xx < BLOCK_SIZE {
                                                    {
                                                         let pixel: i32 = luminances[offset + xx] & 0xFF;
                                                        sum += pixel;
                                                        // still looking for good contrast
                                                        if pixel < min {
                                                            min = pixel;
                                                        }
                                                        if pixel > max {
                                                            max = pixel;
                                                        }
                                                    }
                                                    xx += 1;
                                                 }
                                             }

                                            // short-circuit min/max tests once dynamic range is met
                                            if max - min > MIN_DYNAMIC_RANGE {
                                                // finish the rest of the rows quickly
                                                 {
                                                    yy += 1;
                                                    offset += width;
                                                    while yy < BLOCK_SIZE {
                                                        {
                                                             {
                                                                 let mut xx: i32 = 0;
                                                                while xx < BLOCK_SIZE {
                                                                    {
                                                                        sum += luminances[offset + xx] & 0xFF;
                                                                    }
                                                                    xx += 1;
                                                                 }
                                                             }

                                                        }
                                                        yy += 1;
                                                        offset += width;
                                                     }
                                                 }

                                            }
                                        }
                                        yy += 1;
                                        offset += width;
                                     }
                                 }

                                // The default estimate is the average of the values in the block.
                                 let mut average: i32 = sum >> (BLOCK_SIZE_POWER * 2);
                                if max - min <= MIN_DYNAMIC_RANGE {
                                    // If variation within the block is low, assume this is a block with only light or only
                                    // dark pixels. In that case we do not want to use the average, as it would divide this
                                    // low contrast area into black and white pixels, essentially creating data out of noise.
                                    //
                                    // The default assumption is that the block is light/background. Since no estimate for
                                    // the level of dark pixels exists locally, use half the min for the block.
                                    average = min / 2;
                                    if y > 0 && x > 0 {
                                        // Correct the "white background" assumption for blocks that have neighbors by comparing
                                        // the pixels in this block to the previously calculated black points. This is based on
                                        // the fact that dark barcode symbology is always surrounded by some amount of light
                                        // background for which reasonable black point estimates were made. The bp estimated at
                                        // the boundaries is used for the interior.
                                        // The (min < bp) is arbitrary but works better than other heuristics that were tried.
                                         let average_neighbor_black_point: i32 = (black_points[y - 1][x] + (2 * black_points[y][x - 1]) + black_points[y - 1][x - 1]) / 4;
                                        if min < average_neighbor_black_point {
                                            average = average_neighbor_black_point;
                                        }
                                    }
                                }
                                black_points[y][x] = average;
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return black_points;
    }
}

