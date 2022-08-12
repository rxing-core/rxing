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
// package com::google::zxing::qrcode;

/**
 * This object renders a QR Code as a BitMatrix 2D array of greyscale values.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

 const QUIET_ZONE_SIZE: i32 = 4;
#[derive(Writer)]
pub struct QRCodeWriter {
}

impl QRCodeWriter {

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> /*  throws WriterException */Result<BitMatrix, Rc<Exception>>   {
        return Ok(self.encode(&contents, format, width, height, null));
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> /*  throws WriterException */Result<BitMatrix, Rc<Exception>>   {
        if contents.is_empty() {
            throw IllegalArgumentException::new("Found empty contents");
        }
        if format != BarcodeFormat::QR_CODE {
            throw IllegalArgumentException::new(format!("Can only encode QR_CODE, but got {}", format));
        }
        if width < 0 || height < 0 {
            throw IllegalArgumentException::new(format!("Requested dimensions are too small: {}x{}", width, height));
        }
         let error_correction_level: ErrorCorrectionLevel = ErrorCorrectionLevel::L;
         let quiet_zone: i32 = QUIET_ZONE_SIZE;
        if hints != null {
            if hints.contains_key(EncodeHintType::ERROR_CORRECTION) {
                error_correction_level = ErrorCorrectionLevel::value_of(&hints.get(EncodeHintType::ERROR_CORRECTION).to_string());
            }
            if hints.contains_key(EncodeHintType::MARGIN) {
                quiet_zone = Integer::parse_int(&hints.get(EncodeHintType::MARGIN).to_string());
            }
        }
         let code: QRCode = Encoder::encode(&contents, error_correction_level, &hints);
        return Ok(::render_result(code, width, height, quiet_zone));
    }

    // Note that the input matrix uses 0 == white, 1 == black, while the output matrix uses
    // 0 == black, 255 == white (i.e. an 8 bit greyscale bitmap).
    fn  render_result( code: &QRCode,  width: i32,  height: i32,  quiet_zone: i32) -> BitMatrix  {
         let input: ByteMatrix = code.get_matrix();
        if input == null {
            throw IllegalStateException::new();
        }
         let input_width: i32 = input.get_width();
         let input_height: i32 = input.get_height();
         let qr_width: i32 = input_width + (quiet_zone * 2);
         let qr_height: i32 = input_height + (quiet_zone * 2);
         let output_width: i32 = Math::max(width, qr_width);
         let output_height: i32 = Math::max(height, qr_height);
         let multiple: i32 = Math::min(output_width / qr_width, output_height / qr_height);
        // Padding includes both the quiet zone and the extra white pixels to accommodate the requested
        // dimensions. For example, if input is 25x25 the QR will be 33x33 including the quiet zone.
        // If the requested size is 200x160, the multiple will be 4, for a QR of 132x132. These will
        // handle all the padding from 100x100 (the actual QR) up to 200x160.
         let left_padding: i32 = (output_width - (input_width * multiple)) / 2;
         let top_padding: i32 = (output_height - (input_height * multiple)) / 2;
         let output: BitMatrix = BitMatrix::new(output_width, output_height);
         {
             let input_y: i32 = 0, let output_y: i32 = top_padding;
            while input_y < input_height {
                {
                    // Write the contents of this row of the barcode
                     {
                         let input_x: i32 = 0, let output_x: i32 = left_padding;
                        while input_x < input_width {
                            {
                                if input.get(input_x, input_y) == 1 {
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

