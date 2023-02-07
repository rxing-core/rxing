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
#[derive(Default)]
pub struct MultiFormatOneDReader(Vec<Box<dyn OneDReader>>);
impl OneDReader for MultiFormatOneDReader {
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        for reader in self.0.iter_mut() {
            if let Ok(res) = reader.decodeRow(rowNumber, row, hints) {
                return Ok(res);
            }
        }

        Err(Exceptions::NotFoundException(None))
    }
}
impl MultiFormatOneDReader {
    pub fn new(hints: &DecodingHintDictionary) -> Self {
        let useCode39CheckDigit = matches!(
            hints.get(&DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT),
            Some(DecodeHintValue::AssumeCode39CheckDigit(true))
        );
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
                readers.push(Box::<Code93Reader>::default());
            }
            if possibleFormats.contains(&BarcodeFormat::CODE_128) {
                readers.push(Box::<Code128Reader>::default());
            }
            if possibleFormats.contains(&BarcodeFormat::ITF) {
                readers.push(Box::<ITFReader>::default());
            }
            if possibleFormats.contains(&BarcodeFormat::CODABAR) {
                readers.push(Box::<CodaBarReader>::default());
            }
            if possibleFormats.contains(&BarcodeFormat::RSS_14) {
                readers.push(Box::<RSS14Reader>::default());
            }
            if possibleFormats.contains(&BarcodeFormat::RSS_EXPANDED) {
                readers.push(Box::<RSSExpandedReader>::default());
            }
        }
        if readers.is_empty() {
            readers.push(Box::new(MultiFormatUPCEANReader::new(hints)));
            readers.push(Box::<Code39Reader>::default());
            readers.push(Box::<CodaBarReader>::default());
            readers.push(Box::<Code93Reader>::default());
            readers.push(Box::<Code128Reader>::default());
            readers.push(Box::<ITFReader>::default());
            readers.push(Box::<RSS14Reader>::default());
            readers.push(Box::<RSSExpandedReader>::default());
        }

        Self(readers)
    }
}

use crate::DecodeHintType;
use crate::DecodingHintDictionary;
use crate::RXingResultMetadataType;
use crate::RXingResultMetadataValue;
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
        let first_try = self.doDecode(image, hints);
        if first_try.is_ok() {
            return first_try;
        }

        let tryHarder = matches!(
            hints.get(&DecodeHintType::TRY_HARDER),
            Some(DecodeHintValue::TryHarder(true))
        );
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
            let height = rotatedImage.getHeight();
            let total_points = result.getRXingResultPoints().len();
            let points = result.getRXingResultPointsMut();
            for point in points.iter_mut().take(total_points) {
                std::mem::swap(&mut point.x, &mut point.y);
                point.x = height as f32 - point.x - 1.0;
            }

            Ok(result)
        } else {
            Err(Exceptions::NotFoundException(None))
        }
    }

    fn reset(&mut self) {
        for reader in self.0.iter_mut() {
            reader.reset();
        }
    }
}
