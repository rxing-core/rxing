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
// package com::google::zxing::aztec::encoder;

/**
 * Generates Aztec 2D barcodes.
 *
 * @author Rustam Abdullaev
 */

// default minimal percentage of error check words
 const DEFAULT_EC_PERCENT: i32 = 33;

 const DEFAULT_AZTEC_LAYERS: i32 = 0;

 const MAX_NB_BITS: i32 = 32;

 const MAX_NB_BITS_COMPACT: i32 = 4;

 const WORD_SIZE: vec![Vec<i32>; 33] = vec![4, 6, 6, 8, 8, 8, 8, 8, 8, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, ]
;
pub struct Encoder {
}

impl Encoder {

    fn new() -> Encoder {
    }

    /**
   * Encodes the given string content as an Aztec symbol (without ECI code)
   *
   * @param data input data string; must be encodable as ISO/IEC 8859-1 (Latin-1)
   * @return Aztec symbol matrix with metadata
   */
    pub fn  encode( data: &String) -> AztecCode  {
        return ::encode(&data.get_bytes(StandardCharsets::ISO_8859_1));
    }

    /**
   * Encodes the given string content as an Aztec symbol (without ECI code)
   *
   * @param data input data string; must be encodable as ISO/IEC 8859-1 (Latin-1)
   * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
   *                      a minimum of 23% + 3 words is recommended)
   * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
   * @return Aztec symbol matrix with metadata
   */
    pub fn  encode( data: &String,  min_e_c_c_percent: i32,  user_specified_layers: i32) -> AztecCode  {
        return ::encode(&data.get_bytes(StandardCharsets::ISO_8859_1), min_e_c_c_percent, user_specified_layers, null);
    }

    /**
   * Encodes the given string content as an Aztec symbol
   *
   * @param data input data string
   * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
   *                      a minimum of 23% + 3 words is recommended)
   * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
   * @param charset character set in which to encode string using ECI; if null, no ECI code
   *                will be inserted, and the string must be encodable as ISO/IEC 8859-1
   *                (Latin-1), the default encoding of the symbol.
   * @return Aztec symbol matrix with metadata
   */
    pub fn  encode( data: &String,  min_e_c_c_percent: i32,  user_specified_layers: i32,  charset: &Charset) -> AztecCode  {
         let bytes: Vec<i8> = data.get_bytes( if null != charset { charset } else { StandardCharsets::ISO_8859_1 });
        return ::encode(&bytes, min_e_c_c_percent, user_specified_layers, &charset);
    }

    /**
   * Encodes the given binary content as an Aztec symbol (without ECI code)
   *
   * @param data input data string
   * @return Aztec symbol matrix with metadata
   */
    pub fn  encode( data: &Vec<i8>) -> AztecCode  {
        return ::encode(&data, DEFAULT_EC_PERCENT, DEFAULT_AZTEC_LAYERS, null);
    }

    /**
   * Encodes the given binary content as an Aztec symbol (without ECI code)
   *
   * @param data input data string
   * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
   *                      a minimum of 23% + 3 words is recommended)
   * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
   * @return Aztec symbol matrix with metadata
   */
    pub fn  encode( data: &Vec<i8>,  min_e_c_c_percent: i32,  user_specified_layers: i32) -> AztecCode  {
        return ::encode(&data, min_e_c_c_percent, user_specified_layers, null);
    }

    /**
   * Encodes the given binary content as an Aztec symbol
   *
   * @param data input data string
   * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
   *                      a minimum of 23% + 3 words is recommended)
   * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
   * @param charset character set to mark using ECI; if null, no ECI code will be inserted, and the
   *                default encoding of ISO/IEC 8859-1 will be assuming by readers.
   * @return Aztec symbol matrix with metadata
   */
    pub fn  encode( data: &Vec<i8>,  min_e_c_c_percent: i32,  user_specified_layers: i32,  charset: &Charset) -> AztecCode  {
        // High-level encode
         let bits: BitArray = HighLevelEncoder::new(&data, &charset).encode();
        // stuff bits and choose symbol size
         let ecc_bits: i32 = bits.get_size() * min_e_c_c_percent / 100 + 11;
         let total_size_bits: i32 = bits.get_size() + ecc_bits;
         let mut compact: bool;
         let mut layers: i32;
         let total_bits_in_layer: i32;
         let word_size: i32;
         let stuffed_bits: BitArray;
        if user_specified_layers != DEFAULT_AZTEC_LAYERS {
            compact = user_specified_layers < 0;
            layers = Math::abs(user_specified_layers);
            if layers > ( if compact { MAX_NB_BITS_COMPACT } else { MAX_NB_BITS }) {
                throw IllegalArgumentException::new(&String::format("Illegal value %s for layers", user_specified_layers));
            }
            total_bits_in_layer = self.total_bits_in_layer(layers, compact);
            word_size = WORD_SIZE[layers];
             let usable_bits_in_layers: i32 = total_bits_in_layer - (total_bits_in_layer % word_size);
            stuffed_bits = ::stuff_bits(bits, word_size);
            if stuffed_bits.get_size() + ecc_bits > usable_bits_in_layers {
                throw IllegalArgumentException::new("Data to large for user specified layer");
            }
            if compact && stuffed_bits.get_size() > word_size * 64 {
                // Compact format only allows 64 data words, though C4 can hold more words than that
                throw IllegalArgumentException::new("Data to large for user specified layer");
            }
        } else {
            word_size = 0;
            stuffed_bits = null;
            // is the same size, but has more data.
             {
                 let mut i: i32 = 0;
                loop  {
                    {
                        if i > MAX_NB_BITS {
                            throw IllegalArgumentException::new("Data too large for an Aztec code");
                        }
                        compact = i <= 3;
                        layers =  if compact { i + 1 } else { i };
                        total_bits_in_layer = self.total_bits_in_layer(layers, compact);
                        if total_size_bits > total_bits_in_layer {
                            continue;
                        }
                        // wordSize has changed
                        if stuffed_bits == null || word_size != WORD_SIZE[layers] {
                            word_size = WORD_SIZE[layers];
                            stuffed_bits = ::stuff_bits(bits, word_size);
                        }
                         let usable_bits_in_layers: i32 = total_bits_in_layer - (total_bits_in_layer % word_size);
                        if compact && stuffed_bits.get_size() > word_size * 64 {
                            // Compact format only allows 64 data words, though C4 can hold more words than that
                            continue;
                        }
                        if stuffed_bits.get_size() + ecc_bits <= usable_bits_in_layers {
                            break;
                        }
                    }
                    i += 1;
                 }
             }

        }
         let message_bits: BitArray = ::generate_check_words(stuffed_bits, total_bits_in_layer, word_size);
        // generate mode message
         let message_size_in_words: i32 = stuffed_bits.get_size() / word_size;
         let mode_message: BitArray = ::generate_mode_message(compact, layers, message_size_in_words);
        // allocate symbol
        // not including alignment lines
         let base_matrix_size: i32 = ( if compact { 11 } else { 14 }) + layers * 4;
         let alignment_map: [i32; base_matrix_size] = [0; base_matrix_size];
         let matrix_size: i32;
        if compact {
            // no alignment marks in compact mode, alignmentMap is a no-op
            matrix_size = base_matrix_size;
             {
                 let mut i: i32 = 0;
                while i < alignment_map.len() {
                    {
                        alignment_map[i] = i;
                    }
                    i += 1;
                 }
             }

        } else {
            matrix_size = base_matrix_size + 1 + 2 * ((base_matrix_size / 2 - 1) / 15);
             let orig_center: i32 = base_matrix_size / 2;
             let center: i32 = matrix_size / 2;
             {
                 let mut i: i32 = 0;
                while i < orig_center {
                    {
                         let new_offset: i32 = i + i / 15;
                        alignment_map[orig_center - i - 1] = center - new_offset - 1;
                        alignment_map[orig_center + i] = center + new_offset + 1;
                    }
                    i += 1;
                 }
             }

        }
         let matrix: BitMatrix = BitMatrix::new(matrix_size);
        // draw data bits
         {
             let mut i: i32 = 0, let row_offset: i32 = 0;
            while i < layers {
                {
                     let row_size: i32 = (layers - i) * 4 + ( if compact { 9 } else { 12 });
                     {
                         let mut j: i32 = 0;
                        while j < row_size {
                            {
                                 let column_offset: i32 = j * 2;
                                 {
                                     let mut k: i32 = 0;
                                    while k < 2 {
                                        {
                                            if message_bits.get(row_offset + column_offset + k) {
                                                matrix.set(alignment_map[i * 2 + k], alignment_map[i * 2 + j]);
                                            }
                                            if message_bits.get(row_offset + row_size * 2 + column_offset + k) {
                                                matrix.set(alignment_map[i * 2 + j], alignment_map[base_matrix_size - 1 - i * 2 - k]);
                                            }
                                            if message_bits.get(row_offset + row_size * 4 + column_offset + k) {
                                                matrix.set(alignment_map[base_matrix_size - 1 - i * 2 - k], alignment_map[base_matrix_size - 1 - i * 2 - j]);
                                            }
                                            if message_bits.get(row_offset + row_size * 6 + column_offset + k) {
                                                matrix.set(alignment_map[base_matrix_size - 1 - i * 2 - j], alignment_map[i * 2 + k]);
                                            }
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            j += 1;
                         }
                     }

                    row_offset += row_size * 8;
                }
                i += 1;
             }
         }

        // draw mode message
        ::draw_mode_message(matrix, compact, matrix_size, mode_message);
        // draw alignment marks
        if compact {
            ::draw_bulls_eye(matrix, matrix_size / 2, 5);
        } else {
            ::draw_bulls_eye(matrix, matrix_size / 2, 7);
             {
                 let mut i: i32 = 0, let mut j: i32 = 0;
                while i < base_matrix_size / 2 - 1 {
                    {
                         {
                             let mut k: i32 = (matrix_size / 2) & 1;
                            while k < matrix_size {
                                {
                                    matrix.set(matrix_size / 2 - j, k);
                                    matrix.set(matrix_size / 2 + j, k);
                                    matrix.set(k, matrix_size / 2 - j);
                                    matrix.set(k, matrix_size / 2 + j);
                                }
                                k += 2;
                             }
                         }

                    }
                    i += 15;
                    j += 16;
                 }
             }

        }
         let aztec: AztecCode = AztecCode::new();
        aztec.set_compact(compact);
        aztec.set_size(matrix_size);
        aztec.set_layers(layers);
        aztec.set_code_words(message_size_in_words);
        aztec.set_matrix(matrix);
        return aztec;
    }

    fn  draw_bulls_eye( matrix: &BitMatrix,  center: i32,  size: i32)   {
         {
             let mut i: i32 = 0;
            while i < size {
                {
                     {
                         let mut j: i32 = center - i;
                        while j <= center + i {
                            {
                                matrix.set(j, center - i);
                                matrix.set(j, center + i);
                                matrix.set(center - i, j);
                                matrix.set(center + i, j);
                            }
                            j += 1;
                         }
                     }

                }
                i += 2;
             }
         }

        matrix.set(center - size, center - size);
        matrix.set(center - size + 1, center - size);
        matrix.set(center - size, center - size + 1);
        matrix.set(center + size, center - size);
        matrix.set(center + size, center - size + 1);
        matrix.set(center + size, center + size - 1);
    }

    fn  generate_mode_message( compact: bool,  layers: i32,  message_size_in_words: i32) -> BitArray  {
         let mode_message: BitArray = BitArray::new();
        if compact {
            mode_message.append_bits(layers - 1, 2);
            mode_message.append_bits(message_size_in_words - 1, 6);
            mode_message = ::generate_check_words(mode_message, 28, 4);
        } else {
            mode_message.append_bits(layers - 1, 5);
            mode_message.append_bits(message_size_in_words - 1, 11);
            mode_message = ::generate_check_words(mode_message, 40, 4);
        }
        return mode_message;
    }

    fn  draw_mode_message( matrix: &BitMatrix,  compact: bool,  matrix_size: i32,  mode_message: &BitArray)   {
         let center: i32 = matrix_size / 2;
        if compact {
             {
                 let mut i: i32 = 0;
                while i < 7 {
                    {
                         let offset: i32 = center - 3 + i;
                        if mode_message.get(i) {
                            matrix.set(offset, center - 5);
                        }
                        if mode_message.get(i + 7) {
                            matrix.set(center + 5, offset);
                        }
                        if mode_message.get(20 - i) {
                            matrix.set(offset, center + 5);
                        }
                        if mode_message.get(27 - i) {
                            matrix.set(center - 5, offset);
                        }
                    }
                    i += 1;
                 }
             }

        } else {
             {
                 let mut i: i32 = 0;
                while i < 10 {
                    {
                         let offset: i32 = center - 5 + i + i / 5;
                        if mode_message.get(i) {
                            matrix.set(offset, center - 7);
                        }
                        if mode_message.get(i + 10) {
                            matrix.set(center + 7, offset);
                        }
                        if mode_message.get(29 - i) {
                            matrix.set(offset, center + 7);
                        }
                        if mode_message.get(39 - i) {
                            matrix.set(center - 7, offset);
                        }
                    }
                    i += 1;
                 }
             }

        }
    }

    fn  generate_check_words( bit_array: &BitArray,  total_bits: i32,  word_size: i32) -> BitArray  {
        // bitArray is guaranteed to be a multiple of the wordSize, so no padding needed
         let message_size_in_words: i32 = bit_array.get_size() / word_size;
         let rs: ReedSolomonEncoder = ReedSolomonEncoder::new(&::get_g_f(word_size));
         let total_words: i32 = total_bits / word_size;
         let message_words: Vec<i32> = ::bits_to_words(bit_array, word_size, total_words);
        rs.encode(&message_words, total_words - message_size_in_words);
         let start_pad: i32 = total_bits % word_size;
         let message_bits: BitArray = BitArray::new();
        message_bits.append_bits(0, start_pad);
        for  let message_word: i32 in message_words {
            message_bits.append_bits(message_word, word_size);
        }
        return message_bits;
    }

    fn  bits_to_words( stuffed_bits: &BitArray,  word_size: i32,  total_words: i32) -> Vec<i32>  {
         let mut message: [i32; total_words] = [0; total_words];
         let mut i: i32;
         let mut n: i32;
         {
            i = 0;
            n = stuffed_bits.get_size() / word_size;
            while i < n {
                {
                     let mut value: i32 = 0;
                     {
                         let mut j: i32 = 0;
                        while j < word_size {
                            {
                                value |=  if stuffed_bits.get(i * word_size + j) { (1 << word_size - j - 1) } else { 0 };
                            }
                            j += 1;
                         }
                     }

                    message[i] = value;
                }
                i += 1;
             }
         }

        return message;
    }

    fn  get_g_f( word_size: i32) -> GenericGF  {
        match word_size {
              4 => 
                 {
                    return GenericGF::AZTEC_PARAM;
                }
              6 => 
                 {
                    return GenericGF::AZTEC_DATA_6;
                }
              8 => 
                 {
                    return GenericGF::AZTEC_DATA_8;
                }
              10 => 
                 {
                    return GenericGF::AZTEC_DATA_10;
                }
              12 => 
                 {
                    return GenericGF::AZTEC_DATA_12;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("Unsupported word size {}", word_size));
                }
        }
    }

    fn  stuff_bits( bits: &BitArray,  word_size: i32) -> BitArray  {
         let out: BitArray = BitArray::new();
         let n: i32 = bits.get_size();
         let mask: i32 = (1 << word_size) - 2;
         {
             let mut i: i32 = 0;
            while i < n {
                {
                     let mut word: i32 = 0;
                     {
                         let mut j: i32 = 0;
                        while j < word_size {
                            {
                                if i + j >= n || bits.get(i + j) {
                                    word |= 1 << (word_size - 1 - j);
                                }
                            }
                            j += 1;
                         }
                     }

                    if (word & mask) == mask {
                        out.append_bits(word & mask, word_size);
                        i -= 1;
                    } else if (word & mask) == 0 {
                        out.append_bits(word | 1, word_size);
                        i -= 1;
                    } else {
                        out.append_bits(word, word_size);
                    }
                }
                i += word_size;
             }
         }

        return out;
    }

    fn  total_bits_in_layer( layers: i32,  compact: bool) -> i32  {
        return (( if compact { 88 } else { 112 }) + 16 * layers) * layers;
    }
}

