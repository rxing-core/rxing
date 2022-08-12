/*
 * Copyright 2011 ZXing authors
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
// package com::google::zxing::maxicode;

/**
 * This implementation can detect and decode a MaxiCode in an image.
 */

 const NO_POINTS: [Option<ResultPoint>; 0] = [None; 0];

 const MATRIX_WIDTH: i32 = 30;

 const MATRIX_HEIGHT: i32 = 33;
#[derive(Reader)]
pub struct MaxiCodeReader {

     let decoder: Decoder = Decoder::new();
}

impl MaxiCodeReader {

    /**
   * Locates and decodes a MaxiCode in an image.
   *
   * @return a String representing the content encoded by the MaxiCode
   * @throws NotFoundException if a MaxiCode cannot be found
   * @throws FormatException if a MaxiCode cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode(image, null));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
        // Note that MaxiCode reader effectively always assumes PURE_BARCODE mode
        // and can't detect it in an image
         let bits: BitMatrix = ::extract_pure_bits(&image.get_black_matrix());
         let decoder_result: DecoderResult = self.decoder.decode(bits, &hints);
         let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), NO_POINTS, BarcodeFormat::MAXICODE);
         let ec_level: String = decoder_result.get_e_c_level();
        if ec_level != null {
            result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &ec_level);
        }
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
         let enclosing_rectangle: Vec<i32> = image.get_enclosing_rectangle();
        if enclosing_rectangle == null {
            throw NotFoundException::get_not_found_instance();
        }
         let left: i32 = enclosing_rectangle[0];
         let top: i32 = enclosing_rectangle[1];
         let width: i32 = enclosing_rectangle[2];
         let height: i32 = enclosing_rectangle[3];
        // Now just read off the bits
         let bits: BitMatrix = BitMatrix::new(MATRIX_WIDTH, MATRIX_HEIGHT);
         {
             let mut y: i32 = 0;
            while y < MATRIX_HEIGHT {
                {
                     let iy: i32 = Math::min(top + (y * height + height / 2) / MATRIX_HEIGHT, height - 1);
                     {
                         let mut x: i32 = 0;
                        while x < MATRIX_WIDTH {
                            {
                                // srowen: I don't quite understand why the formula below is necessary, but it
                                // can walk off the image if left + width = the right boundary. So cap it.
                                 let ix: i32 = left + Math::min((x * width + width / 2 + (y & 0x01) * width / 2) / MATRIX_WIDTH, width - 1);
                                if image.get(ix, iy) {
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
}

