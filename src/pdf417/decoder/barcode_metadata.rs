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

/**
 * @author Guenther Grau
 */
pub struct BarcodeMetadata {
    columnCount: u32,
    errorCorrectionLevel: u32,
    rowCountUpperPart: u32,
    rowCountLowerPart: u32,
    rowCount: u32,
}
impl BarcodeMetadata {
    pub fn new(
        columnCount: u32,
        rowCountUpperPart: u32,
        rowCountLowerPart: u32,
        errorCorrectionLevel: u32,
    ) -> Self {
        Self {
            columnCount,
            errorCorrectionLevel,
            rowCountUpperPart,
            rowCountLowerPart,
            rowCount: rowCountUpperPart + rowCountLowerPart,
        }
    }

    pub fn getColumnCount(&self) -> u32 {
        self.columnCount
    }

    pub fn getErrorCorrectionLevel(&self) -> u32 {
        self.errorCorrectionLevel
    }

    pub fn getRowCount(&self) -> u32 {
        self.rowCount
    }

    pub fn getRowCountUpperPart(&self) -> u32 {
        self.rowCountUpperPart
    }

    pub fn getRowCountLowerPart(&self) -> u32 {
        self.rowCountLowerPart
    }
}
