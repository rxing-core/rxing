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
use crate::BarcodeFormat;
use crate::DecodeHintValue;
use crate::Exceptions;
use crate::RXingResult;
use crate::Reader;

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
pub struct MultiFormatUPCEANReader(Vec<Box<dyn UPCEANReader>>);

impl MultiFormatUPCEANReader {
    pub fn new(hints: &DecodingHintDictionary) -> Self {
        let mut readers: Vec<Box<dyn UPCEANReader>> = Vec::new();
        if let Some(DecodeHintValue::PossibleFormats(possibleFormats)) =
            hints.get(&DecodeHintType::POSSIBLE_FORMATS)
        {
            // Collection<BarcodeFormat> possibleFormats = hints == null ? null :
            //   (Collection<BarcodeFormat>) hints.get(DecodeHintType.POSSIBLE_FORMATS);
            // Collection<UPCEANReader> readers = new ArrayList<>();
            if possibleFormats.contains(&BarcodeFormat::EAN_13) {
                readers.push(Box::<EAN13Reader>::default());
            } else if possibleFormats.contains(&BarcodeFormat::UPC_A) {
                readers.push(Box::<UPCAReader>::default());
            }
            if possibleFormats.contains(&BarcodeFormat::EAN_8) {
                readers.push(Box::<EAN8Reader>::default());
            }
            if possibleFormats.contains(&BarcodeFormat::UPC_E) {
                readers.push(Box::<UPCEReader>::default());
            }
        }
        if readers.is_empty() {
            readers.push(Box::<EAN13Reader>::default());
            // UPC-A is covered by EAN-13
            readers.push(Box::<EAN8Reader>::default());
            readers.push(Box::<UPCEReader>::default());
        }

        Self(readers)
    }

    fn try_decode_function(
        &self,
        reader: &Box<dyn UPCEANReader>,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
        startGuardPattern: &[usize; 2],
    ) -> Result<crate::RXingResult> {
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
                result.getRXingResultPoints().clone(),
                BarcodeFormat::UPC_A,
            );
            resultUPCA.putAllMetadata(result.getRXingResultMetadata().clone());

            return Ok(resultUPCA);
        }

        Ok(result)
    }
}

impl OneDReader for MultiFormatUPCEANReader {
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult> {
        // Compute this location once and reuse it on multiple implementations
        let startGuardPattern = STAND_IN.findStartGuardPattern(row)?;
        for reader in &self.0 {
            // for (UPCEANReader reader : readers) {
            let try_result =
                self.try_decode_function(reader, rowNumber, row, hints, &startGuardPattern);
            if try_result.is_ok() {
                return try_result;
            }
        }

        Err(Exceptions::notFound)
    }
}

use crate::DecodeHintType;
use crate::DecodingHintDictionary;
use crate::RXingResultMetadataType;
use crate::RXingResultMetadataValue;
use std::collections::HashMap;

impl Reader for MultiFormatUPCEANReader {
    fn decode(&mut self, image: &mut crate::BinaryBitmap) -> Result<crate::RXingResult> {
        self.decode_with_hints(image, &HashMap::new())
    }

    // Note that we don't try rotation without the try harder flag, even if rotation was supported.
    fn decode_with_hints(
        &mut self,
        image: &mut crate::BinaryBitmap,
        hints: &DecodingHintDictionary,
    ) -> Result<crate::RXingResult> {
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
            Err(Exceptions::notFound)
        }
    }

    fn reset(&mut self) {
        for reader in self.0.iter_mut() {
            reader.reset();
        }
    }
}
