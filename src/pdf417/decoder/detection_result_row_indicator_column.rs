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

use crate::pdf417::pdf_417_common;

use super::{
    BarcodeMetadata, BarcodeValue, Codeword, DetectionRXingResultColumn,
    DetectionRXingResultColumnTrait,
};

/**
 * @author Guenther Grau
 */
pub trait DetectionRXingResultRowIndicatorColumn: DetectionRXingResultColumnTrait {
    // TODO implement properly
    // TODO maybe we should add missing codewords to store the correct row number to make
    // finding row numbers for other columns easier
    // use row height count to make detection of invalid row numbers more reliable
    fn adjustCompleteIndicatorColumnRowNumbers(&mut self, barcodeMetadata: &BarcodeMetadata)
        -> u32;
    fn getRowHeights(&mut self) -> Option<Vec<u32>>;
    fn getBarcodeMetadata(&mut self) -> Option<BarcodeMetadata>;
    fn isLeft(&self) -> bool;
}

impl DetectionRXingResultRowIndicatorColumn for DetectionRXingResultColumn {
    // TODO implement properly
    // TODO maybe we should add missing codewords to store the correct row number to make
    // finding row numbers for other columns easier
    // use row height count to make detection of invalid row numbers more reliable
    fn adjustCompleteIndicatorColumnRowNumbers(
        &mut self,
        barcodeMetadata: &BarcodeMetadata,
    ) -> u32 {
        setRowNumbers(self.getCodewordsMut());

        let isLeft = matches!(self.isLeft, Some(true));

        removeIncorrectCodewords(self.getCodewordsMut(), barcodeMetadata, isLeft);

        let boundingBox = self.getBoundingBox();
        let top = if self.isLeft() {
            boundingBox.getTopLeft()
        } else {
            boundingBox.getTopRight()
        };
        let bottom = if self.isLeft() {
            boundingBox.getBottomLeft()
        } else {
            boundingBox.getBottomRight()
        };

        let firstRow = self.imageRowToCodewordIndex(top.y as u32);
        let lastRow = self.imageRowToCodewordIndex(bottom.y as u32);
        // We need to be careful using the average row height. Barcode could be skewed so that we have smaller and
        // taller rows
        let averageRowHeight: f64 =
            (lastRow as f64 - firstRow as f64) / barcodeMetadata.getRowCount() as f64;
        let mut barcodeRow = -1;
        let mut maxRowHeight = 1;
        let mut currentRowHeight = 0;
        for codewordsRow in firstRow..lastRow {
            if let Some(codeword) = self.getCodewordsMut()[codewordsRow] {
                let rowDifference = codeword.getRowNumber() - barcodeRow;

                // TODO improve handling with case where first row indicator doesn't start with 0

                if rowDifference == 0 {
                    currentRowHeight += 1;
                } else if rowDifference == 1 {
                    maxRowHeight = std::cmp::max(maxRowHeight, currentRowHeight);
                    currentRowHeight = 1;
                    barcodeRow = codeword.getRowNumber();
                } else if rowDifference < 0
                    || codeword.getRowNumber() >= barcodeMetadata.getRowCount() as i32
                    || rowDifference > codewordsRow as i32
                {
                    self.getCodewordsMut()[codewordsRow] = None;
                } else {
                    let checkedRows = if maxRowHeight > 2 {
                        (maxRowHeight - 2) * rowDifference
                    } else {
                        rowDifference
                    };
                    let mut closePreviousCodewordFound = checkedRows >= codewordsRow as i32;
                    let mut i = 1;
                    while i <= checkedRows && !closePreviousCodewordFound {
                        // there must be (height * rowDifference) number of codewords missing. For now we assume height = 1.
                        // This should hopefully get rid of most problems already.
                        closePreviousCodewordFound =
                            self.getCodewords()[codewordsRow - i as usize].is_some();

                        i += 1;
                    }
                    if closePreviousCodewordFound {
                        self.getCodewordsMut()[codewordsRow] = None;
                    } else {
                        barcodeRow = codeword.getRowNumber();
                        currentRowHeight = 1;
                    }
                }
            } else {
                continue;
            }
        }
        (averageRowHeight + 0.5) as u32
    }

    fn getRowHeights(&mut self) -> Option<Vec<u32>> {
        if let Some(barcodeMetadata) = self.getBarcodeMetadata() {
            adjustIncompleteIndicatorColumnRowNumbers(self, &barcodeMetadata);
            let mut result = vec![0; barcodeMetadata.getRowCount() as usize];
            for codeword in self.getCodewords().iter().flatten() {
                // if let Some(codeword) = codeword_opt {
                let rowNumber = codeword.getRowNumber() as usize;
                if rowNumber >= result.len() {
                    // We have more rows than the barcode metadata allows for, ignore them.
                    continue;
                }
                result[rowNumber] += 1;
                // }
                // else throw exception?
                // else {
                //     continue;
                // }
            }
            Some(result)
        } else {
            None
        }
    }

    fn getBarcodeMetadata(&mut self) -> Option<BarcodeMetadata> {
        let isLeft = matches!(self.isLeft, Some(true));
        let codewords = self.getCodewordsMut();
        let mut barcodeColumnCount = BarcodeValue::new();
        let mut barcodeRowCountUpperPart = BarcodeValue::new();
        let mut barcodeRowCountLowerPart = BarcodeValue::new();
        let mut barcodeECLevel = BarcodeValue::new();
        for codeword in codewords.iter_mut().flatten() {
            // for (Codeword codeword : codewords) {
            // if let Some(codeword) = codeword_opt {
            codeword.setRowNumberAsRowIndicatorColumn();
            let rowIndicatorValue = codeword.getValue() % 30;
            let mut codewordRowNumber = codeword.getRowNumber();
            if !isLeft {
                codewordRowNumber += 2;
            }
            match codewordRowNumber % 3 {
                0 => barcodeRowCountUpperPart.setValue(rowIndicatorValue * 3 + 1),
                1 => {
                    barcodeECLevel.setValue(rowIndicatorValue / 3);
                    barcodeRowCountLowerPart.setValue(rowIndicatorValue % 3);
                }
                2 => barcodeColumnCount.setValue(rowIndicatorValue + 1),
                _ => {}
            }
            // } else {
            //     continue;
            // }
        }
        // Maybe we should check if we have ambiguous values?
        if barcodeColumnCount.getValue().is_empty()
            || barcodeRowCountUpperPart.getValue().is_empty()
            || barcodeRowCountLowerPart.getValue().is_empty()
            || barcodeECLevel.getValue().is_empty()
            || barcodeColumnCount.getValue()[0] < 1
            || barcodeRowCountUpperPart.getValue()[0] + barcodeRowCountLowerPart.getValue()[0]
                < pdf_417_common::MIN_ROWS_IN_BARCODE
            || barcodeRowCountUpperPart.getValue()[0] + barcodeRowCountLowerPart.getValue()[0]
                > pdf_417_common::MAX_ROWS_IN_BARCODE
        {
            return None;
        }
        let barcodeMetadata = BarcodeMetadata::new(
            barcodeColumnCount.getValue()[0],
            barcodeRowCountUpperPart.getValue()[0],
            barcodeRowCountLowerPart.getValue()[0],
            barcodeECLevel.getValue()[0],
        );
        removeIncorrectCodewords(codewords, &barcodeMetadata, isLeft);

        Some(barcodeMetadata)
    }

    fn isLeft(&self) -> bool {
        matches!(self.isLeft, Some(true))
    }
}

fn setRowNumbers(code_words: &mut [Option<Codeword>]) {
    for codeword in code_words.iter_mut().flatten() {
        codeword.setRowNumberAsRowIndicatorColumn();
    }
}

fn removeIncorrectCodewords(
    codewords: &mut [Option<Codeword>],
    barcodeMetadata: &BarcodeMetadata,
    isLeft: bool,
) {
    // Remove codewords which do not match the metadata
    // TODO Maybe we should keep the incorrect codewords for the start and end positions?
    for codeword_row in codewords.iter_mut() {
        if let Some(codeword) = codeword_row {
            let rowIndicatorValue = codeword.getValue() % 30;
            let mut codewordRowNumber = codeword.getRowNumber();
            if codewordRowNumber > barcodeMetadata.getRowCount() as i32 {
                *codeword_row = None;
                continue;
            }
            if !isLeft {
                codewordRowNumber += 2;
            }
            match codewordRowNumber % 3 {
                0 if rowIndicatorValue * 3 + 1 != barcodeMetadata.getRowCountUpperPart() => {
                    *codeword_row = None;
                }
                1 if rowIndicatorValue / 3 != barcodeMetadata.getErrorCorrectionLevel()
                    || rowIndicatorValue % 3 != barcodeMetadata.getRowCountLowerPart() =>
                {
                    *codeword_row = None;
                }
                2 if rowIndicatorValue + 1 != barcodeMetadata.getColumnCount() => {
                    *codeword_row = None;
                }
                _ => {}
            }
        } else {
            continue;
        }
    }
}

// TODO maybe we should add missing codewords to store the correct row number to make
// finding row numbers for other columns easier
// use row height count to make detection of invalid row numbers more reliable
fn adjustIncompleteIndicatorColumnRowNumbers(
    col: &mut DetectionRXingResultColumn,
    barcodeMetadata: &BarcodeMetadata,
) -> i32 {
    let boundingBox = col.getBoundingBox();
    let top = if col.isLeft() {
        boundingBox.getTopLeft()
    } else {
        boundingBox.getTopRight()
    };
    let bottom = if col.isLeft() {
        boundingBox.getBottomLeft()
    } else {
        boundingBox.getBottomRight()
    };
    let firstRow = col.imageRowToCodewordIndex(top.y as u32);
    let lastRow = col.imageRowToCodewordIndex(bottom.y as u32);
    let averageRowHeight: f64 =
        (lastRow as f64 - firstRow as f64) / barcodeMetadata.getRowCount() as f64;
    let codewords = col.getCodewordsMut();
    let mut barcodeRow = -1;
    let mut maxRowHeight = 1;
    let mut currentRowHeight = 0;
    // todo: It might be clearer what we're doing if we rewrote this a different way?
    // Perhaps codewords.iter_mut().skip(firstRow).take(lastRow-firstRow)?
    for codword_opt in codewords.iter_mut().take(lastRow).skip(firstRow) {
        // for (int codewordsRow = firstRow; codewordsRow < lastRow; codewordsRow++) {

        if let Some(codeword) = codword_opt {
            codeword.setRowNumberAsRowIndicatorColumn();

            let rowDifference = codeword.getRowNumber() - barcodeRow;

            // TODO improve handling with case where first row indicator doesn't start with 0

            if rowDifference == 0 {
                currentRowHeight += 1;
            } else if rowDifference == 1 {
                maxRowHeight = maxRowHeight.max(currentRowHeight);
                currentRowHeight = 1;
                barcodeRow = codeword.getRowNumber();
            } else if codeword.getRowNumber() >= barcodeMetadata.getRowCount() as i32 {
                *codword_opt = None;
            } else {
                barcodeRow = codeword.getRowNumber();
                currentRowHeight = 1;
            }
        } else {
            continue;
        }
    }
    (averageRowHeight + 0.5) as i32
}
