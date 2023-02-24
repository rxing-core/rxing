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

use std::fmt;

use crate::{
    common::{
        detector::WhiteRectangleDetector,
        reedsolomon::{self, ReedSolomonDecoder},
        BitMatrix, DefaultGridSampler, GridSampler, Result,
    },
    exceptions::Exceptions,
    point, Point,
};

use super::aztec_detector_result::AztecDetectorRXingResult;

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
pub struct Detector<'a> {
    image: &'a BitMatrix,

    compact: bool,
    nb_layers: u32,
    nb_data_blocks: u32,
    nb_center_layers: u32,
    shift: u32,
}

impl<'a> Detector<'_> {
    pub fn new(image: &'a BitMatrix) -> Detector<'a> {
        Detector {
            image,
            compact: false,
            nb_layers: 0,
            nb_data_blocks: 0,
            nb_center_layers: 0,
            shift: 0,
        }
    }

    pub fn detect_false(&mut self) -> Result<AztecDetectorRXingResult> {
        self.detect(false)
    }

    /**
     * Detects an Aztec Code in an image.
     *
     * @param isMirror if true, image is a mirror-image of original
     * @return {@link AztecDetectorRXingResult} encapsulating results of detecting an Aztec Code
     * @throws NotFoundException if no Aztec Code can be found
     */
    pub fn detect(&mut self, is_mirror: bool) -> Result<AztecDetectorRXingResult> {
        // dbg!(self.image.to_string());
        // 1. Get the center of the aztec matrix
        let p_center = self.get_matrix_center();

        // 2. Get the center points of the four diagonal points just outside the bull's eye
        //  [topRight, bottomRight, bottomLeft, topLeft]
        let mut bulls_eye_corners = self.get_bulls_eye_corners(p_center)?;

        if is_mirror {
            bulls_eye_corners.swap(0, 2);
        }

        // 3. Get the size of the matrix and other parameters from the bull's eye
        self.extractParameters(&bulls_eye_corners)?;

        // 4. Sample the grid
        let bits = self.sample_grid(
            self.image,
            bulls_eye_corners[self.shift as usize % 4],
            bulls_eye_corners[(self.shift as usize + 1) % 4],
            bulls_eye_corners[(self.shift as usize + 2) % 4],
            bulls_eye_corners[(self.shift as usize + 3) % 4],
        )?;

        // 5. Get the corners of the matrix.
        let corners = self.get_matrix_corner_points(&bulls_eye_corners);

        Ok(AztecDetectorRXingResult::new(
            bits,
            corners,
            self.compact,
            self.nb_data_blocks,
            self.nb_layers,
        ))
    }

    /**
     * Extracts the number of data layers and data blocks from the layer around the bull's eye.
     *
     * @param bullsEyeCorners the array of bull's eye corners
     * @throws NotFoundException in case of too many errors or invalid parameters
     */
    fn extractParameters(&mut self, bulls_eye_corners: &[Point]) -> Result<()> {
        if !self.is_valid(bulls_eye_corners[0])
            || !self.is_valid(bulls_eye_corners[1])
            || !self.is_valid(bulls_eye_corners[2])
            || !self.is_valid(bulls_eye_corners[3])
        {
            return Err(Exceptions::notFoundWith("no valid points"));
        }
        let length = 2 * self.nb_center_layers;
        // Get the bits around the bull's eye
        let sides = [
            self.sample_line(bulls_eye_corners[0], bulls_eye_corners[1], length), // Right side
            self.sample_line(bulls_eye_corners[1], bulls_eye_corners[2], length), // Bottom
            self.sample_line(bulls_eye_corners[2], bulls_eye_corners[3], length), // Left side
            self.sample_line(bulls_eye_corners[3], bulls_eye_corners[0], length), // Top
        ];

        // bullsEyeCorners[shift] is the corner of the bulls'eye that has three
        // orientation marks.
        // sides[shift] is the row/column that goes from the corner with three
        // orientation marks to the corner with two.
        self.shift = Self::get_rotation(&sides, length)?;

        // Flatten the parameter bits into a single 28- or 40-bit long
        let mut parameter_data: u64 = 0;
        for i in 0..4 {
            // for (int i = 0; i < 4; i++) {
            let side = sides[(self.shift + i) as usize % 4];
            if self.compact {
                // Each side of the form ..XXXXXXX. where Xs are parameter data
                parameter_data <<= 7;
                parameter_data += (side as u64 >> 1) & 0x7F;
            } else {
                // Each side of the form ..XXXXX.XXXXX. where Xs are parameter data
                parameter_data <<= 10;
                parameter_data += ((side as u64 >> 2) & (0x1f << 5)) + ((side as u64 >> 1) & 0x1F);
            }
        }

        // Corrects parameter data using RS.  Returns just the data portion
        // without the error correction.
        let corrected_data = Self::get_corrected_parameter_data(parameter_data, self.compact)?;

        if self.compact {
            // 8 bits:  2 bits layers and 6 bits data blocks
            self.nb_layers = (corrected_data >> 6) + 1;
            self.nb_data_blocks = (corrected_data & 0x3F) + 1;
        } else {
            // 16 bits:  5 bits layers and 11 bits data blocks
            self.nb_layers = (corrected_data >> 11) + 1;
            self.nb_data_blocks = (corrected_data & 0x7FF) + 1;
        }

        Ok(())
    }

    fn get_rotation(sides: &[u32], length: u32) -> Result<u32> {
        // In a normal pattern, we expect to See
        //   **    .*             D       A
        //   *      *
        //
        //   .      *
        //   ..    ..             C       B
        //
        // Grab the 3 bits from each of the sides the form the locator pattern and concatenate
        // into a 12-bit integer.  Start with the bit at A
        let mut corner_bits = 0;
        for side in sides {
            // for (int side : sides) {
            // XX......X where X's are orientation marks
            let t = ((side >> (length - 2)) << 1) + (side & 1);
            corner_bits = (corner_bits << 3) + t;
        }
        // Mov the bottom bit to the top, so that the three bits of the locator pattern at A are
        // together.  cornerBits is now:
        //  3 orientation bits at A || 3 orientation bits at B || ... || 3 orientation bits at D
        corner_bits = ((corner_bits & 1) << 11) + (corner_bits >> 1);
        // The result shift indicates which element of BullsEyeCorners[] goes into the top-left
        // corner. Since the four rotation values have a Hamming distance of 8, we
        // can easily tolerate two errors.
        for shift in 0..4 {
            // for (int shift = 0; shift < 4; shift++) {
            if (corner_bits ^ EXPECTED_CORNER_BITS[shift as usize]).count_ones() <= 2 {
                // if (Integer.bitCount(cornerBits ^ EXPECTED_CORNER_BITS[shift]) <= 2) {
                return Ok(shift);
            }
        }
        Err(Exceptions::notFoundWith("rotation failure"))
    }

    /**
     * Corrects the parameter bits using Reed-Solomon algorithm.
     *
     * @param parameterData parameter bits
     * @param compact true if this is a compact Aztec code
     * @throws NotFoundException if the array contains too many errors
     */
    fn get_corrected_parameter_data(parameterData: u64, compact: bool) -> Result<u32> {
        let mut parameter_data = parameterData;

        let num_codewords: i32;
        let num_data_codewords: i32;

        if compact {
            num_codewords = 7;
            num_data_codewords = 2;
        } else {
            num_codewords = 10;
            num_data_codewords = 4;
        }

        let num_eccodewords = num_codewords - num_data_codewords;
        let mut parameterWords = vec![0; num_codewords as usize];
        for i in (0..num_codewords).rev() {
            // for (int i = numCodewords - 1; i >= 0; --i) {
            parameterWords[i as usize] = (parameter_data & 0xF) as i32;
            parameter_data >>= 4;
        }
        //try {
        let field =
            reedsolomon::get_predefined_genericgf(reedsolomon::PredefinedGenericGF::AztecParam);
        let rs_decoder = ReedSolomonDecoder::new(field);
        rs_decoder.decode(&mut parameterWords, num_eccodewords)?;
        //} catch (ReedSolomonException ignored) {
        //throw NotFoundException.getNotFoundInstance();
        //}
        // Toss the error correction.  Just return the data as an integer
        let mut result: u32 = 0;
        for i in 0..num_data_codewords {
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
    fn get_bulls_eye_corners(&mut self, pCenter: AztecPoint) -> Result<[Point; 4]> {
        let mut pina = pCenter;
        let mut pinb = pCenter;
        let mut pinc = pCenter;
        let mut pind = pCenter;

        let mut color = true;

        self.nb_center_layers = 1;

        while self.nb_center_layers < 9 {
            // for nbCenterLayers in 1..9 {
            // for (nbCenterLayers = 1; nbCenterLayers < 9; nbCenterLayers++) {
            let pouta = self.get_first_different(&pina, color, 1, -1);
            let poutb = self.get_first_different(&pinb, color, 1, 1);
            let poutc = self.get_first_different(&pinc, color, -1, 1);
            let poutd = self.get_first_different(&pind, color, -1, -1);

            //d      a
            //
            //c      b

            if self.nb_center_layers > 2 {
                let q: f32 = Self::distance_points(poutd, pouta) * self.nb_center_layers as f32
                    / (Self::distance_points(pind, pina) * (self.nb_center_layers + 2) as f32);

                // let q: f32 = Self::distance(
                //     &poutd.to_rxing_result_point(),
                //     &pouta.to_rxing_result_point(),
                // ) * nbCenterLayers as f32
                //     / (Self::distance(
                //         &pind.to_rxing_result_point(),
                //         &pina.to_rxing_result_point(),
                //     ) * (nbCenterLayers + 2) as f32);
                if !(0.75..=1.25).contains(&q)
                    || !self.is_white_or_black_rectangle(&pouta, &poutb, &poutc, &poutd)
                {
                    break;
                }
            }

            pina = pouta;
            pinb = poutb;
            pinc = poutc;
            pind = poutd;

            color = !color;

            self.nb_center_layers += 1;
        }

        if self.nb_center_layers != 5 && self.nb_center_layers != 7 {
            return Err(Exceptions::notFound);
        }

        self.compact = self.nb_center_layers == 5;

        // Expand the square by .5 pixel in each direction so that we're on the border
        // between the white square and the black square
        let pinax = point(pina.get_x() as f32 + 0.5, pina.get_y() as f32 - 0.5);
        let pinbx = point(pinb.get_x() as f32 + 0.5, pinb.get_y() as f32 + 0.5);
        let pincx = point(pinc.get_x() as f32 - 0.5, pinc.get_y() as f32 + 0.5);
        let pindx = point(pind.get_x() as f32 - 0.5, pind.get_y() as f32 - 0.5);

        // Expand the square so that its corners are the centers of the points
        // just outside the bull's eye.
        Ok(Self::expand_square(
            &[pinax, pinbx, pincx, pindx],
            2 * self.nb_center_layers - 3,
            2 * self.nb_center_layers,
        ))
    }

    /**
     * Finds a candidate center point of an Aztec code from an image
     *
     * @return the center point
     */
    fn get_matrix_center(&self) -> AztecPoint {
        let mut point_a = Point::default();
        let mut point_b = Point::default();
        let mut point_c = Point::default();
        let mut point_d = Point::default();

        let mut fnd = false;

        //Get a white rectangle that can be the border of the matrix in center bull's eye or
        if let Ok(wrd) = WhiteRectangleDetector::new_from_image(self.image) {
            if let Ok(cornerPoints) = wrd.detect() {
                point_a = cornerPoints[0];
                point_b = cornerPoints[1];
                point_c = cornerPoints[2];
                point_d = cornerPoints[3];
                fnd = true;
            }
        }

        // This exception can be in case the initial rectangle is white
        // In that case, surely in the bull's eye, we try to expand the rectangle.
        if !fnd {
            let cx: i32 = (self.image.getWidth() / 2) as i32;
            let cy: i32 = (self.image.getHeight() / 2) as i32;
            point_a = self
                .get_first_different(&AztecPoint::new(cx + 7, cy - 7), false, 1, -1)
                .into();
            point_b = self
                .get_first_different(&AztecPoint::new(cx + 7, cy + 7), false, 1, 1)
                .into();
            point_c = self
                .get_first_different(&AztecPoint::new(cx - 7, cy + 7), false, -1, 1)
                .into();
            point_d = self
                .get_first_different(&AztecPoint::new(cx - 7, cy - 7), false, -1, -1)
                .into();
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
        //   pointA = getFirstDifferent(new Point(cx + 7, cy - 7), false, 1, -1).toPoint();
        //   pointB = getFirstDifferent(new Point(cx + 7, cy + 7), false, 1, 1).toPoint();
        //   pointC = getFirstDifferent(new Point(cx - 7, cy + 7), false, -1, 1).toPoint();
        //   pointD = getFirstDifferent(new Point(cx - 7, cy - 7), false, -1, -1).toPoint();

        // }

        //Compute the center of the rectangle
        let mut cx = ((point_a.x + point_d.x + point_b.x + point_c.x) / 4.0).round() as i32;
        let mut cy = ((point_a.y + point_d.y + point_b.y + point_c.y) / 4.0).round() as i32;

        // Redetermine the white rectangle starting from previously computed center.
        // This will ensure that we end up with a white rectangle in center bull's eye
        // in order to compute a more accurate center.
        let mut fnd = false;
        if let Ok(wrd) = WhiteRectangleDetector::new(self.image, 15, cx, cy) {
            if let Ok(cornerPoints) = wrd.detect() {
                point_a = cornerPoints[0];
                point_b = cornerPoints[1];
                point_c = cornerPoints[2];
                point_d = cornerPoints[3];
                fnd = true;
            }
        }
        // This exception can be in case the initial rectangle is white
        // In that case we try to expand the rectangle.
        if !fnd {
            point_a = self
                .get_first_different(&AztecPoint::new(cx + 7, cy - 7), false, 1, -1)
                .into();
            point_b = self
                .get_first_different(&AztecPoint::new(cx + 7, cy + 7), false, 1, 1)
                .into();
            point_c = self
                .get_first_different(&AztecPoint::new(cx - 7, cy + 7), false, -1, 1)
                .into();
            point_d = self
                .get_first_different(&AztecPoint::new(cx - 7, cy - 7), false, -1, -1)
                .into();
        }
        // try {
        //   Point[] cornerPoints = new WhiteRectangleDetector(image, 15, cx, cy).detect();
        //   pointA = cornerPoints[0];
        //   pointB = cornerPoints[1];
        //   pointC = cornerPoints[2];
        //   pointD = cornerPoints[3];
        // } catch (NotFoundException e) {
        //   // This exception can be in case the initial rectangle is white
        //   // In that case we try to expand the rectangle.
        //   pointA = getFirstDifferent(new Point(cx + 7, cy - 7), false, 1, -1).toPoint();
        //   pointB = getFirstDifferent(new Point(cx + 7, cy + 7), false, 1, 1).toPoint();
        //   pointC = getFirstDifferent(new Point(cx - 7, cy + 7), false, -1, 1).toPoint();
        //   pointD = getFirstDifferent(new Point(cx - 7, cy - 7), false, -1, -1).toPoint();
        // }

        // Recompute the center of the rectangle
        cx = ((point_a.x + point_d.x + point_b.x + point_c.x) / 4.0).round() as i32;
        cy = ((point_a.y + point_d.y + point_b.y + point_c.y) / 4.0).round() as i32;

        AztecPoint::new(cx, cy)
    }

    /**
     * Gets the Aztec code corners from the bull's eye corners and the parameters.
     *
     * @param bullsEyeCorners the array of bull's eye corners
     * @return the array of aztec code corners
     */
    fn get_matrix_corner_points(&self, bulls_eye_corners: &[Point]) -> [Point; 4] {
        Self::expand_square(
            bulls_eye_corners,
            2 * self.nb_center_layers,
            self.get_dimension(),
        )
    }

    /**
     * Creates a BitMatrix by sampling the provided image.
     * topLeft, topRight, bottomRight, and bottomLeft are the centers of the squares on the
     * diagonal just outside the bull's eye.
     */
    fn sample_grid(
        &self,
        image: &BitMatrix,
        top_left: Point,
        top_right: Point,
        bottom_right: Point,
        bottom_left: Point,
    ) -> Result<BitMatrix> {
        let sampler = DefaultGridSampler::default();
        let dimension = self.get_dimension();

        let low = dimension as f32 / 2.0 - self.nb_center_layers as f32;
        let high = dimension as f32 / 2.0 + self.nb_center_layers as f32;

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
            top_left.x,
            top_left.y,
            top_right.x,
            top_right.y,
            bottom_right.x,
            bottom_right.y,
            bottom_left.x,
            bottom_left.y,
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
    fn sample_line(&self, p1: Point, p2: Point, size: u32) -> u32 {
        let mut result = 0;

        let d = Self::distance(p1, p2);
        let module_size = d / size as f32;
        let px = p1.x;
        let py = p1.y;
        let dx = module_size * (p2.x - p1.x) / d;
        let dy = module_size * (p2.y - p1.y) / d;
        for i in 0..size {
            // for (int i = 0; i < size; i++) {
            if self.image.get(
                (px + i as f32 * dx).round() as u32,
                (py + i as f32 * dy).round() as u32,
            ) {
                result |= 1 << (size - i - 1);
            }
        }
        result
    }

    /**
     * @return true if the border of the rectangle passed in parameter is compound of white points only
     *         or black points only
     */
    fn is_white_or_black_rectangle(
        &self,
        p1: &AztecPoint,
        p2: &AztecPoint,
        p3: &AztecPoint,
        p4: &AztecPoint,
    ) -> bool {
        let corr = 3;

        let p1 = AztecPoint::new(
            0.max(p1.get_x() - corr),
            (self.image.getHeight() as i32 - 1).min(p1.get_y() + corr),
        );
        // let p1 =  point(Math.max(0, p1.getX() - corr), Math.min(image.getHeight() - 1, p1.getY() + corr));
        let p2 = AztecPoint::new(0.max(p2.get_x() - corr), 0.max(p2.get_y() - corr));
        // let p2 =  point(Math.max(0, p2.getX() - corr), Math.max(0, p2.getY() - corr));
        let p3 = AztecPoint::new(
            (self.image.getWidth() as i32 - 1).min(p3.get_x() + corr),
            0.max((self.image.getHeight() as i32 - 1).min(p3.get_y() - corr)),
        );
        //  let p3 =  point(Math.min(image.getWidth() - 1, p3.getX() + corr),
        //  Math.max(0, Math.min(image.getHeight() - 1, p3.getY() - corr)));
        let p4 = AztecPoint::new(
            (self.image.getWidth() as i32 - 1).min(p4.get_x() + corr),
            (self.image.getHeight() as i32 - 1).min(p4.get_y() + corr),
        );
        //  let p4 =  point(Math.min(image.getWidth() - 1, p4.getX() + corr),
        //  Math.min(image.getHeight() - 1, p4.getY() + corr));

        let c_init = self.get_color(p4, p1);

        if c_init == 0 {
            return false;
        }

        let c = self.get_color(p1, p2);

        if c != c_init {
            return false;
        }

        let c = self.get_color(p2, p3);

        if c != c_init {
            return false;
        }

        let c = self.get_color(p3, p4);

        c == c_init
    }

    /**
     * Gets the color of a segment
     *
     * @return 1 if segment more than 90% black, -1 if segment is more than 90% white, 0 else
     */
    fn get_color(&self, p1: AztecPoint, p2: AztecPoint) -> i32 {
        let d = Self::distance_points(p1, p2);
        if d == 0.0 {
            return 0;
        }
        let dx = (p2.get_x() - p1.get_x()) as f32 / d;
        let dy = (p2.get_y() - p1.get_y()) as f32 / d;
        let mut error = 0;

        let mut px = p1.get_x() as f32;
        let mut py = p1.get_y() as f32;

        let color_model = self.image.get(p1.get_x() as u32, p1.get_y() as u32);

        let i_max = d.floor() as u32; //(int) Math.floor(d);
        for _i in 0..i_max {
            // for (int i = 0; i < iMax; i++) {

            if self.image.get(px.round() as u32, py.round() as u32) != color_model {
                error += 1;
            }
            px += dx;
            py += dy;
        }

        let err_ratio = error as f32 / d;

        if err_ratio > 0.1 && err_ratio < 0.9 {
            return 0;
        }

        if (err_ratio <= 0.1) == color_model {
            1
        } else {
            -1
        }
    }

    /**
     * Gets the coordinate of the first point with a different color in the given direction
     */
    fn get_first_different(&self, init: &AztecPoint, color: bool, dx: i32, dy: i32) -> AztecPoint {
        let mut x = init.get_x() + dx;
        let mut y = init.get_y() + dy;

        while self.is_valid_points(x, y) && self.image.get(x as u32, y as u32) == color {
            x += dx;
            y += dy;
        }

        x -= dx;
        y -= dy;

        while self.is_valid_points(x, y) && self.image.get(x as u32, y as u32) == color {
            x += dx;
        }
        x -= dx;

        while self.is_valid_points(x, y) && self.image.get(x as u32, y as u32) == color {
            y += dy;
        }
        y -= dy;

        AztecPoint::new(x, y)
    }

    /**
     * Expand the square represented by the corner points by pushing out equally in all directions
     *
     * @param cornerPoints the corners of the square, which has the bull's eye at its center
     * @param oldSide the original length of the side of the square in the target bit matrix
     * @param newSide the new length of the size of the square in the target bit matrix
     * @return the corners of the expanded square
     */
    fn expand_square(corner_points: &[Point], old_side: u32, new_side: u32) -> [Point; 4] {
        let ratio = new_side as f32 / (2.0 * old_side as f32);

        let d = corner_points[0] - corner_points[2];
        let middle = corner_points[0].middle(corner_points[2]);
        let result0 = middle + ratio * d;
        let result2 = middle - ratio * d;

        let d = corner_points[1] - corner_points[3];
        let middle = corner_points[1].middle(corner_points[3]);
        let result1 = middle + ratio * d;
        let result3 = middle - ratio * d;

        [result0, result1, result2, result3]
    }

    fn is_valid_points(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.image.getWidth() as i32 && y >= 0 && y < self.image.getHeight() as i32
    }

    fn is_valid(&self, point: Point) -> bool {
        let x = point.x.round() as i32;
        let y = point.y.round() as i32;
        self.is_valid_points(x, y)
    }

    fn distance_points(a: AztecPoint, b: AztecPoint) -> f32 {
        Point::from(a).distance(b.into())
    }

    fn distance(a: Point, b: Point) -> f32 {
        a.distance(b)
    }

    fn get_dimension(&self) -> u32 {
        if self.compact {
            4 * self.nb_layers + 11
        } else {
            4 * self.nb_layers + 2 * ((2 * self.nb_layers + 6) / 15) + 15
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AztecPoint {
    x: i32,
    y: i32,
}

impl AztecPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl From<AztecPoint> for Point {
    fn from(value: AztecPoint) -> Self {
        point(value.x as f32, value.y as f32)
    }
}

impl fmt::Display for AztecPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} {}>", &self.x, &self.y)
    }
}
