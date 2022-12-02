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

use std::collections::HashMap;

use crate::{
    BinaryBitmap, DecodingHintDictionary, Exceptions, RXingResult, RXingResultPoint, Reader,
    ResultPoint,
};

use super::MultipleBarcodeReader;

/**
 * <p>Attempts to locate multiple barcodes in an image by repeatedly decoding portion of the image.
 * After one barcode is found, the areas left, above, right and below the barcode's
 * {@link RXingResultPoint}s are scanned, recursively.</p>
 *
 * <p>A caller may want to also employ {@link ByQuadrantReader} when attempting to find multiple
 * 2D barcodes, like QR Codes, in an image, where the presence of multiple barcodes might prevent
 * detecting any one of them.</p>
 *
 * <p>That is, instead of passing a {@link Reader} a caller might pass
 * {@code new ByQuadrantReader(reader)}.</p>
 *
 * @author Sean Owen
 */
pub struct GenericMultipleBarcodeReader<T: Reader>(T);

impl<T: Reader> MultipleBarcodeReader for GenericMultipleBarcodeReader<T> {
    fn decodeMultiple(
        &mut self,
        image: &crate::BinaryBitmap,
    ) -> Result<Vec<crate::RXingResult>, crate::Exceptions> {
        self.decodeMultipleWithHints(image, &HashMap::new())
    }

    fn decodeMultipleWithHints(
        &mut self,
        image: &crate::BinaryBitmap,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<Vec<crate::RXingResult>, crate::Exceptions> {
        let mut results = Vec::new();
        self.doDecodeMultiple(image, hints, &mut results, 0, 0, 0);
        if results.is_empty() {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }
        Ok(results)
    }
}
impl<T: Reader> GenericMultipleBarcodeReader<T> {
    const MIN_DIMENSION_TO_RECUR: f32 = 100.0;
    const MAX_DEPTH: u32 = 4;

    pub fn new(delegate: T) -> Self {
        Self(delegate)
    }

    fn doDecodeMultiple(
        &mut self,
        image: &BinaryBitmap,
        hints: &DecodingHintDictionary,
        results: &mut Vec<RXingResult>,
        xOffset: u32,
        yOffset: u32,
        currentDepth: u32,
    ) {
        if currentDepth > Self::MAX_DEPTH {
            return;
        }

        // let result;
        let Ok(result) = self.0.decode_with_hints(image, hints) else {
            return;
        };

        let mut alreadyFound = false;
        for existingRXingResult in results.iter() {
            if existingRXingResult.getText() == result.getText() {
                alreadyFound = true;
                break;
            }
        }

        let resultPoints = result.getRXingResultPoints().clone();

        if !alreadyFound {
            results.push(Self::translateRXingResultPoints(result, xOffset, yOffset));
        }

        if resultPoints.is_empty() {
            return;
        }

        let width = image.getWidth();
        let height = image.getHeight();
        let mut minX: f32 = width as f32;
        let mut minY: f32 = height as f32;
        let mut maxX: f32 = 0.0;
        let mut maxY: f32 = 0.0;
        for point in resultPoints {
            // if (point == null) {
            //   continue;
            // }
            let x = point.getX();
            let y = point.getY();
            if x < minX {
                minX = x;
            }
            if y < minY {
                minY = y;
            }
            if x > maxX {
                maxX = x;
            }
            if y > maxY {
                maxY = y;
            }
        }

        // Decode left of barcode
        if minX > Self::MIN_DIMENSION_TO_RECUR {
            self.doDecodeMultiple(
                &image.crop(0, 0, minX as usize, height),
                hints,
                results,
                xOffset,
                yOffset,
                currentDepth + 1,
            );
        }
        // Decode above barcode
        if minY > Self::MIN_DIMENSION_TO_RECUR {
            self.doDecodeMultiple(
                &image.crop(0, 0, width, minY as usize),
                hints,
                results,
                xOffset,
                yOffset,
                currentDepth + 1,
            );
        }
        // Decode right of barcode
        if maxX < (width as f32) - Self::MIN_DIMENSION_TO_RECUR {
            self.doDecodeMultiple(
                &image.crop(maxX as usize, 0, width - maxX as usize, height),
                hints,
                results,
                xOffset + maxX as u32,
                yOffset,
                currentDepth + 1,
            );
        }
        // Decode below barcode
        if maxY < (height as f32) - Self::MIN_DIMENSION_TO_RECUR {
            self.doDecodeMultiple(
                &image.crop(0, maxY as usize, width, height - maxY as usize),
                hints,
                results,
                xOffset,
                yOffset + maxY as u32,
                currentDepth + 1,
            );
        }
    }

    fn translateRXingResultPoints(result: RXingResult, xOffset: u32, yOffset: u32) -> RXingResult {
        let oldRXingResultPoints = result.getRXingResultPoints();
        if oldRXingResultPoints.is_empty() {
            return result;
        }
        let mut newRXingResultPoints = Vec::with_capacity(oldRXingResultPoints.len()); //new RXingResultPoint[oldRXingResultPoints.length];
        for oldPoint in oldRXingResultPoints {
            // for (int i = 0; i < oldRXingResultPoints.length; i++) {
            // RXingResultPoint oldPoint = oldRXingResultPoints[i];
            // if (oldPoint != null) {
            newRXingResultPoints.push(RXingResultPoint::new(
                oldPoint.getX() + xOffset as f32,
                oldPoint.getY() + yOffset as f32,
            ));
            // }
        }
        let mut newRXingResult = RXingResult::new_complex(
            result.getText(),
            result.getRawBytes().clone(),
            result.getNumBits(),
            newRXingResultPoints,
            *result.getBarcodeFormat(),
            result.getTimestamp(),
        );
        newRXingResult.putAllMetadata(result.getRXingResultMetadata().clone());

        newRXingResult
    }
}
