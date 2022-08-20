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
use crate::common::BitMatrix;
use crate::{NotFoundException, RXingResultPoint};

/**
 * <p>A somewhat generic detector that looks for a barcode-like rectangular region within an image.
 * It looks within a mostly white region of an image for a region of black and white, but mostly
 * black. It returns the four corners of the region, as best it can determine.</p>
 *
 * @author Sean Owen
 * @deprecated without replacement since 3.3.0
 */
#[deprecated]
pub struct MonochromeRectangleDetector {
    MAX_MODULES: i32,

    image: BitMatrix,
}

impl MonochromeRectangleDetector {
    pub fn new(image: &BitMatrix) -> Self {
        Self {
            MAX_MODULES: 32,
            image: image,
        }
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
    pub fn detect() -> Result<Vec<RXingResultPoint>, NotFoundException> {
        let height = image.getHeight();
        let width = image.getWidth();
        let halfHeight = height / 2;
        let halfWidth = width / 2;
        let deltaY = Math.max(1, height / (MAX_MODULES * 8));
        let deltaX = Math.max(1, width / (MAX_MODULES * 8));

        let top = 0;
        let bottom = height;
        let left = 0;
        let right = width;
        let pointA = findCornerFromCenter(
            halfWidth,
            0,
            left,
            right,
            halfHeight,
            -deltaY,
            top,
            bottom,
            halfWidth / 2,
        );
        top = pointA.getY() - 1;
        let pointB = findCornerFromCenter(
            halfWidth,
            -deltaX,
            left,
            right,
            halfHeight,
            0,
            top,
            bottom,
            halfHeight / 2,
        );
        left = pointB.getX() - 1;
        let pointC = findCornerFromCenter(
            halfWidth,
            deltaX,
            left,
            right,
            halfHeight,
            0,
            top,
            bottom,
            halfHeight / 2,
        );
        right = pointC.getX() + 1;
        let pointD = findCornerFromCenter(
            halfWidth,
            0,
            left,
            right,
            halfHeight,
            deltaY,
            top,
            bottom,
            halfWidth / 2,
        );
        bottom = pointD.getY() + 1;

        // Go try to find point A again with better information -- might have been off at first.
        pointA = findCornerFromCenter(
            halfWidth,
            0,
            left,
            right,
            halfHeight,
            -deltaY,
            top,
            bottom,
            halfWidth / 4,
        );

        return Vec!([pointA, pointB, pointC, pointD]);
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
        lastRange = null;
        let y: i32 = centerY;
        let x: i32 = centerX;
        while (y < bottom && y >= top && x < right && x >= left) {
            let range: Vec::new();
            if (deltaX == 0) {
                // horizontal slices, up and down
                range = blackWhiteRange(y, maxWhiteRun, left, right, true);
            } else {
                // vertical slices, left and right
                range = blackWhiteRange(x, maxWhiteRun, top, bottom, false);
            }
            if (range == null) {
                if (lastRange == null) {
                    return Err(NotFoundException.getNotFoundInstance());
                }
                // lastRange was found
                if (deltaX == 0) {
                    let lastY = y - deltaY;
                    if (lastRange[0] < centerX) {
                        if (lastRange[1] > centerX) {
                            // straddle, choose one or the other based on direction
                            return RXingResultPoint::new(
                                lastRange[if deltaY > 0 { 0 } else { 1 }],
                                lastY,
                            );
                        }
                        return RXingResultPoint::new(lastRange[0], lastY);
                    } else {
                        return RXingResultPoint::new(lastRange[1], lastY);
                    }
                } else {
                    let lastX = x - deltaX;
                    if (lastRange[0] < centerY) {
                        if (lastRange[1] > centerY) {
                            return RXingResultPoint::new(
                                lastX,
                                lastRange[if deltaX < 0 { 0 } else { 1 }],
                            );
                        }
                        return RXingResultPoint::new(lastX, lastRange[0]);
                    } else {
                        return RXingResultPoint::new(lastX, lastRange[1]);
                    }
                }
            }
            lastRange = range;
            y += deltaY;
            x += deltaX
        }
        return Err(NotFoundException.getNotFoundInstance());
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
        fixedDimension: i32,
        maxWhiteRun: i32,
        minDim: i32,
        maxDim: i32,
        horizontal: bool,
    ) -> Option<Vec<i32>> {
        let center = (minDim + maxDim) / 2;

        // Scan left/up first
        let start = center;
        while (start >= minDim) {
            if (if horizontal {
                image.get(start, fixedDimension)
            } else {
                image.get(fixedDimension, start)
            }) {
                start = start - 1;
            } else {
                let whiteRunStart = start;
                start = start - 1;
                while start >= minDim
                    && !(if horizontal {
                        image.get(start, fixedDimension);
                    } else {
                        image.get(fixedDimension, start);
                    })
                {
                    start = start - 1;
                }
                let whiteRunSize = whiteRunStart - start;
                if (start < minDim || whiteRunSize > maxWhiteRun) {
                    start = whiteRunStart;
                    break;
                }
            }
        }
        start = start + 1;

        // Then try right/down
        let end = center;
        while (end < maxDim) {
            if (if horizontal {
                image.get(end, fixedDimension)
            } else {
                image.get(fixedDimension, end)
            }) {
                end = end + 1;
            } else {
                let whiteRunStart = end;
                end = end + 1;
                while end < maxDim
                    && !(if horizontal {
                        image.get(end, fixedDimension)
                    } else {
                        image.get(fixedDimension, end)
                    })
                {
                    end = end + 1;
                }
                let whiteRunSize = end - whiteRunStart;
                if (end >= maxDim || whiteRunSize > maxWhiteRun) {
                    end = whiteRunStart;
                    break;
                }
            }
        }
        end = end - 1;

        return if end > start {
            Some(Vec! {start, end})
        } else {
            None
        };
    }
}