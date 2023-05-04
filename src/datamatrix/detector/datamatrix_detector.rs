/*
 * Copyright 2008 ZXing authors
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

use crate::{
    common::{
        detector::WhiteRectangleDetector, BitMatrix, DefaultGridSampler, GridSampler,
        Quadrilateral, Result,
    },
    point_f, Exceptions, Point,
};

use super::DatamatrixDetectorResult;

/**
 * <p>Encapsulates logic that can detect a Data Matrix Code in an image, even if the Data Matrix Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 */
pub struct Detector<'a> {
    image: &'a BitMatrix,
    rectangleDetector: WhiteRectangleDetector<'a>,
}
impl<'a> Detector<'_> {
    pub fn new(image: &'a BitMatrix) -> Result<Detector<'a>> {
        Ok(Detector {
            rectangleDetector: WhiteRectangleDetector::new_from_image(image)?,
            image,
        })
    }

    /**
     * <p>Detects a Data Matrix Code in an image.</p>
     *
     * @return {@link DetectorRXingResult} encapsulating results of detecting a Data Matrix Code
     * @throws NotFoundException if no Data Matrix Code can be found
     */
    pub fn detect(&self) -> Result<DatamatrixDetectorResult> {
        let cornerPoints = self.rectangleDetector.detect()?;

        let mut points = self.detectSolid1(cornerPoints);
        points = self.detectSolid2(points);
        if let Some(point) = self.correctTopRight(&points) {
            points[3] = point;
        } else {
            return Err(Exceptions::not_found_with("point 4 unfound"));
        }
        // points[3] = self.correctTopRight(&points);
        // if points[3] == null {
        //   throw NotFoundException.getNotFoundInstance();
        // }
        points = self.shiftToModuleCenter(points);

        let topLeft = points[0];
        let bottomLeft = points[1];
        let bottomRight = points[2];
        let topRight = points[3];

        let mut dimensionTop = self.transitionsBetween(topLeft, topRight) + 1;
        let mut dimensionRight = self.transitionsBetween(bottomRight, topRight) + 1;
        if (dimensionTop & 0x01) == 1 {
            dimensionTop += 1;
        }
        if (dimensionRight & 0x01) == 1 {
            dimensionRight += 1;
        }

        if 4 * dimensionTop < 6 * dimensionRight && 4 * dimensionRight < 6 * dimensionTop {
            // The matrix is square
            dimensionTop = dimensionTop.max(dimensionRight);
            dimensionRight = dimensionTop.max(dimensionRight);
        }

        let bits = Self::sampleGrid(
            self.image,
            topLeft,
            bottomLeft,
            bottomRight,
            topRight,
            dimensionTop,
            dimensionRight,
        )?;

        Ok(DatamatrixDetectorResult::new(
            bits,
            vec![topLeft, bottomLeft, bottomRight, topRight],
        ))
    }

    fn shiftPoint(p: Point, to: Point, div: u32) -> Point {
        let x = (to.x - p.x) / (div as f32 + 1.0);
        let y = (to.y - p.y) / (div as f32 + 1.0);
        point_f(p.x + x, p.y + y)
    }

    fn moveAway(p: Point, fromX: f32, fromY: f32) -> Point {
        let mut x = p.x;
        let mut y = p.y;

        if x < fromX {
            x -= 1.0;
        } else {
            x += 1.0;
        }

        if y < fromY {
            y -= 1.0;
        } else {
            y += 1.0;
        }

        point_f(x, y)
    }

    /**
     * Detect a solid side which has minimum transition.
     */
    fn detectSolid1(&self, cornerPoints: [Point; 4]) -> [Point; 4] {
        // 0  2
        // 1  3
        let pointA = cornerPoints[0];
        let pointB = cornerPoints[1];
        let pointC = cornerPoints[3];
        let pointD = cornerPoints[2];

        let trAB = self.transitionsBetween(pointA, pointB);
        let trBC = self.transitionsBetween(pointB, pointC);
        let trCD = self.transitionsBetween(pointC, pointD);
        let trDA = self.transitionsBetween(pointD, pointA);

        // 0..3
        // :  :
        // 1--2
        let mut min = trAB;
        let mut points = [pointD, pointA, pointB, pointC];
        if min > trBC {
            min = trBC;
            points[0] = pointA;
            points[1] = pointB;
            points[2] = pointC;
            points[3] = pointD;
        }
        if min > trCD {
            min = trCD;
            points[0] = pointB;
            points[1] = pointC;
            points[2] = pointD;
            points[3] = pointA;
        }
        if min > trDA {
            points[0] = pointC;
            points[1] = pointD;
            points[2] = pointA;
            points[3] = pointB;
        }

        points
    }

    /**
     * Detect a second solid side next to first solid side.
     */
    fn detectSolid2(&self, points: [Point; 4]) -> [Point; 4] {
        // A..D
        // :  :
        // B--C
        let pointA = points[0];
        let pointB = points[1];
        let pointC = points[2];
        let pointD = points[3];

        // Transition detection on the edge is not stable.
        // To safely detect, shift the points to the module center.
        let tr = self.transitionsBetween(pointA, pointD);
        let pointBs = Self::shiftPoint(pointB, pointC, (tr + 1) * 4);
        let pointCs = Self::shiftPoint(pointC, pointB, (tr + 1) * 4);
        let trBA = self.transitionsBetween(pointBs, pointA);
        let trCD = self.transitionsBetween(pointCs, pointD);

        // 0..3
        // |  :
        // 1--2
        if trBA < trCD {
            // solid sides: A-B-C
            [pointA, pointB, pointC, pointD]
            // points[0] = pointA;
            // points[1] = pointB;
            // points[2] = pointC;
            // points[3] = pointD;
        } else {
            // solid sides: B-C-D
            [pointB, pointC, pointD, pointA]
            // points[0] = pointB;
            // points[1] = pointC;
            // points[2] = pointD;
            // points[3] = pointA;
        }
    }

    /**
     * Calculates the corner position of the white top right module.
     */
    fn correctTopRight(&self, points: &[Point; 4]) -> Option<Point> {
        // A..D
        // |  :
        // B--C
        let pointA = points[0];
        let pointB = points[1];
        let pointC = points[2];
        let pointD = points[3];

        // shift points for safe transition detection.
        let mut trTop = self.transitionsBetween(pointA, pointD);
        let mut trRight = self.transitionsBetween(pointB, pointD);
        let pointAs = Self::shiftPoint(pointA, pointB, (trRight + 1) * 4);
        let pointCs = Self::shiftPoint(pointC, pointB, (trTop + 1) * 4);

        trTop = self.transitionsBetween(pointAs, pointD);
        trRight = self.transitionsBetween(pointCs, pointD);

        let candidate1 = point_f(
            pointD.x + (pointC.x - pointB.x) / (trTop as f32 + 1.0),
            pointD.y + (pointC.y - pointB.y) / (trTop as f32 + 1.0),
        );
        let candidate2 = point_f(
            pointD.x + (pointA.x - pointB.x) / (trRight as f32 + 1.0),
            pointD.y + (pointA.y - pointB.y) / (trRight as f32 + 1.0),
        );

        if !self.isValid(candidate1) {
            if self.isValid(candidate2) {
                return Some(candidate2);
            }
            return None;
        }
        if !self.isValid(candidate2) {
            return Some(candidate1);
        }

        let sumc1 = self.transitionsBetween(pointAs, candidate1)
            + self.transitionsBetween(pointCs, candidate1);
        let sumc2 = self.transitionsBetween(pointAs, candidate2)
            + self.transitionsBetween(pointCs, candidate2);

        if sumc1 > sumc2 {
            Some(candidate1)
        } else {
            Some(candidate2)
        }
    }

    /**
     * Shift the edge points to the module center.
     */
    fn shiftToModuleCenter(&self, points: [Point; 4]) -> [Point; 4] {
        // A..D
        // |  :
        // B--C
        let mut pointA = points[0];
        let mut pointB = points[1];
        let mut pointC = points[2];
        let mut pointD = points[3];

        // calculate pseudo dimensions
        let mut dimH = self.transitionsBetween(pointA, pointD) + 1;
        let mut dimV = self.transitionsBetween(pointC, pointD) + 1;

        // shift points for safe dimension detection
        let mut pointAs = Self::shiftPoint(pointA, pointB, dimV * 4);
        let mut pointCs = Self::shiftPoint(pointC, pointB, dimH * 4);

        //  calculate more precise dimensions
        dimH = self.transitionsBetween(pointAs, pointD) + 1;
        dimV = self.transitionsBetween(pointCs, pointD) + 1;
        if (dimH & 0x01) == 1 {
            dimH += 1;
        }
        if (dimV & 0x01) == 1 {
            dimV += 1;
        }

        // WhiteRectangleDetector returns points inside of the rectangle.
        // I want points on the edges.
        let centerX = (pointA.x + pointB.x + pointC.x + pointD.x) / 4.0;
        let centerY = (pointA.y + pointB.y + pointC.y + pointD.y) / 4.0;
        pointA = Self::moveAway(pointA, centerX, centerY);
        pointB = Self::moveAway(pointB, centerX, centerY);
        pointC = Self::moveAway(pointC, centerX, centerY);
        pointD = Self::moveAway(pointD, centerX, centerY);

        let mut pointBs;
        let mut pointDs;

        // shift points to the center of each modules
        pointAs = Self::shiftPoint(pointA, pointB, dimV * 4);
        pointAs = Self::shiftPoint(pointAs, pointD, dimH * 4);
        pointBs = Self::shiftPoint(pointB, pointA, dimV * 4);
        pointBs = Self::shiftPoint(pointBs, pointC, dimH * 4);
        pointCs = Self::shiftPoint(pointC, pointD, dimV * 4);
        pointCs = Self::shiftPoint(pointCs, pointB, dimH * 4);
        pointDs = Self::shiftPoint(pointD, pointC, dimV * 4);
        pointDs = Self::shiftPoint(pointDs, pointA, dimH * 4);

        [pointAs, pointBs, pointCs, pointDs]
    }

    fn isValid(&self, p: Point) -> bool {
        p.x >= 0.0
            && p.x <= self.image.getWidth() as f32 - 1.0
            && p.y > 0.0
            && p.y <= self.image.getHeight() as f32 - 1.0
    }

    fn sampleGrid(
        image: &BitMatrix,
        topLeft: Point,
        bottomLeft: Point,
        bottomRight: Point,
        topRight: Point,
        dimensionX: u32,
        dimensionY: u32,
    ) -> Result<BitMatrix> {
        let sampler = DefaultGridSampler::default();

        let dst = Quadrilateral::new(
            point_f(0.5, 0.5),
            point_f(dimensionX as f32 - 0.5, 0.5),
            point_f(dimensionX as f32 - 0.5, dimensionY as f32 - 0.5),
            point_f(0.5, dimensionY as f32 - 0.5),
        );
        let src = Quadrilateral::new(topRight, topLeft, bottomRight, bottomLeft);

        let (res, _) = sampler.sample_grid_detailed(image, dimensionX, dimensionY, dst, src)?;
        Ok(res)
    }

    /**
     * Counts the number of black/white transitions between two points, using something like Bresenham's algorithm.
     */
    fn transitionsBetween(&self, from: Point, to: Point) -> u32 {
        // See QR Code Detector, sizeOfBlackWhiteBlackRun()
        let mut fromX = from.x.floor() as i32;
        let mut fromY = from.y.floor() as i32;
        let mut toX = to.x.floor() as i32;
        let mut toY = (self.image.getHeight() - 1).min(to.y.floor() as u32) as i32;

        let steep = (toY - fromY).abs() > (toX - fromX).abs();
        if steep {
            std::mem::swap(&mut fromX, &mut fromY);
            std::mem::swap(&mut toX, &mut toY);
        }

        let dx = (toX - fromX).abs();
        let dy = (toY - fromY).abs();
        let mut error = -dx / 2;
        let ystep = if fromY < toY { 1 } else { -1 };
        let xstep = if fromX < toX { 1 } else { -1 };
        let mut transitions = 0;
        let mut inBlack = self.image.get(
            if steep { fromY as u32 } else { fromX as u32 },
            if steep { fromX as u32 } else { fromY as u32 },
        );
        let mut x = fromX;
        let mut y = fromY;
        while x != toX {
            // for (int x = fromX, y = fromY; x != toX; x += xstep) {
            let isBlack = self.image.get(
                if steep { y as u32 } else { x as u32 },
                if steep { x as u32 } else { y as u32 },
            );
            if isBlack != inBlack {
                transitions += 1;
                inBlack = isBlack;
            }
            error += dy;
            if error > 0 {
                if y == toY {
                    break;
                }
                y += ystep;
                error -= dx;
            }

            x += xstep;
        }
        transitions
    }
}
