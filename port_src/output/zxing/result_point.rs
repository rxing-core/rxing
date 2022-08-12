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
// package com::google::zxing;

/**
 * <p>Encapsulates a point of interest in an image containing a barcode. Typically, this
 * would be the location of a finder pattern or the corner of the barcode, for example.</p>
 *
 * @author Sean Owen
 */
pub struct ResultPoint {

     let x: f32;

     let y: f32;
}

impl ResultPoint {

    pub fn new( x: f32,  y: f32) -> ResultPoint {
        let .x = x;
        let .y = y;
    }

    pub fn  get_x(&self) -> f32  {
        return self.x;
    }

    pub fn  get_y(&self) -> f32  {
        return self.y;
    }

    pub fn  equals(&self,  other: &Object) -> bool  {
        if other instanceof ResultPoint {
             let other_point: ResultPoint = other as ResultPoint;
            return self.x == other_point.x && self.y == other_point.y;
        }
        return false;
    }

    pub fn  hash_code(&self) -> i32  {
        return 31 * Float::float_to_int_bits(self.x) + Float::float_to_int_bits(self.y);
    }

    pub fn  to_string(&self) -> String  {
        return format!("({},{})", self.x, self.y);
    }

    /**
   * Orders an array of three ResultPoints in an order [A,B,C] such that AB is less than AC
   * and BC is less than AC, and the angle between BC and BA is less than 180 degrees.
   *
   * @param patterns array of three {@code ResultPoint} to order
   */
    pub fn  order_best_patterns( patterns: &Vec<ResultPoint>)   {
        // Find distances between pattern centers
         let zero_one_distance: f32 = ::distance(patterns[0], patterns[1]);
         let one_two_distance: f32 = ::distance(patterns[1], patterns[2]);
         let zero_two_distance: f32 = ::distance(patterns[0], patterns[2]);
         let point_a: ResultPoint;
         let point_b: ResultPoint;
         let point_c: ResultPoint;
        // Assume one closest to other two is B; A and C will just be guesses at first
        if one_two_distance >= zero_one_distance && one_two_distance >= zero_two_distance {
            point_b = patterns[0];
            point_a = patterns[1];
            point_c = patterns[2];
        } else if zero_two_distance >= one_two_distance && zero_two_distance >= zero_one_distance {
            point_b = patterns[1];
            point_a = patterns[0];
            point_c = patterns[2];
        } else {
            point_b = patterns[2];
            point_a = patterns[0];
            point_c = patterns[1];
        }
        // should swap A and C.
        if ::cross_product_z(point_a, point_b, point_c) < 0.0f {
             let temp: ResultPoint = point_a;
            point_a = point_c;
            point_c = temp;
        }
        patterns[0] = point_a;
        patterns[1] = point_b;
        patterns[2] = point_c;
    }

    /**
   * @param pattern1 first pattern
   * @param pattern2 second pattern
   * @return distance between two points
   */
    pub fn  distance( pattern1: &ResultPoint,  pattern2: &ResultPoint) -> f32  {
        return MathUtils::distance(pattern1.x, pattern1.y, pattern2.x, pattern2.y);
    }

    /**
   * Returns the z component of the cross product between vectors BC and BA.
   */
    fn  cross_product_z( point_a: &ResultPoint,  point_b: &ResultPoint,  point_c: &ResultPoint) -> f32  {
         let b_x: f32 = point_b.x;
         let b_y: f32 = point_b.y;
        return ((point_c.x - b_x) * (point_a.y - b_y)) - ((point_c.y - b_y) * (point_a.x - b_x));
    }
}

