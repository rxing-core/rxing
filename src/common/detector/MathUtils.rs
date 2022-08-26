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

//package com.google.zxing.common.detector;
use std::{f32, i32};

/**
 * General math-related and numeric utility functions.
 */

/**
 * Ends up being a bit faster than {@link Math#round(float)}. This merely rounds its
 * argument to the nearest int, where x.5 rounds up to x+1. Semantics of this shortcut
 * differ slightly from {@link Math#round(float)} in that half rounds down for negative
 * values. -2.5 rounds to -3, not -2. For purposes here it makes no difference.
 *
 * @param d real value to round
 * @return nearest {@code int}
 */
pub fn round(d: f32) -> i32 {
    return (d + (if d < 0.0f32 { -0.5f32 } else { 0.5f32 })) as i32;
}

/**
 * @param aX point A x coordinate
 * @param aY point A y coordinate
 * @param bX point B x coordinate
 * @param bY point B y coordinate
 * @return Euclidean distance between points A and B
 */
pub fn distance_float(aX: f32, aY: f32, bX: f32, bY: f32) -> f32 {
    let xDiff: f64 = (aX - bX).into();
    let yDiff: f64 = (aY - bY).into();
    return (xDiff * xDiff + yDiff * yDiff).sqrt() as f32;
}

/**
 * @param aX point A x coordinate
 * @param aY point A y coordinate
 * @param bX point B x coordinate
 * @param bY point B y coordinate
 * @return Euclidean distance between points A and B
 */
pub fn distance_int(aX: i32, aY: i32, bX: i32, bY: i32) -> f32 {
    let xDiff: f64 = (aX - bX).into();
    let yDiff: f64 = (aY - bY).into();
    return (xDiff * xDiff + yDiff * yDiff).sqrt() as f32;
}

/**
 * @param array values to sum
 * @return sum of values in array
 */
pub fn sum(array: &[i32]) -> i32 {
    let mut count = 0;
    for a in array {
        count += a;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use crate::common::detector::MathUtils;

    static EPSILON: f32 = 1.0E-8f32;

    #[test]
    fn testRound() {
        assert_eq!(-1, MathUtils::round(-1.0f32));
        assert_eq!(0, MathUtils::round(0.0f32));
        assert_eq!(1, MathUtils::round(1.0f32));

        assert_eq!(2, MathUtils::round(1.9f32));
        assert_eq!(2, MathUtils::round(2.1f32));

        assert_eq!(3, MathUtils::round(2.5f32));

        assert_eq!(-2, MathUtils::round(-1.9f32));
        assert_eq!(-2, MathUtils::round(-2.1f32));

        assert_eq!(-3, MathUtils::round(-2.5f32)); // This differs from Math.round()

        assert_eq!(i32::MAX, MathUtils::round(i32::MAX as f32));
        assert_eq!(i32::MIN, MathUtils::round(i32::MIN as f32));

        assert_eq!(i32::MAX, MathUtils::round(f32::MAX));
        assert_eq!(i32::MIN, MathUtils::round(f32::NEG_INFINITY));

        assert_eq!(0, MathUtils::round(f32::NAN));
    }

    #[test]
    fn testDistance() {
        assert_eq!(
            (8.0f32).sqrt(),
            MathUtils::distance_float(1.0f32, 2.0f32, 3.0f32, 4.0f32)
        );
        assert_eq!(
            0.0f32,
            MathUtils::distance_float(1.0f32, 2.0f32, 1.0f32, 2.0f32)
        );

        assert_eq!((8.0f32).sqrt(), MathUtils::distance_int(1, 2, 3, 4));
        assert_eq!(0.0f32, MathUtils::distance_int(1, 2, 1, 2));
    }

    #[test]
    fn testSum() {
        assert_eq!(0, MathUtils::sum(&vec![]));
        assert_eq!(1, MathUtils::sum(&[1]));
        assert_eq!(4, MathUtils::sum(&[1, 3]));
        assert_eq!(0, MathUtils::sum(&[-1, 1]));
    }
}
