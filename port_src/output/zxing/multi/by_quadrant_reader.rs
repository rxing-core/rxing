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
 * This class attempts to decode a barcode from an image, not by scanning the whole image,
 * but by scanning subsets of the image. This is important when there may be multiple barcodes in
 * an image, and detecting a barcode may find parts of multiple barcode and fail to decode
 * (e.g. QR Codes). Instead this scans the four quadrants of the image -- and also the center
 * 'quadrant' to cover the case where a barcode is found in the center.
 *
 * @see GenericMultipleBarcodeReader
 */
#[derive(Reader)]
pub struct ByQuadrantReader {

     let delegate: Reader;
}

impl ByQuadrantReader {

    pub fn new( delegate: &Reader) -> ByQuadrantReader {
        let .delegate = delegate;
    }

    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode(image, null));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
         let width: i32 = image.get_width();
         let height: i32 = image.get_height();
         let half_width: i32 = width / 2;
         let half_height: i32 = height / 2;
        let tryResult1 = 0;
        'try1: loop {
        {
            // No need to call makeAbsolute as results will be relative to original top left here
            return Ok(self.delegate.decode(&image.crop(0, 0, half_width, half_height), &hints));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let result: Result = self.delegate.decode(&image.crop(half_width, 0, half_width, half_height), &hints);
            ::make_absolute(&result.get_result_points(), half_width, 0);
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let result: Result = self.delegate.decode(&image.crop(0, half_height, half_width, half_height), &hints);
            ::make_absolute(&result.get_result_points(), 0, half_height);
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let result: Result = self.delegate.decode(&image.crop(half_width, half_height, half_width, half_height), &hints);
            ::make_absolute(&result.get_result_points(), half_width, half_height);
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

         let quarter_width: i32 = half_width / 2;
         let quarter_height: i32 = half_height / 2;
         let center: BinaryBitmap = image.crop(quarter_width, quarter_height, half_width, half_height);
         let result: Result = self.delegate.decode(center, &hints);
        ::make_absolute(&result.get_result_points(), quarter_width, quarter_height);
        return Ok(result);
    }

    pub fn  reset(&self)   {
        self.delegate.reset();
    }

    fn  make_absolute( points: &Vec<ResultPoint>,  left_offset: i32,  top_offset: i32)   {
        if points != null {
             {
                 let mut i: i32 = 0;
                while i < points.len() {
                    {
                         let relative: ResultPoint = points[i];
                        if relative != null {
                            points[i] = ResultPoint::new(relative.get_x() + left_offset, relative.get_y() + top_offset);
                        }
                    }
                    i += 1;
                 }
             }

        }
    }
}

