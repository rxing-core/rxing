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
// package com::google::zxing::pdf417;

/**
 * @author Jacob Haynes
 * @author qwandor@google.com (Andrew Walbran)
 */

/**
   * default white space (margin) around the code
   */
 const WHITE_SPACE: i32 = 30;

/**
   * default error correction level
   */
 const DEFAULT_ERROR_CORRECTION_LEVEL: i32 = 2;
#[derive(Writer)]
pub struct PDF417Writer {
}

impl PDF417Writer {

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> /*  throws WriterException */Result<BitMatrix, Rc<Exception>>   {
        if format != BarcodeFormat::PDF_417 {
            throw IllegalArgumentException::new(format!("Can only encode PDF_417, but got {}", format));
        }
         let encoder: PDF417 = PDF417::new();
         let mut margin: i32 = WHITE_SPACE;
         let error_correction_level: i32 = DEFAULT_ERROR_CORRECTION_LEVEL;
         let auto_e_c_i: bool = false;
        if hints != null {
            if hints.contains_key(EncodeHintType::PDF417_COMPACT) {
                encoder.set_compact(&Boolean::parse_boolean(&hints.get(EncodeHintType::PDF417_COMPACT).to_string()));
            }
            if hints.contains_key(EncodeHintType::PDF417_COMPACTION) {
                encoder.set_compaction(&Compaction::value_of(&hints.get(EncodeHintType::PDF417_COMPACTION).to_string()));
            }
            if hints.contains_key(EncodeHintType::PDF417_DIMENSIONS) {
                 let dimensions: Dimensions = hints.get(EncodeHintType::PDF417_DIMENSIONS) as Dimensions;
                encoder.set_dimensions(&dimensions.get_max_cols(), &dimensions.get_min_cols(), &dimensions.get_max_rows(), &dimensions.get_min_rows());
            }
            if hints.contains_key(EncodeHintType::MARGIN) {
                margin = Integer::parse_int(&hints.get(EncodeHintType::MARGIN).to_string());
            }
            if hints.contains_key(EncodeHintType::ERROR_CORRECTION) {
                error_correction_level = Integer::parse_int(&hints.get(EncodeHintType::ERROR_CORRECTION).to_string());
            }
            if hints.contains_key(EncodeHintType::CHARACTER_SET) {
                 let encoding: Charset = Charset::for_name(&hints.get(EncodeHintType::CHARACTER_SET).to_string());
                encoder.set_encoding(&encoding);
            }
            auto_e_c_i = hints.contains_key(EncodeHintType::PDF417_AUTO_ECI) && Boolean::parse_boolean(&hints.get(EncodeHintType::PDF417_AUTO_ECI).to_string());
        }
        return Ok(::bit_matrix_from_encoder(encoder, &contents, error_correction_level, width, height, margin, auto_e_c_i));
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> /*  throws WriterException */Result<BitMatrix, Rc<Exception>>   {
        return Ok(self.encode(&contents, format, width, height, null));
    }

    /**
   * Takes encoder, accounts for width/height, and retrieves bit matrix
   */
    fn  bit_matrix_from_encoder( encoder: &PDF417,  contents: &String,  error_correction_level: i32,  width: i32,  height: i32,  margin: i32,  auto_e_c_i: bool) -> /*  throws WriterException */Result<BitMatrix, Rc<Exception>>   {
        encoder.generate_barcode_logic(&contents, error_correction_level, auto_e_c_i);
         let aspect_ratio: i32 = 4;
         let original_scale: Vec<Vec<i8>> = encoder.get_barcode_matrix().get_scaled_matrix(1, aspect_ratio);
         let mut rotated: bool = false;
        if (height > width) != (original_scale[0].len() < original_scale.len()) {
            original_scale = ::rotate_array(&original_scale);
            rotated = true;
        }
         let scale_x: i32 = width / original_scale[0].len();
         let scale_y: i32 = height / original_scale.len();
         let scale: i32 = Math::min(scale_x, scale_y);
        if scale > 1 {
             let scaled_matrix: Vec<Vec<i8>> = encoder.get_barcode_matrix().get_scaled_matrix(scale, scale * aspect_ratio);
            if rotated {
                scaled_matrix = ::rotate_array(&scaled_matrix);
            }
            return Ok(::bit_matrix_from_bit_array(&scaled_matrix, margin));
        }
        return Ok(::bit_matrix_from_bit_array(&original_scale, margin));
    }

    /**
   * This takes an array holding the values of the PDF 417
   *
   * @param input a byte array of information with 0 is black, and 1 is white
   * @param margin border around the barcode
   * @return BitMatrix of the input
   */
    fn  bit_matrix_from_bit_array( input: &Vec<Vec<i8>>,  margin: i32) -> BitMatrix  {
        // Creates the bit matrix with extra space for whitespace
         let output: BitMatrix = BitMatrix::new(input[0].len() + 2 * margin, input.len() + 2 * margin);
        output.clear();
         {
             let mut y: i32 = 0, let y_output: i32 = output.get_height() - margin - 1;
            while y < input.len() {
                {
                     let input_y: Vec<i8> = input[y];
                     {
                         let mut x: i32 = 0;
                        while x < input[0].len() {
                            {
                                // Zero is white in the byte matrix
                                if input_y[x] == 1 {
                                    output.set(x + margin, y_output);
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
                y_output -= 1;
             }
         }

        return output;
    }

    /**
   * Takes and rotates the it 90 degrees
   */
    fn  rotate_array( bitarray: &Vec<Vec<i8>>) -> Vec<Vec<i8>>  {
         let mut temp: [[i8; bitarray.len()]; bitarray[0].len()] = [[0; bitarray.len()]; bitarray[0].len()];
         {
             let mut ii: i32 = 0;
            while ii < bitarray.len() {
                {
                    // This makes the direction consistent on screen when rotating the
                    // screen;
                     let mut inverseii: i32 = bitarray.len() - ii - 1;
                     {
                         let mut jj: i32 = 0;
                        while jj < bitarray[0].len() {
                            {
                                temp[jj][inverseii] = bitarray[ii][jj];
                            }
                            jj += 1;
                         }
                     }

                }
                ii += 1;
             }
         }

        return temp;
    }
}

