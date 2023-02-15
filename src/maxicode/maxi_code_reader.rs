/*
 * Copyright 2011 ZXing authors
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
    common::{BitMatrix, DetectorRXingResult},
    BarcodeFormat, DecodeHintType, DecodeHintValue, Exceptions, RXingResult,
    RXingResultMetadataType, Reader,
};

use super::{decoder::maxicode_decoder, detector};

/**
 * This implementation can detect and decode a MaxiCode in an image.
 */
#[derive(Default)]
pub struct MaxiCodeReader {
    // private final Decoder decoder = new Decoder();
}

impl Reader for MaxiCodeReader {
    /**
     * Locates and decodes a MaxiCode in an image.
     *
     * @return a String representing the content encoded by the MaxiCode
     * @throws NotFoundException if a MaxiCode cannot be found
     * @throws FormatException if a MaxiCode cannot be decoded
     * @throws ChecksumException if error correction fails
     */
    fn decode(
        &mut self,
        image: &mut crate::BinaryBitmap,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        self.decode_with_hints(image, &HashMap::new())
    }

    /**
     * Locates and decodes a MaxiCode in an image.
     *
     * @return a String representing the content encoded by the MaxiCode
     * @throws NotFoundException if a MaxiCode cannot be found
     * @throws FormatException if a MaxiCode cannot be decoded
     * @throws ChecksumException if error correction fails
     */
    fn decode_with_hints(
        &mut self,
        image: &mut crate::BinaryBitmap,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        // Note that MaxiCode reader effectively always assumes PURE_BARCODE mode
        // and can't detect it in an image
        let try_harder = matches!(
            hints.get(&DecodeHintType::TRY_HARDER),
            Some(DecodeHintValue::TryHarder(true))
        );

        let mut rotation = None;

        let decoderRXingResult = if try_harder {
            let result = detector::detect(image.getBlackMatrixMut(), try_harder)?;
            rotation = Some(result.rotation());
            let parsed_result = detector::read_bits(result.getBits())?;
            maxicode_decoder::decode_with_hints(&parsed_result, hints)?
        } else {
            let bits = Self::extractPureBits(image.getBlackMatrix())?;
            maxicode_decoder::decode_with_hints(&bits, hints)?
        };

        // let bits = Self::extractPureBits(image.getBlackMatrix())?;
        // let decoderRXingResult = maxicode_decoder::decode_with_hints(bits, hints)?;
        let mut result = RXingResult::new(
            decoderRXingResult.getText(),
            decoderRXingResult.getRawBytes().clone(),
            Vec::new(),
            BarcodeFormat::MAXICODE,
        );

        let ecLevel = decoderRXingResult.getECLevel();
        if !ecLevel.is_empty() {
            result.putMetadata(
                RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
                crate::RXingResultMetadataValue::ErrorCorrectionLevel(ecLevel.to_owned()),
            );
        }

        if let Some(rot) = rotation {
            if rot > 0.0 {
                result.putMetadata(
                    RXingResultMetadataType::ORIENTATION,
                    crate::RXingResultMetadataValue::Orientation(rot as i32),
                )
            }
        }

        Ok(result)
    }

    fn reset(&mut self) {
        // do nothing
    }
}
impl MaxiCodeReader {
    pub const MATRIX_WIDTH: u32 = 30;
    pub const MATRIX_HEIGHT: u32 = 33;

    /**
     * This method detects a code in a "pure" image -- that is, pure monochrome image
     * which contains only an unrotated, unskewed, image of a code, with some white border
     * around it. This is a specialized method that works exceptionally fast in this special
     * case.
     */
    fn extractPureBits(image: &BitMatrix) -> Result<BitMatrix, Exceptions> {
        let enclosingRectangleOption = image.getEnclosingRectangle();
        if enclosingRectangleOption.is_none() {
            return Err(Exceptions::notFoundEmpty());
        }

        let enclosingRectangle = enclosingRectangleOption.ok_or(Exceptions::notFoundEmpty())?;

        let left = enclosingRectangle[0];
        let top = enclosingRectangle[1];
        let width = enclosingRectangle[2];
        let height = enclosingRectangle[3];

        // Now just read off the bits
        let mut bits = BitMatrix::new(Self::MATRIX_WIDTH, Self::MATRIX_HEIGHT)?;
        for y in 0..Self::MATRIX_HEIGHT {
            // for (int y = 0; y < MATRIX_HEIGHT; y++) {
            let iy = (top + (y * height + height / 2) / Self::MATRIX_HEIGHT).min(height - 1);
            for x in 0..Self::MATRIX_WIDTH {
                // for (int x = 0; x < MATRIX_WIDTH; x++) {
                // srowen: I don't quite understand why the formula below is necessary, but it
                // can walk off the image if left + width = the right boundary. So cap it.
                let ix = left
                    + ((x * width + width / 2 + (y & 0x01) * width / 2) / Self::MATRIX_WIDTH)
                        .min(width - 1);
                if image.get(ix, iy) {
                    bits.set(x, y);
                }
            }
        }
        Ok(bits)
    }
}
