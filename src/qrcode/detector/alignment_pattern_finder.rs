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
    common::{BitMatrix, Result},
    Exceptions, RXingResultPointCallback,
};

use super::AlignmentPattern;

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
pub struct AlignmentPatternFinder {
    image: BitMatrix,
    possibleCenters: Vec<AlignmentPattern>,
    startX: u32,
    startY: u32,
    width: u32,
    height: u32,
    moduleSize: f32,
    crossCheckStateCount: [u32; 3],
    resultPointCallback: Option<RXingResultPointCallback>,
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
    pub fn new(
        image: BitMatrix,
        startX: u32,
        startY: u32,
        width: u32,
        height: u32,
        moduleSize: f32,
        resultPointCallback: Option<RXingResultPointCallback>,
    ) -> Self {
        Self {
            image,
            possibleCenters: Vec::with_capacity(5),
            startX,
            startY,
            width,
            height,
            moduleSize,
            crossCheckStateCount: [0u32; 3],
            resultPointCallback,
        }
    }

    /**
     * <p>This method attempts to find the bottom-right alignment pattern in the image. It is a bit messy since
     * it's pretty performance-critical and so is written to be fast foremost.</p>
     *
     * @return {@link AlignmentPattern} if found
     * @throws NotFoundException if not found
     */
    pub fn find(&mut self) -> Result<AlignmentPattern> {
        let startX = self.startX;
        let height = self.height;
        let maxJ = startX + self.width;
        let middleI = self.startY + (height / 2);
        // We are looking for black/white/black modules in 1:1:1 ratio;
        // this tracks the number of black/white/black modules seen so far
        let mut stateCount = [0u32; 3];
        for iGen in 0..height {
            // Search from middle outwards
            let i = (middleI as i32
                + (if (iGen & 0x01) == 0 {
                    (iGen as i32 + 1) / 2
                } else {
                    -((iGen as i32 + 1) / 2)
                })) as u32;

            stateCount.fill(0);

            let mut j = startX;
            // Burn off leading white pixels before anything else; if we start in the middle of
            // a white run, it doesn't make sense to count its length, since we don't know if the
            // white run continued to the left of the start point
            while j < maxJ && !self.image.get(j, i) {
                j += 1;
            }
            let mut currentState = 0;
            while j < maxJ {
                if self.image.get(j, i) {
                    // Black pixel
                    if currentState == 1 {
                        // Counting black pixels
                        stateCount[1] += 1;
                    } else {
                        // Counting white pixels
                        if currentState == 2 {
                            // A winner?
                            if self.foundPatternCross(&stateCount) {
                                // Yes
                                if let Some(confirmed) =
                                    self.handlePossibleCenter(&stateCount, i, j)
                                {
                                    return Ok(confirmed);
                                }
                            }
                            stateCount[0] = stateCount[2];
                            stateCount[1] = 1;
                            stateCount[2] = 0;
                            currentState = 1;
                        } else {
                            currentState += 1;
                            stateCount[currentState] += 1;
                        }
                    }
                } else {
                    // White pixel
                    if currentState == 1 {
                        // Counting black pixels
                        currentState += 1;
                    }
                    stateCount[currentState] += 1;
                }
                j += 1;
            }
            if self.foundPatternCross(&stateCount) {
                if let Some(confirmed) = self.handlePossibleCenter(&stateCount, i, maxJ) {
                    return Ok(confirmed);
                }
            }
        }

        // Hmm, nothing we saw was observed and confirmed twice. If we had
        // any guess at all, return it.
        if !self.possibleCenters.is_empty() {
            Ok(*(self
                .possibleCenters
                .get(0)
                .ok_or(Exceptions::IndexOutOfBoundsException(None)))?)
        } else {
            Err(Exceptions::NotFoundException(None))
        }
    }

    /**
     * Given a count of black/white/black pixels just seen and an end position,
     * figures the location of the center of this black/white/black run.
     */
    fn centerFromEnd(stateCount: &[u32], end: u32) -> f32 {
        (end as f32 - stateCount[2] as f32) - stateCount[1] as f32 / 2.0
    }

    /**
     * @param stateCount count of black/white/black pixels just read
     * @return true iff the proportions of the counts is close enough to the 1/1/1 ratios
     *         used by alignment patterns to be considered a match
     */
    fn foundPatternCross(&self, stateCount: &[u32]) -> bool {
        let moduleSize = self.moduleSize;
        let maxVariance = moduleSize / 2.0;
        for state in stateCount.iter().take(3) {
            if (moduleSize - *state as f32).abs() >= maxVariance {
                return false;
            }
        }
        true
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
    fn crossCheckVertical(
        &mut self,
        startI: u32,
        centerJ: u32,
        maxCount: u32,
        originalStateCountTotal: u32,
    ) -> f32 {
        let image = &self.image;

        let maxI = image.getHeight();
        self.crossCheckStateCount.fill(0);

        // Start counting up from center
        let mut i = startI as i32;
        while i >= 0 && image.get(centerJ, i as u32) && self.crossCheckStateCount[1] <= maxCount {
            self.crossCheckStateCount[1] += 1;
            i -= 1;
        }
        // If already too many modules in this state or ran off the edge:
        if i < 0 || self.crossCheckStateCount[1] > maxCount {
            return f32::NAN;
        }
        while i >= 0 && !image.get(centerJ, i as u32) && self.crossCheckStateCount[0] <= maxCount {
            self.crossCheckStateCount[0] += 1;
            i -= 1;
        }
        if self.crossCheckStateCount[0] > maxCount {
            return f32::NAN;
        }

        // Now also count down from center
        i = startI as i32 + 1;
        while i < maxI as i32
            && image.get(centerJ, i as u32)
            && self.crossCheckStateCount[1] <= maxCount
        {
            self.crossCheckStateCount[1] += 1;
            i += 1;
        }
        if i == maxI as i32 || self.crossCheckStateCount[1] > maxCount {
            return f32::NAN;
        }
        while i < maxI as i32
            && !image.get(centerJ, i as u32)
            && self.crossCheckStateCount[2] <= maxCount
        {
            self.crossCheckStateCount[2] += 1;
            i += 1;
        }
        if self.crossCheckStateCount[2] > maxCount {
            return f32::NAN;
        }

        let stateCountTotal = self.crossCheckStateCount[0]
            + self.crossCheckStateCount[1]
            + self.crossCheckStateCount[2];
        if 5 * (stateCountTotal as i64 - originalStateCountTotal as i64).unsigned_abs() as u32
            >= 2 * originalStateCountTotal
        {
            return f32::NAN;
        }

        if self.foundPatternCross(&self.crossCheckStateCount) {
            Self::centerFromEnd(&self.crossCheckStateCount, i as u32)
        } else {
            f32::NAN
        }
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
    fn handlePossibleCenter(
        &mut self,
        stateCount: &[u32],
        i: u32,
        j: u32,
    ) -> Option<AlignmentPattern> {
        let stateCountTotal = stateCount[0] + stateCount[1] + stateCount[2];
        let centerJ = Self::centerFromEnd(stateCount, j);
        let centerI = self.crossCheckVertical(
            i,
            centerJ.floor() as u32,
            2 * stateCount[1],
            stateCountTotal,
        );

        if !centerI.is_nan() {
            let estimatedModuleSize = (stateCount[0] + stateCount[1] + stateCount[2]) as f32 / 3.0;
            for center in &self.possibleCenters {
                // Look for about the same center and module size:
                if center.aboutEquals(estimatedModuleSize, centerI, centerJ) {
                    return Some(center.combineEstimate(centerI, centerJ, estimatedModuleSize));
                }
            }
            // Hadn't found this before; save it
            let point = AlignmentPattern::new(centerJ, centerI, estimatedModuleSize);
            if let Some(rpc) = self.resultPointCallback.clone() {
                rpc(&point);
            }

            self.possibleCenters.push(point);
        }

        None
    }
}
