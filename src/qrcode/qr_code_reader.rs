/*
 * Copyright 2007 ZXing authors
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
    common::{BitMatrix, DecoderRXingResult, DetectorRXingResult},
    BarcodeFormat, DecodeHintType, Exceptions, RXingResult, RXingResultMetadataType,
    RXingResultMetadataValue, RXingResultPoint, Reader, 
};

use super::{
    decoder::{decoder, QRCodeDecoderMetaData},
    detector::Detector,
};

/**
 * This implementation can detect and decode QR Codes in an image.
 *
 * @author Sean Owen
 */
pub struct QRCodeReader;
// pub struct QRCodeReader;  {

//   // private static final RXingResultPoint[] NO_POINTS = new RXingResultPoint[0];
// }

impl Reader for QRCodeReader {
    /**
     * Locates and decodes a QR code in an image.
     *
     * @return a String representing the content encoded by the QR code
     * @throws NotFoundException if a QR code cannot be found
     * @throws FormatException if a QR code cannot be decoded
     * @throws ChecksumException if error correction fails
     */
    fn decode(&self, image: &crate::BinaryBitmap) -> Result<crate::RXingResult, crate::Exceptions> {
        self.decode_with_hints(image, &HashMap::new())
    }

    fn decode_with_hints(
        &self,
        image: &crate::BinaryBitmap,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        let decoderRXingResult: DecoderRXingResult;
        let mut points: Vec<RXingResultPoint>;
        if hints.contains_key(&DecodeHintType::PURE_BARCODE) {
            let bits = Self::extractPureBits(image.getBlackMatrix()?)?;
            decoderRXingResult = decoder::decode_bitmatrix_with_hints(&bits, &hints)?;
            points = Vec::new();
        } else {
            let detectorRXingResult =
                Detector::new(image.getBlackMatrix()?.clone()).detect_with_hints(&hints)?;
            decoderRXingResult =
                decoder::decode_bitmatrix_with_hints(detectorRXingResult.getBits(), &hints)?;
            points = detectorRXingResult.getPoints().clone();
        }

        // If the code was mirrored: swap the bottom-left and the top-right points.
        if decoderRXingResult.getOther().is::<QRCodeDecoderMetaData>() {
            // if (decoderRXingResult.getOther() instanceof QRCodeDecoderMetaData) {
            decoderRXingResult
                .getOther()
                .downcast_ref::<QRCodeDecoderMetaData>()
                .unwrap()
                .applyMirroredCorrection(&mut points);
            // ((QRCodeDecoderMetaData) decoderRXingResult.getOther()).applyMirroredCorrection(points);
        }

        let mut result = RXingResult::new(
            decoderRXingResult.getText(),
            decoderRXingResult.getRawBytes().clone(),
            points,
            BarcodeFormat::QR_CODE,
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
        result.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier(format!(
                "]Q{}",
                decoderRXingResult.getSymbologyModifier()
            )),
        );

        Ok(result)
    }

    fn reset(&self) {
        // nothing
    }
}

impl QRCodeReader {
    pub fn new() -> Self {
        Self {}
    }

    /**
     * This method detects a code in a "pure" image -- that is, pure monochrome image
     * which contains only an unrotated, unskewed, image of a code, with some white border
     * around it. This is a specialized method that works exceptionally fast in this special
     * case.
     */
    fn extractPureBits(image: &BitMatrix) -> Result<BitMatrix, Exceptions> {
        let leftTopBlack = image.getTopLeftOnBit();
        let rightBottomBlack = image.getBottomRightOnBit();
        if leftTopBlack.is_none() || rightBottomBlack.is_none() {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        let leftTopBlack = leftTopBlack.unwrap();
        let rightBottomBlack = rightBottomBlack.unwrap();

        let moduleSize = Self::moduleSize(&leftTopBlack, image)?;

        let mut top = leftTopBlack[1] as i32;
        let  bottom = rightBottomBlack[1] as i32;
        let mut left = leftTopBlack[0] as i32;
        let mut right = rightBottomBlack[0] as i32;

        // Sanity check!
        if left >= right || top >= bottom {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        if bottom - top != right - left {
            // Special case, where bottom-right module wasn't black so we found something else in the last row
            // Assume it's a square, so use height as the width
            right = left + (bottom - top);
            if right >= image.getWidth() as i32 {
                // Abort if that would not make sense -- off image
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
        }
        let matrixWidth = ((right as f32 - left as f32 + 1.0) / moduleSize).round() as u32;
        let matrixHeight = ((bottom as f32 - top as f32 + 1.0) / moduleSize).round() as u32;
        if matrixWidth == 0 || matrixHeight == 0 {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }
        if matrixHeight != matrixWidth {
            // Only possibly decode square regions
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        // Push in the "border" by half the module width so that we start
        // sampling in the middle of the module. Just in case the image is a
        // little off, this will help recover.
        let nudge = (moduleSize / 2.0) as u32;
        top += nudge as i32;
        left += nudge as i32;

        // But careful that this does not sample off the edge
        // "right" is the farthest-right valid pixel location -- right+1 is not necessarily
        // This is positive by how much the inner x loop below would be too large
        let nudgedTooFarRight = left as i32 + ((matrixWidth as i32 - 1) as f32 * moduleSize as f32) as i32 - right as i32;
        if nudgedTooFarRight > 0 {
            if nudgedTooFarRight > nudge as i32 {
                // Neither way fits; abort
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
            left -= nudgedTooFarRight;
        }
        // See logic above
        let nudgedTooFarDown = top + ((matrixHeight - 1) as f32 * moduleSize) as i32 - bottom;
        if nudgedTooFarDown > 0 {
            if nudgedTooFarDown > nudge as i32 {
                // Neither way fits; abort
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
            top -= nudgedTooFarDown;
        }

        // Now just read off the bits
        let mut bits = BitMatrix::new(matrixWidth, matrixHeight)?;
        for y in 0..matrixHeight {
            // for (int y = 0; y < matrixHeight; y++) {
            let iOffset = top + ((y as f32) * moduleSize) as i32;
            for x in 0..matrixWidth {
                // for (int x = 0; x < matrixWidth; x++) {
                if image.get(left as u32 + (x as f32 * moduleSize) as u32, iOffset as u32) {
                    bits.set(x, y);
                }
            }
        }
        Ok(bits)
    }

    fn moduleSize(leftTopBlack: &[u32], image: &BitMatrix) -> Result<f32, Exceptions> {
        let height = image.getHeight();
        let width = image.getWidth();
        let mut x = leftTopBlack[0];
        let mut y = leftTopBlack[1];
        let mut inBlack = true;
        let mut transitions = 0;
        while x < width && y < height {
            if inBlack != image.get(x, y) {
                transitions += 1;
                if transitions == 5 {
                    break;
                }
                inBlack = !inBlack;
            }
            x += 1;
            y += 1;
        }
        if x == width || y == height {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }
        Ok((x - leftTopBlack[0]) as f32 / 7.0)
    }
}
