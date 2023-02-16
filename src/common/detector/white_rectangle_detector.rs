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

//package com.google.zxing.common.detector;

use crate::{
    common::{BitMatrix, Result},
    point, Exceptions, Point,
};

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
pub struct WhiteRectangleDetector<'a> {
    image: &'a BitMatrix,
    height: i32,
    width: i32,
    leftInit: i32,
    rightInit: i32,
    downInit: i32,
    upInit: i32,
}

impl<'a> WhiteRectangleDetector<'_> {
    pub fn new_from_image(image: &'a BitMatrix) -> Result<WhiteRectangleDetector<'a>> {
        WhiteRectangleDetector::new(
            image,
            INIT_SIZE,
            image.getWidth() as i32 / 2,
            image.getHeight() as i32 / 2,
        )
    }

    /**
     * @param image barcode image to find a rectangle in
     * @param initSize initial size of search area around center
     * @param x x position of search center
     * @param y y position of search center
     * @throws NotFoundException if image is too small to accommodate {@code initSize}
     */
    pub fn new(
        image: &'a BitMatrix,
        initSize: i32,
        x: i32,
        y: i32,
    ) -> Result<WhiteRectangleDetector<'a>> {
        let halfsize = initSize / 2;

        let leftInit = x - halfsize;
        let rightInit = x + halfsize;
        let upInit = y - halfsize;
        let downInit = y + halfsize;

        if upInit < 0
            || leftInit < 0
            || downInit >= image.getHeight() as i32
            || rightInit >= image.getWidth() as i32
        {
            return Err(Exceptions::NotFoundException(None));
        }

        Ok(WhiteRectangleDetector {
            image,
            height: image.getHeight() as i32,
            width: image.getWidth() as i32,
            leftInit,
            rightInit,
            downInit,
            upInit,
        })
    }

    /**
     * <p>
     * Detects a candidate barcode-like rectangular region within an image. It
     * starts around the center of the image, increases the size of the candidate
     * region until it finds a white rectangular region.
     * </p>
     *
     * @return {@link Point}[] describing the corners of the rectangular
     *         region. The first and last points are opposed on the diagonal, as
     *         are the second and third. The first point will be the topmost
     *         point and the last, the bottommost. The second point will be
     *         leftmost and the third, the rightmost
     * @throws NotFoundException if no Data Matrix Code can be found
     */
    pub fn detect(&self) -> Result<[Point; 4]> {
        let mut left: i32 = self.leftInit;
        let mut right: i32 = self.rightInit;
        let mut up: i32 = self.upInit;
        let mut down: i32 = self.downInit;
        let mut size_exceeded = false;
        let mut a_black_point_found_on_border = true;

        let mut at_least_one_black_point_found_on_right = false;
        let mut at_least_one_black_point_found_on_bottom = false;
        let mut at_least_one_black_point_found_on_left = false;
        let mut at_least_one_black_point_found_on_top = false;

        while a_black_point_found_on_border {
            a_black_point_found_on_border = false;

            // .....
            // .   |
            // .....
            let mut right_border_not_white = true;
            while (right_border_not_white || !at_least_one_black_point_found_on_right)
                && right < self.width
            {
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
            let mut bottom_border_not_white = true;
            while (bottom_border_not_white || !at_least_one_black_point_found_on_bottom)
                && down < self.height
            {
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
            let mut left_border_not_white = true;
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
            let mut top_border_not_white = true;
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
            let max_size = right - left;

            let mut z: Option<Point> = None;
            let mut i = 1;
            while z.is_none() && i < max_size {
                //for (int i = 1; z == null && i < maxSize; i++) {
                z = self.get_black_point_on_segment(
                    left as f32,
                    (down - i) as f32,
                    (left + i) as f32,
                    down as f32,
                );
                i += 1;
            }

            if z.is_none() {
                return Err(Exceptions::NotFoundException(None));
            }

            let mut t: Option<Point> = None;
            //go down right
            let mut i = 1;
            while t.is_none() && i < max_size {
                //for (int i = 1; t == null && i < maxSize; i++) {
                t = self.get_black_point_on_segment(
                    left as f32,
                    (up + i) as f32,
                    (left + i) as f32,
                    up as f32,
                );
                i += 1;
            }

            if t.is_none() {
                return Err(Exceptions::NotFoundException(None));
            }

            let mut x: Option<Point> = None;
            //go down left
            let mut i = 1;
            while x.is_none() && i < max_size {
                //for (int i = 1; x == null && i < maxSize; i++) {
                x = self.get_black_point_on_segment(
                    right as f32,
                    (up + i) as f32,
                    (right - i) as f32,
                    up as f32,
                );
                i += 1;
            }

            if x.is_none() {
                return Err(Exceptions::NotFoundException(None));
            }

            let mut y: Option<Point> = None;
            //go up left
            let mut i = 1;
            while y.is_none() && i < max_size {
                //for (int i = 1; y == null && i < maxSize; i++) {
                y = self.get_black_point_on_segment(
                    right as f32,
                    (down - i) as f32,
                    (right - i) as f32,
                    down as f32,
                );
                i += 1;
            }

            if y.is_none() {
                return Err(Exceptions::NotFoundException(None));
            }

            Ok(self.center_edges(y.unwrap(), z.unwrap(), x.unwrap(), t.unwrap()))
        } else {
            Err(Exceptions::NotFoundException(None))
        }
    }

    fn get_black_point_on_segment(&self, a_x: f32, a_y: f32, b_x: f32, b_y: f32) -> Option<Point> {
        let a = point(a_x, a_y);
        let b = point(b_x, b_y);

        let dist = a.distance(b).round() as i32;
        let x_step: f32 = (b_x - a_x) / dist as f32;
        let y_step: f32 = (b_y - a_y) / dist as f32;

        for i in 0..dist {
            let x = (a_x + i as f32 * x_step).round() as i32;
            let y = (a_y + i as f32 * y_step).round() as i32;
            if self.image.get(x as u32, y as u32) {
                return Some(point(x as f32, y as f32));
            }
        }
        None
    }

    /**
     * recenters the points of a constant distance towards the center
     *
     * @param y bottom most point
     * @param z left most point
     * @param x right most point
     * @param t top most point
     * @return {@link Point}[] describing the corners of the rectangular
     *         region. The first and last points are opposed on the diagonal, as
     *         are the second and third. The first point will be the topmost
     *         point and the last, the bottommost. The second point will be
     *         leftmost and the third, the rightmost
     */
    fn center_edges(&self, y: Point, z: Point, x: Point, t: Point) -> [Point; 4] {
        //
        //       t            t
        //  z                      x
        //        x    OR    z
        //   y                    y
        //

        let yi = y.x;
        let yj = y.y;
        let zi = z.x;
        let zj = z.y;
        let xi = x.x;
        let xj = x.y;
        let ti = t.x;
        let tj = t.y;

        if yi < self.width as f32 / 2.0f32 {
            [
                point(ti - CORR as f32, tj + CORR as f32),
                point(zi + CORR as f32, zj + CORR as f32),
                point(xi - CORR as f32, xj - CORR as f32),
                point(yi + CORR as f32, yj - CORR as f32),
            ]
        } else {
            [
                point(ti + CORR as f32, tj + CORR as f32),
                point(zi + CORR as f32, zj - CORR as f32),
                point(xi - CORR as f32, xj + CORR as f32),
                point(yi - CORR as f32, yj - CORR as f32),
            ]
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
    fn contains_black_point(&self, a: i32, b: i32, fixed: i32, horizontal: bool) -> bool {
        if horizontal {
            for x in a..=b {
                if self.image.get(x as u32, fixed as u32) {
                    return true;
                }
            }
        } else {
            for y in a..=b {
                if self.image.get(fixed as u32, y as u32) {
                    return true;
                }
            }
        }

        false
    }
}
