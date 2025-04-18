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

use crate::{
    common::Result, multi::MultipleBarcodeReader, BarcodeFormat, Binarizer, BinaryBitmap,
    DecodeHints, Exceptions, ImmutableReader, Point, RXingResult, RXingResultMetadataType,
    RXingResultMetadataValue, Reader,
};

use super::{
    decoder::pdf_417_scanning_decoder, detector::pdf_417_detector, pdf_417_common,
    PDF417RXingResultMetadata,
};

/**
 * This implementation can detect and decode PDF417 codes in an image.
 *
 * @author Guenther Grau
 */
#[derive(Default)]
pub struct PDF417Reader;

impl Reader for PDF417Reader {
    /**
     * Locates and decodes a PDF417 code in an image.
     *
     * @return a String representing the content encoded by the PDF417 code
     * @throws NotFoundException if a PDF417 code cannot be found,
     * @throws FormatException if a PDF417 cannot be decoded
     */
    fn decode<B: Binarizer>(&mut self, image: &mut BinaryBitmap<B>) -> Result<RXingResult> {
        self.decode_with_hints(image, &DecodeHints::default())
    }

    fn decode_with_hints<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> Result<crate::RXingResult> {
        self.internal_decode_with_hints(image, hints)
    }
}

impl ImmutableReader for PDF417Reader {
    fn immutable_decode_with_hints<B: Binarizer>(
        &self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> Result<RXingResult> {
        self.internal_decode_with_hints(image, hints)
    }
}

impl MultipleBarcodeReader for PDF417Reader {
    fn decode_multiple<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
    ) -> Result<Vec<RXingResult>> {
        self.decode_multiple_with_hints(image, &DecodeHints::default())
    }

    fn decode_multiple_with_hints<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> Result<Vec<RXingResult>> {
        Self::decode(image, hints, true)
    }
}

impl PDF417Reader {
    pub fn new() -> Self {
        Self
    }

    fn decode<B: Binarizer>(
        image: &mut BinaryBitmap<B>,
        hints: &DecodeHints,
        multiple: bool,
    ) -> Result<Vec<RXingResult>> {
        let mut results = Vec::new();
        let detectorRXingResult = pdf_417_detector::detect_with_hints(image, hints, multiple)?;

        for points in detectorRXingResult.getPoints() {
            let points_filtered = points.iter().flatten().copied().collect();
            // let points_filtered = points.iter().filter_map(|e| *e).collect();

            let decoderRXingResult = pdf_417_scanning_decoder::decode(
                detectorRXingResult.getBits(),
                points[4],
                points[5],
                points[6],
                points[7],
                Self::getMinCodewordWidth(points),
                Self::getMaxCodewordWidth(points),
            )?;

            let mut result = RXingResult::new(
                decoderRXingResult.getText(),
                decoderRXingResult.getRawBytes().clone(),
                points_filtered,
                BarcodeFormat::PDF_417,
            );

            result.putMetadata(
                RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
                RXingResultMetadataValue::ErrorCorrectionLevel(
                    decoderRXingResult.getECLevel().to_owned(),
                ),
            );

            if let Some(pdf417RXingResultMetadata) = decoderRXingResult.getOther() {
                if pdf417RXingResultMetadata.is::<PDF417RXingResultMetadata>() {
                    let data = RXingResultMetadataValue::Pdf417ExtraMetadata(
                        pdf417RXingResultMetadata
                            .clone()
                            .downcast::<PDF417RXingResultMetadata>()
                            .map_err(|_| Exceptions::ILLEGAL_STATE)?,
                    );
                    result.putMetadata(RXingResultMetadataType::PDF417_EXTRA_METADATA, data);
                }
            }
            // PDF417RXingResultMetadata pdf417RXingResultMetadata = (PDF417RXingResultMetadata) decoderRXingResult.getOther();

            // if (pdf417RXingResultMetadata != null) {
            //   result.putMetadata(RXingResultMetadataType.PDF417_EXTRA_METADATA, pdf417RXingResultMetadata);
            // }

            result.putMetadata(
                RXingResultMetadataType::ORIENTATION,
                RXingResultMetadataValue::Orientation(detectorRXingResult.getRotation() as i32),
            );
            result.putMetadata(
                RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
                RXingResultMetadataValue::SymbologyIdentifier(format!(
                    "]L{}",
                    decoderRXingResult.getSymbologyModifier()
                )),
            );
            results.push(result);
        }
        Ok(results)
    }

    fn getMaxWidth(p1: &Option<Point>, p2: &Option<Point>) -> u64 {
        if let (Some(p1), Some(p2)) = (p1, p2) {
            (p1.x - p2.x).abs() as u64
        } else {
            0
        }
    }

    fn getMinWidth(p1: &Option<Point>, p2: &Option<Point>) -> u64 {
        if let (Some(p1), Some(p2)) = (p1, p2) {
            (p1.x - p2.x).abs() as u64
        } else {
            u32::MAX as u64
        }
    }

    fn getMaxCodewordWidth(p: &[Option<Point>]) -> u32 {
        Self::getMaxWidth(&p[0], &p[4])
            .max(
                Self::getMaxWidth(&p[6], &p[2]) * pdf_417_common::MODULES_IN_CODEWORD as u64
                    / pdf_417_common::MODULES_IN_STOP_PATTERN as u64,
            )
            .max(Self::getMaxWidth(&p[1], &p[5]).max(
                Self::getMaxWidth(&p[7], &p[3]) * pdf_417_common::MODULES_IN_CODEWORD as u64
                    / pdf_417_common::MODULES_IN_STOP_PATTERN as u64,
            )) as u32
    }

    fn getMinCodewordWidth(p: &[Option<Point>]) -> u32 {
        Self::getMinWidth(&p[0], &p[4])
            .min(
                Self::getMinWidth(&p[6], &p[2]) * pdf_417_common::MODULES_IN_CODEWORD as u64
                    / pdf_417_common::MODULES_IN_STOP_PATTERN as u64,
            )
            .min(Self::getMinWidth(&p[1], &p[5]).min(
                Self::getMinWidth(&p[7], &p[3]) * pdf_417_common::MODULES_IN_CODEWORD as u64
                    / pdf_417_common::MODULES_IN_STOP_PATTERN as u64,
            )) as u32
    }

    fn internal_decode_with_hints<B: Binarizer>(
        &self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> Result<RXingResult> {
        let result = Self::decode(image, hints, false)?;
        if result.is_empty() {
            return Err(Exceptions::NOT_FOUND);
        }
        Ok(result[0].clone())
    }
}
