/*
 * Copyright 2013 ZXing authors
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

use std::rc::Rc;

use crate::{
    common::{BitMatrix, DecoderRXingResult},
    pdf417::pdf_417_common,
    Exceptions, RXingResultPoint, ResultPoint,
};

use super::{
    decoded_bit_stream_parser, ec, pdf_417_codeword_decoder, BarcodeMetadata, BarcodeValue,
    BoundingBox, Codeword, DetectionRXingResult, DetectionRXingResultColumn,
    DetectionRXingResultColumnTrait, DetectionRXingResultRowIndicatorColumn,
};

/**
 * @author Guenther Grau
 */

const CODEWORD_SKEW_SIZE: u32 = 2;

const MAX_ERRORS: u32 = 3;
const MAX_EC_CODEWORDS: u32 = 512;
// const  errorCorrection:ErrorCorrection =  ErrorCorrection::new();

// TODO don't pass in minCodewordWidth and maxCodewordWidth, pass in barcode columns for start and stop pattern
// columns. That way width can be deducted from the pattern column.
// This approach also allows to detect more details about the barcode, e.g. if a bar type (white or black) is wider
// than it should be. This can happen if the scanner used a bad blackpoint.
pub fn decode(
    image: &BitMatrix,
    imageTopLeft: Option<RXingResultPoint>,
    imageBottomLeft: Option<RXingResultPoint>,
    imageTopRight: Option<RXingResultPoint>,
    imageBottomRight: Option<RXingResultPoint>,
    minCodewordWidth: u32,
    maxCodewordWidth: u32,
) -> Result<DecoderRXingResult, Exceptions> {
    let mut minCodewordWidth = minCodewordWidth;
    let mut maxCodewordWidth = maxCodewordWidth;
    let mut boundingBox = Rc::new(BoundingBox::new(
        Rc::new(image.clone()),
        imageTopLeft,
        imageBottomLeft,
        imageTopRight,
        imageBottomRight,
    )?);
    let mut leftRowIndicatorColumn = None;
    let mut rightRowIndicatorColumn = None;
    let mut detectionRXingResult = None;
    for firstPass in [true, false] {
        if imageTopLeft.is_some() {
            leftRowIndicatorColumn = Some(getRowIndicatorColumn(
                image,
                boundingBox.clone(),
                imageTopLeft.unwrap(),
                true,
                minCodewordWidth,
                maxCodewordWidth,
            ));
        }
        if imageTopRight.is_some() {
            rightRowIndicatorColumn = Some(getRowIndicatorColumn(
                image,
                boundingBox.clone(),
                imageTopRight.unwrap(),
                false,
                minCodewordWidth,
                maxCodewordWidth,
            ));
        }
        detectionRXingResult = merge(&mut leftRowIndicatorColumn, &mut rightRowIndicatorColumn)?;
        if detectionRXingResult.is_none() {
            return Err(Exceptions::NotFoundException(None));
        }
        // detectionRXingResult = detectionRXingResult;

        let resultBox = detectionRXingResult.as_ref().unwrap().getBoundingBox();
        if firstPass
            && (resultBox.getMinY() < boundingBox.getMinY()
                || resultBox.getMaxY() > boundingBox.getMaxY())
        // if firstPass && resultBox.is_some() &&
        {
            boundingBox = resultBox.clone();
        } else {
            break;
        }
    }
    let mut detectionRXingResult = detectionRXingResult.unwrap();

    let leftToRight = leftRowIndicatorColumn.is_some();

    detectionRXingResult.setBoundingBox(boundingBox.clone());
    let maxBarcodeColumn = detectionRXingResult.getBarcodeColumnCount() + 1;
    detectionRXingResult.setDetectionRXingResultColumn(0, leftRowIndicatorColumn);
    detectionRXingResult.setDetectionRXingResultColumn(maxBarcodeColumn, rightRowIndicatorColumn);

    // let leftToRight = leftRowIndicatorColumn.is_some();
    for barcodeColumnCount in 1..=maxBarcodeColumn {
        // for (int barcodeColumnCount = 1; barcodeColumnCount <= maxBarcodeColumn; barcodeColumnCount++) {
        let barcodeColumn = if leftToRight {
            barcodeColumnCount
        } else {
            maxBarcodeColumn - barcodeColumnCount
        };
        if detectionRXingResult
            .getDetectionRXingResultColumn(barcodeColumn)
            .is_some()
        {
            // This will be the case for the opposite row indicator column, which doesn't need to be decoded again.
            continue;
        }
        let detectionRXingResultColumn = if barcodeColumn == 0 || barcodeColumn == maxBarcodeColumn
        {
            DetectionRXingResultColumn::new_with_is_left(boundingBox.clone(), barcodeColumn == 0)
        } else {
            DetectionRXingResultColumn::new(boundingBox.clone())
        };

        detectionRXingResult
            .setDetectionRXingResultColumn(barcodeColumn, Some(detectionRXingResultColumn));

        let mut startColumn: i32 = -1;
        let mut previousStartColumn = startColumn;
        // TODO start at a row for which we know the start position, then detect upwards and downwards from there.
        for imageRow in boundingBox.getMinY()..=boundingBox.getMaxY() {
            // for (int imageRow = boundingBox.getMinY(); imageRow <= boundingBox.getMaxY(); imageRow++) {
            startColumn =
                getStartColumn(&detectionRXingResult, barcodeColumn, imageRow, leftToRight)
                    .ok_or(Exceptions::IllegalStateException(None))? as i32;
            if startColumn < 0 || startColumn > boundingBox.getMaxX() as i32 {
                if previousStartColumn == -1 {
                    continue;
                }
                startColumn = previousStartColumn;
            }
            let codeword = detectCodeword(
                image,
                boundingBox.getMinX(),
                boundingBox.getMaxX(),
                leftToRight,
                startColumn as u32,
                imageRow,
                minCodewordWidth,
                maxCodewordWidth,
            );
            if let Some(codeword) = codeword {
                // let codeword = codeword.unwrap();
                //detectionRXingResultColumn.setCodeword(imageRow, codeword);
                detectionRXingResult
                    .getDetectionRXingResultColumnMut(barcodeColumn)
                    .as_mut()
                    .unwrap()
                    .setCodeword(imageRow, codeword);
                previousStartColumn = startColumn;
                minCodewordWidth = minCodewordWidth.min(codeword.getWidth());
                maxCodewordWidth = maxCodewordWidth.max(codeword.getWidth());
            }
        }
    }

    createDecoderRXingResult(&mut detectionRXingResult)
}

fn merge<'a, T: DetectionRXingResultRowIndicatorColumn>(
    leftRowIndicatorColumn: &'a mut Option<T>,
    rightRowIndicatorColumn: &'a mut Option<T>,
) -> Result<Option<DetectionRXingResult>, Exceptions> {
    if leftRowIndicatorColumn.is_none() && rightRowIndicatorColumn.is_none() {
        return Ok(None);
    }
    let barcodeMetadata = getBarcodeMetadata(leftRowIndicatorColumn, rightRowIndicatorColumn);
    if barcodeMetadata.is_none() {
        return Ok(None);
    }
    let boundingBox = Rc::new(BoundingBox::merge(
        adjustBoundingBox(leftRowIndicatorColumn)?,
        adjustBoundingBox(rightRowIndicatorColumn)?,
    )?);

    Ok(Some(DetectionRXingResult::new(
        barcodeMetadata.unwrap(),
        boundingBox,
    )))
}

fn adjustBoundingBox<T: DetectionRXingResultRowIndicatorColumn>(
    rowIndicatorColumn: &mut Option<T>,
) -> Result<Option<BoundingBox>, Exceptions> {
    if rowIndicatorColumn.is_none() {
        return Ok(None);
    }
    let rowIndicatorColumn = rowIndicatorColumn.as_mut().unwrap();

    let rowHeights = rowIndicatorColumn.getRowHeights();
    if rowHeights.is_none() {
        return Ok(None);
    }
    let rowHeights = rowHeights.unwrap();
    let maxRowHeight = getMax(&rowHeights);
    let mut missingStartRows = 0;
    for rowHeight in &rowHeights {
        // for (int rowHeight : rowHeights) {
        missingStartRows += maxRowHeight - rowHeight;
        if *rowHeight > 0 {
            break;
        }
    }
    let codewords = rowIndicatorColumn.getCodewords();

    let mut row = 0;
    while missingStartRows > 0 && codewords[row].is_none() {
        // for (int row = 0; missingStartRows > 0 && codewords[row] == null; row++) {
        missingStartRows -= 1;
        row += 1;
    }
    let mut missingEndRows = 0;
    for row in (0..rowHeights.len()).rev() {
        // for (int row = rowHeights.length - 1; row >= 0; row--) {
        missingEndRows += maxRowHeight - rowHeights[row];
        if rowHeights[row] > 0 {
            break;
        }
    }
    let mut row = codewords.len() - 1;
    while missingEndRows > 0 && codewords[row].is_none() {
        // for (int row = codewords.length - 1; missingEndRows > 0 && codewords[row] == null; row--) {
        missingEndRows -= 1;

        row -= 1;
    }
    Ok(Some(rowIndicatorColumn.getBoundingBox().addMissingRows(
        missingStartRows,
        missingEndRows,
        rowIndicatorColumn.isLeft(),
    )?))
}

fn getMax(values: &[u32]) -> u32 {
    // let maxValue = -1;
    // for (int value : values) {
    //   maxValue = Math.max(maxValue, value);
    // }
    // return maxValue;
    *values.iter().max().unwrap()
}

fn getBarcodeMetadata<T: DetectionRXingResultRowIndicatorColumn>(
    leftRowIndicatorColumn: &mut Option<T>,
    rightRowIndicatorColumn: &mut Option<T>,
) -> Option<BarcodeMetadata> {
    let left_ri_md = leftRowIndicatorColumn
        .as_mut()
        .map_or_else(|| None, |col| col.getBarcodeMetadata());
    let right_ri_md = rightRowIndicatorColumn
        .as_mut()
        .map_or_else(|| None, |col| col.getBarcodeMetadata());

    if leftRowIndicatorColumn.is_none() && rightRowIndicatorColumn.is_none() {
        return None;
    } else if leftRowIndicatorColumn.is_none() {
        return right_ri_md;
    } else if rightRowIndicatorColumn.is_none() && right_ri_md.is_none() {
        return left_ri_md;
    } else if let Some((leftBarcodeMetadata, rightBarcodeMetadata)) =
        left_ri_md.as_ref().zip(right_ri_md.as_ref())
    {
        if leftBarcodeMetadata.getColumnCount() != rightBarcodeMetadata.getColumnCount()
            && leftBarcodeMetadata.getErrorCorrectionLevel()
                != rightBarcodeMetadata.getErrorCorrectionLevel()
            && leftBarcodeMetadata.getRowCount() != rightBarcodeMetadata.getRowCount()
        {
            return None;
        }
    }

    left_ri_md

    // let leftBarcodeMetadata = if leftRowIndicatorColumn.is_none()
    //     || leftRowIndicatorColumn
    //         .as_mut()
    //         .unwrap()
    //         .getBarcodeMetadata()
    //         .is_none()
    // {
    //     return if rightRowIndicatorColumn.is_none() {
    //         None
    //     } else {
    //         rightRowIndicatorColumn
    //             .as_mut()
    //             .unwrap()
    //             .getBarcodeMetadata()
    //     };
    // } else {
    //     leftRowIndicatorColumn
    //         .as_mut()
    //         .unwrap()
    //         .getBarcodeMetadata()
    // };
    // // if leftRowIndicatorColumn.is_none() ||
    // //     (leftBarcodeMetadata = leftRowIndicatorColumn.getBarcodeMetadata()).is_none() {
    // //   return if rightRowIndicatorColumn.is_none()  {None} else  {rightRowIndicatorColumn.getBarcodeMetadata()};
    // // }

    // let rightBarcodeMetadata = if rightRowIndicatorColumn.is_none() {
    //     return leftBarcodeMetadata;
    // } else if let Some(mdt) = rightRowIndicatorColumn
    //     .as_mut()
    //     .unwrap()
    //     .getBarcodeMetadata()
    // {
    //     mdt
    //     // rightRowIndicatorColumn
    //     //     .as_mut()
    //     //     .unwrap()
    //     //     .getBarcodeMetadata()
    //     //     .unwrap()
    // } else {
    //     return leftBarcodeMetadata;
    // };
    // // if rightRowIndicatorColumn.is_none() ||
    // //     (rightBarcodeMetadata = rightRowIndicatorColumn.getBarcodeMetadata()).is_none() {
    // //   return leftBarcodeMetadata;
    // // }

    // leftBarcodeMetadata?;

    // if leftBarcodeMetadata.as_ref().unwrap().getColumnCount()
    //     != rightBarcodeMetadata.getColumnCount()
    //     && leftBarcodeMetadata
    //         .as_ref()
    //         .unwrap()
    //         .getErrorCorrectionLevel()
    //         != rightBarcodeMetadata.getErrorCorrectionLevel()
    //     && leftBarcodeMetadata.as_ref().unwrap().getRowCount() != rightBarcodeMetadata.getRowCount()
    // {
    //     return None;
    // }

    // leftBarcodeMetadata
}

fn getRowIndicatorColumn<'a>(
    image: &BitMatrix,
    boundingBox: Rc<BoundingBox>,
    startPoint: RXingResultPoint,
    leftToRight: bool,
    minCodewordWidth: u32,
    maxCodewordWidth: u32,
) -> impl DetectionRXingResultRowIndicatorColumn + 'a {
    let mut rowIndicatorColumn =
        DetectionRXingResultColumn::new_with_is_left(boundingBox.clone(), leftToRight);
    for i in 0..2 {
        // for (int i = 0; i < 2; i++) {
        let increment: i32 = if i == 0 { 1 } else { -1 };
        let mut startColumn: u32 = startPoint.getX() as u32;
        let mut imageRow: i32 = startPoint.getY() as i32;
        while imageRow <= boundingBox.getMaxY() as i32 && imageRow >= boundingBox.getMinY() as i32 {
            // for (int imageRow = (int) startPoint.getY(); imageRow <= boundingBox.getMaxY() &&
            //     imageRow >= boundingBox.getMinY(); imageRow += increment) {
            let codeword = detectCodeword(
                image,
                0,
                image.getWidth(),
                leftToRight,
                startColumn,
                imageRow as u32,
                minCodewordWidth,
                maxCodewordWidth,
            );
            if let Some(codeword) = codeword {
                // if codeword.is_some() {
                rowIndicatorColumn.setCodeword(imageRow as u32, codeword);
                if leftToRight {
                    startColumn = codeword.getStartX();
                } else {
                    startColumn = codeword.getEndX();
                }
            }

            imageRow += increment;
        }
    }

    rowIndicatorColumn
}

fn adjustCodewordCount(
    detectionRXingResult: &DetectionRXingResult,
    barcodeMatrix: &mut [Vec<BarcodeValue>],
) -> Result<(), Exceptions> {
    let barcodeMatrix01 = &mut barcodeMatrix[0][1];
    let numberOfCodewords = barcodeMatrix01.getValue();
    let calculatedNumberOfCodewords = (detectionRXingResult.getBarcodeColumnCount() as isize
        * detectionRXingResult.getBarcodeRowCount() as isize
        - getNumberOfECCodeWords(detectionRXingResult.getBarcodeECLevel()) as isize)
        as u32;
    if numberOfCodewords.is_empty() {
        if !(1..=pdf_417_common::MAX_CODEWORDS_IN_BARCODE).contains(&calculatedNumberOfCodewords) {
            return Err(Exceptions::NotFoundException(None));
        }
        barcodeMatrix01.setValue(calculatedNumberOfCodewords);
    } else if numberOfCodewords[0] != calculatedNumberOfCodewords
        && (1..=pdf_417_common::MAX_CODEWORDS_IN_BARCODE).contains(&calculatedNumberOfCodewords)
    {
        // The calculated one is more reliable as it is derived from the row indicator columns
        barcodeMatrix01.setValue(calculatedNumberOfCodewords);
    }
    Ok(())
}

fn createDecoderRXingResult(
    detectionRXingResult: &mut DetectionRXingResult,
) -> Result<DecoderRXingResult, Exceptions> {
    let mut barcodeMatrix = createBarcodeMatrix(detectionRXingResult);
    adjustCodewordCount(detectionRXingResult, &mut barcodeMatrix)?;
    let mut erasures = Vec::new();
    let mut codewords = vec![
        0;
        detectionRXingResult.getBarcodeRowCount() as usize
            * detectionRXingResult.getBarcodeColumnCount()
    ];
    let mut ambiguousIndexValuesList: Vec<Vec<u32>> = Vec::new();
    let mut ambiguousIndexesList = Vec::new();
    for row in 0..detectionRXingResult.getBarcodeRowCount() {
        for column in 0..detectionRXingResult.getBarcodeColumnCount() {
            let values = barcodeMatrix[row as usize][column + 1].getValue();
            let codewordIndex =
                row as usize * detectionRXingResult.getBarcodeColumnCount() + column;
            if values.is_empty() {
                erasures.push(codewordIndex as u32);
            } else if values.len() == 1 {
                codewords[codewordIndex] = values[0];
            } else {
                ambiguousIndexesList.push(codewordIndex as u32);
                ambiguousIndexValuesList.push(values);
            }
        }
    }
    let ambiguousIndexValues = Vec::from_iter(ambiguousIndexValuesList.into_iter());
    // for value in ambiguousIndexValuesList {
    //     ambiguousIndexValues.push(value);
    // }
    // for i in 0..ambiguousIndexValuesList.len() {
    // // for (int i = 0; i < ambiguousIndexValues.length; i++) {
    //   ambiguousIndexValues[i] = ambiguousIndexValuesList.get(i) as u32;
    // }
    createDecoderRXingResultFromAmbiguousValues(
        detectionRXingResult.getBarcodeECLevel(),
        &mut codewords,
        &mut erasures,
        &mut ambiguousIndexesList,
        &ambiguousIndexValues,
    )
}

/**
 * This method deals with the fact, that the decoding process doesn't always yield a single most likely value. The
 * current error correction implementation doesn't deal with erasures very well, so it's better to provide a value
 * for these ambiguous codewords instead of treating it as an erasure. The problem is that we don't know which of
 * the ambiguous values to choose. We try decode using the first value, and if that fails, we use another of the
 * ambiguous values and try to decode again. This usually only happens on very hard to read and decode barcodes,
 * so decoding the normal barcodes is not affected by this.
 *
 * @param erasureArray contains the indexes of erasures
 * @param ambiguousIndexes array with the indexes that have more than one most likely value
 * @param ambiguousIndexValues two dimensional array that contains the ambiguous values. The first dimension must
 * be the same length as the ambiguousIndexes array
 */
fn createDecoderRXingResultFromAmbiguousValues(
    ecLevel: u32,
    codewords: &mut [u32],
    erasureArray: &mut [u32],
    ambiguousIndexes: &mut [u32],
    ambiguousIndexValues: &[Vec<u32>],
) -> Result<DecoderRXingResult, Exceptions> {
    let mut ambiguousIndexCount = vec![0; ambiguousIndexes.len()];

    let mut tries = 100;
    while tries > 0 {
        for i in 0..ambiguousIndexCount.len() {
            // for (int i = 0; i < ambiguousIndexCount.length; i++) {
            codewords[ambiguousIndexes[i] as usize] =
                ambiguousIndexValues[i][ambiguousIndexCount[i]];
        }
        let attempted_decode = decodeCodewords(codewords, ecLevel, erasureArray);
        if attempted_decode.is_ok() {
            return attempted_decode;
        }
        // try {
        //   return decodeCodewords(codewords, ecLevel, erasureArray);
        // } catch (ChecksumException ignored) {
        //   //
        // }
        if ambiguousIndexCount.is_empty() {
            return Err(Exceptions::ChecksumException(None));
        }
        for i in 0..ambiguousIndexCount.len() {
            // for (int i = 0; i < ambiguousIndexCount.length; i++) {
            if ambiguousIndexCount[i] < ambiguousIndexValues[i].len() - 1 {
                ambiguousIndexCount[i] += 1;
                break;
            } else {
                ambiguousIndexCount[i] = 0;
                if i == ambiguousIndexCount.len() - 1 {
                    return Err(Exceptions::ChecksumException(None));
                }
            }
        }

        tries -= 1;
    }
    Err(Exceptions::ChecksumException(None))
}

fn createBarcodeMatrix(detectionRXingResult: &mut DetectionRXingResult) -> Vec<Vec<BarcodeValue>> {
    let mut barcodeMatrix =
        vec![
            vec![BarcodeValue::new(); detectionRXingResult.getBarcodeColumnCount() + 2];
            detectionRXingResult.getBarcodeRowCount() as usize
        ];
    // BarcodeValue[][] barcodeMatrix =
    //     new BarcodeValue[detectionRXingResult.getBarcodeRowCount()][detectionRXingResult.getBarcodeColumnCount() + 2];
    // for row in 0..barcodeMatrix.len() {
    // // for (int row = 0; row < barcodeMatrix.length; row++) {
    //   for column in 0..barcodeMatrix[row].len() {
    //   // for (int column = 0; column < barcodeMatrix[row].length; column++) {
    //     barcodeMatrix[row][column] =  BarcodeValue::new();
    //   }
    // }

    let mut column = 0;
    for detectionRXingResultColumn in detectionRXingResult.getDetectionRXingResultColumns() {
        // for (DetectionRXingResultColumn detectionRXingResultColumn : detectionRXingResult.getDetectionRXingResultColumns()) {
        if detectionRXingResultColumn.is_some() {
            for codeword in detectionRXingResultColumn
                .as_ref()
                .unwrap()
                .getCodewords()
                .iter()
                .flatten()
            {
                // for (Codeword codeword : detectionRXingResultColumn.getCodewords()) {
                // if let Some(codeword) = codeword {
                // if codeword.is_some() {
                let rowNumber = codeword.getRowNumber();
                if rowNumber >= 0 {
                    if rowNumber as usize >= barcodeMatrix.len() {
                        // We have more rows than the barcode metadata allows for, ignore them.
                        continue;
                    }
                    barcodeMatrix[rowNumber as usize][column].setValue(codeword.getValue());
                }
                // }
            }
        }
        column += 1;
    }
    barcodeMatrix
}

fn isValidBarcodeColumn(detectionRXingResult: &DetectionRXingResult, barcodeColumn: usize) -> bool {
    /*barcodeColumn >= 0 &&*/
    barcodeColumn <= detectionRXingResult.getBarcodeColumnCount() + 1
}

fn getStartColumn(
    detectionRXingResult: &DetectionRXingResult,
    barcodeColumn: usize,
    imageRow: u32,
    leftToRight: bool,
) -> Option<u32> {
    let offset: isize = if leftToRight { 1 } else { -1 };
    let mut barcodeColumn = barcodeColumn as isize;
    let mut codeword = &None;
    if isValidBarcodeColumn(detectionRXingResult, (barcodeColumn - offset) as usize) {
        codeword = detectionRXingResult
            .getDetectionRXingResultColumn((barcodeColumn - offset) as usize)
            .as_ref()?
            .getCodeword(imageRow);
    }
    if let Some(codeword) = codeword {
        return if leftToRight {
            Some(codeword.getEndX())
        } else {
            Some(codeword.getStartX())
        };
    }

    if detectionRXingResult
        .getDetectionRXingResultColumn(barcodeColumn as usize)
        .is_some()
    {
        codeword = detectionRXingResult
            .getDetectionRXingResultColumn(barcodeColumn as usize)
            .as_ref()?
            .getCodewordNearby(imageRow);
    }

    if let Some(codeword) = codeword {
        return if leftToRight {
            Some(codeword.getStartX())
        } else {
            Some(codeword.getEndX())
        };
    }
    if isValidBarcodeColumn(detectionRXingResult, (barcodeColumn - offset) as usize) {
        codeword = detectionRXingResult
            .getDetectionRXingResultColumn((barcodeColumn - offset) as usize)
            .as_ref()?
            .getCodewordNearby(imageRow);
    }
    if let Some(codeword) = codeword {
        return if leftToRight {
            Some(codeword.getEndX())
        } else {
            Some(codeword.getStartX())
        };
    }
    let mut skippedColumns = 0;

    while isValidBarcodeColumn(detectionRXingResult, (barcodeColumn - offset) as usize) {
        barcodeColumn -= offset;
        if let Some(previousRowCodeword) = detectionRXingResult
            .getDetectionRXingResultColumn(barcodeColumn as usize)
            .as_ref()?
            .getCodewords()
            .iter()
            .flatten()
            .next()
        {
            // for (Codeword previousRowCodeword : detectionRXingResult.getDetectionRXingResultColumn(barcodeColumn).getCodewords()) {
            // if let Some(previousRowCodeword) = previousRowCodeword {
            // if previousRowCodeword.is_some() {
            return Some(
                ((if leftToRight {
                    previousRowCodeword.getEndX()
                } else {
                    previousRowCodeword.getStartX()
                }) as isize
                    + offset
                        * skippedColumns as isize
                        * (previousRowCodeword.getEndX() - previousRowCodeword.getStartX())
                            as isize) as u32,
            );
            // }
        }
        skippedColumns += 1;
    }
    if leftToRight {
        Some(detectionRXingResult.getBoundingBox().getMinX())
    } else {
        Some(detectionRXingResult.getBoundingBox().getMaxX())
    }
}

#[allow(clippy::too_many_arguments)]
fn detectCodeword(
    image: &BitMatrix,
    minColumn: u32,
    maxColumn: u32,
    leftToRight: bool,
    startColumn: u32,
    imageRow: u32,
    minCodewordWidth: u32,
    maxCodewordWidth: u32,
) -> Option<Codeword> {
    let mut startColumn = adjustCodewordStartColumn(
        image,
        minColumn,
        maxColumn,
        leftToRight,
        startColumn,
        imageRow,
    );
    // we usually know fairly exact now how long a codeword is. We should provide minimum and maximum expected length
    // and try to adjust the read pixels, e.g. remove single pixel errors or try to cut off exceeding pixels.
    // min and maxCodewordWidth should not be used as they are calculated for the whole barcode an can be inaccurate
    // for the current position
    let mut moduleBitCount = getModuleBitCount(
        image,
        minColumn,
        maxColumn,
        leftToRight,
        startColumn,
        imageRow,
    )?;

    let endColumn;
    let codewordBitCount = moduleBitCount.iter().sum::<u32>();
    if leftToRight {
        endColumn = startColumn + codewordBitCount;
    } else {
        for i in 0..(moduleBitCount.len() / 2) {
            // for (int i = 0; i < moduleBitCount.length / 2; i++) {

            let len = moduleBitCount.len();
            moduleBitCount.swap(i, len - 1 - i);

            // let tmpCount = moduleBitCount[i];
            // moduleBitCount[i] = moduleBitCount[moduleBitCount.len() - 1 - i];
            // moduleBitCount[moduleBitCount.len() - 1 - i] = tmpCount;
        }
        endColumn = startColumn;
        startColumn = endColumn - codewordBitCount;
    }
    // TODO implement check for width and correction of black and white bars
    // use start (and maybe stop pattern) to determine if black bars are wider than white bars. If so, adjust.
    // should probably done only for codewords with a lot more than 17 bits.
    // The following fixes 10-1.png, which has wide black bars and small white bars
    //    for (int i = 0; i < moduleBitCount.length; i++) {
    //      if (i % 2 == 0) {
    //        moduleBitCount[i]--;
    //      } else {
    //        moduleBitCount[i]++;
    //      }
    //    }

    // We could also use the width of surrounding codewords for more accurate results, but this seems
    // sufficient for now
    if !checkCodewordSkew(codewordBitCount, minCodewordWidth, maxCodewordWidth) {
        // We could try to use the startX and endX position of the codeword in the same column in the previous row,
        // create the bit count from it and normalize it to 8. This would help with single pixel errors.
        return None;
    }

    let decodedValue = pdf_417_codeword_decoder::getDecodedValue(&moduleBitCount);
    let codeword = pdf_417_common::getCodeword(decodedValue);
    if codeword == -1 {
        return None;
    }

    Some(Codeword::new(
        startColumn,
        endColumn,
        getCodewordBucketNumber(decodedValue),
        codeword as u32,
    ))
}

fn getModuleBitCount(
    image: &BitMatrix,
    minColumn: u32,
    maxColumn: u32,
    leftToRight: bool,
    startColumn: u32,
    imageRow: u32,
) -> Option<[u32; 8]> {
    let mut imageColumn = startColumn as i32;
    let mut moduleBitCount = [0_u32; 8];
    let mut moduleNumber = 0;
    let increment: i32 = if leftToRight { 1 } else { -1 };
    let mut previousPixelValue = leftToRight;
    while (if leftToRight {
        imageColumn < maxColumn as i32
    } else {
        imageColumn >= minColumn as i32
    }) && moduleNumber < moduleBitCount.len()
    {
        if image.get(imageColumn as u32, imageRow) == previousPixelValue {
            moduleBitCount[moduleNumber] += 1;
            imageColumn += increment;
        } else {
            moduleNumber += 1;
            previousPixelValue = !previousPixelValue;
        }
    }
    if moduleNumber == moduleBitCount.len()
        || ((imageColumn
            == (if leftToRight {
                maxColumn as i32
            } else {
                minColumn as i32
            }))
            && moduleNumber == moduleBitCount.len() - 1)
    {
        return Some(moduleBitCount);
    }

    None
}

fn getNumberOfECCodeWords(barcodeECLevel: u32) -> u32 {
    2 << barcodeECLevel
}

fn adjustCodewordStartColumn(
    image: &BitMatrix,
    minColumn: u32,
    maxColumn: u32,
    leftToRight: bool,
    codewordStartColumn: u32,
    imageRow: u32,
) -> u32 {
    let mut correctedStartColumn = codewordStartColumn;
    let mut increment: i32 = if leftToRight { -1 } else { 1 };
    let mut leftToRight = leftToRight;
    // there should be no black pixels before the start column. If there are, then we need to start earlier.
    for _i in 0..2 {
        while (if leftToRight {
            correctedStartColumn >= minColumn
        } else {
            correctedStartColumn < maxColumn
        }) && leftToRight == image.get(correctedStartColumn, imageRow)
        {
            if (codewordStartColumn as i64 - correctedStartColumn as i64).unsigned_abs() as u32
                > CODEWORD_SKEW_SIZE
            {
                return codewordStartColumn;
            }
            correctedStartColumn = (correctedStartColumn as i32 + increment) as u32;
            if image.check_in_bounds(correctedStartColumn, imageRow) {
                return 0;
            }
        }
        increment = -increment;
        leftToRight = !leftToRight;
    }
    correctedStartColumn
}

fn checkCodewordSkew(codewordSize: u32, minCodewordWidth: u32, maxCodewordWidth: u32) -> bool {
    minCodewordWidth as i64 - CODEWORD_SKEW_SIZE as i64 <= codewordSize as i64
        && codewordSize <= maxCodewordWidth + CODEWORD_SKEW_SIZE
}

fn decodeCodewords(
    codewords: &mut [u32],
    ecLevel: u32,
    erasures: &mut [u32],
) -> Result<DecoderRXingResult, Exceptions> {
    if codewords.is_empty() {
        return Err(Exceptions::FormatException(None));
    }

    let numECCodewords = 1 << (ecLevel + 1);
    let correctedErrorsCount = correctErrors(codewords, erasures, numECCodewords)?;
    verifyCodewordCount(codewords, numECCodewords)?;

    // Decode the codewords
    let mut decoderRXingResult =
        decoded_bit_stream_parser::decode(codewords, &ecLevel.to_string())?;
    decoderRXingResult.setErrorsCorrected(correctedErrorsCount);
    decoderRXingResult.setErasures(erasures.len());

    Ok(decoderRXingResult)
}

/**
 * <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
 * correct the errors in-place.</p>
 *
 * @param codewords   data and error correction codewords
 * @param erasures positions of any known erasures
 * @param numECCodewords number of error correction codewords that are available in codewords
 * @throws ChecksumException if error correction fails
 */
fn correctErrors(
    codewords: &mut [u32],
    erasures: &mut [u32],
    numECCodewords: u32,
) -> Result<usize, Exceptions> {
    if !erasures.is_empty() && erasures.len() as u32 > numECCodewords / 2 + MAX_ERRORS
        /*|| numECCodewords < 0*/
        || numECCodewords > MAX_EC_CODEWORDS
    {
        // Too many errors or EC Codewords is corrupted
        return Err(Exceptions::ChecksumException(None));
    }
    ec::error_correction::decode(codewords, numECCodewords, erasures)
}

/**
 * Verify that all is OK with the codeword array.
 */
fn verifyCodewordCount(codewords: &mut [u32], numECCodewords: u32) -> Result<(), Exceptions> {
    if codewords.len() < 4 {
        // Codeword array size should be at least 4 allowing for
        // Count CW, At least one Data CW, Error Correction CW, Error Correction CW
        return Err(Exceptions::FormatException(None));
    }
    // The first codeword, the Symbol Length Descriptor, shall always encode the total number of data
    // codewords in the symbol, including the Symbol Length Descriptor itself, data codewords and pad
    // codewords, but excluding the number of error correction codewords.
    let numberOfCodewords = codewords[0];
    if numberOfCodewords > codewords.len() as u32 {
        return Err(Exceptions::FormatException(None));
    }
    if numberOfCodewords == 0 {
        // Reset to the length of the array - 8 (Allow for at least level 3 Error Correction (8 Error Codewords)
        if numECCodewords < codewords.len() as u32 {
            codewords[0] = codewords.len() as u32 - numECCodewords;
        } else {
            return Err(Exceptions::FormatException(None));
        }
    }
    Ok(())
}

fn getBitCountForCodeword(codeword: u32) -> [u32; 8] {
    let mut codeword = codeword;
    let mut result = [0; 8];
    let mut previousValue = 0;
    let mut i = result.len() as isize - 1;
    loop {
        if (codeword & 0x1) != previousValue {
            previousValue = codeword & 0x1;
            i -= 1;
            if i < 0 {
                break;
            }
        }
        result[i as usize] += 1;
        codeword >>= 1;
    }

    result
}

fn getCodewordBucketNumber(codeword: u32) -> u32 {
    getCodewordBucketNumberArray(&getBitCountForCodeword(codeword))
}

fn getCodewordBucketNumberArray(moduleBitCount: &[u32]) -> u32 {
    (moduleBitCount[0] as i32 - moduleBitCount[2] as i32 + moduleBitCount[4] as i32
        - moduleBitCount[6] as i32
        + 9) as u32
        % 9
}

// fn toString( barcodeMatrix:Vec<Vec<BarcodeValue>>) -> String{
//   try (Formatter formatter = new Formatter()) {
//     for (int row = 0; row < barcodeMatrix.length; row++) {
//       formatter.format("Row %2d: ", row);
//       for (int column = 0; column < barcodeMatrix[row].length; column++) {
//         BarcodeValue barcodeValue = barcodeMatrix[row][column];
//         if (barcodeValue.getValue().length == 0) {
//           formatter.format("        ", (Object[]) null);
//         } else {
//           formatter.format("%4d(%2d)", barcodeValue.getValue()[0],
//               barcodeValue.getConfidence(barcodeValue.getValue()[0]));
//         }
//       }
//       formatter.format("%n");
//     }
//     return formatter.toString();
//   }
// }
