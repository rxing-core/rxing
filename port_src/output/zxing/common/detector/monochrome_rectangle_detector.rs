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
// package com::google::zxing::common::detector;

/**
 * <p>A somewhat generic detector that looks for a barcode-like rectangular region within an image.
 * It looks within a mostly white region of an image for a region of black and white, but mostly
 * black. It returns the four corners of the region, as best it can determine.</p>
 *
 * @author Sean Owen
 * @deprecated without replacement since 3.3.0
 */

 const MAX_MODULES: i32 = 32;
pub struct MonochromeRectangleDetector {

     let image: BitMatrix;
}

impl MonochromeRectangleDetector {

    pub fn new( image: &BitMatrix) -> MonochromeRectangleDetector {
        let .image = image;
    }

    /**
   * <p>Detects a rectangular region of black and white -- mostly black -- with a region of mostly
   * white, in an image.</p>
   *
   * @return {@link ResultPoint}[] describing the corners of the rectangular region. The first and
   *  last points are opposed on the diagonal, as are the second and third. The first point will be
   *  the topmost point and the last, the bottommost. The second point will be leftmost and the
   *  third, the rightmost
   * @throws NotFoundException if no Data Matrix Code can be found
   */
    pub fn  detect(&self) -> /*  throws NotFoundException */Result<Vec<ResultPoint>, Rc<Exception>>   {
         let height: i32 = self.image.get_height();
         let width: i32 = self.image.get_width();
         let half_height: i32 = height / 2;
         let half_width: i32 = width / 2;
         let delta_y: i32 = Math::max(1, height / (MAX_MODULES * 8));
         let delta_x: i32 = Math::max(1, width / (MAX_MODULES * 8));
         let mut top: i32 = 0;
         let mut bottom: i32 = height;
         let mut left: i32 = 0;
         let mut right: i32 = width;
         let point_a: ResultPoint = self.find_corner_from_center(half_width, 0, left, right, half_height, -delta_y, top, bottom, half_width / 2);
        top = point_a.get_y() as i32 - 1;
         let point_b: ResultPoint = self.find_corner_from_center(half_width, -delta_x, left, right, half_height, 0, top, bottom, half_height / 2);
        left = point_b.get_x() as i32 - 1;
         let point_c: ResultPoint = self.find_corner_from_center(half_width, delta_x, left, right, half_height, 0, top, bottom, half_height / 2);
        right = point_c.get_x() as i32 + 1;
         let point_d: ResultPoint = self.find_corner_from_center(half_width, 0, left, right, half_height, delta_y, top, bottom, half_width / 2);
        bottom = point_d.get_y() as i32 + 1;
        // Go try to find point A again with better information -- might have been off at first.
        point_a = self.find_corner_from_center(half_width, 0, left, right, half_height, -delta_y, top, bottom, half_width / 4);
        return Ok( : vec![ResultPoint; 4] = vec![point_a, point_b, point_c, point_d, ]
        );
    }

    /**
   * Attempts to locate a corner of the barcode by scanning up, down, left or right from a center
   * point which should be within the barcode.
   *
   * @param centerX center's x component (horizontal)
   * @param deltaX same as deltaY but change in x per step instead
   * @param left minimum value of x
   * @param right maximum value of x
   * @param centerY center's y component (vertical)
   * @param deltaY change in y per step. If scanning up this is negative; down, positive;
   *  left or right, 0
   * @param top minimum value of y to search through (meaningless when di == 0)
   * @param bottom maximum value of y
   * @param maxWhiteRun maximum run of white pixels that can still be considered to be within
   *  the barcode
   * @return a {@link ResultPoint} encapsulating the corner that was found
   * @throws NotFoundException if such a point cannot be found
   */
    fn  find_corner_from_center(&self,  center_x: i32,  delta_x: i32,  left: i32,  right: i32,  center_y: i32,  delta_y: i32,  top: i32,  bottom: i32,  max_white_run: i32) -> /*  throws NotFoundException */Result<ResultPoint, Rc<Exception>>   {
         let last_range: Vec<i32> = null;
         {
             let mut y: i32 = center_y, let mut x: i32 = center_x;
            while y < bottom && y >= top && x < right && x >= left {
                {
                     let mut range: Vec<i32>;
                    if delta_x == 0 {
                        // horizontal slices, up and down
                        range = self.black_white_range(y, max_white_run, left, right, true);
                    } else {
                        // vertical slices, left and right
                        range = self.black_white_range(x, max_white_run, top, bottom, false);
                    }
                    if range == null {
                        if last_range == null {
                            throw NotFoundException::get_not_found_instance();
                        }
                        // lastRange was found
                        if delta_x == 0 {
                             let last_y: i32 = y - delta_y;
                            if last_range[0] < center_x {
                                if last_range[1] > center_x {
                                    // straddle, choose one or the other based on direction
                                    return Ok(ResultPoint::new(last_range[ if delta_y > 0 { 0 } else { 1 }], last_y));
                                }
                                return Ok(ResultPoint::new(last_range[0], last_y));
                            } else {
                                return Ok(ResultPoint::new(last_range[1], last_y));
                            }
                        } else {
                             let last_x: i32 = x - delta_x;
                            if last_range[0] < center_y {
                                if last_range[1] > center_y {
                                    return Ok(ResultPoint::new(last_x, last_range[ if delta_x < 0 { 0 } else { 1 }]));
                                }
                                return Ok(ResultPoint::new(last_x, last_range[0]));
                            } else {
                                return Ok(ResultPoint::new(last_x, last_range[1]));
                            }
                        }
                    }
                    last_range = range;
                }
                y += delta_y;
                x += delta_x;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    /**
   * Computes the start and end of a region of pixels, either horizontally or vertically, that could
   * be part of a Data Matrix barcode.
   *
   * @param fixedDimension if scanning horizontally, this is the row (the fixed vertical location)
   *  where we are scanning. If scanning vertically it's the column, the fixed horizontal location
   * @param maxWhiteRun largest run of white pixels that can still be considered part of the
   *  barcode region
   * @param minDim minimum pixel location, horizontally or vertically, to consider
   * @param maxDim maximum pixel location, horizontally or vertically, to consider
   * @param horizontal if true, we're scanning left-right, instead of up-down
   * @return int[] with start and end of found range, or null if no such range is found
   *  (e.g. only white was found)
   */
    fn  black_white_range(&self,  fixed_dimension: i32,  max_white_run: i32,  min_dim: i32,  max_dim: i32,  horizontal: bool) -> Vec<i32>  {
         let center: i32 = (min_dim + max_dim) / 2;
        // Scan left/up first
         let mut start: i32 = center;
        while start >= min_dim {
            if  if horizontal { self.image.get(start, fixed_dimension) } else { self.image.get(fixed_dimension, start) } {
                start -= 1;
            } else {
                 let white_run_start: i32 = start;
                loop { {
                    start -= 1;
                }if !(start >= min_dim && !( if horizontal { self.image.get(start, fixed_dimension) } else { self.image.get(fixed_dimension, start) })) break;}
                 let white_run_size: i32 = white_run_start - start;
                if start < min_dim || white_run_size > max_white_run {
                    start = white_run_start;
                    break;
                }
            }
        }
        start += 1;
        // Then try right/down
         let mut end: i32 = center;
        while end < max_dim {
            if  if horizontal { self.image.get(end, fixed_dimension) } else { self.image.get(fixed_dimension, end) } {
                end += 1;
            } else {
                 let white_run_start: i32 = end;
                loop { {
                    end += 1;
                }if !(end < max_dim && !( if horizontal { self.image.get(end, fixed_dimension) } else { self.image.get(fixed_dimension, end) })) break;}
                 let white_run_size: i32 = end - white_run_start;
                if end >= max_dim || white_run_size > max_white_run {
                    end = white_run_start;
                    break;
                }
            }
        }
        end -= 1;
        return  if end > start {  : vec![i32; 2] = vec![start, end, ]
         } else { null };
    }
}

