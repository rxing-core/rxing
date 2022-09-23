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

// package com.google.zxing.aztec.detector;

// import com.google.zxing.NotFoundException;
// import com.google.zxing.RXingResultPoint;
// import com.google.zxing.aztec.AztecDetectorRXingResult;
// import com.google.zxing.common.BitMatrix;
// import com.google.zxing.common.GridSampler;
// import com.google.zxing.common.detector.MathUtils;
// import com.google.zxing.common.detector.WhiteRectangleDetector;
// import com.google.zxing.common.reedsolomon.GenericGF;
// import com.google.zxing.common.reedsolomon.ReedSolomonDecoder;
// import com.google.zxing.common.reedsolomon.ReedSolomonException;

use std::fmt;

use crate::{
    common::{
        detector::{MathUtils, WhiteRectangleDetector},
        reedsolomon::{self, GenericGF, ReedSolomonDecoder},
        BitMatrix, DefaultGridSampler, GridSampler,
    },
    exceptions::Exceptions,
    RXingResultPoint,
};

use super::AztecDetectorResult::AztecDetectorRXingResult;

const EXPECTED_CORNER_BITS: [u32; 4] = [
    0xee0, // 07340  XXX .XX X.. ...
    0x1dc, // 00734  ... XXX .XX X..
    0x83b, // 04073  X.. ... XXX .XX
    0x707, // 03407 .XX X.. ... XXX
];

/**
 * Encapsulates logic that can detect an Aztec Code in an image, even if the Aztec Code
 * is rotated or skewed, or partially obscured.
 *
 * @author David Olivier
 * @author Frank Yellin
 */
pub struct Detector {
    image: BitMatrix,

    compact: bool,
    nbLayers: u32,
    nbDataBlocks: u32,
    nbCenterLayers: u32,
    shift: u32,
}

impl Detector {
    pub fn new(image: BitMatrix) -> Self {
        Self {
            image,
            compact: false,
            nbLayers: 0,
            nbDataBlocks: 0,
            nbCenterLayers: 0,
            shift: 0,
        }
    }

    pub fn detect_false(&mut self) -> Result<AztecDetectorRXingResult, Exceptions> {
        self.detect(false)
    }

    /**
     * Detects an Aztec Code in an image.
     *
     * @param isMirror if true, image is a mirror-image of original
     * @return {@link AztecDetectorRXingResult} encapsulating results of detecting an Aztec Code
     * @throws NotFoundException if no Aztec Code can be found
     */
    pub fn detect(&mut self, is_mirror: bool) -> Result<AztecDetectorRXingResult, Exceptions> {
        // 1. Get the center of the aztec matrix
        let pCenter = self.getMatrixCenter();

        // 2. Get the center points of the four diagonal points just outside the bull's eye
        //  [topRight, bottomRight, bottomLeft, topLeft]
        let mut bullsEyeCorners = self.getBullsEyeCorners(pCenter)?;

        if is_mirror {
            let temp = bullsEyeCorners[0];
            bullsEyeCorners[0] = bullsEyeCorners[2];
            bullsEyeCorners[2] = temp;
        }

        // 3. Get the size of the matrix and other parameters from the bull's eye
        self.extractParameters(&bullsEyeCorners);

        // 4. Sample the grid
        let bits = self.sampleGrid(
            &self.image,
            &bullsEyeCorners[self.shift as usize % 4],
            &bullsEyeCorners[(self.shift as usize + 1) % 4],
            &bullsEyeCorners[(self.shift as usize + 2) % 4],
            &bullsEyeCorners[(self.shift as usize + 3) % 4],
        )?;

        // 5. Get the corners of the matrix.
        let corners = self.getMatrixCornerPoints(&bullsEyeCorners);

        Ok(AztecDetectorRXingResult::new(
            bits,
            corners,
            self.compact,
            self.nbDataBlocks,
            self.nbLayers,
        ))
    }

    /**
     * Extracts the number of data layers and data blocks from the layer around the bull's eye.
     *
     * @param bullsEyeCorners the array of bull's eye corners
     * @throws NotFoundException in case of too many errors or invalid parameters
     */
    fn extractParameters(
        &mut self,
        bullsEyeCorners: &[RXingResultPoint],
    ) -> Result<(), Exceptions> {
        if !self.isValid(&bullsEyeCorners[0])
            || !self.isValid(&bullsEyeCorners[1])
            || !self.isValid(&bullsEyeCorners[2])
            || !self.isValid(&bullsEyeCorners[3])
        {
            return Err(Exceptions::NotFoundException("no valid points".to_owned()));
        }
        let length = 2 * self.nbCenterLayers;
        // Get the bits around the bull's eye
        let sides = [
            self.sampleLine(&bullsEyeCorners[0], &bullsEyeCorners[1], length), // Right side
            self.sampleLine(&bullsEyeCorners[1], &bullsEyeCorners[2], length), // Bottom
            self.sampleLine(&bullsEyeCorners[2], &bullsEyeCorners[3], length), // Left side
            self.sampleLine(&bullsEyeCorners[3], &bullsEyeCorners[0], length), // Top
        ];

        // bullsEyeCorners[shift] is the corner of the bulls'eye that has three
        // orientation marks.
        // sides[shift] is the row/column that goes from the corner with three
        // orientation marks to the corner with two.
        self.shift = Self::getRotation(&sides, length)?;

        // Flatten the parameter bits into a single 28- or 40-bit long
        let mut parameterData = 0u64;
        for i in 0..4 {
            // for (int i = 0; i < 4; i++) {
            let side = sides[(self.shift + i) as usize % 4];
            if self.compact {
                // Each side of the form ..XXXXXXX. where Xs are parameter data
                parameterData <<= 7;
                parameterData += (side as u64 >> 1) & 0x7F;
            } else {
                // Each side of the form ..XXXXX.XXXXX. where Xs are parameter data
                parameterData <<= 10;
                parameterData += ((side as u64 >> 2) & (0x1f << 5)) + ((side as u64 >> 1) & 0x1F);
            }
        }

        // Corrects parameter data using RS.  Returns just the data portion
        // without the error correction.
        let correctedData = Self::getCorrectedParameterData(parameterData, self.compact)?;

        if self.compact {
            // 8 bits:  2 bits layers and 6 bits data blocks
            self.nbLayers = (correctedData >> 6) + 1;
            self.nbDataBlocks = (correctedData & 0x3F) + 1;
        } else {
            // 16 bits:  5 bits layers and 11 bits data blocks
            self.nbLayers = (correctedData >> 11) + 1;
            self.nbDataBlocks = (correctedData & 0x7FF) + 1;
        }

        Ok(())
    }

    fn getRotation(sides: &[u32], length: u32) -> Result<u32, Exceptions> {
        // In a normal pattern, we expect to See
        //   **    .*             D       A
        //   *      *
        //
        //   .      *
        //   ..    ..             C       B
        //
        // Grab the 3 bits from each of the sides the form the locator pattern and concatenate
        // into a 12-bit integer.  Start with the bit at A
        let mut cornerBits = 0;
        for side in sides {
            // for (int side : sides) {
            // XX......X where X's are orientation marks
            let t = ((side >> (length - 2)) << 1) + (side & 1);
            cornerBits = (cornerBits << 3) + t;
        }
        // Mov the bottom bit to the top, so that the three bits of the locator pattern at A are
        // together.  cornerBits is now:
        //  3 orientation bits at A || 3 orientation bits at B || ... || 3 orientation bits at D
        cornerBits = ((cornerBits & 1) << 11) + (cornerBits >> 1);
        // The result shift indicates which element of BullsEyeCorners[] goes into the top-left
        // corner. Since the four rotation values have a Hamming distance of 8, we
        // can easily tolerate two errors.
        for shift in 0..4 {
            // for (int shift = 0; shift < 4; shift++) {
            if (cornerBits ^ EXPECTED_CORNER_BITS[shift as usize]).count_ones() <= 2 {
                // if (Integer.bitCount(cornerBits ^ EXPECTED_CORNER_BITS[shift]) <= 2) {
                return Ok(shift);
            }
        }
        Err(Exceptions::NotFoundException("rotation failure".to_owned()))
    }

    /**
     * Corrects the parameter bits using Reed-Solomon algorithm.
     *
     * @param parameterData parameter bits
     * @param compact true if this is a compact Aztec code
     * @throws NotFoundException if the array contains too many errors
     */
    fn getCorrectedParameterData(parameterData: u64, compact: bool) -> Result<u32, Exceptions> {
        let mut parameterData = parameterData;

        let numCodewords: i32;
        let numDataCodewords: i32;

        if compact {
            numCodewords = 7;
            numDataCodewords = 2;
        } else {
            numCodewords = 10;
            numDataCodewords = 4;
        }

        let numECCodewords = numCodewords - numDataCodewords;
        let mut parameterWords = vec![0i32; numCodewords as usize];
        for i in (0..numCodewords - 1).rev() {
            // for (int i = numCodewords - 1; i >= 0; --i) {
            parameterWords[i as usize] = (parameterData & 0xF) as i32;
            parameterData >>= 4;
        }
        //try {
        let field =
            reedsolomon::get_predefined_genericgf(reedsolomon::PredefinedGenericGF::AztecParam);
        let rsDecoder = ReedSolomonDecoder::new(field);
        rsDecoder.decode(&mut parameterWords, numECCodewords)?;
        //} catch (ReedSolomonException ignored) {
        //throw NotFoundException.getNotFoundInstance();
        //}
        // Toss the error correction.  Just return the data as an integer
        let mut result = 0u32;
        for i in 0..numDataCodewords {
            // for (int i = 0; i < numDataCodewords; i++) {
            result = (result << 4) + parameterWords[i as usize] as u32;
        }
        Ok(result)
    }

    /**
     * Finds the corners of a bull-eye centered on the passed point.
     * This returns the centers of the diagonal points just outside the bull's eye
     * Returns [topRight, bottomRight, bottomLeft, topLeft]
     *
     * @param pCenter Center point
     * @return The corners of the bull-eye
     * @throws NotFoundException If no valid bull-eye can be found
     */
    fn getBullsEyeCorners(&mut self, pCenter: Point) -> Result<Vec<RXingResultPoint>, Exceptions> {
        let mut pina = pCenter;
        let mut pinb = pCenter;
        let mut pinc = pCenter;
        let mut pind = pCenter;

        let mut color = true;

        for nbCenterLayers in 1..9 {
            // for (nbCenterLayers = 1; nbCenterLayers < 9; nbCenterLayers++) {
            let pouta = self.getFirstDifferent(&pina, color, 1, -1);
            let poutb = self.getFirstDifferent(&pinb, color, 1, 1);
            let poutc = self.getFirstDifferent(&pinc, color, -1, 1);
            let poutd = self.getFirstDifferent(&pind, color, -1, -1);

            //d      a
            //
            //c      b

            if nbCenterLayers > 2 {
                let q: f32 =
                    Self::distance(&poutd.toRXingResultPoint(), &pouta.toRXingResultPoint())
                        * nbCenterLayers as f32
                        / (Self::distance(&pind.toRXingResultPoint(), &pina.toRXingResultPoint())
                            * (nbCenterLayers + 2) as f32);
                if q < 0.75
                    || q > 1.25
                    || !self.isWhiteOrBlackRectangle(&pouta, &poutb, &poutc, &poutd)
                {
                    break;
                }
            }

            pina = pouta;
            pinb = poutb;
            pinc = poutc;
            pind = poutd;

            color = !color;
        }

        if self.nbCenterLayers != 5 && self.nbCenterLayers != 7 {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        self.compact = self.nbCenterLayers == 5;

        // Expand the square by .5 pixel in each direction so that we're on the border
        // between the white square and the black square
        let pinax = RXingResultPoint::new(pina.getX() as f32 + 0.5f32, pina.getY() as f32 - 0.5f32);
        let pinbx = RXingResultPoint::new(pinb.getX() as f32 + 0.5f32, pinb.getY() as f32 + 0.5f32);
        let pincx = RXingResultPoint::new(pinc.getX() as f32 - 0.5f32, pinc.getY() as f32 + 0.5f32);
        let pindx = RXingResultPoint::new(pind.getX() as f32 - 0.5f32, pind.getY() as f32 - 0.5f32);

        // Expand the square so that its corners are the centers of the points
        // just outside the bull's eye.
        Ok(Self::expandSquare(
            &[pinax, pinbx, pincx, pindx],
            2 * self.nbCenterLayers - 3,
            2 * self.nbCenterLayers,
        ))
    }

    /**
     * Finds a candidate center point of an Aztec code from an image
     *
     * @return the center point
     */
    fn getMatrixCenter(&self) -> Point {
        let mut pointA = RXingResultPoint { x: 0.0, y: 0.0 };
        let mut pointB = RXingResultPoint { x: 0.0, y: 0.0 };
        let mut pointC = RXingResultPoint { x: 0.0, y: 0.0 };
        let mut pointD = RXingResultPoint { x: 0.0, y: 0.0 };

        let mut fnd = false;

        //Get a white rectangle that can be the border of the matrix in center bull's eye or
        if let Ok(wrd) = WhiteRectangleDetector::new_from_image(&self.image) {
            if let Ok(cornerPoints) = wrd.detect() {
                pointA = cornerPoints[0];
                pointB = cornerPoints[1];
                pointC = cornerPoints[2];
                pointD = cornerPoints[3];
                fnd = true;
            }
        }

        // This exception can be in case the initial rectangle is white
        // In that case, surely in the bull's eye, we try to expand the rectangle.
        if !fnd {
            let cx: i32 = (self.image.getWidth() / 2).try_into().unwrap();
            let cy: i32 = (self.image.getHeight() / 2).try_into().unwrap();
            pointA = self
                .getFirstDifferent(&Point::new(cx + 7, cy - 7), false, 1, -1)
                .toRXingResultPoint();
            pointB = self
                .getFirstDifferent(&Point::new(cx + 7, cy + 7), false, 1, 1)
                .toRXingResultPoint();
            pointC = self
                .getFirstDifferent(&Point::new(cx - 7, cy + 7), false, -1, 1)
                .toRXingResultPoint();
            pointD = self
                .getFirstDifferent(&Point::new(cx - 7, cy - 7), false, -1, -1)
                .toRXingResultPoint();
        }
        // try {

        //   let cornerPoints =  WhiteRectangleDetector::new(image).detect();
        //   pointA = cornerPoints[0];
        //   pointB = cornerPoints[1];
        //   pointC = cornerPoints[2];
        //   pointD = cornerPoints[3];

        // } catch (NotFoundException e) {

        //   // This exception can be in case the initial rectangle is white
        //   // In that case, surely in the bull's eye, we try to expand the rectangle.
        //   int cx = image.getWidth() / 2;
        //   int cy = image.getHeight() / 2;
        //   pointA = getFirstDifferent(new Point(cx + 7, cy - 7), false, 1, -1).toRXingResultPoint();
        //   pointB = getFirstDifferent(new Point(cx + 7, cy + 7), false, 1, 1).toRXingResultPoint();
        //   pointC = getFirstDifferent(new Point(cx - 7, cy + 7), false, -1, 1).toRXingResultPoint();
        //   pointD = getFirstDifferent(new Point(cx - 7, cy - 7), false, -1, -1).toRXingResultPoint();

        // }

        //Compute the center of the rectangle
        let mut cx = MathUtils::round(
            (pointA.getX() + pointD.getX() + pointB.getX() + pointC.getX()) / 4.0f32,
        );
        let mut cy = MathUtils::round(
            (pointA.getY() + pointD.getY() + pointB.getY() + pointC.getY()) / 4.0f32,
        );

        // Redetermine the white rectangle starting from previously computed center.
        // This will ensure that we end up with a white rectangle in center bull's eye
        // in order to compute a more accurate center.
        let mut fnd = false;
        if let Ok(wrd) = WhiteRectangleDetector::new(&self.image, 15, cx, cy) {
            if let Ok(cornerPoints) = wrd.detect() {
                pointA = cornerPoints[0];
                pointB = cornerPoints[1];
                pointC = cornerPoints[2];
                pointD = cornerPoints[3];
                fnd = true;
            }
        }
        // This exception can be in case the initial rectangle is white
        // In that case we try to expand the rectangle.
        if !fnd {
            pointA = self
                .getFirstDifferent(&Point::new(cx + 7, cy - 7), false, 1, -1)
                .toRXingResultPoint();
            pointB = self
                .getFirstDifferent(&Point::new(cx + 7, cy + 7), false, 1, 1)
                .toRXingResultPoint();
            pointC = self
                .getFirstDifferent(&Point::new(cx - 7, cy + 7), false, -1, 1)
                .toRXingResultPoint();
            pointD = self
                .getFirstDifferent(&Point::new(cx - 7, cy - 7), false, -1, -1)
                .toRXingResultPoint();
        }
        // try {
        //   RXingResultPoint[] cornerPoints = new WhiteRectangleDetector(image, 15, cx, cy).detect();
        //   pointA = cornerPoints[0];
        //   pointB = cornerPoints[1];
        //   pointC = cornerPoints[2];
        //   pointD = cornerPoints[3];
        // } catch (NotFoundException e) {
        //   // This exception can be in case the initial rectangle is white
        //   // In that case we try to expand the rectangle.
        //   pointA = getFirstDifferent(new Point(cx + 7, cy - 7), false, 1, -1).toRXingResultPoint();
        //   pointB = getFirstDifferent(new Point(cx + 7, cy + 7), false, 1, 1).toRXingResultPoint();
        //   pointC = getFirstDifferent(new Point(cx - 7, cy + 7), false, -1, 1).toRXingResultPoint();
        //   pointD = getFirstDifferent(new Point(cx - 7, cy - 7), false, -1, -1).toRXingResultPoint();
        // }

        // Recompute the center of the rectangle
        cx = MathUtils::round(
            (pointA.getX() + pointD.getX() + pointB.getX() + pointC.getX()) / 4.0f32,
        );
        cy = MathUtils::round(
            (pointA.getY() + pointD.getY() + pointB.getY() + pointC.getY()) / 4.0f32,
        );

        Point::new(cx, cy)
    }

    /**
     * Gets the Aztec code corners from the bull's eye corners and the parameters.
     *
     * @param bullsEyeCorners the array of bull's eye corners
     * @return the array of aztec code corners
     */
    fn getMatrixCornerPoints(&self, bullsEyeCorners: &[RXingResultPoint]) -> Vec<RXingResultPoint> {
        Self::expandSquare(
            bullsEyeCorners,
            2 * self.nbCenterLayers,
            self.getDimension(),
        )
    }

    /**
     * Creates a BitMatrix by sampling the provided image.
     * topLeft, topRight, bottomRight, and bottomLeft are the centers of the squares on the
     * diagonal just outside the bull's eye.
     */
    fn sampleGrid(
        &self,
        image: &BitMatrix,
        topLeft: &RXingResultPoint,
        topRight: &RXingResultPoint,
        bottomRight: &RXingResultPoint,
        bottomLeft: &RXingResultPoint,
    ) -> Result<BitMatrix, Exceptions> {
        let sampler = DefaultGridSampler {};
        let dimension = self.getDimension();

        let low = dimension as f32 / 2.0f32 - self.nbCenterLayers as f32;
        let high = dimension as f32 / 2.0f32 + self.nbCenterLayers as f32;

        sampler.sample_grid_detailed(
            image,
            dimension,
            dimension,
            low,
            low, // topleft
            high,
            low, // topright
            high,
            high, // bottomright
            low,
            high, // bottomleft
            topLeft.getX(),
            topLeft.getY(),
            topRight.getX(),
            topRight.getY(),
            bottomRight.getX(),
            bottomRight.getY(),
            bottomLeft.getX(),
            bottomLeft.getY(),
        )
    }

    /**
     * Samples a line.
     *
     * @param p1   start point (inclusive)
     * @param p2   end point (exclusive)
     * @param size number of bits
     * @return the array of bits as an int (first bit is high-order bit of result)
     */
    fn sampleLine(&self, p1: &RXingResultPoint, p2: &RXingResultPoint, size: u32) -> u32 {
        let mut result = 0;

        let d = Self::distance(p1, p2);
        let moduleSize = d / size as f32;
        let px = p1.getX();
        let py = p1.getY();
        let dx = moduleSize * (p2.getX() - p1.getX()) / d;
        let dy = moduleSize * (p2.getY() - p1.getY()) / d;
        for i in 0..size {
            // for (int i = 0; i < size; i++) {
            if self.image.get(
                MathUtils::round(px + i as f32 * dx) as u32,
                MathUtils::round(py + i as f32 * dy) as u32,
            ) {
                result |= 1 << (size - i - 1);
            }
        }
        return result;
    }

    /**
     * @return true if the border of the rectangle passed in parameter is compound of white points only
     *         or black points only
     */
    fn isWhiteOrBlackRectangle(&self, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
        let corr = 3;

        let p1 = Point::new(
            0.max(p1.getX() - corr),
            (self.image.getHeight() as i32 - 1).min(p1.getY() + corr),
        );
        // let p1 =  Point::new(Math.max(0, p1.getX() - corr), Math.min(image.getHeight() - 1, p1.getY() + corr));
        let p2 = Point::new(0.max(p2.getX() - corr), 0.max(p2.getY() - corr));
        // let p2 =  Point::new(Math.max(0, p2.getX() - corr), Math.max(0, p2.getY() - corr));
        let p3 = Point::new(
            (self.image.getWidth() as i32 - 1).min(p3.getX() + corr),
            0.max((self.image.getHeight() as i32 - 1).min(p3.getY() - corr)),
        );
        //  let p3 =  Point::new(Math.min(image.getWidth() - 1, p3.getX() + corr),
        //  Math.max(0, Math.min(image.getHeight() - 1, p3.getY() - corr)));
        let p4 = Point::new(
            self.image.getWidth() as i32 - 1.min(p4.getX() + corr),
            (self.image.getHeight() as i32 - 1).min(p4.getY() + corr),
        );
        //  let p4 =  Point::new(Math.min(image.getWidth() - 1, p4.getX() + corr),
        //  Math.min(image.getHeight() - 1, p4.getY() + corr));

        let cInit = self.getColor(&p4, &p1);

        if cInit == 0 {
            return false;
        }

        let c = self.getColor(&p1, &p2);

        if c != cInit {
            return false;
        }

        let c = self.getColor(&p2, &p3);

        if c != cInit {
            return false;
        }

        let c = self.getColor(&p3, &p4);

        return c == cInit;
    }

    /**
     * Gets the color of a segment
     *
     * @return 1 if segment more than 90% black, -1 if segment is more than 90% white, 0 else
     */
    fn getColor(&self, p1: &Point, p2: &Point) -> i32 {
        let d = Self::distance_points(p1, p2);
        if d == 0.0f32 {
            return 0;
        }
        let dx = (p2.getX() - p1.getX()) as f32 / d;
        let dy = (p2.getY() - p1.getY()) as f32 / d;
        let mut error = 0;

        let mut px = p1.getX();
        let mut py = p1.getY();

        let colorModel = self.image.get(p1.getX() as u32, p1.getY() as u32);

        let iMax = d.floor() as u32; //(int) Math.floor(d);
        for _i in 0..iMax {
            // for (int i = 0; i < iMax; i++) {
            if self.image.get(px as u32, py as u32) != colorModel {
                error += 1;
            }
            px += dx.floor() as i32;
            py += dy.floor() as i32;
        }

        let errRatio = error as f32 / d;

        if errRatio > 0.1f32 && errRatio < 0.9f32 {
            return 0;
        }

        if (errRatio <= 0.1f32) == colorModel {
            1
        } else {
            -1
        }
    }

    /**
     * Gets the coordinate of the first point with a different color in the given direction
     */
    fn getFirstDifferent(&self, init: &Point, color: bool, dx: i32, dy: i32) -> Point {
        let mut x = init.getX() + dx;
        let mut y = init.getY() + dy;

        while self.isValidPoints(x, y) && self.image.get(x as u32, y as u32) == color {
            x += dx;
            y += dy;
        }

        x -= dx;
        y -= dy;

        while self.isValidPoints(x, y) && self.image.get(x as u32, y as u32) == color {
            x += dx;
        }
        x -= dx;

        while self.isValidPoints(x, y) && self.image.get(x as u32, y as u32) == color {
            y += dy;
        }
        y -= dy;

        Point::new(x, y)
    }

    /**
     * Expand the square represented by the corner points by pushing out equally in all directions
     *
     * @param cornerPoints the corners of the square, which has the bull's eye at its center
     * @param oldSide the original length of the side of the square in the target bit matrix
     * @param newSide the new length of the size of the square in the target bit matrix
     * @return the corners of the expanded square
     */
    fn expandSquare(
        cornerPoints: &[RXingResultPoint],
        oldSide: u32,
        newSide: u32,
    ) -> Vec<RXingResultPoint> {
        let ratio = newSide as f32 / (2.0f32 * oldSide as f32);
        let mut dx = cornerPoints[0].getX() - cornerPoints[2].getX();
        let mut dy = cornerPoints[0].getY() - cornerPoints[2].getY();
        let mut centerx = (cornerPoints[0].getX() + cornerPoints[2].getX()) / 2.0f32;
        let mut centery = (cornerPoints[0].getY() + cornerPoints[2].getY()) / 2.0f32;

        let result0 = RXingResultPoint::new(centerx + ratio * dx, centery + ratio * dy);
        let result2 = RXingResultPoint::new(centerx - ratio * dx, centery - ratio * dy);

        dx = cornerPoints[1].getX() - cornerPoints[3].getX();
        dy = cornerPoints[1].getY() - cornerPoints[3].getY();
        centerx = (cornerPoints[1].getX() + cornerPoints[3].getX()) / 2.0f32;
        centery = (cornerPoints[1].getY() + cornerPoints[3].getY()) / 2.0f32;
        let result1 = RXingResultPoint::new(centerx + ratio * dx, centery + ratio * dy);
        let result3 = RXingResultPoint::new(centerx - ratio * dx, centery - ratio * dy);

        vec![result0, result1, result2, result3]
    }

    fn isValidPoints(&self, x: i32, y: i32) -> bool {
        x >= 0
            && x < self.image.getWidth().try_into().unwrap()
            && y >= 0
            && y < self.image.getHeight().try_into().unwrap()
    }

    fn isValid(&self, point: &RXingResultPoint) -> bool {
        let x = MathUtils::round(point.getX());
        let y = MathUtils::round(point.getY());
        self.isValidPoints(x, y)
    }

    fn distance_points(a: &Point, b: &Point) -> f32 {
        MathUtils::distance_int(a.getX(), a.getY(), b.getX(), b.getY())
    }

    fn distance(a: &RXingResultPoint, b: &RXingResultPoint) -> f32 {
        MathUtils::distance_float(a.getX(), a.getY(), b.getX(), b.getY())
    }

    fn getDimension(&self) -> u32 {
        if self.compact {
            return 4 * self.nbLayers + 11;
        }
        4 * self.nbLayers + 2 * ((2 * self.nbLayers + 6) / 15) + 15
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn toRXingResultPoint(&self) -> RXingResultPoint {
        RXingResultPoint::new(self.x as f32, self.y as f32)
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn getX(&self) -> i32 {
        self.x
    }

    pub fn getY(&self) -> i32 {
        self.y
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} {}>", &self.x, &self.y)
    }
}
