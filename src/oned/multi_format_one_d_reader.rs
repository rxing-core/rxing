/*
 * Copyright 2008 ZXing authors
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

use super::rss::expanded::RSSExpandedReader;
use super::rss::RSS14Reader;
use super::CodaBarReader;
use super::Code128Reader;
use super::Code39Reader;
use super::Code93Reader;
use super::ITFReader;
use super::MultiFormatUPCEANReader;
use super::OneDReader;
use crate::BarcodeFormat;
use crate::DecodeHintValue;
use crate::Exceptions;

/**
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
pub struct MultiFormatOneDReader(Vec<Box<dyn OneDReader>>);
impl OneDReader for MultiFormatOneDReader {
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        for reader in self.0.iter_mut() {
            // for (OneDReader reader : readers) {
            // try {
            if let Ok(res) = reader.decodeRow(rowNumber, row, hints) {
                return Ok(res);
            }
            // } catch (ReaderException re) {
            // continue
            // }
        }

        return Err(Exceptions::NotFoundException("".to_owned()));
    }
}
impl MultiFormatOneDReader {
    // private static final OneDReader[] EMPTY_ONED_ARRAY = new OneDReader[0];

    // private final OneDReader[] readers;

    pub fn new(hints: &DecodingHintDictionary) -> Self {
        let useCode39CheckDigit = hints.contains_key(&DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT);
        let mut readers: Vec<Box<dyn OneDReader>> = Vec::new();
        if let Some(DecodeHintValue::PossibleFormats(possibleFormats)) =
            hints.get(&DecodeHintType::POSSIBLE_FORMATS)
        {
            if possibleFormats.contains(&BarcodeFormat::EAN_13)
                || possibleFormats.contains(&BarcodeFormat::UPC_A)
                || possibleFormats.contains(&BarcodeFormat::EAN_8)
                || possibleFormats.contains(&BarcodeFormat::UPC_E)
            {
                readers.push(Box::new(MultiFormatUPCEANReader::new(hints)));
            }
            if possibleFormats.contains(&BarcodeFormat::CODE_39) {
                readers.push(Box::new(Code39Reader::with_use_check_digit(
                    useCode39CheckDigit,
                )));
            }
            if possibleFormats.contains(&BarcodeFormat::CODE_93) {
                readers.push(Box::new(Code93Reader::new()));
            }
            if possibleFormats.contains(&BarcodeFormat::CODE_128) {
                readers.push(Box::new(Code128Reader {}));
            }
            if possibleFormats.contains(&BarcodeFormat::ITF) {
                readers.push(Box::new(ITFReader::default()));
            }
            if possibleFormats.contains(&BarcodeFormat::CODABAR) {
                readers.push(Box::new(CodaBarReader::new()));
            }
            if possibleFormats.contains(&BarcodeFormat::RSS_14) {
                readers.push(Box::new(RSS14Reader::new()));
            }
            if possibleFormats.contains(&BarcodeFormat::RSS_EXPANDED) {
                readers.push(Box::new(RSSExpandedReader::new()));
            }
        }
        if readers.is_empty() {
            readers.push(Box::new(MultiFormatUPCEANReader::new(hints)));
            readers.push(Box::new(Code39Reader::new()));
            readers.push(Box::new(CodaBarReader::new()));
            readers.push(Box::new(Code93Reader::new()));
            readers.push(Box::new(Code128Reader {}));
            readers.push(Box::new(ITFReader::default()));
            readers.push(Box::new(RSS14Reader::new()));
            readers.push(Box::new(RSSExpandedReader::new()));
        }

        Self(readers)
    }
}

use crate::result_point::ResultPoint;
use crate::DecodeHintType;
use crate::DecodingHintDictionary;
use crate::RXingResultMetadataType;
use crate::RXingResultMetadataValue;
use crate::RXingResultPoint;
use crate::Reader;
use std::collections::HashMap;

impl Reader for MultiFormatOneDReader {
    fn decode(
        &mut self,
        image: &mut crate::BinaryBitmap,
    ) -> Result<crate::RXingResult, Exceptions> {
        self.decode_with_hints(image, &HashMap::new())
    }

    // Note that we don't try rotation without the try harder flag, even if rotation was supported.
    fn decode_with_hints(
        &mut self,
        image: &mut crate::BinaryBitmap,
        hints: &DecodingHintDictionary,
    ) -> Result<crate::RXingResult, Exceptions> {
        if let Ok(res) = self.doDecode(image, hints) {
            Ok(res)
        } else {
            let tryHarder = hints.contains_key(&DecodeHintType::TRY_HARDER);
            if tryHarder && image.isRotateSupported() {
                let mut rotatedImage = image.rotateCounterClockwise();
                let mut result = self.doDecode(&mut rotatedImage, hints)?;
                // Record that we found it rotated 90 degrees CCW / 270 degrees CW
                let metadata = result.getRXingResultMetadata();
                let mut orientation = 270;
                if metadata.contains_key(&RXingResultMetadataType::ORIENTATION) {
                    // But if we found it reversed in doDecode(), add in that result here:
                    orientation = (orientation
                        + if let Some(crate::RXingResultMetadataValue::Orientation(or)) =
                            metadata.get(&RXingResultMetadataType::ORIENTATION)
                        {
                            *or
                        } else {
                            0
                        })
                        % 360;
                }
                result.putMetadata(
                    RXingResultMetadataType::ORIENTATION,
                    RXingResultMetadataValue::Orientation(orientation),
                );
                // Update result points
                // let points = result.getRXingResultPoints();
                // if points != null {
                let height = rotatedImage.getHeight();
                // for point in result.getRXingResultPointsMut().iter_mut() {
                let total_points = result.getRXingResultPoints().len();
                let points = result.getRXingResultPointsMut();
                for i in 0..total_points {
                    // for (int i = 0; i < points.length; i++) {
                    points[i] = RXingResultPoint::new(
                        height as f32 - points[i].getY() - 1.0,
                        points[i].getX(),
                    );
                }
                // }

                Ok(result)
            } else {
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
        }
    }
    fn reset(&mut self) {
        for reader in self.0.iter_mut() {
            // for (Reader reader : readers) {
            reader.reset();
        }
    }
}
