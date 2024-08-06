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
    common::{BitArray, Result}, point_f, Binarizer, BinaryBitmap, DecodeHintType, DecodeHintValue, DecodeHints, DecodingHintDictionary, Exceptions, LuminanceSource, RXingResult, RXingResultMetadataType, RXingResultMetadataValue, Reader
};

/**
 * Encapsulates functionality and implementation that is common to all families
 * of one-dimensional barcodes.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
pub trait OneDReader: Reader {
    const QUIET_ZONE: usize = 15;
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
    fn _do_decode<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> Result<RXingResult> {
        let mut hints = hints.clone();
        let width = image.get_width();
        let height = image.get_height();

        let try_harder = hints.TryHarder.unwrap_or(false);

        let try_pure = hints.PureBarcode.unwrap_or(false);

        // Attempt to decode the barcode as "pure". This method may be very inneficient and uses
        // a very poor version of a binarizer.
        // ToDo: Add a better binarizer for pure barcodes
        if try_pure {
            let mid_line = 1.max(image.get_height() / 2);

            let rw = image.get_source().get_row(mid_line);

            let decoded = self.decode_pure(mid_line as u32, &rw, &hints);
            if decoded.is_ok() {
                return decoded;
            }
        }

        let row_step = 1.max(height >> (if try_harder { 8 } else { 5 }));
        let max_lines = if try_harder {
            height // Look at the whole image, not just the center
        } else {
            15 // 15 rows spaced 1/32 apart is roughly the middle half of the image
        };

        let middle = height / 2;
        for x in 0..max_lines {
            // Scanning from the middle out. Determine which row we're looking at next:
            let row_steps_above_or_below = (x + 1) / 2;
            let is_above = (x & 0x01) == 0; // i.e. is x even?
            let row_number: isize = middle as isize
                + row_step as isize
                    * (if is_above {
                        row_steps_above_or_below as isize
                    } else {
                        -(row_steps_above_or_below as isize)
                    });
            if row_number < 0 || row_number >= height as isize {
                // Oops, if we run off the top or bottom, stop
                break;
            }

            // Estimate black point for this row and load it:
            let mut row = if let Ok(res) = image.get_black_row(row_number as usize) {
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

                    hints.NeedResultPointCallback = None;
                }
                let Ok(mut result) = self.decode_row(row_number as u32, &row, &hints) else {
                    continue;
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
                        points[0] = point_f(width as f32 - points[0].x - 1.0, points[0].y);
                        points[1] = point_f(width as f32 - points[1].x - 1.0, points[1].y);
                    }
                }
                return Ok(result);
            }
        }

        Err(Exceptions::NOT_FOUND)
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
    fn decode_row(
        &mut self,
        rowNumber: u32,
        row: &BitArray,
        hints: &DecodeHints,
    ) -> Result<RXingResult>;

    fn decode_pure(
        &mut self,
        rowNumber: u32,
        row: &[u8],
        hints: &DecodeHints,
    ) -> Result<RXingResult> {
        let new_row = pad_bitarray(row, Self::QUIET_ZONE);

        self.decode_row(rowNumber, &new_row, hints)
    }
}

// Add a buffer on either side of the row to mimic a quiet zone. This may not exist in a "pure barcode"
fn pad_bitarray(bits: &[u8], quiet_zone: usize) -> BitArray {
    const PIXEL_COLOR_SPLIT_POINT: u8 = u8::MAX / 2;

    let mut new_row = BitArray::with_capacity(bits.len() + (quiet_zone * 2));

    let value = bits[0] >= PIXEL_COLOR_SPLIT_POINT;

    for _ in 0..quiet_zone {
        new_row.appendBit(value);
    }

    for bit in bits {
        new_row.appendBit(bit < &PIXEL_COLOR_SPLIT_POINT)
    }

    for _ in 0..quiet_zone {
        new_row.appendBit(value);
    }

    new_row
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
pub fn pattern_match_variance(
    counters: &[u32],
    pattern: &[u32],
    mut max_individual_variance: f32,
) -> f32 {
    let num_counters = counters.len();
    let mut total = 0.0;
    let mut pattern_length = 0;
    for i in 0..num_counters {
        total += counters[i] as f32;
        pattern_length += pattern[i];
    }
    if total < pattern_length as f32 {
        // If we don't even have one pixel per unit of bar width, assume this is too small
        // to reliably match, so fail:
        return f32::INFINITY;
    }

    let unit_bar_width = total / pattern_length as f32;
    max_individual_variance *= unit_bar_width;

    let mut total_variance = 0.0;
    for x in 0..num_counters {
        let counter = counters[x];
        let scaled_pattern = (pattern[x] as f32) * unit_bar_width;
        let variance = if (counter as f32) > scaled_pattern {
            counter as f32 - scaled_pattern
        } else {
            scaled_pattern - counter as f32
        };
        if variance > max_individual_variance {
            return f32::INFINITY;
        }
        total_variance += variance;
    }
    total_variance / total
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
pub fn record_pattern(row: &BitArray, start: usize, counters: &mut [u32]) -> Result<()> {
    let num_counters = counters.len();
    counters.fill(0);

    let end = row.get_size();
    if start >= end {
        return Err(Exceptions::NOT_FOUND);
    }

    let mut is_white = !row.get(start);
    let mut counter_position = 0;
    let mut i = start;
    while i < end {
        if row.get(i) != is_white {
            counters[counter_position] += 1;
        } else {
            counter_position += 1;
            if counter_position == num_counters {
                break;
            } else {
                counters[counter_position] = 1;
                is_white = !is_white;
            }
        }
        i += 1;
    }
    // If we read fully the last section of pixels and filled up our counters -- or filled
    // the last counter but ran off the side of the image, OK. Otherwise, a problem.
    if !(counter_position == num_counters || (counter_position == num_counters - 1 && i == end)) {
        return Err(Exceptions::NOT_FOUND);
    }
    Ok(())
}

pub fn record_pattern_in_reverse(row: &BitArray, start: usize, counters: &mut [u32]) -> Result<()> {
    let mut start = start;
    // This could be more efficient I guess
    let mut num_transitions_left = counters.len() as isize;
    let mut last = row.get(start);
    while start > 0 && num_transitions_left >= 0 {
        start -= 1;
        if row.get(start) != last {
            num_transitions_left -= 1;
            last = !last;
        }
    }
    if num_transitions_left >= 0 {
        return Err(Exceptions::NOT_FOUND);
    }
    record_pattern(row, start + 1, counters)?;

    Ok(())
}
