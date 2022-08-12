/*
 * Copyright 2010 ZXing authors
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
 * This implementation can detect and decode Aztec codes in an image.
 *
 * @author David Olivier
 */
#[derive(Reader)]
pub struct AztecReader {
}

impl AztecReader {

    /**
   * Locates and decodes a Data Matrix code in an image.
   *
   * @return a String representing the content encoded by the Data Matrix code
   * @throws NotFoundException if a Data Matrix code cannot be found
   * @throws FormatException if a Data Matrix code cannot be decoded
   */
    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode(image, null));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
         let not_found_exception: NotFoundException = null;
         let format_exception: FormatException = null;
         let detector: Detector = Detector::new(&image.get_black_matrix());
         let mut points: Vec<ResultPoint> = null;
         let decoder_result: DecoderResult = null;
        let tryResult1 = 0;
        'try1: loop {
        {
             let detector_result: AztecDetectorResult = detector.detect(false);
            points = detector_result.get_points();
            decoder_result = Decoder::new().decode(detector_result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &NotFoundException) {
                not_found_exception = e;
            } catch ( e: &FormatException) {
                format_exception = e;
            }  0 => break
        }

        if decoder_result == null {
            let tryResult1 = 0;
            'try1: loop {
            {
                 let detector_result: AztecDetectorResult = detector.detect(true);
                points = detector_result.get_points();
                decoder_result = Decoder::new().decode(detector_result);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( e: &NotFoundExceptionFormatException | ) {
                    if not_found_exception != null {
                        throw not_found_exception;
                    }
                    if format_exception != null {
                        throw format_exception;
                    }
                    throw e;
                }  0 => break
            }

        }
        if hints != null {
             let rpcb: ResultPointCallback = hints.get(DecodeHintType::NEED_RESULT_POINT_CALLBACK) as ResultPointCallback;
            if rpcb != null {
                for  let point: ResultPoint in points {
                    rpcb.found_possible_result_point(point);
                }
            }
        }
         let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), &decoder_result.get_num_bits(), points, BarcodeFormat::AZTEC, &System::current_time_millis());
         let byte_segments: List<Vec<i8>> = decoder_result.get_byte_segments();
        if byte_segments != null {
            result.put_metadata(ResultMetadataType::BYTE_SEGMENTS, &byte_segments);
        }
         let ec_level: String = decoder_result.get_e_c_level();
        if ec_level != null {
            result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &ec_level);
        }
        result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, format!("]z{}", decoder_result.get_symbology_modifier()));
        return Ok(result);
    }

    pub fn  reset(&self)   {
    // do nothing
    }
}

