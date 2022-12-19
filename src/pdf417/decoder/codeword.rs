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

use std::fmt::Display;

const BARCODE_ROW_UNKNOWN: i32 = -1;

/**
 * @author Guenther Grau
 */
#[derive(Clone, Copy)]
pub struct Codeword {
    startX: u32,
    endX: u32,
    bucket: u32,
    value: u32,
    rowNumber: i32,
}

impl Codeword {
    pub fn new(startX: u32, endX: u32, bucket: u32, value: u32) -> Self {
        Self {
            startX,
            endX,
            bucket,
            value,
            rowNumber: BARCODE_ROW_UNKNOWN,
        }
    }

    pub fn hasValidRowNumber(&self) -> bool {
        self.isValidRowNumber(self.rowNumber)
    }

    pub fn isValidRowNumber(&self, rowNumber: i32) -> bool {
        rowNumber != BARCODE_ROW_UNKNOWN && self.bucket == (rowNumber as u32 % 3) * 3
    }

    pub fn setRowNumberAsRowIndicatorColumn(&mut self) {
        self.rowNumber = ((self.value / 30) * 3 + self.bucket / 3) as i32;
    }

    pub fn getWidth(&self) -> u32 {
        self.endX - self.startX
    }

    pub fn getStartX(&self) -> u32 {
        self.startX
    }

    pub fn getEndX(&self) -> u32 {
        self.endX
    }

    pub fn getBucket(&self) -> u32 {
        self.bucket
    }

    pub fn getValue(&self) -> u32 {
        self.value
    }

    pub fn getRowNumber(&self) -> i32 {
        self.rowNumber
    }

    pub fn setRowNumber(&mut self, rowNumber: i32) {
        self.rowNumber = rowNumber;
    }
}

impl Display for Codeword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.rowNumber, self.value)
    }
}
