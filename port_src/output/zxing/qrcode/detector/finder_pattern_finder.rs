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

