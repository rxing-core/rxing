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

use std::collections::HashMap;

use crate::{
    common::{DecoderRXingResult, DetectorRXingResult},
    exceptions::Exceptions,
    BarcodeFormat, BinaryBitmap, DecodeHintType, DecodeHintValue, RXingResult,
    RXingResultMetadataType, RXingResultMetadataValue, Reader,
};

use super::{decoder, detector::Detector};

/**
 * This implementation can detect and decode Aztec codes in an image.
 *
 * @author David Olivier
 */
#[derive(Default)]
pub struct AztecReader;

impl Reader for AztecReader {
    /**
     * Locates and decodes a Data Matrix code in an image.
     *
     * @return a String representing the content encoded by the Data Matrix code
     * @throws NotFoundException if a Data Matrix code cannot be found
     * @throws FormatException if a Data Matrix code cannot be decoded
     */
    fn decode(&mut self, image: &mut BinaryBitmap) -> Result<RXingResult, Exceptions> {
        self.decode_with_hints(image, &HashMap::new())
    }

    fn decode_with_hints(
        &mut self,
        image: &mut BinaryBitmap,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> Result<RXingResult, Exceptions> {
        // let notFoundException = None;
        // let formatException = None;
        let mut detector = Detector::new(image.getBlackMatrix());

        //  try {

        let detectorRXingResult = if let Ok(det) = detector.detect(false) {
            det
        } else if let Ok(det) = detector.detect(true) {
            det
        } else {
            return Err(Exceptions::NotFoundException(None));
        };

        let points = detectorRXingResult.getPoints();
        let decoderRXingResult: DecoderRXingResult = decoder::decode(&detectorRXingResult)?;
        // } catch (NotFoundException e) {
        //   notFoundException = e;
        // } catch (FormatException e) {
        //   formatException = e;
        // }
        // if (decoderRXingResult == null) {
        // try {
        // let detectorRXingResult = detector.detect(true)?;
        // points = detectorRXingResult.getPoints();
        // decoderRXingResult = decoder::decode(&detectorRXingResult)?;
        // } catch (NotFoundException | FormatException e) {
        //   if (notFoundException != null) {
        //     throw notFoundException;
        //   }
        //   if (formatException != null) {
        //     throw formatException;
        //   }
        //   throw e;
        // }
        // }

        if let Some(DecodeHintValue::NeedResultPointCallback(cb)) =
            hints.get(&DecodeHintType::NEED_RESULT_POINT_CALLBACK)
        {
            // if let DecodeHintValue::NeedResultPointCallback(cb) = rpcb {
            for point in points {
                cb(point);
            }
            // }
        }

        let mut result = RXingResult::new_complex(
            decoderRXingResult.getText(),
            decoderRXingResult.getRawBytes().clone(),
            decoderRXingResult.getNumBits(),
            points.to_vec(),
            BarcodeFormat::AZTEC,
            chrono::Utc::now().timestamp_millis() as u128,
        );

        let byteSegments = decoderRXingResult.getByteSegments();
        if !byteSegments.is_empty() {
            result.putMetadata(
                RXingResultMetadataType::BYTE_SEGMENTS,
                RXingResultMetadataValue::ByteSegments(byteSegments.clone()),
            );
        }
        let ecLevel = decoderRXingResult.getECLevel();
        if !ecLevel.is_empty() {
            result.putMetadata(
                RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
                RXingResultMetadataValue::ErrorCorrectionLevel(ecLevel.to_owned()),
            );
        }
        result.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier(format!(
                "]z{}",
                decoderRXingResult.getSymbologyModifier()
            )),
        );

        Ok(result)
    }

    fn reset(&mut self) {
        // do nothing
    }
}
