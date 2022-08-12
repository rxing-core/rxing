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
// package com::google::zxing::qrcode::detector;

/**
 * <p>Encapsulates logic that can detect a QR Code in an image, even if the QR Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 */
pub struct Detector {

     let image: BitMatrix;

     let result_point_callback: ResultPointCallback;
}

impl Detector {

    pub fn new( image: &BitMatrix) -> Detector {
        let .image = image;
    }

    pub fn  get_image(&self) -> BitMatrix  {
        return self.image;
    }

    pub fn  get_result_point_callback(&self) -> ResultPointCallback  {
        return self.result_point_callback;
    }

    /**
   * <p>Detects a QR Code in an image.</p>
   *
   * @return {@link DetectorResult} encapsulating results of detecting a QR Code
   * @throws NotFoundException if QR Code cannot be found
   * @throws FormatException if a QR Code cannot be decoded
   */
    pub fn  detect(&self) -> /*  throws NotFoundException, FormatException */Result<DetectorResult, Rc<Exception>>   {
        return Ok(self.detect(null));
    }

    /**
   * <p>Detects a QR Code in an image.</p>
   *
   * @param hints optional hints to detector
   * @return {@link DetectorResult} encapsulating results of detecting a QR Code
   * @throws NotFoundException if QR Code cannot be found
   * @throws FormatException if a QR Code cannot be decoded
   */
    pub fn  detect(&self,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException */Result<DetectorResult, Rc<Exception>>   {
        self.result_point_callback =  if hints == null { null } else { hints.get(DecodeHintType::NEED_RESULT_POINT_CALLBACK) as ResultPointCallback };
         let finder: FinderPatternFinder = FinderPatternFinder::new(self.image, self.result_point_callback);
         let info: FinderPatternInfo = finder.find(&hints);
        return Ok(self.process_finder_pattern_info(info));
    }

    pub fn  process_finder_pattern_info(&self,  info: &FinderPatternInfo) -> /*  throws NotFoundException, FormatException */Result<DetectorResult, Rc<Exception>>   {
         let top_left: FinderPattern = info.get_top_left();
         let top_right: FinderPattern = info.get_top_right();
         let bottom_left: FinderPattern = info.get_bottom_left();
         let module_size: f32 = self.calculate_module_size(top_left, top_right, bottom_left);
        if module_size < 1.0f {
            throw NotFoundException::get_not_found_instance();
        }
         let dimension: i32 = ::compute_dimension(top_left, top_right, bottom_left, module_size);
         let provisional_version: Version = Version::get_provisional_version_for_dimension(dimension);
         let modules_between_f_p_centers: i32 = provisional_version.get_dimension_for_version() - 7;
         let alignment_pattern: AlignmentPattern = null;
        // Anything above version 1 has an alignment pattern
        if provisional_version.get_alignment_pattern_centers().len() > 0 {
            // Guess where a "bottom right" finder pattern would have been
             let bottom_right_x: f32 = top_right.get_x() - top_left.get_x() + bottom_left.get_x();
             let bottom_right_y: f32 = top_right.get_y() - top_left.get_y() + bottom_left.get_y();
            // Estimate that alignment pattern is closer by 3 modules
            // from "bottom right" to known top left location
             let correction_to_top_left: f32 = 1.0f - 3.0f / modules_between_f_p_centers;
             let est_alignment_x: i32 = (top_left.get_x() + correction_to_top_left * (bottom_right_x - top_left.get_x())) as i32;
             let est_alignment_y: i32 = (top_left.get_y() + correction_to_top_left * (bottom_right_y - top_left.get_y())) as i32;
            // Kind of arbitrary -- expand search radius before giving up
             {
                 let mut i: i32 = 4;
                while i <= 16 {
                    {
                        let tryResult1 = 0;
                        'try1: loop {
                        {
                            alignment_pattern = self.find_alignment_in_region(module_size, est_alignment_x, est_alignment_y, i);
                            break;
                        }
                        break 'try1
                        }
                        match tryResult1 {
                             catch ( re: &NotFoundException) {
                            }  0 => break
                        }

                    }
                    i <<= 1;
                 }
             }

        // If we didn't find alignment pattern... well try anyway without it
        }
         let transform: PerspectiveTransform = ::create_transform(top_left, top_right, bottom_left, alignment_pattern, dimension);
         let bits: BitMatrix = ::sample_grid(self.image, transform, dimension);
         let mut points: Vec<ResultPoint>;
        if alignment_pattern == null {
            points =  : vec![ResultPoint; 3] = vec![bottom_left, top_left, top_right, ]
            ;
        } else {
            points =  : vec![ResultPoint; 4] = vec![bottom_left, top_left, top_right, alignment_pattern, ]
            ;
        }
        return Ok(DetectorResult::new(bits, points));
    }

    fn  create_transform( top_left: &ResultPoint,  top_right: &ResultPoint,  bottom_left: &ResultPoint,  alignment_pattern: &ResultPoint,  dimension: i32) -> PerspectiveTransform  {
         let dim_minus_three: f32 = dimension - 3.5f;
         let bottom_right_x: f32;
         let bottom_right_y: f32;
         let source_bottom_right_x: f32;
         let source_bottom_right_y: f32;
        if alignment_pattern != null {
            bottom_right_x = alignment_pattern.get_x();
            bottom_right_y = alignment_pattern.get_y();
            source_bottom_right_x = dim_minus_three - 3.0f;
            source_bottom_right_y = source_bottom_right_x;
        } else {
            // Don't have an alignment pattern, just make up the bottom-right point
            bottom_right_x = (top_right.get_x() - top_left.get_x()) + bottom_left.get_x();
            bottom_right_y = (top_right.get_y() - top_left.get_y()) + bottom_left.get_y();
            source_bottom_right_x = dim_minus_three;
            source_bottom_right_y = dim_minus_three;
        }
        return PerspectiveTransform::quadrilateral_to_quadrilateral(3.5f, 3.5f, dim_minus_three, 3.5f, source_bottom_right_x, source_bottom_right_y, 3.5f, dim_minus_three, &top_left.get_x(), &top_left.get_y(), &top_right.get_x(), &top_right.get_y(), bottom_right_x, bottom_right_y, &bottom_left.get_x(), &bottom_left.get_y());
    }

    fn  sample_grid( image: &BitMatrix,  transform: &PerspectiveTransform,  dimension: i32) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
         let sampler: GridSampler = GridSampler::get_instance();
        return Ok(sampler.sample_grid(image, dimension, dimension, transform));
    }

    /**
   * <p>Computes the dimension (number of modules on a size) of the QR Code based on the position
   * of the finder patterns and estimated module size.</p>
   */
    fn  compute_dimension( top_left: &ResultPoint,  top_right: &ResultPoint,  bottom_left: &ResultPoint,  module_size: f32) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let tltr_centers_dimension: i32 = MathUtils::round(ResultPoint::distance(top_left, top_right) / module_size);
         let tlbl_centers_dimension: i32 = MathUtils::round(ResultPoint::distance(top_left, bottom_left) / module_size);
         let mut dimension: i32 = ((tltr_centers_dimension + tlbl_centers_dimension) / 2) + 7;
        match // mod 4
        dimension & 0x03 {
              0 => 
                 {
                    dimension += 1;
                    break;
                }
            // 1? do nothing
              2 => 
                 {
                    dimension -= 1;
                    break;
                }
              3 => 
                 {
                    throw NotFoundException::get_not_found_instance();
                }
        }
        return Ok(dimension);
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
    pub fn  calculate_module_size(&self,  top_left: &ResultPoint,  top_right: &ResultPoint,  bottom_left: &ResultPoint) -> f32  {
        // Take the average
        return (self.calculate_module_size_one_way(top_left, top_right) + self.calculate_module_size_one_way(top_left, bottom_left)) / 2.0f;
    }

    /**
   * <p>Estimates module size based on two finder patterns -- it uses
   * {@link #sizeOfBlackWhiteBlackRunBothWays(int, int, int, int)} to figure the
   * width of each, measuring along the axis between their centers.</p>
   */
    fn  calculate_module_size_one_way(&self,  pattern: &ResultPoint,  other_pattern: &ResultPoint) -> f32  {
         let module_size_est1: f32 = self.size_of_black_white_black_run_both_ways(pattern.get_x() as i32, pattern.get_y() as i32, other_pattern.get_x() as i32, other_pattern.get_y() as i32);
         let module_size_est2: f32 = self.size_of_black_white_black_run_both_ways(other_pattern.get_x() as i32, other_pattern.get_y() as i32, pattern.get_x() as i32, pattern.get_y() as i32);
        if Float::is_na_n(module_size_est1) {
            return module_size_est2 / 7.0f;
        }
        if Float::is_na_n(module_size_est2) {
            return module_size_est1 / 7.0f;
        }
        // and 1 white and 1 black module on either side. Ergo, divide sum by 14.
        return (module_size_est1 + module_size_est2) / 14.0f;
    }

    /**
   * See {@link #sizeOfBlackWhiteBlackRun(int, int, int, int)}; computes the total width of
   * a finder pattern by looking for a black-white-black run from the center in the direction
   * of another point (another finder pattern center), and in the opposite direction too.
   */
    fn  size_of_black_white_black_run_both_ways(&self,  from_x: i32,  from_y: i32,  to_x: i32,  to_y: i32) -> f32  {
         let mut result: f32 = self.size_of_black_white_black_run(from_x, from_y, to_x, to_y);
        // Now count other way -- don't run off image though of course
         let mut scale: f32 = 1.0f;
         let other_to_x: i32 = from_x - (to_x - from_x);
        if other_to_x < 0 {
            scale = from_x / (from_x - other_to_x) as f32;
            other_to_x = 0;
        } else if other_to_x >= self.image.get_width() {
            scale = (self.image.get_width() - 1.0 - from_x) / (other_to_x - from_x) as f32;
            other_to_x = self.image.get_width() - 1;
        }
         let other_to_y: i32 = (from_y - (to_y - from_y) * scale) as i32;
        scale = 1.0f;
        if other_to_y < 0 {
            scale = from_y / (from_y - other_to_y) as f32;
            other_to_y = 0;
        } else if other_to_y >= self.image.get_height() {
            scale = (self.image.get_height() - 1.0 - from_y) / (other_to_y - from_y) as f32;
            other_to_y = self.image.get_height() - 1;
        }
        other_to_x = (from_x + (other_to_x - from_x) * scale) as i32;
        result += self.size_of_black_white_black_run(from_x, from_y, other_to_x, other_to_y);
        // Middle pixel is double-counted this way; subtract 1
        return result - 1.0f;
    }

    /**
   * <p>This method traces a line from a point in the image, in the direction towards another point.
   * It begins in a black region, and keeps going until it finds white, then black, then white again.
   * It reports the distance from the start to this point.</p>
   *
   * <p>This is used when figuring out how wide a finder pattern is, when the finder pattern
   * may be skewed or rotated.</p>
   */
    fn  size_of_black_white_black_run(&self,  from_x: i32,  from_y: i32,  to_x: i32,  to_y: i32) -> f32  {
        // Mild variant of Bresenham's algorithm;
        // see http://en.wikipedia.org/wiki/Bresenham's_line_algorithm
         let steep: bool = Math::abs(to_y - from_y) > Math::abs(to_x - from_x);
        if steep {
             let mut temp: i32 = from_x;
            from_x = from_y;
            from_y = temp;
            temp = to_x;
            to_x = to_y;
            to_y = temp;
        }
         let dx: i32 = Math::abs(to_x - from_x);
         let dy: i32 = Math::abs(to_y - from_y);
         let mut error: i32 = -dx / 2;
         let xstep: i32 =  if from_x < to_x { 1 } else { -1 };
         let ystep: i32 =  if from_y < to_y { 1 } else { -1 };
        // In black pixels, looking for white, first or second time.
         let mut state: i32 = 0;
        // Loop up until x == toX, but not beyond
         let x_limit: i32 = to_x + xstep;
         {
             let mut x: i32 = from_x, let mut y: i32 = from_y;
            while x != x_limit {
                {
                     let real_x: i32 =  if steep { y } else { x };
                     let real_y: i32 =  if steep { x } else { y };
                    // color, advance to next state or end if we are in state 2 already
                    if (state == 1) == self.image.get(real_x, real_y) {
                        if state == 2 {
                            return MathUtils::distance(x, y, from_x, from_y);
                        }
                        state += 1;
                    }
                    error += dy;
                    if error > 0 {
                        if y == to_y {
                            break;
                        }
                        y += ystep;
                        error -= dx;
                    }
                }
                x += xstep;
             }
         }

        // small approximation; (toX+xStep,toY+yStep) might be really correct. Ignore this.
        if state == 2 {
            return MathUtils::distance(to_x + xstep, to_y, from_x, from_y);
        }
        // else we didn't find even black-white-black; no estimate is really possible
        return Float::NaN;
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
    pub fn  find_alignment_in_region(&self,  overall_est_module_size: f32,  est_alignment_x: i32,  est_alignment_y: i32,  allowance_factor: f32) -> /*  throws NotFoundException */Result<AlignmentPattern, Rc<Exception>>   {
        // Look for an alignment pattern (3 modules in size) around where it
        // should be
         let allowance: i32 = (allowance_factor * overall_est_module_size) as i32;
         let alignment_area_left_x: i32 = Math::max(0, est_alignment_x - allowance);
         let alignment_area_right_x: i32 = Math::min(self.image.get_width() - 1, est_alignment_x + allowance);
        if alignment_area_right_x - alignment_area_left_x < overall_est_module_size * 3.0 {
            throw NotFoundException::get_not_found_instance();
        }
         let alignment_area_top_y: i32 = Math::max(0, est_alignment_y - allowance);
         let alignment_area_bottom_y: i32 = Math::min(self.image.get_height() - 1, est_alignment_y + allowance);
        if alignment_area_bottom_y - alignment_area_top_y < overall_est_module_size * 3.0 {
            throw NotFoundException::get_not_found_instance();
        }
         let alignment_finder: AlignmentPatternFinder = AlignmentPatternFinder::new(self.image, alignment_area_left_x, alignment_area_top_y, alignment_area_right_x - alignment_area_left_x, alignment_area_bottom_y - alignment_area_top_y, overall_est_module_size, self.result_point_callback);
        return Ok(alignment_finder.find());
    }
}

