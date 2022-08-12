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
// package com::google::zxing::oned;

/**
 * <p>Encapsulates functionality and implementation that is common to one-dimensional barcodes.</p>
 *
 * @author dsbnatut@gmail.com (Kazuki Nishiura)
 */

 const NUMERIC: Pattern = Pattern::compile("[0-9]+");
#[derive(Writer)]
pub struct OneDimensionalCodeWriter {
}

impl OneDimensionalCodeWriter {

    /**
   * Encode the contents to boolean array expression of one-dimensional barcode.
   * Start code and end code should be included in result, and side margins should not be included.
   *
   * @param contents barcode contents to encode
   * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
   */
    pub fn  encode(&self,  contents: &String) -> Vec<bool> ;

    /**
   * Can be overwritten if the encode requires to read the hints map. Otherwise it defaults to {@code encode}.
   * @param contents barcode contents to encode
   * @param hints encoding hints
   * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
   */
    pub fn  encode(&self,  contents: &String,  hints: &Map<EncodeHintType, ?>) -> Vec<bool>  {
        return self.encode(&contents);
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> BitMatrix  {
        return self.encode(&contents, format, width, height, null);
    }

    /**
   * Encode the contents following specified format.
   * {@code width} and {@code height} are required size. This method may return bigger size
   * {@code BitMatrix} when specified size is too small. The user can set both {@code width} and
   * {@code height} to zero to get minimum size barcode. If negative value is set to {@code width}
   * or {@code height}, {@code IllegalArgumentException} is thrown.
   */
    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> BitMatrix  {
        if contents.is_empty() {
            throw IllegalArgumentException::new("Found empty contents");
        }
        if width < 0 || height < 0 {
            throw IllegalArgumentException::new(format!("Negative size is not allowed. Input: {}x{}", width, height));
        }
         let supported_formats: Collection<BarcodeFormat> = self.get_supported_write_formats();
        if supported_formats != null && !supported_formats.contains(format) {
            throw IllegalArgumentException::new(format!("Can only encode {}, but got {}", supported_formats, format));
        }
         let sides_margin: i32 = self.get_default_margin();
        if hints != null && hints.contains_key(EncodeHintType::MARGIN) {
            sides_margin = Integer::parse_int(&hints.get(EncodeHintType::MARGIN).to_string());
        }
         let code: Vec<bool> = self.encode(&contents, &hints);
        return ::render_result(&code, width, height, sides_margin);
    }

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return null;
    }

    /**
   * @return a byte array of horizontal pixels (0 = white, 1 = black)
   */
    fn  render_result( code: &Vec<bool>,  width: i32,  height: i32,  sides_margin: i32) -> BitMatrix  {
         let input_width: i32 = code.len();
        // Add quiet zone on both sides.
         let full_width: i32 = input_width + sides_margin;
         let output_width: i32 = Math::max(width, full_width);
         let output_height: i32 = Math::max(1, height);
         let multiple: i32 = output_width / full_width;
         let left_padding: i32 = (output_width - (input_width * multiple)) / 2;
         let output: BitMatrix = BitMatrix::new(output_width, output_height);
         {
             let input_x: i32 = 0, let output_x: i32 = left_padding;
            while input_x < input_width {
                {
                    if code[input_x] {
                        output.set_region(output_x, 0, multiple, output_height);
                    }
                }
                input_x += 1;
                output_x += multiple;
             }
         }

        return output;
    }

    /**
   * @param contents string to check for numeric characters
   * @throws IllegalArgumentException if input contains characters other than digits 0-9.
   */
    pub fn  check_numeric( contents: &String)   {
        if !NUMERIC::matcher(&contents)::matches() {
            throw IllegalArgumentException::new("Input should only contain digits 0-9");
        }
    }

    /**
   * @param target encode black/white pattern into this array
   * @param pos position to start encoding at in {@code target}
   * @param pattern lengths of black/white runs to encode
   * @param startColor starting color - false for white, true for black
   * @return the number of elements added to target.
   */
    pub fn  append_pattern( target: &Vec<bool>,  pos: i32,  pattern: &Vec<i32>,  start_color: bool) -> i32  {
         let mut color: bool = start_color;
         let num_added: i32 = 0;
        for  let len: i32 in pattern {
             {
                 let mut j: i32 = 0;
                while j < len {
                    {
                        target[pos += 1 !!!check!!! post increment] = color;
                    }
                    j += 1;
                 }
             }

            num_added += len;
            // flip color after each segment
            color = !color;
        }
        return num_added;
    }

    pub fn  get_default_margin(&self) -> i32  {
        // This seems like a decent idea for a default for all formats.
        return 10;
    }
}

