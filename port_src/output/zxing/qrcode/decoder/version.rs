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

