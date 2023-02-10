/*
 * Copyright 2009 ZXing authors
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

use std::cmp::Ordering;

use crate::{
    common::BitMatrix,
    qrcode::detector::{FinderPattern, FinderPatternFinder, FinderPatternInfo},
    result_point_utils, DecodeHintType, DecodingHintDictionary, Exceptions,
    RXingResultPointCallback,
};

// max. legal count of modules per QR code edge (177)
const MAX_MODULE_COUNT_PER_EDGE: f32 = 180_f32;
// min. legal count per modules per QR code edge (11)
const MIN_MODULE_COUNT_PER_EDGE: f32 = 9_f32;

/**
 * More or less arbitrary cutoff point for determining if two finder patterns might belong
 * to the same code if they differ less than DIFF_MODSIZE_CUTOFF_PERCENT percent in their
 * estimated modules sizes.
 */
const DIFF_MODSIZE_CUTOFF_PERCENT: f32 = 0.05_f32;

/**
 * More or less arbitrary cutoff point for determining if two finder patterns might belong
 * to the same code if they differ less than DIFF_MODSIZE_CUTOFF pixels/module in their
 * estimated modules sizes.
 */
const DIFF_MODSIZE_CUTOFF: f32 = 0.5_f32;

/**
 * <p>This class attempts to find finder patterns in a QR Code. Finder patterns are the square
 * markers at three corners of a QR Code.</p>
 *
 * <p>This class is thread-safe but not reentrant. Each thread must allocate its own object.
 *
 * <p>In contrast to {@link FinderPatternFinder}, this class will return an array of all possible
 * QR code locations in the image.</p>
 *
 * <p>Use the TRY_HARDER hint to ask for a more thorough detection.</p>
 *
 * @author Sean Owen
 * @author Hannes Erven
 */
pub struct MultiFinderPatternFinder<'a>(FinderPatternFinder<'a>);

impl<'a> MultiFinderPatternFinder<'_> {
    // private static final FinderPatternInfo[] EMPTY_RESULT_ARRAY = new FinderPatternInfo[0];
    // private static final FinderPattern[] EMPTY_FP_ARRAY = new FinderPattern[0];
    // private static final FinderPattern[][] EMPTY_FP_2D_ARRAY = new FinderPattern[0][];

    // TODO MIN_MODULE_COUNT and MAX_MODULE_COUNT would be great hints to ask the user for
    // since it limits the number of regions to decode

    pub fn new(
        image: &'a BitMatrix,
        resultPointCallback: Option<RXingResultPointCallback>,
    ) -> MultiFinderPatternFinder<'a> {
        MultiFinderPatternFinder(FinderPatternFinder::with_callback(
            image,
            resultPointCallback,
        ))
    }

    /**
     * @return the 3 best {@link FinderPattern}s from our list of candidates. The "best" are
     *         those that have been detected at least 2 times, and whose module
     *         size differs from the average among those patterns the least
     * @throws NotFoundException if 3 such finder patterns do not exist
     */
    fn selectMultipleBestPatterns(&self) -> Result<Vec<[FinderPattern; 3]>, Exceptions> {
        let mut possibleCenters = Vec::new();
        for fp in self.0.getPossibleCenters() {
            if fp.getCount() >= 2 {
                possibleCenters.push(*fp);
            }
        }
        let size = possibleCenters.len();

        if size < 3 {
            // Couldn't find enough finder patterns
            return Err(Exceptions::NotFoundException(Some(
                "Couldn't find enough finder patterns".to_owned(),
            )));
        }

        /*
         * Begin HE modifications to safely detect multiple codes of equal size
         */
        if size == 3 {
            return Ok(vec![[
                possibleCenters[0],
                possibleCenters[1],
                possibleCenters[2],
            ]]);
        }

        // Sort by estimated module size to speed up the upcoming checks
        possibleCenters.sort_by(compare_finder_patterns);
        // Collections.sort(possibleCenters, new ModuleSizeComparator());

        /*
         * Now lets start: build a list of tuples of three finder locations that
         *  - feature similar module sizes
         *  - are placed in a distance so the estimated module count is within the QR specification
         *  - have similar distance between upper left/right and left top/bottom finder patterns
         *  - form a triangle with 90° angle (checked by comparing top right/bottom left distance
         *    with pythagoras)
         *
         * Note: we allow each point to be used for more than one code region: this might seem
         * counterintuitive at first, but the performance penalty is not that big. At this point,
         * we cannot make a good quality decision whether the three finders actually represent
         * a QR code, or are just by chance laid out so it looks like there might be a QR code there.
         * So, if the layout seems right, lets have the decoder try to decode.
         */

        let mut results = Vec::new(); // holder for the results

        for i1 in 0..(size - 2) {
            let Some(p1) = possibleCenters.get(i1) else {
                continue;
            };

            for i2 in (i1 + 1)..(size - 1) {
                // for (int i2 = i1 + 1; i2 < (size - 1); i2++) {
                let Some(p2) = possibleCenters.get(i2) else {
                    continue;
                };

                // Compare the expected module sizes; if they are really off, skip
                let vModSize12 = (p1.getEstimatedModuleSize() - p2.getEstimatedModuleSize())
                    / p1.getEstimatedModuleSize().min(p2.getEstimatedModuleSize());
                let vModSize12A = (p1.getEstimatedModuleSize() - p2.getEstimatedModuleSize()).abs();
                if vModSize12A > DIFF_MODSIZE_CUTOFF && vModSize12 >= DIFF_MODSIZE_CUTOFF_PERCENT {
                    // break, since elements are ordered by the module size deviation there cannot be
                    // any more interesting elements for the given p1.
                    break;
                }

                for i3 in (i2 + 1)..size {
                    // for (int i3 = i2 + 1; i3 < size; i3++) {
                    let Some( p3) = possibleCenters.get(i3) else {
                        continue;
                    };

                    // Compare the expected module sizes; if they are really off, skip
                    let vModSize23 = (p2.getEstimatedModuleSize() - p3.getEstimatedModuleSize())
                        / p2.getEstimatedModuleSize().min(p3.getEstimatedModuleSize());
                    let vModSize23A =
                        (p2.getEstimatedModuleSize() - p3.getEstimatedModuleSize()).abs();
                    if vModSize23A > DIFF_MODSIZE_CUTOFF
                        && vModSize23 >= DIFF_MODSIZE_CUTOFF_PERCENT
                    {
                        // break, since elements are ordered by the module size deviation there cannot be
                        // any more interesting elements for the given p1.
                        break;
                    }

                    let mut test = [*p1, *p2, *p3];
                    result_point_utils::orderBestPatterns(&mut test);

                    // Calculate the distances: a = topleft-bottomleft, b=topleft-topright, c = diagonal
                    let info = FinderPatternInfo::new(test);
                    let dA = result_point_utils::distance(info.getTopLeft(), info.getBottomLeft());
                    let dC = result_point_utils::distance(info.getTopRight(), info.getBottomLeft());
                    let dB = result_point_utils::distance(info.getTopLeft(), info.getTopRight());

                    // Check the sizes
                    let estimatedModuleCount = (dA + dB) / (p1.getEstimatedModuleSize() * 2.0);
                    if !(MIN_MODULE_COUNT_PER_EDGE..=MAX_MODULE_COUNT_PER_EDGE)
                        .contains(&estimatedModuleCount)
                    {
                        continue;
                    }

                    // Calculate the difference of the edge lengths in percent
                    let vABBC = ((dA - dB) / dA.min(dB)).abs();
                    if vABBC >= 0.1 {
                        continue;
                    }

                    // Calculate the diagonal length by assuming a 90° angle at topleft
                    let dCpy =
                        ((dA as f64) * (dA as f64) + (dB as f64) * (dB as f64)).sqrt() as f32;
                    // Compare to the real distance in %
                    let vPyC = ((dC - dCpy) / dC.min(dCpy)).abs();

                    if vPyC >= 0.1 {
                        continue;
                    }

                    // All tests passed!
                    results.push(test);
                }
            }
        }

        if !results.is_empty() {
            Ok(results)
        } else {
            Err(Exceptions::NotFoundException(None))
        }
    }

    pub fn findMulti(
        &mut self,
        hints: &DecodingHintDictionary,
    ) -> Result<Vec<FinderPatternInfo>, Exceptions> {
        let tryHarder = hints.contains_key(&DecodeHintType::TRY_HARDER);
        let image = self.0.getImage().clone();
        let maxI = image.getHeight();
        let maxJ = image.getWidth();
        // We are looking for black/white/black/white/black modules in
        // 1:1:3:1:1 ratio; this tracks the number of such modules seen so far

        // Let's assume that the maximum version QR Code we support takes up 1/4 the height of the
        // image, and then account for the center being 3 modules in size. This gives the smallest
        // number of pixels the center could be, so skip this often. When trying harder, look for all
        // QR versions regardless of how dense they are.
        let mut iSkip = (3 * maxI) / (4 * FinderPatternFinder::MAX_MODULES);
        if iSkip < FinderPatternFinder::MIN_SKIP || tryHarder {
            iSkip = FinderPatternFinder::MIN_SKIP;
        }

        let mut stateCount = [0_u32; 5]; //new int[5];
        let mut i = iSkip - 1;
        while i < maxI {
            // for (int i = iSkip - 1; i < maxI; i += iSkip) {
            // Get a row of black/white values
            FinderPatternFinder::doClearCounts(&mut stateCount);
            let mut currentState = 0;
            for j in 0..maxJ {
                // for (int j = 0; j < maxJ; j++) {
                if image.get(j, i) {
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
                            if FinderPatternFinder::foundPatternCross(&stateCount)
                                && self.0.handlePossibleCenter(&stateCount, i, j)
                            {
                                // Yes
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
            } // for j=...

            if FinderPatternFinder::foundPatternCross(&stateCount) {
                self.0.handlePossibleCenter(&stateCount, i, maxJ);
            }

            i += iSkip;
        } // for i=iSkip-1 ...
        let mut patternInfo = self.selectMultipleBestPatterns()?;
        let mut result = Vec::new(); //new ArrayList<>();
        for pattern in patternInfo.iter_mut() {
            result_point_utils::orderBestPatterns(pattern);
            result.push(FinderPatternInfo::new(*pattern));
        }

        // if result.isEmpty() {
        //   return EMPTY_RESULT_ARRAY;
        // } else {
        //   return result.toArray(EMPTY_RESULT_ARRAY);
        // }
        Ok(result)
    }
}

/**
 * A comparator that orders FinderPatterns by their estimated module size.
 */
// private static final class ModuleSizeComparator implements Comparator<FinderPattern>, Serializable {
// @Override
fn compare_finder_patterns(center1: &FinderPattern, center2: &FinderPattern) -> Ordering {
    let value = center2.getEstimatedModuleSize() - center1.getEstimatedModuleSize();
    if value < 0.0 {
        Ordering::Less
    } else if value > 0.0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
    // return value < 0.0 ? -1 : value > 0.0 ? 1 : 0;
}
// }
