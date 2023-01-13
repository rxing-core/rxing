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

use std::{cmp::Ordering, collections::HashMap};

use crate::{
    common::DetectorRXingResult,
    multi::MultipleBarcodeReader,
    qrcode::{
        decoder::{self, QRCodeDecoderMetaData},
        QRCodeReader,
    },
    BarcodeFormat, Exceptions, RXingResult, RXingResultMetadataType, RXingResultMetadataValue,
};

use super::detector::MultiDetector;

/**
 * This implementation can detect and decode multiple QR Codes in an image.
 *
 * @author Sean Owen
 * @author Hannes Erven
 */
#[derive(Default)]
pub struct QRCodeMultiReader(QRCodeReader);
impl MultipleBarcodeReader for QRCodeMultiReader {
    fn decode_multiple(
        &mut self,
        image: &mut crate::BinaryBitmap,
    ) -> Result<Vec<crate::RXingResult>, crate::Exceptions> {
        self.decode_multiple_with_hints(image, &HashMap::new())
    }

    fn decode_multiple_with_hints(
        &mut self,
        image: &mut crate::BinaryBitmap,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<Vec<crate::RXingResult>, crate::Exceptions> {
        let mut results = Vec::new();
        let detectorRXingResults = MultiDetector::new(image.getBlackMatrix()).detectMulti(hints)?;
        for detectorRXingResult in detectorRXingResults {
            let mut proc = || -> Result<(), Exceptions> {
                let decoderRXingResult = decoder::qrcode_decoder::decode_bitmatrix_with_hints(
                    detectorRXingResult.getBits(),
                    hints,
                )?;
                let mut points = detectorRXingResult.getPoints().to_vec();
                // If the code was mirrored: swap the bottom-left and the top-right points.
                if let Some(other) = decoderRXingResult.getOther() {
                    if other.is::<QRCodeDecoderMetaData>() {
                        (other
                            .downcast::<QRCodeDecoderMetaData>()
                            .expect("must downcast to QRCodeDecoderMetaData"))
                        .applyMirroredCorrection(&mut points);
                    }
                }
                // if (decoderRXingResult.getOther() instanceof QRCodeDecoderMetaData) {
                //   ((QRCodeDecoderMetaData) decoderRXingResult.getOther()).applyMirroredCorrection(points);
                // }
                let mut result = RXingResult::new(
                    decoderRXingResult.getText(),
                    decoderRXingResult.getRawBytes().clone(),
                    points.to_vec(),
                    BarcodeFormat::QR_CODE,
                );
                let byteSegments = decoderRXingResult.getByteSegments();
                // if (byteSegments != null) {
                result.putMetadata(
                    RXingResultMetadataType::BYTE_SEGMENTS,
                    RXingResultMetadataValue::ByteSegments(byteSegments.clone()),
                );
                // }
                let ecLevel = decoderRXingResult.getECLevel();
                // if (ecLevel != null) {
                result.putMetadata(
                    RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
                    RXingResultMetadataValue::ErrorCorrectionLevel(ecLevel.to_owned()),
                );
                // }
                if decoderRXingResult.hasStructuredAppend() {
                    result.putMetadata(
                        RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE,
                        RXingResultMetadataValue::StructuredAppendSequence(
                            decoderRXingResult.getStructuredAppendSequenceNumber(),
                        ),
                    );
                    result.putMetadata(
                        RXingResultMetadataType::STRUCTURED_APPEND_PARITY,
                        RXingResultMetadataValue::StructuredAppendParity(
                            decoderRXingResult.getStructuredAppendParity(),
                        ),
                    );
                }
                results.push(result);

                Ok(())
            };
            let output = proc();
            if output.is_ok() {
                continue;
            } else if let Err(Exceptions::ReaderException(_)) = output {
                // ignore and continue
                continue;
            } else {
                return Err(output.err().unwrap());
            }
        }

        results = Self::processStructuredAppend(results)?;

        Ok(results)
    }
}

impl QRCodeMultiReader {
    pub fn new() -> Self {
        Self(QRCodeReader::new())
    }

    fn processStructuredAppend(results: Vec<RXingResult>) -> Result<Vec<RXingResult>, Exceptions> {
        let mut newRXingResults = Vec::new();
        let mut saRXingResults = Vec::new();
        for result in &results {
            if result
                .getRXingResultMetadata()
                .contains_key(&RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE)
            {
                saRXingResults.push(result.clone());
            } else {
                newRXingResults.push(result.clone());
            }
        }
        if saRXingResults.is_empty() {
            return Ok(results);
        }

        // sort and concatenate the SA list items
        saRXingResults.sort_by(compareRXingResult);
        let mut newText = String::new();
        let mut newRawBytes = Vec::new(); //new ByteArrayOutputStream();
        let mut newByteSegment = Vec::new(); //new ByteArrayOutputStream();
        for saRXingResult in saRXingResults {
            newText.push_str(saRXingResult.getText());
            let saBytes = saRXingResult.getRawBytes();
            newRawBytes.extend_from_slice(saBytes);
            // newRawBytes.write(saBytes, 0, saBytes.len());

            if let Some(RXingResultMetadataValue::ByteSegments(byteSegments)) = saRXingResult
                .getRXingResultMetadata()
                .get(&RXingResultMetadataType::BYTE_SEGMENTS)
            {
                for segment in byteSegments {
                    // newByteSegment.write(segment, 0, segment.len());
                    newByteSegment.extend_from_slice(segment);
                }
            };
        }

        let mut newRXingResult =
            RXingResult::new(&newText, newRawBytes, Vec::new(), BarcodeFormat::QR_CODE);
        if !newByteSegment.is_empty() {
            newRXingResult.putMetadata(
                RXingResultMetadataType::BYTE_SEGMENTS,
                RXingResultMetadataValue::ByteSegments(vec![newByteSegment]),
            );
        }
        newRXingResults.push(newRXingResult);

        Ok(newRXingResults)
    }
}

fn compareRXingResult(a: &RXingResult, b: &RXingResult) -> Ordering {
    let aNumber = if let Some(RXingResultMetadataValue::StructuredAppendSequence(v)) = a
        .getRXingResultMetadata()
        .get(&RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE)
    {
        v
    } else {
        &-1
    };
    let bNumber = if let Some(RXingResultMetadataValue::StructuredAppendSequence(v)) = b
        .getRXingResultMetadata()
        .get(&RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE)
    {
        v
    } else {
        &-1
    };

    aNumber.cmp(bNumber)
}

#[cfg(test)]
#[cfg(feature = "image")]
mod multi_qr_code_test_case {
    /*
     * Copyright 2016 ZXing authors
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

    use std::{collections::HashSet, path::PathBuf, rc::Rc};

    use image;

    use crate::{
        common::HybridBinarizer, multi::MultipleBarcodeReader, BarcodeFormat, BinaryBitmap,
        BufferedImageLuminanceSource, RXingResult, RXingResultMetadataType,
        RXingResultMetadataValue,
    };

    use super::QRCodeMultiReader;

    /**
     * Tests {@link QRCodeMultiReader}.
     */

    #[test]
    fn testMultiQRCodes() {
        // Very basic test for now
        let mut testBase = PathBuf::from("test_resources/blackbox/multi-qrcode-1");

        testBase.push("1.png");

        let image = image::io::Reader::open(testBase)
            .expect("image must open")
            .decode()
            .expect("must decode");
        let source = BufferedImageLuminanceSource::new(image);
        let mut bitmap = BinaryBitmap::new(Rc::new(HybridBinarizer::new(Box::new(source))));

        let mut reader = QRCodeMultiReader::new();
        let results = reader.decode_multiple(&mut bitmap).expect("must decode");
        // assertNotNull(results);
        assert_eq!(4, results.len());

        let mut barcodeContents = HashSet::new();
        for result in results {
            barcodeContents.insert(result.getText().to_owned());
            assert_eq!(&BarcodeFormat::QR_CODE, result.getBarcodeFormat());
            assert!(!result.getRXingResultMetadata().is_empty());
        }
        let mut expectedContents = HashSet::new();
        expectedContents.insert(
            "You earned the class a 5 MINUTE DANCE PARTY!!  Awesome!  Way to go!  Let's boogie!"
                .to_owned(),
        );
        expectedContents.insert(
            "You earned the class 5 EXTRA MINUTES OF RECESS!!  Fabulous!!  Way to go!!".to_owned(),
        );
        expectedContents.insert(
        "You get to SIT AT MRS. SIGMON'S DESK FOR A DAY!!  Awesome!!  Way to go!! Guess I better clean up! :)".to_owned());
        expectedContents.insert(
            "You get to CREATE OUR JOURNAL PROMPT FOR THE DAY!  Yay!  Way to go!  ".to_owned(),
        );
        assert_eq!(expectedContents, barcodeContents);
    }

    #[test]
    fn testProcessStructuredAppend() {
        let mut sa1 = RXingResult::new("SA1", Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
        let mut sa2 = RXingResult::new("SA2", Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
        let mut sa3 = RXingResult::new("SA3", Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
        sa1.putMetadata(
            RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE,
            RXingResultMetadataValue::StructuredAppendSequence(2),
        );
        sa1.putMetadata(
            RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
            RXingResultMetadataValue::ErrorCorrectionLevel("L".to_owned()),
        );
        sa2.putMetadata(
            RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE,
            RXingResultMetadataValue::StructuredAppendSequence((1 << 4) + 2),
        );
        sa2.putMetadata(
            RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
            RXingResultMetadataValue::ErrorCorrectionLevel("L".to_owned()),
        );
        sa3.putMetadata(
            RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE,
            RXingResultMetadataValue::StructuredAppendSequence((2 << 4) + 2),
        );
        sa3.putMetadata(
            RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
            RXingResultMetadataValue::ErrorCorrectionLevel("L".to_owned()),
        );

        let mut nsa = RXingResult::new("NotSA", Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
        nsa.putMetadata(
            RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
            RXingResultMetadataValue::ErrorCorrectionLevel("L".to_owned()),
        );

        let inputs = vec![sa3, sa1, nsa, sa2];

        let results =
            QRCodeMultiReader::processStructuredAppend(inputs).expect("result must return");
        // assertNotNull(results);
        assert_eq!(2, results.len());

        let mut barcodeContents = HashSet::new();
        for result in results {
            barcodeContents.insert(result.getText().to_owned());
        }
        let mut expectedContents = HashSet::new();
        expectedContents.insert("SA1SA2SA3".to_owned());
        expectedContents.insert("NotSA".to_owned());
        assert_eq!(expectedContents, barcodeContents);
    }
}
