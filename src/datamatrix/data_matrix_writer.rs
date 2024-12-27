#![allow(deprecated)]
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

use std::collections::HashMap;

use crate::{
    common::{BitMatrix, CharacterSet, Result},
    qrcode::encoder::ByteMatrix,
    BarcodeFormat, EncodeHintType, EncodeHintValue, EncodeHints, Exceptions, Writer,
};

use super::encoder::{
    high_level_encoder, minimal_encoder, DefaultPlacement, SymbolInfo, SymbolInfoLookup,
    SymbolShapeHint,
};

use super::encoder::error_correction;

// make default 0 to match previous behavior
const MARGINS_SIZE: u32 = 0;

/**
 * This object renders a Data Matrix code as a BitMatrix 2D array of greyscale values.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Guillaume Le Biller Added to zxing lib.
 */
#[derive(Default)]
pub struct DataMatrixWriter;

impl Writer for DataMatrixWriter {
    fn encode(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<crate::common::BitMatrix> {
        self.encode_with_hints(contents, format, width, height, &EncodeHints::default())
    }

    fn encode_with_hints(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
        hints: &EncodeHints,
    ) -> Result<crate::common::BitMatrix> {
        if contents.is_empty() {
            return Err(Exceptions::illegal_argument_with("Found empty contents"));
        }

        if format != &BarcodeFormat::DATA_MATRIX {
            return Err(Exceptions::illegal_argument_with(format!(
                "Can only encode DATA_MATRIX, but got {format:?}"
            )));
        }

        if width < 0 || height < 0 {
            return Err(Exceptions::illegal_argument_with(format!(
                "Requested dimensions can't be negative: {width}x{height}"
            )));
        }

        // Try to get force shape & min / max size
        let shape = &hints.DataMatrixShape.unwrap_or(SymbolShapeHint::FORCE_NONE);
        let minSize = hints.MinSize;
        let maxSize = hints.MaxSize;

        //1. step: Data encodation
        let encoded;

        let hasCompactionHint = hints.DataMatrixCompact.unwrap_or(false);
        if hasCompactionHint {
            let hasGS1FormatHint = hints.Gs1Format.unwrap_or(false);

            let mut charset: Option<CharacterSet> = None;
            if let Some(cs_name) = &hints.CharacterSet {
                charset = CharacterSet::get_character_set_by_name(cs_name);
            }
            encoded = minimal_encoder::encodeHighLevelWithDetails(
                contents,
                charset,
                if hasGS1FormatHint {
                    Some(0x1D as char)
                } else {
                    None
                },
                *shape,
            )?;
        } else {
            let hasForceC40Hint = hints.ForceC40.unwrap_or(false);
            encoded = high_level_encoder::encodeHighLevelWithDimensionForceC40(
                contents,
                *shape,
                minSize,
                maxSize,
                hasForceC40Hint,
            )?;
        }

        let symbol_lookup = SymbolInfoLookup::new();
        let Some(symbolInfo) = symbol_lookup.lookup_with_codewords_shape_size_fail(
            encoded.chars().count() as u32,
            *shape,
            &minSize,
            &maxSize,
            true,
        )?
        else {
            return Err(Exceptions::not_found_with("symbol info is bad"));
        };

        //2. step: ECC generation
        let codewords = error_correction::encodeECC200(&encoded, symbolInfo)?;

        //3. step: Module placement in Matrix
        let mut placement = DefaultPlacement::new(
            codewords,
            symbolInfo.getSymbolDataWidth()? as usize,
            symbolInfo.getSymbolDataHeight()? as usize,
        );
        placement.place()?;

        let margins = if let Some(margin) = &hints.Margin {
            margin
                .parse::<u32>()
                .map_err(|e| Exceptions::parse_with(format!("could not parse {margin}: {e}")))?
        } else {
            MARGINS_SIZE
        };

        //4. step: low-level encoding
        Self::encodeLowLevel(&placement, symbolInfo, width as u32, height as u32, margins)
    }
}

impl DataMatrixWriter {
    /**
     * Encode the given symbol info to a bit matrix.
     *
     * @param placement  The DataMatrix placement.
     * @param symbolInfo The symbol info to encode.
     * @return The bit matrix generated.
     */
    fn encodeLowLevel(
        placement: &DefaultPlacement,
        symbolInfo: &SymbolInfo,
        width: u32,
        height: u32,
        margins: u32,
    ) -> Result<BitMatrix> {
        let symbolWidth = symbolInfo.getSymbolDataWidth()?;
        let symbolHeight = symbolInfo.getSymbolDataHeight()?;

        let mut matrix =
            ByteMatrix::new(symbolInfo.getSymbolWidth()?, symbolInfo.getSymbolHeight()?);

        let mut matrixY = 0;

        for y in 0..symbolHeight {
            // for (int y = 0; y < symbolHeight; y++) {
            // Fill the top edge with alternate 0 / 1
            let mut matrixX;
            if (y % symbolInfo.matrixHeight) == 0 {
                matrixX = 0;
                for x in 0..symbolInfo.getSymbolWidth()? {
                    // for (int x = 0; x < symbolInfo.getSymbolWidth(); x++) {
                    matrix.set_bool(matrixX, matrixY, (x % 2) == 0);
                    matrixX += 1;
                }
                matrixY += 1;
            }
            matrixX = 0;
            for x in 0..symbolWidth {
                // for (int x = 0; x < symbolWidth; x++) {
                // Fill the right edge with full 1
                if (x % symbolInfo.matrixWidth) == 0 {
                    matrix.set_bool(matrixX, matrixY, true);
                    matrixX += 1;
                }
                matrix.set_bool(matrixX, matrixY, placement.getBit(x as usize, y as usize));
                matrixX += 1;
                // Fill the right edge with alternate 0 / 1
                if (x % symbolInfo.matrixWidth) == symbolInfo.matrixWidth - 1 {
                    matrix.set_bool(matrixX, matrixY, (y % 2) == 0);
                    matrixX += 1;
                }
            }
            matrixY += 1;
            // Fill the bottom edge with full 1
            if (y % symbolInfo.matrixHeight) == symbolInfo.matrixHeight - 1 {
                matrixX = 0;
                for _x in 0..symbolInfo.getSymbolWidth()? {
                    // for (int x = 0; x < symbolInfo.getSymbolWidth(); x++) {
                    matrix.set_bool(matrixX, matrixY, true);
                    matrixX += 1;
                }
                matrixY += 1;
            }
        }

        Self::convertByteMatrixToBitMatrix(&matrix, width, height, margins)
    }

    /**
     * Convert the ByteMatrix to BitMatrix.
     *
     * @param reqHeight The requested height of the image (in pixels) with the Datamatrix code
     * @param reqWidth The requested width of the image (in pixels) with the Datamatrix code
     * @param matrix The input matrix.
     * @return The output matrix.
     */
    fn convertByteMatrixToBitMatrix(
        matrix: &ByteMatrix,
        reqWidth: u32,
        reqHeight: u32,
        margins: u32,
    ) -> Result<BitMatrix> {
        let matrixWidth = matrix.getWidth();
        let matrixHeight = matrix.getHeight();
        let paddedWidth = matrixWidth + (margins * 2);
        let paddedHeight = matrixHeight + (margins * 2);
        let outputWidth = reqWidth.max(paddedWidth);
        let outputHeight = reqHeight.max(paddedHeight);

        let multiple = (outputWidth / paddedWidth).min(outputHeight / paddedHeight);

        let mut leftPadding = (outputWidth - (matrixWidth * multiple)) / 2;
        let mut topPadding = (outputHeight - (matrixHeight * multiple)) / 2;

        let mut output;

        // remove padding if requested width and height are too small
        if reqHeight < matrixHeight || reqWidth < matrixWidth {
            leftPadding = 0;
            topPadding = 0;
            output = BitMatrix::new(matrixWidth, matrixHeight)?;
        } else {
            output = BitMatrix::new(reqWidth, reqHeight)?;
        }

        output.clear();
        let mut inputY = 0;
        let mut outputY = topPadding;
        while inputY < matrixHeight {
            // for (int inputY = 0, outputY = topPadding; inputY < matrixHeight; inputY++, outputY += multiple) {
            // Write the contents of this row of the bytematrix
            let mut inputX = 0;
            let mut outputX = leftPadding;
            while inputX < matrixWidth {
                // for (int inputX = 0, outputX = leftPadding; inputX < matrixWidth; inputX++, outputX += multiple) {
                if matrix.get(inputX, inputY) == 1 {
                    output.setRegion(outputX, outputY, multiple, multiple)?;
                }

                inputX += 1;
                outputX += multiple
            }
            inputY += 1;
            outputY += multiple
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        datamatrix::{encoder::SymbolShapeHint, DataMatrixWriter},
        BarcodeFormat, EncodeHintType, EncodeHintValue, EncodeHints, Writer,
    };

    #[test]
    fn testDataMatrixImageWriter() {
        let mut hints = EncodeHints::default().with(EncodeHintValue::DataMatrixShape(
            SymbolShapeHint::FORCE_SQUARE,
        ));

        let bigEnough = 64;
        let writer = DataMatrixWriter {};
        let matrix = writer
            .encode_with_hints(
                "Hello Google",
                &BarcodeFormat::DATA_MATRIX,
                bigEnough,
                bigEnough,
                &hints,
            )
            .expect("must encode");
        assert!(bigEnough >= matrix.getWidth() as i32);
        assert!(bigEnough >= matrix.getHeight() as i32);
    }

    #[test]
    fn testDataMatrixWriter() {
        let mut hints = EncodeHints::default().with(EncodeHintValue::DataMatrixShape(
            SymbolShapeHint::FORCE_SQUARE,
        ));

        let bigEnough = 14;
        let writer = DataMatrixWriter {};
        let matrix = writer
            .encode_with_hints(
                "Hello Me",
                &BarcodeFormat::DATA_MATRIX,
                bigEnough,
                bigEnough,
                &hints,
            )
            .expect("must encode");
        assert_eq!(bigEnough, matrix.getWidth() as i32);
        assert_eq!(bigEnough, matrix.getHeight() as i32);
    }

    #[test]
    fn testDataMatrixTooSmall() {
        // The DataMatrix will not fit in this size, so the matrix should come back bigger
        let tooSmall = 8;
        let writer = DataMatrixWriter {};
        let matrix = writer
            .encode_with_hints(
                "http://www.google.com/",
                &BarcodeFormat::DATA_MATRIX,
                tooSmall,
                tooSmall,
                &EncodeHints::default(),
            )
            .expect("must encode");

        assert!(tooSmall < matrix.getWidth() as i32);
        assert!(tooSmall < matrix.getHeight() as i32);
    }
}
