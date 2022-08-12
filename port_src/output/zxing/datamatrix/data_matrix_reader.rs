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
// package com::google::zxing::datamatrix;

/**
 * This implementation can detect and decode Data Matrix codes in an image.
 *
 * @author bbrown@google.com (Brian Brown)
 */

 const NO_POINTS: [Option<ResultPoint>; 0] = [None; 0];
#[derive(Reader)]
pub struct DataMatrixReader {

     let decoder: Decoder = Decoder::new();
}

impl DataMatrixReader {

    /**
   * Locates and decodes a Data Matrix code in an image.
   *
   * @return a String representing the content encoded by the Data Matrix code
   * @throws NotFoundException if a Data Matrix code cannot be found
   * @throws FormatException if a Data Matrix code cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode(image, null));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
         let decoder_result: DecoderResult;
         let mut points: Vec<ResultPoint>;
        if hints != null && hints.contains_key(DecodeHintType::PURE_BARCODE) {
             let bits: BitMatrix = ::extract_pure_bits(&image.get_black_matrix());
            decoder_result = self.decoder.decode(bits);
            points = NO_POINTS;
        } else {
             let detector_result: DetectorResult = Detector::new(&image.get_black_matrix()).detect();
            decoder_result = self.decoder.decode(&detector_result.get_bits());
            points = detector_result.get_points();
        }
         let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), points, BarcodeFormat::DATA_MATRIX);
         let byte_segments: List<Vec<i8>> = decoder_result.get_byte_segments();
        if byte_segments != null {
            result.put_metadata(ResultMetadataType::BYTE_SEGMENTS, &byte_segments);
        }
         let ec_level: String = decoder_result.get_e_c_level();
        if ec_level != null {
            result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &ec_level);
        }
        result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, format!("]d{}", decoder_result.get_symbology_modifier()));
        return Ok(result);
    }

    pub fn  reset(&self)   {
    // do nothing
    }

    /**
   * This method detects a code in a "pure" image -- that is, pure monochrome image
   * which contains only an unrotated, unskewed, image of a code, with some white border
   * around it. This is a specialized method that works exceptionally fast in this special
   * case.
   */
    fn  extract_pure_bits( image: &BitMatrix) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
         let left_top_black: Vec<i32> = image.get_top_left_on_bit();
         let right_bottom_black: Vec<i32> = image.get_bottom_right_on_bit();
        if left_top_black == null || right_bottom_black == null {
            throw NotFoundException::get_not_found_instance();
        }
         let module_size: i32 = self.module_size(&left_top_black, image);
         let mut top: i32 = left_top_black[1];
         let bottom: i32 = right_bottom_black[1];
         let mut left: i32 = left_top_black[0];
         let right: i32 = right_bottom_black[0];
         let matrix_width: i32 = (right - left + 1) / module_size;
         let matrix_height: i32 = (bottom - top + 1) / module_size;
        if matrix_width <= 0 || matrix_height <= 0 {
            throw NotFoundException::get_not_found_instance();
        }
        // Push in the "border" by half the module width so that we start
        // sampling in the middle of the module. Just in case the image is a
        // little off, this will help recover.
         let nudge: i32 = module_size / 2;
        top += nudge;
        left += nudge;
        // Now just read off the bits
         let bits: BitMatrix = BitMatrix::new(matrix_width, matrix_height);
         {
             let mut y: i32 = 0;
            while y < matrix_height {
                {
                     let i_offset: i32 = top + y * module_size;
                     {
                         let mut x: i32 = 0;
                        while x < matrix_width {
                            {
                                if image.get(left + x * module_size, i_offset) {
                                    bits.set(x, y);
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return Ok(bits);
    }

    fn  module_size( left_top_black: &Vec<i32>,  image: &BitMatrix) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let width: i32 = image.get_width();
         let mut x: i32 = left_top_black[0];
         let y: i32 = left_top_black[1];
        while x < width && image.get(x, y) {
            x += 1;
        }
        if x == width {
            throw NotFoundException::get_not_found_instance();
        }
         let module_size: i32 = x - left_top_black[0];
        if module_size == 0 {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(module_size);
    }
}

