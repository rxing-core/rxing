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
// package com::google::zxing::datamatrix::decoder;

/**
 * @author bbrown@google.com (Brian Brown)
 */
struct BitMatrixParser {

     let mapping_bit_matrix: BitMatrix;

     let read_mapping_matrix: BitMatrix;

     let mut version: Version;
}

impl BitMatrixParser {

    /**
   * @param bitMatrix {@link BitMatrix} to parse
   * @throws FormatException if dimension is < 8 or > 144 or not 0 mod 2
   */
    fn new( bit_matrix: &BitMatrix) -> BitMatrixParser throws FormatException {
         let dimension: i32 = bit_matrix.get_height();
        if dimension < 8 || dimension > 144 || (dimension & 0x01) != 0 {
            throw FormatException::get_format_instance();
        }
        version = ::read_version(bit_matrix);
        let .mappingBitMatrix = self.extract_data_region(bit_matrix);
        let .readMappingMatrix = BitMatrix::new(&let .mappingBitMatrix.get_width(), &let .mappingBitMatrix.get_height());
    }

    fn  get_version(&self) -> Version  {
        return self.version;
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
    fn  read_version( bit_matrix: &BitMatrix) -> /*  throws FormatException */Result<Version, Rc<Exception>>   {
         let num_rows: i32 = bit_matrix.get_height();
         let num_columns: i32 = bit_matrix.get_width();
        return Ok(Version::get_version_for_dimensions(num_rows, num_columns));
    }

    /**
   * <p>Reads the bits in the {@link BitMatrix} representing the mapping matrix (No alignment patterns)
   * in the correct order in order to reconstitute the codewords bytes contained within the
   * Data Matrix Code.</p>
   *
   * @return bytes encoded within the Data Matrix Code
   * @throws FormatException if the exact number of bytes expected is not read
   */
    fn  read_codewords(&self) -> /*  throws FormatException */Result<Vec<i8>, Rc<Exception>>   {
         let mut result: [i8; self.version.get_total_codewords()] = [0; self.version.get_total_codewords()];
         let result_offset: i32 = 0;
         let mut row: i32 = 4;
         let mut column: i32 = 0;
         let num_rows: i32 = self.mapping_bit_matrix.get_height();
         let num_columns: i32 = self.mapping_bit_matrix.get_width();
         let corner1_read: bool = false;
         let corner2_read: bool = false;
         let corner3_read: bool = false;
         let corner4_read: bool = false;
        // Read all of the codewords
        loop { {
            // Check the four corner cases
            if (row == num_rows) && (column == 0) && !corner1_read {
                result[result_offset += 1 !!!check!!! post increment] = self.read_corner1(num_rows, num_columns) as i8;
                row -= 2;
                column += 2;
                corner1_read = true;
            } else if (row == num_rows - 2) && (column == 0) && ((num_columns & 0x03) != 0) && !corner2_read {
                result[result_offset += 1 !!!check!!! post increment] = self.read_corner2(num_rows, num_columns) as i8;
                row -= 2;
                column += 2;
                corner2_read = true;
            } else if (row == num_rows + 4) && (column == 2) && ((num_columns & 0x07) == 0) && !corner3_read {
                result[result_offset += 1 !!!check!!! post increment] = self.read_corner3(num_rows, num_columns) as i8;
                row -= 2;
                column += 2;
                corner3_read = true;
            } else if (row == num_rows - 2) && (column == 0) && ((num_columns & 0x07) == 4) && !corner4_read {
                result[result_offset += 1 !!!check!!! post increment] = self.read_corner4(num_rows, num_columns) as i8;
                row -= 2;
                column += 2;
                corner4_read = true;
            } else {
                // Sweep upward diagonally to the right
                loop { {
                    if (row < num_rows) && (column >= 0) && !self.read_mapping_matrix.get(column, row) {
                        result[result_offset += 1 !!!check!!! post increment] = self.read_utah(row, column, num_rows, num_columns) as i8;
                    }
                    row -= 2;
                    column += 2;
                }if !((row >= 0) && (column < num_columns)) break;}
                row += 1;
                column += 3;
                // Sweep downward diagonally to the left
                loop { {
                    if (row >= 0) && (column < num_columns) && !self.read_mapping_matrix.get(column, row) {
                        result[result_offset += 1 !!!check!!! post increment] = self.read_utah(row, column, num_rows, num_columns) as i8;
                    }
                    row += 2;
                    column -= 2;
                }if !((row < num_rows) && (column >= 0)) break;}
                row += 3;
                column += 1;
            }
        }if !((row < num_rows) || (column < num_columns)) break;}
        if result_offset != self.version.get_total_codewords() {
            throw FormatException::get_format_instance();
        }
        return Ok(result);
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
    fn  read_module(&self,  row: i32,  column: i32,  num_rows: i32,  num_columns: i32) -> bool  {
        // Adjust the row and column indices based on boundary wrapping
        if row < 0 {
            row += num_rows;
            column += 4 - ((num_rows + 4) & 0x07);
        }
        if column < 0 {
            column += num_columns;
            row += 4 - ((num_columns + 4) & 0x07);
        }
        if row >= num_rows {
            row -= num_rows;
        }
        self.read_mapping_matrix.set(column, row);
        return self.mapping_bit_matrix.get(column, row);
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
    fn  read_utah(&self,  row: i32,  column: i32,  num_rows: i32,  num_columns: i32) -> i32  {
         let current_byte: i32 = 0;
        if self.read_module(row - 2, column - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(row - 2, column - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(row - 1, column - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(row - 1, column - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(row - 1, column, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(row, column - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(row, column - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(row, column, num_rows, num_columns) {
            current_byte |= 1;
        }
        return current_byte;
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
    fn  read_corner1(&self,  num_rows: i32,  num_columns: i32) -> i32  {
         let current_byte: i32 = 0;
        if self.read_module(num_rows - 1, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(num_rows - 1, 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(num_rows - 1, 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(1, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(2, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(3, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        return current_byte;
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
    fn  read_corner2(&self,  num_rows: i32,  num_columns: i32) -> i32  {
         let current_byte: i32 = 0;
        if self.read_module(num_rows - 3, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(num_rows - 2, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(num_rows - 1, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 4, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 3, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(1, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        return current_byte;
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
    fn  read_corner3(&self,  num_rows: i32,  num_columns: i32) -> i32  {
         let current_byte: i32 = 0;
        if self.read_module(num_rows - 1, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(num_rows - 1, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 3, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(1, num_columns - 3, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(1, num_columns - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(1, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        return current_byte;
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
    fn  read_corner4(&self,  num_rows: i32,  num_columns: i32) -> i32  {
         let current_byte: i32 = 0;
        if self.read_module(num_rows - 3, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(num_rows - 2, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(num_rows - 1, 0, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 2, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(0, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(1, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(2, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        current_byte <<= 1;
        if self.read_module(3, num_columns - 1, num_rows, num_columns) {
            current_byte |= 1;
        }
        return current_byte;
    }

    /**
   * <p>Extracts the data region from a {@link BitMatrix} that contains
   * alignment patterns.</p>
   *
   * @param bitMatrix Original {@link BitMatrix} with alignment patterns
   * @return BitMatrix that has the alignment patterns removed
   */
    fn  extract_data_region(&self,  bit_matrix: &BitMatrix) -> BitMatrix  {
         let symbol_size_rows: i32 = self.version.get_symbol_size_rows();
         let symbol_size_columns: i32 = self.version.get_symbol_size_columns();
        if bit_matrix.get_height() != symbol_size_rows {
            throw IllegalArgumentException::new("Dimension of bitMatrix must match the version size");
        }
         let data_region_size_rows: i32 = self.version.get_data_region_size_rows();
         let data_region_size_columns: i32 = self.version.get_data_region_size_columns();
         let num_data_regions_row: i32 = symbol_size_rows / data_region_size_rows;
         let num_data_regions_column: i32 = symbol_size_columns / data_region_size_columns;
         let size_data_region_row: i32 = num_data_regions_row * data_region_size_rows;
         let size_data_region_column: i32 = num_data_regions_column * data_region_size_columns;
         let bit_matrix_without_alignment: BitMatrix = BitMatrix::new(size_data_region_column, size_data_region_row);
         {
             let data_region_row: i32 = 0;
            while data_region_row < num_data_regions_row {
                {
                     let data_region_row_offset: i32 = data_region_row * data_region_size_rows;
                     {
                         let data_region_column: i32 = 0;
                        while data_region_column < num_data_regions_column {
                            {
                                 let data_region_column_offset: i32 = data_region_column * data_region_size_columns;
                                 {
                                     let mut i: i32 = 0;
                                    while i < data_region_size_rows {
                                        {
                                             let read_row_offset: i32 = data_region_row * (data_region_size_rows + 2) + 1 + i;
                                             let write_row_offset: i32 = data_region_row_offset + i;
                                             {
                                                 let mut j: i32 = 0;
                                                while j < data_region_size_columns {
                                                    {
                                                         let read_column_offset: i32 = data_region_column * (data_region_size_columns + 2) + 1 + j;
                                                        if bit_matrix.get(read_column_offset, read_row_offset) {
                                                             let write_column_offset: i32 = data_region_column_offset + j;
                                                            bit_matrix_without_alignment.set(write_column_offset, write_row_offset);
                                                        }
                                                    }
                                                    j += 1;
                                                 }
                                             }

                                        }
                                        i += 1;
                                     }
                                 }

                            }
                            data_region_column += 1;
                         }
                     }

                }
                data_region_row += 1;
             }
         }

        return bit_matrix_without_alignment;
    }
}

