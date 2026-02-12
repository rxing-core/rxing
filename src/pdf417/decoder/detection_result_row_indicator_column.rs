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
use crate::{RowIndicatorVars, WitnessData};

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
    fn getBarcodeMetadata(
        &mut self,
        witness_data: Option<&mut WitnessData>,
    ) -> Option<BarcodeMetadata>;
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
        if let Some(barcodeMetadata) = self.getBarcodeMetadata(None) {
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

    fn getBarcodeMetadata(
        &mut self,
        witness_data: Option<&mut WitnessData>,
    ) -> Option<BarcodeMetadata> {
        let isLeft = matches!(self.isLeft, Some(true));
        let codewords = self.getCodewordsMut();
        let mut barcodeColumnCount = BarcodeValue::new();
        let mut barcodeRowCountUpperPart = BarcodeValue::new();
        let mut barcodeRowCountLowerPart = BarcodeValue::new();
        let mut barcodeECLevel = BarcodeValue::new();

        // Track row indicator values for witness data (only for left indicators)
        let mut have_cluster_0 = false;
        let mut have_cluster_1 = false;
        let mut have_cluster_2 = false;
        let mut l0: u32 = 0;
        let mut l3: u32 = 0;
        let mut l6: u32 = 0;
        let mut q0: u32 = 0;
        let mut q3: u32 = 0;
        let mut q6: u32 = 0;
        let mut r0: u32 = 0;
        let mut r3: u32 = 0;

        for codeword in codewords.iter_mut().flatten() {
            // for (Codeword codeword : codewords) {
            // if let Some(codeword) = codeword_opt {
            codeword.setRowNumberAsRowIndicatorColumn();
            let fullValue = codeword.getValue();
            let rowIndicatorValue = fullValue % 30;
            let quotient = fullValue / 30;
            let mut codewordRowNumber = codeword.getRowNumber();
            if !isLeft {
                codewordRowNumber += 2;
            }
            match codewordRowNumber % 3 {
                0 => {
                    barcodeRowCountUpperPart.setValue(rowIndicatorValue * 3 + 1);
                    if isLeft && !have_cluster_0 && codewordRowNumber == 0 {
                        l0 = fullValue;
                        q0 = quotient;
                        r0 = rowIndicatorValue;
                        have_cluster_0 = true;
                    }
                }
                1 => {
                    barcodeECLevel.setValue(rowIndicatorValue / 3);
                    barcodeRowCountLowerPart.setValue(rowIndicatorValue % 3);
                    if isLeft && !have_cluster_1 && codewordRowNumber == 1 {
                        l3 = fullValue;
                        q3 = quotient;
                        r3 = rowIndicatorValue;
                        have_cluster_1 = true;
                    }
                }
                2 => {
                    barcodeColumnCount.setValue(rowIndicatorValue + 1);
                    if isLeft && !have_cluster_2 && codewordRowNumber == 2 {
                        l6 = fullValue;
                        q6 = quotient;
                        // r6 is not stored - it equals num_cols - 1
                        have_cluster_2 = true;
                    }
                }
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

        // Write row indicator values to witness data if provided and this is the left column
        if let Some(wd) = witness_data {
            if isLeft && have_cluster_0 && have_cluster_1 && have_cluster_2 {
                wd.set_row_indicators(RowIndicatorVars {
                    l0,
                    l3,
                    l6,
                    q0,
                    q3,
                    q6,
                    r0,
                    r3,
                });
            }
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

#[test]
fn test_witness_collection_fails_on_missing_first_three_rows() {
    use std::sync::Arc;
    use crate::common::BitMatrix;
    use crate::pdf417::decoder::BoundingBox;
    // Ensure you import the trait so you can use the constructor
    use crate::pdf417::decoder::DetectionRXingResultColumnTrait;
    use crate::Point;

    // 1. Setup basic requirements
    let width = 100;
    let height = 100;
    let image_data = vec![vec![0u8; width]; height];
    let mut witness = WitnessData::new(width, height, image_data);
    
    // FIX: Use Arc for the BitMatrix
    let shared_matrix = Arc::new(BitMatrix::new(100, 100).unwrap());
    
    // Create the 4 corners of the box using the Point struct
    let topLeft = Point::new(0.0, 0.0);
    let bottomLeft = Point::new(0.0, 20.0);
    let topRight = Point::new(20.0, 0.0);
    let bottomRight = Point::new(20.0, 20.0);

    // Now BoundingBox should validate correctly
    let bounding_box = BoundingBox::new(
        shared_matrix, 
        Some(topLeft), 
        Some(bottomLeft), 
        Some(topRight), 
        Some(bottomRight)
    ).expect("BoundingBox should now be valid with explicit Points");      

    // 2. FIX: Use the specific constructor name from your file
    // Note: It takes a reference &BoundingBox
    let mut column = DetectionRXingResultColumn::new_with_is_left(&bounding_box, true);

    // 3. Mock the codewords (Index matches the Row Number for simplicity)
    // We provide Row 0 and Row 2, but leave Row 1 as None
    column.getCodewordsMut()[0] = Some(Codeword::new(0, 500, 0, 10)); // Row 0
    column.getCodewordsMut()[1] = None;                             // Row 1 (MISSING)
    column.getCodewordsMut()[2] = Some(Codeword::new(10, 510, 0, 10)); // Row 2

    // 4. Run the metadata extraction
    // This triggers the internal check: if have_cluster_0 && have_cluster_1 && have_cluster_2
    let _ = column.getBarcodeMetadata(Some(&mut witness));

    // 5. ASSERTIONS
    // have_cluster_1 was false, so row_indicators must still be None
    assert!(witness.row_indicators.is_none(), "Witness should be empty because Row 1 was missing");

    // 6. FINALIZATION CHECK
    // This proves the 'security' of your collection: incomplete data cannot be finalized
    let result = witness.finalize();
    assert!(result.is_err(), "Finalization should fail without first 3 row indicators");
}
