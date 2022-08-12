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

