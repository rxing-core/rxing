use crate::{NotFoundException,ResultPoint}; 
use crate::aztec::AztecDetectorResult;
use crate::common::{BitMatrix,GridSampler};
use crate::common::detector::{MathUtils,WhiteRectangleDetector};
use crate::common::reedsolomon::{GenericGF,ReedSolomonDecoder,ReedSolomonException};


/**
 * Encapsulates logic that can detect an Aztec Code in an image, even if the Aztec Code
 * is rotated or skewed, or partially obscured.
 *
 * @author David Olivier
 * @author Frank Yellin
 */

const EXPECTED_CORNER_BITS: vec![Vec<i32>; 4] = vec![// 07340  XXX .XX X.. ...
0xee0, // 00734  ... XXX .XX X..
0x1dc, // 04073  X.. ... XXX .XX
0x83b, // 03407 .XX X.. ... XXX
0x707, ]
;
pub struct Detector {

      image: BitMatrix,

       compact: bool,

      nb_layers: i32,

      nb_data_blocks: i32,

      nb_center_layers: i32,

       shift: i32
}

impl Detector {

    pub fn new( image: &BitMatrix) -> Self {
        let new_d : Self;
        new_d.image = image;

        new_d
    }

    /**
   * Detects an Aztec Code in an image.
   *
   * @param isMirror if true, image is a mirror-image of original
   * @return {@link AztecDetectorResult} encapsulating results of detecting an Aztec Code
   * @throws NotFoundException if no Aztec Code can be found
   */
    pub fn  detect(&self,  is_mirror: Option<bool>) -> Result<AztecDetectorResult,NotFoundException>   {
        // 1. Get the center of the aztec matrix
         let p_center: Point = self.get_matrix_center();
        // 2. Get the center points of the four diagonal points just outside the bull's eye
        //  [topRight, bottomRight, bottomLeft, topLeft]
         let bulls_eye_corners: Vec<ResultPoint> = self.get_bulls_eye_corners(&p_center);
        if is_mirror.unwrap_or(false) {
             let temp: ResultPoint = bulls_eye_corners[0];
            bulls_eye_corners[0] = bulls_eye_corners[2];
            bulls_eye_corners[2] = temp;
        }
        // 3. Get the size of the matrix and other parameters from the bull's eye
        self.extract_parameters(&bulls_eye_corners);
        // 4. Sample the grid
         let bits: BitMatrix = self.sample_grid(&self.image, bulls_eye_corners[self.shift % 4], bulls_eye_corners[(self.shift + 1) % 4], bulls_eye_corners[(self.shift + 2) % 4], bulls_eye_corners[(self.shift + 3) % 4]);
        // 5. Get the corners of the matrix.
         let corners: Vec<ResultPoint> = self.get_matrix_corner_points(&bulls_eye_corners);
        return Ok(AztecDetectorResult::new(&bits, &corners, self.compact, self.nb_data_blocks, self.nb_layers));
    }

    /**
   * Extracts the number of data layers and data blocks from the layer around the bull's eye.
   *
   * @param bullsEyeCorners the array of bull's eye corners
   * @throws NotFoundException in case of too many errors or invalid parameters
   */
    fn  extract_parameters(&self,  bulls_eye_corners: &Vec<ResultPoint>)  -> Result<(), NotFoundException>   {
        if !self.is_valid(bulls_eye_corners[0]) || !self.is_valid(bulls_eye_corners[1]) || !self.is_valid(bulls_eye_corners[2]) || !self.is_valid(bulls_eye_corners[3]) {
            return Err( NotFoundException::get_not_found_instance());
        }
         let length: i32 = 2 * self.nb_center_layers;
        // Get the bits around the bull's eye
         let sides: vec![Vec<i32>; 4] = vec![// Right side
        self.sample_line(&bulls_eye_corners[0], &bulls_eye_corners[1], length), // Bottom
        self.sample_line(&bulls_eye_corners[1], &bulls_eye_corners[2], length), // Left side
        self.sample_line(&bulls_eye_corners[2], &bulls_eye_corners[3], length), // Top
        self.sample_line(&bulls_eye_corners[3], &bulls_eye_corners[0], length), ]
        ;
        // bullsEyeCorners[shift] is the corner of the bulls'eye that has three
        // orientation marks.
        // sides[shift] is the row/column that goes from the corner with three
        // orientation marks to the corner with two.
        self.shift = ::get_rotation(&sides, length);
        // Flatten the parameter bits into a single 28- or 40-bit long
         let parameter_data: i64 = 0;
         {
             let mut i: i32 = 0;
            while i < 4 {
                {
                     let side: i32 = sides[(self.shift + i) % 4];
                    if self.compact {
                        // Each side of the form ..XXXXXXX. where Xs are parameter data
                        parameter_data <<= 7;
                        parameter_data += (side >> 1) & 0x7F;
                    } else {
                        // Each side of the form ..XXXXX.XXXXX. where Xs are parameter data
                        parameter_data <<= 10;
                        parameter_data += ((side >> 2) & (0x1f << 5)) + ((side >> 1) & 0x1F);
                    }
                }
                i += 1;
             }
         }

        // Corrects parameter data using RS.  Returns just the data portion
        // without the error correction.
         let corrected_data: i32 = ::get_corrected_parameter_data(parameter_data, self.compact);
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

    fn  get_rotation( sides: &Vec<i32>,  length: i32) -> Result<i32, NotFoundException>   {
        // In a normal pattern, we expect to See
        //   **    .*             D       A
        //   *      *
        //
        //   .      *
        //   ..    ..             C       B
        //
        // Grab the 3 bits from each of the sides the form the locator pattern and concatenate
        // into a 12-bit integer.  Start with the bit at A
         let corner_bits: i32 = 0;
        for   side in sides {
            // XX......X where X's are orientation marks
             let t: i32 = ((side >> (length - 2)) << 1) + (side & 1);
            corner_bits = (corner_bits << 3) + t;
        }
        // Mov the bottom bit to the top, so that the three bits of the locator pattern at A are
        // together.  cornerBits is now:
        //  3 orientation bits at A || 3 orientation bits at B || ... || 3 orientation bits at D
        corner_bits = ((corner_bits & 1) << 11) + (corner_bits >> 1);
        // can easily tolerate two errors.
         {
             let mut shift: i32 = 0;
            while shift < 4 {
                {
                    if Integer::bit_count(corner_bits ^ EXPECTED_CORNER_BITS[shift]) <= 2 {
                        return Ok(shift);
                    }
                }
                shift += 1;
             }
         }

        return Err(NotFoundException::get_not_found_instance());
    }

    /**
   * Corrects the parameter bits using Reed-Solomon algorithm.
   *
   * @param parameterData parameter bits
   * @param compact true if this is a compact Aztec code
   * @throws NotFoundException if the array contains too many errors
   */
    fn  get_corrected_parameter_data( parameter_data: i64,  compact: bool) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let num_codewords: i32;
         let num_data_codewords: i32;
        if compact {
            num_codewords = 7;
            num_data_codewords = 2;
        } else {
            num_codewords = 10;
            num_data_codewords = 4;
        }
         let num_e_c_codewords: i32 = num_codewords - num_data_codewords;
         let parameter_words: [i32; num_codewords] = [0; num_codewords];
         {
             let mut i: i32 = num_codewords - 1;
            while i >= 0 {
                {
                    parameter_words[i] = parameter_data as i32 & 0xF;
                    parameter_data >>= 4;
                }
                i -= 1;
             }
         }

        let tryResult1 = 0;
        /*'try1: loop {
        {*/
             let rs_decoder: ReedSolomonDecoder = ReedSolomonDecoder::new(GenericGF::AZTEC_PARAM);
            rs_decoder.decode(&parameter_words, num_e_c_codewords);
        /*}
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &ReedSolomonException) {
                throw NotFoundException::get_not_found_instance();
            }  0 => break
        }
        */

        // Toss the error correction.  Just return the data as an integer
         let mut result: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < num_data_codewords {
                {
                    result = (result << 4) + parameter_words[i];
                }
                i += 1;
             }
         }

        return Ok(result);
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
    fn  get_bulls_eye_corners(&self,  p_center: &Point) -> /*  throws NotFoundException */Result<Vec<ResultPoint>, Rc<Exception>>   {
         let mut pina: Point = p_center;
         let mut pinb: Point = p_center;
         let mut pinc: Point = p_center;
         let mut pind: Point = p_center;
         let mut color: bool = true;
         {
            self.nb_center_layers = 1;
            while self.nb_center_layers < 9 {
                {
                     let pouta: Point = self.get_first_different(&pina, color, 1, -1);
                     let poutb: Point = self.get_first_different(&pinb, color, 1, 1);
                     let poutc: Point = self.get_first_different(&pinc, color, -1, 1);
                     let poutd: Point = self.get_first_different(&pind, color, -1, -1);
                    if self.nb_center_layers > 2 {
                         let q: f32 = ::distance(poutd, pouta) * self.nb_center_layers / (::distance(pind, pina) * (self.nb_center_layers + 2));
                        if q < 0.75 || q > 1.25 || !self.is_white_or_black_rectangle(&pouta, &poutb, &poutc, &poutd) {
                            break;
                        }
                    }
                    pina = pouta;
                    pinb = poutb;
                    pinc = poutc;
                    pind = poutd;
                    color = !color;
                }
                self.nb_center_layers += 1;
             }
         }

        if self.nb_center_layers != 5 && self.nb_center_layers != 7 {
            return Err( NotFoundException::get_not_found_instance());
        }
        self.compact = self.nb_center_layers == 5;
        // Expand the square by .5 pixel in each direction so that we're on the border
        // between the white square and the black square
         let pinax: ResultPoint = ResultPoint::new(pina.get_x() + 0.5f32, pina.get_y() - 0.5f32);
         let pinbx: ResultPoint = ResultPoint::new(pinb.get_x() + 0.5f32, pinb.get_y() + 0.5f32);
         let pincx: ResultPoint = ResultPoint::new(pinc.get_x() - 0.5f32, pinc.get_y() + 0.5f32);
         let pindx: ResultPoint = ResultPoint::new(pind.get_x() - 0.5f32, pind.get_y() - 0.5f32);
        // just outside the bull's eye.
        return Ok(::expand_square(  vec![pinax, pinbx, pincx, pindx, ]
        , 2 * self.nb_center_layers - 3, 2 * self.nb_center_layers));
    }

    /**
   * Finds a candidate center point of an Aztec code from an image
   *
   * @return the center point
   */
    fn  get_matrix_center(&self) -> Point  {
         let point_a: ResultPoint;
         let point_b: ResultPoint;
         let point_c: ResultPoint;
         let point_d: ResultPoint;
        //Get a white rectangle that can be the border of the matrix in center bull's eye or
        let tryResult1 = 0;
        
        let corner_points_detector = WhiteRectangleDetector::new(&self.image, None, None, None);
        if corner_points_detector.is_ok() {

        let corner_points: Vec<ResultPoint> = corner_points_detector.detect();
        
            point_a = corner_points[0];
            point_b = corner_points[1];
            point_c = corner_points[2];
            point_d = corner_points[3];
        }else {
            let cx: i32 = self.image.get_width() / 2;
            let cy: i32 = self.image.get_height() / 2;
           point_a = self.get_first_different(&Point::new(cx + 7, cy - 7), false, 1, -1).to_result_point();
           point_b = self.get_first_different(&Point::new(cx + 7, cy + 7), false, 1, 1).to_result_point();
           point_c = self.get_first_different(&Point::new(cx - 7, cy + 7), false, -1, 1).to_result_point();
           point_d = self.get_first_different(&Point::new(cx - 7, cy - 7), false, -1, -1).to_result_point();
        }

        //Compute the center of the rectangle
         let mut cx: i32 = MathUtils::round((point_a.get_x() + point_d.get_x() + point_b.get_x() + point_c.get_x()) / 4.0f32);
         let mut cy: i32 = MathUtils::round((point_a.get_y() + point_d.get_y() + point_b.get_y() + point_c.get_y()) / 4.0f32);
        // in order to compute a more accurate center.
        let tryResult1 = 0;

        let corner_points_wrd = WhiteRectangleDetector::new(&self.image, Some(15), Some(cx), Some(cy));
        if corner_points_wrd.is_ok() {
            let corner_points: Vec<ResultPoint> = corner_points_wrd.detect();
            point_a = corner_points[0];
            point_b = corner_points[1];
            point_c = corner_points[2];
            point_d = corner_points[3];
        } else {
            point_a = self.get_first_different(&Point::new(cx + 7, cy - 7), false, 1, -1).to_result_point();
                point_b = self.get_first_different(&Point::new(cx + 7, cy + 7), false, 1, 1).to_result_point();
                point_c = self.get_first_different(&Point::new(cx - 7, cy + 7), false, -1, 1).to_result_point();
                point_d = self.get_first_different(&Point::new(cx - 7, cy - 7), false, -1, -1).to_result_point();
        }

        // Recompute the center of the rectangle
        cx = MathUtils::round((point_a.get_x() + point_d.get_x() + point_b.get_x() + point_c.get_x()) / 4.0f32);
        cy = MathUtils::round((point_a.get_y() + point_d.get_y() + point_b.get_y() + point_c.get_y()) / 4.0f32);
        return Point::new(cx, cy);
    }

    /**
   * Gets the Aztec code corners from the bull's eye corners and the parameters.
   *
   * @param bullsEyeCorners the array of bull's eye corners
   * @return the array of aztec code corners
   */
    fn  get_matrix_corner_points(&self,  bulls_eye_corners: &Vec<ResultPoint>) -> Vec<ResultPoint>  {
        return ::expand_square(bulls_eye_corners, 2 * self.nb_center_layers, &self.get_dimension());
    }

    /**
   * Creates a BitMatrix by sampling the provided image.
   * topLeft, topRight, bottomRight, and bottomLeft are the centers of the squares on the
   * diagonal just outside the bull's eye.
   */
    fn  sample_grid(&self,  image: &BitMatrix,  top_left: &ResultPoint,  top_right: &ResultPoint,  bottom_right: &ResultPoint,  bottom_left: &ResultPoint) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
         let sampler: GridSampler = GridSampler::get_instance();
         let dimension: i32 = self.get_dimension();
         let low: f32 = dimension / 2.0f32 - self.nb_center_layers;
         let high: f32 = dimension / 2.0f32 + self.nb_center_layers;
        return Ok(sampler.sample_grid(image, dimension, dimension, // topleft
        low, // topleft
        low, // topright
        high, // topright
        low, // bottomright
        high, // bottomright
        high, // bottomleft
        low, // bottomleft
        high, &top_left.get_x(), &top_left.get_y(), &top_right.get_x(), &top_right.get_y(), &bottom_right.get_x(), &bottom_right.get_y(), &bottom_left.get_x(), &bottom_left.get_y()));
    }

    /**
   * Samples a line.
   *
   * @param p1   start point (inclusive)
   * @param p2   end point (exclusive)
   * @param size number of bits
   * @return the array of bits as an int (first bit is high-order bit of result)
   */
    fn  sample_line(&self,  p1: &ResultPoint,  p2: &ResultPoint,  size: i32) -> i32  {
         let mut result: i32 = 0;
         let d: f32 = ::distance(p1, p2);
         let module_size: f32 = d / size;
         let px: f32 = p1.get_x();
         let py: f32 = p1.get_y();
         let dx: f32 = module_size * (p2.get_x() - p1.get_x()) / d;
         let dy: f32 = module_size * (p2.get_y() - p1.get_y()) / d;
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    if self.image.get(&MathUtils::round(px + i * dx), &MathUtils::round(py + i * dy)) {
                        result |= 1 << (size - i - 1);
                    }
                }
                i += 1;
             }
         }

        return result;
    }

    /**
   * @return true if the border of the rectangle passed in parameter is compound of white points only
   *         or black points only
   */
    fn  is_white_or_black_rectangle(&self,  p1: &Point,  p2: &Point,  p3: &Point,  p4: &Point) -> bool  {
         let corr: i32 = 3;
        p1 = &Point::new(&Math::max(0, p1.get_x() - corr), &Math::min(self.image.get_height() - 1, p1.get_y() + corr));
        p2 = &Point::new(&Math::max(0, p2.get_x() - corr), &Math::max(0, p2.get_y() - corr));
        p3 = &Point::new(&Math::min(self.image.get_width() - 1, p3.get_x() + corr), &Math::max(0, &Math::min(self.image.get_height() - 1, p3.get_y() - corr)));
        p4 = &Point::new(&Math::min(self.image.get_width() - 1, p4.get_x() + corr), &Math::min(self.image.get_height() - 1, p4.get_y() + corr));
         let c_init: i32 = self.get_color(p4, p1);
        if c_init == 0 {
            return false;
        }
         let mut c: i32 = self.get_color(p1, p2);
        if c != c_init {
            return false;
        }
        c = self.get_color(p2, p3);
        if c != c_init {
            return false;
        }
        c = self.get_color(p3, p4);
        return c == c_init;
    }

    /**
   * Gets the color of a segment
   *
   * @return 1 if segment more than 90% black, -1 if segment is more than 90% white, 0 else
   */
    fn  get_color(&self,  p1: &Point,  p2: &Point) -> i32  {
         let d: f32 = ::distance(p1, p2);
        if d == 0.0f32 {
            return 0;
        }
         let dx: f32 = (p2.get_x() - p1.get_x()) / d;
         let dy: f32 = (p2.get_y() - p1.get_y()) / d;
         let mut error: i32 = 0;
         let mut px: f32 = p1.get_x();
         let mut py: f32 = p1.get_y();
         let color_model: bool = self.image.get(&p1.get_x(), &p1.get_y());
         let i_max: i32 = Math::floor(d) as i32;
         {
             let mut i: i32 = 0;
            while i < i_max {
                {
                    if self.image.get(&MathUtils::round(px), &MathUtils::round(py)) != color_model {
                        error += 1;
                    }
                    px += dx;
                    py += dy;
                }
                i += 1;
             }
         }

         let err_ratio: f32 = error / d;
        if err_ratio > 0.1f32 && err_ratio < 0.9f32 {
            return 0;
        }
        return  if (err_ratio <= 0.1f32) == color_model { 1 } else { -1 };
    }

    /**
   * Gets the coordinate of the first point with a different color in the given direction
   */
    fn  get_first_different(&self,  init: &Point,  color: bool,  dx: i32,  dy: i32) -> Point  {
         let mut x: i32 = init.get_x() + dx;
         let mut y: i32 = init.get_y() + dy;
        while self.is_valid(x, y) && self.image.get(x, y) == color {
            x += dx;
            y += dy;
        }
        x -= dx;
        y -= dy;
        while self.is_valid(x, y) && self.image.get(x, y) == color {
            x += dx;
        }
        x -= dx;
        while self.is_valid(x, y) && self.image.get(x, y) == color {
            y += dy;
        }
        y -= dy;
        return Point::new(x, y);
    }

    /**
   * Expand the square represented by the corner points by pushing out equally in all directions
   *
   * @param cornerPoints the corners of the square, which has the bull's eye at its center
   * @param oldSide the original length of the side of the square in the target bit matrix
   * @param newSide the new length of the size of the square in the target bit matrix
   * @return the corners of the expanded square
   */
    fn  expand_square( corner_points: &Vec<ResultPoint>,  old_side: i32,  new_side: i32) -> Vec<ResultPoint>  {
         let ratio: f32 = new_side / (2.0f32 * old_side);
         let mut dx: f32 = corner_points[0].get_x() - corner_points[2].get_x();
         let mut dy: f32 = corner_points[0].get_y() - corner_points[2].get_y();
         let mut centerx: f32 = (corner_points[0].get_x() + corner_points[2].get_x()) / 2.0f32;
         let mut centery: f32 = (corner_points[0].get_y() + corner_points[2].get_y()) / 2.0f32;
         let result0: ResultPoint = ResultPoint::new(centerx + ratio * dx, centery + ratio * dy);
         let result2: ResultPoint = ResultPoint::new(centerx - ratio * dx, centery - ratio * dy);
        dx = corner_points[1].get_x() - corner_points[3].get_x();
        dy = corner_points[1].get_y() - corner_points[3].get_y();
        centerx = (corner_points[1].get_x() + corner_points[3].get_x()) / 2.0f32;
        centery = (corner_points[1].get_y() + corner_points[3].get_y()) / 2.0f32;
         let result1: ResultPoint = ResultPoint::new(centerx + ratio * dx, centery + ratio * dy);
         let result3: ResultPoint = ResultPoint::new(centerx - ratio * dx, centery - ratio * dy);
        return  vec![result0, result1, result2, result3, ]
        ;
    }

    fn  is_valid_coords(&self,  x: i32,  y: i32) -> bool  {
        return x >= 0 && x < self.image.get_width() && y >= 0 && y < self.image.get_height();
    }

    fn  is_valid_rp(&self,  point: &ResultPoint) -> bool  {
         let x: i32 = MathUtils::round(&point.get_x());
         let y: i32 = MathUtils::round(&point.get_y());
        return self.is_valid(x, y);
    }

    fn  distance( a: &Point,  b: &Point) -> f32  {
        return MathUtils::distance(&a.get_x(), &a.get_y(), &b.get_x(), &b.get_y());
    }

    fn  distance( a: &ResultPoint,  b: &ResultPoint) -> f32  {
        return MathUtils::distance(&a.get_x(), &a.get_y(), &b.get_x(), &b.get_y());
    }

    fn  get_dimension(&self) -> i32  {
        if self.compact {
            return 4 * self.nb_layers + 11;
        }
        return 4 * self.nb_layers + 2 * ((2 * self.nb_layers + 6) / 15) + 15;
    }

   

}

struct Point {

     x: i32,

     y: i32
}

impl Point {

   fn  to_result_point(&self) -> ResultPoint  {
       return ResultPoint::new(self.x, self.y);
   }

   fn new( x: i32,  y: i32) -> Self {
       Self { x: x, y: y }
   }

   fn  get_x(&self) -> i32  {
       return self.x;
   }

   fn  get_y(&self) -> i32  {
       return self.y;
   }

   pub fn  to_string(&self) -> String  {
       return format!("<{} {}>", self.x, self.y);
   }
}