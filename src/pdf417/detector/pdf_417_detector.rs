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

use crate::{
    common::{BitMatrix, Result},
    point_f, Binarizer, BinaryBitmap, DecodeHints, Exceptions, Point,
};

use std::borrow::Cow;

use super::PDF417DetectorRXingResult;

/*
 * <p>Encapsulates logic that can detect a PDF417 Code in an image, even if the
 * PDF417 Code is rotated or skewed, or partially obscured.</p>
 *
 * @author SITA Lab (kevin.osullivan@sita.aero)
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Guenther Grau
 */

const INDEXES_START_PATTERN: [u32; 4] = [0, 4, 1, 5];
const INDEXES_STOP_PATTERN: [u32; 4] = [6, 2, 7, 3];
const MAX_AVG_VARIANCE: f64 = 0.42;
const MAX_INDIVIDUAL_VARIANCE: f64 = 0.8;

// B S B S B S B S Bar/Space pattern
// 11111111 0 1 0 1 0 1 000
const START_PATTERN: [u32; 8] = [8, 1, 1, 1, 1, 1, 1, 3];
// 1111111 0 1 000 1 0 1 00 1
const STOP_PATTERN: [u32; 9] = [7, 1, 1, 3, 1, 1, 1, 2, 1];
const MAX_PIXEL_DRIFT: u32 = 3;
const MAX_PATTERN_DRIFT: u32 = 5;
// if we set the value too low, then we don't detect the correct height of the bar if the start patterns are damaged.
// if we set the value too high, then we might detect the start pattern from a neighbor barcode.
const SKIPPED_ROW_COUNT_MAX: u32 = 25;
// A PDF471 barcode should have at least 3 rows, with each row being >= 3 times the module width.
// Therefore it should be at least 9 pixels tall. To be conservative, we use about half the size to
// ensure we don't miss it.
const ROW_STEP: u32 = 5;
const BARCODE_MIN_HEIGHT: u32 = 10;
const ROTATIONS: [u32; 4] = [0, 180, 270, 90];

/**
 * <p>Detects a PDF417 Code in an image. Checks 0, 90, 180, and 270 degree rotations.</p>
 *
 * @param image barcode image to decode
 * @param hints optional hints to detector
 * @param multiple if true, then the image is searched for multiple codes. If false, then at most one code will
 * be found and returned
 * @return {@link PDF417DetectorRXingResult} encapsulating results of detecting a PDF417 code
 * @throws NotFoundException if no PDF417 Code can be found
 */
pub fn detect_with_hints<B: Binarizer>(
    image: &mut BinaryBitmap<B>,
    _hints: &DecodeHints,
    multiple: bool,
) -> Result<PDF417DetectorRXingResult> {
    // TODO detection improvement, tryHarder could try several different luminance thresholds/blackpoints or even
    // different binarizers
    //boolean tryHarder = hints != null && hints.containsKey(DecodeHintType.TRY_HARDER);
    //let try_harder = matches!(hints.get(&DecodeHintType::TRY_HARDER), Some(DecodeHintValue::TryHarder(true)));

    let originalMatrix = image.get_black_matrix();
    for rotation in ROTATIONS {
        // for (int rotation : ROTATIONS) {
        let bitMatrix = applyRotation(originalMatrix, rotation)?;
        let barcodeCoordinates = detect(multiple, &bitMatrix).ok_or(Exceptions::NOT_FOUND)?;
        if !barcodeCoordinates.is_empty() {
            return Ok(PDF417DetectorRXingResult::with_rotation(
                bitMatrix.into_owned(),
                barcodeCoordinates,
                rotation,
            ));
        }
    }
    Ok(PDF417DetectorRXingResult::with_rotation(
        originalMatrix.clone(),
        Vec::new(),
        0,
    ))
}

/**
 * Applies a rotation to the supplied BitMatrix.
 * @param matrix bit matrix to apply rotation to
 * @param rotation the degrees of rotation to apply
 * @return BitMatrix with applied rotation
 */
fn applyRotation(matrix: &BitMatrix, rotation: u32) -> Result<Cow<BitMatrix>> {
    if rotation % 360 == 0 {
        Ok(Cow::Borrowed(matrix))
    } else {
        let mut newMatrix = matrix.clone();
        newMatrix.rotate(rotation)?;
        Ok(Cow::Owned(newMatrix))
    }
}

/**
 * Detects PDF417 codes in an image. Only checks 0 degree rotation
 * @param multiple if true, then the image is searched for multiple codes. If false, then at most one code will
 * be found and returned
 * @param bitMatrix bit matrix to detect barcodes in
 * @return List of Point arrays containing the coordinates of found barcodes
 */
pub fn detect(multiple: bool, bitMatrix: &BitMatrix) -> Option<Vec<[Option<Point>; 8]>> {
    let mut barcodeCoordinates: Vec<[Option<Point>; 8]> = Vec::new();
    let mut row = 0;
    let mut column = 0;
    let mut foundBarcodeInRow = false;
    while row < bitMatrix.getHeight() {
        let vertices = findVertices(bitMatrix, row, column)?;

        if vertices[0].is_none() && vertices[3].is_none() {
            if !foundBarcodeInRow {
                // we didn't find any barcode so that's the end of searching
                break;
            }
            // we didn't find a barcode starting at the given column and row. Try again from the first column and slightly
            // below the lowest barcode we found so far.
            foundBarcodeInRow = false;
            column = 0;
            for barcodeCoordinate in &barcodeCoordinates {
                if let Some(coord_1) = barcodeCoordinate[1] {
                    row = row.max(coord_1.y as u32);
                }
                if let Some(coord_3) = barcodeCoordinate[3] {
                    row = row.max(coord_3.y as u32);
                }
            }
            row += ROW_STEP;
            continue;
        }
        foundBarcodeInRow = true;
        barcodeCoordinates.push(vertices);
        if !multiple {
            break;
        }
        // if we didn't find a right row indicator column, then continue the search for the next barcode after the
        // start pattern of the barcode just found.
        if let Some(vert_2) = vertices[2] {
            column = vert_2.x as u32;
            row = vert_2.y as u32;
        } else {
            column = vertices[4].as_ref().unwrap().x as u32;
            row = vertices[4].as_ref().unwrap().y as u32;
        }
    }
    Some(barcodeCoordinates)
}

/**
 * Locate the vertices and the codewords area of a black blob using the Start
 * and Stop patterns as locators.
 *
 * @param matrix the scanned barcode image.
 * @return an array containing the vertices:
 *           vertices[0] x, y top left barcode
 *           vertices[1] x, y bottom left barcode
 *           vertices[2] x, y top right barcode
 *           vertices[3] x, y bottom right barcode
 *           vertices[4] x, y top left codeword area
 *           vertices[5] x, y bottom left codeword area
 *           vertices[6] x, y top right codeword area
 *           vertices[7] x, y bottom right codeword area
 */
fn findVertices(matrix: &BitMatrix, startRow: u32, startColumn: u32) -> Option<[Option<Point>; 8]> {
    let height = matrix.getHeight();
    let width = matrix.getWidth();
    let mut startRow = startRow;
    let mut startColumn = startColumn;

    let mut result = [None::<Point>; 8]; //Point[8];
    copyToRXingResult(
        &mut result,
        &findRowsWithPattern(matrix, height, width, startRow, startColumn, &START_PATTERN)?,
        &INDEXES_START_PATTERN,
    );

    if let Some(result_4) = result[4] {
        startColumn = result_4.x as u32;
        startRow = result_4.y as u32;
    }
    copyToRXingResult(
        &mut result,
        &findRowsWithPattern(matrix, height, width, startRow, startColumn, &STOP_PATTERN)?,
        &INDEXES_STOP_PATTERN,
    );

    Some(result)
}

fn copyToRXingResult(
    result: &mut [Option<Point>],
    tmpRXingResult: &[Option<Point>],
    destinationIndexes: &[u32],
) {
    for i in 0..destinationIndexes.len() {
        result[destinationIndexes[i] as usize] = tmpRXingResult[i];
    }
}

fn findRowsWithPattern(
    matrix: &BitMatrix,
    height: u32,
    width: u32,
    startRow: u32,
    startColumn: u32,
    pattern: &[u32],
) -> Option<[Option<Point>; 4]> {
    let mut startRow = startRow;
    let mut result = [None; 4];
    let mut found = false;
    let mut counters = vec![0_u32; pattern.len()];
    while startRow < height {
        let mut loc_store;
        if let Some(loc) =
            findGuardPattern(matrix, startColumn, startRow, width, pattern, &mut counters)
        {
            loc_store = Some(loc);
            while startRow > 0 {
                startRow -= 1;
                if let Some(previousRowLoc) =
                    findGuardPattern(matrix, startColumn, startRow, width, pattern, &mut counters)
                {
                    loc_store.replace(previousRowLoc);
                    // loc_store = Some(previousRowLoc);
                } else {
                    startRow += 1;
                    break;
                }
            }
            result[0] = Some(point_f(loc_store.as_ref()?[0] as f32, startRow as f32));
            result[1] = Some(point_f(loc_store.as_ref()?[1] as f32, startRow as f32));
            found = true;
            break;
        }

        startRow += ROW_STEP;
    }

    let mut stopRow = startRow + 1;
    // Last row of the current symbol that contains pattern
    if found {
        let mut skippedRowCount = 0;
        let mut previousRowLoc = [result[0].as_ref()?.x as u32, result[1].as_ref()?.x as u32];
        while stopRow < height {
            if let Some(loc) = findGuardPattern(
                matrix,
                previousRowLoc[0],
                stopRow,
                width,
                pattern,
                &mut counters,
            ) {
                // a found pattern is only considered to belong to the same barcode if the start and end positions
                // don't differ too much. Pattern drift should be not bigger than two for consecutive rows. With
                // a higher number of skipped rows drift could be larger. To keep it simple for now, we allow a slightly
                // larger drift and don't check for skipped rows.
                if (previousRowLoc[0] as i32 - loc[0] as i32).unsigned_abs() < MAX_PATTERN_DRIFT
                    && (previousRowLoc[1] as i32 - loc[1] as i32).unsigned_abs() < MAX_PATTERN_DRIFT
                {
                    previousRowLoc = loc;
                    skippedRowCount = 0;
                } else if skippedRowCount > SKIPPED_ROW_COUNT_MAX {
                    break;
                } else {
                    skippedRowCount += 1;
                }
            } else if skippedRowCount > SKIPPED_ROW_COUNT_MAX {
                break;
            } else {
                skippedRowCount += 1;
            }

            stopRow += 1;
        }
        stopRow -= skippedRowCount + 1;
        result[2] = Some(point_f(previousRowLoc[0] as f32, stopRow as f32));
        result[3] = Some(point_f(previousRowLoc[1] as f32, stopRow as f32));
    }
    if stopRow - startRow < BARCODE_MIN_HEIGHT {
        result.fill(None);
    }

    Some(result)
}

/**
 * @param matrix row of black/white values to search
 * @param column x position to start search
 * @param row y position to start search
 * @param width the number of pixels to search on this row
 * @param pattern pattern of counts of number of black and white pixels that are
 *                 being searched for as a pattern
 * @param counters array of counters, as long as pattern, to re-use
 * @return start/end horizontal offset of guard pattern, as an array of two ints.
 */
fn findGuardPattern(
    matrix: &BitMatrix,
    column: u32,
    row: u32,
    width: u32,
    pattern: &[u32],
    counters: &mut [u32],
) -> Option<[u32; 2]> {
    counters.fill(0);
    let mut patternStart = column;
    let mut pixelDrift = 0;

    // if there are black pixels left of the current pixel shift to the left, but only for MAX_PIXEL_DRIFT pixels
    while matrix.get(patternStart, row) && patternStart > 0 && pixelDrift < MAX_PIXEL_DRIFT {
        pixelDrift += 1;
        patternStart -= 1;
    }
    let mut x = patternStart;
    let mut counterPosition = 0;
    let patternLength = pattern.len();
    let mut isWhite = false;
    while x < width {
        // for (boolean isWhite = false; x < width; x++) {
        let pixel = matrix.get(x, row);
        if pixel != isWhite {
            counters[counterPosition] += 1;
        } else {
            if counterPosition == patternLength - 1 {
                if patternMatchVariance(counters, pattern) < MAX_AVG_VARIANCE {
                    return Some([patternStart, x]);
                }
                patternStart += counters[0] + counters[1];

                counters.copy_within(2..counterPosition - 1 + 2, 0);
                // System.arraycopy(counters, 2, counters, 0, counterPosition - 1);

                counters[counterPosition - 1] = 0;
                counters[counterPosition] = 0;
                counterPosition -= 1;
            } else {
                counterPosition += 1;
            }
            counters[counterPosition] = 1;
            isWhite = !isWhite;
        }
        x += 1;
    }
    if counterPosition == patternLength - 1
        && patternMatchVariance(counters, pattern) < MAX_AVG_VARIANCE
    {
        return Some([patternStart, x - 1]);
    }

    None
}

/**
 * Determines how closely a set of observed counts of runs of black/white
 * values matches a given target pattern. This is reported as the ratio of
 * the total variance from the expected pattern proportions across all
 * pattern elements, to the length of the pattern.
 *
 * @param counters observed counters
 * @param pattern expected pattern
 * @return ratio of total variance between counters and pattern compared to total pattern size
 */
fn patternMatchVariance(counters: &[u32], pattern: &[u32]) -> f64 {
    let numCounters = counters.len();
    let total = counters.iter().take(numCounters).sum::<u32>();
    let patternLength = pattern.iter().take(numCounters).sum::<u32>();
    // for i in 0..numCounters {
    //     total += counters[i];
    //     patternLength += pattern[i];
    // }
    if total < patternLength {
        // If we don't even have one pixel per unit of bar width, assume this
        // is too small to reliably match, so fail:
        return f64::INFINITY; //Float.POSITIVE_INFINITY;
    }
    // We're going to fake floating-point math in integers. We just need to use more bits.
    // Scale up patternLength so that intermediate values below like scaledCounter will have
    // more "significant digits".
    let unitBarWidth: f64 = total as f64 / patternLength as f64;
    let maxIndividualVariance = MAX_INDIVIDUAL_VARIANCE * unitBarWidth;

    let mut totalVariance = 0.0;
    for x in 0..numCounters {
        let counter = counters[x];
        let scaledPattern: f64 = pattern[x] as f64 * unitBarWidth;
        let variance: f64 = if counter as f64 > scaledPattern {
            counter as f64 - scaledPattern
        } else {
            scaledPattern - counter as f64
        };
        if variance > maxIndividualVariance {
            return f64::INFINITY;
        }
        totalVariance += variance;
    }
    totalVariance / total as f64
}
