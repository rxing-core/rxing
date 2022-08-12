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
// package com::google::zxing::multi::qrcode::detector;

/**
 * <p>Encapsulates logic that can detect one or more QR Codes in an image, even if the QR Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 * @author Hannes Erven
 */

 const EMPTY_DETECTOR_RESULTS: [Option<DetectorResult>; 0] = [None; 0];
pub struct MultiDetector {
    super: Detector;
}

impl MultiDetector {

    pub fn new( image: &BitMatrix) -> MultiDetector {
        super(image);
    }

    pub fn  detect_multi(&self,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<DetectorResult>, Rc<Exception>>   {
         let image: BitMatrix = get_image();
         let result_point_callback: ResultPointCallback =  if hints == null { null } else { hints.get(DecodeHintType::NEED_RESULT_POINT_CALLBACK) as ResultPointCallback };
         let finder: MultiFinderPatternFinder = MultiFinderPatternFinder::new(image, result_point_callback);
         let infos: Vec<FinderPatternInfo> = finder.find_multi(&hints);
        if infos.len() == 0 {
            throw NotFoundException::get_not_found_instance();
        }
         let result: List<DetectorResult> = ArrayList<>::new();
        for  let info: FinderPatternInfo in infos {
            let tryResult1 = 0;
            'try1: loop {
            {
                result.add(&process_finder_pattern_info(info));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( e: &ReaderException) {
                }  0 => break
            }

        }
        if result.is_empty() {
            return Ok(EMPTY_DETECTOR_RESULTS);
        } else {
            return Ok(result.to_array(EMPTY_DETECTOR_RESULTS));
        }
    }
}

