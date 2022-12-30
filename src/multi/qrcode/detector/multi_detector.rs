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

use crate::{
    common::BitMatrix,
    qrcode::detector::{Detector, QRCodeDetectorResult},
    DecodeHintType, DecodeHintValue, DecodingHintDictionary, Exceptions,
};

use super::MultiFinderPatternFinder;

/**
 * <p>Encapsulates logic that can detect one or more QR Codes in an image, even if the QR Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 * @author Hannes Erven
 */
pub struct MultiDetector(Detector);
impl MultiDetector {
    pub fn new(image: BitMatrix) -> Self {
        Self(Detector::new(image))
    }

    // private static final DetectorRXingResult[] EMPTY_DETECTOR_RESULTS = new DetectorRXingResult[0];

    pub fn detectMulti(
        &self,
        hints: &DecodingHintDictionary,
    ) -> Result<Vec<QRCodeDetectorResult>, Exceptions> {
        let image = self.0.getImage();
        let resultPointCallback = if let Some(DecodeHintValue::NeedResultPointCallback(cb)) =
            hints.get(&DecodeHintType::NEED_RESULT_POINT_CALLBACK)
        {
            Some(*cb)
        } else {
            None
        };
        let mut finder = MultiFinderPatternFinder::new(image, resultPointCallback);
        let infos = finder.findMulti(hints)?;

        if infos.len() == 0 {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        let mut result = Vec::new();
        for info in infos {
            if let Ok(potential) = self.0.processFinderPatternInfo(info) {
                result.push(potential);
            }
            // try {
            //   result.add(processFinderPatternInfo(info));
            // } catch (ReaderException e) {
            //   // ignore
            // }
        }

        Ok(result)
    }
}
