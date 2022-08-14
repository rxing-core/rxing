use crate::{ResultPoint,NotFoundException,ResultPointCallback,DecodeHintType,FormatException};
use crate::common::{BitMatrix,DetectorResult,GridSampler,PerspectiveTransform};
use crate::common::detector::{MathUtils};
use crate::qrcode::decoder::Version;


// NEW FILE: alignment_pattern.rs
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
 * <p>Encapsulates an alignment pattern, which are the smaller square patterns found in
 * all but the simplest QR Codes.</p>
 *
 * @author Sean Owen
 */
pub struct AlignmentPattern {
    super: ResultPoint;

     let estimated_module_size: f32;
}

impl AlignmentPattern {

    fn new( pos_x: f32,  pos_y: f32,  estimated_module_size: f32) -> AlignmentPattern {
        super(pos_x, pos_y);
        let .estimatedModuleSize = estimated_module_size;
    }

    /**
   * <p>Determines if this alignment pattern "about equals" an alignment pattern at the stated
   * position and size -- meaning, it is at nearly the same center with nearly the same size.</p>
   */
    fn  about_equals(&self,  module_size: f32,  i: f32,  j: f32) -> bool  {
        if Math::abs(i - get_y()) <= module_size && Math::abs(j - get_x()) <= module_size {
             let module_size_diff: f32 = Math::abs(module_size - self.estimated_module_size);
            return module_size_diff <= 1.0f || module_size_diff <= self.estimated_module_size;
        }
        return false;
    }

    /**
   * Combines this object's current estimate of a finder pattern position and module size
   * with a new estimate. It returns a new {@code FinderPattern} containing an average of the two.
   */
    fn  combine_estimate(&self,  i: f32,  j: f32,  new_module_size: f32) -> AlignmentPattern  {
         let combined_x: f32 = (get_x() + j) / 2.0f;
         let combined_y: f32 = (get_y() + i) / 2.0f;
         let combined_module_size: f32 = (self.estimated_module_size + new_module_size) / 2.0f;
        return AlignmentPattern::new(combined_x, combined_y, combined_module_size);
    }
}

// NEW FILE: alignment_pattern_finder.rs
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
 * <p>This class attempts to find alignment patterns in a QR Code. Alignment patterns look like finder
 * patterns but are smaller and appear at regular intervals throughout the image.</p>
 *
 * <p>At the moment this only looks for the bottom-right alignment pattern.</p>
 *
 * <p>This is mostly a simplified copy of {@link FinderPatternFinder}. It is copied,
 * pasted and stripped down here for maximum performance but does unfortunately duplicate
 * some code.</p>
 *
 * <p>This class is thread-safe but not reentrant. Each thread must allocate its own object.</p>
 *
 * @author Sean Owen
 */
struct AlignmentPatternFinder {

     let image: BitMatrix;

     let possible_centers: List<AlignmentPattern>;

     let start_x: i32;

     let start_y: i32;

     let width: i32;

     let height: i32;

     let module_size: f32;

     let cross_check_state_count: Vec<i32>;

     let result_point_callback: ResultPointCallback;
}

impl AlignmentPatternFinder {

    /**
   * <p>Creates a finder that will look in a portion of the whole image.</p>
   *
   * @param image image to search
   * @param startX left column from which to start searching
   * @param startY top row from which to start searching
   * @param width width of region to search
   * @param height height of region to search
   * @param moduleSize estimated module size so far
   */
    fn new( image: &BitMatrix,  start_x: i32,  start_y: i32,  width: i32,  height: i32,  module_size: f32,  result_point_callback: &ResultPointCallback) -> AlignmentPatternFinder {
        let .image = image;
        let .possibleCenters = ArrayList<>::new(5);
        let .startX = start_x;
        let .startY = start_y;
        let .width = width;
        let .height = height;
        let .moduleSize = module_size;
        let .crossCheckStateCount = : [i32; 3] = [0; 3];
        let .resultPointCallback = result_point_callback;
    }

    /**
   * <p>This method attempts to find the bottom-right alignment pattern in the image. It is a bit messy since
   * it's pretty performance-critical and so is written to be fast foremost.</p>
   *
   * @return {@link AlignmentPattern} if found
   * @throws NotFoundException if not found
   */
    fn  find(&self) -> /*  throws NotFoundException */Result<AlignmentPattern, Rc<Exception>>   {
         let start_x: i32 = self.startX;
         let height: i32 = self.height;
         let max_j: i32 = start_x + self.width;
         let middle_i: i32 = self.start_y + (height / 2);
        // We are looking for black/white/black modules in 1:1:1 ratio;
        // this tracks the number of black/white/black modules seen so far
         let state_count: [i32; 3] = [0; 3];
         {
             let i_gen: i32 = 0;
            while i_gen < height {
                {
                    // Search from middle outwards
                     let i: i32 = middle_i + ( if (i_gen & 0x01) == 0 { (i_gen + 1) / 2 } else { -((i_gen + 1) / 2) });
                    state_count[0] = 0;
                    state_count[1] = 0;
                    state_count[2] = 0;
                     let mut j: i32 = start_x;
                    // white run continued to the left of the start point
                    while j < max_j && !self.image.get(j, i) {
                        j += 1;
                    }
                     let current_state: i32 = 0;
                    while j < max_j {
                        if self.image.get(j, i) {
                            // Black pixel
                            if current_state == 1 {
                                // Counting black pixels
                                state_count[1] += 1;
                            } else {
                                // Counting white pixels
                                if current_state == 2 {
                                    // A winner?
                                    if self.found_pattern_cross(&state_count) {
                                        // Yes
                                         let confirmed: AlignmentPattern = self.handle_possible_center(&state_count, i, j);
                                        if confirmed != null {
                                            return Ok(confirmed);
                                        }
                                    }
                                    state_count[0] = state_count[2];
                                    state_count[1] = 1;
                                    state_count[2] = 0;
                                    current_state = 1;
                                } else {
                                    state_count[current_state += 1] += 1;
                                }
                            }
                        } else {
                            // White pixel
                            if current_state == 1 {
                                // Counting black pixels
                                current_state += 1;
                            }
                            state_count[current_state] += 1;
                        }
                        j += 1;
                    }
                    if self.found_pattern_cross(&state_count) {
                         let confirmed: AlignmentPattern = self.handle_possible_center(&state_count, i, max_j);
                        if confirmed != null {
                            return Ok(confirmed);
                        }
                    }
                }
                i_gen += 1;
             }
         }

        // any guess at all, return it.
        if !self.possible_centers.is_empty() {
            return Ok(self.possible_centers.get(0));
        }
        throw NotFoundException::get_not_found_instance();
    }

    /**
   * Given a count of black/white/black pixels just seen and an end position,
   * figures the location of the center of this black/white/black run.
   */
    fn  center_from_end( state_count: &Vec<i32>,  end: i32) -> f32  {
        return (end - state_count[2]) - state_count[1] / 2.0f;
    }

    /**
   * @param stateCount count of black/white/black pixels just read
   * @return true iff the proportions of the counts is close enough to the 1/1/1 ratios
   *         used by alignment patterns to be considered a match
   */
    fn  found_pattern_cross(&self,  state_count: &Vec<i32>) -> bool  {
         let module_size: f32 = self.moduleSize;
         let max_variance: f32 = module_size / 2.0f;
         {
             let mut i: i32 = 0;
            while i < 3 {
                {
                    if Math::abs(module_size - state_count[i]) >= max_variance {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    /**
   * <p>After a horizontal scan finds a potential alignment pattern, this method
   * "cross-checks" by scanning down vertically through the center of the possible
   * alignment pattern to see if the same proportion is detected.</p>
   *
   * @param startI row where an alignment pattern was detected
   * @param centerJ center of the section that appears to cross an alignment pattern
   * @param maxCount maximum reasonable number of modules that should be
   * observed in any reading state, based on the results of the horizontal scan
   * @return vertical center of alignment pattern, or {@link Float#NaN} if not found
   */
    fn  cross_check_vertical(&self,  start_i: i32,  center_j: i32,  max_count: i32,  original_state_count_total: i32) -> f32  {
         let image: BitMatrix = self.image;
         let max_i: i32 = image.get_height();
         let state_count: Vec<i32> = self.cross_check_state_count;
        state_count[0] = 0;
        state_count[1] = 0;
        state_count[2] = 0;
        // Start counting up from center
         let mut i: i32 = start_i;
        while i >= 0 && image.get(center_j, i) && state_count[1] <= max_count {
            state_count[1] += 1;
            i -= 1;
        }
        // If already too many modules in this state or ran off the edge:
        if i < 0 || state_count[1] > max_count {
            return Float::NaN;
        }
        while i >= 0 && !image.get(center_j, i) && state_count[0] <= max_count {
            state_count[0] += 1;
            i -= 1;
        }
        if state_count[0] > max_count {
            return Float::NaN;
        }
        // Now also count down from center
        i = start_i + 1;
        while i < max_i && image.get(center_j, i) && state_count[1] <= max_count {
            state_count[1] += 1;
            i += 1;
        }
        if i == max_i || state_count[1] > max_count {
            return Float::NaN;
        }
        while i < max_i && !image.get(center_j, i) && state_count[2] <= max_count {
            state_count[2] += 1;
            i += 1;
        }
        if state_count[2] > max_count {
            return Float::NaN;
        }
         let state_count_total: i32 = state_count[0] + state_count[1] + state_count[2];
        if 5 * Math::abs(state_count_total - original_state_count_total) >= 2 * original_state_count_total {
            return Float::NaN;
        }
        return  if self.found_pattern_cross(&state_count) { ::center_from_end(&state_count, i) } else { Float::NaN };
    }

    /**
   * <p>This is called when a horizontal scan finds a possible alignment pattern. It will
   * cross check with a vertical scan, and if successful, will see if this pattern had been
   * found on a previous horizontal scan. If so, we consider it confirmed and conclude we have
   * found the alignment pattern.</p>
   *
   * @param stateCount reading state module counts from horizontal scan
   * @param i row where alignment pattern may be found
   * @param j end of possible alignment pattern in row
   * @return {@link AlignmentPattern} if we have found the same pattern twice, or null if not
   */
    fn  handle_possible_center(&self,  state_count: &Vec<i32>,  i: i32,  j: i32) -> AlignmentPattern  {
         let state_count_total: i32 = state_count[0] + state_count[1] + state_count[2];
         let center_j: f32 = ::center_from_end(&state_count, j);
         let center_i: f32 = self.cross_check_vertical(i, center_j as i32, 2 * state_count[1], state_count_total);
        if !Float::is_na_n(center_i) {
             let estimated_module_size: f32 = (state_count[0] + state_count[1] + state_count[2]) / 3.0f;
            for  let center: AlignmentPattern in self.possible_centers {
                // Look for about the same center and module size:
                if center.about_equals(estimated_module_size, center_i, center_j) {
                    return center.combine_estimate(center_i, center_j, estimated_module_size);
                }
            }
            // Hadn't found this before; save it
             let point: AlignmentPattern = AlignmentPattern::new(center_j, center_i, estimated_module_size);
            self.possible_centers.add(point);
            if self.result_point_callback != null {
                self.result_point_callback.found_possible_result_point(point);
            }
        }
        return null;
    }
}

// NEW FILE: detector.rs
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

// NEW FILE: finder_pattern.rs
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
 * <p>Encapsulates a finder pattern, which are the three square patterns found in
 * the corners of QR Codes. It also encapsulates a count of similar finder patterns,
 * as a convenience to the finder's bookkeeping.</p>
 *
 * @author Sean Owen
 */
pub struct FinderPattern {
    super: ResultPoint;

     let estimated_module_size: f32;

     let count: i32;
}

impl FinderPattern {

    fn new( pos_x: f32,  pos_y: f32,  estimated_module_size: f32) -> FinderPattern {
        this(pos_x, pos_y, estimated_module_size, 1);
    }

    fn new( pos_x: f32,  pos_y: f32,  estimated_module_size: f32,  count: i32) -> FinderPattern {
        super(pos_x, pos_y);
        let .estimatedModuleSize = estimated_module_size;
        let .count = count;
    }

    pub fn  get_estimated_module_size(&self) -> f32  {
        return self.estimated_module_size;
    }

    pub fn  get_count(&self) -> i32  {
        return self.count;
    }

    /**
   * <p>Determines if this finder pattern "about equals" a finder pattern at the stated
   * position and size -- meaning, it is at nearly the same center with nearly the same size.</p>
   */
    fn  about_equals(&self,  module_size: f32,  i: f32,  j: f32) -> bool  {
        if Math::abs(i - get_y()) <= module_size && Math::abs(j - get_x()) <= module_size {
             let module_size_diff: f32 = Math::abs(module_size - self.estimated_module_size);
            return module_size_diff <= 1.0f || module_size_diff <= self.estimated_module_size;
        }
        return false;
    }

    /**
   * Combines this object's current estimate of a finder pattern position and module size
   * with a new estimate. It returns a new {@code FinderPattern} containing a weighted average
   * based on count.
   */
    fn  combine_estimate(&self,  i: f32,  j: f32,  new_module_size: f32) -> FinderPattern  {
         let combined_count: i32 = self.count + 1;
         let combined_x: f32 = (self.count * get_x() + j) / combined_count;
         let combined_y: f32 = (self.count * get_y() + i) / combined_count;
         let combined_module_size: f32 = (self.count * self.estimated_module_size + new_module_size) / combined_count;
        return FinderPattern::new(combined_x, combined_y, combined_module_size, combined_count);
    }
}

// NEW FILE: finder_pattern_finder.rs
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
 * <p>This class attempts to find finder patterns in a QR Code. Finder patterns are the square
 * markers at three corners of a QR Code.</p>
 *
 * <p>This class is thread-safe but not reentrant. Each thread must allocate its own object.
 *
 * @author Sean Owen
 */

 const CENTER_QUORUM: i32 = 2;

 let module_comparator: EstimatedModuleComparator = EstimatedModuleComparator::new();

// 1 pixel/module times 3 modules/center
 const MIN_SKIP: i32 = 3;

// support up to version 20 for mobile clients
 const MAX_MODULES: i32 = 97;
pub struct FinderPatternFinder {

     let image: BitMatrix;

     let possible_centers: List<FinderPattern>;

     let has_skipped: bool;

     let cross_check_state_count: Vec<i32>;

     let result_point_callback: ResultPointCallback;
}

impl FinderPatternFinder {

    /**
   * <p>Creates a finder that will search the image for three finder patterns.</p>
   *
   * @param image image to search
   */
    pub fn new( image: &BitMatrix) -> FinderPatternFinder {
        this(image, null);
    }

    pub fn new( image: &BitMatrix,  result_point_callback: &ResultPointCallback) -> FinderPatternFinder {
        let .image = image;
        let .possibleCenters = ArrayList<>::new();
        let .crossCheckStateCount = : [i32; 5] = [0; 5];
        let .resultPointCallback = result_point_callback;
    }

    pub fn  get_image(&self) -> BitMatrix  {
        return self.image;
    }

    pub fn  get_possible_centers(&self) -> List<FinderPattern>  {
        return self.possible_centers;
    }

    fn  find(&self,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<FinderPatternInfo, Rc<Exception>>   {
         let try_harder: bool = hints != null && hints.contains_key(DecodeHintType::TRY_HARDER);
         let max_i: i32 = self.image.get_height();
         let max_j: i32 = self.image.get_width();
        // We are looking for black/white/black/white/black modules in
        // 1:1:3:1:1 ratio; this tracks the number of such modules seen so far
        // Let's assume that the maximum version QR Code we support takes up 1/4 the height of the
        // image, and then account for the center being 3 modules in size. This gives the smallest
        // number of pixels the center could be, so skip this often. When trying harder, look for all
        // QR versions regardless of how dense they are.
         let i_skip: i32 = (3 * max_i) / (4 * MAX_MODULES);
        if i_skip < MIN_SKIP || try_harder {
            i_skip = MIN_SKIP;
        }
         let mut done: bool = false;
         let state_count: [i32; 5] = [0; 5];
         {
             let mut i: i32 = i_skip - 1;
            while i < max_i && !done {
                {
                    // Get a row of black/white values
                    ::do_clear_counts(&state_count);
                     let current_state: i32 = 0;
                     {
                         let mut j: i32 = 0;
                        while j < max_j {
                            {
                                if self.image.get(j, i) {
                                    // Black pixel
                                    if (current_state & 1) == 1 {
                                        // Counting white pixels
                                        current_state += 1;
                                    }
                                    state_count[current_state] += 1;
                                } else {
                                    // White pixel
                                    if (current_state & 1) == 0 {
                                        // Counting black pixels
                                        if current_state == 4 {
                                            // A winner?
                                            if ::found_pattern_cross(&state_count) {
                                                // Yes
                                                 let confirmed: bool = self.handle_possible_center(&state_count, i, j);
                                                if confirmed {
                                                    // Start examining every other line. Checking each line turned out to be too
                                                    // expensive and didn't improve performance.
                                                    i_skip = 2;
                                                    if self.has_skipped {
                                                        done = self.have_multiply_confirmed_centers();
                                                    } else {
                                                         let row_skip: i32 = self.find_row_skip();
                                                        if row_skip > state_count[2] {
                                                            // Skip rows between row of lower confirmed center
                                                            // and top of presumed third confirmed center
                                                            // but back up a bit to get a full chance of detecting
                                                            // it, entire width of center of finder pattern
                                                            // Skip by rowSkip, but back off by stateCount[2] (size of last center
                                                            // of pattern we saw) to be conservative, and also back off by iSkip which
                                                            // is about to be re-added
                                                            i += row_skip - state_count[2] - i_skip;
                                                            j = max_j - 1;
                                                        }
                                                    }
                                                } else {
                                                    ::do_shift_counts2(&state_count);
                                                    current_state = 3;
                                                    continue;
                                                }
                                                // Clear state to start looking again
                                                current_state = 0;
                                                ::do_clear_counts(&state_count);
                                            } else {
                                                // No, shift counts back by two
                                                ::do_shift_counts2(&state_count);
                                                current_state = 3;
                                            }
                                        } else {
                                            state_count[current_state += 1] += 1;
                                        }
                                    } else {
                                        // Counting white pixels
                                        state_count[current_state] += 1;
                                    }
                                }
                            }
                            j += 1;
                         }
                     }

                    if ::found_pattern_cross(&state_count) {
                         let confirmed: bool = self.handle_possible_center(&state_count, i, max_j);
                        if confirmed {
                            i_skip = state_count[0];
                            if self.has_skipped {
                                // Found a third one
                                done = self.have_multiply_confirmed_centers();
                            }
                        }
                    }
                }
                i += i_skip;
             }
         }

         let pattern_info: Vec<FinderPattern> = self.select_best_patterns();
        ResultPoint::order_best_patterns(pattern_info);
        return Ok(FinderPatternInfo::new(pattern_info));
    }

    /**
   * Given a count of black/white/black/white/black pixels just seen and an end position,
   * figures the location of the center of this run.
   */
    fn  center_from_end( state_count: &Vec<i32>,  end: i32) -> f32  {
        return (end - state_count[4] - state_count[3]) - state_count[2] / 2.0f;
    }

    /**
   * @param stateCount count of black/white/black/white/black pixels just read
   * @return true iff the proportions of the counts is close enough to the 1/1/3/1/1 ratios
   *         used by finder patterns to be considered a match
   */
    pub fn  found_pattern_cross( state_count: &Vec<i32>) -> bool  {
         let total_module_size: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 5 {
                {
                     let count: i32 = state_count[i];
                    if count == 0 {
                        return false;
                    }
                    total_module_size += count;
                }
                i += 1;
             }
         }

        if total_module_size < 7 {
            return false;
        }
         let module_size: f32 = total_module_size / 7.0f;
         let max_variance: f32 = module_size / 2.0f;
        // Allow less than 50% variance from 1-1-3-1-1 proportions
        return Math::abs(module_size - state_count[0]) < max_variance && Math::abs(module_size - state_count[1]) < max_variance && Math::abs(3.0f * module_size - state_count[2]) < 3.0 * max_variance && Math::abs(module_size - state_count[3]) < max_variance && Math::abs(module_size - state_count[4]) < max_variance;
    }

    /**
   * @param stateCount count of black/white/black/white/black pixels just read
   * @return true iff the proportions of the counts is close enough to the 1/1/3/1/1 ratios
   *         used by finder patterns to be considered a match
   */
    pub fn  found_pattern_diagonal( state_count: &Vec<i32>) -> bool  {
         let total_module_size: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 5 {
                {
                     let count: i32 = state_count[i];
                    if count == 0 {
                        return false;
                    }
                    total_module_size += count;
                }
                i += 1;
             }
         }

        if total_module_size < 7 {
            return false;
        }
         let module_size: f32 = total_module_size / 7.0f;
         let max_variance: f32 = module_size / 1.333f;
        // Allow less than 75% variance from 1-1-3-1-1 proportions
        return Math::abs(module_size - state_count[0]) < max_variance && Math::abs(module_size - state_count[1]) < max_variance && Math::abs(3.0f * module_size - state_count[2]) < 3.0 * max_variance && Math::abs(module_size - state_count[3]) < max_variance && Math::abs(module_size - state_count[4]) < max_variance;
    }

    fn  get_cross_check_state_count(&self) -> Vec<i32>  {
        ::do_clear_counts(&self.cross_check_state_count);
        return self.cross_check_state_count;
    }

    pub fn  clear_counts(&self,  counts: &Vec<i32>)   {
        ::do_clear_counts(&counts);
    }

    pub fn  shift_counts2(&self,  state_count: &Vec<i32>)   {
        ::do_shift_counts2(&state_count);
    }

    pub fn  do_clear_counts( counts: &Vec<i32>)   {
        Arrays::fill(&counts, 0);
    }

    pub fn  do_shift_counts2( state_count: &Vec<i32>)   {
        state_count[0] = state_count[2];
        state_count[1] = state_count[3];
        state_count[2] = state_count[4];
        state_count[3] = 1;
        state_count[4] = 0;
    }

    /**
   * After a vertical and horizontal scan finds a potential finder pattern, this method
   * "cross-cross-cross-checks" by scanning down diagonally through the center of the possible
   * finder pattern to see if the same proportion is detected.
   *
   * @param centerI row where a finder pattern was detected
   * @param centerJ center of the section that appears to cross a finder pattern
   * @return true if proportions are withing expected limits
   */
    fn  cross_check_diagonal(&self,  center_i: i32,  center_j: i32) -> bool  {
         let state_count: Vec<i32> = self.get_cross_check_state_count();
        // Start counting up, left from center finding black center mass
         let mut i: i32 = 0;
        while center_i >= i && center_j >= i && self.image.get(center_j - i, center_i - i) {
            state_count[2] += 1;
            i += 1;
        }
        if state_count[2] == 0 {
            return false;
        }
        // Continue up, left finding white space
        while center_i >= i && center_j >= i && !self.image.get(center_j - i, center_i - i) {
            state_count[1] += 1;
            i += 1;
        }
        if state_count[1] == 0 {
            return false;
        }
        // Continue up, left finding black border
        while center_i >= i && center_j >= i && self.image.get(center_j - i, center_i - i) {
            state_count[0] += 1;
            i += 1;
        }
        if state_count[0] == 0 {
            return false;
        }
         let max_i: i32 = self.image.get_height();
         let max_j: i32 = self.image.get_width();
        // Now also count down, right from center
        i = 1;
        while center_i + i < max_i && center_j + i < max_j && self.image.get(center_j + i, center_i + i) {
            state_count[2] += 1;
            i += 1;
        }
        while center_i + i < max_i && center_j + i < max_j && !self.image.get(center_j + i, center_i + i) {
            state_count[3] += 1;
            i += 1;
        }
        if state_count[3] == 0 {
            return false;
        }
        while center_i + i < max_i && center_j + i < max_j && self.image.get(center_j + i, center_i + i) {
            state_count[4] += 1;
            i += 1;
        }
        if state_count[4] == 0 {
            return false;
        }
        return ::found_pattern_diagonal(&state_count);
    }

    /**
   * <p>After a horizontal scan finds a potential finder pattern, this method
   * "cross-checks" by scanning down vertically through the center of the possible
   * finder pattern to see if the same proportion is detected.</p>
   *
   * @param startI row where a finder pattern was detected
   * @param centerJ center of the section that appears to cross a finder pattern
   * @param maxCount maximum reasonable number of modules that should be
   * observed in any reading state, based on the results of the horizontal scan
   * @return vertical center of finder pattern, or {@link Float#NaN} if not found
   */
    fn  cross_check_vertical(&self,  start_i: i32,  center_j: i32,  max_count: i32,  original_state_count_total: i32) -> f32  {
         let image: BitMatrix = self.image;
         let max_i: i32 = image.get_height();
         let state_count: Vec<i32> = self.get_cross_check_state_count();
        // Start counting up from center
         let mut i: i32 = start_i;
        while i >= 0 && image.get(center_j, i) {
            state_count[2] += 1;
            i -= 1;
        }
        if i < 0 {
            return Float::NaN;
        }
        while i >= 0 && !image.get(center_j, i) && state_count[1] <= max_count {
            state_count[1] += 1;
            i -= 1;
        }
        // If already too many modules in this state or ran off the edge:
        if i < 0 || state_count[1] > max_count {
            return Float::NaN;
        }
        while i >= 0 && image.get(center_j, i) && state_count[0] <= max_count {
            state_count[0] += 1;
            i -= 1;
        }
        if state_count[0] > max_count {
            return Float::NaN;
        }
        // Now also count down from center
        i = start_i + 1;
        while i < max_i && image.get(center_j, i) {
            state_count[2] += 1;
            i += 1;
        }
        if i == max_i {
            return Float::NaN;
        }
        while i < max_i && !image.get(center_j, i) && state_count[3] < max_count {
            state_count[3] += 1;
            i += 1;
        }
        if i == max_i || state_count[3] >= max_count {
            return Float::NaN;
        }
        while i < max_i && image.get(center_j, i) && state_count[4] < max_count {
            state_count[4] += 1;
            i += 1;
        }
        if state_count[4] >= max_count {
            return Float::NaN;
        }
        // If we found a finder-pattern-like section, but its size is more than 40% different than
        // the original, assume it's a false positive
         let state_count_total: i32 = state_count[0] + state_count[1] + state_count[2] + state_count[3] + state_count[4];
        if 5 * Math::abs(state_count_total - original_state_count_total) >= 2 * original_state_count_total {
            return Float::NaN;
        }
        return  if ::found_pattern_cross(&state_count) { ::center_from_end(&state_count, i) } else { Float::NaN };
    }

    /**
   * <p>Like {@link #crossCheckVertical(int, int, int, int)}, and in fact is basically identical,
   * except it reads horizontally instead of vertically. This is used to cross-cross
   * check a vertical cross check and locate the real center of the alignment pattern.</p>
   */
    fn  cross_check_horizontal(&self,  start_j: i32,  center_i: i32,  max_count: i32,  original_state_count_total: i32) -> f32  {
         let image: BitMatrix = self.image;
         let max_j: i32 = image.get_width();
         let state_count: Vec<i32> = self.get_cross_check_state_count();
         let mut j: i32 = start_j;
        while j >= 0 && image.get(j, center_i) {
            state_count[2] += 1;
            j -= 1;
        }
        if j < 0 {
            return Float::NaN;
        }
        while j >= 0 && !image.get(j, center_i) && state_count[1] <= max_count {
            state_count[1] += 1;
            j -= 1;
        }
        if j < 0 || state_count[1] > max_count {
            return Float::NaN;
        }
        while j >= 0 && image.get(j, center_i) && state_count[0] <= max_count {
            state_count[0] += 1;
            j -= 1;
        }
        if state_count[0] > max_count {
            return Float::NaN;
        }
        j = start_j + 1;
        while j < max_j && image.get(j, center_i) {
            state_count[2] += 1;
            j += 1;
        }
        if j == max_j {
            return Float::NaN;
        }
        while j < max_j && !image.get(j, center_i) && state_count[3] < max_count {
            state_count[3] += 1;
            j += 1;
        }
        if j == max_j || state_count[3] >= max_count {
            return Float::NaN;
        }
        while j < max_j && image.get(j, center_i) && state_count[4] < max_count {
            state_count[4] += 1;
            j += 1;
        }
        if state_count[4] >= max_count {
            return Float::NaN;
        }
        // If we found a finder-pattern-like section, but its size is significantly different than
        // the original, assume it's a false positive
         let state_count_total: i32 = state_count[0] + state_count[1] + state_count[2] + state_count[3] + state_count[4];
        if 5 * Math::abs(state_count_total - original_state_count_total) >= original_state_count_total {
            return Float::NaN;
        }
        return  if ::found_pattern_cross(&state_count) { ::center_from_end(&state_count, j) } else { Float::NaN };
    }

    /**
   * @param stateCount reading state module counts from horizontal scan
   * @param i row where finder pattern may be found
   * @param j end of possible finder pattern in row
   * @param pureBarcode ignored
   * @return true if a finder pattern candidate was found this time
   * @deprecated only exists for backwards compatibility
   * @see #handlePossibleCenter(int[], int, int)
   */
    pub fn  handle_possible_center(&self,  state_count: &Vec<i32>,  i: i32,  j: i32,  pure_barcode: bool) -> bool  {
        return self.handle_possible_center(&state_count, i, j);
    }

    /**
   * <p>This is called when a horizontal scan finds a possible alignment pattern. It will
   * cross check with a vertical scan, and if successful, will, ah, cross-cross-check
   * with another horizontal scan. This is needed primarily to locate the real horizontal
   * center of the pattern in cases of extreme skew.
   * And then we cross-cross-cross check with another diagonal scan.</p>
   *
   * <p>If that succeeds the finder pattern location is added to a list that tracks
   * the number of times each location has been nearly-matched as a finder pattern.
   * Each additional find is more evidence that the location is in fact a finder
   * pattern center
   *
   * @param stateCount reading state module counts from horizontal scan
   * @param i row where finder pattern may be found
   * @param j end of possible finder pattern in row
   * @return true if a finder pattern candidate was found this time
   */
    pub fn  handle_possible_center(&self,  state_count: &Vec<i32>,  i: i32,  j: i32) -> bool  {
         let state_count_total: i32 = state_count[0] + state_count[1] + state_count[2] + state_count[3] + state_count[4];
         let center_j: f32 = ::center_from_end(&state_count, j);
         let center_i: f32 = self.cross_check_vertical(i, center_j as i32, state_count[2], state_count_total);
        if !Float::is_na_n(center_i) {
            // Re-cross check
            center_j = self.cross_check_horizontal(center_j as i32, center_i as i32, state_count[2], state_count_total);
            if !Float::is_na_n(center_j) && self.cross_check_diagonal(center_i as i32, center_j as i32) {
                 let estimated_module_size: f32 = state_count_total / 7.0f;
                 let mut found: bool = false;
                 {
                     let mut index: i32 = 0;
                    while index < self.possible_centers.size() {
                        {
                             let center: FinderPattern = self.possible_centers.get(index);
                            // Look for about the same center and module size:
                            if center.about_equals(estimated_module_size, center_i, center_j) {
                                self.possible_centers.set(index, &center.combine_estimate(center_i, center_j, estimated_module_size));
                                found = true;
                                break;
                            }
                        }
                        index += 1;
                     }
                 }

                if !found {
                     let point: FinderPattern = FinderPattern::new(center_j, center_i, estimated_module_size);
                    self.possible_centers.add(point);
                    if self.result_point_callback != null {
                        self.result_point_callback.found_possible_result_point(point);
                    }
                }
                return true;
            }
        }
        return false;
    }

    /**
   * @return number of rows we could safely skip during scanning, based on the first
   *         two finder patterns that have been located. In some cases their position will
   *         allow us to infer that the third pattern must lie below a certain point farther
   *         down in the image.
   */
    fn  find_row_skip(&self) -> i32  {
         let max: i32 = self.possible_centers.size();
        if max <= 1 {
            return 0;
        }
         let first_confirmed_center: ResultPoint = null;
        for  let center: FinderPattern in self.possible_centers {
            if center.get_count() >= CENTER_QUORUM {
                if first_confirmed_center == null {
                    first_confirmed_center = center;
                } else {
                    // We have two confirmed centers
                    // How far down can we skip before resuming looking for the next
                    // pattern? In the worst case, only the difference between the
                    // difference in the x / y coordinates of the two centers.
                    // This is the case where you find top left last.
                    self.has_skipped = true;
                    return (Math::abs(first_confirmed_center.get_x() - center.get_x()) - Math::abs(first_confirmed_center.get_y() - center.get_y())) as i32 / 2;
                }
            }
        }
        return 0;
    }

    /**
   * @return true iff we have found at least 3 finder patterns that have been detected
   *         at least {@link #CENTER_QUORUM} times each, and, the estimated module size of the
   *         candidates is "pretty similar"
   */
    fn  have_multiply_confirmed_centers(&self) -> bool  {
         let confirmed_count: i32 = 0;
         let total_module_size: f32 = 0.0f;
         let max: i32 = self.possible_centers.size();
        for  let pattern: FinderPattern in self.possible_centers {
            if pattern.get_count() >= CENTER_QUORUM {
                confirmed_count += 1;
                total_module_size += pattern.get_estimated_module_size();
            }
        }
        if confirmed_count < 3 {
            return false;
        }
        // OK, we have at least 3 confirmed centers, but, it's possible that one is a "false positive"
        // and that we need to keep looking. We detect this by asking if the estimated module sizes
        // vary too much. We arbitrarily say that when the total deviation from average exceeds
        // 5% of the total module size estimates, it's too much.
         let average: f32 = total_module_size / max;
         let total_deviation: f32 = 0.0f;
        for  let pattern: FinderPattern in self.possible_centers {
            total_deviation += Math::abs(pattern.get_estimated_module_size() - average);
        }
        return total_deviation <= 0.05f * total_module_size;
    }

    /**
   * Get square of distance between a and b.
   */
    fn  squared_distance( a: &FinderPattern,  b: &FinderPattern) -> f64  {
         let x: f64 = a.get_x() - b.get_x();
         let y: f64 = a.get_y() - b.get_y();
        return x * x + y * y;
    }

    /**
   * @return the 3 best {@link FinderPattern}s from our list of candidates. The "best" are
   *         those have similar module size and form a shape closer to a isosceles right triangle.
   * @throws NotFoundException if 3 such finder patterns do not exist
   */
    fn  select_best_patterns(&self) -> /*  throws NotFoundException */Result<Vec<FinderPattern>, Rc<Exception>>   {
         let start_size: i32 = self.possible_centers.size();
        if start_size < 3 {
            // Couldn't find enough finder patterns
            throw NotFoundException::get_not_found_instance();
        }
        self.possible_centers.sort(module_comparator);
         let mut distortion: f64 = Double::MAX_VALUE;
         let best_patterns: [Option<FinderPattern>; 3] = [None; 3];
         {
             let mut i: i32 = 0;
            while i < self.possible_centers.size() - 2 {
                {
                     let fpi: FinderPattern = self.possible_centers.get(i);
                     let min_module_size: f32 = fpi.get_estimated_module_size();
                     {
                         let mut j: i32 = i + 1;
                        while j < self.possible_centers.size() - 1 {
                            {
                                 let fpj: FinderPattern = self.possible_centers.get(j);
                                 let squares0: f64 = ::squared_distance(fpi, fpj);
                                 {
                                     let mut k: i32 = j + 1;
                                    while k < self.possible_centers.size() {
                                        {
                                             let fpk: FinderPattern = self.possible_centers.get(k);
                                             let max_module_size: f32 = fpk.get_estimated_module_size();
                                            if max_module_size > min_module_size * 1.4f {
                                                // module size is not similar
                                                continue;
                                            }
                                             let mut a: f64 = squares0;
                                             let mut b: f64 = ::squared_distance(fpj, fpk);
                                             let mut c: f64 = ::squared_distance(fpi, fpk);
                                            // sorts ascending - inlined
                                            if a < b {
                                                if b > c {
                                                    if a < c {
                                                         let temp: f64 = b;
                                                        b = c;
                                                        c = temp;
                                                    } else {
                                                         let temp: f64 = a;
                                                        a = c;
                                                        c = b;
                                                        b = temp;
                                                    }
                                                }
                                            } else {
                                                if b < c {
                                                    if a < c {
                                                         let temp: f64 = a;
                                                        a = b;
                                                        b = temp;
                                                    } else {
                                                         let temp: f64 = a;
                                                        a = b;
                                                        b = c;
                                                        c = temp;
                                                    }
                                                } else {
                                                     let temp: f64 = a;
                                                    a = c;
                                                    c = temp;
                                                }
                                            }
                                            // a^2 + b^2 = c^2 (Pythagorean theorem), and a = b (isosceles triangle).
                                            // Since any right triangle satisfies the formula c^2 - b^2 - a^2 = 0,
                                            // we need to check both two equal sides separately.
                                            // The value of |c^2 - 2 * b^2| + |c^2 - 2 * a^2| increases as dissimilarity
                                            // from isosceles right triangle.
                                             let d: f64 = Math::abs(c - 2.0 * b) + Math::abs(c - 2.0 * a);
                                            if d < distortion {
                                                distortion = d;
                                                best_patterns[0] = fpi;
                                                best_patterns[1] = fpj;
                                                best_patterns[2] = fpk;
                                            }
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        if distortion == Double::MAX_VALUE {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(best_patterns);
    }

    /**
   * <p>Orders by {@link FinderPattern#getEstimatedModuleSize()}</p>
   */
    #[derive(Comparator<FinderPattern>, Serializable)]
    struct EstimatedModuleComparator {
    }
    
    impl EstimatedModuleComparator {

        pub fn  compare(&self,  center1: &FinderPattern,  center2: &FinderPattern) -> i32  {
            return Float::compare(&center1.get_estimated_module_size(), &center2.get_estimated_module_size());
        }
    }

}

// NEW FILE: finder_pattern_info.rs
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
 * <p>Encapsulates information about finder patterns in an image, including the location of
 * the three finder patterns, and their estimated module size.</p>
 *
 * @author Sean Owen
 */
pub struct FinderPatternInfo {

     let bottom_left: FinderPattern;

     let top_left: FinderPattern;

     let top_right: FinderPattern;
}

impl FinderPatternInfo {

    pub fn new( pattern_centers: &Vec<FinderPattern>) -> FinderPatternInfo {
        let .bottomLeft = pattern_centers[0];
        let .topLeft = pattern_centers[1];
        let .topRight = pattern_centers[2];
    }

    pub fn  get_bottom_left(&self) -> FinderPattern  {
        return self.bottom_left;
    }

    pub fn  get_top_left(&self) -> FinderPattern  {
        return self.top_left;
    }

    pub fn  get_top_right(&self) -> FinderPattern  {
        return self.top_right;
    }
}

