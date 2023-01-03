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
#[derive(Default)]
pub struct AztecWriter;

impl Writer for AztecWriter {
    fn encode(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<crate::common::BitMatrix, crate::exceptions::Exceptions> {
        self.encode_with_hints(contents, format, width, height, &HashMap::new())
    }

    fn encode_with_hints(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
        hints: &std::collections::HashMap<crate::EncodeHintType, crate::EncodeHintValue>,
    ) -> Result<crate::common::BitMatrix, crate::exceptions::Exceptions> {
        let mut charset = None; // Do not add any ECI code by default
        let mut ecc_percent = encoder::DEFAULT_EC_PERCENT;
        let mut layers = encoder::DEFAULT_AZTEC_LAYERS;
        if hints.contains_key(&EncodeHintType::CHARACTER_SET) {
            if let EncodeHintValue::CharacterSet(cset_name) = hints
                .get(&EncodeHintType::CHARACTER_SET)
                .expect("already knonw presence")
            {
                if cset_name != "iso-8859-1" {
                    charset = Some(encoding::label::encoding_from_whatwg_label(cset_name).unwrap());
                }
                //          dbg!(cset_name);
                //  dbg!(encoding::label::encoding_from_whatwg_label(cset_name).unwrap().name(), encoding::label::encoding_from_whatwg_label(cset_name).unwrap().whatwg_name());
            }
            // charset = Charset.forName(hints.get(EncodeHintType.CHARACTER_SET).toString());
        }
        if hints.contains_key(&EncodeHintType::ERROR_CORRECTION) {
            if let EncodeHintValue::ErrorCorrection(ecc_level) = hints
                .get(&EncodeHintType::ERROR_CORRECTION)
                .expect("key exists")
            {
                ecc_percent = ecc_level.parse().expect("should convert to int");
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
            ecc_percent,
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
    ecc_percent: u32,
    layers: i32,
) -> Result<BitMatrix, Exceptions> {
    if format != BarcodeFormat::AZTEC {
        return Err(Exceptions::IllegalArgumentException(format!(
            "Can only encode AZTEC, but got {:?}",
            format
        )));
    }
    let aztec = if let Some(cset) = charset {
        // dbg!(cset.name(), cset.whatwg_name());
        encoder::encode_with_charset(contents, ecc_percent, layers, cset)?
    } else {
        encoder::encode(contents, ecc_percent, layers)?
    };
    renderRXingResult(&aztec, width, height)
}

fn renderRXingResult(code: &AztecCode, width: u32, height: u32) -> Result<BitMatrix, Exceptions> {
    let input = code.getMatrix();
    // if input == null {
    //   throw new IllegalStateException();
    // }
    let input_width = input.getWidth();
    let input_height = input.getHeight();
    let output_width = width.max(input_width);
    let output_height = height.max(input_height);

    let multiple = (output_width / input_width).min(output_height / input_height);
    let left_padding = (output_width - (input_width * multiple)) / 2;
    let top_padding = (output_height - (input_height * multiple)) / 2;

    let mut output = BitMatrix::new(output_width, output_height)?;

    let mut input_y = 0;
    let mut output_y = top_padding;
    while input_y < input_height {
        let mut input_x = 0;
        let mut output_x = left_padding;
        while input_x < input_width {
            if input.get(input_x, input_y) {
                output.setRegion(output_x, output_y, multiple, multiple)?;
            }

            input_x += 1;
            output_x += multiple;
        }

        input_y += 1;
        output_y += multiple
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
