/*
 * Copyright 2008 ZXing authors
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
// package com::google::zxing::qrcode::encoder;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

 const POSITION_DETECTION_PATTERN: vec![vec![Vec<Vec<i32>>; 7]; 7] = vec![vec![1, 1, 1, 1, 1, 1, 1, ]
, vec![1, 0, 0, 0, 0, 0, 1, ]
, vec![1, 0, 1, 1, 1, 0, 1, ]
, vec![1, 0, 1, 1, 1, 0, 1, ]
, vec![1, 0, 1, 1, 1, 0, 1, ]
, vec![1, 0, 0, 0, 0, 0, 1, ]
, vec![1, 1, 1, 1, 1, 1, 1, ]
, ]
;

 const POSITION_ADJUSTMENT_PATTERN: vec![vec![Vec<Vec<i32>>; 5]; 5] = vec![vec![1, 1, 1, 1, 1, ]
, vec![1, 0, 0, 0, 1, ]
, vec![1, 0, 1, 0, 1, ]
, vec![1, 0, 0, 0, 1, ]
, vec![1, 1, 1, 1, 1, ]
, ]
;

// From Appendix E. Table 1, JIS0510X:2004 (p 71). The table was double-checked by komatsu.
 const POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE: vec![vec![Vec<Vec<i32>>; 7]; 40] = vec![// Version 1
vec![-1, -1, -1, -1, -1, -1, -1, ]
, // Version 2
vec![6, 18, -1, -1, -1, -1, -1, ]
, // Version 3
vec![6, 22, -1, -1, -1, -1, -1, ]
, // Version 4
vec![6, 26, -1, -1, -1, -1, -1, ]
, // Version 5
vec![6, 30, -1, -1, -1, -1, -1, ]
, // Version 6
vec![6, 34, -1, -1, -1, -1, -1, ]
, // Version 7
vec![6, 22, 38, -1, -1, -1, -1, ]
, // Version 8
vec![6, 24, 42, -1, -1, -1, -1, ]
, // Version 9
vec![6, 26, 46, -1, -1, -1, -1, ]
, // Version 10
vec![6, 28, 50, -1, -1, -1, -1, ]
, // Version 11
vec![6, 30, 54, -1, -1, -1, -1, ]
, // Version 12
vec![6, 32, 58, -1, -1, -1, -1, ]
, // Version 13
vec![6, 34, 62, -1, -1, -1, -1, ]
, // Version 14
vec![6, 26, 46, 66, -1, -1, -1, ]
, // Version 15
vec![6, 26, 48, 70, -1, -1, -1, ]
, // Version 16
vec![6, 26, 50, 74, -1, -1, -1, ]
, // Version 17
vec![6, 30, 54, 78, -1, -1, -1, ]
, // Version 18
vec![6, 30, 56, 82, -1, -1, -1, ]
, // Version 19
vec![6, 30, 58, 86, -1, -1, -1, ]
, // Version 20
vec![6, 34, 62, 90, -1, -1, -1, ]
, // Version 21
vec![6, 28, 50, 72, 94, -1, -1, ]
, // Version 22
vec![6, 26, 50, 74, 98, -1, -1, ]
, // Version 23
vec![6, 30, 54, 78, 102, -1, -1, ]
, // Version 24
vec![6, 28, 54, 80, 106, -1, -1, ]
, // Version 25
vec![6, 32, 58, 84, 110, -1, -1, ]
, // Version 26
vec![6, 30, 58, 86, 114, -1, -1, ]
, // Version 27
vec![6, 34, 62, 90, 118, -1, -1, ]
, // Version 28
vec![6, 26, 50, 74, 98, 122, -1, ]
, // Version 29
vec![6, 30, 54, 78, 102, 126, -1, ]
, // Version 30
vec![6, 26, 52, 78, 104, 130, -1, ]
, // Version 31
vec![6, 30, 56, 82, 108, 134, -1, ]
, // Version 32
vec![6, 34, 60, 86, 112, 138, -1, ]
, // Version 33
vec![6, 30, 58, 86, 114, 142, -1, ]
, // Version 34
vec![6, 34, 62, 90, 118, 146, -1, ]
, // Version 35
vec![6, 30, 54, 78, 102, 126, 150, ]
, // Version 36
vec![6, 24, 50, 76, 102, 128, 154, ]
, // Version 37
vec![6, 28, 54, 80, 106, 132, 158, ]
, // Version 38
vec![6, 32, 58, 84, 110, 136, 162, ]
, // Version 39
vec![6, 26, 54, 82, 110, 138, 166, ]
, // Version 40
vec![6, 30, 58, 86, 114, 142, 170, ]
, ]
;

// Type info cells at the left top corner.
 const TYPE_INFO_COORDINATES: vec![vec![Vec<Vec<i32>>; 2]; 15] = vec![vec![8, 0, ]
, vec![8, 1, ]
, vec![8, 2, ]
, vec![8, 3, ]
, vec![8, 4, ]
, vec![8, 5, ]
, vec![8, 7, ]
, vec![8, 8, ]
, vec![7, 8, ]
, vec![5, 8, ]
, vec![4, 8, ]
, vec![3, 8, ]
, vec![2, 8, ]
, vec![1, 8, ]
, vec![0, 8, ]
, ]
;

// From Appendix D in JISX0510:2004 (p. 67)
// 1 1111 0010 0101
 const VERSION_INFO_POLY: i32 = 0x1f25;

// From Appendix C in JISX0510:2004 (p.65).
 const TYPE_INFO_POLY: i32 = 0x537;

 const TYPE_INFO_MASK_PATTERN: i32 = 0x5412;
struct MatrixUtil {
}

impl MatrixUtil {

    fn new() -> MatrixUtil {
    // do nothing
    }

    // Set all cells to -1.  -1 means that the cell is empty (not set yet).
    //
    // JAVAPORT: We shouldn't need to do this at all. The code should be rewritten to begin encoding
    // with the ByteMatrix initialized all to zero.
    fn  clear_matrix( matrix: &ByteMatrix)   {
        matrix.clear(-1 as i8);
    }

    // Build 2D matrix of QR Code from "dataBits" with "ecLevel", "version" and "getMaskPattern". On
    // success, store the result in "matrix" and return true.
    fn  build_matrix( data_bits: &BitArray,  ec_level: &ErrorCorrectionLevel,  version: &Version,  mask_pattern: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        ::clear_matrix(matrix);
        ::embed_basic_patterns(version, matrix);
        // Type information appear with any version.
        ::embed_type_info(ec_level, mask_pattern, matrix);
        // Version info appear if version >= 7.
        ::maybe_embed_version_info(version, matrix);
        // Data should be embedded at end.
        ::embed_data_bits(data_bits, mask_pattern, matrix);
    }

    // Embed basic patterns. On success, modify the matrix and return true.
    // The basic patterns are:
    // - Position detection patterns
    // - Timing patterns
    // - Dark dot at the left bottom corner
    // - Position adjustment patterns, if need be
    fn  embed_basic_patterns( version: &Version,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        // Let's get started with embedding big squares at corners.
        ::embed_position_detection_patterns_and_separators(matrix);
        // Then, embed the dark dot at the left bottom corner.
        ::embed_dark_dot_at_left_bottom_corner(matrix);
        // Position adjustment patterns appear if version >= 2.
        ::maybe_embed_position_adjustment_patterns(version, matrix);
        // Timing patterns should be embedded after position adj. patterns.
        ::embed_timing_patterns(matrix);
    }

    // Embed type information. On success, modify the matrix.
    fn  embed_type_info( ec_level: &ErrorCorrectionLevel,  mask_pattern: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let type_info_bits: BitArray = BitArray::new();
        ::make_type_info_bits(ec_level, mask_pattern, type_info_bits);
         {
             let mut i: i32 = 0;
            while i < type_info_bits.get_size() {
                {
                    // Place bits in LSB to MSB order.  LSB (least significant bit) is the last value in
                    // "typeInfoBits".
                     let bit: bool = type_info_bits.get(type_info_bits.get_size() - 1 - i);
                    // Type info bits at the left top corner. See 8.9 of JISX0510:2004 (p.46).
                     let coordinates: Vec<i32> = TYPE_INFO_COORDINATES[i];
                     let x1: i32 = coordinates[0];
                     let y1: i32 = coordinates[1];
                    matrix.set(x1, y1, bit);
                     let mut x2: i32;
                     let mut y2: i32;
                    if i < 8 {
                        // Right top corner.
                        x2 = matrix.get_width() - i - 1;
                        y2 = 8;
                    } else {
                        // Left bottom corner.
                        x2 = 8;
                        y2 = matrix.get_height() - 7 + (i - 8);
                    }
                    matrix.set(x2, y2, bit);
                }
                i += 1;
             }
         }

    }

    // Embed version information if need be. On success, modify the matrix and return true.
    // See 8.10 of JISX0510:2004 (p.47) for how to embed version information.
    fn  maybe_embed_version_info( version: &Version,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if version.get_version_number() < 7 {
            // Don't need version info.
            return;
        }
         let version_info_bits: BitArray = BitArray::new();
        ::make_version_info_bits(version, version_info_bits);
        // It will decrease from 17 to 0.
         let bit_index: i32 = 6 * 3 - 1;
         {
             let mut i: i32 = 0;
            while i < 6 {
                {
                     {
                         let mut j: i32 = 0;
                        while j < 3 {
                            {
                                // Place bits in LSB (least significant bit) to MSB order.
                                 let bit: bool = version_info_bits.get(bit_index);
                                bit_index -= 1;
                                // Left bottom corner.
                                matrix.set(i, matrix.get_height() - 11 + j, bit);
                                // Right bottom corner.
                                matrix.set(matrix.get_height() - 11 + j, i, bit);
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

    }

    // Embed "dataBits" using "getMaskPattern". On success, modify the matrix and return true.
    // For debugging purposes, it skips masking process if "getMaskPattern" is -1.
    // See 8.7 of JISX0510:2004 (p.38) for how to embed data bits.
    fn  embed_data_bits( data_bits: &BitArray,  mask_pattern: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let bit_index: i32 = 0;
         let mut direction: i32 = -1;
        // Start from the right bottom cell.
         let mut x: i32 = matrix.get_width() - 1;
         let mut y: i32 = matrix.get_height() - 1;
        while x > 0 {
            // Skip the vertical timing pattern.
            if x == 6 {
                x -= 1;
            }
            while y >= 0 && y < matrix.get_height() {
                 {
                     let mut i: i32 = 0;
                    while i < 2 {
                        {
                             let xx: i32 = x - i;
                            // Skip the cell if it's not empty.
                            if !::is_empty(&matrix.get(xx, y)) {
                                continue;
                            }
                             let mut bit: bool;
                            if bit_index < data_bits.get_size() {
                                bit = data_bits.get(bit_index);
                                bit_index += 1;
                            } else {
                                // Padding bit. If there is no bit left, we'll fill the left cells with 0, as described
                                // in 8.4.9 of JISX0510:2004 (p. 24).
                                bit = false;
                            }
                            // Skip masking if mask_pattern is -1.
                            if mask_pattern != -1 && MaskUtil::get_data_mask_bit(mask_pattern, xx, y) {
                                bit = !bit;
                            }
                            matrix.set(xx, y, bit);
                        }
                        i += 1;
                     }
                 }

                y += direction;
            }
            // Reverse the direction.
            direction = -direction;
            y += direction;
            // Move to the left.
            x -= 2;
        }
        // All bits should be consumed.
        if bit_index != data_bits.get_size() {
            throw WriterException::new(format!("Not all bits consumed: {}/{}", bit_index, data_bits.get_size()));
        }
    }

    // Return the position of the most significant bit set (to one) in the "value". The most
    // significant bit is position 32. If there is no bit set, return 0. Examples:
    // - findMSBSet(0) => 0
    // - findMSBSet(1) => 1
    // - findMSBSet(255) => 8
    fn  find_m_s_b_set( value: i32) -> i32  {
        return 32 - Integer::number_of_leading_zeros(value);
    }

    // Calculate BCH (Bose-Chaudhuri-Hocquenghem) code for "value" using polynomial "poly". The BCH
    // code is used for encoding type information and version information.
    // Example: Calculation of version information of 7.
    // f(x) is created from 7.
    //   - 7 = 000111 in 6 bits
    //   - f(x) = x^2 + x^1 + x^0
    // g(x) is given by the standard (p. 67)
    //   - g(x) = x^12 + x^11 + x^10 + x^9 + x^8 + x^5 + x^2 + 1
    // Multiply f(x) by x^(18 - 6)
    //   - f'(x) = f(x) * x^(18 - 6)
    //   - f'(x) = x^14 + x^13 + x^12
    // Calculate the remainder of f'(x) / g(x)
    //         x^2
    //         __________________________________________________
    //   g(x) )x^14 + x^13 + x^12
    //         x^14 + x^13 + x^12 + x^11 + x^10 + x^7 + x^4 + x^2
    //         --------------------------------------------------
    //                              x^11 + x^10 + x^7 + x^4 + x^2
    //
    // The remainder is x^11 + x^10 + x^7 + x^4 + x^2
    // Encode it in binary: 110010010100
    // The return value is 0xc94 (1100 1001 0100)
    //
    // Since all coefficients in the polynomials are 1 or 0, we can do the calculation by bit
    // operations. We don't care if coefficients are positive or negative.
    fn  calculate_b_c_h_code( value: i32,  poly: i32) -> i32  {
        if poly == 0 {
            throw IllegalArgumentException::new("0 polynomial");
        }
        // If poly is "1 1111 0010 0101" (version info poly), msbSetInPoly is 13. We'll subtract 1
        // from 13 to make it 12.
         let msb_set_in_poly: i32 = ::find_m_s_b_set(poly);
        value <<= msb_set_in_poly - 1;
        // Do the division business using exclusive-or operations.
        while ::find_m_s_b_set(value) >= msb_set_in_poly {
            value ^= poly << (::find_m_s_b_set(value) - msb_set_in_poly);
        }
        // Now the "value" is the remainder (i.e. the BCH code)
        return value;
    }

    // Make bit vector of type information. On success, store the result in "bits" and return true.
    // Encode error correction level and mask pattern. See 8.9 of
    // JISX0510:2004 (p.45) for details.
    fn  make_type_info_bits( ec_level: &ErrorCorrectionLevel,  mask_pattern: i32,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if !QRCode::is_valid_mask_pattern(mask_pattern) {
            throw WriterException::new("Invalid mask pattern");
        }
         let type_info: i32 = (ec_level.get_bits() << 3) | mask_pattern;
        bits.append_bits(type_info, 5);
         let bch_code: i32 = ::calculate_b_c_h_code(type_info, TYPE_INFO_POLY);
        bits.append_bits(bch_code, 10);
         let mask_bits: BitArray = BitArray::new();
        mask_bits.append_bits(TYPE_INFO_MASK_PATTERN, 15);
        bits.xor(mask_bits);
        if bits.get_size() != 15 {
            // Just in case.
            throw WriterException::new(format!("should not happen but we got: {}", bits.get_size()));
        }
    }

    // Make bit vector of version information. On success, store the result in "bits" and return true.
    // See 8.10 of JISX0510:2004 (p.45) for details.
    fn  make_version_info_bits( version: &Version,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        bits.append_bits(&version.get_version_number(), 6);
         let bch_code: i32 = ::calculate_b_c_h_code(&version.get_version_number(), VERSION_INFO_POLY);
        bits.append_bits(bch_code, 12);
        if bits.get_size() != 18 {
            // Just in case.
            throw WriterException::new(format!("should not happen but we got: {}", bits.get_size()));
        }
    }

    // Check if "value" is empty.
    fn  is_empty( value: i32) -> bool  {
        return value == -1;
    }

    fn  embed_timing_patterns( matrix: &ByteMatrix)   {
        // separation patterns (size 1). Thus, 8 = 7 + 1.
         {
             let mut i: i32 = 8;
            while i < matrix.get_width() - 8 {
                {
                     let bit: i32 = (i + 1) % 2;
                    // Horizontal line.
                    if ::is_empty(&matrix.get(i, 6)) {
                        matrix.set(i, 6, bit);
                    }
                    // Vertical line.
                    if ::is_empty(&matrix.get(6, i)) {
                        matrix.set(6, i, bit);
                    }
                }
                i += 1;
             }
         }

    }

    // Embed the lonely dark dot at left bottom corner. JISX0510:2004 (p.46)
    fn  embed_dark_dot_at_left_bottom_corner( matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if matrix.get(8, matrix.get_height() - 8) == 0 {
            throw WriterException::new();
        }
        matrix.set(8, matrix.get_height() - 8, 1);
    }

    fn  embed_horizontal_separation_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         {
             let mut x: i32 = 0;
            while x < 8 {
                {
                    if !::is_empty(&matrix.get(x_start + x, y_start)) {
                        throw WriterException::new();
                    }
                    matrix.set(x_start + x, y_start, 0);
                }
                x += 1;
             }
         }

    }

    fn  embed_vertical_separation_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         {
             let mut y: i32 = 0;
            while y < 7 {
                {
                    if !::is_empty(&matrix.get(x_start, y_start + y)) {
                        throw WriterException::new();
                    }
                    matrix.set(x_start, y_start + y, 0);
                }
                y += 1;
             }
         }

    }

    fn  embed_position_adjustment_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)   {
         {
             let mut y: i32 = 0;
            while y < 5 {
                {
                     let pattern_y: Vec<i32> = POSITION_ADJUSTMENT_PATTERN[y];
                     {
                         let mut x: i32 = 0;
                        while x < 5 {
                            {
                                matrix.set(x_start + x, y_start + y, pattern_y[x]);
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

    }

    fn  embed_position_detection_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)   {
         {
             let mut y: i32 = 0;
            while y < 7 {
                {
                     let pattern_y: Vec<i32> = POSITION_DETECTION_PATTERN[y];
                     {
                         let mut x: i32 = 0;
                        while x < 7 {
                            {
                                matrix.set(x_start + x, y_start + y, pattern_y[x]);
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

    }

    // Embed position detection patterns and surrounding vertical/horizontal separators.
    fn  embed_position_detection_patterns_and_separators( matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        // Embed three big squares at corners.
         let pdp_width: i32 = POSITION_DETECTION_PATTERN[0].len();
        // Left top corner.
        ::embed_position_detection_pattern(0, 0, matrix);
        // Right top corner.
        ::embed_position_detection_pattern(matrix.get_width() - pdp_width, 0, matrix);
        // Left bottom corner.
        ::embed_position_detection_pattern(0, matrix.get_width() - pdp_width, matrix);
        // Embed horizontal separation patterns around the squares.
         let hsp_width: i32 = 8;
        // Left top corner.
        ::embed_horizontal_separation_pattern(0, hsp_width - 1, matrix);
        // Right top corner.
        ::embed_horizontal_separation_pattern(matrix.get_width() - hsp_width, hsp_width - 1, matrix);
        // Left bottom corner.
        ::embed_horizontal_separation_pattern(0, matrix.get_width() - hsp_width, matrix);
        // Embed vertical separation patterns around the squares.
         let vsp_size: i32 = 7;
        // Left top corner.
        ::embed_vertical_separation_pattern(vsp_size, 0, matrix);
        // Right top corner.
        ::embed_vertical_separation_pattern(matrix.get_height() - vsp_size - 1, 0, matrix);
        // Left bottom corner.
        ::embed_vertical_separation_pattern(vsp_size, matrix.get_height() - vsp_size, matrix);
    }

    // Embed position adjustment patterns if need be.
    fn  maybe_embed_position_adjustment_patterns( version: &Version,  matrix: &ByteMatrix)   {
        if version.get_version_number() < 2 {
            // The patterns appear if version >= 2
            return;
        }
         let index: i32 = version.get_version_number() - 1;
         let coordinates: Vec<i32> = POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE[index];
        for  let y: i32 in coordinates {
            if y >= 0 {
                for  let x: i32 in coordinates {
                    if x >= 0 && ::is_empty(&matrix.get(x, y)) {
                        // If the cell is unset, we embed the position adjustment pattern here.
                        // -2 is necessary since the x/y coordinates point to the center of the pattern, not the
                        // left top corner.
                        ::embed_position_adjustment_pattern(x - 2, y - 2, matrix);
                    }
                }
            }
        }
    }
}

