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

use crate::common::Result;
use crate::DecodeHintValue;
use crate::Exceptions;
use crate::RXingResult;
use crate::Reader;
use crate::{BarcodeFormat, Binarizer};

use super::EAN13Reader;
use super::EAN8Reader;
use super::UPCAReader;
use super::UPCEReader;
use super::STAND_IN;
use super::{OneDReader, UPCEANReader};

/**
 * <p>A reader that can read all available UPC/EAN formats. If a caller wants to try to
 * read all such formats, it is most efficient to use this implementation rather than invoke
 * individual readers.</p>
 *
 * @author Sean Owen
 */
pub struct MultiFormatUPCEANReader {
    possible_formats: HashSet<BarcodeFormat>,
}

impl OneDReader for MultiFormatUPCEANReader {
    fn decode_row(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &DecodingHintDictionary,
    ) -> Result<RXingResult> {
        let Self {
            ref possible_formats,
        } = self;
        // Compute this location once and reuse it on multiple implementations
        let start_guard_pattern = STAND_IN.find_start_guard_pattern(row)?;

        if !possible_formats.is_empty() {
            if possible_formats.contains(&BarcodeFormat::EAN_13) {
                if let Ok(res) = self.try_decode_function(
                    &EAN13Reader::default(),
                    rowNumber,
                    row,
                    hints,
                    &start_guard_pattern,
                ) {
                    return Ok(res);
                }
            } else if possible_formats.contains(&BarcodeFormat::UPC_A) {
                if let Ok(res) = self.try_decode_function(
                    &UPCAReader::default(),
                    rowNumber,
                    row,
                    hints,
                    &start_guard_pattern,
                ) {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::EAN_8) {
                if let Ok(res) = self.try_decode_function(
                    &EAN8Reader::default(),
                    rowNumber,
                    row,
                    hints,
                    &start_guard_pattern,
                ) {
                    return Ok(res);
                }
            }
            if possible_formats.contains(&BarcodeFormat::UPC_E) {
                if let Ok(res) = self.try_decode_function(
                    &UPCEReader::default(),
                    rowNumber,
                    row,
                    hints,
                    &start_guard_pattern,
                ) {
                    return Ok(res);
                }
            }
        } else {
            if let Ok(res) = self.try_decode_function(
                &EAN13Reader::default(),
                rowNumber,
                row,
                hints,
                &start_guard_pattern,
            ) {
                return Ok(res);
            }
            if let Ok(res) = self.try_decode_function(
                &EAN8Reader::default(),
                rowNumber,
                row,
                hints,
                &start_guard_pattern,
            ) {
                return Ok(res);
            }
            if let Ok(res) = self.try_decode_function(
                &UPCEReader::default(),
                rowNumber,
                row,
                hints,
                &start_guard_pattern,
            ) {
                return Ok(res);
            }
        }

        Err(Exceptions::NOT_FOUND)
    }
}

impl MultiFormatUPCEANReader {
    pub fn new(hints: &DecodingHintDictionary) -> Self {
        let possible_formats = if let Some(DecodeHintValue::PossibleFormats(p)) =
            hints.get(&DecodeHintType::POSSIBLE_FORMATS)
        {
            p.clone()
        } else {
            HashSet::default()
        };

        Self { possible_formats }
    }

    fn try_decode_function<R: UPCEANReader>(
        &self,
        reader: &R,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &DecodingHintDictionary,
        startGuardPattern: &[usize; 2],
    ) -> Result<RXingResult> {
        let result = reader.decodeRowWithGuardRange(rowNumber, row, startGuardPattern, hints)?;
        // Special case: a 12-digit code encoded in UPC-A is identical to a "0"
        // followed by those 12 digits encoded as EAN-13. Each will recognize such a code,
        // UPC-A as a 12-digit string and EAN-13 as a 13-digit string starting with "0".
        // Individually these are correct and their readers will both read such a code
        // and correctly call it EAN-13, or UPC-A, respectively.
        //
        // In this case, if we've been looking for both types, we'd like to call it
        // a UPC-A code. But for efficiency we only run the EAN-13 decoder to also read
        // UPC-A. So we special case it here, and convert an EAN-13 result to a UPC-A
        // result if appropriate.
        //
        // But, don't return UPC-A if UPC-A was not a requested format!
        let ean13MayBeUPCA = result.getBarcodeFormat() == &BarcodeFormat::EAN_13
            && result.getText().starts_with('0');

        let canReturnUPCA = if let Some(DecodeHintValue::PossibleFormats(possibleFormats)) =
            hints.get(&DecodeHintType::POSSIBLE_FORMATS)
        {
            possibleFormats.contains(&BarcodeFormat::UPC_A)
        } else {
            true
        };

        if ean13MayBeUPCA && canReturnUPCA {
            // Transfer the metadata across
            let mut resultUPCA = RXingResult::new(
                &result.getText()[1..],
                result.getRawBytes().clone(),
                result.getPoints().clone(),
                BarcodeFormat::UPC_A,
            );
            resultUPCA.putAllMetadata(result.getRXingResultMetadata().clone());

            return Ok(resultUPCA);
        }

        Ok(result)
    }
}

use crate::DecodeHintType;
use crate::DecodingHintDictionary;
use crate::RXingResultMetadataType;
use crate::RXingResultMetadataValue;
use std::collections::{HashMap, HashSet};

impl Reader for MultiFormatUPCEANReader {
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
                    + if let Some(RXingResultMetadataValue::Orientation(or)) =
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
