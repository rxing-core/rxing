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

use std::{fmt::Display, rc::Rc};

use crate::pdf417::pdf_417_common;

use super::{
    BarcodeMetadata, BoundingBox, Codeword, DetectionRXingResultColumnTrait,
    DetectionRXingResultRowIndicatorColumn,
};

const ADJUST_ROW_NUMBER_SKIP: u32 = 2;

/**
 * @author Guenther Grau
 */
pub struct DetectionRXingResult {
    barcodeMetadata: BarcodeMetadata,
    detectionRXingResultColumns: Vec<Option<Box<dyn DetectionRXingResultColumnTrait>>>,
    boundingBox: Rc<BoundingBox>,
    barcodeColumnCount: usize,
}

impl DetectionRXingResult {
    pub fn new(
        barcodeMetadata: BarcodeMetadata,
        boundingBox: Rc<BoundingBox>,
    ) -> DetectionRXingResult {
        let mut columns = Vec::new();
        for _i in 0..(barcodeMetadata.getColumnCount() as usize + 2) {
            columns.push(None);
        }
        DetectionRXingResult {
            barcodeColumnCount: barcodeMetadata.getColumnCount() as usize,
            detectionRXingResultColumns: columns, //vec![None; barcodeMetadata.getColumnCount() as usize + 2],
            barcodeMetadata,
            boundingBox,
        }
        // this.barcodeMetadata = barcodeMetadata;
        // this.barcodeColumnCount = barcodeMetadata.getColumnCount();
        // this.boundingBox = boundingBox;
        // detectionRXingResultColumns = new DetectionRXingResultColumn[barcodeColumnCount + 2];
    }

    pub fn getDetectionRXingResultColumns(
        &mut self,
    ) -> &Vec<Option<Box<dyn DetectionRXingResultColumnTrait>>> {
        self.adjustIndicatorColumnRowNumbers(0);
        let pos = self.barcodeColumnCount + 1;
        self.adjustIndicatorColumnRowNumbers(pos);
        let mut unadjustedCodewordCount = pdf_417_common::MAX_CODEWORDS_IN_BARCODE;
        let mut previousUnadjustedCount;
        loop {
            previousUnadjustedCount = unadjustedCodewordCount;
            unadjustedCodewordCount = self.adjustRowNumbers();
            if !(unadjustedCodewordCount > 0 && unadjustedCodewordCount < previousUnadjustedCount) {
                break;
            }
        } //while (unadjustedCodewordCount > 0 && unadjustedCodewordCount < previousUnadjustedCount);
        &self.detectionRXingResultColumns
    }

    fn adjustIndicatorColumnRowNumbers(
        &mut self,
        pos: usize,
        // detectionRXingResultColumn: &mut Option<DetectionRXingResultColumn>,
    ) {
        if self.detectionRXingResultColumns[pos].is_some() {
            // if (detectionRXingResultColumn != null) {
            //   ((DetectionRXingResultRowIndicatorColumn) detectionRXingResultColumn)
            //       .adjustCompleteIndicatorColumnRowNumbers(barcodeMetadata);
            // }
            self.detectionRXingResultColumns[pos]
                .as_mut()
                .unwrap()
                .as_indicator_row()
                .adjustCompleteIndicatorColumnRowNumbers(&self.barcodeMetadata);
        }
    }

    // TODO ensure that no detected codewords with unknown row number are left
    // we should be able to estimate the row height and use it as a hint for the row number
    // we should also fill the rows top to bottom and bottom to top
    /**
     * @return number of codewords which don't have a valid row number. Note that the count is not accurate as codewords
     * will be counted several times. It just serves as an indicator to see when we can stop adjusting row numbers
     */
    fn adjustRowNumbers(&mut self) -> u32 {
        let unadjustedCount = self.adjustRowNumbersByRow();
        if unadjustedCount == 0 {
            return 0;
        }
        for barcodeColumn in 1..(self.barcodeColumnCount + 1) {
            // for (int barcodeColumn = 1; barcodeColumn < barcodeColumnCount + 1; barcodeColumn++) {
            if self.detectionRXingResultColumns[barcodeColumn].is_some() {
                let codewords_len = self.detectionRXingResultColumns[barcodeColumn]
                    .as_ref()
                    .unwrap()
                    .getCodewords()
                    .len();
                for codewordsRow in 0..codewords_len {
                    // for (int codewordsRow = 0; codewordsRow < codewords.length; codewordsRow++) {
                    if let Some(cw_row) = self.detectionRXingResultColumns[barcodeColumn]
                        .as_ref()
                        .unwrap()
                        .getCodewords()[codewordsRow]
                    {
                        if !cw_row.hasValidRowNumber() {
                            self.adjustRowNumbersWithCodewords(
                                barcodeColumn,
                                codewordsRow,
                                barcodeColumn,
                            );
                        }
                    } else {
                        continue;
                    }
                    // if (codewords[codewordsRow] == null) {
                    //   continue;
                    // }
                    // if (!codewords[codewordsRow].hasValidRowNumber()) {
                    //   self.adjustRowNumbers(barcodeColumn, codewordsRow, codewords);
                    // }
                }
            }
        }
        return unadjustedCount;
    }

    fn adjustRowNumbersByRow(&mut self) -> u32 {
        self.adjustRowNumbersFromBothRI();
        // TODO we should only do full row adjustments if row numbers of left and right row indicator column match.
        // Maybe it's even better to calculated the height (in codeword rows) and divide it by the number of barcode
        // rows. This, together with the LRI and RRI row numbers should allow us to get a good estimate where a row
        // number starts and ends.
        let unadjustedCount = self.adjustRowNumbersFromLRI();
        unadjustedCount + self.adjustRowNumbersFromRRI()
    }

    fn adjustRowNumbersFromBothRI(&mut self) {
        if self.detectionRXingResultColumns[0].is_none()
            && self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1].is_none()
        {
            return;
        }

        // let LRIcodewords = self.detectionRXingResultColumns[0].as_ref().unwrap().getCodewords();
        // let RRIcodewords = self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1].as_ref().unwrap().getCodewords();
        for codewordsRow in 0..self.detectionRXingResultColumns[0]
            .as_ref()
            .unwrap()
            .getCodewords()
            .len()
        {
            // for (int codewordsRow = 0; codewordsRow < LRIcodewords.length; codewordsRow++) {
            if
            //let (Some(lricw), Some(rricw)) =
            self.detectionRXingResultColumns[0]
                .as_ref()
                .unwrap()
                .getCodewords()[codewordsRow]
                .is_some()
                && self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1]
                    .as_ref()
                    .unwrap()
                    .getCodewords()[codewordsRow]
                    .is_some()
            {
                if self.detectionRXingResultColumns[0]
                    .as_ref()
                    .unwrap()
                    .getCodewords()[codewordsRow]
                    .as_ref()
                    .unwrap()
                    .getRowNumber()
                    == self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1]
                        .as_ref()
                        .unwrap()
                        .getCodewords()[codewordsRow]
                        .as_ref()
                        .unwrap()
                        .getRowNumber()
                {
                    // if (LRIcodewords[codewordsRow] != null &&
                    //     RRIcodewords[codewordsRow] != null &&
                    //     LRIcodewords[codewordsRow].getRowNumber() == RRIcodewords[codewordsRow].getRowNumber()) {
                    for barcodeColumn in 1..=self.barcodeColumnCount {
                        // for (int barcodeColumn = 1; barcodeColumn <= barcodeColumnCount; barcodeColumn++) {
                        if self.detectionRXingResultColumns[barcodeColumn].is_some()
                        //let Some(dc_col) =
                        //&mut self.detectionRXingResultColumns[barcodeColumn]
                        {
                            if self.detectionRXingResultColumns[barcodeColumn]
                                .as_mut()
                                .unwrap()
                                .getCodewordsMut()[codewordsRow]
                                .is_some()
                            {
                                //let Some(codeword) = &mut self.detectionRXingResultColumns[barcodeColumn].as_mut().unwrap().getCodewordsMut()[codewordsRow] {
                                let new_row_number = self.detectionRXingResultColumns[0]
                                    .as_ref()
                                    .unwrap()
                                    .getCodewords()[codewordsRow]
                                    .as_ref()
                                    .unwrap()
                                    .getRowNumber();
                                self.detectionRXingResultColumns[barcodeColumn]
                                    .as_mut()
                                    .unwrap()
                                    .getCodewordsMut()[codewordsRow]
                                    .as_mut()
                                    .unwrap()
                                    .setRowNumber(new_row_number);
                                if !self.detectionRXingResultColumns[barcodeColumn]
                                    .as_mut()
                                    .unwrap()
                                    .getCodewordsMut()[codewordsRow]
                                    .as_ref()
                                    .unwrap()
                                    .hasValidRowNumber()
                                {
                                    // self.detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow] = None;
                                    self.detectionRXingResultColumns[barcodeColumn]
                                        .as_mut()
                                        .unwrap()
                                        .getCodewordsMut()[codewordsRow] = None;
                                }
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                        // let codeword = self.detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow];
                        // if (codeword == null) {
                        //   continue;
                        // }
                    }
                }
            }
        }

        // if (detectionRXingResultColumns[0] == null || detectionRXingResultColumns[barcodeColumnCount + 1] == null) {
        //   return;
        // }
    }

    fn adjustRowNumbersFromRRI(&mut self) -> u32 {
        if self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1].is_none() {
            return 0;
        }
        // if let Some(col) = &self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1] {
        let mut unadjustedCount = 0;
        let codewords_len = self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1]
            .as_ref()
            .unwrap()
            .getCodewords()
            .len();
        for codewordsRow in 0..codewords_len {
            // for (int codewordsRow = 0; codewordsRow < codewords.length; codewordsRow++) {
            // if let Some(codeword_col) = codewords[codewordsRow] {
            if self.detectionRXingResultColumns[self.barcodeColumnCount as usize + 1]
                .as_ref()
                .unwrap()
                .getCodewords()[codewordsRow]
                .is_none()
            {
                continue;
            }
            let rowIndicatorRowNumber = self.detectionRXingResultColumns
                [self.barcodeColumnCount as usize + 1]
                .as_ref()
                .unwrap()
                .getCodewords()[codewordsRow]
                .as_ref()
                .unwrap()
                .getRowNumber();
            let mut invalidRowCounts = 0;
            let mut barcodeColumn = self.barcodeColumnCount as usize + 1;
            while barcodeColumn > 0 && invalidRowCounts < ADJUST_ROW_NUMBER_SKIP {
                // for (int barcodeColumn = barcodeColumnCount + 1;
                //      barcodeColumn > 0 && invalidRowCounts < ADJUST_ROW_NUMBER_SKIP;
                //      barcodeColumn--) {
                if let Some(bc_col) = &mut self.detectionRXingResultColumns[barcodeColumn] {
                    if let Some(codeword) = &mut bc_col.getCodewordsMut()[codewordsRow] {
                        invalidRowCounts = Self::adjustRowNumberIfValid(
                            rowIndicatorRowNumber,
                            invalidRowCounts,
                            codeword,
                        );
                        if !codeword.hasValidRowNumber() {
                            unadjustedCount += 1;
                        }
                    }
                }
                barcodeColumn -= 1;
            }
            // } else {
            //     continue;
            // }
        }
        unadjustedCount
        // } else {
        //     0
        // }
        // if (detectionRXingResultColumns[barcodeColumnCount + 1] == null) {
        //   return 0;
        // }
        // int unadjustedCount = 0;
        // Codeword[] codewords = detectionRXingResultColumns[barcodeColumnCount + 1].getCodewords();
        // for (int codewordsRow = 0; codewordsRow < codewords.length; codewordsRow++) {
        //   if (codewords[codewordsRow] == null) {
        //     continue;
        //   }
        //   int rowIndicatorRowNumber = codewords[codewordsRow].getRowNumber();
        //   int invalidRowCounts = 0;
        //   for (int barcodeColumn = barcodeColumnCount + 1;
        //        barcodeColumn > 0 && invalidRowCounts < ADJUST_ROW_NUMBER_SKIP;
        //        barcodeColumn--) {
        //     Codeword codeword = detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow];
        //     if (codeword != null) {
        //       invalidRowCounts = adjustRowNumberIfValid(rowIndicatorRowNumber, invalidRowCounts, codeword);
        //       if (!codeword.hasValidRowNumber()) {
        //         unadjustedCount++;
        //       }
        //     }
        //   }
        // }
        // return unadjustedCount;
    }

    fn adjustRowNumbersFromLRI(&mut self) -> u32 {
        if self.detectionRXingResultColumns[0].is_none() {
            return 0;
        }

        // if let Some(col) = &self.detectionRXingResultColumns[0] {
        let mut unadjustedCount = 0;
        let codewords_len = self.detectionRXingResultColumns[0]
            .as_ref()
            .unwrap()
            .getCodewords()
            .len();
        for codewordsRow in 0..codewords_len {
            // for (int codewordsRow = 0; codewordsRow < codewords.length; codewordsRow++) {
            // if let Some(codeword_in_row) = codewords[codewordsRow] {
            if self.detectionRXingResultColumns[0]
                .as_ref()
                .unwrap()
                .getCodewords()[codewordsRow]
                .is_none()
            {
                continue;
            }
            let rowIndicatorRowNumber = self.detectionRXingResultColumns[0]
                .as_ref()
                .unwrap()
                .getCodewords()[codewordsRow]
                .as_ref()
                .unwrap()
                .getRowNumber();
            let mut invalidRowCounts = 0;
            let mut barcodeColumn = 1_usize;
            while barcodeColumn < self.barcodeColumnCount as usize + 1
                && invalidRowCounts < ADJUST_ROW_NUMBER_SKIP
            {
                // for (int barcodeColumn = 1;
                //      barcodeColumn < barcodeColumnCount + 1 && invalidRowCounts < ADJUST_ROW_NUMBER_SKIP;
                //      barcodeColumn++) {
                if let Some(bc_column) = &mut self.detectionRXingResultColumns[barcodeColumn] {
                    if let Some(codeword) = &mut bc_column.getCodewordsMut()[codewordsRow] {
                        invalidRowCounts = Self::adjustRowNumberIfValid(
                            rowIndicatorRowNumber,
                            invalidRowCounts,
                            codeword,
                        );
                        if !codeword.hasValidRowNumber() {
                            unadjustedCount += 1;
                        }
                    }
                }
                // let codeword = self.detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow];
                // if (codeword != null) {
                //   invalidRowCounts = adjustRowNumberIfValid(rowIndicatorRowNumber, invalidRowCounts, codeword);
                //   if (!codeword.hasValidRowNumber()) {
                //     unadjustedCount+=1;
                //   }
                // }
                barcodeColumn += 1;
            }
            // } else {
            //     continue;
            // }
            // if (codewords[codewordsRow] == null) {
            //   continue;
            // }
            // let rowIndicatorRowNumber = codewords[codewordsRow].getRowNumber();
            // let invalidRowCounts = 0;
            // for (int barcodeColumn = 1;
            //      barcodeColumn < barcodeColumnCount + 1 && invalidRowCounts < ADJUST_ROW_NUMBER_SKIP;
            //      barcodeColumn++) {
            //   Codeword codeword = detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow];
            //   if (codeword != null) {
            //     invalidRowCounts = adjustRowNumberIfValid(rowIndicatorRowNumber, invalidRowCounts, codeword);
            //     if (!codeword.hasValidRowNumber()) {
            //       unadjustedCount++;
            //     }
            //   }
            // }
        }
        unadjustedCount
        // } else {
        //     0
        // }

        // if (detectionRXingResultColumns[0] == null) {
        //   return 0;
        // }
        // int unadjustedCount = 0;
        // Codeword[] codewords = detectionRXingResultColumns[0].getCodewords();
        // for (int codewordsRow = 0; codewordsRow < codewords.length; codewordsRow++) {
        //   if (codewords[codewordsRow] == null) {
        //     continue;
        //   }
        //   int rowIndicatorRowNumber = codewords[codewordsRow].getRowNumber();
        //   int invalidRowCounts = 0;
        //   for (int barcodeColumn = 1;
        //        barcodeColumn < barcodeColumnCount + 1 && invalidRowCounts < ADJUST_ROW_NUMBER_SKIP;
        //        barcodeColumn++) {
        //     Codeword codeword = detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow];
        //     if (codeword != null) {
        //       invalidRowCounts = adjustRowNumberIfValid(rowIndicatorRowNumber, invalidRowCounts, codeword);
        //       if (!codeword.hasValidRowNumber()) {
        //         unadjustedCount++;
        //       }
        //     }
        //   }
        // }
        // return unadjustedCount;
    }

    fn adjustRowNumberIfValid(
        rowIndicatorRowNumber: i32,
        mut invalidRowCounts: u32,
        codeword: &mut Codeword,
    ) -> u32 {
        // if let Some(codeword) = codeword {
        if !codeword.hasValidRowNumber() {
            if codeword.isValidRowNumber(rowIndicatorRowNumber) {
                codeword.setRowNumber(rowIndicatorRowNumber);
                invalidRowCounts = 0;
            } else {
                invalidRowCounts += 1;
            }
        }
        invalidRowCounts
        // } else {
        //     invalidRowCounts
        // }
        // if (codeword == null) {
        //   return invalidRowCounts;
        // }
        // if (!codeword.hasValidRowNumber()) {
        //   if (codeword.isValidRowNumber(rowIndicatorRowNumber)) {
        //     codeword.setRowNumber(rowIndicatorRowNumber);
        //     invalidRowCounts = 0;
        //   } else {
        //     invalidRowCounts+=1;
        //   }
        // }
        // return invalidRowCounts;
    }

    fn adjustRowNumbersWithCodewords(
        &mut self,
        barcodeColumn: usize,
        codewordsRow: usize,
        codewordsColumn: usize,
        // codewords: &mut [Option<Codeword>],
    ) {
        // let codewords = self.detectionRXingResultColumns[codewordsColumn]
        //     .as_mut()
        //     .unwrap()
        //     .getCodewordsMut();

        let codewords_len = self.detectionRXingResultColumns[codewordsColumn]
            .as_mut()
            .unwrap()
            .getCodewordsMut()
            .len();

        // let codeword = &mut codewords[codewordsRow];

        let previousColumnCodewords = self.detectionRXingResultColumns[barcodeColumn - 1]
            .as_ref()
            .unwrap()
            .getCodewords()
            .to_vec();

        let mut nextColumnCodewords = previousColumnCodewords.clone();

        if self.detectionRXingResultColumns[barcodeColumn + 1].is_some() {
            nextColumnCodewords = self.detectionRXingResultColumns[barcodeColumn + 1]
                .as_ref()
                .unwrap()
                .getCodewords()
                .to_vec(); //col.getCodewords();
        }
        // if (self.detectionRXingResultColumns[barcodeColumn + 1] != null) {
        //   nextColumnCodewords = self.detectionRXingResultColumns[barcodeColumn + 1].getCodewords();
        // }

        let mut otherCodewords = [None; 14]; // new Codeword[14];

        otherCodewords[2] = previousColumnCodewords[codewordsRow];
        otherCodewords[3] = nextColumnCodewords[codewordsRow];

        if codewordsRow > 0 {
            otherCodewords[0] = self.detectionRXingResultColumns[codewordsColumn]
                .as_mut()
                .unwrap()
                .getCodewordsMut()[codewordsRow - 1];
            otherCodewords[4] = previousColumnCodewords[codewordsRow - 1];
            otherCodewords[5] = nextColumnCodewords[codewordsRow - 1];
        }
        if codewordsRow > 1 {
            otherCodewords[8] = self.detectionRXingResultColumns[codewordsColumn]
                .as_mut()
                .unwrap()
                .getCodewordsMut()[codewordsRow - 2];
            otherCodewords[10] = previousColumnCodewords[codewordsRow - 2];
            otherCodewords[11] = nextColumnCodewords[codewordsRow - 2];
        }
        if codewordsRow < codewords_len - 1 {
            otherCodewords[1] = self.detectionRXingResultColumns[codewordsColumn]
                .as_mut()
                .unwrap()
                .getCodewordsMut()[codewordsRow + 1];
            otherCodewords[6] = previousColumnCodewords[codewordsRow + 1];
            otherCodewords[7] = nextColumnCodewords[codewordsRow + 1];
        }
        if codewordsRow < codewords_len - 2 {
            otherCodewords[9] = self.detectionRXingResultColumns[codewordsColumn]
                .as_mut()
                .unwrap()
                .getCodewordsMut()[codewordsRow + 2];
            otherCodewords[12] = previousColumnCodewords[codewordsRow + 2];
            otherCodewords[13] = nextColumnCodewords[codewordsRow + 2];
        }
        for otherCodeword in otherCodewords {
            if Self::adjustRowNumber(
                self.detectionRXingResultColumns[codewordsColumn]
                    .as_mut()
                    .unwrap()
                    .getCodewordsMut()[codewordsRow]
                    .as_mut()
                    .unwrap(),
                &otherCodeword,
            ) {
                return;
            }
        }
        // for (Codeword otherCodeword : otherCodewords) {
        //   if (adjustRowNumber(codeword, otherCodeword)) {
        //     return;
        //   }
        // }
    }

    /**
     * @return true, if row number was adjusted, false otherwise
     */
    fn adjustRowNumber(codeword: &mut Codeword, otherCodeword: &Option<Codeword>) -> bool {
        if let Some(otherCodeword) = otherCodeword {
            if otherCodeword.hasValidRowNumber()
                && otherCodeword.getBucket() == codeword.getBucket()
            {
                codeword.setRowNumber(otherCodeword.getRowNumber());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn getBarcodeColumnCount(&self) -> usize {
        self.barcodeColumnCount
    }

    pub fn getBarcodeRowCount(&self) -> u32 {
        self.barcodeMetadata.getRowCount()
    }

    pub fn getBarcodeECLevel(&self) -> u32 {
        self.barcodeMetadata.getErrorCorrectionLevel()
    }

    pub fn setBoundingBox(&mut self, boundingBox: Rc<BoundingBox>) {
        self.boundingBox = boundingBox;
    }

    pub fn getBoundingBox(&self) -> Rc<BoundingBox> {
        self.boundingBox.clone()
    }

    pub fn setDetectionRXingResultColumn(
        &mut self,
        barcodeColumn: usize,
        detectionRXingResultColumn: Option<impl DetectionRXingResultRowIndicatorColumn + 'static>,
    ) {
        self.detectionRXingResultColumns[barcodeColumn] = if detectionRXingResultColumn.is_none() {
            None
        } else {
            Some(Box::new(detectionRXingResultColumn.unwrap()))
        };
    }

    pub fn getDetectionRXingResultColumn(
        &self,
        barcodeColumn: usize,
    ) -> &Option<Box<dyn DetectionRXingResultColumnTrait>> {
        &self.detectionRXingResultColumns[barcodeColumn]
    }

    pub fn getDetectionRXingResultColumnMut(
        &mut self,
        barcodeColumn: usize,
    ) -> &mut Option<Box<dyn DetectionRXingResultColumnTrait>> {
        &mut self.detectionRXingResultColumns[barcodeColumn]
    }
}

impl Display for DetectionRXingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rowIndicatorColumn = &self.detectionRXingResultColumns[0];
        if rowIndicatorColumn.is_none() {
            rowIndicatorColumn = &self.detectionRXingResultColumns[self.barcodeColumnCount + 1];
        }
        // try (Formatter formatter = new Formatter()) {
        for codewordsRow in 0..rowIndicatorColumn.as_ref().unwrap().getCodewords().len() {
            //   for (int codewordsRow = 0; codewordsRow < rowIndicatorColumn.getCodewords().length; codewordsRow++) {
            write!(f, "CW {0:3}", codewordsRow)?;
            // formatter.format("CW %3d:", codewordsRow);
            for barcodeColumn in 0..self.barcodeColumnCount + 2 {
                // for (int barcodeColumn = 0; barcodeColumn < barcodeColumnCount + 2; barcodeColumn++) {
                if self.detectionRXingResultColumns[barcodeColumn].is_none() {
                    write!(f, "{}", "    |   ")?;
                    // formatter.format("    |   ");
                    continue;
                }
                let codeword = self.detectionRXingResultColumns[barcodeColumn]
                    .as_ref()
                    .unwrap()
                    .getCodewords()[codewordsRow];
                if codeword.is_none() {
                    write!(f, "{}", "    |   ")?;
                    // formatter.format("    |   ");
                    continue;
                }
                write!(
                    f,
                    " {}|{}",
                    codeword.as_ref().unwrap().getRowNumber(),
                    codeword.as_ref().unwrap().getValue()
                )?;
                //   formatter.format(" %3d|%3d", codeword.getRowNumber(), codeword.getValue());
            }
            // formatter.format("%n");
            write!(f, "{}", "\n")?;
        }
        //   return formatter.toString();
        write!(f, "")
        // }
    }
}

// @Override
//   public String toString() {
//     DetectionRXingResultColumn rowIndicatorColumn = detectionRXingResultColumns[0];
//     if (rowIndicatorColumn == null) {
//       rowIndicatorColumn = detectionRXingResultColumns[barcodeColumnCount + 1];
//     }
//     try (Formatter formatter = new Formatter()) {
//       for (int codewordsRow = 0; codewordsRow < rowIndicatorColumn.getCodewords().length; codewordsRow++) {
//         formatter.format("CW %3d:", codewordsRow);
//         for (int barcodeColumn = 0; barcodeColumn < barcodeColumnCount + 2; barcodeColumn++) {
//           if (detectionRXingResultColumns[barcodeColumn] == null) {
//             formatter.format("    |   ");
//             continue;
//           }
//           Codeword codeword = detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow];
//           if (codeword == null) {
//             formatter.format("    |   ");
//             continue;
//           }
//           formatter.format(" %3d|%3d", codeword.getRowNumber(), codeword.getValue());
//         }
//         formatter.format("%n");
//       }
//       return formatter.toString();
//     }
//   }
