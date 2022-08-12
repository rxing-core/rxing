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

