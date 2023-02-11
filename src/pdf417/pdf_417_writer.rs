/*
 * Copyright 2012 ZXing authors
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
    common::BitMatrix, BarcodeFormat, EncodeHintType, EncodeHintValue, Exceptions, Writer,
};

use super::encoder::PDF417;

/**
 * default white space (margin) around the code
 */
const WHITE_SPACE: u32 = 30;

/**
 * default error correction level
 */
const DEFAULT_ERROR_CORRECTION_LEVEL: u32 = 2;

/**
 * @author Jacob Haynes
 * @author qwandor@google.com (Andrew Walbran)
 */
#[derive(Default)]
pub struct PDF417Writer;

impl Writer for PDF417Writer {
    fn encode(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<crate::common::BitMatrix, crate::Exceptions> {
        self.encode_with_hints(contents, format, width, height, &HashMap::new())
    }

    fn encode_with_hints(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
        hints: &crate::EncodingHintDictionary,
    ) -> Result<crate::common::BitMatrix, crate::Exceptions> {
        if format != &BarcodeFormat::PDF_417 {
            return Err(Exceptions::IllegalArgumentException(Some(format!(
                "Can only encode PDF_417, but got {format}"
            ))));
        }

        let mut encoder = PDF417::new();
        let mut margin = WHITE_SPACE;
        let mut errorCorrectionLevel = DEFAULT_ERROR_CORRECTION_LEVEL;
        let mut autoECI = false;

        if !hints.is_empty() {
            if let Some(EncodeHintValue::Pdf417Compact(compact)) =
                hints.get(&EncodeHintType::PDF417_COMPACT)
            {
                if let Ok(res) = compact.parse::<bool>() {
                    encoder.setCompact(res);
                }
            }
            if let Some(EncodeHintValue::Pdf417Compaction(compaction)) =
                hints.get(&EncodeHintType::PDF417_COMPACTION)
            {
                encoder.setCompaction(compaction.try_into()?);
            }
            if let Some(EncodeHintValue::Pdf417Dimensions(dimensions)) =
                hints.get(&EncodeHintType::PDF417_DIMENSIONS)
            {
                encoder.setDimensions(
                    dimensions.getMaxCols() as u32,
                    dimensions.getMinCols() as u32,
                    dimensions.getMaxRows() as u32,
                    dimensions.getMinRows() as u32,
                );
            }
            if let Some(EncodeHintValue::Margin(m1)) = hints.get(&EncodeHintType::MARGIN) {
                if let Ok(m) = m1.parse::<u32>() {
                    margin = m;
                }
            }
            if let Some(EncodeHintValue::ErrorCorrection(ec)) =
                hints.get(&EncodeHintType::ERROR_CORRECTION)
            {
                if let Ok(ec_parsed) = ec.parse::<u32>() {
                    errorCorrectionLevel = ec_parsed;
                }
            }
            if let Some(EncodeHintValue::CharacterSet(cs)) =
                hints.get(&EncodeHintType::CHARACTER_SET)
            {
                encoder.setEncoding(encoding::label::encoding_from_whatwg_label(cs));
            }
            if let Some(EncodeHintValue::Pdf417AutoEci(auto_eci_str)) =
                hints.get(&EncodeHintType::PDF417_AUTO_ECI)
            {
                if let Ok(auto_eci_parsed) = auto_eci_str.parse::<bool>() {
                    autoECI = auto_eci_parsed;
                }
            }
        }

        Self::bitMatrixFromEncoder(
            &mut encoder,
            contents,
            errorCorrectionLevel,
            width as u32,
            height as u32,
            margin,
            autoECI,
        )
    }
}

impl PDF417Writer {
    /**
     * Takes encoder, accounts for width/height, and retrieves bit matrix
     */
    fn bitMatrixFromEncoder(
        encoder: &mut PDF417,
        contents: &str,
        errorCorrectionLevel: u32,
        width: u32,
        height: u32,
        margin: u32,
        autoECI: bool,
    ) -> Result<BitMatrix, Exceptions> {
        encoder.generateBarcodeLogicWithAutoECI(contents, errorCorrectionLevel, autoECI)?;

        let aspectRatio = 4;
        let mut originalScale = encoder
            .getBarcodeMatrix()
            .as_ref()
            .ok_or(Exceptions::IllegalStateException(None))?
            .getScaledMatrix(1, aspectRatio);
        let mut rotated = false;
        if (height > width) != (originalScale[0].len() < originalScale.len()) {
            originalScale = Self::rotateArray(&originalScale);
            rotated = true;
        }

        let scaleX = width as usize / originalScale[0].len();
        let scaleY = height as usize / originalScale.len();
        let scale = scaleX.min(scaleY);

        if scale > 1 {
            let mut scaledMatrix = encoder
                .getBarcodeMatrix()
                .as_ref()
                .ok_or(Exceptions::IllegalStateException(None))?
                .getScaledMatrix(scale, scale * aspectRatio);
            if rotated {
                scaledMatrix = Self::rotateArray(&scaledMatrix);
            }
            return Self::bitMatrixFromBitArray(&scaledMatrix, margin)
                .ok_or(Exceptions::IllegalStateException(None));
        }

        Self::bitMatrixFromBitArray(&originalScale, margin)
            .ok_or(Exceptions::IllegalStateException(None))
    }

    /**
     * This takes an array holding the values of the PDF 417
     *
     * @param input a byte array of information with 0 is black, and 1 is white
     * @param margin border around the barcode
     * @return BitMatrix of the input
     */
    fn bitMatrixFromBitArray(input: &Vec<Vec<u8>>, margin: u32) -> Option<BitMatrix> {
        // Creates the bit matrix with extra space for whitespace
        let mut output = BitMatrix::new(
            input[0].len() as u32 + 2 * margin,
            input.len() as u32 + 2 * margin,
        )
        .ok()?;
        output.clear();
        let mut y = 0;
        let mut yOutput = (output.getHeight() - margin - 1) as isize;
        while y < input.len() {
            let inputY = &input[y];
            for (x, x_index_val) in inputY.iter().enumerate().take(input[y].len()) {
                // Zero is white in the byte matrix
                if x_index_val == &1 {
                    output.set(x as u32 + margin, yOutput as u32);
                }
            }
            y += 1;
            yOutput -= 1;
        }
        Some(output)
    }

    /**
     * Takes and rotates the it 90 degrees
     */
    fn rotateArray(bitarray: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut temp = vec![vec![0; bitarray[0].len()]; bitarray.len()];

        for ii in 0..bitarray.len() {
            // This makes the direction consistent on screen when rotating the screen;
            let inverseii = bitarray.len() - ii - 1;
            for (jj, tmp_spot) in temp.iter_mut().enumerate().take(bitarray[0].len()) {
                tmp_spot[inverseii] = bitarray[ii][jj];
            }
        }

        temp
    }
    pub fn new() -> Self {
        Self::default()
    }
}

/**
 * Tests {@link PDF417Writer}.
 */
#[cfg(test)]
mod PDF417WriterTestCase {
    use std::collections::HashMap;

    use crate::{pdf417::PDF417Writer, BarcodeFormat, EncodeHintType, EncodeHintValue, Writer};

    #[test]
    fn testDataMatrixImageWriter() {
        let mut hints = HashMap::new();
        hints.insert(
            EncodeHintType::MARGIN,
            EncodeHintValue::Margin(0.to_string()),
        );
        let size = 64;
        let writer = PDF417Writer::new();
        let matrix = writer
            .encode_with_hints("Hello Google", &BarcodeFormat::PDF_417, size, size, &hints)
            .expect("encode");
        // assert!(matrix.is_ok());
        let expected = r"X X X X X X X X   X   X   X       X X X X   X   X   X X X X         X X   X   X           X X         X X X X   X X     X     X X X     X X   X           X       X X     X X X X X   X   X   X X X X X     X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X   X   X   X X X X         X X   X   X           X X         X X X X   X X     X     X X X     X X   X           X       X X     X X X X X   X   X   X X X X X     X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X   X   X   X X X X         X X   X   X           X X         X X X X   X X     X     X X X     X X   X           X       X X     X X X X X   X   X   X X X X X     X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X   X   X   X X X X         X X   X   X           X X         X X X X   X X     X     X X X     X X   X           X       X X     X X X X X   X   X   X X X X X     X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X   X   X         X         X   X X     X     X X X X X X     X X X           X   X X       X   X X X   X           X X     X     X X X X X X   X   X   X X X       X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X   X   X         X         X   X X     X     X X X X X X     X X X           X   X X       X   X X X   X           X X     X     X X X X X X   X   X   X X X       X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X   X   X         X         X   X X     X     X X X X X X     X X X           X   X X       X   X X X   X           X X     X     X X X X X X   X   X   X X X       X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X   X   X         X         X   X X     X     X X X X X X     X X X           X   X X       X   X X X   X           X X     X     X X X X X X   X   X   X X X       X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X     X X X X             X   X X X       X       X X       X     X X   X X     X X X X       X X       X X X X X     X     X   X   X   X         X X X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X     X X X X             X   X X X       X       X X       X     X X   X X     X X X X       X X       X X X X X     X     X   X   X   X         X X X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X     X X X X             X   X X X       X       X X       X     X X   X X     X X X X       X X       X X X X X     X     X   X   X   X         X X X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X     X X X X             X   X X X       X       X X       X     X X   X X     X X X X       X X       X X X X X     X     X   X   X   X         X X X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X X X X     X X X X       X         X X       X X     X     X   X     X X X X       X X X X   X       X X       X X         X   X X   X   X X X X     X X X X X   X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X X X X     X X X X       X         X X       X X     X     X   X     X X X X       X X X X   X       X X       X X         X   X X   X   X X X X     X X X X X   X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X X X X     X X X X       X         X X       X X     X     X   X     X X X X       X X X X   X       X X       X X         X   X X   X   X X X X     X X X X X   X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X   X   X X X X     X X X X       X         X X       X X     X     X   X     X X X X       X X X X   X       X X       X X         X   X X   X   X X X X     X X X X X   X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X   X   X X X           X       X X     X       X X X     X       X X X X X X   X X   X     X X     X   X X X   X     X X X X X       X X X   X   X X X     X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X   X   X X X           X       X X     X       X X X     X       X X X X X X   X X   X     X X     X   X X X   X     X X X X X       X X X   X   X X X     X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X   X   X X X           X       X X     X       X X X     X       X X X X X X   X X   X     X X     X   X X X   X     X X X X X       X X X   X   X X X     X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X   X   X X X           X       X X     X       X X X     X       X X X X X X   X X   X     X X     X   X X X   X     X X X X X       X X X   X   X X X     X X         X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X X   X   X X X X   X X     X           X   X       X X X X   X       X   X         X X X X     X           X X X   X       X X   X X X X   X   X X X X X   X X     X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X X   X   X X X X   X X     X           X   X       X X X X   X       X   X         X X X X     X           X X X   X       X X   X X X X   X   X X X X X   X X     X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X X   X   X X X X   X X     X           X   X       X X X X   X       X   X         X X X X     X           X X X   X       X X   X X X X   X   X X X X X   X X     X X X X X X X   X       X   X     X 
X X X X X X X X   X   X   X       X X X X X   X   X X X X   X X     X           X   X       X X X X   X       X   X         X X X X     X           X X X   X       X X   X X X X   X   X X X X X   X X     X X X X X X X   X       X   X     X 
";
        assert_eq!(expected, matrix.to_string());
    }
}
