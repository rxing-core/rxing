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

use crate::{
    common::BitMatrix, result_point_utils, DecodeHintType, DecodeHintValue, DecodingHintDictionary,
    Exceptions, RXingResultPointCallback, ResultPoint,
};

use super::{FinderPattern, FinderPatternInfo};

/**
 * <p>This class attempts to find finder patterns in a QR Code. Finder patterns are the square
 * markers at three corners of a QR Code.</p>
 *
 * <p>This class is thread-safe but not reentrant. Each thread must allocate its own object.
 *
 * @author Sean Owen
 */
pub struct FinderPatternFinder<'a> {
    image: &'a BitMatrix,
    possibleCenters: Vec<FinderPattern>,
    hasSkipped: bool,
    crossCheckStateCount: [u32; 5],
    resultPointCallback: Option<RXingResultPointCallback>,
}
impl<'a> FinderPatternFinder<'_> {
    pub const CENTER_QUORUM: usize = 2;
    pub const MIN_SKIP: u32 = 3; // 1 pixel/module times 3 modules/center
    pub const MAX_MODULES: u32 = 97; // support up to version 20 for mobile clients

    /**
     * <p>Creates a finder that will search the image for three finder patterns.</p>
     *
     * @param image image to search
     */
    pub fn new(image: &'a BitMatrix) -> FinderPatternFinder<'a> {
        Self::with_callback(image, None)
    }

    pub fn with_callback(
        image: &'a BitMatrix,
        resultPointCallback: Option<RXingResultPointCallback>,
    ) -> FinderPatternFinder<'a> {
        FinderPatternFinder {
            image,
            possibleCenters: Vec::new(),
            hasSkipped: false,
            crossCheckStateCount: [0u32; 5],
            resultPointCallback,
        }
    }

    pub fn getImage(&self) -> &BitMatrix {
        self.image
    }

    pub fn getPossibleCenters(&self) -> &Vec<FinderPattern> {
        &self.possibleCenters
    }

    pub fn find(
        &mut self,
        hints: &DecodingHintDictionary,
    ) -> Result<FinderPatternInfo, Exceptions> {
        let tryHarder = matches!(
            hints.get(&DecodeHintType::TRY_HARDER),
            Some(DecodeHintValue::TryHarder(true))
        );
        let maxI = self.image.getHeight();
        let maxJ = self.image.getWidth();
        // We are looking for black/white/black/white/black modules in
        // 1:1:3:1:1 ratio; this tracks the number of such modules seen so far

        // Let's assume that the maximum version QR Code we support takes up 1/4 the height of the
        // image, and then account for the center being 3 modules in size. This gives the smallest
        // number of pixels the center could be, so skip this often. When trying harder, look for all
        // QR versions regardless of how dense they are.
        let mut iSkip = (3 * maxI) / (4 * Self::MAX_MODULES);
        if iSkip < Self::MIN_SKIP || tryHarder {
            iSkip = Self::MIN_SKIP;
        }

        let mut done = false;
        let mut stateCount = [0u32; 5];
        let mut i = iSkip as i32 - 1;
        while i < maxI as i32 && !done {
            // Get a row of black/white values
            FinderPatternFinder::doClearCounts(&mut stateCount);
            let mut currentState = 0;
            let mut j = 0;
            while j < maxJ {
                if self.image.get(j, i as u32) {
                    // Black pixel
                    if (currentState & 1) == 1 {
                        // Counting white pixels
                        currentState += 1;
                    }
                    stateCount[currentState] += 1;
                } else {
                    // White pixel
                    if (currentState & 1) == 0 {
                        // Counting black pixels
                        if currentState == 4 {
                            // A winner?
                            if FinderPatternFinder::foundPatternCross(&stateCount) {
                                // Yes
                                let confirmed = self.handlePossibleCenter(&stateCount, i as u32, j);
                                if confirmed {
                                    // Start examining every other line. Checking each line turned out to be too
                                    // expensive and didn't improve performance.
                                    iSkip = 2;
                                    if self.hasSkipped {
                                        done = self.haveMultiplyConfirmedCenters();
                                    } else {
                                        let rowSkip = self.findRowSkip();
                                        if rowSkip > stateCount[2] {
                                            // Skip rows between row of lower confirmed center
                                            // and top of presumed third confirmed center
                                            // but back up a bit to get a full chance of detecting
                                            // it, entire width of center of finder pattern

                                            // Skip by rowSkip, but back off by stateCount[2] (size of last center
                                            // of pattern we saw) to be conservative, and also back off by iSkip which
                                            // is about to be re-added
                                            i += rowSkip as i32
                                                - stateCount[2] as i32
                                                - iSkip as i32;
                                            // i += rowSkip  - stateCount[2]  - iSkip ;
                                            j = maxJ - 1;
                                        }
                                    }
                                } else {
                                    FinderPatternFinder::doShiftCounts2(&mut stateCount);
                                    currentState = 3;
                                    j += 1;
                                    continue;
                                }
                                // Clear state to start looking again
                                currentState = 0;
                                FinderPatternFinder::doClearCounts(&mut stateCount);
                            } else {
                                // No, shift counts back by two
                                FinderPatternFinder::doShiftCounts2(&mut stateCount);
                                currentState = 3;
                            }
                        } else {
                            currentState += 1;
                            stateCount[currentState] += 1;
                        }
                    } else {
                        // Counting white pixels
                        stateCount[currentState] += 1;
                    }
                }
                j += 1;
            }
            if FinderPatternFinder::foundPatternCross(&stateCount) {
                let confirmed = self.handlePossibleCenter(&stateCount, i as u32, maxJ);
                if confirmed {
                    iSkip = stateCount[0];
                    if self.hasSkipped {
                        // Found a third one
                        done = self.haveMultiplyConfirmedCenters();
                    }
                }
            }

            i += iSkip as i32;
        }

        let mut patternInfo = self.selectBestPatterns()?;
        result_point_utils::orderBestPatterns(&mut patternInfo);

        Ok(FinderPatternInfo::new(patternInfo))
    }

    /**
     * Given a count of black/white/black/white/black pixels just seen and an end position,
     * figures the location of the center of this run.
     */
    fn centerFromEnd(stateCount: &[u32], end: u32) -> f32 {
        (end - stateCount[4] - stateCount[3]) as f32 - ((stateCount[2] as f32) / 2.0)
    }

    /**
     * @param stateCount count of black/white/black/white/black pixels just read
     * @return true iff the proportions of the counts is close enough to the 1/1/3/1/1 ratios
     *         used by finder patterns to be considered a match
     */
    pub fn foundPatternCross(stateCount: &[u32]) -> bool {
        let mut totalModuleSize = 0;
        for count in stateCount.iter().take(5) {
            if *count == 0 {
                return false;
            }
            totalModuleSize += *count;
        }
        if totalModuleSize < 7 {
            return false;
        }
        let moduleSize = totalModuleSize as f64 / 7.0;
        let maxVariance = moduleSize / 2.0;
        // Allow less than 50% variance from 1-1-3-1-1 proportions
        ((moduleSize - stateCount[0] as f64).abs()) < maxVariance
            && ((moduleSize - stateCount[1] as f64).abs()) < maxVariance
            && ((3.0 * moduleSize - stateCount[2] as f64).abs()) < 3.0 * maxVariance
            && (moduleSize - stateCount[3] as f64).abs() < maxVariance
            && (moduleSize - stateCount[4] as f64).abs() < maxVariance
    }

    /**
     * @param stateCount count of black/white/black/white/black pixels just read
     * @return true iff the proportions of the counts is close enough to the 1/1/3/1/1 ratios
     *         used by finder patterns to be considered a match
     */
    pub fn foundPatternDiagonal(stateCount: &[u32]) -> bool {
        let mut totalModuleSize = 0;
        for count in stateCount.iter().take(5) {
            if *count == 0 {
                return false;
            }
            totalModuleSize += *count;
        }
        if totalModuleSize < 7 {
            return false;
        }
        let moduleSize = totalModuleSize as f64 / 7.0;
        let maxVariance = moduleSize / 1.333;
        // Allow less than 75% variance from 1-1-3-1-1 proportions
        (moduleSize - stateCount[0] as f64).abs() < maxVariance
            && (moduleSize - stateCount[1] as f64).abs() < maxVariance
            && (3.0 * moduleSize - stateCount[2] as f64).abs() < 3.0 * maxVariance
            && (moduleSize - stateCount[3] as f64).abs() < maxVariance
            && (moduleSize - stateCount[4] as f64).abs() < maxVariance
    }

    fn getCrossCheckStateCount(&mut self) -> &[u32; 5] {
        FinderPatternFinder::doClearCounts(&mut self.crossCheckStateCount);
        &self.crossCheckStateCount
    }

    #[deprecated]
    pub fn clearCounts(&self, counts: &mut [u32; 5]) {
        Self::doClearCounts(counts);
    }

    #[deprecated]
    pub fn shiftCounts2(&self, stateCount: &mut [u32; 5]) {
        Self::doShiftCounts2(stateCount);
    }

    pub fn doClearCounts(counts: &mut [u32; 5]) {
        counts.fill(0)
    }

    pub fn doShiftCounts2(stateCount: &mut [u32]) {
        stateCount[0] = stateCount[2];
        stateCount[1] = stateCount[3];
        stateCount[2] = stateCount[4];
        stateCount[3] = 1;
        stateCount[4] = 0;
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
    fn crossCheckDiagonal(&mut self, centerI: u32, centerJ: u32) -> bool {
        let _state_count = self.getCrossCheckStateCount();

        // Start counting up, left from center finding black center mass
        let mut i = 0;
        while centerI >= i && centerJ >= i && self.image.get(centerJ - i, centerI - i) {
            self.crossCheckStateCount[2] += 1;
            i += 1;
        }
        if self.crossCheckStateCount[2] == 0 {
            return false;
        }

        // Continue up, left finding white space
        while centerI >= i && centerJ >= i && !self.image.get(centerJ - i, centerI - i) {
            self.crossCheckStateCount[1] += 1;
            i += 1;
        }
        if self.crossCheckStateCount[1] == 0 {
            return false;
        }

        // Continue up, left finding black border
        while centerI >= i && centerJ >= i && self.image.get(centerJ - i, centerI - i) {
            self.crossCheckStateCount[0] += 1;
            i += 1;
        }
        if self.crossCheckStateCount[0] == 0 {
            return false;
        }

        let maxI = self.image.getHeight();
        let maxJ = self.image.getWidth();

        // Now also count down, right from center
        i = 1;
        while centerI + i < maxI && centerJ + i < maxJ && self.image.get(centerJ + i, centerI + i) {
            self.crossCheckStateCount[2] += 1;
            i += 1;
        }

        while centerI + i < maxI && centerJ + i < maxJ && !self.image.get(centerJ + i, centerI + i)
        {
            self.crossCheckStateCount[3] += 1;
            i += 1;
        }
        if self.crossCheckStateCount[3] == 0 {
            return false;
        }

        while centerI + i < maxI && centerJ + i < maxJ && self.image.get(centerJ + i, centerI + i) {
            self.crossCheckStateCount[4] += 1;
            i += 1;
        }
        if self.crossCheckStateCount[4] == 0 {
            return false;
        }

        Self::foundPatternDiagonal(&self.crossCheckStateCount)
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
    fn crossCheckVertical(
        &mut self,
        startI: u32,
        centerJ: u32,
        maxCount: u32,
        originalStateCountTotal: u32,
    ) -> f32 {
        let maxI = self.image.getHeight() as i32;
        let _stateCount = self.getCrossCheckStateCount();

        // Start counting up from center
        let mut i = startI as i32;
        while i >= 0 && self.image.get(centerJ, i as u32) {
            self.crossCheckStateCount[2] += 1;
            i -= 1;
        }
        if i < 0 {
            return f32::NAN;
        }
        while i >= 0
            && !self.image.get(centerJ, i as u32)
            && self.crossCheckStateCount[1] <= maxCount
        {
            self.crossCheckStateCount[1] += 1;
            i -= 1;
        }
        // If already too many modules in this state or ran off the edge:
        if i < 0 || self.crossCheckStateCount[1] > maxCount {
            return f32::NAN;
        }
        while i >= 0
            && self.image.get(centerJ, i as u32)
            && self.crossCheckStateCount[0] <= maxCount
        {
            self.crossCheckStateCount[0] += 1;
            i -= 1;
        }
        if self.crossCheckStateCount[0] > maxCount {
            return f32::NAN;
        }

        // Now also count down from center
        i = startI as i32 + 1;
        while i < maxI && self.image.get(centerJ, i as u32) {
            self.crossCheckStateCount[2] += 1;
            i += 1;
        }
        if i == maxI {
            return f32::NAN;
        }
        while i < maxI
            && !self.image.get(centerJ, i as u32)
            && self.crossCheckStateCount[3] < maxCount
        {
            self.crossCheckStateCount[3] += 1;
            i += 1;
        }
        if i == maxI || self.crossCheckStateCount[3] >= maxCount {
            return f32::NAN;
        }
        while i < maxI
            && self.image.get(centerJ, i as u32)
            && self.crossCheckStateCount[4] < maxCount
        {
            self.crossCheckStateCount[4] += 1;
            i += 1;
        }
        if self.crossCheckStateCount[4] >= maxCount {
            return f32::NAN;
        }

        // If we found a finder-pattern-like section, but its size is more than 40% different than
        // the original, assume it's a false positive
        let stateCountTotal = self.crossCheckStateCount.iter().sum::<u32>();

        if 5 * (stateCountTotal as i64 - originalStateCountTotal as i64)
            >= 2 * originalStateCountTotal as i64
        {
            return f32::NAN;
        }

        if Self::foundPatternCross(&self.crossCheckStateCount) {
            Self::centerFromEnd(&self.crossCheckStateCount, i as u32)
        } else {
            f32::NAN
        }
    }

    /**
     * <p>Like {@link #crossCheckVertical(int, int, int, int)}, and in fact is basically identical,
     * except it reads horizontally instead of vertically. This is used to cross-cross
     * check a vertical cross check and locate the real center of the alignment pattern.</p>
     */
    fn crossCheckHorizontal(
        &mut self,
        startJ: u32,
        centerI: u32,
        maxCount: u32,
        originalStateCountTotal: u32,
    ) -> f32 {
        let maxJ = self.image.getWidth();
        let _stateCount = self.getCrossCheckStateCount();

        let mut j = startJ as i32;
        while j >= 0 && self.image.get(j as u32, centerI) {
            self.crossCheckStateCount[2] += 1;
            j -= 1;
        }
        if j < 0 {
            return f32::NAN;
        }

        while j >= 0
            && !self.image.get(j as u32, centerI)
            && self.crossCheckStateCount[1] <= maxCount
        {
            self.crossCheckStateCount[1] += 1;
            j -= 1;
        }
        if j < 0 || self.crossCheckStateCount[1] > maxCount {
            return f32::NAN;
        }

        while j >= 0
            && self.image.get(j as u32, centerI)
            && self.crossCheckStateCount[0] <= maxCount
        {
            self.crossCheckStateCount[0] += 1;
            j -= 1;
        }
        if self.crossCheckStateCount[0] > maxCount {
            return f32::NAN;
        }

        j = startJ as i32 + 1;
        while j < (maxJ as i32) && self.image.get(j as u32, centerI) {
            self.crossCheckStateCount[2] += 1;
            j += 1;
        }
        if j == maxJ as i32 {
            return f32::NAN;
        }

        while j < maxJ as i32
            && !self.image.get(j as u32, centerI)
            && self.crossCheckStateCount[3] < maxCount
        {
            self.crossCheckStateCount[3] += 1;
            j += 1;
        }
        if j == (maxJ as i32) || self.crossCheckStateCount[3] >= maxCount {
            return f32::NAN;
        }

        while j < (maxJ as i32)
            && self.image.get(j as u32, centerI)
            && self.crossCheckStateCount[4] < maxCount
        {
            self.crossCheckStateCount[4] += 1;
            j += 1;
        }
        if self.crossCheckStateCount[4] >= maxCount {
            return f32::NAN;
        }

        // If we found a finder-pattern-like section, but its size is significantly different than
        // the original, assume it's a false positive
        let stateCountTotal = self.crossCheckStateCount.iter().sum::<u32>();

        if 5 * (stateCountTotal as i64 - originalStateCountTotal as i64)
            >= originalStateCountTotal as i64
        {
            return f32::NAN;
        }

        if Self::foundPatternCross(&self.crossCheckStateCount) {
            Self::centerFromEnd(&self.crossCheckStateCount, j as u32)
        } else {
            f32::NAN
        }
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
    #[deprecated]
    pub fn handlePossibleCenterWithPureBarcodeFlag(
        &mut self,
        stateCount: &[u32],
        i: u32,
        j: u32,
        _pureBarcode: bool,
    ) -> bool {
        self.handlePossibleCenter(stateCount, i, j)
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
    pub fn handlePossibleCenter(&mut self, stateCount: &[u32], i: u32, j: u32) -> bool {
        let stateCountTotal =
            stateCount[0] + stateCount[1] + stateCount[2] + stateCount[3] + stateCount[4];
        let mut centerJ = Self::centerFromEnd(stateCount, j);
        let centerI =
            self.crossCheckVertical(i, centerJ.floor() as u32, stateCount[2], stateCountTotal);
        if !centerI.is_nan() {
            // Re-cross check
            centerJ = self.crossCheckHorizontal(
                centerJ.floor() as u32,
                centerI.floor() as u32,
                stateCount[2],
                stateCountTotal,
            );
            if !centerJ.is_nan()
                && self.crossCheckDiagonal(centerI.floor() as u32, centerJ.floor() as u32)
            {
                let estimatedModuleSize = stateCountTotal as f32 / 7.0;
                let mut found = false;
                for center in self.possibleCenters.iter_mut() {
                    // Look for about the same center and module size:
                    if center.aboutEquals(estimatedModuleSize, centerI, centerJ) {
                        *center = center.combineEstimate(centerI, centerJ, estimatedModuleSize);
                        found = true;
                        break;
                    }
                }
                if !found {
                    let point = FinderPattern::new(centerJ, centerI, estimatedModuleSize);
                    self.possibleCenters.push(point);
                    if let Some(rpc) = self.resultPointCallback.clone() {
                        rpc(&point);
                    }
                }
                return true;
            }
        }
        false
    }

    /**
     * @return number of rows we could safely skip during scanning, based on the first
     *         two finder patterns that have been located. In some cases their position will
     *         allow us to infer that the third pattern must lie below a certain point farther
     *         down in the image.
     */
    fn findRowSkip(&mut self) -> u32 {
        let max = self.possibleCenters.len();
        if max <= 1 {
            return 0;
        }
        let mut firstConfirmedCenter: Option<&FinderPattern> = None;
        for center in &self.possibleCenters {
            if center.getCount() >= Self::CENTER_QUORUM {
                if let Some(fnp) = firstConfirmedCenter {
                    // We have two confirmed centers
                    // How far down can we skip before resuming looking for the next
                    // pattern? In the worst case, only the difference between the
                    // difference in the x / y coordinates of the two centers.
                    // This is the case where you find top left last.
                    self.hasSkipped = true;
                    return (((fnp.getX() - center.getX()).abs()
                        - (fnp.getY() - center.getY()).abs())
                        / 2.0)
                        .floor() as u32;
                } else {
                    firstConfirmedCenter.replace(center);
                }
            }
        }
        0
    }

    /**
     * @return true iff we have found at least 3 finder patterns that have been detected
     *         at least {@link #CENTER_QUORUM} times each, and, the estimated module size of the
     *         candidates is "pretty similar"
     */
    fn haveMultiplyConfirmedCenters(&self) -> bool {
        let mut confirmedCount = 0;
        let mut totalModuleSize = 0.0;
        let max = self.possibleCenters.len();
        for pattern in &self.possibleCenters {
            if pattern.getCount() >= Self::CENTER_QUORUM {
                confirmedCount += 1;
                totalModuleSize += pattern.getEstimatedModuleSize();
            }
        }
        if confirmedCount < 3 {
            return false;
        }
        // OK, we have at least 3 confirmed centers, but, it's possible that one is a "false positive"
        // and that we need to keep looking. We detect this by asking if the estimated module sizes
        // vary too much. We arbitrarily say that when the total deviation from average exceeds
        // 5% of the total module size estimates, it's too much.
        let average = totalModuleSize / max as f32;
        let totalDeviation = self.possibleCenters.iter().fold(0.0, |acc, pattern| {
            acc + (pattern.getEstimatedModuleSize() - average).abs()
        });

        totalDeviation <= 0.05 * totalModuleSize
    }

    /**
     * Get square of distance between a and b.
     */
    fn squaredDistance(a: &FinderPattern, b: &FinderPattern) -> f64 {
        let x = a.getX() as f64 - b.getX() as f64;
        let y = a.getY() as f64 - b.getY() as f64;

        x * x + y * y
    }

    /**
     * @return the 3 best {@link FinderPattern}s from our list of candidates. The "best" are
     *         those have similar module size and form a shape closer to a isosceles right triangle.
     * @throws NotFoundException if 3 such finder patterns do not exist
     */
    fn selectBestPatterns(&mut self) -> Result<[FinderPattern; 3], Exceptions> {
        let startSize = self.possibleCenters.len();
        if startSize < 3 {
            // Couldn't find enough finder patterns
            return Err(Exceptions::NotFoundException(None));
        }

        self.possibleCenters
            .retain(|fp| fp.getCount() >= Self::CENTER_QUORUM);

        self.possibleCenters.sort_unstable_by(|x, y| {
            x.getEstimatedModuleSize()
                .partial_cmp(&y.getEstimatedModuleSize())
                .unwrap_or(std::cmp::Ordering::Less) // we are making a weird assumption that uncomparable items are result in Less
        });

        let mut distortion = f64::MAX;
        let mut bestPatterns = [None; 3];

        for i in 0..self.possibleCenters.len() {
            let Some(fpi) = self.possibleCenters.get(i) else {
                return Err(Exceptions::NotFoundException(None));
            };
            let minModuleSize = fpi.getEstimatedModuleSize();

            for j in (i + 1)..(self.possibleCenters.len() - 1) {
                let Some(fpj) =  self.possibleCenters.get(j) else {
                    return Err(Exceptions::NotFoundException(None));
                };
                let squares0 = Self::squaredDistance(fpi, fpj);

                for k in (j + 1)..(self.possibleCenters.len()) {
                    let Some(fpk) = self.possibleCenters.get(k) else {
                        return Err(Exceptions::NotFoundException(None));
                    };
                    let maxModuleSize = fpk.getEstimatedModuleSize();
                    if maxModuleSize > minModuleSize * 1.4 {
                        // module size is not similar
                        continue;
                    }

                    let mut a = squares0;
                    let mut b = Self::squaredDistance(fpj, fpk);
                    let mut c = Self::squaredDistance(fpi, fpk);

                    // sorts ascending - inlined
                    if a < b {
                        if b > c {
                            if a < c {
                                std::mem::swap(&mut b, &mut c)
                            } else {
                                let temp = a;
                                a = c;
                                c = b;
                                b = temp;
                            }
                        }
                    } else if b < c {
                        if a < c {
                            std::mem::swap(&mut a, &mut b)
                        } else {
                            let temp = a;
                            a = b;
                            b = c;
                            c = temp;
                        }
                    } else {
                        std::mem::swap(&mut a, &mut c);
                    }

                    // a^2 + b^2 = c^2 (Pythagorean theorem), and a = b (isosceles triangle).
                    // Since any right triangle satisfies the formula c^2 - b^2 - a^2 = 0,
                    // we need to check both two equal sides separately.
                    // The value of |c^2 - 2 * b^2| + |c^2 - 2 * a^2| increases as dissimilarity
                    // from isosceles right triangle.
                    let d = (c - 2.0 * b).abs() + (c - 2.0 * a).abs();
                    if d < distortion {
                        distortion = d;
                        bestPatterns = [Some(*fpi), Some(*fpj), Some(*fpk)];
                    }
                }
            }
        }

        if distortion == f64::MAX {
            return Err(Exceptions::NotFoundException(None));
        }

        if bestPatterns[0].is_none() {
            return Err(Exceptions::NotFoundException(None));
        }

        let p1 = bestPatterns[0].ok_or(Exceptions::NotFoundException(None))?;
        let p2 = bestPatterns[1].ok_or(Exceptions::NotFoundException(None))?;
        let p3 = bestPatterns[2].ok_or(Exceptions::NotFoundException(None))?;

        Ok([p1, p2, p3])
    }
}
