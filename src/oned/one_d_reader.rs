/*
 * Copyright 2008 ZXing authors
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
    common::{BitArray, Result},
    point, BinaryBitmap, DecodeHintType, DecodeHintValue, DecodingHintDictionary, Exceptions,
    RXingResult, RXingResultMetadataType, RXingResultMetadataValue, Reader,
};

/**
 * Encapsulates functionality and implementation that is common to all families
 * of one-dimensional barcodes.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
pub trait OneDReader: Reader {
    /**
     * We're going to examine rows from the middle outward, searching alternately above and below the
     * middle, and farther out each time. rowStep is the number of rows between each successive
     * attempt above and below the middle. So we'd scan row middle, then middle - rowStep, then
     * middle + rowStep, then middle - (2 * rowStep), etc.
     * rowStep is bigger as the image is taller, but is always at least 1. We've somewhat arbitrarily
     * decided that moving up and down by about 1/16 of the image is pretty good; we try more of the
     * image if "trying harder".
     *
     * @param image The image to decode
     * @param hints Any hints that were requested
     * @return The contents of the decoded barcode
     * @throws NotFoundException Any spontaneous errors which occur
     */
    fn doDecode(
        &mut self,
        image: &mut BinaryBitmap,
        hints: &DecodingHintDictionary,
    ) -> Result<RXingResult> {
        let mut hints = hints.clone();
        let width = image.getWidth();
        let height = image.getHeight();

        let tryHarder = matches!(
            hints.get(&DecodeHintType::TRY_HARDER),
            Some(DecodeHintValue::TryHarder(true))
        );
        let rowStep = 1.max(height >> (if tryHarder { 8 } else { 5 }));
        let maxLines = if tryHarder {
            height // Look at the whole image, not just the center
        } else {
            15 // 15 rows spaced 1/32 apart is roughly the middle half of the image
        };

        let middle = height / 2;
        for x in 0..maxLines {
            // Scanning from the middle out. Determine which row we're looking at next:
            let rowStepsAboveOrBelow = (x + 1) / 2;
            let isAbove = (x & 0x01) == 0; // i.e. is x even?
            let rowNumber: isize = middle as isize
                + rowStep as isize
                    * (if isAbove {
                        rowStepsAboveOrBelow as isize
                    } else {
                        -(rowStepsAboveOrBelow as isize)
                    });
            if rowNumber < 0 || rowNumber >= height as isize {
                // Oops, if we run off the top or bottom, stop
                break;
            }

            // Estimate black point for this row and load it:
            let mut row = if let Ok(res) = image.getBlackRow(rowNumber as usize) {
                res
            } else {
                continue;
            };

            // While we have the image data in a BitArray, it's fairly cheap to reverse it in place to
            // handle decoding upside down barcodes.
            for attempt in 0..2 {
                // for (int attempt = 0; attempt < 2; attempt++) {
                if attempt == 1 {
                    // trying again?

                    // reverse the row and continue
                    // This means we will only ever draw result points *once* in the life of this method
                    // since we want to avoid drawing the wrong points after flipping the row, and,
                    // don't want to clutter with noise from every single row scan -- just the scans
                    // that start on the center line.
                    row.to_mut().reverse();

                    if hints.contains_key(&DecodeHintType::NEED_RESULT_POINT_CALLBACK) {
                        hints.remove(&DecodeHintType::NEED_RESULT_POINT_CALLBACK);
                    }
                }
                let Ok(mut result) = self.decodeRow(rowNumber as u32, &row, &hints) else {
            continue
          };
                // We found our barcode
                if attempt == 1 {
                    // But it was upside down, so note that
                    result.putMetadata(
                        RXingResultMetadataType::ORIENTATION,
                        RXingResultMetadataValue::Orientation(180),
                    );
                    // And remember to flip the result points horizontally.
                    let points = result.getPointsMut();
                    if !points.is_empty() && points.len() >= 2 {
                        points[0] = point(width as f32 - points[0].x - 1.0, points[0].y);
                        points[1] = point(width as f32 - points[1].x - 1.0, points[1].y);
                    }
                }
                return Ok(result);
            }
        }

        Err(Exceptions::notFound)
    }

    /**
     * <p>Attempts to decode a one-dimensional barcode format given a single row of
     * an image.</p>
     *
     * @param rowNumber row number from top of the row
     * @param row the black/white pixel data of the row
     * @param hints decode hints
     * @return {@link RXingResult} containing encoded string and start/end of barcode
     * @throws NotFoundException if no potential barcode is found
     * @throws ChecksumException if a potential barcode is found but does not pass its checksum
     * @throws FormatException if a potential barcode is found but format is invalid
     */
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &BitArray,
        hints: &DecodingHintDictionary,
    ) -> Result<RXingResult>;
}

/**
 * Determines how closely a set of observed counts of runs of black/white values matches a given
 * target pattern. This is reported as the ratio of the total variance from the expected pattern
 * proportions across all pattern elements, to the length of the pattern.
 *
 * @param counters observed counters
 * @param pattern expected pattern
 * @param maxIndividualVariance The most any counter can differ before we give up
 * @return ratio of total variance between counters and pattern compared to total pattern size
 */
pub fn patternMatchVariance(counters: &[u32], pattern: &[u32], maxIndividualVariance: f32) -> f32 {
    let mut maxIndividualVariance = maxIndividualVariance;
    let numCounters = counters.len();
    let mut total = 0.0;
    let mut patternLength = 0;
    for i in 0..numCounters {
        total += counters[i] as f32;
        patternLength += pattern[i];
    }
    if total < patternLength as f32 {
        // If we don't even have one pixel per unit of bar width, assume this is too small
        // to reliably match, so fail:
        return f32::INFINITY;
    }

    let unitBarWidth = total / patternLength as f32;
    maxIndividualVariance *= unitBarWidth;

    let mut totalVariance = 0.0;
    for x in 0..numCounters {
        let counter = counters[x];
        let scaledPattern = (pattern[x] as f32) * unitBarWidth;
        let variance = if (counter as f32) > scaledPattern {
            counter as f32 - scaledPattern
        } else {
            scaledPattern - counter as f32
        };
        if variance > maxIndividualVariance {
            return f32::INFINITY;
        }
        totalVariance += variance;
    }
    totalVariance / total
}

/**
 * Records the size of successive runs of white and black pixels in a row, starting at a given point.
 * The values are recorded in the given array, and the number of runs recorded is equal to the size
 * of the array. If the row starts on a white pixel at the given start point, then the first count
 * recorded is the run of white pixels starting from that point; likewise it is the count of a run
 * of black pixels if the row begin on a black pixels at that point.
 *
 * @param row row to count from
 * @param start offset into row to start at
 * @param counters array into which to record counts
 * @throws NotFoundException if counters cannot be filled entirely from row before running out
 *  of pixels
 */
pub fn recordPattern(row: &BitArray, start: usize, counters: &mut [u32]) -> Result<()> {
    let numCounters = counters.len();
    counters.fill(0);

    let end = row.getSize();
    if start >= end {
        return Err(Exceptions::notFound);
    }

    let mut isWhite = !row.get(start);
    let mut counterPosition = 0;
    let mut i = start;
    while i < end {
        if row.get(i) != isWhite {
            counters[counterPosition] += 1;
        } else {
            counterPosition += 1;
            if counterPosition == numCounters {
                break;
            } else {
                counters[counterPosition] = 1;
                isWhite = !isWhite;
            }
        }
        i += 1;
    }
    // If we read fully the last section of pixels and filled up our counters -- or filled
    // the last counter but ran off the side of the image, OK. Otherwise, a problem.
    if !(counterPosition == numCounters || (counterPosition == numCounters - 1 && i == end)) {
        return Err(Exceptions::notFound);
    }
    Ok(())
}

pub fn recordPatternInReverse(row: &BitArray, start: usize, counters: &mut [u32]) -> Result<()> {
    let mut start = start;
    // This could be more efficient I guess
    let mut numTransitionsLeft = counters.len() as isize;
    let mut last = row.get(start);
    while start > 0 && numTransitionsLeft >= 0 {
        start -= 1;
        if row.get(start) != last {
            numTransitionsLeft -= 1;
            last = !last;
        }
    }
    if numTransitionsLeft >= 0 {
        return Err(Exceptions::notFound);
    }
    recordPattern(row, start + 1, counters)?;

    Ok(())
}
