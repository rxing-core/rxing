import com.google.zxing.BarcodeFormat;
import com.google.zxing.BinaryBitmap;
import com.google.zxing.ChecksumException;
import com.google.zxing.DecodeHintType;
import com.google.zxing.FormatException;
import com.google.zxing.NotFoundException;
import com.google.zxing.Reader;
import com.google.zxing.Result;
import com.google.zxing.ResultMetadataType;
import com.google.zxing.ResultPoint;
import com.google.zxing.common.BitMatrix;
import com.google.zxing.common.DecoderResult;
import com.google.zxing.common.DetectorResult;
import com.google.zxing.qrcode.decoder.Decoder;
import com.google.zxing.qrcode.decoder.QRCodeDecoderMetaData;
import com.google.zxing.qrcode.detector.Detector;

import com.google.zxing.BarcodeFormat;
import com.google.zxing.EncodeHintType;
import com.google.zxing.Writer;
import com.google.zxing.WriterException;
import com.google.zxing.common.BitMatrix;
import com.google.zxing.qrcode.encoder.ByteMatrix;
import com.google.zxing.qrcode.decoder.ErrorCorrectionLevel;
import com.google.zxing.qrcode.encoder.Encoder;
import com.google.zxing.qrcode.encoder.QRCode;

// NEW FILE: q_r_code_reader.rs
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
// package com::google::zxing::qrcode;

/**
 * This implementation can detect and decode QR Codes in an image.
 *
 * @author Sean Owen
 */

 const NO_POINTS: [Option<ResultPoint>; 0] = [None; 0];
#[derive(Reader)]
pub struct QRCodeReader {

     let decoder: Decoder = Decoder::new();
}

impl QRCodeReader {

    pub fn  get_decoder(&self) -> Decoder  {
        return self.decoder;
    }

    /**
   * Locates and decodes a QR code in an image.
   *
   * @return a String representing the content encoded by the QR code
   * @throws NotFoundException if a QR code cannot be found
   * @throws FormatException if a QR code cannot be decoded
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
            decoder_result = self.decoder.decode(bits, &hints);
            points = NO_POINTS;
        } else {
             let detector_result: DetectorResult = Detector::new(&image.get_black_matrix()).detect(&hints);
            decoder_result = self.decoder.decode(&detector_result.get_bits(), &hints);
            points = detector_result.get_points();
        }
        // If the code was mirrored: swap the bottom-left and the top-right points.
        if decoder_result.get_other() instanceof QRCodeDecoderMetaData {
            (decoder_result.get_other() as QRCodeDecoderMetaData).apply_mirrored_correction(points);
        }
         let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), points, BarcodeFormat::QR_CODE);
         let byte_segments: List<Vec<i8>> = decoder_result.get_byte_segments();
        if byte_segments != null {
            result.put_metadata(ResultMetadataType::BYTE_SEGMENTS, &byte_segments);
        }
         let ec_level: String = decoder_result.get_e_c_level();
        if ec_level != null {
            result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &ec_level);
        }
        if decoder_result.has_structured_append() {
            result.put_metadata(ResultMetadataType::STRUCTURED_APPEND_SEQUENCE, &decoder_result.get_structured_append_sequence_number());
            result.put_metadata(ResultMetadataType::STRUCTURED_APPEND_PARITY, &decoder_result.get_structured_append_parity());
        }
        result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, format!("]Q{}", decoder_result.get_symbology_modifier()));
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
         let module_size: f32 = self.module_size(&left_top_black, image);
         let mut top: i32 = left_top_black[1];
         let bottom: i32 = right_bottom_black[1];
         let mut left: i32 = left_top_black[0];
         let mut right: i32 = right_bottom_black[0];
        // Sanity check!
        if left >= right || top >= bottom {
            throw NotFoundException::get_not_found_instance();
        }
        if bottom - top != right - left {
            // Special case, where bottom-right module wasn't black so we found something else in the last row
            // Assume it's a square, so use height as the width
            right = left + (bottom - top);
            if right >= image.get_width() {
                // Abort if that would not make sense -- off image
                throw NotFoundException::get_not_found_instance();
            }
        }
         let matrix_width: i32 = Math::round((right - left + 1.0) / module_size);
         let matrix_height: i32 = Math::round((bottom - top + 1.0) / module_size);
        if matrix_width <= 0 || matrix_height <= 0 {
            throw NotFoundException::get_not_found_instance();
        }
        if matrix_height != matrix_width {
            // Only possibly decode square regions
            throw NotFoundException::get_not_found_instance();
        }
        // Push in the "border" by half the module width so that we start
        // sampling in the middle of the module. Just in case the image is a
        // little off, this will help recover.
         let nudge: i32 = (module_size / 2.0f) as i32;
        top += nudge;
        left += nudge;
        // But careful that this does not sample off the edge
        // "right" is the farthest-right valid pixel location -- right+1 is not necessarily
        // This is positive by how much the inner x loop below would be too large
         let nudged_too_far_right: i32 = left + ((matrix_width - 1.0) * module_size) as i32 - right;
        if nudged_too_far_right > 0 {
            if nudged_too_far_right > nudge {
                // Neither way fits; abort
                throw NotFoundException::get_not_found_instance();
            }
            left -= nudged_too_far_right;
        }
        // See logic above
         let nudged_too_far_down: i32 = top + ((matrix_height - 1.0) * module_size) as i32 - bottom;
        if nudged_too_far_down > 0 {
            if nudged_too_far_down > nudge {
                // Neither way fits; abort
                throw NotFoundException::get_not_found_instance();
            }
            top -= nudged_too_far_down;
        }
        // Now just read off the bits
         let bits: BitMatrix = BitMatrix::new(matrix_width, matrix_height);
         {
             let mut y: i32 = 0;
            while y < matrix_height {
                {
                     let i_offset: i32 = top + (y * module_size) as i32;
                     {
                         let mut x: i32 = 0;
                        while x < matrix_width {
                            {
                                if image.get(left + (x * module_size) as i32, i_offset) {
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

    fn  module_size( left_top_black: &Vec<i32>,  image: &BitMatrix) -> /*  throws NotFoundException */Result<f32, Rc<Exception>>   {
         let height: i32 = image.get_height();
         let width: i32 = image.get_width();
         let mut x: i32 = left_top_black[0];
         let mut y: i32 = left_top_black[1];
         let in_black: bool = true;
         let mut transitions: i32 = 0;
        while x < width && y < height {
            if in_black != image.get(x, y) {
                if transitions += 1 == 5 {
                    break;
                }
                in_black = !in_black;
            }
            x += 1;
            y += 1;
        }
        if x == width || y == height {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok((x - left_top_black[0]) / 7.0f);
    }
}

// NEW FILE: q_r_code_writer.rs
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

