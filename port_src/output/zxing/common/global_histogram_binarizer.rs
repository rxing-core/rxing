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
 * This Binarizer implementation uses the old ZXing global histogram approach. It is suitable
 * for low-end mobile devices which don't have enough CPU or memory to use a local thresholding
 * algorithm. However, because it picks a global black point, it cannot handle difficult shadows
 * and gradients.
 *
 * Faster mobile devices and all desktop applications should probably use HybridBinarizer instead.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */

 const LUMINANCE_BITS: i32 = 5;

 const LUMINANCE_SHIFT: i32 = 8 - LUMINANCE_BITS;

 const LUMINANCE_BUCKETS: i32 = 1 << LUMINANCE_BITS;

 const EMPTY: [i8; 0] = [0; 0];
pub struct GlobalHistogramBinarizer {
    super: Binarizer;

     let mut luminances: Vec<i8>;

     let mut buckets: Vec<i32>;
}

impl GlobalHistogramBinarizer {

    pub fn new( source: &LuminanceSource) -> GlobalHistogramBinarizer {
        super(source);
        luminances = EMPTY;
        buckets = : [i32; LUMINANCE_BUCKETS] = [0; LUMINANCE_BUCKETS];
    }

    // Applies simple sharpening to the row data to improve performance of the 1D Readers.
    pub fn  get_black_row(&self,  y: i32,  row: &BitArray) -> /*  throws NotFoundException */Result<BitArray, Rc<Exception>>   {
         let source: LuminanceSource = get_luminance_source();
         let width: i32 = source.get_width();
        if row == null || row.get_size() < width {
            row = BitArray::new(width);
        } else {
            row.clear();
        }
        self.init_arrays(width);
         let local_luminances: Vec<i8> = source.get_row(y, &self.luminances);
         let local_buckets: Vec<i32> = self.buckets;
         {
             let mut x: i32 = 0;
            while x < width {
                {
                    local_buckets[(local_luminances[x] & 0xff) >> LUMINANCE_SHIFT] += 1;
                }
                x += 1;
             }
         }

         let black_point: i32 = ::estimate_black_point(&local_buckets);
        if width < 3 {
            // Special case for very small images
             {
                 let mut x: i32 = 0;
                while x < width {
                    {
                        if (local_luminances[x] & 0xff) < black_point {
                            row.set(x);
                        }
                    }
                    x += 1;
                 }
             }

        } else {
             let mut left: i32 = local_luminances[0] & 0xff;
             let mut center: i32 = local_luminances[1] & 0xff;
             {
                 let mut x: i32 = 1;
                while x < width - 1 {
                    {
                         let right: i32 = local_luminances[x + 1] & 0xff;
                        // A simple -1 4 -1 box filter with a weight of 2.
                        if ((center * 4) - left - right) / 2 < black_point {
                            row.set(x);
                        }
                        left = center;
                        center = right;
                    }
                    x += 1;
                 }
             }

        }
        return Ok(row);
    }

    // Does not sharpen the data, as this call is intended to only be used by 2D Readers.
    pub fn  get_black_matrix(&self) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
         let source: LuminanceSource = get_luminance_source();
         let width: i32 = source.get_width();
         let height: i32 = source.get_height();
         let matrix: BitMatrix = BitMatrix::new(width, height);
        // Quickly calculates the histogram by sampling four rows from the image. This proved to be
        // more robust on the blackbox tests than sampling a diagonal as we used to do.
        self.init_arrays(width);
         let local_buckets: Vec<i32> = self.buckets;
         {
             let mut y: i32 = 1;
            while y < 5 {
                {
                     let row: i32 = height * y / 5;
                     let local_luminances: Vec<i8> = source.get_row(row, &self.luminances);
                     let right: i32 = (width * 4) / 5;
                     {
                         let mut x: i32 = width / 5;
                        while x < right {
                            {
                                 let mut pixel: i32 = local_luminances[x] & 0xff;
                                local_buckets[pixel >> LUMINANCE_SHIFT] += 1;
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

         let black_point: i32 = ::estimate_black_point(&local_buckets);
        // We delay reading the entire image luminance until the black point estimation succeeds.
        // Although we end up reading four rows twice, it is consistent with our motto of
        // "fail quickly" which is necessary for continuous scanning.
         let local_luminances: Vec<i8> = source.get_matrix();
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let offset: i32 = y * width;
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                 let pixel: i32 = local_luminances[offset + x] & 0xff;
                                if pixel < black_point {
                                    matrix.set(x, y);
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return Ok(matrix);
    }

    pub fn  create_binarizer(&self,  source: &LuminanceSource) -> Binarizer  {
        return GlobalHistogramBinarizer::new(source);
    }

    fn  init_arrays(&self,  luminance_size: i32)   {
        if self.luminances.len() < luminance_size {
            self.luminances = : [i8; luminance_size] = [0; luminance_size];
        }
         {
             let mut x: i32 = 0;
            while x < LUMINANCE_BUCKETS {
                {
                    self.buckets[x] = 0;
                }
                x += 1;
             }
         }

    }

    fn  estimate_black_point( buckets: &Vec<i32>) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
        // Find the tallest peak in the histogram.
         let num_buckets: i32 = buckets.len();
         let max_bucket_count: i32 = 0;
         let first_peak: i32 = 0;
         let first_peak_size: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < num_buckets {
                {
                    if buckets[x] > first_peak_size {
                        first_peak = x;
                        first_peak_size = buckets[x];
                    }
                    if buckets[x] > max_bucket_count {
                        max_bucket_count = buckets[x];
                    }
                }
                x += 1;
             }
         }

        // Find the second-tallest peak which is somewhat far from the tallest peak.
         let second_peak: i32 = 0;
         let second_peak_score: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < num_buckets {
                {
                     let distance_to_biggest: i32 = x - first_peak;
                    // Encourage more distant second peaks by multiplying by square of distance.
                     let score: i32 = buckets[x] * distance_to_biggest * distance_to_biggest;
                    if score > second_peak_score {
                        second_peak = x;
                        second_peak_score = score;
                    }
                }
                x += 1;
             }
         }

        // Make sure firstPeak corresponds to the black peak.
        if first_peak > second_peak {
             let temp: i32 = first_peak;
            first_peak = second_peak;
            second_peak = temp;
        }
        // than waste time trying to decode the image, and risk false positives.
        if second_peak - first_peak <= num_buckets / 16 {
            throw NotFoundException::get_not_found_instance();
        }
        // Find a valley between them that is low and closer to the white peak.
         let best_valley: i32 = second_peak - 1;
         let best_valley_score: i32 = -1;
         {
             let mut x: i32 = second_peak - 1;
            while x > first_peak {
                {
                     let from_first: i32 = x - first_peak;
                     let score: i32 = from_first * from_first * (second_peak - x) * (max_bucket_count - buckets[x]);
                    if score > best_valley_score {
                        best_valley = x;
                        best_valley_score = score;
                    }
                }
                x -= 1;
             }
         }

        return Ok(best_valley << LUMINANCE_SHIFT);
    }
}

