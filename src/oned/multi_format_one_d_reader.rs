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
use crate::common::Result;
use crate::DecodeHintValue;
use crate::Exceptions;
use crate::{BarcodeFormat, Binarizer, RXingResult};

/**
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
#[derive(Default)]
pub struct MultiFormatOneDReader {
    internal_hints: DecodingHintDictionary,
    possible_formats: HashSet<BarcodeFormat>,
    use_code_39_check_digit: bool,
    rss_14_reader: RSS14Reader,
    rss_expanded_reader: RSSExpandedReader
}
impl OneDReader for MultiFormatOneDReader {
    fn decode_row(
        &mut self,
        row_number: u32,
        row: &crate::common::BitArray,
        hints: &DecodingHintDictionary,
    ) -> Result<RXingResult> {
        let Self {
            possible_formats,
            use_code_39_check_digit,
            internal_hints,
            rss_14_reader,
            rss_expanded_reader,
        } = self;

        if !possible_formats.is_empty() {
            if possible_formats.contains(&BarcodeFormat::EAN_13)
                || possible_formats.contains(&BarcodeFormat::UPC_A)
                || possible_formats.contains(&BarcodeFormat::EAN_8)
                || possible_formats.contains(&BarcodeFormat::UPC_E)
            {
                if let Ok(res) =
                    MultiFormatUPCEANReader::new(internal_hints).decode_row(row_number, row, hints)
                {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::CODE_39) {
                if let Ok(res) = Code39Reader::with_use_check_digit(*use_code_39_check_digit)
                    .decode_row(row_number, row, hints)
                {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::CODE_93) {
                if let Ok(res) = Code93Reader::default().decode_row(row_number, row, hints) {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::CODE_128) {
                if let Ok(res) = Code128Reader::default().decode_row(row_number, row, hints) {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::ITF) {
                if let Ok(res) = ITFReader::default().decode_row(row_number, row, hints) {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::CODABAR) {
                if let Ok(res) = CodaBarReader::default().decode_row(row_number, row, hints) {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::RSS_14) {
                if let Ok(res) = rss_14_reader.decode_row(row_number, row, hints) {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::RSS_EXPANDED) {
                if let Ok(res) = rss_expanded_reader.decode_row(row_number, row, hints) {
                    return Ok(res);
                }
            }
        } else {
            if let Ok(res) =
                MultiFormatUPCEANReader::new(internal_hints).decode_row(row_number, row, hints)
            {
                return Ok(res);
            }
            if let Ok(res) = Code39Reader::with_use_check_digit(*use_code_39_check_digit)
                .decode_row(row_number, row, hints)
            {
                return Ok(res);
            }
            if let Ok(res) = CodaBarReader::default().decode_row(row_number, row, hints) {
                return Ok(res);
            }
            if let Ok(res) = Code93Reader::default().decode_row(row_number, row, hints) {
                return Ok(res);
            }
            if let Ok(res) = Code128Reader::default().decode_row(row_number, row, hints) {
                return Ok(res);
            }
            if let Ok(res) = ITFReader::default().decode_row(row_number, row, hints) {
                return Ok(res);
            }
            if let Ok(res) = rss_14_reader.decode_row(row_number, row, hints) {
                return Ok(res);
            }
            if let Ok(res) = rss_expanded_reader.decode_row(row_number, row, hints) {
                return Ok(res);
            }
        }

        Err(Exceptions::NOT_FOUND)
    }
}
impl MultiFormatOneDReader {
    pub fn new(hints: &DecodingHintDictionary) -> Self {
        let use_code_39_check_digit = matches!(
            hints.get(&DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT),
            Some(DecodeHintValue::AssumeCode39CheckDigit(true))
        );
        let possible_formats = if let Some(DecodeHintValue::PossibleFormats(p)) =
            hints.get(&DecodeHintType::POSSIBLE_FORMATS)
        {
            p.clone()
        } else {
            HashSet::new()
        };

        Self {
            possible_formats,
            use_code_39_check_digit,
            rss_14_reader: RSS14Reader::default(),
            internal_hints: hints.clone(),
            rss_expanded_reader: RSSExpandedReader::default(),
        }
    }
}

use crate::DecodeHintType;
use crate::DecodingHintDictionary;
use crate::RXingResultMetadataType;
use crate::RXingResultMetadataValue;
use crate::Reader;
use std::collections::{HashMap, HashSet};

impl Reader for MultiFormatOneDReader {
    fn decode<B: Binarizer>(&mut self, image: &mut crate::BinaryBitmap<B>) -> Result<RXingResult> {
        self.decode_with_hints(image, &HashMap::new())
    }

    // Note that we don't try rotation without the try harder flag, even if rotation was supported.
    fn decode_with_hints<B: Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodingHintDictionary,
    ) -> Result<RXingResult> {
        let first_try = self._do_decode(image, hints);
        if first_try.is_ok() {
            return first_try;
        }

        let tryHarder = matches!(
            hints.get(&DecodeHintType::TRY_HARDER),
            Some(DecodeHintValue::TryHarder(true))
        );
        if tryHarder && image.is_rotate_supported() {
            let mut rotatedImage = image.rotate_counter_clockwise();
            let mut result = self._do_decode(&mut rotatedImage, hints)?;
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
            let height = rotatedImage.get_height();
            let total_points = result.getPoints().len();
            let points = result.getPointsMut();
            for point in points.iter_mut().take(total_points) {
                std::mem::swap(&mut point.x, &mut point.y);
                point.x = height as f32 - point.x - 1.0;
            }

            Ok(result)
        } else {
            Err(Exceptions::NOT_FOUND)
        }
    }
}
