/*
 * Copyright 2013 ZXing authors
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

use encoding::EncodingRef;

use crate::{
    common::BitMatrix, exceptions::Exceptions, BarcodeFormat, EncodeHintType, EncodeHintValue,
    Writer,
};

use super::encoder::{encoder, AztecCode};

/**
 * Renders an Aztec code as a {@link BitMatrix}.
 */
pub struct AztecWriter;

impl Writer for AztecWriter {
    fn encode(
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<crate::common::BitMatrix, crate::exceptions::Exceptions> {
        Self::encode_with_hints(contents, format, width, height, &HashMap::new())
    }

    fn encode_with_hints(
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
        hints: &std::collections::HashMap<crate::EncodeHintType, crate::EncodeHintValue>,
    ) -> Result<crate::common::BitMatrix, crate::exceptions::Exceptions> {
        let mut charset = None; // Do not add any ECI code by default
        let mut eccPercent = encoder::DEFAULT_EC_PERCENT;
        let mut layers = encoder::DEFAULT_AZTEC_LAYERS;
        if hints.contains_key(&EncodeHintType::CHARACTER_SET) {
            if let EncodeHintValue::CharacterSet(cset_name) = hints
                .get(&EncodeHintType::CHARACTER_SET)
                .expect("already knonw presence")
            {
                charset = Some(encoding::label::encoding_from_whatwg_label(cset_name).unwrap());
            }
            // charset = Charset.forName(hints.get(EncodeHintType.CHARACTER_SET).toString());
        }
        if hints.contains_key(&EncodeHintType::ERROR_CORRECTION) {
            if let EncodeHintValue::ErrorCorrection(ecc_level) = hints
                .get(&EncodeHintType::ERROR_CORRECTION)
                .expect("key exists")
            {
                eccPercent = ecc_level.parse().expect("should convert to int");
            }
            // eccPercent = Integer.parseInt(hints.get(EncodeHintType::ERROR_CORRECTION).toString());
        }
        if hints.contains_key(&EncodeHintType::AZTEC_LAYERS) {
            if let EncodeHintValue::AztecLayers(az_layers) = hints
                .get(&EncodeHintType::AZTEC_LAYERS)
                .expect("key exists")
            {
                layers = *az_layers;
            }
            // layers = Integer.parseInt(hints.get(EncodeHintType.AZTEC_LAYERS).toString());
        }
        encode(
            contents,
            *format,
            width as u32,
            height as u32,
            charset,
            eccPercent,
            layers,
        )
    }
}

fn encode(
    contents: &str,
    format: BarcodeFormat,
    width: u32,
    height: u32,
    charset: Option<EncodingRef>,
    eccPercent: u32,
    layers: u32,
) -> Result<BitMatrix, Exceptions> {
    if format != BarcodeFormat::AZTEC {
        return Err(Exceptions::IllegalArgumentException(format!(
            "Can only encode AZTEC, but got {:?}",
            format
        )));
    }
    let aztec = if let Some(cset) = charset {
        encoder::encode_with_charset(contents, eccPercent, layers, cset)?
    } else {
        encoder::encode(contents, eccPercent, layers)?
    };
    renderRXingResult(&aztec, width, height)
}

fn renderRXingResult(code: &AztecCode, width: u32, height: u32) -> Result<BitMatrix, Exceptions> {
    let input = code.getMatrix();
    // if input == null {
    //   throw new IllegalStateException();
    // }
    let inputWidth = input.getWidth();
    let inputHeight = input.getHeight();
    let outputWidth = width.max(inputWidth);
    let outputHeight = height.max(inputHeight);

    let multiple = (outputWidth / inputWidth).min(outputHeight / inputHeight);
    let leftPadding = (outputWidth - (inputWidth * multiple)) / 2;
    let topPadding = (outputHeight - (inputHeight * multiple)) / 2;

    let mut output = BitMatrix::new(outputWidth, outputHeight)?;

    let mut inputY = 0;
    let mut outputY = topPadding;
    while inputY < inputHeight {
        let mut inputX = 0;
        let mut outputX = leftPadding;
        while inputX < inputWidth {
            if input.get(inputX, inputY) {
                output.setRegion(outputX, outputY, multiple, multiple);
            }

            inputX += 1;
            outputX += multiple;
        }

        inputY += 1;
        outputY += multiple
    }
    // for (int inputY = 0, outputY = topPadding; inputY < inputHeight; inputY++, outputY += multiple) {
    //   // Write the contents of this row of the barcode
    //   for (int inputX = 0, outputX = leftPadding; inputX < inputWidth; inputX++, outputX += multiple) {
    //     if (input.get(inputX, inputY)) {
    //       output.setRegion(outputX, outputY, multiple, multiple);
    //     }
    //   }
    // }
    Ok(output)
}
