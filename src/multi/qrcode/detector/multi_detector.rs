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
    common::{BitMatrix, Result}, qrcode::detector::{Detector, QRCodeDetectorResult}, DecodeHintType, DecodeHintValue, DecodeHints, DecodingHintDictionary, Exceptions
};

use super::MultiFinderPatternFinder;

/**
 * <p>Encapsulates logic that can detect one or more QR Codes in an image, even if the QR Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 * @author Hannes Erven
 */
pub struct MultiDetector<'a>(Detector<'a>);
impl<'a> MultiDetector<'_> {
    pub fn new(image: &'a BitMatrix) -> MultiDetector<'a> {
        MultiDetector(Detector::new(image))
    }

    // private static final DetectorRXingResult[] EMPTY_DETECTOR_RESULTS = new DetectorRXingResult[0];

    pub fn detectMulti(&self, hints: &DecodeHints) -> Result<Vec<QRCodeDetectorResult>> {
        let image = self.0.getImage();
        let resultPointCallback = if let Some(cb) =
            hints.NeedResultPointCallback.clone()
        {
            Some(cb.clone())
        } else {
            None
        };
        let mut finder = MultiFinderPatternFinder::new(image, resultPointCallback);
        let infos = finder.findMulti(hints)?;

        if infos.is_empty() {
            return Err(Exceptions::NOT_FOUND);
        }

        let mut result = Vec::new();
        for info in infos {
            if let Ok(potential) = self.0.processFinderPatternInfo(info) {
                result.push(potential);
            }
        }

        Ok(result)
    }
}
