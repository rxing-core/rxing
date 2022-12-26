/*
 * Copyright 2011 ZXing authors
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

use super::BarcodeRow;

/**
 * Holds all of the information for a barcode in a format where it can be easily accessible
 *
 * @author Jacob Haynes
 */
pub struct BarcodeMatrix {
    matrix: Vec<BarcodeRow>,
    currentRow: isize,
    height: usize,
    width: usize,
}
impl BarcodeMatrix {
    /**
     * @param height the height of the matrix (Rows)
     * @param width  the width of the matrix (Cols)
     */
    pub fn new(height: usize, width: usize) -> Self {
        let mut matrix = Vec::new(); //new BarcodeRow[height];
                                     //Initializes the array to the correct width
        for _i in 0..height {
            // for (int i = 0, matrixLength = matrix.length; i < matrixLength; i++) {
            //matrix[i] = new BarcodeRow((width + 4) * 17 + 1);
            matrix.push(BarcodeRow::new((width + 4) * 17 + 1));
        }
        Self {
            matrix,
            currentRow: -1,
            height,
            width: width * 17,
        }
        // this.width = width * 17;
        // this.height = height;
        // this.currentRow = -1;
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.matrix[y].set(x, value);
    }

    pub fn startRow(&mut self) {
        self.currentRow += 1;
    }

    pub fn getCurrentRow(&self) -> &BarcodeRow {
        &self.matrix[self.currentRow as usize]
    }

    pub fn getCurrentRowMut(&mut self) -> &mut BarcodeRow {
        &mut self.matrix[self.currentRow as usize]
    }

    pub fn getMatrix(&self) -> Vec<Vec<u8>> {
        self.getScaledMatrix(1, 1)
    }

    pub fn getScaledMatrix(&self, xScale: usize, yScale: usize) -> Vec<Vec<u8>> {
        let mut matrixOut = vec![vec![0; self.width * xScale]; self.height * yScale];
        // byte[][] matrixOut = new byte[height * yScale][width * xScale];
        let yMax = self.height * yScale;
        for i in 0..yMax {
            // for (int i = 0; i < yMax; i++) {
            matrixOut[yMax - i - 1] = self.matrix[i / yScale].getScaledRow(xScale);
        }

        matrixOut
    }
}
