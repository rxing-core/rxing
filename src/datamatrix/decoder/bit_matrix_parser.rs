/*
 * Copyright 2007 ZXing authors
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

use crate::{common::BitMatrix, Exceptions};

use super::{Version, VersionRef};

/**
 * @author bbrown@google.com (Brian Brown)
 */
pub struct BitMatrixParser {
    mappingBitMatrix: BitMatrix,
    readMappingMatrix: BitMatrix,
    version: VersionRef,
}
impl BitMatrixParser {
    /**
     * @param bitMatrix {@link BitMatrix} to parse
     * @throws FormatException if dimension is < 8 or > 144 or not 0 mod 2
     */
    pub fn new(bitMatrix: &BitMatrix) -> Result<Self, Exceptions> {
        let dimension = bitMatrix.getHeight();
        if !(8..=144).contains(&dimension) || (dimension & 0x01) != 0 {
            return Err(Exceptions::FormatException(None));
        }

        let version = Self::readVersion(bitMatrix)?;
        let mappingBitMatrix = Self::extractDataRegion(bitMatrix, version)?;
        let readMappingMatrix =
            BitMatrix::new(mappingBitMatrix.getWidth(), mappingBitMatrix.getHeight())?;

        Ok(Self {
            mappingBitMatrix,
            readMappingMatrix,
            version,
        })
    }

    pub fn getVersion(&self) -> &Version {
        self.version
    }

    /**
     * <p>Creates the version object based on the dimension of the original bit matrix from
     * the datamatrix code.</p>
     *
     * <p>See ISO 16022:2006 Table 7 - ECC 200 symbol attributes</p>
     *
     * @param bitMatrix Original {@link BitMatrix} including alignment patterns
     * @return {@link Version} encapsulating the Data Matrix Code's "version"
     * @throws FormatException if the dimensions of the mapping matrix are not valid
     * Data Matrix dimensions.
     */
    fn readVersion(bitMatrix: &BitMatrix) -> Result<VersionRef, Exceptions> {
        let numRows = bitMatrix.getHeight();
        let numColumns = bitMatrix.getWidth();
        Version::getVersionForDimensions(numRows, numColumns)
    }

    /**
     * <p>Reads the bits in the {@link BitMatrix} representing the mapping matrix (No alignment patterns)
     * in the correct order in order to reconstitute the codewords bytes contained within the
     * Data Matrix Code.</p>
     *
     * @return bytes encoded within the Data Matrix Code
     * @throws FormatException if the exact number of bytes expected is not read
     */
    pub fn readCodewords(&mut self) -> Result<Vec<u8>, Exceptions> {
        let mut result = vec![0u8; self.version.getTotalCodewords() as usize];
        let mut resultOffset = 0;

        let mut row = 4;
        let mut column = 0;

        let numRows = self.mappingBitMatrix.getHeight() as isize;
        let numColumns = self.mappingBitMatrix.getWidth() as isize;

        let mut corner1Read = false;
        let mut corner2Read = false;
        let mut corner3Read = false;
        let mut corner4Read = false;

        // Read all of the codewords
        loop {
            // Check the four corner cases
            if (row == numRows) && (column == 0) && !corner1Read {
                result[resultOffset] = self.readCorner1(numRows, numColumns) as u8;
                resultOffset += 1;
                row -= 2;
                column += 2;
                corner1Read = true;
            } else if (row == numRows - 2)
                && (column == 0)
                && ((numColumns & 0x03) != 0)
                && !corner2Read
            {
                result[resultOffset] = self.readCorner2(numRows, numColumns) as u8;
                resultOffset += 1;
                row -= 2;
                column += 2;
                corner2Read = true;
            } else if (row == numRows + 4)
                && (column == 2)
                && ((numColumns & 0x07) == 0)
                && !corner3Read
            {
                result[resultOffset] = self.readCorner3(numRows, numColumns) as u8;
                resultOffset += 1;
                row -= 2;
                column += 2;
                corner3Read = true;
            } else if (row == numRows - 2)
                && (column == 0)
                && ((numColumns & 0x07) == 4)
                && !corner4Read
            {
                result[resultOffset] = self.readCorner4(numRows, numColumns) as u8;
                resultOffset += 1;
                row -= 2;
                column += 2;
                corner4Read = true;
            } else {
                // Sweep upward diagonally to the right
                loop {
                    if (row < numRows)
                        && (column >= 0)
                        && !self.readMappingMatrix.get(column as u32, row as u32)
                    {
                        result[resultOffset] =
                            self.readUtah(row, column, numRows, numColumns) as u8;
                        resultOffset += 1;
                    }
                    row -= 2;
                    column += 2;
                    if !((row >= 0) && (column < numColumns)) {
                        break;
                    }
                }
                row += 1;
                column += 3;

                // Sweep downward diagonally to the left
                loop {
                    if (row >= 0)
                        && (column < numColumns)
                        && !self.readMappingMatrix.get(column as u32, row as u32)
                    {
                        result[resultOffset] =
                            self.readUtah(row, column, numRows, numColumns) as u8;
                        resultOffset += 1;
                    }
                    row += 2;
                    column -= 2;

                    if !((row < numRows) && (column >= 0)) {
                        break;
                    }
                }
                row += 3;
                column += 1;
            }
            if !((row < numRows) || (column < numColumns)) {
                break;
            }
        }

        if resultOffset != self.version.getTotalCodewords() as usize {
            return Err(Exceptions::FormatException(None));
        }

        Ok(result)
    }

    /**
     * <p>Reads a bit of the mapping matrix accounting for boundary wrapping.</p>
     *
     * @param row Row to read in the mapping matrix
     * @param column Column to read in the mapping matrix
     * @param numRows Number of rows in the mapping matrix
     * @param numColumns Number of columns in the mapping matrix
     * @return value of the given bit in the mapping matrix
     */
    fn readModule(&mut self, row: isize, column: isize, numRows: isize, numColumns: isize) -> bool {
        let mut row = row;
        let mut column = column;
        // Adjust the row and column indices based on boundary wrapping
        if row < 0 {
            row += numRows;
            column += 4 - ((numRows + 4) & 0x07);
        }
        if column < 0 {
            column += numColumns;
            row += 4 - ((numColumns + 4) & 0x07);
        }
        if row >= numRows {
            row -= numRows;
        }
        self.readMappingMatrix.set(column as u32, row as u32);

        self.mappingBitMatrix.get(column as u32, row as u32)
    }

    /**
     * <p>Reads the 8 bits of the standard Utah-shaped pattern.</p>
     *
     * <p>See ISO 16022:2006, 5.8.1 Figure 6</p>
     *
     * @param row Current row in the mapping matrix, anchored at the 8th bit (LSB) of the pattern
     * @param column Current column in the mapping matrix, anchored at the 8th bit (LSB) of the pattern
     * @param numRows Number of rows in the mapping matrix
     * @param numColumns Number of columns in the mapping matrix
     * @return byte from the utah shape
     */
    fn readUtah(&mut self, row: isize, column: isize, numRows: isize, numColumns: isize) -> u32 {
        let mut currentByte = 0;
        if self.readModule(row - 2, column - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(row - 2, column - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(row - 1, column - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(row - 1, column - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(row - 1, column, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(row, column - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(row, column - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(row, column, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte
    }

    /**
     * <p>Reads the 8 bits of the special corner condition 1.</p>
     *
     * <p>See ISO 16022:2006, Figure F.3</p>
     *
     * @param numRows Number of rows in the mapping matrix
     * @param numColumns Number of columns in the mapping matrix
     * @return byte from the Corner condition 1
     */
    fn readCorner1(&mut self, numRows: isize, numColumns: isize) -> u32 {
        let mut currentByte = 0;
        if self.readModule(numRows - 1, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(numRows - 1, 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(numRows - 1, 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(1, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(2, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(3, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte
    }

    /**
     * <p>Reads the 8 bits of the special corner condition 2.</p>
     *
     * <p>See ISO 16022:2006, Figure F.4</p>
     *
     * @param numRows Number of rows in the mapping matrix
     * @param numColumns Number of columns in the mapping matrix
     * @return byte from the Corner condition 2
     */
    fn readCorner2(&mut self, numRows: isize, numColumns: isize) -> u32 {
        let mut currentByte = 0;
        if self.readModule(numRows - 3, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(numRows - 2, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(numRows - 1, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 4, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 3, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(1, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte
    }

    /**
     * <p>Reads the 8 bits of the special corner condition 3.</p>
     *
     * <p>See ISO 16022:2006, Figure F.5</p>
     *
     * @param numRows Number of rows in the mapping matrix
     * @param numColumns Number of columns in the mapping matrix
     * @return byte from the Corner condition 3
     */
    fn readCorner3(&mut self, numRows: isize, numColumns: isize) -> u32 {
        let mut currentByte = 0;
        if self.readModule(numRows - 1, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(numRows - 1, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 3, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(1, numColumns - 3, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(1, numColumns - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(1, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte
    }

    /**
     * <p>Reads the 8 bits of the special corner condition 4.</p>
     *
     * <p>See ISO 16022:2006, Figure F.6</p>
     *
     * @param numRows Number of rows in the mapping matrix
     * @param numColumns Number of columns in the mapping matrix
     * @return byte from the Corner condition 4
     */
    fn readCorner4(&mut self, numRows: isize, numColumns: isize) -> u32 {
        let mut currentByte = 0;
        if self.readModule(numRows - 3, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(numRows - 2, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(numRows - 1, 0, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 2, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(0, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(1, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(2, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte <<= 1;
        if self.readModule(3, numColumns - 1, numRows, numColumns) {
            currentByte |= 1;
        }
        currentByte
    }

    /**
     * <p>Extracts the data region from a {@link BitMatrix} that contains
     * alignment patterns.</p>
     *
     * @param bitMatrix Original {@link BitMatrix} with alignment patterns
     * @return BitMatrix that has the alignment patterns removed
     */
    fn extractDataRegion(
        bitMatrix: &BitMatrix,
        version: VersionRef,
    ) -> Result<BitMatrix, Exceptions> {
        // dbg!(bitMatrix.to_string());
        let symbolSizeRows = version.getSymbolSizeRows();
        let symbolSizeColumns = version.getSymbolSizeColumns();

        if bitMatrix.getHeight() != symbolSizeRows {
            return Err(Exceptions::IllegalArgumentException(Some(
                "Dimension of bitMatrix must match the version size".to_owned(),
            )));
        }

        let dataRegionSizeRows = version.getDataRegionSizeRows();
        let dataRegionSizeColumns = version.getDataRegionSizeColumns();

        let numDataRegionsRow = symbolSizeRows / dataRegionSizeRows;
        let numDataRegionsColumn = symbolSizeColumns / dataRegionSizeColumns;

        let sizeDataRegionRow = numDataRegionsRow * dataRegionSizeRows;
        let sizeDataRegionColumn = numDataRegionsColumn * dataRegionSizeColumns;

        let mut bitMatrixWithoutAlignment =
            BitMatrix::new(sizeDataRegionColumn, sizeDataRegionRow)?;
        for dataRegionRow in 0..numDataRegionsRow {
            // for (int dataRegionRow = 0; dataRegionRow < numDataRegionsRow; ++dataRegionRow) {
            let dataRegionRowOffset = dataRegionRow * dataRegionSizeRows;
            for dataRegionColumn in 0..numDataRegionsColumn {
                // for (int dataRegionColumn = 0; dataRegionColumn < numDataRegionsColumn; ++dataRegionColumn) {
                let dataRegionColumnOffset = dataRegionColumn * dataRegionSizeColumns;
                for i in 0..dataRegionSizeRows {
                    // for (int i = 0; i < dataRegionSizeRows; ++i) {
                    let readRowOffset = dataRegionRow * (dataRegionSizeRows + 2) + 1 + i;
                    let writeRowOffset = dataRegionRowOffset + i;
                    for j in 0..dataRegionSizeColumns {
                        // for (int j = 0; j < dataRegionSizeColumns; ++j) {
                        let readColumnOffset =
                            dataRegionColumn * (dataRegionSizeColumns + 2) + 1 + j;
                        if bitMatrix.get(readColumnOffset, readRowOffset) {
                            let writeColumnOffset = dataRegionColumnOffset + j;
                            bitMatrixWithoutAlignment.set(writeColumnOffset, writeRowOffset);
                        }
                    }
                }
            }
        }
        Ok(bitMatrixWithoutAlignment)
    }
}
