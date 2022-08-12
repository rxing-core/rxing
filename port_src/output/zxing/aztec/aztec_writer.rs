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
// package com::google::zxing::aztec;

/**
 * Renders an Aztec code as a {@link BitMatrix}.
 */
#[derive(Writer)]
pub struct AztecWriter {
}

impl AztecWriter {

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> BitMatrix  {
        return ::encode(&contents, format, width, height, null);
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> BitMatrix  {
        // Do not add any ECI code by default
         let mut charset: Charset = null;
         let ecc_percent: i32 = Encoder::DEFAULT_EC_PERCENT;
         let mut layers: i32 = Encoder::DEFAULT_AZTEC_LAYERS;
        if hints != null {
            if hints.contains_key(EncodeHintType::CHARACTER_SET) {
                charset = Charset::for_name(&hints.get(EncodeHintType::CHARACTER_SET).to_string());
            }
            if hints.contains_key(EncodeHintType::ERROR_CORRECTION) {
                ecc_percent = Integer::parse_int(&hints.get(EncodeHintType::ERROR_CORRECTION).to_string());
            }
            if hints.contains_key(EncodeHintType::AZTEC_LAYERS) {
                layers = Integer::parse_int(&hints.get(EncodeHintType::AZTEC_LAYERS).to_string());
            }
        }
        return ::encode(&contents, format, width, height, &charset, ecc_percent, layers);
    }

    fn  encode( contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  charset: &Charset,  ecc_percent: i32,  layers: i32) -> BitMatrix  {
        if format != BarcodeFormat::AZTEC {
            throw IllegalArgumentException::new(format!("Can only encode AZTEC, but got {}", format));
        }
         let aztec: AztecCode = Encoder::encode(&contents, ecc_percent, layers, &charset);
        return ::render_result(aztec, width, height);
    }

    fn  render_result( code: &AztecCode,  width: i32,  height: i32) -> BitMatrix  {
         let input: BitMatrix = code.get_matrix();
        if input == null {
            throw IllegalStateException::new();
        }
         let input_width: i32 = input.get_width();
         let input_height: i32 = input.get_height();
         let output_width: i32 = Math::max(width, input_width);
         let output_height: i32 = Math::max(height, input_height);
         let multiple: i32 = Math::min(output_width / input_width, output_height / input_height);
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
                                if input.get(input_x, input_y) {
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

