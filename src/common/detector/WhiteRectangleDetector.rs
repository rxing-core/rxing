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

use crate::{NotFoundException,RXingResultPoint};
use crate::common::BitMatrix;

use super::MathUtils;

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
const INIT_SIZE:i32 = 10;
const CORR:i32 = 1;
pub struct WhiteRectangleDetector {

  image: BitMatrix,
   height: i32,
  width: i32,
  leftInit: i32,
  rightInit: i32,
  downInit: i32,
  upInit: i32,
}

impl WhiteRectangleDetector {

  pub fn new_from_image(image:&BitMatrix) -> Result<Self,NotFoundException> {
    Self::new(image, INIT_SIZE, image.getWidth() / 2, image.getHeight() / 2)
  }

  /**
   * @param image barcode image to find a rectangle in
   * @param initSize initial size of search area around center
   * @param x x position of search center
   * @param y y position of search center
   * @throws NotFoundException if image is too small to accommodate {@code initSize}
   */
  pub fn new( image:&BitMatrix,  initSize:i32,  x:i32,  y:i32) -> Result<Self, NotFoundException> {
    let new_wrd : Self;
    new_wrd.image = image;
    new_wrd.height = image.getHeight();
    new_wrd.width = image.getWidth();
    let halfsize = initSize / 2;
    new_wrd.leftInit = x - halfsize;
    new_wrd.rightInit = x + halfsize;
    new_wrd.upInit = y - halfsize;
    new_wrd.downInit = y + halfsize;
    if (new_wrd.upInit < 0 || new_wrd.leftInit < 0 || new_wrd.downInit >= new_wrd.height || new_wrd.rightInit >= new_wrd.width) {
      return Err( NotFoundException.getNotFoundInstance());
    }
    
    Ok(new_wrd)
  }

  /**
   * <p>
   * Detects a candidate barcode-like rectangular region within an image. It
   * starts around the center of the image, increases the size of the candidate
   * region until it finds a white rectangular region.
   * </p>
   *
   * @return {@link RXingResultPoint}[] describing the corners of the rectangular
   *         region. The first and last points are opposed on the diagonal, as
   *         are the second and third. The first point will be the topmost
   *         point and the last, the bottommost. The second point will be
   *         leftmost and the third, the rightmost
   * @throws NotFoundException if no Data Matrix Code can be found
   */
  pub fn  detect(&self) -> Result<Vec<RXingResultPoint>, NotFoundException> {

    let left :i32= self.leftInit;
    let right:i32 = self.rightInit;
    let up:i32 = self.upInit;
    let down:i32 = self.downInit;
    let sizeExceeded = false;
    let aBlackPointFoundOnBorder = true;

    let atLeastOneBlackPointFoundOnRight = false;
    let atLeastOneBlackPointFoundOnBottom = false;
    let atLeastOneBlackPointFoundOnLeft = false;
    let atLeastOneBlackPointFoundOnTop = false;

    while (aBlackPointFoundOnBorder) {

      aBlackPointFoundOnBorder = false;

      // .....
      // .   |
      // .....
      let rightBorderNotWhite = true;
      while ((rightBorderNotWhite || !atLeastOneBlackPointFoundOnRight) && right < self.width) {
        rightBorderNotWhite = self.containsBlackPoint(up, down, right, false);
        if (rightBorderNotWhite) {
          right += 1;
          aBlackPointFoundOnBorder = true;
          atLeastOneBlackPointFoundOnRight = true;
        } else if (!atLeastOneBlackPointFoundOnRight) {
          right+=1;
        }
      }

      if (right >= self.width) {
        sizeExceeded = true;
        break;
      }

      // .....
      // .   .
      // .___.
      let bottomBorderNotWhite = true;
      while ((bottomBorderNotWhite || !atLeastOneBlackPointFoundOnBottom) && down < self.height) {
        bottomBorderNotWhite = self.containsBlackPoint(left, right, down, true);
        if (bottomBorderNotWhite) {
          down+=1;
          aBlackPointFoundOnBorder = true;
          atLeastOneBlackPointFoundOnBottom = true;
        } else if (!atLeastOneBlackPointFoundOnBottom) {
          down+=1;
        }
      }

      if (down >= self.height) {
        sizeExceeded = true;
        break;
      }

      // .....
      // |   .
      // .....
      let leftBorderNotWhite = true;
      while ((leftBorderNotWhite || !atLeastOneBlackPointFoundOnLeft) && left >= 0) {
        leftBorderNotWhite = self.containsBlackPoint(up, down, left, false);
        if (leftBorderNotWhite) {
          left-=1;
          aBlackPointFoundOnBorder = true;
          atLeastOneBlackPointFoundOnLeft = true;
        } else if (!atLeastOneBlackPointFoundOnLeft) {
          left-=1;
        }
      }

      if (left < 0) {
        sizeExceeded = true;
        break;
      }

      // .___.
      // .   .
      // .....
      let topBorderNotWhite = true;
      while ((topBorderNotWhite || !atLeastOneBlackPointFoundOnTop) && up >= 0) {
        topBorderNotWhite = self.containsBlackPoint(left, right, up, true);
        if (topBorderNotWhite) {
          up-=1;
          aBlackPointFoundOnBorder = true;
          atLeastOneBlackPointFoundOnTop = true;
        } else if (!atLeastOneBlackPointFoundOnTop) {
          up-=1;
        }
      }

      if (up < 0) {
        sizeExceeded = true;
        break;
      }

    }

    if (!sizeExceeded) {

      let maxSize = right - left;

      let mut z: Option<RXingResultPoint> = None;
      let mut i = 1;
      while z.is_none() && i < maxSize {
      //for (int i = 1; z == null && i < maxSize; i++) {
        z = self.getBlackPointOnSegment(left, down - i, left + i, down);
        i+=1;
      }

      if (z .is_none()) {
        return Err( NotFoundException.getNotFoundInstance());
      }

      let mut  t : Option<RXingResultPoint> = None;
      //go down right
      let mut i = 1;
      while t.is_none() && i < maxSize {
      //for (int i = 1; t == null && i < maxSize; i++) {
        t = self.getBlackPointOnSegment(left, up + i, left + i, up);
        i+=1;
      }

      if (t .is_none()) {
        return Err( NotFoundException.getNotFoundInstance());
      }

      let mut  x : Option<RXingResultPoint> = None;
      //go down left
      let mut i = 1;
      while x.is_none() && i < maxSize {
      //for (int i = 1; x == null && i < maxSize; i++) {
        x = self.getBlackPointOnSegment(right, up + i, right - i, up);
        i += 1;
      }

      if (x .is_none()) {
        return Err( NotFoundException.getNotFoundInstance());
      }

      let mut  y : Option<RXingResultPoint> = None;
      //go up left
      let mut i = 1;
      while y.is_none() && i < maxSize {
      //for (int i = 1; y == null && i < maxSize; i++) {
        y = self.getBlackPointOnSegment(right, down - i, right - i, down);
        i+=1;
      }

      if (y .is_none()) {
        return Err( NotFoundException.getNotFoundInstance());
      }

      return self.centerEdges(y, z, x, t);

    } else {
      return Err( NotFoundException.getNotFoundInstance());
    }
  }

  fn  getBlackPointOnSegment(&self, aX:f32,  aY:f32,  bX:f32,  bY:f32) -> Option<RXingResultPoint> {
    let dist = MathUtils::round(MathUtils::distance_float(aX, aY, bX, bY));
    let xStep :f32= (bX - aX) / dist;
    let yStep:f32 = (bY - aY) / dist;

    for i in 0..dist {
      let x = MathUtils::round(aX + i * xStep);
      let y = MathUtils::round(aY + i * yStep);
      if (self.image.get(x, y)) {
        return  RXingResultPoint::new(x, y);
      }
    }
    return None;
  }

  /**
   * recenters the points of a constant distance towards the center
   *
   * @param y bottom most point
   * @param z left most point
   * @param x right most point
   * @param t top most point
   * @return {@link RXingResultPoint}[] describing the corners of the rectangular
   *         region. The first and last points are opposed on the diagonal, as
   *         are the second and third. The first point will be the topmost
   *         point and the last, the bottommost. The second point will be
   *         leftmost and the third, the rightmost
   */
  fn  centerEdges( &self,y:&RXingResultPoint,  z:&RXingResultPoint,
                                     x:&RXingResultPoint,  t:&RXingResultPoint) -> Vec<RXingResultPoint> {

    //
    //       t            t
    //  z                      x
    //        x    OR    z
    //   y                    y
    //

    let yi = y.getX();
    let yj = y.getY();
    let zi = z.getX();
    let zj = z.getY();
    let xi = x.getX();
    let xj = x.getY();
    let ti = t.getX();
    let tj = t.getY();

    if (yi < self.width.into() / 2.0f32) {
      return vec!
          [ RXingResultPoint::new(ti - CORR, tj + CORR),
           RXingResultPoint::new(zi + CORR, zj + CORR),
           RXingResultPoint::new(xi - CORR, xj - CORR),
           RXingResultPoint::new(yi + CORR, yj - CORR)];
    } else {
      return vec![
           RXingResultPoint::new(ti + CORR, tj + CORR),
           RXingResultPoint::new(zi + CORR, zj - CORR),
           RXingResultPoint::new(xi - CORR, xj + CORR),
           RXingResultPoint::new(yi - CORR, yj - CORR)];
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
  fn  containsBlackPoint(&self, a:i32,  b:i32,  fixed:i32,  horizontal:bool) -> bool {

    if (horizontal) {
      
      for x in a..=b {
        if (self.image.get(x, fixed)) {
          return true;
        }
      }
    } else {
      
      for y in a..=b {
        if (self.image.get(fixed, y)) {
          return true;
        }
      }
    }

    return false;
  }

}
