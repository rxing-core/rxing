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

use std::collections::HashMap;

use crate::{
    common::{
        detector::MathUtils, BitMatrix, DefaultGridSampler, GridSampler, PerspectiveTransform,
    },
    qrcode::decoder::Version,
    result_point_utils, DecodeHintType, DecodeHintValue, DecodingHintDictionary, Exceptions,
    RXingResultPointCallback, ResultPoint,
};

use super::{
    AlignmentPattern, AlignmentPatternFinder, FinderPatternFinder, FinderPatternInfo,
    QRCodeDetectorResult,
};

/**
 * <p>Encapsulates logic that can detect a QR Code in an image, even if the QR Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 */
pub struct Detector {
    image: BitMatrix,
    resultPointCallback: Option<RXingResultPointCallback>,
}

impl Detector {
    pub fn new(image: BitMatrix) -> Self {
        Self {
            image,
            resultPointCallback: None,
        }
    }

    pub fn getImage(&self) -> &BitMatrix {
        &self.image
    }

    pub fn getRXingResultPointCallback(&self) -> &Option<RXingResultPointCallback> {
        &self.resultPointCallback
    }

    /**
     * <p>Detects a QR Code in an image.</p>
     *
     * @return {@link DetectorRXingResult} encapsulating results of detecting a QR Code
     * @throws NotFoundException if QR Code cannot be found
     * @throws FormatException if a QR Code cannot be decoded
     */
    pub fn detect(&mut self) -> Result<QRCodeDetectorResult, Exceptions> {
        self.detect_with_hints(&HashMap::new())
    }

    /**
     * <p>Detects a QR Code in an image.</p>
     *
     * @param hints optional hints to detector
     * @return {@link DetectorRXingResult} encapsulating results of detecting a QR Code
     * @throws NotFoundException if QR Code cannot be found
     * @throws FormatException if a QR Code cannot be decoded
     */
    pub fn detect_with_hints(
        &mut self,
        hints: &DecodingHintDictionary,
    ) -> Result<QRCodeDetectorResult, Exceptions> {
        self.resultPointCallback =
            if let Some(nrpc) = hints.get(&DecodeHintType::NEED_RESULT_POINT_CALLBACK) {
                if let DecodeHintValue::NeedResultPointCallback(cb) = nrpc {
                    Some(*cb)
                } else {
                    None
                }
            } else {
                None
            };

        // self.resultPointCallback = hints.get(&DecodeHintType::NEED_RESULT_POINT_CALLBACK);
        // resultPointCallback = hints == null ? null :
        //     (RXingResultPointCallback) hints.get(DecodeHintType.NEED_RESULT_POINT_CALLBACK);

        let mut finder =
            FinderPatternFinder::with_callback(self.image.clone(), self.resultPointCallback);
        let info = finder.find(hints)?;

        self.processFinderPatternInfo(info)
    }

    pub fn processFinderPatternInfo(
        &self,
        info: FinderPatternInfo,
    ) -> Result<QRCodeDetectorResult, Exceptions> {
        let topLeft = info.getTopLeft();
        let topRight = info.getTopRight();
        let bottomLeft = info.getBottomLeft();

        let moduleSize = self.calculateModuleSize(topLeft, topRight, bottomLeft);
        if moduleSize < 1.0 {
            return Err(Exceptions::NotFoundException("not found".to_owned()));
        }
        let dimension = Self::computeDimension(topLeft, topRight, bottomLeft, moduleSize)?;
        let provisionalVersion = Version::getProvisionalVersionForDimension(dimension)?;
        let modulesBetweenFPCenters = provisionalVersion.getDimensionForVersion() - 7;

        let mut alignmentPattern = None;
        // Anything above version 1 has an alignment pattern
        if provisionalVersion.getAlignmentPatternCenters().len() > 0 {
            // Guess where a "bottom right" finder pattern would have been
            let bottomRightX = topRight.getX() - topLeft.getX() + bottomLeft.getX();
            let bottomRightY = topRight.getY() - topLeft.getY() + bottomLeft.getY();

            // Estimate that alignment pattern is closer by 3 modules
            // from "bottom right" to known top left location
            let correctionToTopLeft = 1.0 - (3.0 / modulesBetweenFPCenters as f32);
            let estAlignmentX =
                (topLeft.getX() + correctionToTopLeft * (bottomRightX - topLeft.getX())) as u32;
            let estAlignmentY =
                (topLeft.getY() + correctionToTopLeft * (bottomRightY - topLeft.getY())) as u32;

            // Kind of arbitrary -- expand search radius before giving up
            let mut i = 4;
            while i <= 16 {
                // for (int i = 4; i <= 16; i <<= 1) {
                if let Ok(ap) =
                    self.findAlignmentInRegion(moduleSize, estAlignmentX, estAlignmentY, i as f32)
                {
                    alignmentPattern = Some(ap);
                    break;
                }
                // try {
                //   alignmentPattern = findAlignmentInRegion(moduleSize,
                //       estAlignmentX,
                //       estAlignmentY,
                //       i);
                //   break;
                // } catch (NotFoundException re) {
                //   // try next round
                // }
                i <<= 1;
            }
            // If we didn't find alignment pattern... well try anyway without it
        }

        let ap_ref = if alignmentPattern.is_some() {
            Some(alignmentPattern.as_ref().unwrap())
        } else {
            None
        };

        let transform = Self::createTransform(topLeft, topRight, bottomLeft, ap_ref, dimension);

        let bits = Detector::sampleGrid(&self.image, &transform, dimension)?;

        let points = if alignmentPattern.is_none() {
            vec![
                bottomLeft.into_rxing_result_point(),
                topLeft.into_rxing_result_point(),
                topRight.into_rxing_result_point(),
            ]
        } else {
            vec![
                bottomLeft.into_rxing_result_point(),
                topLeft.into_rxing_result_point(),
                topRight.into_rxing_result_point(),
                alignmentPattern.unwrap().into_rxing_result_point(),
            ]
        };

        Ok(QRCodeDetectorResult::new(bits, points))
    }

    fn createTransform<T: ResultPoint, X: ResultPoint>(
        topLeft: &T,
        topRight: &T,
        bottomLeft: &T,
        alignmentPattern: Option<&X>,
        dimension: u32,
    ) -> PerspectiveTransform {
        let dimMinusThree = dimension as f32 - 3.5;
        let bottomRightX: f32;
        let bottomRightY: f32;
        let sourceBottomRightX: f32;
        let sourceBottomRightY: f32;
        if alignmentPattern.is_some() {
            let alignmentPattern = alignmentPattern.as_ref().unwrap();
            bottomRightX = alignmentPattern.getX();
            bottomRightY = alignmentPattern.getY();
            sourceBottomRightX = dimMinusThree - 3.0;
            sourceBottomRightY = sourceBottomRightX;
        } else {
            // Don't have an alignment pattern, just make up the bottom-right point
            bottomRightX = (topRight.getX() - topLeft.getX()) + bottomLeft.getX();
            bottomRightY = (topRight.getY() - topLeft.getY()) + bottomLeft.getY();
            sourceBottomRightX = dimMinusThree;
            sourceBottomRightY = dimMinusThree;
        }

        return PerspectiveTransform::quadrilateralToQuadrilateral(
            3.5,
            3.5,
            dimMinusThree,
            3.5,
            sourceBottomRightX,
            sourceBottomRightY,
            3.5,
            dimMinusThree,
            topLeft.getX(),
            topLeft.getY(),
            topRight.getX(),
            topRight.getY(),
            bottomRightX,
            bottomRightY,
            bottomLeft.getX(),
            bottomLeft.getY(),
        );
    }

    fn sampleGrid(
        image: &BitMatrix,
        transform: &PerspectiveTransform,
        dimension: u32,
    ) -> Result<BitMatrix, Exceptions> {
        let sampler = DefaultGridSampler {};
        return sampler.sample_grid(&image, dimension, dimension, transform);
    }

    /**
     * <p>Computes the dimension (number of modules on a size) of the QR Code based on the position
     * of the finder patterns and estimated module size.</p>
     */
    fn computeDimension<T: ResultPoint>(
        topLeft: &T,
        topRight: &T,
        bottomLeft: &T,
        moduleSize: f32,
    ) -> Result<u32, Exceptions> {
        let tltrCentersDimension =
            MathUtils::round(result_point_utils::distance(topLeft, topRight) / moduleSize);
        let tlblCentersDimension =
            MathUtils::round(result_point_utils::distance(topLeft, bottomLeft) / moduleSize);
        let mut dimension = ((tltrCentersDimension + tlblCentersDimension) / 2) + 7;
        match dimension & 0x03 {
            0 => dimension += 1,
            2 => dimension -= 1,
            3 => return Err(Exceptions::NotFoundException("not found".to_owned())),
            _ => {}
        }
        // switch (dimension & 0x03) { // mod 4
        //   case 0:
        //     dimension++;
        //     break;
        //     // 1? do nothing
        //   case 2:
        //     dimension--;
        //     break;
        //   case 3:
        //   return Err(Exceptions::NotFoundException("not found".to_owned()))
        // }
        Ok(dimension as u32)
    }

    /**
     * <p>Computes an average estimated module size based on estimated derived from the positions
     * of the three finder patterns.</p>
     *
     * @param topLeft detected top-left finder pattern center
     * @param topRight detected top-right finder pattern center
     * @param bottomLeft detected bottom-left finder pattern center
     * @return estimated module size
     */
    pub fn calculateModuleSize<T: ResultPoint>(
        &self,
        topLeft: &T,
        topRight: &T,
        bottomLeft: &T,
    ) -> f32 {
        // Take the average
        return (self.calculateModuleSizeOneWay(topLeft, topRight)
            + self.calculateModuleSizeOneWay(topLeft, bottomLeft))
            / 2.0;
    }

    /**
     * <p>Estimates module size based on two finder patterns -- it uses
     * {@link #sizeOfBlackWhiteBlackRunBothWays(int, int, int, int)} to figure the
     * width of each, measuring along the axis between their centers.</p>
     */
    fn calculateModuleSizeOneWay<T: ResultPoint>(&self, pattern: &T, otherPattern: &T) -> f32 {
        let moduleSizeEst1 = self.sizeOfBlackWhiteBlackRunBothWays(
            pattern.getX().floor() as u32,
            pattern.getY().floor() as u32,
            otherPattern.getX().floor() as u32,
            otherPattern.getY().floor() as u32,
        );
        let moduleSizeEst2 = self.sizeOfBlackWhiteBlackRunBothWays(
            otherPattern.getX().floor() as u32,
            otherPattern.getY().floor() as u32,
            pattern.getX().floor() as u32,
            pattern.getY().floor() as u32,
        );
        if moduleSizeEst1.is_nan() {
            return moduleSizeEst2 / 7.0;
        }
        if moduleSizeEst2.is_nan() {
            return moduleSizeEst1 / 7.0;
        }
        // Average them, and divide by 7 since we've counted the width of 3 black modules,
        // and 1 white and 1 black module on either side. Ergo, divide sum by 14.
        return (moduleSizeEst1 + moduleSizeEst2) / 14.0;
    }

    /**
     * See {@link #sizeOfBlackWhiteBlackRun(int, int, int, int)}; computes the total width of
     * a finder pattern by looking for a black-white-black run from the center in the direction
     * of another point (another finder pattern center), and in the opposite direction too.
     */
    fn sizeOfBlackWhiteBlackRunBothWays(&self, fromX: u32, fromY: u32, toX: u32, toY: u32) -> f32 {
        let mut result = self.sizeOfBlackWhiteBlackRun(fromX, fromY, toX, toY);

        // Now count other way -- don't run off image though of course
        let mut scale = 1.0;
        let mut otherToX = fromX as i32 - (toX as i32 - fromX as i32);
        if otherToX < 0 {
            scale = fromX as f32 / (fromX as i32 - otherToX) as f32;
            otherToX = 0;
        } else if otherToX as u32 >= self.image.getWidth() {
            scale = (self.image.getWidth() as i32 - 1 - fromX as i32) as f32
                / (otherToX - fromX as i32) as f32;
            otherToX = self.image.getWidth() as i32 - 1;
        }
        let mut otherToY = (fromY as f32 - (toY as f32 - fromY as f32) * scale).floor() as i32;

        scale = 1.0;
        if otherToY < 0 {
            scale = fromY as f32 / (fromY as i32 - otherToY) as f32;
            otherToY = 0;
        } else if otherToY as u32 >= self.image.getHeight() {
            scale = (self.image.getHeight() as i32 - 1 - fromY as i32) as f32
                / (otherToY - fromY as i32) as f32;
            otherToY = self.image.getHeight() as i32 - 1;
        }
        otherToX = (fromX as f32 + (otherToX as f32 - fromX as f32) * scale).floor() as i32;

        result += self.sizeOfBlackWhiteBlackRun(
            fromX as u32,
            fromY as u32,
            otherToX as u32,
            otherToY as u32,
        );

        // Middle pixel is double-counted this way; subtract 1
        return result - 1.0;
    }

    /**
     * <p>This method traces a line from a point in the image, in the direction towards another point.
     * It begins in a black region, and keeps going until it finds white, then black, then white again.
     * It reports the distance from the start to this point.</p>
     *
     * <p>This is used when figuring out how wide a finder pattern is, when the finder pattern
     * may be skewed or rotated.</p>
     */
    fn sizeOfBlackWhiteBlackRun(&self, fromX: u32, fromY: u32, toX: u32, toY: u32) -> f32 {
        let mut fromX = fromX;
        let mut fromY = fromY;
        let mut toX = toX;
        let mut toY = toY;
        // Mild variant of Bresenham's algorithm;
        // see http://en.wikipedia.org/wiki/Bresenham's_line_algorithm
        let steep = (toY as i64 - fromY as i64).abs() > (toX as i64 - fromX as i64).abs();
        if steep {
            std::mem::swap(&mut fromX, &mut fromY);
            std::mem::swap(&mut toX, &mut toY);
        }

        let dx: i32 = (toX as i64 - fromX as i64).abs() as i32;
        let dy: i32 = (toY as i64 - fromY as i64).abs() as i32;
        let mut error = -dx / 2;
        let xstep: i32 = if fromX < toX { 1 } else { -1 };
        let ystep: i32 = if fromY < toY { 1 } else { -1 };

        // In black pixels, looking for white, first or second time.
        let mut state = 0;
        // Loop up until x == toX, but not beyond
        let xLimit = toX as i32 + xstep;

        let mut x: i32 = fromX as i32;
        let mut y: i32 = fromY as i32;
        while x != xLimit {
            // for (int x = fromX, y = fromY; x != xLimit; x += xstep) {
            let realX = if steep { y } else { x };
            let realY = if steep { x } else { y };

            // Does current pixel mean we have moved white to black or vice versa?
            // Scanning black in state 0,2 and white in state 1, so if we find the wrong
            // color, advance to next state or end if we are in state 2 already
            if (state == 1) == self.image.get(realX as u32, realY as u32) {
                if state == 2 {
                    return MathUtils::distance_int(x, y, fromX as i32, fromY as i32);
                }
                state += 1;
            }

            error += dy;
            if error > 0 {
                if y == toY as i32 {
                    break;
                }
                y += ystep;
                error -= dx;
            }

            x += xstep;
        }
        // Found black-white-black; give the benefit of the doubt that the next pixel outside the image
        // is "white" so this last point at (toX+xStep,toY) is the right ending. This is really a
        // small approximation; (toX+xStep,toY+yStep) might be really correct. Ignore this.
        if state == 2 {
            return MathUtils::distance_int(
                toX as i32 + xstep as i32,
                toY as i32,
                fromX as i32,
                fromY as i32,
            );
        }
        // else we didn't find even black-white-black; no estimate is really possible
        return f32::NAN;
    }

    /**
     * <p>Attempts to locate an alignment pattern in a limited region of the image, which is
     * guessed to contain it. This method uses {@link AlignmentPattern}.</p>
     *
     * @param overallEstModuleSize estimated module size so far
     * @param estAlignmentX x coordinate of center of area probably containing alignment pattern
     * @param estAlignmentY y coordinate of above
     * @param allowanceFactor number of pixels in all directions to search from the center
     * @return {@link AlignmentPattern} if found, or null otherwise
     * @throws NotFoundException if an unexpected error occurs during detection
     */
    pub fn findAlignmentInRegion(
        &self,
        overallEstModuleSize: f32,
        estAlignmentX: u32,
        estAlignmentY: u32,
        allowanceFactor: f32,
    ) -> Result<AlignmentPattern, Exceptions> {
        // Look for an alignment pattern (3 modules in size) around where it
        // should be
        let allowance = (allowanceFactor * overallEstModuleSize) as u32;
        let alignmentAreaLeftX = 0.max(estAlignmentX as i32 - allowance as i32) as u32;
        let alignmentAreaRightX = (self.image.getWidth() - 1).min(estAlignmentX + allowance);
        if ((alignmentAreaRightX - alignmentAreaLeftX) as f32) < overallEstModuleSize * 3.0 {
            return Err(Exceptions::NotFoundException("not found".to_owned()));
        }

        let alignmentAreaTopY = 0.max(estAlignmentY as i32 - allowance as i32) as u32;
        let alignmentAreaBottomY = (self.image.getHeight() - 1).min(estAlignmentY + allowance);
        if alignmentAreaBottomY - alignmentAreaTopY < overallEstModuleSize as u32 * 3 {
            return Err(Exceptions::NotFoundException("not found".to_owned()));
        }

        let mut alignmentFinder = AlignmentPatternFinder::new(
            self.image.clone(),
            alignmentAreaLeftX,
            alignmentAreaTopY,
            alignmentAreaRightX - alignmentAreaLeftX,
            alignmentAreaBottomY - alignmentAreaTopY,
            overallEstModuleSize,
            self.resultPointCallback,
        );
        alignmentFinder.find()
    }
}
