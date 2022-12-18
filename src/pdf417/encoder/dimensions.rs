/*
 * Copyright 2012 ZXing authors
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
 * Data object to specify the minimum and maximum number of rows and columns for a PDF417 barcode.
 *
 * @author qwandor@google.com (Andrew Walbran)
 */
pub struct Dimensions {
    minCols: usize,
    maxCols: usize,
    minRows: usize,
    maxRows: usize,
}
impl Dimensions {
    pub fn new(minCols: usize, maxCols: usize, minRows: usize, maxRows: usize) -> Self {
        Self {
            minCols,
            maxCols,
            minRows,
            maxRows,
        }
    }

    pub fn getMinCols(&self) -> usize {
        self.minCols
    }

    pub fn getMaxCols(&self) -> usize {
        self.maxCols
    }

    pub fn getMinRows(&self) -> usize {
        self.minRows
    }

    pub fn getMaxRows(&self) -> usize {
        self.maxRows
    }
}
