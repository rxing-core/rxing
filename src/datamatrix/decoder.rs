use crate::{FormatException,ChecksumException,FormatException};
use crate::common::{BitMatrix,DecoderResult,BitSource,ECIStringBuilder};
use crate::common::reedsolomon::{GenericGF,ReedSolomonDecoder,ReedSolomonException};


// BitMatrixParser.java
/**
 * @author bbrown@google.com (Brian Brown)
 */
struct BitMatrixParser {

     mapping_bit_matrix: BitMatrix,

     read_mapping_matrix: BitMatrix,

      version: Version
}

impl BitMatrixParser {

   /**
  * @param bitMatrix {@link BitMatrix} to parse
  * @throws FormatException if dimension is < 8 or > 144 or not 0 mod 2
  */
   fn new( bit_matrix: &BitMatrix) -> Result<Self, FormatException> {
        let dimension: i32 = bit_matrix.get_height();
       if dimension < 8 || dimension > 144 || (dimension & 0x01) != 0 {
           return Err( FormatException::get_format_instance());
       }
       let new_bmp : Self;
       new_bmp.version = ::read_version(bit_matrix);
       new_bmp .mappingBitMatrix = self.extract_data_region(bit_matrix);
       new_bmp .readMappingMatrix = BitMatrix::new(&new_bmp.mappingBitMatrix.get_width(), &new_bmp.mappingBitMatrix.get_height());

   Ok(new_bmp)
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
   fn  read_codewords(&self) -> Result<Vec<i8>, FormatException>   {
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
               result[result_offset += 1 ] = self.read_corner1(num_rows, num_columns) as i8;
               row -= 2;
               column += 2;
               corner1_read = true;
           } else if (row == num_rows - 2) && (column == 0) && ((num_columns & 0x03) != 0) && !corner2_read {
               result[result_offset += 1 ] = self.read_corner2(num_rows, num_columns) as i8;
               row -= 2;
               column += 2;
               corner2_read = true;
           } else if (row == num_rows + 4) && (column == 2) && ((num_columns & 0x07) == 0) && !corner3_read {
               result[result_offset += 1 ] = self.read_corner3(num_rows, num_columns) as i8;
               row -= 2;
               column += 2;
               corner3_read = true;
           } else if (row == num_rows - 2) && (column == 0) && ((num_columns & 0x07) == 4) && !corner4_read {
               result[result_offset += 1 ] = self.read_corner4(num_rows, num_columns) as i8;
               row -= 2;
               column += 2;
               corner4_read = true;
           } else {
               // Sweep upward diagonally to the right
               loop { {
                   if (row < num_rows) && (column >= 0) && !self.read_mapping_matrix.get(column, row) {
                       result[result_offset += 1 ] = self.read_utah(row, column, num_rows, num_columns) as i8;
                   }
                   row -= 2;
                   column += 2;
               }if !((row >= 0) && (column < num_columns)) {break;}}
               row += 1;
               column += 3;
               // Sweep downward diagonally to the left
               loop { {
                   if (row >= 0) && (column < num_columns) && !self.read_mapping_matrix.get(column, row) {
                       result[result_offset += 1 ] = self.read_utah(row, column, num_rows, num_columns) as i8;
                   }
                   row += 2;
                   column -= 2;
            }if !((row < num_rows) && (column >= 0)) {break;}}
               row += 3;
               column += 1;
           }
    }if !((row < num_rows) || (column < num_columns)) {break;}}
       if result_offset != self.version.get_total_codewords() {
           return Err( FormatException::get_format_instance());
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
           return Err( IllegalArgumentException::new("Dimension of bitMatrix must match the version size"));
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


// DataBlock.java
/**
 * <p>Encapsulates a block of data within a Data Matrix Code. Data Matrix Codes may split their data into
 * multiple blocks, each of which is a unit of data and error-correction codewords. Each
 * is represented by an instance of this class.</p>
 *
 * @author bbrown@google.com (Brian Brown)
 */
struct DataBlock {

     num_data_codewords: i32,

     codewords: Vec<i8>
}

impl DataBlock {

   fn new( num_data_codewords: i32,  codewords: &Vec<i8>) -> Self {
Self {
    num_data_codewords,
    codewords: codewords,
}}

   /**
  * <p>When Data Matrix Codes use multiple data blocks, they actually interleave the bytes of each of them.
  * That is, the first byte of data block 1 to n is written, then the second bytes, and so on. This
  * method will separate the data into original blocks.</p>
  *
  * @param rawCodewords bytes as read directly from the Data Matrix Code
  * @param version version of the Data Matrix Code
  * @return DataBlocks containing original bytes, "de-interleaved" from representation in the
  *         Data Matrix Code
  */
   fn  get_data_blocks( raw_codewords: &Vec<i8>,  version: &Version) -> Vec<DataBlock>  {
       // Figure out the number and size of data blocks used by this version
        let ec_blocks: Version::ECBlocks = version.get_e_c_blocks();
       // First count the total number of data blocks
        let total_blocks: i32 = 0;
        let ec_block_array: Vec<Version::ECB> = ec_blocks.get_e_c_blocks();
       for   ec_block in ec_block_array {
           total_blocks += ec_block.get_count();
       }
       // Now establish DataBlocks of the appropriate size and number of data codewords
        let mut result: [Option<DataBlock>; total_blocks] = [None; total_blocks];
        let num_result_blocks: i32 = 0;
       for   ec_block in ec_block_array {
            {
                let mut i: i32 = 0;
               while i < ec_block.get_count() {
                   {
                        let num_data_codewords: i32 = ec_block.get_data_codewords();
                        let num_block_codewords: i32 = ec_blocks.get_e_c_codewords() + num_data_codewords;
                       result[num_result_blocks += 1 ] = DataBlock::new(num_data_codewords,  [0; num_block_codewords]);
                   }
                   i += 1;
                }
            }

       }
       // All blocks have the same amount of data, except that the last n
       // (where n may be 0) have 1 less byte. Figure out where these start.
       // TODO(bbrown): There is only one case where there is a difference for Data Matrix for size 144
        let longer_blocks_total_codewords: i32 = result[0].codewords.len();
       //int shorterBlocksTotalCodewords = longerBlocksTotalCodewords - 1;
        let longer_blocks_num_data_codewords: i32 = longer_blocks_total_codewords - ec_blocks.get_e_c_codewords();
        let shorter_blocks_num_data_codewords: i32 = longer_blocks_num_data_codewords - 1;
       // The last elements of result may be 1 element shorter for 144 matrix
       // first fill out as many elements as all of them have minus 1
        let raw_codewords_offset: i32 = 0;
        {
            let mut i: i32 = 0;
           while i < shorter_blocks_num_data_codewords {
               {
                    {
                        let mut j: i32 = 0;
                       while j < num_result_blocks {
                           {
                               result[j].codewords[i] = raw_codewords[raw_codewords_offset += 1 ];
                           }
                           j += 1;
                        }
                    }

               }
               i += 1;
            }
        }

       // Fill out the last data block in the longer ones
        let special_version: bool = version.get_version_number() == 24;
        let num_longer_blocks: i32 =  if special_version { 8 } else { num_result_blocks };
        {
            let mut j: i32 = 0;
           while j < num_longer_blocks {
               {
                   result[j].codewords[longer_blocks_num_data_codewords - 1] = raw_codewords[raw_codewords_offset += 1 ];
               }
               j += 1;
            }
        }

       // Now add in error correction blocks
        let max: i32 = result[0].codewords.len();
        {
            let mut i: i32 = longer_blocks_num_data_codewords;
           while i < max {
               {
                    {
                        let mut j: i32 = 0;
                       while j < num_result_blocks {
                           {
                                let j_offset: i32 =  if special_version { (j + 8) % num_result_blocks } else { j };
                                let i_offset: i32 =  if special_version && j_offset > 7 { i - 1 } else { i };
                               result[j_offset].codewords[i_offset] = raw_codewords[raw_codewords_offset += 1 ];
                           }
                           j += 1;
                        }
                    }

               }
               i += 1;
            }
        }

       if raw_codewords_offset != raw_codewords.len() {
           return Err( IllegalArgumentException::new());
       }
       return result;
   }

   fn  get_num_data_codewords(&self) -> i32  {
       return self.num_data_codewords;
   }

   fn  get_codewords(&self) -> Vec<i8>  {
       return self.codewords;
   }
}


// DecodedBitStreamParser.java
/**
 * <p>Data Matrix Codes can encode text as bits in one of several modes, and can use multiple modes
 * in one Data Matrix Code. This class decodes the bits back into text.</p>
 *
 * <p>See ISO 16022:2006, 5.2.1 - 5.2.9.2</p>
 *
 * @author bbrown@google.com (Brian Brown)
 * @author Sean Owen
 */

/**
   * See ISO 16022:2006, Annex C Table C.1
   * The C40 Basic Character Set (*'s used for placeholders for the shift values)
   */
  const C40_BASIC_SET_CHARS: vec![Vec<char>; 40] = vec!['*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ]
  ;
  
   const C40_SHIFT2_SET_CHARS: vec![Vec<char>; 27] = vec!['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', ]
  ;
  
   const TEXT_BASIC_SET_CHARS: vec![Vec<char>; 40] = vec!['*', '*', '*', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ]
  ;
  
  // Shift 2 for Text is the same encoding as C40
   const TEXT_SHIFT2_SET_CHARS: Vec<char> = C40_SHIFT2_SET_CHARS;
  
   const TEXT_SHIFT3_SET_CHARS: vec![Vec<char>; 32] = vec!['`', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '{', '|', '}', '~', 127 as char, ]
  ;

  enum Mode {
  
    // Not really a mode
    PAD_ENCODE(), ASCII_ENCODE(), C40_ENCODE(), TEXT_ENCODE(), ANSIX12_ENCODE(), EDIFACT_ENCODE(), BASE256_ENCODE(), ECI_ENCODE()
}

  struct DecodedBitStreamParser {
  }
  
  impl DecodedBitStreamParser {
  
      
  
      fn new() -> DecodedBitStreamParser {
      }
  
      fn  decode( bytes: &Vec<i8>) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
           let bits: BitSource = BitSource::new(&bytes);
           let result: ECIStringBuilder = ECIStringBuilder::new();
           let result_trailer: StringBuilder = StringBuilder::new(0);
           let byte_segments: List<Vec<i8>> = Vec::new();
           let mut mode: Mode = Mode::ASCII_ENCODE;
          // Could look directly at 'bytes', if we're sure of not having to account for multi byte values
           let fnc1_positions: Set<Integer> = HashSet::new();
           let symbology_modifier: i32;
           let is_e_c_iencoded: bool = false;
          loop { {
              if mode == Mode::ASCII_ENCODE {
                  mode = ::decode_ascii_segment(bits, result, &result_trailer, &fnc1_positions);
              } else {
                  match mode {
                        C40_ENCODE => 
                           {
                              ::decode_c40_segment(bits, result, &fnc1_positions);
                              break;
                          }
                        TEXT_ENCODE => 
                           {
                              ::decode_text_segment(bits, result, &fnc1_positions);
                              break;
                          }
                        ANSIX12_ENCODE => 
                           {
                              ::decode_ansi_x12_segment(bits, result);
                              break;
                          }
                        EDIFACT_ENCODE => 
                           {
                              ::decode_edifact_segment(bits, result);
                              break;
                          }
                        BASE256_ENCODE => 
                           {
                              ::decode_base256_segment(bits, result, &byte_segments);
                              break;
                          }
                        ECI_ENCODE => 
                           {
                              ::decode_e_c_i_segment(bits, result);
                              // ECI detection only, atm continue decoding as ASCII
                              is_e_c_iencoded = true;
                              break;
                          }
                      _ => 
                           {
                              return Err( FormatException::get_format_instance());
                          }
                  }
                  mode = Mode::ASCII_ENCODE;
              }
          }if !(mode != Mode::PAD_ENCODE && bits.available() > 0) {break;}}
          if result_trailer.length() > 0 {
              result.append_characters(&result_trailer);
          }
          if is_e_c_iencoded {
              // https://honeywellaidc.force.com/supportppr/s/article/List-of-barcode-symbology-AIM-Identifiers
              if fnc1_positions.contains(0) || fnc1_positions.contains(4) {
                  symbology_modifier = 5;
              } else if fnc1_positions.contains(1) || fnc1_positions.contains(5) {
                  symbology_modifier = 6;
              } else {
                  symbology_modifier = 4;
              }
          } else {
              if fnc1_positions.contains(0) || fnc1_positions.contains(4) {
                  symbology_modifier = 2;
              } else if fnc1_positions.contains(1) || fnc1_positions.contains(5) {
                  symbology_modifier = 3;
              } else {
                  symbology_modifier = 1;
              }
              
        
        
          }
          return Ok(DecoderResult::new(&bytes, &result.to_string(), &byte_segments , null, Some(symbology_modifier), None, None));
      }
  
      /**
     * See ISO 16022:2006, 5.2.3 and Annex C, Table C.2
     */
      fn  decode_ascii_segment( bits: &BitSource,  result: &ECIStringBuilder,  result_trailer: &StringBuilder,  fnc1positions: &Set<Integer>) -> /*  throws FormatException */Result<Mode, Rc<Exception>>   {
           let upper_shift: bool = false;
          loop { {
               let one_byte: i32 = bits.read_bits(8);
              if one_byte == 0 {
                  return Err( FormatException::get_format_instance());
              } else if one_byte <= 128 {
                  // ASCII data (ASCII value + 1)
                  if upper_shift {
                      one_byte += 128;
                  //upperShift = false;
                  }
                  result.append((one_byte - 1) as char);
                  return Ok(Mode::ASCII_ENCODE);
              } else if one_byte == 129 {
                  // Pad
                  return Ok(Mode::PAD_ENCODE);
              } else if one_byte <= 229 {
                  // 2-digit data 00-99 (Numeric Value + 130)
                   let value: i32 = one_byte - 130;
                  if value < 10 {
                      // pad with '0' for single digit values
                      result.append('0');
                  }
                  result.append(value);
              } else {
                  match one_byte {
                        // Latch to C40 encodation
                      230 => 
                           {
                              return Ok(Mode::C40_ENCODE);
                          }
                        // Latch to Base 256 encodation
                      231 => 
                           {
                              return Ok(Mode::BASE256_ENCODE);
                          }
                        // FNC1
                      232 => 
                           {
                              fnc1positions.add(&result.length());
                              // translate as ASCII 29
                              result.append(29 as char);
                              break;
                          }
                      // Structured Append
                        233 => 
                           {
                          }
                        // Reader Programming
                      234 => 
                           {
                              //throw ReaderException.getInstance();
                              break;
                          }
                        // Upper Shift (shift to Extended ASCII)
                      235 => 
                           {
                              upper_shift = true;
                              break;
                          }
                        // 05 Macro
                      236 => 
                           {
                              result.append("[)>\u{001E05}\u{001D}");
                              result_trailer.insert(0, "\u{001E}\u{0004}");
                              break;
                          }
                        // 06 Macro
                      237 => 
                           {
                            result.append("[)>\u{001E06}\u{001D}");
                            resultTrailer.insert(0, "\u{001E}\u{0004}");
                              break;
                          }
                        // Latch to ANSI X12 encodation
                      238 => 
                           {
                              return Ok(Mode::ANSIX12_ENCODE);
                          }
                        // Latch to Text encodation
                      239 => 
                           {
                              return Ok(Mode::TEXT_ENCODE);
                          }
                        // Latch to EDIFACT encodation
                      240 => 
                           {
                              return Ok(Mode::EDIFACT_ENCODE);
                          }
                        // ECI Character
                      241 => 
                           {
                              return Ok(Mode::ECI_ENCODE);
                          }
                      _ => 
                           {
                              // but work around encoders that end with 254, latch back to ASCII
                              if one_byte != 254 || bits.available() != 0 {
                                  return Err( FormatException::get_format_instance());
                              }
                              break;
                          }
                  }
              }
          }if !(bits.available() > 0) {break;}}
          return Ok(Mode::ASCII_ENCODE);
      }
  
      /**
     * See ISO 16022:2006, 5.2.5 and Annex C, Table C.1
     */
      fn  decode_c40_segment( bits: &BitSource,  result: &ECIStringBuilder,  fnc1positions: &Set<Integer>)  -> Result<(), FormatException>   {
          // Three C40 values are encoded in a 16-bit value as
          // (1600 * C1) + (40 * C2) + C3 + 1
          // TODO(bbrown): The Upper Shift with C40 doesn't work in the 4 value scenario all the time
           let upper_shift: bool = false;
           let c_values: [i32; 3] = [0; 3];
           let mut shift: i32 = 0;
          loop { {
              // If there is only one byte left then it will be encoded as ASCII
              if bits.available() == 8 {
                  return;
              }
               let first_byte: i32 = bits.read_bits(8);
              if first_byte == 254 {
                  // Unlatch codeword
                  return;
              }
              ::parse_two_bytes(first_byte, &bits.read_bits(8), &c_values);
               {
                   let mut i: i32 = 0;
                  while i < 3 {
                      {
                           let c_value: i32 = c_values[i];
                          match shift {
                                0 => 
                                   {
                                      if c_value < 3 {
                                          shift = c_value + 1;
                                      } else if c_value < C40_BASIC_SET_CHARS.len() {
                                           let c40char: char = C40_BASIC_SET_CHARS[c_value];
                                          if upper_shift {
                                              result.append((c40char + 128) as char);
                                              upper_shift = false;
                                          } else {
                                              result.append(c40char);
                                          }
                                      } else {
                                          return Err( FormatException::get_format_instance());
                                      }
                                      break;
                                  }
                                1 => 
                                   {
                                      if upper_shift {
                                          result.append((c_value + 128) as char);
                                          upper_shift = false;
                                      } else {
                                          result.append(c_value as char);
                                      }
                                      shift = 0;
                                      break;
                                  }
                                2 => 
                                   {
                                      if c_value < C40_SHIFT2_SET_CHARS.len() {
                                           let c40char: char = C40_SHIFT2_SET_CHARS[c_value];
                                          if upper_shift {
                                              result.append((c40char + 128) as char);
                                              upper_shift = false;
                                          } else {
                                              result.append(c40char);
                                          }
                                      } else {
                                          match c_value {
                                                // FNC1
                                              27 => 
                                                   {
                                                      fnc1positions.add(&result.length());
                                                      // translate as ASCII 29
                                                      result.append(29 as char);
                                                      break;
                                                  }
                                                // Upper Shift
                                              30 => 
                                                   {
                                                      upper_shift = true;
                                                      break;
                                                  }
                                              _ => 
                                                   {
                                                      return Err( FormatException::get_format_instance());
                                                  }
                                          }
                                      }
                                      shift = 0;
                                      break;
                                  }
                                3 => 
                                   {
                                      if upper_shift {
                                          result.append((c_value + 224) as char);
                                          upper_shift = false;
                                      } else {
                                          result.append((c_value + 96) as char);
                                      }
                                      shift = 0;
                                      break;
                                  }
                              _ => 
                                   {
                                      return Err( FormatException::get_format_instance());
                                  }
                          }
                      }
                      i += 1;
                   }
               }
  
          }if !(bits.available() > 0) {break;}}
          Ok(())
      }
  
      /**
     * See ISO 16022:2006, 5.2.6 and Annex C, Table C.2
     */
      fn  decode_text_segment( bits: &BitSource,  result: &ECIStringBuilder,  fnc1positions: &Set<Integer>)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
          // Three Text values are encoded in a 16-bit value as
          // (1600 * C1) + (40 * C2) + C3 + 1
          // TODO(bbrown): The Upper Shift with Text doesn't work in the 4 value scenario all the time
           let upper_shift: bool = false;
           let c_values: [i32; 3] = [0; 3];
           let mut shift: i32 = 0;
          loop { {
              // If there is only one byte left then it will be encoded as ASCII
              if bits.available() == 8 {
                  return;
              }
               let first_byte: i32 = bits.read_bits(8);
              if first_byte == 254 {
                  // Unlatch codeword
                  return;
              }
              ::parse_two_bytes(first_byte, &bits.read_bits(8), &c_values);
               {
                   let mut i: i32 = 0;
                  while i < 3 {
                      {
                           let c_value: i32 = c_values[i];
                          match shift {
                                0 => 
                                   {
                                      if c_value < 3 {
                                          shift = c_value + 1;
                                      } else if c_value < TEXT_BASIC_SET_CHARS.len() {
                                           let text_char: char = TEXT_BASIC_SET_CHARS[c_value];
                                          if upper_shift {
                                              result.append((text_char + 128) as char);
                                              upper_shift = false;
                                          } else {
                                              result.append(text_char);
                                          }
                                      } else {
                                          return Err( FormatException::get_format_instance());
                                      }
                                      break;
                                  }
                                1 => 
                                   {
                                      if upper_shift {
                                          result.append((c_value + 128) as char);
                                          upper_shift = false;
                                      } else {
                                          result.append(c_value as char);
                                      }
                                      shift = 0;
                                      break;
                                  }
                                2 => 
                                   {
                                      // Shift 2 for Text is the same encoding as C40
                                      if c_value < TEXT_SHIFT2_SET_CHARS.len() {
                                           let text_char: char = TEXT_SHIFT2_SET_CHARS[c_value];
                                          if upper_shift {
                                              result.append((text_char + 128) as char);
                                              upper_shift = false;
                                          } else {
                                              result.append(text_char);
                                          }
                                      } else {
                                          match c_value {
                                                // FNC1
                                              27 => 
                                                   {
                                                      fnc1positions.add(&result.length());
                                                      // translate as ASCII 29
                                                      result.append(29 as char);
                                                      break;
                                                  }
                                                // Upper Shift
                                              30 => 
                                                   {
                                                      upper_shift = true;
                                                      break;
                                                  }
                                              _ => 
                                                   {
                                                      return Err( FormatException::get_format_instance());
                                                  }
                                          }
                                      }
                                      shift = 0;
                                      break;
                                  }
                                3 => 
                                   {
                                      if c_value < TEXT_SHIFT3_SET_CHARS.len() {
                                           let text_char: char = TEXT_SHIFT3_SET_CHARS[c_value];
                                          if upper_shift {
                                              result.append((text_char + 128) as char);
                                              upper_shift = false;
                                          } else {
                                              result.append(text_char);
                                          }
                                          shift = 0;
                                      } else {
                                          return Err( FormatException::get_format_instance());
                                      }
                                      break;
                                  }
                              _ => 
                                   {
                                      return Err( FormatException::get_format_instance());
                                  }
                          }
                      }
                      i += 1;
                   }
               }
  
          }if !(bits.available() > 0) {break;}}
          Ok(())
      }
  
      /**
     * See ISO 16022:2006, 5.2.7
     */
      fn  decode_ansi_x12_segment( bits: &BitSource,  result: &ECIStringBuilder)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
          // Three ANSI X12 values are encoded in a 16-bit value as
          // (1600 * C1) + (40 * C2) + C3 + 1
           let c_values: [i32; 3] = [0; 3];
          loop { {
              // If there is only one byte left then it will be encoded as ASCII
              if bits.available() == 8 {
                  return;
              }
               let first_byte: i32 = bits.read_bits(8);
              if first_byte == 254 {
                  // Unlatch codeword
                  return;
              }
              ::parse_two_bytes(first_byte, &bits.read_bits(8), &c_values);
               {
                   let mut i: i32 = 0;
                  while i < 3 {
                      {
                           let c_value: i32 = c_values[i];
                          match c_value {
                                // X12 segment terminator <CR>
                              0 => 
                                   {
                                      result.append('\r');
                                      break;
                                  }
                                // X12 segment separator *
                              1 => 
                                   {
                                      result.append('*');
                                      break;
                                  }
                                // X12 sub-element separator >
                              2 => 
                                   {
                                      result.append('>');
                                      break;
                                  }
                                // space
                              3 => 
                                   {
                                      result.append(' ');
                                      break;
                                  }
                              _ => 
                                   {
                                      if c_value < 14 {
                                          // 0 - 9
                                          result.append((c_value + 44) as char);
                                      } else if c_value < 40 {
                                          // A - Z
                                          result.append((c_value + 51) as char);
                                      } else {
                                          return Err( FormatException::get_format_instance());
                                      }
                                      break;
                                  }
                          }
                      }
                      i += 1;
                   }
               }
  
          }if !(bits.available() > 0) {break;}}
          Ok(())
      }
  
      fn  parse_two_bytes( first_byte: i32,  second_byte: i32,  result: &Vec<i32>)   {
           let full_bit_value: i32 = (first_byte << 8) + second_byte - 1;
           let mut temp: i32 = full_bit_value / 1600;
          result[0] = temp;
          full_bit_value -= temp * 1600;
          temp = full_bit_value / 40;
          result[1] = temp;
          result[2] = full_bit_value - temp * 40;
      }
  
      /**
     * See ISO 16022:2006, 5.2.8 and Annex C Table C.3
     */
      fn  decode_edifact_segment( bits: &BitSource,  result: &ECIStringBuilder)   {
          loop { {
              // If there is only two or less bytes left then it will be encoded as ASCII
              if bits.available() <= 16 {
                  return;
              }
               {
                   let mut i: i32 = 0;
                  while i < 4 {
                      {
                           let edifact_value: i32 = bits.read_bits(6);
                          // Check for the unlatch character
                          if edifact_value == 0x1F {
                              // 011111
                              // Read rest of byte, which should be 0, and stop
                               let bits_left: i32 = 8 - bits.get_bit_offset();
                              if bits_left != 8 {
                                  bits.read_bits(bits_left);
                              }
                              return;
                          }
                          if (edifact_value & 0x20) == 0 {
                              // no 1 in the leading (6th) bit
                              // Add a leading 01 to the 6 bit binary value
                              edifact_value |= 0x40;
                          }
                          result.append(edifact_value as char);
                      }
                      i += 1;
                   }
               }
  
          }if !(bits.available() > 0) {break;}}
      }
  
      /**
     * See ISO 16022:2006, 5.2.9 and Annex B, B.2
     */
      fn  decode_base256_segment( bits: &BitSource,  result: &ECIStringBuilder,  byte_segments: &Collection<Vec<i8>>)  -> Result<(), FormatException>   {
          // Figure out how long the Base 256 Segment is.
          // position is 1-indexed
           let codeword_position: i32 = 1 + bits.get_byte_offset();
           let d1: i32 = ::unrandomize255_state(&bits.read_bits(8), codeword_position += 1 );
           let mut count: i32;
          if d1 == 0 {
              // Read the remainder of the symbol
              count = bits.available() / 8;
          } else if d1 < 250 {
              count = d1;
          } else {
              count = 250 * (d1 - 249) + ::unrandomize255_state(&bits.read_bits(8), codeword_position += 1 );
          }
          // We're seeing NegativeArraySizeException errors from users.
          if count < 0 {
              return Err( FormatException::get_format_instance());
          }
           let mut bytes: [i8; count] = [0; count];
           {
               let mut i: i32 = 0;
              while i < count {
                  {
                      // http://www.bcgen.com/demo/IDAutomationStreamingDataMatrix.aspx?MODE=3&D=Fred&PFMT=3&PT=F&X=0.3&O=0&LM=0.2
                      if bits.available() < 8 {
                          return Err( FormatException::get_format_instance());
                      }
                      bytes[i] = ::unrandomize255_state(&bits.read_bits(8), codeword_position += 1 ) as i8;
                  }
                  i += 1;
               }
           }
  
          byte_segments.add(&bytes);
          {
            use encoding::{Encoding,DecoderTrap};
            use encoding::all::ISO_8859_1;

            result.append(ISO_8859_1.decode(&bytes, DecoderTrap::Strict).unwrap_or("".to_owned()))
            // result.append(String::new(&bytes, StandardCharsets::ISO_8859_1));
          }
          Ok(())
      }
  
      /**
     * See ISO 16022:2007, 5.4.1
     */
      fn  decode_e_c_i_segment( bits: &BitSource,  result: &ECIStringBuilder)  -> Result<(),FormatException>   {
          if bits.available() < 8 {
              return Err( FormatException::get_format_instance());
          }
           let c1: i32 = bits.read_bits(8);
          if c1 <= 127 {
              result.append_e_c_i(c1 - 1);
          }
          Ok(())
      //currently we only support character set ECIs
      /*} else {
        if (bits.available() < 8) {
          throw FormatException.getFormatInstance();
        }
        int c2 = bits.readBits(8);
        if (c1 >= 128 && c1 <= 191) {
        } else {
          if (bits.available() < 8) {
            throw FormatException.getFormatInstance();
          }
          int c3 = bits.readBits(8);
        }
      }*/
      }
  
      /**
     * See ISO 16022:2006, Annex B, B.2
     */
      fn  unrandomize255_state( randomized_base256_codeword: i32,  base256_codeword_position: i32) -> i32  {
           let pseudo_random_number: i32 = ((149 * base256_codeword_position) % 255) + 1;
           let temp_variable: i32 = randomized_base256_codeword - pseudo_random_number;
          return  if temp_variable >= 0 { temp_variable } else { temp_variable + 256 };
      }
  }
  
  
  // Decoder.java
  /**
 * <p>The main class which implements Data Matrix Code decoding -- as opposed to locating and extracting
 * the Data Matrix Code from an image.</p>
 *
 * @author bbrown@google.com (Brian Brown)
 */
pub struct Decoder {

     rs_decoder: ReedSolomonDecoder
}

impl Decoder {

   pub fn new() -> Decoder {
       rs_decoder = ReedSolomonDecoder::new(GenericGF::DATA_MATRIX_FIELD_256);
   }

   /**
  * <p>Convenience method that can decode a Data Matrix Code represented as a 2D array of booleans.
  * "true" is taken to mean a black module.</p>
  *
  * @param image booleans representing white/black Data Matrix Code modules
  * @return text and bytes encoded within the Data Matrix Code
  * @throws FormatException if the Data Matrix Code cannot be decoded
  * @throws ChecksumException if error correction fails
  */
   pub fn  decode(&self,  image: &Vec<Vec<bool>>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
       return Ok(self.decode(&BitMatrix::parse(&image)));
   }

   /**
  * <p>Decodes a Data Matrix Code represented as a {@link BitMatrix}. A 1 or "true" is taken
  * to mean a black module.</p>
  *
  * @param bits booleans representing white/black Data Matrix Code modules
  * @return text and bytes encoded within the Data Matrix Code
  * @throws FormatException if the Data Matrix Code cannot be decoded
  * @throws ChecksumException if error correction fails
  */
   pub fn  decode(&self,  bits: &BitMatrix) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
       // Construct a parser and read version, error-correction level
        let parser: BitMatrixParser = BitMatrixParser::new(bits);
        let version: Version = parser.get_version();
       // Read codewords
        let codewords: Vec<i8> = parser.read_codewords();
       // Separate into data blocks
        let data_blocks: Vec<DataBlock> = DataBlock::get_data_blocks(&codewords, &version);
       // Count total number of data bytes
        let total_bytes: i32 = 0;
       for   db in data_blocks {
           total_bytes += db.get_num_data_codewords();
       }
        let result_bytes: [i8; total_bytes] = [0; total_bytes];
        let data_blocks_count: i32 = data_blocks.len();
       // Error-correct and copy data blocks together into a stream of bytes
        {
            let mut j: i32 = 0;
           while j < data_blocks_count {
               {
                    let data_block: DataBlock = data_blocks[j];
                    let codeword_bytes: Vec<i8> = data_block.get_codewords();
                    let num_data_codewords: i32 = data_block.get_num_data_codewords();
                   self.correct_errors(&codeword_bytes, num_data_codewords);
                    {
                        let mut i: i32 = 0;
                       while i < num_data_codewords {
                           {
                               // De-interlace data blocks.
                               result_bytes[i * data_blocks_count + j] = codeword_bytes[i];
                           }
                           i += 1;
                        }
                    }

               }
               j += 1;
            }
        }

       // Decode the contents of that stream of bytes
       return Ok(DecodedBitStreamParser::decode(&result_bytes));
   }

   /**
  * <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
  * correct the errors in-place using Reed-Solomon error correction.</p>
  *
  * @param codewordBytes data and error correction codewords
  * @param numDataCodewords number of codewords that are data bytes
  * @throws ChecksumException if error correction fails
  */
   fn  correct_errors(&self,  codeword_bytes: &Vec<i8>,  num_data_codewords: i32)  -> Result<(), ChecksumException>   {
        let num_codewords: i32 = codeword_bytes.len();
       // First read into an array of ints
        let codewords_ints: [i32; num_codewords] = [0; num_codewords];
        {
            let mut i: i32 = 0;
           while i < num_codewords {
               {
                   codewords_ints[i] = codeword_bytes[i] & 0xFF;
               }
               i += 1;
            }
        }

       
           self.rs_decoder.decode(&codewords_ints, codeword_bytes.len() - num_data_codewords);
       

       // We don't care about errors in the error-correction codewords
        {
            let mut i: i32 = 0;
           while i < num_data_codewords {
               {
                   codeword_bytes[i] = codewords_ints[i] as i8;
               }
               i += 1;
            }
        }
        Ok(())

   }
}


// Version.java
/**
 * The Version object encapsulates attributes about a particular
 * size Data Matrix Code.
 *
 * @author bbrown@google.com (Brian Brown)
 */

const VERSIONS: Vec<Version> = ::build_versions();
pub struct Version {

      version_number: i32,

      symbol_size_rows: i32,

      symbol_size_columns: i32,

      data_region_size_rows: i32,

      data_region_size_columns: i32,

      ec_blocks: ECBlocks,

      total_codewords: i32
}

impl Version {

    fn new( version_number: i32,  symbol_size_rows: i32,  symbol_size_columns: i32,  data_region_size_rows: i32,  data_region_size_columns: i32,  ec_blocks: &ECBlocks) -> Self {
        let new_v : Self;
        new_v .versionNumber = version_number;
        new_v .symbolSizeRows = symbol_size_rows;
        new_v .symbolSizeColumns = symbol_size_columns;
        new_v .dataRegionSizeRows = data_region_size_rows;
        new_v .dataRegionSizeColumns = data_region_size_columns;
        new_v .ecBlocks = ec_blocks;
        // Calculate the total number of codewords
         let mut total: i32 = 0;
         let ec_codewords: i32 = ec_blocks.get_e_c_codewords();
         let ecb_array: Vec<ECB> = ec_blocks.get_e_c_blocks();
        for   ec_block in ecb_array {
            total += ec_block.get_count() * (ec_block.get_data_codewords() + ec_codewords);
        }
        new_v .totalCodewords = total;

        new_v
    }

    pub fn  get_version_number(&self) -> i32  {
        return self.version_number;
    }

    pub fn  get_symbol_size_rows(&self) -> i32  {
        return self.symbol_size_rows;
    }

    pub fn  get_symbol_size_columns(&self) -> i32  {
        return self.symbol_size_columns;
    }

    pub fn  get_data_region_size_rows(&self) -> i32  {
        return self.data_region_size_rows;
    }

    pub fn  get_data_region_size_columns(&self) -> i32  {
        return self.data_region_size_columns;
    }

    pub fn  get_total_codewords(&self) -> i32  {
        return self.total_codewords;
    }

    fn  get_e_c_blocks(&self) -> ECBlocks  {
        return self.ec_blocks;
    }

    /**
   * <p>Deduces version information from Data Matrix dimensions.</p>
   *
   * @param numRows Number of rows in modules
   * @param numColumns Number of columns in modules
   * @return Version for a Data Matrix Code of those dimensions
   * @throws FormatException if dimensions do correspond to a valid Data Matrix size
   */
    pub fn  get_version_for_dimensions( num_rows: i32,  num_columns: i32) -> /*  throws FormatException */Result<Version, Rc<Exception>>   {
        if (num_rows & 0x01) != 0 || (num_columns & 0x01) != 0 {
            return Err( FormatException::get_format_instance());
        }
        for   version in VERSIONS {
            if version.symbolSizeRows == num_rows && version.symbolSizeColumns == num_columns {
                return Ok(version);
            }
        }
        return Err( FormatException::get_format_instance());
    }


    pub fn  to_string(&self) -> String  {
        return String::value_of(self.version_number);
    }

    /**
   * See ISO 16022:2006 5.5.1 Table 7
   */
    fn  build_versions() -> Vec<Version>  {
        return   vec![Version::new(1, 10, 10, 8, 8, &ECBlocks::new_simple(5, &ECB::new(1, 3))), Version::new(2, 12, 12, 10, 10, &ECBlocks::new_simple(7, &ECB::new(1, 5))), Version::new(3, 14, 14, 12, 12, &ECBlocks::new_simple(10, &ECB::new(1, 8))), Version::new(4, 16, 16, 14, 14, &ECBlocks::new_simple(12, &ECB::new(1, 12))), Version::new(5, 18, 18, 16, 16,& ECBlocks::new_simple(14, &ECB::new(1, 18))), Version::new(6, 20, 20, 18, 18, &ECBlocks::new_simple(18, &ECB::new(1, 22))), Version::new(7, 22, 22, 20, 20, &ECBlocks::new_simple(20, &ECB::new(1, 30))), Version::new(8, 24, 24, 22, 22,& ECBlocks::new_simple(24, &ECB::new(1, 36))), Version::new(9, 26, 26, 24, 24,& ECBlocks::new_simple(28, &ECB::new(1, 44))), Version::new(10, 32, 32, 14, 14,& ECBlocks::new_simple(36, &ECB::new(1, 62))), Version::new(11, 36, 36, 16, 16, &ECBlocks::new_simple(42,& ECB::new(1, 86))), Version::new(12, 40, 40, 18, 18, &ECBlocks::new_simple(48, &ECB::new(1, 114))), Version::new(13, 44, 44, 20, 20,& ECBlocks::new_simple(56,& ECB::new(1, 144))), Version::new(14, 48, 48, 22, 22,& ECBlocks::new_simple(68,& ECB::new(1, 174))), Version::new(15, 52, 52, 24, 24,& ECBlocks::new_simple(42, &ECB::new(2, 102))), Version::new(16, 64, 64, 14, 14,& ECBlocks::new_simple(56, &ECB::new(2, 140))), Version::new(17, 72, 72, 16, 16, &ECBlocks::new_simple(36, &ECB::new(4, 92))), Version::new(18, 80, 80, 18, 18,& ECBlocks::new_simple(48, &ECB::new(4, 114))), Version::new(19, 88, 88, 20, 20,& ECBlocks::new_simple(56, &ECB::new(4, 144))), Version::new(20, 96, 96, 22, 22, &ECBlocks::new_simple(68, &ECB::new(4, 174))), Version::new(21, 104, 104, 24, 24, &ECBlocks::new_simple(56, &ECB::new(6, 136))), Version::new(22, 120, 120, 18, 18, &ECBlocks::new_simple(68, &ECB::new(6, 175))), Version::new(23, 132, 132, 20, 20, &ECBlocks::new_simple(62, &ECB::new(8, 163))), Version::new(24, 144, 144, 22, 22, &ECBlocks::new(62, &ECB::new(8, 156), &ECB::new(2, 155))), Version::new(25, 8, 18, 6, 16, &ECBlocks::new_simple(7, &ECB::new(1, 5))), Version::new(26, 8, 32, 6, 14,& ECBlocks::new_simple(11, &ECB::new(1, 10))), Version::new(27, 12, 26, 10, 24,& ECBlocks::new_simple(14, &ECB::new(1, 16))), Version::new(28, 12, 36, 10, 16,& ECBlocks::new_simple(18, &ECB::new(1, 22))), Version::new(29, 16, 36, 14, 16, &ECBlocks::new_simple(24,& ECB::new(1, 32))), Version::new(30, 16, 48, 14, 22, &ECBlocks::new_simple(28, &ECB::new(1, 49))), // ISO 21471:2020 (DMRE) 5.5.1 Table 7
        Version::new(31, 8, 48, 6, 22,& ECBlocks::new_simple(15, &ECB::new(1, 18))), Version::new(32, 8, 64, 6, 14,& ECBlocks::new_simple(18, &ECB::new(1, 24))), Version::new(33, 8, 80, 6, 18, &ECBlocks::new_simple(22, &ECB::new(1, 32))), Version::new(34, 8, 96, 6, 22, &ECBlocks::new_simple(28, &ECB::new(1, 38))), Version::new(35, 8, 120, 6, 18, &ECBlocks::new_simple(32, &ECB::new(1, 49))), Version::new(36, 8, 144, 6, 22, &ECBlocks::new_simple(36, &ECB::new(1, 63))), Version::new(37, 12, 64, 10, 14, &ECBlocks::new_simple(27, &ECB::new(1, 43))), Version::new(38, 12, 88, 10, 20, &ECBlocks::new_simple(36, &ECB::new(1, 64))), Version::new(39, 16, 64, 14, 14,& ECBlocks::new_simple(36, &ECB::new(1, 62))), Version::new(40, 20, 36, 18, 16,& ECBlocks::new_simple(28, &ECB::new(1, 44))), Version::new(41, 20, 44, 18, 20,& ECBlocks::new_simple(34, &ECB::new(1, 56))), Version::new(42, 20, 64, 18, 14,& ECBlocks::new_simple(42,& ECB::new(1, 84))), Version::new(43, 22, 48, 20, 22, &ECBlocks::new_simple(38,& ECB::new(1, 72))), Version::new(44, 24, 48, 22, 22, &ECBlocks::new_simple(41, &ECB::new(1, 80))), Version::new(45, 24, 64, 22, 14, &ECBlocks::new_simple(46, &ECB::new(1, 108))), Version::new(46, 26, 40, 24, 18, &ECBlocks::new_simple(38, &ECB::new(1, 70))), Version::new(47, 26, 48, 24, 22, &ECBlocks::new_simple(42, &ECB::new(1, 90))), Version::new(48, 26, 64, 24, 14, &ECBlocks::new_simple(50, &ECB::new(1, 118))), ]
        ;
    }
}

/**
   * <p>Encapsulates a set of error-correction blocks in one symbol version. Most versions will
   * use blocks of differing sizes within one version, so, this encapsulates the parameters for
   * each set of blocks. It also holds the number of error-correction codewords per block since it
   * will be the same across all blocks within one version.</p>
   */
  struct ECBlocks {

    ec_codewords: i32,

    ec_blocks: Vec<ECB>,
}

impl ECBlocks {

  fn new_simple( ec_codewords: i32,  ec_blocks: &ECB) -> Self {
Self{
ec_codewords: ec_codewords,
ec_blocks: vec![ec_blocks, ]
}
  }

  fn new( ec_codewords: i32,  ec_blocks1: &ECB,  ec_blocks2: &ECB) -> ECBlocks {
      Self{
          ec_codewords: ec_codewords,
          ec_blocks:  vec![ec_blocks1, ec_blocks2, ]
      }
      
  }

  fn  get_e_c_codewords(&self) -> i32  {
      return self.ec_codewords;
  }

  fn  get_e_c_blocks(&self) -> Vec<ECB>  {
      return self.ec_blocks;
  }
}


/**
* <p>Encapsulates the parameters for one error-correction block in one symbol version.
* This includes the number of data codewords, and the number of times a block with these
* parameters is used consecutively in the Data Matrix code version's format.</p>
*/
struct ECB {

    count: i32,

    data_codewords: i32
}

impl ECB {

  fn new( count: i32,  data_codewords: i32) -> Self {
      Self {

          count,
          data_codewords
      }
  }

  fn  get_count(&self) -> i32  {
      return self.count;
  }

  fn  get_data_codewords(&self) -> i32  {
      return self.data_codewords;
  }
}