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
// package com::google::zxing::datamatrix;

/**
 * This object renders a Data Matrix code as a BitMatrix 2D array of greyscale values.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Guillaume Le Biller Added to zxing lib.
 */
#[derive(Writer)]
pub struct DataMatrixWriter {
}

impl DataMatrixWriter {

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> BitMatrix  {
        return self.encode(&contents, format, width, height, null);
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> BitMatrix  {
        if contents.is_empty() {
            throw IllegalArgumentException::new("Found empty contents");
        }
        if format != BarcodeFormat::DATA_MATRIX {
            throw IllegalArgumentException::new(format!("Can only encode DATA_MATRIX, but got {}", format));
        }
        if width < 0 || height < 0 {
            throw IllegalArgumentException::new(format!("Requested dimensions can't be negative: {}x{}", width, height));
        }
        // Try to get force shape & min / max size
         let mut shape: SymbolShapeHint = SymbolShapeHint::FORCE_NONE;
         let min_size: Dimension = null;
         let max_size: Dimension = null;
        if hints != null {
             let requested_shape: SymbolShapeHint = hints.get(EncodeHintType::DATA_MATRIX_SHAPE) as SymbolShapeHint;
            if requested_shape != null {
                shape = requested_shape;
            }
             let requested_min_size: Dimension = hints.get(EncodeHintType::MIN_SIZE) as Dimension;
            if requested_min_size != null {
                min_size = requested_min_size;
            }
             let requested_max_size: Dimension = hints.get(EncodeHintType::MAX_SIZE) as Dimension;
            if requested_max_size != null {
                max_size = requested_max_size;
            }
        }
        //1. step: Data encodation
         let mut encoded: String;
         let has_compaction_hint: bool = hints != null && hints.contains_key(EncodeHintType::DATA_MATRIX_COMPACT) && Boolean::parse_boolean(&hints.get(EncodeHintType::DATA_MATRIX_COMPACT).to_string());
        if has_compaction_hint {
             let has_g_s1_format_hint: bool = hints.contains_key(EncodeHintType::GS1_FORMAT) && Boolean::parse_boolean(&hints.get(EncodeHintType::GS1_FORMAT).to_string());
             let mut charset: Charset = null;
             let has_encoding_hint: bool = hints.contains_key(EncodeHintType::CHARACTER_SET);
            if has_encoding_hint {
                charset = Charset::for_name(&hints.get(EncodeHintType::CHARACTER_SET).to_string());
            }
            encoded = MinimalEncoder::encode_high_level(&contents, &charset,  if has_g_s1_format_hint { 0x1D } else { -1 }, shape);
        } else {
             let has_force_c40_hint: bool = hints != null && hints.contains_key(EncodeHintType::FORCE_C40) && Boolean::parse_boolean(&hints.get(EncodeHintType::FORCE_C40).to_string());
            encoded = HighLevelEncoder::encode_high_level(&contents, shape, min_size, max_size, has_force_c40_hint);
        }
         let symbol_info: SymbolInfo = SymbolInfo::lookup(&encoded.length(), shape, min_size, max_size, true);
        //2. step: ECC generation
         let codewords: String = ErrorCorrection::encode_e_c_c200(&encoded, symbol_info);
        //3. step: Module placement in Matrix
         let placement: DefaultPlacement = DefaultPlacement::new(&codewords, &symbol_info.get_symbol_data_width(), &symbol_info.get_symbol_data_height());
        placement.place();
        //4. step: low-level encoding
        return ::encode_low_level(placement, symbol_info, width, height);
    }

    /**
   * Encode the given symbol info to a bit matrix.
   *
   * @param placement  The DataMatrix placement.
   * @param symbolInfo The symbol info to encode.
   * @return The bit matrix generated.
   */
    fn  encode_low_level( placement: &DefaultPlacement,  symbol_info: &SymbolInfo,  width: i32,  height: i32) -> BitMatrix  {
         let symbol_width: i32 = symbol_info.get_symbol_data_width();
         let symbol_height: i32 = symbol_info.get_symbol_data_height();
         let matrix: ByteMatrix = ByteMatrix::new(&symbol_info.get_symbol_width(), &symbol_info.get_symbol_height());
         let matrix_y: i32 = 0;
         {
             let mut y: i32 = 0;
            while y < symbol_height {
                {
                    // Fill the top edge with alternate 0 / 1
                     let matrix_x: i32;
                    if (y % symbol_info.matrixHeight) == 0 {
                        matrix_x = 0;
                         {
                             let mut x: i32 = 0;
                            while x < symbol_info.get_symbol_width() {
                                {
                                    matrix.set(matrix_x, matrix_y, (x % 2) == 0);
                                    matrix_x += 1;
                                }
                                x += 1;
                             }
                         }

                        matrix_y += 1;
                    }
                    matrix_x = 0;
                     {
                         let mut x: i32 = 0;
                        while x < symbol_width {
                            {
                                // Fill the right edge with full 1
                                if (x % symbol_info.matrixWidth) == 0 {
                                    matrix.set(matrix_x, matrix_y, true);
                                    matrix_x += 1;
                                }
                                matrix.set(matrix_x, matrix_y, &placement.get_bit(x, y));
                                matrix_x += 1;
                                // Fill the right edge with alternate 0 / 1
                                if (x % symbol_info.matrixWidth) == symbol_info.matrixWidth - 1 {
                                    matrix.set(matrix_x, matrix_y, (y % 2) == 0);
                                    matrix_x += 1;
                                }
                            }
                            x += 1;
                         }
                     }

                    matrix_y += 1;
                    // Fill the bottom edge with full 1
                    if (y % symbol_info.matrixHeight) == symbol_info.matrixHeight - 1 {
                        matrix_x = 0;
                         {
                             let mut x: i32 = 0;
                            while x < symbol_info.get_symbol_width() {
                                {
                                    matrix.set(matrix_x, matrix_y, true);
                                    matrix_x += 1;
                                }
                                x += 1;
                             }
                         }

                        matrix_y += 1;
                    }
                }
                y += 1;
             }
         }

        return ::convert_byte_matrix_to_bit_matrix(matrix, width, height);
    }

    /**
   * Convert the ByteMatrix to BitMatrix.
   *
   * @param reqHeight The requested height of the image (in pixels) with the Datamatrix code
   * @param reqWidth The requested width of the image (in pixels) with the Datamatrix code
   * @param matrix The input matrix.
   * @return The output matrix.
   */
    fn  convert_byte_matrix_to_bit_matrix( matrix: &ByteMatrix,  req_width: i32,  req_height: i32) -> BitMatrix  {
         let matrix_width: i32 = matrix.get_width();
         let matrix_height: i32 = matrix.get_height();
         let output_width: i32 = Math::max(req_width, matrix_width);
         let output_height: i32 = Math::max(req_height, matrix_height);
         let multiple: i32 = Math::min(output_width / matrix_width, output_height / matrix_height);
         let left_padding: i32 = (output_width - (matrix_width * multiple)) / 2;
         let top_padding: i32 = (output_height - (matrix_height * multiple)) / 2;
         let mut output: BitMatrix;
        // remove padding if requested width and height are too small
        if req_height < matrix_height || req_width < matrix_width {
            left_padding = 0;
            top_padding = 0;
            output = BitMatrix::new(matrix_width, matrix_height);
        } else {
            output = BitMatrix::new(req_width, req_height);
        }
        output.clear();
         {
             let input_y: i32 = 0, let output_y: i32 = top_padding;
            while input_y < matrix_height {
                {
                    // Write the contents of this row of the bytematrix
                     {
                         let input_x: i32 = 0, let output_x: i32 = left_padding;
                        while input_x < matrix_width {
                            {
                                if matrix.get(input_x, input_y) == 1 {
                                    output.set_region(output_x, output_y, multiple, multiple);
                                }
                            }
                            input_x += 1;
                            output_x += multiple;
                         }
                     }

                }
                input_y += 1;
                output_y += multiple;
             }
         }

        return output;
    }
}

