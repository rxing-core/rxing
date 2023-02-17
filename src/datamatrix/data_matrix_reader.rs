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
    common::{BitMatrix, DecoderRXingResult, DetectorRXingResult, Result},
    BarcodeFormat, DecodeHintType, DecodeHintValue, Exceptions, RXingResult,
    RXingResultMetadataType, RXingResultMetadataValue, Reader,
};

use super::{
    decoder::Decoder,
    detector::{zxing_cpp_detector, Detector},
};

use once_cell::sync::Lazy;

static DECODER: Lazy<Decoder> = Lazy::new(Decoder::new);

/**
 * This implementation can detect and decode Data Matrix codes in an image.
 *
 * @author bbrown@google.com (Brian Brown)
 */
#[derive(Default)]
pub struct DataMatrixReader;

// private static final Point[] NO_POINTS = new Point[0];

// private final Decoder decoder = new Decoder();

impl Reader for DataMatrixReader {
    /**
     * Locates and decodes a Data Matrix code in an image.
     *
     * @return a String representing the content encoded by the Data Matrix code
     * @throws NotFoundException if a Data Matrix code cannot be found
     * @throws FormatException if a Data Matrix code cannot be decoded
     * @throws ChecksumException if error correction fails
     */
    fn decode(&mut self, image: &mut crate::BinaryBitmap) -> Result<crate::RXingResult> {
        self.decode_with_hints(image, &HashMap::new())
    }

    /**
     * Locates and decodes a Data Matrix code in an image.
     *
     * @return a String representing the content encoded by the Data Matrix code
     * @throws NotFoundException if a Data Matrix code cannot be found
     * @throws FormatException if a Data Matrix code cannot be decoded
     * @throws ChecksumException if error correction fails
     */
    fn decode_with_hints(
        &mut self,
        image: &mut crate::BinaryBitmap,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult> {
        let try_harder = matches!(
            hints.get(&DecodeHintType::TRY_HARDER),
            Some(DecodeHintValue::TryHarder(true))
        );
        let decoderRXingResult;
        let mut points = Vec::new();
        if hints.contains_key(&DecodeHintType::PURE_BARCODE) {
            let bits = self.extractPureBits(image.getBlackMatrix())?;
            decoderRXingResult = DECODER.decode(&bits)?;
            points.clear();
        } else {
            //Result<DatamatrixDetectorResult, Exceptions>
            decoderRXingResult = if let Ok(fnd) = || -> Result<DecoderRXingResult> {
                let detectorRXingResult =
                    zxing_cpp_detector::detect(image.getBlackMatrix(), try_harder, true)?;
                let decoded = DECODER.decode(detectorRXingResult.getBits())?;
                points = detectorRXingResult.getPoints().to_vec();
                Ok(decoded)
            }() {
                fnd
            } else if try_harder {
                if let Ok(fnd) = || -> Result<DecoderRXingResult> {
                    let detectorRXingResult = Detector::new(image.getBlackMatrix())?.detect()?;
                    let decoded = DECODER.decode(detectorRXingResult.getBits())?;
                    points = detectorRXingResult.getPoints().to_vec();
                    Ok(decoded)
                }() {
                    fnd
                } else {
                    let bits = self.extractPureBits(image.getBlackMatrix())?;
                    DECODER.decode(&bits)?
                }
            } else {
                return Err(Exceptions::notFound);
            };

            // decoderRXingResult = DECODER.decode(detectorRXingResult.getBits())?;
        }

        let mut result = RXingResult::new(
            decoderRXingResult.getText(),
            decoderRXingResult.getRawBytes().clone(),
            points.clone(),
            BarcodeFormat::DATA_MATRIX,
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
                RXingResultMetadataValue::ErrorCorrectionLevel(ecLevel.to_string()),
            );
        }
        let other_meta = decoderRXingResult.getOther();
        if let Some(other) = other_meta {
            if let Some(dcr) = other.downcast_ref::<String>() {
                result.putMetadata(
                    RXingResultMetadataType::OTHER,
                    RXingResultMetadataValue::OTHER(dcr.to_owned()),
                );
            }
        }
        let contentType = decoderRXingResult.getContentType();
        if !contentType.is_empty() {
            result.putMetadata(
                RXingResultMetadataType::CONTENT_TYPE,
                RXingResultMetadataValue::ContentType(contentType.to_owned()),
            );
        }

        let mirrored = decoderRXingResult.getIsMirrored();
        if mirrored {
            result.putMetadata(
                RXingResultMetadataType::IS_MIRRORED,
                RXingResultMetadataValue::IsMirrored(mirrored),
            );
        }

        result.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier(format!(
                "]d{}",
                decoderRXingResult.getSymbologyModifier()
            )),
        );

        Ok(result)
    }

    fn reset(&mut self) {
        // do nothing
    }
}

impl DataMatrixReader {
    /**
     * This method detects a code in a "pure" image -- that is, pure monochrome image
     * which contains only an unrotated, unskewed, image of a code, with some white border
     * around it. This is a specialized method that works exceptionally fast in this special
     * case.
     */
    fn extractPureBits(&self, image: &BitMatrix) -> Result<BitMatrix> {
        let Some(leftTopBlack) = image.getTopLeftOnBit() else {
      return Err(Exceptions::notFound)
    };
        let Some(rightBottomBlack) = image.getBottomRightOnBit()else {
      return Err(Exceptions::notFound)
    };

        let moduleSize = Self::moduleSize(&leftTopBlack, image)?;

        let mut top = leftTopBlack[1];
        let bottom = rightBottomBlack[1];
        let mut left = leftTopBlack[0];
        let right = rightBottomBlack[0];

        let matrixWidth = (right as i32 - left as i32 + 1) / moduleSize as i32;
        let matrixHeight = (bottom as i32 - top as i32 + 1) / moduleSize as i32;
        if matrixWidth <= 0 || matrixHeight <= 0 {
            return Err(Exceptions::notFound);
            // throw NotFoundException.getNotFoundInstance();
        }

        let matrixWidth = matrixWidth as u32;
        let matrixHeight = matrixHeight as u32;

        // Push in the "border" by half the module width so that we start
        // sampling in the middle of the module. Just in case the image is a
        // little off, this will help recover.
        let nudge = moduleSize / 2;
        top += nudge;
        left += nudge;

        // Now just read off the bits
        let mut bits = BitMatrix::new(matrixWidth, matrixHeight)?;
        for y in 0..matrixHeight {
            // for (int y = 0; y < matrixHeight; y++) {
            let iOffset = top + y * moduleSize;
            for x in 0..matrixWidth {
                // for (int x = 0; x < matrixWidth; x++) {
                if image.get(left + x * moduleSize, iOffset) {
                    bits.set(x, y);
                }
            }
        }
        Ok(bits)
    }

    fn moduleSize(leftTopBlack: &[u32], image: &BitMatrix) -> Result<u32> {
        let width = image.getWidth();
        let mut x = leftTopBlack[0];
        let y = leftTopBlack[1];
        while x < width && image.get(x, y) {
            x += 1;
        }
        if x == width {
            return Err(Exceptions::notFound);
        }

        let moduleSize = x - leftTopBlack[0];
        if moduleSize == 0 {
            return Err(Exceptions::notFound);
        }

        Ok(moduleSize)
    }
}
