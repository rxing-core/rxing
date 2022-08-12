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
// package com::google::zxing::pdf417;

/**
 * This implementation can detect and decode PDF417 codes in an image.
 *
 * @author Guenther Grau
 */

 const EMPTY_RESULT_ARRAY: [Option<Result>; 0] = [None; 0];
#[derive(Reader, MultipleBarcodeReader)]
pub struct PDF417Reader {
}

impl PDF417Reader {

    /**
   * Locates and decodes a PDF417 code in an image.
   *
   * @return a String representing the content encoded by the PDF417 code
   * @throws NotFoundException if a PDF417 code cannot be found,
   * @throws FormatException if a PDF417 cannot be decoded
   */
    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Result, Rc<Exception>>   {
        return Ok(::decode(image, null));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Result, Rc<Exception>>   {
         let result: Vec<Result> = ::decode(image, &hints, false);
        if result.len() == 0 || result[0] == null {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(result[0]);
    }

    pub fn  decode_multiple(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
        return Ok(self.decode_multiple(image, null));
    }

    pub fn  decode_multiple(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(::decode(image, &hints, true));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &FormatExceptionChecksumException | ) {
                throw NotFoundException::get_not_found_instance();
            }  0 => break
        }

    }

    fn  decode( image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>,  multiple: bool) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Vec<Result>, Rc<Exception>>   {
         let results: List<Result> = ArrayList<>::new();
         let detector_result: PDF417DetectorResult = Detector::detect(image, &hints, multiple);
        for  let points: Vec<ResultPoint> in detector_result.get_points() {
             let decoder_result: DecoderResult = PDF417ScanningDecoder::decode(&detector_result.get_bits(), points[4], points[5], points[6], points[7], &::get_min_codeword_width(points), &::get_max_codeword_width(points));
             let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), points, BarcodeFormat::PDF_417);
            result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &decoder_result.get_e_c_level());
             let pdf417_result_metadata: PDF417ResultMetadata = decoder_result.get_other() as PDF417ResultMetadata;
            if pdf417_result_metadata != null {
                result.put_metadata(ResultMetadataType::PDF417_EXTRA_METADATA, pdf417_result_metadata);
            }
            result.put_metadata(ResultMetadataType::ORIENTATION, &detector_result.get_rotation());
            result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, format!("]L{}", decoder_result.get_symbology_modifier()));
            results.add(result);
        }
        return Ok(results.to_array(EMPTY_RESULT_ARRAY));
    }

    fn  get_max_width( p1: &ResultPoint,  p2: &ResultPoint) -> i32  {
        if p1 == null || p2 == null {
            return 0;
        }
        return Math::abs(p1.get_x() - p2.get_x()) as i32;
    }

    fn  get_min_width( p1: &ResultPoint,  p2: &ResultPoint) -> i32  {
        if p1 == null || p2 == null {
            return Integer::MAX_VALUE;
        }
        return Math::abs(p1.get_x() - p2.get_x()) as i32;
    }

    fn  get_max_codeword_width( p: &Vec<ResultPoint>) -> i32  {
        return Math::max(&Math::max(&::get_max_width(p[0], p[4]), ::get_max_width(p[6], p[2]) * PDF417Common.MODULES_IN_CODEWORD / PDF417Common.MODULES_IN_STOP_PATTERN), &Math::max(&::get_max_width(p[1], p[5]), ::get_max_width(p[7], p[3]) * PDF417Common.MODULES_IN_CODEWORD / PDF417Common.MODULES_IN_STOP_PATTERN));
    }

    fn  get_min_codeword_width( p: &Vec<ResultPoint>) -> i32  {
        return Math::min(&Math::min(&::get_min_width(p[0], p[4]), ::get_min_width(p[6], p[2]) * PDF417Common.MODULES_IN_CODEWORD / PDF417Common.MODULES_IN_STOP_PATTERN), &Math::min(&::get_min_width(p[1], p[5]), ::get_min_width(p[7], p[3]) * PDF417Common.MODULES_IN_CODEWORD / PDF417Common.MODULES_IN_STOP_PATTERN));
    }

    pub fn  reset(&self)   {
    // nothing needs to be reset
    }
}

