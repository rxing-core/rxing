/*
 * Copyright 2010 ZXing authors
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
 * <p>
 * Detects a candidate barcode-like rectangular region within an image. It
 * starts around the center of the image, increases the size of the candidate
 * region until it finds a white rectangular region. By keeping track of the
 * last black points it encountered, it determines the corners of the barcode.
 * </p>
 *
 * @author David Olivier
 */

 const INIT_SIZE: i32 = 10;

 const CORR: i32 = 1;
pub struct WhiteRectangleDetector {

     let image: BitMatrix;

     let mut height: i32;

     let mut width: i32;

     let left_init: i32;

     let right_init: i32;

     let down_init: i32;

     let up_init: i32;
}

impl WhiteRectangleDetector {

    pub fn new( image: &BitMatrix) -> WhiteRectangleDetector throws NotFoundException {
        this(image, INIT_SIZE, image.get_width() / 2, image.get_height() / 2);
    }

    /**
   * @param image barcode image to find a rectangle in
   * @param initSize initial size of search area around center
   * @param x x position of search center
   * @param y y position of search center
   * @throws NotFoundException if image is too small to accommodate {@code initSize}
   */
    pub fn new( image: &BitMatrix,  init_size: i32,  x: i32,  y: i32) -> WhiteRectangleDetector throws NotFoundException {
        let .image = image;
        height = image.get_height();
        width = image.get_width();
         let halfsize: i32 = init_size / 2;
        left_init = x - halfsize;
        right_init = x + halfsize;
        up_init = y - halfsize;
        down_init = y + halfsize;
        if up_init < 0 || left_init < 0 || down_init >= height || right_init >= width {
            throw NotFoundException::get_not_found_instance();
        }
    }

    /**
   * <p>
   * Detects a candidate barcode-like rectangular region within an image. It
   * starts around the center of the image, increases the size of the candidate
   * region until it finds a white rectangular region.
   * </p>
   *
   * @return {@link ResultPoint}[] describing the corners of the rectangular
   *         region. The first and last points are opposed on the diagonal, as
   *         are the second and third. The first point will be the topmost
   *         point and the last, the bottommost. The second point will be
   *         leftmost and the third, the rightmost
   * @throws NotFoundException if no Data Matrix Code can be found
   */
    pub fn  detect(&self) -> /*  throws NotFoundException */Result<Vec<ResultPoint>, Rc<Exception>>   {
         let mut left: i32 = self.left_init;
         let mut right: i32 = self.right_init;
         let mut up: i32 = self.up_init;
         let mut down: i32 = self.down_init;
         let size_exceeded: bool = false;
         let a_black_point_found_on_border: bool = true;
         let at_least_one_black_point_found_on_right: bool = false;
         let at_least_one_black_point_found_on_bottom: bool = false;
         let at_least_one_black_point_found_on_left: bool = false;
         let at_least_one_black_point_found_on_top: bool = false;
        while a_black_point_found_on_border {
            a_black_point_found_on_border = false;
            // .....
            // .   |
            // .....
             let right_border_not_white: bool = true;
            while (right_border_not_white || !at_least_one_black_point_found_on_right) && right < self.width {
                right_border_not_white = self.contains_black_point(up, down, right, false);
                if right_border_not_white {
                    right += 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_right = true;
                } else if !at_least_one_black_point_found_on_right {
                    right += 1;
                }
            }
            if right >= self.width {
                size_exceeded = true;
                break;
            }
            // .....
            // .   .
            // .___.
             let bottom_border_not_white: bool = true;
            while (bottom_border_not_white || !at_least_one_black_point_found_on_bottom) && down < self.height {
                bottom_border_not_white = self.contains_black_point(left, right, down, true);
                if bottom_border_not_white {
                    down += 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_bottom = true;
                } else if !at_least_one_black_point_found_on_bottom {
                    down += 1;
                }
            }
            if down >= self.height {
                size_exceeded = true;
                break;
            }
            // .....
            // |   .
            // .....
             let left_border_not_white: bool = true;
            while (left_border_not_white || !at_least_one_black_point_found_on_left) && left >= 0 {
                left_border_not_white = self.contains_black_point(up, down, left, false);
                if left_border_not_white {
                    left -= 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_left = true;
                } else if !at_least_one_black_point_found_on_left {
                    left -= 1;
                }
            }
            if left < 0 {
                size_exceeded = true;
                break;
            }
            // .___.
            // .   .
            // .....
             let top_border_not_white: bool = true;
            while (top_border_not_white || !at_least_one_black_point_found_on_top) && up >= 0 {
                top_border_not_white = self.contains_black_point(left, right, up, true);
                if top_border_not_white {
                    up -= 1;
                    a_black_point_found_on_border = true;
                    at_least_one_black_point_found_on_top = true;
                } else if !at_least_one_black_point_found_on_top {
                    up -= 1;
                }
            }
            if up < 0 {
                size_exceeded = true;
                break;
            }
        }
        if !size_exceeded {
             let max_size: i32 = right - left;
             let mut z: ResultPoint = null;
             {
                 let mut i: i32 = 1;
                while z == null && i < max_size {
                    {
                        z = self.get_black_point_on_segment(left, down - i, left + i, down);
                    }
                    i += 1;
                 }
             }

            if z == null {
                throw NotFoundException::get_not_found_instance();
            }
             let mut t: ResultPoint = null;
            //go down right
             {
                 let mut i: i32 = 1;
                while t == null && i < max_size {
                    {
                        t = self.get_black_point_on_segment(left, up + i, left + i, up);
                    }
                    i += 1;
                 }
             }

            if t == null {
                throw NotFoundException::get_not_found_instance();
            }
             let mut x: ResultPoint = null;
            //go down left
             {
                 let mut i: i32 = 1;
                while x == null && i < max_size {
                    {
                        x = self.get_black_point_on_segment(right, up + i, right - i, up);
                    }
                    i += 1;
                 }
             }

            if x == null {
                throw NotFoundException::get_not_found_instance();
            }
             let mut y: ResultPoint = null;
            //go up left
             {
                 let mut i: i32 = 1;
                while y == null && i < max_size {
                    {
                        y = self.get_black_point_on_segment(right, down - i, right - i, down);
                    }
                    i += 1;
                 }
             }

            if y == null {
                throw NotFoundException::get_not_found_instance();
            }
            return Ok(self.center_edges(y, z, x, t));
        } else {
            throw NotFoundException::get_not_found_instance();
        }
    }

    fn  get_black_point_on_segment(&self,  a_x: f32,  a_y: f32,  b_x: f32,  b_y: f32) -> ResultPoint  {
         let dist: i32 = MathUtils::round(&MathUtils::distance(a_x, a_y, b_x, b_y));
         let x_step: f32 = (b_x - a_x) / dist;
         let y_step: f32 = (b_y - a_y) / dist;
         {
             let mut i: i32 = 0;
            while i < dist {
                {
                     let x: i32 = MathUtils::round(a_x + i * x_step);
                     let y: i32 = MathUtils::round(a_y + i * y_step);
                    if self.image.get(x, y) {
                        return ResultPoint::new(x, y);
                    }
                }
                i += 1;
             }
         }

        return null;
    }

    /**
   * recenters the points of a constant distance towards the center
   *
   * @param y bottom most point
   * @param z left most point
   * @param x right most point
   * @param t top most point
   * @return {@link ResultPoint}[] describing the corners of the rectangular
   *         region. The first and last points are opposed on the diagonal, as
   *         are the second and third. The first point will be the topmost
   *         point and the last, the bottommost. The second point will be
   *         leftmost and the third, the rightmost
   */
    fn  center_edges(&self,  y: &ResultPoint,  z: &ResultPoint,  x: &ResultPoint,  t: &ResultPoint) -> Vec<ResultPoint>  {
        //
        //       t            t
        //  z                      x
        //        x    OR    z
        //   y                    y
        //
         let yi: f32 = y.get_x();
         let yj: f32 = y.get_y();
         let zi: f32 = z.get_x();
         let zj: f32 = z.get_y();
         let xi: f32 = x.get_x();
         let xj: f32 = x.get_y();
         let ti: f32 = t.get_x();
         let tj: f32 = t.get_y();
        if yi < self.width / 2.0f {
            return  : vec![ResultPoint; 4] = vec![ResultPoint::new(ti - CORR, tj + CORR), ResultPoint::new(zi + CORR, zj + CORR), ResultPoint::new(xi - CORR, xj - CORR), ResultPoint::new(yi + CORR, yj - CORR), ]
            ;
        } else {
            return  : vec![ResultPoint; 4] = vec![ResultPoint::new(ti + CORR, tj + CORR), ResultPoint::new(zi + CORR, zj - CORR), ResultPoint::new(xi - CORR, xj + CORR), ResultPoint::new(yi - CORR, yj - CORR), ]
            ;
        }
    }

    /**
   * Determines whether a segment contains a black point
   *
   * @param a          min value of the scanned coordinate
   * @param b          max value of the scanned coordinate
   * @param fixed      value of fixed coordinate
   * @param horizontal set to true if scan must be horizontal, false if vertical
   * @return true if a black point has been found, else false.
   */
    fn  contains_black_point(&self,  a: i32,  b: i32,  fixed: i32,  horizontal: bool) -> bool  {
        if horizontal {
             {
                 let mut x: i32 = a;
                while x <= b {
                    {
                        if self.image.get(x, fixed) {
                            return true;
                        }
                    }
                    x += 1;
                 }
             }

        } else {
             {
                 let mut y: i32 = a;
                while y <= b {
                    {
                        if self.image.get(fixed, y) {
                            return true;
                        }
                    }
                    y += 1;
                 }
             }

        }
        return false;
    }
}

