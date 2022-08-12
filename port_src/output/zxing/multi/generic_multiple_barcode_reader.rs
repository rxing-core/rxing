/*
 * Copyright 2009 ZXing authors
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
// package com::google::zxing::multi;

/**
 * <p>Attempts to locate multiple barcodes in an image by repeatedly decoding portion of the image.
 * After one barcode is found, the areas left, above, right and below the barcode's
 * {@link ResultPoint}s are scanned, recursively.</p>
 *
 * <p>A caller may want to also employ {@link ByQuadrantReader} when attempting to find multiple
 * 2D barcodes, like QR Codes, in an image, where the presence of multiple barcodes might prevent
 * detecting any one of them.</p>
 *
 * <p>That is, instead of passing a {@link Reader} a caller might pass
 * {@code new ByQuadrantReader(reader)}.</p>
 *
 * @author Sean Owen
 */

 const MIN_DIMENSION_TO_RECUR: i32 = 100;

 const MAX_DEPTH: i32 = 4;

 const EMPTY_RESULT_ARRAY: [Option<Result>; 0] = [None; 0];
#[derive(MultipleBarcodeReader)]
pub struct GenericMultipleBarcodeReader {

     let delegate: Reader;
}

impl GenericMultipleBarcodeReader {

    pub fn new( delegate: &Reader) -> GenericMultipleBarcodeReader {
        let .delegate = delegate;
    }

    pub fn  decode_multiple(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
        return Ok(self.decode_multiple(image, null));
    }

    pub fn  decode_multiple(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
         let results: List<Result> = ArrayList<>::new();
        self.do_decode_multiple(image, &hints, &results, 0, 0, 0);
        if results.is_empty() {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(results.to_array(EMPTY_RESULT_ARRAY));
    }

    fn  do_decode_multiple(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>,  results: &List<Result>,  x_offset: i32,  y_offset: i32,  current_depth: i32)   {
        if current_depth > MAX_DEPTH {
            return;
        }
         let mut result: Result;
        let tryResult1 = 0;
        'try1: loop {
        {
            result = self.delegate.decode(image, &hints);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &ReaderException) {
                return;
            }  0 => break
        }

         let already_found: bool = false;
        for  let existing_result: Result in results {
            if existing_result.get_text().equals(&result.get_text()) {
                already_found = true;
                break;
            }
        }
        if !already_found {
            results.add(&::translate_result_points(result, x_offset, y_offset));
        }
         let result_points: Vec<ResultPoint> = result.get_result_points();
        if result_points == null || result_points.len() == 0 {
            return;
        }
         let width: i32 = image.get_width();
         let height: i32 = image.get_height();
         let min_x: f32 = width;
         let min_y: f32 = height;
         let max_x: f32 = 0.0f;
         let max_y: f32 = 0.0f;
        for  let point: ResultPoint in result_points {
            if point == null {
                continue;
            }
             let x: f32 = point.get_x();
             let y: f32 = point.get_y();
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
        // Decode left of barcode
        if min_x > MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(&image.crop(0, 0, min_x as i32, height), &hints, &results, x_offset, y_offset, current_depth + 1);
        }
        // Decode above barcode
        if min_y > MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(&image.crop(0, 0, width, min_y as i32), &hints, &results, x_offset, y_offset, current_depth + 1);
        }
        // Decode right of barcode
        if max_x < width - MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(&image.crop(max_x as i32, 0, width - max_x as i32, height), &hints, &results, x_offset + max_x as i32, y_offset, current_depth + 1);
        }
        // Decode below barcode
        if max_y < height - MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(&image.crop(0, max_y as i32, width, height - max_y as i32), &hints, &results, x_offset, y_offset + max_y as i32, current_depth + 1);
        }
    }

    fn  translate_result_points( result: &Result,  x_offset: i32,  y_offset: i32) -> Result  {
         let old_result_points: Vec<ResultPoint> = result.get_result_points();
        if old_result_points == null {
            return result;
        }
         let new_result_points: [Option<ResultPoint>; old_result_points.len()] = [None; old_result_points.len()];
         {
             let mut i: i32 = 0;
            while i < old_result_points.len() {
                {
                     let old_point: ResultPoint = old_result_points[i];
                    if old_point != null {
                        new_result_points[i] = ResultPoint::new(old_point.get_x() + x_offset, old_point.get_y() + y_offset);
                    }
                }
                i += 1;
             }
         }

         let new_result: Result = Result::new(&result.get_text(), &result.get_raw_bytes(), &result.get_num_bits(), new_result_points, &result.get_barcode_format(), &result.get_timestamp());
        new_result.put_all_metadata(&result.get_result_metadata());
        return new_result;
    }
}

