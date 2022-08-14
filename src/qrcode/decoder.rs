use crate::{FormatException,DecodeHintType,ChecksumException,ResultPoint};
use crate::comon::{BitMatrix,BitSource,CharacterSetECI,DecoderResult,StringUtils};
use crate::common::reedsolomon::{GenericGF,ReedSolomonDecoder,ReedSolomonException};



// NEW FILE: bit_matrix_parser.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * @author Sean Owen
 */
struct BitMatrixParser {

     let bit_matrix: BitMatrix;

     let parsed_version: Version;

     let parsed_format_info: FormatInformation;

     let mirror: bool;
}

impl BitMatrixParser {

    /**
   * @param bitMatrix {@link BitMatrix} to parse
   * @throws FormatException if dimension is not >= 21 and 1 mod 4
   */
    fn new( bit_matrix: &BitMatrix) -> BitMatrixParser throws FormatException {
         let dimension: i32 = bit_matrix.get_height();
        if dimension < 21 || (dimension & 0x03) != 1 {
            throw FormatException::get_format_instance();
        }
        let .bitMatrix = bit_matrix;
    }

    /**
   * <p>Reads format information from one of its two locations within the QR Code.</p>
   *
   * @return {@link FormatInformation} encapsulating the QR Code's format info
   * @throws FormatException if both format information locations cannot be parsed as
   * the valid encoding of format information
   */
    fn  read_format_information(&self) -> /*  throws FormatException */Result<FormatInformation, Rc<Exception>>   {
        if self.parsed_format_info != null {
            return Ok(self.parsed_format_info);
        }
        // Read top-left format info bits
         let format_info_bits1: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 6 {
                {
                    format_info_bits1 = self.copy_bit(i, 8, format_info_bits1);
                }
                i += 1;
             }
         }

        // .. and skip a bit in the timing pattern ...
        format_info_bits1 = self.copy_bit(7, 8, format_info_bits1);
        format_info_bits1 = self.copy_bit(8, 8, format_info_bits1);
        format_info_bits1 = self.copy_bit(8, 7, format_info_bits1);
        // .. and skip a bit in the timing pattern ...
         {
             let mut j: i32 = 5;
            while j >= 0 {
                {
                    format_info_bits1 = self.copy_bit(8, j, format_info_bits1);
                }
                j -= 1;
             }
         }

        // Read the top-right/bottom-left pattern too
         let dimension: i32 = self.bit_matrix.get_height();
         let format_info_bits2: i32 = 0;
         let j_min: i32 = dimension - 7;
         {
             let mut j: i32 = dimension - 1;
            while j >= j_min {
                {
                    format_info_bits2 = self.copy_bit(8, j, format_info_bits2);
                }
                j -= 1;
             }
         }

         {
             let mut i: i32 = dimension - 8;
            while i < dimension {
                {
                    format_info_bits2 = self.copy_bit(i, 8, format_info_bits2);
                }
                i += 1;
             }
         }

        self.parsed_format_info = FormatInformation::decode_format_information(format_info_bits1, format_info_bits2);
        if self.parsed_format_info != null {
            return Ok(self.parsed_format_info);
        }
        throw FormatException::get_format_instance();
    }

    /**
   * <p>Reads version information from one of its two locations within the QR Code.</p>
   *
   * @return {@link Version} encapsulating the QR Code's version
   * @throws FormatException if both version information locations cannot be parsed as
   * the valid encoding of version information
   */
    fn  read_version(&self) -> /*  throws FormatException */Result<Version, Rc<Exception>>   {
        if self.parsed_version != null {
            return Ok(self.parsed_version);
        }
         let dimension: i32 = self.bit_matrix.get_height();
         let provisional_version: i32 = (dimension - 17) / 4;
        if provisional_version <= 6 {
            return Ok(Version::get_version_for_number(provisional_version));
        }
        // Read top-right version info: 3 wide by 6 tall
         let version_bits: i32 = 0;
         let ij_min: i32 = dimension - 11;
         {
             let mut j: i32 = 5;
            while j >= 0 {
                {
                     {
                         let mut i: i32 = dimension - 9;
                        while i >= ij_min {
                            {
                                version_bits = self.copy_bit(i, j, version_bits);
                            }
                            i -= 1;
                         }
                     }

                }
                j -= 1;
             }
         }

         let the_parsed_version: Version = Version::decode_version_information(version_bits);
        if the_parsed_version != null && the_parsed_version.get_dimension_for_version() == dimension {
            self.parsed_version = the_parsed_version;
            return Ok(the_parsed_version);
        }
        // Hmm, failed. Try bottom left: 6 wide by 3 tall
        version_bits = 0;
         {
             let mut i: i32 = 5;
            while i >= 0 {
                {
                     {
                         let mut j: i32 = dimension - 9;
                        while j >= ij_min {
                            {
                                version_bits = self.copy_bit(i, j, version_bits);
                            }
                            j -= 1;
                         }
                     }

                }
                i -= 1;
             }
         }

        the_parsed_version = Version::decode_version_information(version_bits);
        if the_parsed_version != null && the_parsed_version.get_dimension_for_version() == dimension {
            self.parsed_version = the_parsed_version;
            return Ok(the_parsed_version);
        }
        throw FormatException::get_format_instance();
    }

    fn  copy_bit(&self,  i: i32,  j: i32,  version_bits: i32) -> i32  {
         let bit: bool =  if self.mirror { self.bit_matrix.get(j, i) } else { self.bit_matrix.get(i, j) };
        return  if bit { (version_bits << 1) | 0x1 } else { version_bits << 1 };
    }

    /**
   * <p>Reads the bits in the {@link BitMatrix} representing the finder pattern in the
   * correct order in order to reconstruct the codewords bytes contained within the
   * QR Code.</p>
   *
   * @return bytes encoded within the QR Code
   * @throws FormatException if the exact number of bytes expected is not read
   */
    fn  read_codewords(&self) -> /*  throws FormatException */Result<Vec<i8>, Rc<Exception>>   {
         let format_info: FormatInformation = self.read_format_information();
         let version: Version = self.read_version();
        // Get the data mask for the format used in this QR Code. This will exclude
        // some bits from reading as we wind through the bit matrix.
         let data_mask: DataMask = DataMask::values()[format_info.get_data_mask()];
         let dimension: i32 = self.bit_matrix.get_height();
        data_mask.unmask_bit_matrix(self.bit_matrix, dimension);
         let function_pattern: BitMatrix = version.build_function_pattern();
         let reading_up: bool = true;
         let mut result: [i8; version.get_total_codewords()] = [0; version.get_total_codewords()];
         let result_offset: i32 = 0;
         let current_byte: i32 = 0;
         let bits_read: i32 = 0;
        // Read columns in pairs, from right to left
         {
             let mut j: i32 = dimension - 1;
            while j > 0 {
                {
                    if j == 6 {
                        // Skip whole column with vertical alignment pattern;
                        // saves time and makes the other code proceed more cleanly
                        j -= 1;
                    }
                    // Read alternatingly from bottom to top then top to bottom
                     {
                         let mut count: i32 = 0;
                        while count < dimension {
                            {
                                 let i: i32 =  if reading_up { dimension - 1 - count } else { count };
                                 {
                                     let mut col: i32 = 0;
                                    while col < 2 {
                                        {
                                            // Ignore bits covered by the function pattern
                                            if !function_pattern.get(j - col, i) {
                                                // Read a bit
                                                bits_read += 1;
                                                current_byte <<= 1;
                                                if self.bit_matrix.get(j - col, i) {
                                                    current_byte |= 1;
                                                }
                                                // If we've made a whole byte, save it off
                                                if bits_read == 8 {
                                                    result[result_offset += 1 !!!check!!! post increment] = current_byte as i8;
                                                    bits_read = 0;
                                                    current_byte = 0;
                                                }
                                            }
                                        }
                                        col += 1;
                                     }
                                 }

                            }
                            count += 1;
                         }
                     }

                    // readingUp = !readingUp; // switch directions
                    reading_up ^= true;
                }
                j -= 2;
             }
         }

        if result_offset != version.get_total_codewords() {
            throw FormatException::get_format_instance();
        }
        return Ok(result);
    }

    /**
   * Revert the mask removal done while reading the code words. The bit matrix should revert to its original state.
   */
    fn  remask(&self)   {
        if self.parsed_format_info == null {
            // We have no format information, and have no data mask
            return;
        }
         let data_mask: DataMask = DataMask::values()[self.parsed_format_info.get_data_mask()];
         let dimension: i32 = self.bit_matrix.get_height();
        data_mask.unmask_bit_matrix(self.bit_matrix, dimension);
    }

    /**
   * Prepare the parser for a mirrored operation.
   * This flag has effect only on the {@link #readFormatInformation()} and the
   * {@link #readVersion()}. Before proceeding with {@link #readCodewords()} the
   * {@link #mirror()} method should be called.
   *
   * @param mirror Whether to read version and format information mirrored.
   */
    fn  set_mirror(&self,  mirror: bool)   {
        self.parsed_version = null;
        self.parsed_format_info = null;
        self.mirror = mirror;
    }

    /** Mirror the bit matrix in order to attempt a second reading. */
    fn  mirror(&self)   {
         {
             let mut x: i32 = 0;
            while x < self.bit_matrix.get_width() {
                {
                     {
                         let mut y: i32 = x + 1;
                        while y < self.bit_matrix.get_height() {
                            {
                                if self.bit_matrix.get(x, y) != self.bit_matrix.get(y, x) {
                                    self.bit_matrix.flip(y, x);
                                    self.bit_matrix.flip(x, y);
                                }
                            }
                            y += 1;
                         }
                     }

                }
                x += 1;
             }
         }

    }
}

// NEW FILE: data_block.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>Encapsulates a block of data within a QR Code. QR Codes may split their data into
 * multiple blocks, each of which is a unit of data and error-correction codewords. Each
 * is represented by an instance of this class.</p>
 *
 * @author Sean Owen
 */
struct DataBlock {

     let num_data_codewords: i32;

     let mut codewords: Vec<i8>;
}

impl DataBlock {

    fn new( num_data_codewords: i32,  codewords: &Vec<i8>) -> DataBlock {
        let .numDataCodewords = num_data_codewords;
        let .codewords = codewords;
    }

    /**
   * <p>When QR Codes use multiple data blocks, they are actually interleaved.
   * That is, the first byte of data block 1 to n is written, then the second bytes, and so on. This
   * method will separate the data into original blocks.</p>
   *
   * @param rawCodewords bytes as read directly from the QR Code
   * @param version version of the QR Code
   * @param ecLevel error-correction level of the QR Code
   * @return DataBlocks containing original bytes, "de-interleaved" from representation in the
   *         QR Code
   */
    fn  get_data_blocks( raw_codewords: &Vec<i8>,  version: &Version,  ec_level: &ErrorCorrectionLevel) -> Vec<DataBlock>  {
        if raw_codewords.len() != version.get_total_codewords() {
            throw IllegalArgumentException::new();
        }
        // Figure out the number and size of data blocks used by this version and
        // error correction level
         let ec_blocks: Version.ECBlocks = version.get_e_c_blocks_for_level(ec_level);
        // First count the total number of data blocks
         let total_blocks: i32 = 0;
         let ec_block_array: Vec<Version.ECB> = ec_blocks.get_e_c_blocks();
        for  let ec_block: Version.ECB in ec_block_array {
            total_blocks += ec_block.get_count();
        }
        // Now establish DataBlocks of the appropriate size and number of data codewords
         let mut result: [Option<DataBlock>; total_blocks] = [None; total_blocks];
         let num_result_blocks: i32 = 0;
        for  let ec_block: Version.ECB in ec_block_array {
             {
                 let mut i: i32 = 0;
                while i < ec_block.get_count() {
                    {
                         let num_data_codewords: i32 = ec_block.get_data_codewords();
                         let num_block_codewords: i32 = ec_blocks.get_e_c_codewords_per_block() + num_data_codewords;
                        result[num_result_blocks += 1 !!!check!!! post increment] = DataBlock::new(num_data_codewords, : [i8; num_block_codewords] = [0; num_block_codewords]);
                    }
                    i += 1;
                 }
             }

        }
        // All blocks have the same amount of data, except that the last n
        // (where n may be 0) have 1 more byte. Figure out where these start.
         let shorter_blocks_total_codewords: i32 = result[0].codewords.len();
         let longer_blocks_start_at: i32 = result.len() - 1;
        while longer_blocks_start_at >= 0 {
             let num_codewords: i32 = result[longer_blocks_start_at].codewords.len();
            if num_codewords == shorter_blocks_total_codewords {
                break;
            }
            longer_blocks_start_at -= 1;
        }
        longer_blocks_start_at += 1;
         let shorter_blocks_num_data_codewords: i32 = shorter_blocks_total_codewords - ec_blocks.get_e_c_codewords_per_block();
        // The last elements of result may be 1 element longer;
        // first fill out as many elements as all of them have
         let raw_codewords_offset: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < shorter_blocks_num_data_codewords {
                {
                     {
                         let mut j: i32 = 0;
                        while j < num_result_blocks {
                            {
                                result[j].codewords[i] = raw_codewords[raw_codewords_offset += 1 !!!check!!! post increment];
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        // Fill out the last data block in the longer ones
         {
             let mut j: i32 = longer_blocks_start_at;
            while j < num_result_blocks {
                {
                    result[j].codewords[shorter_blocks_num_data_codewords] = raw_codewords[raw_codewords_offset += 1 !!!check!!! post increment];
                }
                j += 1;
             }
         }

        // Now add in error correction blocks
         let max: i32 = result[0].codewords.len();
         {
             let mut i: i32 = shorter_blocks_num_data_codewords;
            while i < max {
                {
                     {
                         let mut j: i32 = 0;
                        while j < num_result_blocks {
                            {
                                 let i_offset: i32 =  if j < longer_blocks_start_at { i } else { i + 1 };
                                result[j].codewords[i_offset] = raw_codewords[raw_codewords_offset += 1 !!!check!!! post increment];
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
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

// NEW FILE: data_mask.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>Encapsulates data masks for the data bits in a QR code, per ISO 18004:2006 6.8. Implementations
 * of this class can un-mask a raw BitMatrix. For simplicity, they will unmask the entire BitMatrix,
 * including areas used for finder patterns, timing patterns, etc. These areas should be unused
 * after the point they are unmasked anyway.</p>
 *
 * <p>Note that the diagram in section 6.8.1 is misleading since it indicates that i is column position
 * and j is row position. In fact, as the text says, i is row position and j is column position.</p>
 *
 * @author Sean Owen
 */
enum DataMask {

    /**
   * 000: mask bits for which (x + y) mod 2 == 0
   */
    DATA_MASK_000() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return ((i + j) & 0x01) == 0;
        }
    }
    , /**
   * 001: mask bits for which x mod 2 == 0
   */
    DATA_MASK_001() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return (i & 0x01) == 0;
        }
    }
    , /**
   * 010: mask bits for which y mod 3 == 0
   */
    DATA_MASK_010() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return j % 3 == 0;
        }
    }
    , /**
   * 011: mask bits for which (x + y) mod 3 == 0
   */
    DATA_MASK_011() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return (i + j) % 3 == 0;
        }
    }
    , /**
   * 100: mask bits for which (x/2 + y/3) mod 2 == 0
   */
    DATA_MASK_100() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return (((i / 2) + (j / 3)) & 0x01) == 0;
        }
    }
    , /**
   * 101: mask bits for which xy mod 2 + xy mod 3 == 0
   * equivalently, such that xy mod 6 == 0
   */
    DATA_MASK_101() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return (i * j) % 6 == 0;
        }
    }
    , /**
   * 110: mask bits for which (xy mod 2 + xy mod 3) mod 2 == 0
   * equivalently, such that xy mod 6 < 3
   */
    DATA_MASK_110() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return ((i * j) % 6) < 3;
        }
    }
    , /**
   * 111: mask bits for which ((x+y)mod 2 + xy mod 3) mod 2 == 0
   * equivalently, such that (x + y + xy mod 3) mod 2 == 0
   */
    DATA_MASK_111() {

        fn  is_masked(&self,  i: i32,  j: i32) -> bool  {
            return ((i + j + ((i * j) % 3)) & 0x01) == 0;
        }
    }
    ;

    // End of enum constants.
    /**
   * <p>Implementations of this method reverse the data masking process applied to a QR Code and
   * make its bits ready to read.</p>
   *
   * @param bits representation of QR Code bits
   * @param dimension dimension of QR Code, represented by bits, being unmasked
   */
    fn  unmask_bit_matrix(&self,  bits: &BitMatrix,  dimension: i32)   {
         {
             let mut i: i32 = 0;
            while i < dimension {
                {
                     {
                         let mut j: i32 = 0;
                        while j < dimension {
                            {
                                if self.is_masked(i, j) {
                                    bits.flip(j, i);
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

    fn  is_masked(&self,  i: i32,  j: i32) -> bool ;
}
// NEW FILE: decoded_bit_stream_parser.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>QR Codes can encode text as bits in one of several modes, and can use multiple modes
 * in one QR Code. This class decodes the bits back into text.</p>
 *
 * <p>See ISO 18004:2006, 6.4.3 - 6.4.7</p>
 *
 * @author Sean Owen
 */

/**
   * See ISO 18004:2006, 6.4.4 Table 5
   */
 const ALPHANUMERIC_CHARS: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:".to_char_array();

 const GB2312_SUBSET: i32 = 1;
struct DecodedBitStreamParser {
}

impl DecodedBitStreamParser {

    fn new() -> DecodedBitStreamParser {
    }

    fn  decode( bytes: &Vec<i8>,  version: &Version,  ec_level: &ErrorCorrectionLevel,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
         let bits: BitSource = BitSource::new(&bytes);
         let result: StringBuilder = StringBuilder::new(50);
         let byte_segments: List<Vec<i8>> = ArrayList<>::new(1);
         let symbol_sequence: i32 = -1;
         let parity_data: i32 = -1;
         let symbology_modifier: i32;
        let tryResult1 = 0;
        'try1: loop {
        {
             let current_character_set_e_c_i: CharacterSetECI = null;
             let fc1_in_effect: bool = false;
             let has_f_n_c1first: bool = false;
             let has_f_n_c1second: bool = false;
             let mut mode: Mode;
            loop { {
                // While still another segment to read...
                if bits.available() < 4 {
                    // OK, assume we're done. Really, a TERMINATOR mode should have been recorded here
                    mode = Mode::TERMINATOR;
                } else {
                    // mode is encoded by 4 bits
                    mode = Mode::for_bits(&bits.read_bits(4));
                }
                match mode {
                      TERMINATOR => 
                         {
                            break;
                        }
                      FNC1_FIRST_POSITION => 
                         {
                            // symbology detection
                            has_f_n_c1first = true;
                            // We do little with FNC1 except alter the parsed result a bit according to the spec
                            fc1_in_effect = true;
                            break;
                        }
                      FNC1_SECOND_POSITION => 
                         {
                            // symbology detection
                            has_f_n_c1second = true;
                            // We do little with FNC1 except alter the parsed result a bit according to the spec
                            fc1_in_effect = true;
                            break;
                        }
                      STRUCTURED_APPEND => 
                         {
                            if bits.available() < 16 {
                                throw FormatException::get_format_instance();
                            }
                            // sequence number and parity is added later to the result metadata
                            // Read next 8 bits (symbol sequence #) and 8 bits (parity data), then continue
                            symbol_sequence = bits.read_bits(8);
                            parity_data = bits.read_bits(8);
                            break;
                        }
                      ECI => 
                         {
                            // Count doesn't apply to ECI
                             let value: i32 = ::parse_e_c_i_value(bits);
                            current_character_set_e_c_i = CharacterSetECI::get_character_set_e_c_i_by_value(value);
                            if current_character_set_e_c_i == null {
                                throw FormatException::get_format_instance();
                            }
                            break;
                        }
                      HANZI => 
                         {
                            // First handle Hanzi mode which does not start with character count
                            // Chinese mode contains a sub set indicator right after mode indicator
                             let subset: i32 = bits.read_bits(4);
                             let count_hanzi: i32 = bits.read_bits(&mode.get_character_count_bits(version));
                            if subset == GB2312_SUBSET {
                                ::decode_hanzi_segment(bits, &result, count_hanzi);
                            }
                            break;
                        }
                    _ => 
                         {
                            // "Normal" QR code modes:
                            // How many characters will follow, encoded in this mode?
                             let count: i32 = bits.read_bits(&mode.get_character_count_bits(version));
                            match mode {
                                  NUMERIC => 
                                     {
                                        ::decode_numeric_segment(bits, &result, count);
                                        break;
                                    }
                                  ALPHANUMERIC => 
                                     {
                                        ::decode_alphanumeric_segment(bits, &result, count, fc1_in_effect);
                                        break;
                                    }
                                  BYTE => 
                                     {
                                        ::decode_byte_segment(bits, &result, count, current_character_set_e_c_i, &byte_segments, &hints);
                                        break;
                                    }
                                  KANJI => 
                                     {
                                        ::decode_kanji_segment(bits, &result, count);
                                        break;
                                    }
                                _ => 
                                     {
                                        throw FormatException::get_format_instance();
                                    }
                            }
                            break;
                        }
                }
            }if !(mode != Mode::TERMINATOR) break;}
            if current_character_set_e_c_i != null {
                if has_f_n_c1first {
                    symbology_modifier = 4;
                } else if has_f_n_c1second {
                    symbology_modifier = 6;
                } else {
                    symbology_modifier = 2;
                }
            } else {
                if has_f_n_c1first {
                    symbology_modifier = 3;
                } else if has_f_n_c1second {
                    symbology_modifier = 5;
                } else {
                    symbology_modifier = 1;
                }
            }
        }
        break 'try1
        }
        match tryResult1 {
             catch ( iae: &IllegalArgumentException) {
                throw FormatException::get_format_instance();
            }  0 => break
        }

        return Ok(DecoderResult::new(&bytes, &result.to_string(),  if byte_segments.is_empty() { null } else { byte_segments },  if ec_level == null { null } else { ec_level.to_string() }, symbol_sequence, parity_data, symbology_modifier));
    }

    /**
   * See specification GBT 18284-2000
   */
    fn  decode_hanzi_segment( bits: &BitSource,  result: &StringBuilder,  count: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Don't crash trying to read more bits than we have available.
        if count * 13 > bits.available() {
            throw FormatException::get_format_instance();
        }
        // Each character will require 2 bytes. Read the characters as 2-byte pairs
        // and decode as GB2312 afterwards
         let mut buffer: [i8; 2 * count] = [0; 2 * count];
         let mut offset: i32 = 0;
        while count > 0 {
            // Each 13 bits encodes a 2-byte character
             let two_bytes: i32 = bits.read_bits(13);
             let assembled_two_bytes: i32 = ((two_bytes / 0x060) << 8) | (two_bytes % 0x060);
            if assembled_two_bytes < 0x00A00 {
                // In the 0xA1A1 to 0xAAFE range
                assembled_two_bytes += 0x0A1A1;
            } else {
                // In the 0xB0A1 to 0xFAFE range
                assembled_two_bytes += 0x0A6A1;
            }
            buffer[offset] = ((assembled_two_bytes >> 8) & 0xFF) as i8;
            buffer[offset + 1] = (assembled_two_bytes & 0xFF) as i8;
            offset += 2;
            count -= 1;
        }
        result.append(String::new(&buffer, StringUtils::GB2312_CHARSET));
    }

    fn  decode_kanji_segment( bits: &BitSource,  result: &StringBuilder,  count: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Don't crash trying to read more bits than we have available.
        if count * 13 > bits.available() {
            throw FormatException::get_format_instance();
        }
        // Each character will require 2 bytes. Read the characters as 2-byte pairs
        // and decode as Shift_JIS afterwards
         let mut buffer: [i8; 2 * count] = [0; 2 * count];
         let mut offset: i32 = 0;
        while count > 0 {
            // Each 13 bits encodes a 2-byte character
             let two_bytes: i32 = bits.read_bits(13);
             let assembled_two_bytes: i32 = ((two_bytes / 0x0C0) << 8) | (two_bytes % 0x0C0);
            if assembled_two_bytes < 0x01F00 {
                // In the 0x8140 to 0x9FFC range
                assembled_two_bytes += 0x08140;
            } else {
                // In the 0xE040 to 0xEBBF range
                assembled_two_bytes += 0x0C140;
            }
            buffer[offset] = (assembled_two_bytes >> 8) as i8;
            buffer[offset + 1] = assembled_two_bytes as i8;
            offset += 2;
            count -= 1;
        }
        result.append(String::new(&buffer, StringUtils::SHIFT_JIS_CHARSET));
    }

    fn  decode_byte_segment( bits: &BitSource,  result: &StringBuilder,  count: i32,  current_character_set_e_c_i: &CharacterSetECI,  byte_segments: &Collection<Vec<i8>>,  hints: &Map<DecodeHintType, ?>)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Don't crash trying to read more bits than we have available.
        if 8 * count > bits.available() {
            throw FormatException::get_format_instance();
        }
         let read_bytes: [i8; count] = [0; count];
         {
             let mut i: i32 = 0;
            while i < count {
                {
                    read_bytes[i] = bits.read_bits(8) as i8;
                }
                i += 1;
             }
         }

         let mut encoding: Charset;
        if current_character_set_e_c_i == null {
            // The spec isn't clear on this mode; see
            // section 6.4.5: t does not say which encoding to assuming
            // upon decoding. I have seen ISO-8859-1 used as well as
            // Shift_JIS -- without anything like an ECI designator to
            // give a hint.
            encoding = StringUtils::guess_charset(&read_bytes, &hints);
        } else {
            encoding = current_character_set_e_c_i.get_charset();
        }
        result.append(String::new(&read_bytes, &encoding));
        byte_segments.add(&read_bytes);
    }

    fn  to_alpha_numeric_char( value: i32) -> /*  throws FormatException */Result<char, Rc<Exception>>   {
        if value >= ALPHANUMERIC_CHARS.len() {
            throw FormatException::get_format_instance();
        }
        return Ok(ALPHANUMERIC_CHARS[value]);
    }

    fn  decode_alphanumeric_segment( bits: &BitSource,  result: &StringBuilder,  count: i32,  fc1_in_effect: bool)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Read two characters at a time
         let start: i32 = result.length();
        while count > 1 {
            if bits.available() < 11 {
                throw FormatException::get_format_instance();
            }
             let next_two_chars_bits: i32 = bits.read_bits(11);
            result.append(&::to_alpha_numeric_char(next_two_chars_bits / 45));
            result.append(&::to_alpha_numeric_char(next_two_chars_bits % 45));
            count -= 2;
        }
        if count == 1 {
            // special case: one character left
            if bits.available() < 6 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(&bits.read_bits(6)));
        }
        // See section 6.4.8.1, 6.4.8.2
        if fc1_in_effect {
            // We need to massage the result a bit if in an FNC1 mode:
             {
                 let mut i: i32 = start;
                while i < result.length() {
                    {
                        if result.char_at(i) == '%' {
                            if i < result.length() - 1 && result.char_at(i + 1) == '%' {
                                // %% is rendered as %
                                result.delete_char_at(i + 1);
                            } else {
                                // In alpha mode, % should be converted to FNC1 separator 0x1D
                                result.set_char_at(i, 0x1D as char);
                            }
                        }
                    }
                    i += 1;
                 }
             }

        }
    }

    fn  decode_numeric_segment( bits: &BitSource,  result: &StringBuilder,  count: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        // Read three digits at a time
        while count >= 3 {
            // Each 10 bits encodes three digits
            if bits.available() < 10 {
                throw FormatException::get_format_instance();
            }
             let three_digits_bits: i32 = bits.read_bits(10);
            if three_digits_bits >= 1000 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(three_digits_bits / 100));
            result.append(&::to_alpha_numeric_char((three_digits_bits / 10) % 10));
            result.append(&::to_alpha_numeric_char(three_digits_bits % 10));
            count -= 3;
        }
        if count == 2 {
            // Two digits left over to read, encoded in 7 bits
            if bits.available() < 7 {
                throw FormatException::get_format_instance();
            }
             let two_digits_bits: i32 = bits.read_bits(7);
            if two_digits_bits >= 100 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(two_digits_bits / 10));
            result.append(&::to_alpha_numeric_char(two_digits_bits % 10));
        } else if count == 1 {
            // One digit left over to read
            if bits.available() < 4 {
                throw FormatException::get_format_instance();
            }
             let digit_bits: i32 = bits.read_bits(4);
            if digit_bits >= 10 {
                throw FormatException::get_format_instance();
            }
            result.append(&::to_alpha_numeric_char(digit_bits));
        }
    }

    fn  parse_e_c_i_value( bits: &BitSource) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
         let first_byte: i32 = bits.read_bits(8);
        if (first_byte & 0x80) == 0 {
            // just one byte
            return Ok(first_byte & 0x7F);
        }
        if (first_byte & 0xC0) == 0x80 {
            // two bytes
             let second_byte: i32 = bits.read_bits(8);
            return Ok(((first_byte & 0x3F) << 8) | second_byte);
        }
        if (first_byte & 0xE0) == 0xC0 {
            // three bytes
             let second_third_bytes: i32 = bits.read_bits(16);
            return Ok(((first_byte & 0x1F) << 16) | second_third_bytes);
        }
        throw FormatException::get_format_instance();
    }
}

// NEW FILE: decoder.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>The main class which implements QR Code decoding -- as opposed to locating and extracting
 * the QR Code from an image.</p>
 *
 * @author Sean Owen
 */
pub struct Decoder {

     let rs_decoder: ReedSolomonDecoder;
}

impl Decoder {

    pub fn new() -> Decoder {
        rs_decoder = ReedSolomonDecoder::new(GenericGF::QR_CODE_FIELD_256);
    }

    pub fn  decode(&self,  image: &Vec<Vec<bool>>) -> /*  throws ChecksumException, FormatException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(&image, null));
    }

    /**
   * <p>Convenience method that can decode a QR Code represented as a 2D array of booleans.
   * "true" is taken to mean a black module.</p>
   *
   * @param image booleans representing white/black QR Code modules
   * @param hints decoding hints that should be used to influence decoding
   * @return text and bytes encoded within the QR Code
   * @throws FormatException if the QR Code cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  image: &Vec<Vec<bool>>,  hints: &Map<DecodeHintType, ?>) -> /*  throws ChecksumException, FormatException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(&BitMatrix::parse(&image), &hints));
    }

    pub fn  decode(&self,  bits: &BitMatrix) -> /*  throws ChecksumException, FormatException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(bits, null));
    }

    /**
   * <p>Decodes a QR Code represented as a {@link BitMatrix}. A 1 or "true" is taken to mean a black module.</p>
   *
   * @param bits booleans representing white/black QR Code modules
   * @param hints decoding hints that should be used to influence decoding
   * @return text and bytes encoded within the QR Code
   * @throws FormatException if the QR Code cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  bits: &BitMatrix,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
        // Construct a parser and read version, error-correction level
         let parser: BitMatrixParser = BitMatrixParser::new(bits);
         let mut fe: FormatException = null;
         let mut ce: ChecksumException = null;
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(self.decode(parser, &hints));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &FormatException) {
                fe = e;
            } catch ( e: &ChecksumException) {
                ce = e;
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
            // Revert the bit matrix
            parser.remask();
            // Will be attempting a mirrored reading of the version and format info.
            parser.set_mirror(true);
            // Preemptively read the version.
            parser.read_version();
            // Preemptively read the format information.
            parser.read_format_information();
            /*
       * Since we're here, this means we have successfully detected some kind
       * of version and format information when mirrored. This is a good sign,
       * that the QR code may be mirrored, and we should try once more with a
       * mirrored content.
       */
            // Prepare for a mirrored reading.
            parser.mirror();
             let result: DecoderResult = self.decode(parser, &hints);
            // Success! Notify the caller that the code was mirrored.
            result.set_other(QRCodeDecoderMetaData::new(true));
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &FormatExceptionChecksumException | ) {
                if fe != null {
                    throw fe;
                }
                throw ce;
            }  0 => break
        }

    }

    fn  decode(&self,  parser: &BitMatrixParser,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
         let version: Version = parser.read_version();
         let ec_level: ErrorCorrectionLevel = parser.read_format_information().get_error_correction_level();
        // Read codewords
         let codewords: Vec<i8> = parser.read_codewords();
        // Separate into data blocks
         let data_blocks: Vec<DataBlock> = DataBlock::get_data_blocks(&codewords, version, ec_level);
        // Count total number of data bytes
         let total_bytes: i32 = 0;
        for  let data_block: DataBlock in data_blocks {
            total_bytes += data_block.get_num_data_codewords();
        }
         let result_bytes: [i8; total_bytes] = [0; total_bytes];
         let result_offset: i32 = 0;
        // Error-correct and copy data blocks together into a stream of bytes
        for  let data_block: DataBlock in data_blocks {
             let codeword_bytes: Vec<i8> = data_block.get_codewords();
             let num_data_codewords: i32 = data_block.get_num_data_codewords();
            self.correct_errors(&codeword_bytes, num_data_codewords);
             {
                 let mut i: i32 = 0;
                while i < num_data_codewords {
                    {
                        result_bytes[result_offset += 1 !!!check!!! post increment] = codeword_bytes[i];
                    }
                    i += 1;
                 }
             }

        }
        // Decode the contents of that stream of bytes
        return Ok(DecodedBitStreamParser::decode(&result_bytes, version, ec_level, &hints));
    }

    /**
   * <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
   * correct the errors in-place using Reed-Solomon error correction.</p>
   *
   * @param codewordBytes data and error correction codewords
   * @param numDataCodewords number of codewords that are data bytes
   * @throws ChecksumException if error correction fails
   */
    fn  correct_errors(&self,  codeword_bytes: &Vec<i8>,  num_data_codewords: i32)  -> /*  throws ChecksumException */Result<Void, Rc<Exception>>   {
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

        let tryResult1 = 0;
        'try1: loop {
        {
            self.rs_decoder.decode(&codewords_ints, codeword_bytes.len() - num_data_codewords);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &ReedSolomonException) {
                throw ChecksumException::get_checksum_instance();
            }  0 => break
        }

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

    }
}

// NEW FILE: error_correction_level.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>See ISO 18004:2006, 6.5.1. This enum encapsulates the four error correction levels
 * defined by the QR code standard.</p>
 *
 * @author Sean Owen
 */
pub enum ErrorCorrectionLevel {

    /** L = ~7% correction */
    L(0x01), /** M = ~15% correction */
    M(0x00), /** Q = ~25% correction */
    Q(0x03), /** H = ~30% correction */
    H(0x02);

     const FOR_BITS: vec![Vec<ErrorCorrectionLevel>; 4] = vec![M, L, H, Q, ]
    ;

     let bits: i32;

    fn new( bits: i32) -> ErrorCorrectionLevel {
        let .bits = bits;
    }

    pub fn  get_bits(&self) -> i32  {
        return self.bits;
    }

    /**
   * @param bits int containing the two bits encoding a QR Code's error correction level
   * @return ErrorCorrectionLevel representing the encoded error correction level
   */
    pub fn  for_bits( bits: i32) -> ErrorCorrectionLevel  {
        if bits < 0 || bits >= FOR_BITS.len() {
            throw IllegalArgumentException::new();
        }
        return FOR_BITS[bits];
    }
}
// NEW FILE: format_information.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>Encapsulates a QR Code's format information, including the data mask used and
 * error correction level.</p>
 *
 * @author Sean Owen
 * @see DataMask
 * @see ErrorCorrectionLevel
 */

 const FORMAT_INFO_MASK_QR: i32 = 0x5412;

/**
   * See ISO 18004:2006, Annex C, Table C.1
   */
 const FORMAT_INFO_DECODE_LOOKUP: vec![vec![Vec<Vec<i32>>; 2]; 32] = vec![vec![0x5412, 0x00, ]
, vec![0x5125, 0x01, ]
, vec![0x5E7C, 0x02, ]
, vec![0x5B4B, 0x03, ]
, vec![0x45F9, 0x04, ]
, vec![0x40CE, 0x05, ]
, vec![0x4F97, 0x06, ]
, vec![0x4AA0, 0x07, ]
, vec![0x77C4, 0x08, ]
, vec![0x72F3, 0x09, ]
, vec![0x7DAA, 0x0A, ]
, vec![0x789D, 0x0B, ]
, vec![0x662F, 0x0C, ]
, vec![0x6318, 0x0D, ]
, vec![0x6C41, 0x0E, ]
, vec![0x6976, 0x0F, ]
, vec![0x1689, 0x10, ]
, vec![0x13BE, 0x11, ]
, vec![0x1CE7, 0x12, ]
, vec![0x19D0, 0x13, ]
, vec![0x0762, 0x14, ]
, vec![0x0255, 0x15, ]
, vec![0x0D0C, 0x16, ]
, vec![0x083B, 0x17, ]
, vec![0x355F, 0x18, ]
, vec![0x3068, 0x19, ]
, vec![0x3F31, 0x1A, ]
, vec![0x3A06, 0x1B, ]
, vec![0x24B4, 0x1C, ]
, vec![0x2183, 0x1D, ]
, vec![0x2EDA, 0x1E, ]
, vec![0x2BED, 0x1F, ]
, ]
;
struct FormatInformation {

     let error_correction_level: ErrorCorrectionLevel;

     let data_mask: i8;
}

impl FormatInformation {

    fn new( format_info: i32) -> FormatInformation {
        // Bits 3,4
        error_correction_level = ErrorCorrectionLevel::for_bits((format_info >> 3) & 0x03);
        // Bottom 3 bits
        data_mask = (format_info & 0x07) as i8;
    }

    fn  num_bits_differing( a: i32,  b: i32) -> i32  {
        return Integer::bit_count(a ^ b);
    }

    /**
   * @param maskedFormatInfo1 format info indicator, with mask still applied
   * @param maskedFormatInfo2 second copy of same info; both are checked at the same time
   *  to establish best match
   * @return information about the format it specifies, or {@code null}
   *  if doesn't seem to match any known pattern
   */
    fn  decode_format_information( masked_format_info1: i32,  masked_format_info2: i32) -> FormatInformation  {
         let format_info: FormatInformation = ::do_decode_format_information(masked_format_info1, masked_format_info2);
        if format_info != null {
            return format_info;
        }
        // first
        return ::do_decode_format_information(masked_format_info1 ^ FORMAT_INFO_MASK_QR, masked_format_info2 ^ FORMAT_INFO_MASK_QR);
    }

    fn  do_decode_format_information( masked_format_info1: i32,  masked_format_info2: i32) -> FormatInformation  {
        // Find the int in FORMAT_INFO_DECODE_LOOKUP with fewest bits differing
         let best_difference: i32 = Integer::MAX_VALUE;
         let best_format_info: i32 = 0;
        for  let decode_info: Vec<i32> in FORMAT_INFO_DECODE_LOOKUP {
             let target_info: i32 = decode_info[0];
            if target_info == masked_format_info1 || target_info == masked_format_info2 {
                // Found an exact match
                return FormatInformation::new(decode_info[1]);
            }
             let bits_difference: i32 = ::num_bits_differing(masked_format_info1, target_info);
            if bits_difference < best_difference {
                best_format_info = decode_info[1];
                best_difference = bits_difference;
            }
            if masked_format_info1 != masked_format_info2 {
                // also try the other option
                bits_difference = ::num_bits_differing(masked_format_info2, target_info);
                if bits_difference < best_difference {
                    best_format_info = decode_info[1];
                    best_difference = bits_difference;
                }
            }
        }
        // differing means we found a match
        if best_difference <= 3 {
            return FormatInformation::new(best_format_info);
        }
        return null;
    }

    fn  get_error_correction_level(&self) -> ErrorCorrectionLevel  {
        return self.error_correction_level;
    }

    fn  get_data_mask(&self) -> i8  {
        return self.data_mask;
    }

    pub fn  hash_code(&self) -> i32  {
        return (self.error_correction_level.ordinal() << 3) | self.data_mask;
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof FormatInformation) {
            return false;
        }
         let other: FormatInformation = o as FormatInformation;
        return self.errorCorrectionLevel == other.errorCorrectionLevel && self.dataMask == other.dataMask;
    }
}

// NEW FILE: mode.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>See ISO 18004:2006, 6.4.1, Tables 2 and 3. This enum encapsulates the various modes in which
 * data can be encoded to bits in the QR code standard.</p>
 *
 * @author Sean Owen
 */
pub enum Mode {

    // Not really a mode...
    TERMINATOR( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x00), NUMERIC( : vec![i32; 3] = vec![10, 12, 14, ]
    , 0x01), ALPHANUMERIC( : vec![i32; 3] = vec![9, 11, 13, ]
    , 0x02), // Not supported
    STRUCTURED_APPEND( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x03), BYTE( : vec![i32; 3] = vec![8, 16, 16, ]
    , 0x04), // character counts don't apply
    ECI( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x07), KANJI( : vec![i32; 3] = vec![8, 10, 12, ]
    , 0x08), FNC1_FIRST_POSITION( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x05), FNC1_SECOND_POSITION( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x09), /** See GBT 18284-2000; "Hanzi" is a transliteration of this mode name. */
    HANZI( : vec![i32; 3] = vec![8, 10, 12, ]
    , 0x0D);

     let character_count_bits_for_versions: Vec<i32>;

     let bits: i32;

    fn new( character_count_bits_for_versions: &Vec<i32>,  bits: i32) -> Mode {
        let .characterCountBitsForVersions = character_count_bits_for_versions;
        let .bits = bits;
    }

    /**
   * @param bits four bits encoding a QR Code data mode
   * @return Mode encoded by these bits
   * @throws IllegalArgumentException if bits do not correspond to a known mode
   */
    pub fn  for_bits( bits: i32) -> Mode  {
        match bits {
              0x0 => 
                 {
                    return TERMINATOR;
                }
              0x1 => 
                 {
                    return NUMERIC;
                }
              0x2 => 
                 {
                    return ALPHANUMERIC;
                }
              0x3 => 
                 {
                    return STRUCTURED_APPEND;
                }
              0x4 => 
                 {
                    return BYTE;
                }
              0x5 => 
                 {
                    return FNC1_FIRST_POSITION;
                }
              0x7 => 
                 {
                    return ECI;
                }
              0x8 => 
                 {
                    return KANJI;
                }
              0x9 => 
                 {
                    return FNC1_SECOND_POSITION;
                }
              0xD => 
                 {
                    // 0xD is defined in GBT 18284-2000, may not be supported in foreign country
                    return HANZI;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new();
                }
        }
    }

    /**
   * @param version version in question
   * @return number of bits used, in this QR Code symbol {@link Version}, to encode the
   *         count of characters that will follow encoded in this Mode
   */
    pub fn  get_character_count_bits(&self,  version: &Version) -> i32  {
         let number: i32 = version.get_version_number();
         let mut offset: i32;
        if number <= 9 {
            offset = 0;
        } else if number <= 26 {
            offset = 1;
        } else {
            offset = 2;
        }
        return self.character_count_bits_for_versions[offset];
    }

    pub fn  get_bits(&self) -> i32  {
        return self.bits;
    }
}
// NEW FILE: q_r_code_decoder_meta_data.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * Meta-data container for QR Code decoding. Instances of this class may be used to convey information back to the
 * decoding caller. Callers are expected to process this.
 *
 * @see com.google.zxing.common.DecoderResult#getOther()
 */
pub struct QRCodeDecoderMetaData {

     let mirrored: bool;
}

impl QRCodeDecoderMetaData {

    fn new( mirrored: bool) -> QRCodeDecoderMetaData {
        let .mirrored = mirrored;
    }

    /**
   * @return true if the QR Code was mirrored.
   */
    pub fn  is_mirrored(&self) -> bool  {
        return self.mirrored;
    }

    /**
   * Apply the result points' order correction due to mirroring.
   *
   * @param points Array of points to apply mirror correction to.
   */
    pub fn  apply_mirrored_correction(&self,  points: &Vec<ResultPoint>)   {
        if !self.mirrored || points == null || points.len() < 3 {
            return;
        }
         let bottom_left: ResultPoint = points[0];
        points[0] = points[2];
        points[2] = bottom_left;
    // No need to 'fix' top-left and alignment pattern.
    }
}

// NEW FILE: version.rs
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
// package com::google::zxing::qrcode::decoder;

/**
 * See ISO 18004:2006 Annex D
 *
 * @author Sean Owen
 */

/**
   * See ISO 18004:2006 Annex D.
   * Element i represents the raw version bits that specify version i + 7
   */
 const VERSION_DECODE_INFO: vec![Vec<i32>; 34] = vec![0x07C94, 0x085BC, 0x09A99, 0x0A4D3, 0x0BBF6, 0x0C762, 0x0D847, 0x0E60D, 0x0F928, 0x10B78, 0x1145D, 0x12A17, 0x13532, 0x149A6, 0x15683, 0x168C9, 0x177EC, 0x18EC4, 0x191E1, 0x1AFAB, 0x1B08E, 0x1CC1A, 0x1D33F, 0x1ED75, 0x1F250, 0x209D5, 0x216F0, 0x228BA, 0x2379F, 0x24B0B, 0x2542E, 0x26A64, 0x27541, 0x28C69, ]
;

 const VERSIONS: Vec<Version> = ::build_versions();
pub struct Version {

     let version_number: i32;

     let alignment_pattern_centers: Vec<i32>;

     let ec_blocks: Vec<ECBlocks>;

     let total_codewords: i32;
}

impl Version {

    fn new( version_number: i32,  alignment_pattern_centers: &Vec<i32>,  ec_blocks: &ECBlocks) -> Version {
        let .versionNumber = version_number;
        let .alignmentPatternCenters = alignment_pattern_centers;
        let .ecBlocks = ec_blocks;
         let mut total: i32 = 0;
         let ec_codewords: i32 = ec_blocks[0].get_e_c_codewords_per_block();
         let ecb_array: Vec<ECB> = ec_blocks[0].get_e_c_blocks();
        for  let ec_block: ECB in ecb_array {
            total += ec_block.get_count() * (ec_block.get_data_codewords() + ec_codewords);
        }
        let .totalCodewords = total;
    }

    pub fn  get_version_number(&self) -> i32  {
        return self.version_number;
    }

    pub fn  get_alignment_pattern_centers(&self) -> Vec<i32>  {
        return self.alignment_pattern_centers;
    }

    pub fn  get_total_codewords(&self) -> i32  {
        return self.total_codewords;
    }

    pub fn  get_dimension_for_version(&self) -> i32  {
        return 17 + 4 * self.version_number;
    }

    pub fn  get_e_c_blocks_for_level(&self,  ec_level: &ErrorCorrectionLevel) -> ECBlocks  {
        return self.ec_blocks[ec_level.ordinal()];
    }

    /**
   * <p>Deduces version information purely from QR Code dimensions.</p>
   *
   * @param dimension dimension in modules
   * @return Version for a QR Code of that dimension
   * @throws FormatException if dimension is not 1 mod 4
   */
    pub fn  get_provisional_version_for_dimension( dimension: i32) -> /*  throws FormatException */Result<Version, Rc<Exception>>   {
        if dimension % 4 != 1 {
            throw FormatException::get_format_instance();
        }
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(::get_version_for_number((dimension - 17) / 4));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &IllegalArgumentException) {
                throw FormatException::get_format_instance();
            }  0 => break
        }

    }

    pub fn  get_version_for_number( version_number: i32) -> Version  {
        if version_number < 1 || version_number > 40 {
            throw IllegalArgumentException::new();
        }
        return VERSIONS[version_number - 1];
    }

    fn  decode_version_information( version_bits: i32) -> Version  {
         let best_difference: i32 = Integer::MAX_VALUE;
         let best_version: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < VERSION_DECODE_INFO.len() {
                {
                     let target_version: i32 = VERSION_DECODE_INFO[i];
                    // Do the version info bits match exactly? done.
                    if target_version == version_bits {
                        return ::get_version_for_number(i + 7);
                    }
                    // Otherwise see if this is the closest to a real version info bit string
                    // we have seen so far
                     let bits_difference: i32 = FormatInformation::num_bits_differing(version_bits, target_version);
                    if bits_difference < best_difference {
                        best_version = i + 7;
                        best_difference = bits_difference;
                    }
                }
                i += 1;
             }
         }

        // differ in less than 8 bits.
        if best_difference <= 3 {
            return ::get_version_for_number(best_version);
        }
        // If we didn't find a close enough match, fail
        return null;
    }

    /**
   * See ISO 18004:2006 Annex E
   */
    fn  build_function_pattern(&self) -> BitMatrix  {
         let dimension: i32 = self.get_dimension_for_version();
         let bit_matrix: BitMatrix = BitMatrix::new(dimension);
        // Top left finder pattern + separator + format
        bit_matrix.set_region(0, 0, 9, 9);
        // Top right finder pattern + separator + format
        bit_matrix.set_region(dimension - 8, 0, 8, 9);
        // Bottom left finder pattern + separator + format
        bit_matrix.set_region(0, dimension - 8, 9, 8);
        // Alignment patterns
         let max: i32 = self.alignment_pattern_centers.len();
         {
             let mut x: i32 = 0;
            while x < max {
                {
                     let i: i32 = self.alignment_pattern_centers[x] - 2;
                     {
                         let mut y: i32 = 0;
                        while y < max {
                            {
                                if (x != 0 || (y != 0 && y != max - 1)) && (x != max - 1 || y != 0) {
                                    bit_matrix.set_region(self.alignment_pattern_centers[y] - 2, i, 5, 5);
                                }
                            // else no o alignment patterns near the three finder patterns
                            }
                            y += 1;
                         }
                     }

                }
                x += 1;
             }
         }

        // Vertical timing pattern
        bit_matrix.set_region(6, 9, 1, dimension - 17);
        // Horizontal timing pattern
        bit_matrix.set_region(9, 6, dimension - 17, 1);
        if self.version_number > 6 {
            // Version info, top right
            bit_matrix.set_region(dimension - 11, 0, 3, 6);
            // Version info, bottom left
            bit_matrix.set_region(0, dimension - 11, 6, 3);
        }
        return bit_matrix;
    }

    /**
   * <p>Encapsulates a set of error-correction blocks in one symbol version. Most versions will
   * use blocks of differing sizes within one version, so, this encapsulates the parameters for
   * each set of blocks. It also holds the number of error-correction codewords per block since it
   * will be the same across all blocks within one version.</p>
   */
    pub struct ECBlocks {

         let ec_codewords_per_block: i32;

         let ec_blocks: Vec<ECB>;
    }
    
    impl ECBlocks {

        fn new( ec_codewords_per_block: i32,  ec_blocks: &ECB) -> ECBlocks {
            let .ecCodewordsPerBlock = ec_codewords_per_block;
            let .ecBlocks = ec_blocks;
        }

        pub fn  get_e_c_codewords_per_block(&self) -> i32  {
            return self.ec_codewords_per_block;
        }

        pub fn  get_num_blocks(&self) -> i32  {
             let mut total: i32 = 0;
            for  let ec_block: ECB in self.ec_blocks {
                total += ec_block.get_count();
            }
            return total;
        }

        pub fn  get_total_e_c_codewords(&self) -> i32  {
            return self.ec_codewords_per_block * self.get_num_blocks();
        }

        pub fn  get_e_c_blocks(&self) -> Vec<ECB>  {
            return self.ec_blocks;
        }
    }


    /**
   * <p>Encapsulates the parameters for one error-correction block in one symbol version.
   * This includes the number of data codewords, and the number of times a block with these
   * parameters is used consecutively in the QR code version's format.</p>
   */
    pub struct ECB {

         let count: i32;

         let data_codewords: i32;
    }
    
    impl ECB {

        fn new( count: i32,  data_codewords: i32) -> ECB {
            let .count = count;
            let .dataCodewords = data_codewords;
        }

        pub fn  get_count(&self) -> i32  {
            return self.count;
        }

        pub fn  get_data_codewords(&self) -> i32  {
            return self.data_codewords;
        }
    }


    pub fn  to_string(&self) -> String  {
        return String::value_of(self.version_number);
    }

    /**
   * See ISO 18004:2006 6.5.1 Table 9
   */
    fn  build_versions() -> Vec<Version>  {
        return  : vec![Version; 40] = vec![Version::new(1,  , ECBlocks::new(7, ECB::new(1, 19)), ECBlocks::new(10, ECB::new(1, 16)), ECBlocks::new(13, ECB::new(1, 13)), ECBlocks::new(17, ECB::new(1, 9))), Version::new(2,  : vec![i32; 2] = vec![6, 18, ]
        , ECBlocks::new(10, ECB::new(1, 34)), ECBlocks::new(16, ECB::new(1, 28)), ECBlocks::new(22, ECB::new(1, 22)), ECBlocks::new(28, ECB::new(1, 16))), Version::new(3,  : vec![i32; 2] = vec![6, 22, ]
        , ECBlocks::new(15, ECB::new(1, 55)), ECBlocks::new(26, ECB::new(1, 44)), ECBlocks::new(18, ECB::new(2, 17)), ECBlocks::new(22, ECB::new(2, 13))), Version::new(4,  : vec![i32; 2] = vec![6, 26, ]
        , ECBlocks::new(20, ECB::new(1, 80)), ECBlocks::new(18, ECB::new(2, 32)), ECBlocks::new(26, ECB::new(2, 24)), ECBlocks::new(16, ECB::new(4, 9))), Version::new(5,  : vec![i32; 2] = vec![6, 30, ]
        , ECBlocks::new(26, ECB::new(1, 108)), ECBlocks::new(24, ECB::new(2, 43)), ECBlocks::new(18, ECB::new(2, 15), ECB::new(2, 16)), ECBlocks::new(22, ECB::new(2, 11), ECB::new(2, 12))), Version::new(6,  : vec![i32; 2] = vec![6, 34, ]
        , ECBlocks::new(18, ECB::new(2, 68)), ECBlocks::new(16, ECB::new(4, 27)), ECBlocks::new(24, ECB::new(4, 19)), ECBlocks::new(28, ECB::new(4, 15))), Version::new(7,  : vec![i32; 3] = vec![6, 22, 38, ]
        , ECBlocks::new(20, ECB::new(2, 78)), ECBlocks::new(18, ECB::new(4, 31)), ECBlocks::new(18, ECB::new(2, 14), ECB::new(4, 15)), ECBlocks::new(26, ECB::new(4, 13), ECB::new(1, 14))), Version::new(8,  : vec![i32; 3] = vec![6, 24, 42, ]
        , ECBlocks::new(24, ECB::new(2, 97)), ECBlocks::new(22, ECB::new(2, 38), ECB::new(2, 39)), ECBlocks::new(22, ECB::new(4, 18), ECB::new(2, 19)), ECBlocks::new(26, ECB::new(4, 14), ECB::new(2, 15))), Version::new(9,  : vec![i32; 3] = vec![6, 26, 46, ]
        , ECBlocks::new(30, ECB::new(2, 116)), ECBlocks::new(22, ECB::new(3, 36), ECB::new(2, 37)), ECBlocks::new(20, ECB::new(4, 16), ECB::new(4, 17)), ECBlocks::new(24, ECB::new(4, 12), ECB::new(4, 13))), Version::new(10,  : vec![i32; 3] = vec![6, 28, 50, ]
        , ECBlocks::new(18, ECB::new(2, 68), ECB::new(2, 69)), ECBlocks::new(26, ECB::new(4, 43), ECB::new(1, 44)), ECBlocks::new(24, ECB::new(6, 19), ECB::new(2, 20)), ECBlocks::new(28, ECB::new(6, 15), ECB::new(2, 16))), Version::new(11,  : vec![i32; 3] = vec![6, 30, 54, ]
        , ECBlocks::new(20, ECB::new(4, 81)), ECBlocks::new(30, ECB::new(1, 50), ECB::new(4, 51)), ECBlocks::new(28, ECB::new(4, 22), ECB::new(4, 23)), ECBlocks::new(24, ECB::new(3, 12), ECB::new(8, 13))), Version::new(12,  : vec![i32; 3] = vec![6, 32, 58, ]
        , ECBlocks::new(24, ECB::new(2, 92), ECB::new(2, 93)), ECBlocks::new(22, ECB::new(6, 36), ECB::new(2, 37)), ECBlocks::new(26, ECB::new(4, 20), ECB::new(6, 21)), ECBlocks::new(28, ECB::new(7, 14), ECB::new(4, 15))), Version::new(13,  : vec![i32; 3] = vec![6, 34, 62, ]
        , ECBlocks::new(26, ECB::new(4, 107)), ECBlocks::new(22, ECB::new(8, 37), ECB::new(1, 38)), ECBlocks::new(24, ECB::new(8, 20), ECB::new(4, 21)), ECBlocks::new(22, ECB::new(12, 11), ECB::new(4, 12))), Version::new(14,  : vec![i32; 4] = vec![6, 26, 46, 66, ]
        , ECBlocks::new(30, ECB::new(3, 115), ECB::new(1, 116)), ECBlocks::new(24, ECB::new(4, 40), ECB::new(5, 41)), ECBlocks::new(20, ECB::new(11, 16), ECB::new(5, 17)), ECBlocks::new(24, ECB::new(11, 12), ECB::new(5, 13))), Version::new(15,  : vec![i32; 4] = vec![6, 26, 48, 70, ]
        , ECBlocks::new(22, ECB::new(5, 87), ECB::new(1, 88)), ECBlocks::new(24, ECB::new(5, 41), ECB::new(5, 42)), ECBlocks::new(30, ECB::new(5, 24), ECB::new(7, 25)), ECBlocks::new(24, ECB::new(11, 12), ECB::new(7, 13))), Version::new(16,  : vec![i32; 4] = vec![6, 26, 50, 74, ]
        , ECBlocks::new(24, ECB::new(5, 98), ECB::new(1, 99)), ECBlocks::new(28, ECB::new(7, 45), ECB::new(3, 46)), ECBlocks::new(24, ECB::new(15, 19), ECB::new(2, 20)), ECBlocks::new(30, ECB::new(3, 15), ECB::new(13, 16))), Version::new(17,  : vec![i32; 4] = vec![6, 30, 54, 78, ]
        , ECBlocks::new(28, ECB::new(1, 107), ECB::new(5, 108)), ECBlocks::new(28, ECB::new(10, 46), ECB::new(1, 47)), ECBlocks::new(28, ECB::new(1, 22), ECB::new(15, 23)), ECBlocks::new(28, ECB::new(2, 14), ECB::new(17, 15))), Version::new(18,  : vec![i32; 4] = vec![6, 30, 56, 82, ]
        , ECBlocks::new(30, ECB::new(5, 120), ECB::new(1, 121)), ECBlocks::new(26, ECB::new(9, 43), ECB::new(4, 44)), ECBlocks::new(28, ECB::new(17, 22), ECB::new(1, 23)), ECBlocks::new(28, ECB::new(2, 14), ECB::new(19, 15))), Version::new(19,  : vec![i32; 4] = vec![6, 30, 58, 86, ]
        , ECBlocks::new(28, ECB::new(3, 113), ECB::new(4, 114)), ECBlocks::new(26, ECB::new(3, 44), ECB::new(11, 45)), ECBlocks::new(26, ECB::new(17, 21), ECB::new(4, 22)), ECBlocks::new(26, ECB::new(9, 13), ECB::new(16, 14))), Version::new(20,  : vec![i32; 4] = vec![6, 34, 62, 90, ]
        , ECBlocks::new(28, ECB::new(3, 107), ECB::new(5, 108)), ECBlocks::new(26, ECB::new(3, 41), ECB::new(13, 42)), ECBlocks::new(30, ECB::new(15, 24), ECB::new(5, 25)), ECBlocks::new(28, ECB::new(15, 15), ECB::new(10, 16))), Version::new(21,  : vec![i32; 5] = vec![6, 28, 50, 72, 94, ]
        , ECBlocks::new(28, ECB::new(4, 116), ECB::new(4, 117)), ECBlocks::new(26, ECB::new(17, 42)), ECBlocks::new(28, ECB::new(17, 22), ECB::new(6, 23)), ECBlocks::new(30, ECB::new(19, 16), ECB::new(6, 17))), Version::new(22,  : vec![i32; 5] = vec![6, 26, 50, 74, 98, ]
        , ECBlocks::new(28, ECB::new(2, 111), ECB::new(7, 112)), ECBlocks::new(28, ECB::new(17, 46)), ECBlocks::new(30, ECB::new(7, 24), ECB::new(16, 25)), ECBlocks::new(24, ECB::new(34, 13))), Version::new(23,  : vec![i32; 5] = vec![6, 30, 54, 78, 102, ]
        , ECBlocks::new(30, ECB::new(4, 121), ECB::new(5, 122)), ECBlocks::new(28, ECB::new(4, 47), ECB::new(14, 48)), ECBlocks::new(30, ECB::new(11, 24), ECB::new(14, 25)), ECBlocks::new(30, ECB::new(16, 15), ECB::new(14, 16))), Version::new(24,  : vec![i32; 5] = vec![6, 28, 54, 80, 106, ]
        , ECBlocks::new(30, ECB::new(6, 117), ECB::new(4, 118)), ECBlocks::new(28, ECB::new(6, 45), ECB::new(14, 46)), ECBlocks::new(30, ECB::new(11, 24), ECB::new(16, 25)), ECBlocks::new(30, ECB::new(30, 16), ECB::new(2, 17))), Version::new(25,  : vec![i32; 5] = vec![6, 32, 58, 84, 110, ]
        , ECBlocks::new(26, ECB::new(8, 106), ECB::new(4, 107)), ECBlocks::new(28, ECB::new(8, 47), ECB::new(13, 48)), ECBlocks::new(30, ECB::new(7, 24), ECB::new(22, 25)), ECBlocks::new(30, ECB::new(22, 15), ECB::new(13, 16))), Version::new(26,  : vec![i32; 5] = vec![6, 30, 58, 86, 114, ]
        , ECBlocks::new(28, ECB::new(10, 114), ECB::new(2, 115)), ECBlocks::new(28, ECB::new(19, 46), ECB::new(4, 47)), ECBlocks::new(28, ECB::new(28, 22), ECB::new(6, 23)), ECBlocks::new(30, ECB::new(33, 16), ECB::new(4, 17))), Version::new(27,  : vec![i32; 5] = vec![6, 34, 62, 90, 118, ]
        , ECBlocks::new(30, ECB::new(8, 122), ECB::new(4, 123)), ECBlocks::new(28, ECB::new(22, 45), ECB::new(3, 46)), ECBlocks::new(30, ECB::new(8, 23), ECB::new(26, 24)), ECBlocks::new(30, ECB::new(12, 15), ECB::new(28, 16))), Version::new(28,  : vec![i32; 6] = vec![6, 26, 50, 74, 98, 122, ]
        , ECBlocks::new(30, ECB::new(3, 117), ECB::new(10, 118)), ECBlocks::new(28, ECB::new(3, 45), ECB::new(23, 46)), ECBlocks::new(30, ECB::new(4, 24), ECB::new(31, 25)), ECBlocks::new(30, ECB::new(11, 15), ECB::new(31, 16))), Version::new(29,  : vec![i32; 6] = vec![6, 30, 54, 78, 102, 126, ]
        , ECBlocks::new(30, ECB::new(7, 116), ECB::new(7, 117)), ECBlocks::new(28, ECB::new(21, 45), ECB::new(7, 46)), ECBlocks::new(30, ECB::new(1, 23), ECB::new(37, 24)), ECBlocks::new(30, ECB::new(19, 15), ECB::new(26, 16))), Version::new(30,  : vec![i32; 6] = vec![6, 26, 52, 78, 104, 130, ]
        , ECBlocks::new(30, ECB::new(5, 115), ECB::new(10, 116)), ECBlocks::new(28, ECB::new(19, 47), ECB::new(10, 48)), ECBlocks::new(30, ECB::new(15, 24), ECB::new(25, 25)), ECBlocks::new(30, ECB::new(23, 15), ECB::new(25, 16))), Version::new(31,  : vec![i32; 6] = vec![6, 30, 56, 82, 108, 134, ]
        , ECBlocks::new(30, ECB::new(13, 115), ECB::new(3, 116)), ECBlocks::new(28, ECB::new(2, 46), ECB::new(29, 47)), ECBlocks::new(30, ECB::new(42, 24), ECB::new(1, 25)), ECBlocks::new(30, ECB::new(23, 15), ECB::new(28, 16))), Version::new(32,  : vec![i32; 6] = vec![6, 34, 60, 86, 112, 138, ]
        , ECBlocks::new(30, ECB::new(17, 115)), ECBlocks::new(28, ECB::new(10, 46), ECB::new(23, 47)), ECBlocks::new(30, ECB::new(10, 24), ECB::new(35, 25)), ECBlocks::new(30, ECB::new(19, 15), ECB::new(35, 16))), Version::new(33,  : vec![i32; 6] = vec![6, 30, 58, 86, 114, 142, ]
        , ECBlocks::new(30, ECB::new(17, 115), ECB::new(1, 116)), ECBlocks::new(28, ECB::new(14, 46), ECB::new(21, 47)), ECBlocks::new(30, ECB::new(29, 24), ECB::new(19, 25)), ECBlocks::new(30, ECB::new(11, 15), ECB::new(46, 16))), Version::new(34,  : vec![i32; 6] = vec![6, 34, 62, 90, 118, 146, ]
        , ECBlocks::new(30, ECB::new(13, 115), ECB::new(6, 116)), ECBlocks::new(28, ECB::new(14, 46), ECB::new(23, 47)), ECBlocks::new(30, ECB::new(44, 24), ECB::new(7, 25)), ECBlocks::new(30, ECB::new(59, 16), ECB::new(1, 17))), Version::new(35,  : vec![i32; 7] = vec![6, 30, 54, 78, 102, 126, 150, ]
        , ECBlocks::new(30, ECB::new(12, 121), ECB::new(7, 122)), ECBlocks::new(28, ECB::new(12, 47), ECB::new(26, 48)), ECBlocks::new(30, ECB::new(39, 24), ECB::new(14, 25)), ECBlocks::new(30, ECB::new(22, 15), ECB::new(41, 16))), Version::new(36,  : vec![i32; 7] = vec![6, 24, 50, 76, 102, 128, 154, ]
        , ECBlocks::new(30, ECB::new(6, 121), ECB::new(14, 122)), ECBlocks::new(28, ECB::new(6, 47), ECB::new(34, 48)), ECBlocks::new(30, ECB::new(46, 24), ECB::new(10, 25)), ECBlocks::new(30, ECB::new(2, 15), ECB::new(64, 16))), Version::new(37,  : vec![i32; 7] = vec![6, 28, 54, 80, 106, 132, 158, ]
        , ECBlocks::new(30, ECB::new(17, 122), ECB::new(4, 123)), ECBlocks::new(28, ECB::new(29, 46), ECB::new(14, 47)), ECBlocks::new(30, ECB::new(49, 24), ECB::new(10, 25)), ECBlocks::new(30, ECB::new(24, 15), ECB::new(46, 16))), Version::new(38,  : vec![i32; 7] = vec![6, 32, 58, 84, 110, 136, 162, ]
        , ECBlocks::new(30, ECB::new(4, 122), ECB::new(18, 123)), ECBlocks::new(28, ECB::new(13, 46), ECB::new(32, 47)), ECBlocks::new(30, ECB::new(48, 24), ECB::new(14, 25)), ECBlocks::new(30, ECB::new(42, 15), ECB::new(32, 16))), Version::new(39,  : vec![i32; 7] = vec![6, 26, 54, 82, 110, 138, 166, ]
        , ECBlocks::new(30, ECB::new(20, 117), ECB::new(4, 118)), ECBlocks::new(28, ECB::new(40, 47), ECB::new(7, 48)), ECBlocks::new(30, ECB::new(43, 24), ECB::new(22, 25)), ECBlocks::new(30, ECB::new(10, 15), ECB::new(67, 16))), Version::new(40,  : vec![i32; 7] = vec![6, 30, 58, 86, 114, 142, 170, ]
        , ECBlocks::new(30, ECB::new(19, 118), ECB::new(6, 119)), ECBlocks::new(28, ECB::new(18, 47), ECB::new(31, 48)), ECBlocks::new(30, ECB::new(34, 24), ECB::new(34, 25)), ECBlocks::new(30, ECB::new(20, 15), ECB::new(61, 16))), ]
        ;
    }
}

