pub mod MathUtils;
use crate::common::BitMatrix;
use crate::{NotFoundException, RXingResultPoint};

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

//package com.google.zxing.common.detector;

/**
 * <p>A somewhat generic detector that looks for a barcode-like rectangular region within an image.
 * It looks within a mostly white region of an image for a region of black and white, but mostly
 * black. It returns the four corners of the region, as best it can determine.</p>
 *
 * @author Sean Owen
 * @deprecated without replacement since 3.3.0
 */
const MAX_MODULES: i32 = 32;
#[deprecated]
pub struct MonochromeRectangleDetector {
    image: BitMatrix,
}

impl MonochromeRectangleDetector {
    pub fn new(image: &BitMatrix) -> Self {
        Self { image: image.clone() }
    }

    /**
     * <p>Detects a rectangular region of black and white -- mostly black -- with a region of mostly
     * white, in an image.</p>
     *
     * @return {@link RXingResultPoint}[] describing the corners of the rectangular region. The first and
     *  last points are opposed on the diagonal, as are the second and third. The first point will be
     *  the topmost point and the last, the bottommost. The second point will be leftmost and the
     *  third, the rightmost
     * @throws NotFoundException if no Data Matrix Code can be found
     */
    pub fn detect(&self) -> Result<Vec<RXingResultPoint>, NotFoundException> {
        let height = self.image.getHeight() as i32;
        let width = self.image.getWidth() as i32;
        let halfHeight= height / 2;
        let halfWidth = width / 2;
        let deltaY = 1.max(height as i32 / (MAX_MODULES * 8));
        let deltaX = 1.max(width as i32 / (MAX_MODULES * 8));

        let mut top = 0;
        let mut bottom = height;
        let mut left = 0;
        let mut right = width;
        let mut pointA = self.findCornerFromCenter(
            halfWidth,
            0,
            left,
            right,
            halfHeight,
            -deltaY,
            top,
            bottom,
            halfWidth / 2,
        )?;
        top = (pointA.getY() - 1f32) as i32;
        let pointB = self.findCornerFromCenter(
            halfWidth,
            -deltaX,
            left,
            right,
            halfHeight,
            0,
            top,
            bottom,
            halfHeight / 2,
        )?;
        left = (pointB.getX() - 1f32) as i32;
        let pointC = self.findCornerFromCenter(
            halfWidth,
            deltaX,
            left,
            right,
            halfHeight,
            0,
            top,
            bottom,
            halfHeight / 2,
        )?;
        right = (pointC.getX() + 1f32) as i32;
        let pointD = self.findCornerFromCenter(
            halfWidth,
            0,
            left,
            right,
            halfHeight,
            deltaY,
            top,
            bottom,
            halfWidth / 2,
        )?;
        bottom = (pointD.getY() + 1f32) as i32;

        // Go try to find point A again with better information -- might have been off at first.
        pointA = self.findCornerFromCenter(
            halfWidth,
            0,
            left,
            right,
            halfHeight,
            -deltaY,
            top,
            bottom,
            halfWidth / 4,
        )?;

        return Ok(vec![pointA, pointB, pointC, pointD]);
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
     * @return a {@link RXingResultPoint} encapsulating the corner that was found
     * @throws NotFoundException if such a point cannot be found
     */
    fn findCornerFromCenter(
        &self,
        centerX: i32,
        deltaX: i32,
        left: i32,
        right: i32,
        centerY: i32,
        deltaY: i32,
        top: i32,
        bottom: i32,
        maxWhiteRun: i32,
    ) -> Result<RXingResultPoint, NotFoundException> {
        let mut lastRange_z: Option<Vec<i32>> = None;
        let mut y: i32 = centerY;
        let mut x: i32 = centerX;
        while y < bottom && y >= top && x < right && x >= left {
            let range: Option<Vec<i32>>;
            if deltaX == 0 {
                // horizontal slices, up and down
                range = self.blackWhiteRange(y, maxWhiteRun, left, right, true);
            } else {
                // vertical slices, left and right
                range = self.blackWhiteRange(x, maxWhiteRun, top, bottom, false);
            }
            if range.is_none() {
                if let Some(lastRange) = lastRange_z {
                // lastRange was found
                if deltaX == 0 {
                    let lastY = y - deltaY;
                    if lastRange[0] < centerX {
                        if lastRange[1] > centerX {
                            // straddle, choose one or the other based on direction
                            return Ok(RXingResultPoint::new(
                                lastRange[if deltaY > 0 { 0 } else { 1 }] as f32,
                                lastY as f32,
                            ));
                        }
                        return Ok(RXingResultPoint::new(
                            lastRange[0] as f32,
                            lastY as f32,
                        ));
                    } else {
                        return Ok(RXingResultPoint::new(
                            lastRange[1] as f32,
                            lastY as f32,
                        ));
                    }
                } else {
                    let lastX = x - deltaX;
                    if lastRange[0] < centerY {
                        if lastRange[1] > centerY {
                            return Ok(RXingResultPoint::new(
                                lastX as f32,
                                lastRange[if deltaX < 0 { 0 } else { 1 }] as f32,
                            ));
                        }
                        return Ok(RXingResultPoint::new(
                            lastX as f32,
                            lastRange[0] as f32,
                        ));
                    } else {
                        return Ok(RXingResultPoint::new(
                            lastX as f32,
                            lastRange[1] as f32,
                        ));
                    }
                }
            }}else {
                return Err(NotFoundException {});
            }
            lastRange_z = range;
            y += deltaY;
            x += deltaX
        }
        return Err(NotFoundException {});
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
    fn blackWhiteRange(
        &self,
        fixedDimension: i32,
        maxWhiteRun: i32,
        minDim: i32,
        maxDim: i32,
        horizontal: bool,
    ) -> Option<Vec<i32>> {
        let center = (minDim + maxDim) / 2;

        // Scan left/up first
        let mut start = center;
        while (start >= minDim) {
            if if horizontal {
                self.image.get(start as u32, fixedDimension as u32)
            } else {
                self.image.get(fixedDimension as u32, start as u32)
            } {
                start = start - 1;
            } else {
                let whiteRunStart = start;
                start = start - 1;
                while start >= minDim
                    && !(if horizontal {
                        self.image.get(start as u32, fixedDimension as u32)
                    } else {
                        self.image.get(fixedDimension as u32, start as u32)
                    })
                {
                    start = start - 1;
                }
                let whiteRunSize = whiteRunStart - start;
                if start < minDim || whiteRunSize > maxWhiteRun {
                    start = whiteRunStart;
                    break;
                }
            }
        }
        start = start + 1;

        // Then try right/down
        let mut end = center;
        while (end < maxDim) {
            if if horizontal {
                self.image.get(end as u32, fixedDimension as u32)
            } else {
                self.image.get(fixedDimension as u32, end as u32)
            } {
                end = end + 1;
            } else {
                let whiteRunStart = end;
                end = end + 1;
                while end < maxDim
                    && !(if horizontal {
                        self.image.get(end as u32, fixedDimension as u32)
                    } else {
                        self.image.get(fixedDimension as u32, end as u32)
                    })
                {
                    end = end + 1;
                }
                let whiteRunSize = end - whiteRunStart;
                if end >= maxDim || whiteRunSize > maxWhiteRun {
                    end = whiteRunStart;
                    break;
                }
            }
        }
        end = end - 1;

        return if end > start {
            Some(vec![start, end])
        } else {
            None
        };
    }
}

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
    image: BitMatrix,
    height: i32,
    width: i32,
    leftInit: i32,
    rightInit: i32,
    downInit: i32,
    upInit: i32,
}

impl WhiteRectangleDetector {
    pub fn new_from_image(image: &BitMatrix) -> Result<Self, NotFoundException> {
        Self::new(
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
        image: &BitMatrix,
        initSize: i32,
        x: i32,
        y: i32,
    ) -> Result<Self, NotFoundException> {
        
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
            return Err(NotFoundException {});
        }

        Ok(Self{
            image: image.clone(),
            height: image.getHeight() as i32,
            width: image.getWidth() as i32,
            leftInit: leftInit,
            rightInit: rightInit,
            downInit: downInit,
            upInit: upInit,
        })
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
    pub fn detect(&self) -> Result<Vec<RXingResultPoint>, NotFoundException> {
        let mut left: i32 = self.leftInit;
        let mut right: i32 = self.rightInit;
        let mut up: i32 = self.upInit;
        let mut down: i32 = self.downInit;
        let mut sizeExceeded = false;
        let mut aBlackPointFoundOnBorder = true;

        let mut atLeastOneBlackPointFoundOnRight = false;
        let mut atLeastOneBlackPointFoundOnBottom = false;
        let mut atLeastOneBlackPointFoundOnLeft = false;
        let mut atLeastOneBlackPointFoundOnTop = false;

        while aBlackPointFoundOnBorder {
            aBlackPointFoundOnBorder = false;

            // .....
            // .   |
            // .....
            let mut rightBorderNotWhite = true;
            while (rightBorderNotWhite || !atLeastOneBlackPointFoundOnRight) && right < self.width {
                rightBorderNotWhite = self.containsBlackPoint(up, down, right, false);
                if rightBorderNotWhite {
                    right += 1;
                    aBlackPointFoundOnBorder = true;
                    atLeastOneBlackPointFoundOnRight = true;
                } else if !atLeastOneBlackPointFoundOnRight {
                    right += 1;
                }
            }

            if right >= self.width {
                sizeExceeded = true;
                break;
            }

            // .....
            // .   .
            // .___.
            let mut bottomBorderNotWhite = true;
            while (bottomBorderNotWhite || !atLeastOneBlackPointFoundOnBottom) && down < self.height
            {
                bottomBorderNotWhite = self.containsBlackPoint(left, right, down, true);
                if bottomBorderNotWhite {
                    down += 1;
                    aBlackPointFoundOnBorder = true;
                    atLeastOneBlackPointFoundOnBottom = true;
                } else if !atLeastOneBlackPointFoundOnBottom {
                    down += 1;
                }
            }

            if down >= self.height {
                sizeExceeded = true;
                break;
            }

            // .....
            // |   .
            // .....
            let mut leftBorderNotWhite = true;
            while (leftBorderNotWhite || !atLeastOneBlackPointFoundOnLeft) && left >= 0 {
                leftBorderNotWhite = self.containsBlackPoint(up, down, left, false);
                if leftBorderNotWhite {
                    left -= 1;
                    aBlackPointFoundOnBorder = true;
                    atLeastOneBlackPointFoundOnLeft = true;
                } else if !atLeastOneBlackPointFoundOnLeft {
                    left -= 1;
                }
            }

            if left < 0 {
                sizeExceeded = true;
                break;
            }

            // .___.
            // .   .
            // .....
            let mut topBorderNotWhite = true;
            while (topBorderNotWhite || !atLeastOneBlackPointFoundOnTop) && up >= 0 {
                topBorderNotWhite = self.containsBlackPoint(left, right, up, true);
                if topBorderNotWhite {
                    up -= 1;
                    aBlackPointFoundOnBorder = true;
                    atLeastOneBlackPointFoundOnTop = true;
                } else if !atLeastOneBlackPointFoundOnTop {
                    up -= 1;
                }
            }

            if up < 0 {
                sizeExceeded = true;
                break;
            }
        }

        if !sizeExceeded {
            let maxSize = right - left;

            let mut z: Option<RXingResultPoint> = None;
            let mut i = 1;
            while z.is_none() && i < maxSize {
                //for (int i = 1; z == null && i < maxSize; i++) {
                z = self.getBlackPointOnSegment(
                    left as f32,
                    (down - i) as f32,
                    (left + i) as f32,
                    down as f32,
                );
                i += 1;
            }

            if z.is_none() {
                return Err(NotFoundException {});
            }

            let mut t: Option<RXingResultPoint> = None;
            //go down right
            let mut i = 1;
            while t.is_none() && i < maxSize {
                //for (int i = 1; t == null && i < maxSize; i++) {
                t = self.getBlackPointOnSegment(
                    left as f32,
                    (up + i) as f32,
                    (left + i) as f32,
                    up as f32,
                );
                i += 1;
            }

            if t.is_none() {
                return Err(NotFoundException {});
            }

            let mut x: Option<RXingResultPoint> = None;
            //go down left
            let mut i = 1;
            while x.is_none() && i < maxSize {
                //for (int i = 1; x == null && i < maxSize; i++) {
                x = self.getBlackPointOnSegment(
                    right as f32,
                    (up + i) as f32,
                    (right - i) as f32,
                    up as f32,
                );
                i += 1;
            }

            if x.is_none() {
                return Err(NotFoundException {});
            }

            let mut y: Option<RXingResultPoint> = None;
            //go up left
            let mut i = 1;
            while y.is_none() && i < maxSize {
                //for (int i = 1; y == null && i < maxSize; i++) {
                y = self.getBlackPointOnSegment(
                    right as f32,
                    (down - i) as f32,
                    (right - i) as f32,
                    down as f32,
                );
                i += 1;
            }

            if y.is_none() {
                return Err(NotFoundException {});
            }

            return Ok(self.centerEdges(&y.unwrap(), &z.unwrap(), &x.unwrap(), &t.unwrap()));
        } else {
            return Err(NotFoundException {});
        }
    }

    fn getBlackPointOnSegment(
        &self,
        aX: f32,
        aY: f32,
        bX: f32,
        bY: f32,
    ) -> Option<RXingResultPoint> {
        let dist = MathUtils::round(MathUtils::distance_float(aX, aY, bX, bY));
        let xStep: f32 = (bX - aX) / dist as f32;
        let yStep: f32 = (bY - aY) / dist as f32;

        for i in 0..dist {
            let x = MathUtils::round(aX + i as f32 * xStep);
            let y = MathUtils::round(aY + i as f32 * yStep);
            if self.image.get(x as u32, y as u32) {
                return Some(RXingResultPoint::new(x as f32, y as f32));
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
    fn centerEdges(
        &self,
        y: &RXingResultPoint,
        z: &RXingResultPoint,
        x: &RXingResultPoint,
        t: &RXingResultPoint,
    ) -> Vec<RXingResultPoint> {
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

        if yi < self.width as f32 / 2.0f32 {
            return vec![
                RXingResultPoint::new(ti - CORR as f32, tj + CORR as f32),
                RXingResultPoint::new(zi + CORR as f32, zj + CORR as f32),
                RXingResultPoint::new(xi - CORR as f32, xj - CORR as f32),
                RXingResultPoint::new(yi + CORR as f32, yj - CORR as f32),
            ];
        } else {
            return vec![
                RXingResultPoint::new(ti + CORR as f32, tj + CORR as f32),
                RXingResultPoint::new(zi + CORR as f32, zj - CORR as f32),
                RXingResultPoint::new(xi - CORR as f32, xj + CORR as f32),
                RXingResultPoint::new(yi - CORR as f32, yj - CORR as f32),
            ];
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
    fn containsBlackPoint(&self, a: i32, b: i32, fixed: i32, horizontal: bool) -> bool {
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

        return false;
    }
}
