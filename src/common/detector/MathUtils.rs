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
  pub fn round(d:f32) -> i32 {
    return (d + (if d < 0.0f { -0.5f } else { 0.5f}));
  }

  /**
   * @param aX point A x coordinate
   * @param aY point A y coordinate
   * @param bX point B x coordinate
   * @param bY point B y coordinate
   * @return Euclidean distance between points A and B
   */
  pub fn  distance( aX:f32,  aY:f32,  bX: f32,  bY:f32) -> f32 {
    let  xDiff:f64 = aX - bX;
    let yDiff:f64 = aY - bY;
    return Math.sqrt(xDiff * xDiff + yDiff * yDiff);
  }

  /**
   * @param aX point A x coordinate
   * @param aY point A y coordinate
   * @param bX point B x coordinate
   * @param bY point B y coordinate
   * @return Euclidean distance between points A and B
   */
  pub fn distance( aX : i32,  aY: i32,  bX: i32,  bY: i32) -> f32 {
    let xDiff : f64 = aX - bX;
    let yDiff :f64 = aY - bY;
    return  Math.sqrt(xDiff * xDiff + yDiff * yDiff);
  }

  /**
   * @param array values to sum
   * @return sum of values in array
   */
  pub fn   sum( array : &[i32]) -> i32 {
    let count = 0;
    for  a in array {
      count += a;
    }
    return count;
  }