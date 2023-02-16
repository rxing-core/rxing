/*
 * Copyright 2012 ZXing authors
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

use std::ops::Add;

/**
 * General math-related and numeric utility functions.
 */

// /**
//  * @param aX point A x coordinate
//  * @param aY point A y coordinate
//  * @param bX point B x coordinate
//  * @param bY point B y coordinate
//  * @return Euclidean distance between points A and B
//  */
// #[inline(always)]
// pub fn distance_float(aX: f32, aY: f32, bX: f32, bY: f32) -> f32 {
//     let xDiff: f64 = (aX - bX).into();
//     let yDiff: f64 = (aY - bY).into();
//     (xDiff * xDiff + yDiff * yDiff).sqrt() as f32
// }

// /**
//  * @param aX point A x coordinate
//  * @param aY point A y coordinate
//  * @param bX point B x coordinate
//  * @param bY point B y coordinate
//  * @return Euclidean distance between points A and B
//  */
// #[inline(always)]
// pub fn distance_int(aX: i32, aY: i32, bX: i32, bY: i32) -> f32 {
//     let xDiff: f64 = (aX - bX).into();
//     let yDiff: f64 = (aY - bY).into();
//     (xDiff * xDiff + yDiff * yDiff).sqrt() as f32
// }

/**
 * @param array values to sum
 * @return sum of values in array
 */
#[inline(always)]
pub fn sum<'a, T>(array: &'a [T]) -> T
where
    T: Add + std::iter::Sum<&'a T>,
{
    array.iter().sum()
}

#[cfg(test)]
mod tests {
    // static EPSILON: f32 = 1.0E-8f32;

    #[test]
    fn testRound() {
        assert_eq!(-1, (-1.0f32).round() as i32);
        assert_eq!(0, (0.0f32).round() as i32);
        assert_eq!(1, (1.0f32).round() as i32);

        assert_eq!(2, (1.9f32).round() as i32);
        assert_eq!(2, (2.1f32).round() as i32);

        assert_eq!(3, (2.5f32).round() as i32);

        assert_eq!(-2, (-1.9f32).round() as i32);
        assert_eq!(-2, (-2.1f32).round() as i32);

        assert_eq!(-3, (-2.5f32).round() as i32); // This differs from Math.round()

        assert_eq!(i32::MAX, (i32::MAX as f32).round() as i32);
        assert_eq!(i32::MIN, (i32::MIN as f32).round() as i32);

        assert_eq!(i32::MAX, (f32::MAX).round() as i32);
        assert_eq!(i32::MIN, (f32::NEG_INFINITY).round() as i32);

        assert_eq!(0, (f32::NAN).round() as i32);
    }

    // #[test]
    // fn testSum() {
    //     assert_eq!(0, MathUtils::sum(&[]));
    //     assert_eq!(1, MathUtils::sum(&[1]));
    //     assert_eq!(4, MathUtils::sum(&[1, 3]));
    //     assert_eq!(0, MathUtils::sum(&[-1, 1]));
    //     assert_eq!(0.0, MathUtils::sum(&[-1.0, 1.0]));
    //     assert_eq!(4.0, MathUtils::sum(&[1.0, 3.0]));
    // }
}
