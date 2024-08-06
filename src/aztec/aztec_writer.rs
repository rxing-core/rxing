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

use crate::{
    common::{BitMatrix, CharacterSet, Result},
    exceptions::Exceptions,
    BarcodeFormat, EncodeHints, Writer,
};

use super::encoder::{aztec_encoder, AztecCode};

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
        let mut charset = None; // Do not add any ECI code by default
        let mut ecc_percent = aztec_encoder::DEFAULT_EC_PERCENT;
        let mut layers = aztec_encoder::DEFAULT_AZTEC_LAYERS;
        if let Some(cset_name) = &hints.CharacterSet {
            if cset_name.to_lowercase() != "iso-8859-1" {
                charset = CharacterSet::get_character_set_by_name(cset_name);
            }
        }
        if let Some(ecc_level) = &hints.ErrorCorrection {
            ecc_percent = ecc_level.parse().unwrap_or(23);
        }
        if let Some(az_layers) = hints.AztecLayers {
            layers = az_layers;
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
    charset: Option<CharacterSet>,
    ecc_percent: u32,
    layers: i32,
) -> Result<BitMatrix> {
    if format != BarcodeFormat::AZTEC {
        return Err(Exceptions::illegal_argument_with(format!(
            "can only encode AZTEC, but got {format:?}"
        )));
    }
    let aztec = if let Some(cset) = charset {
        // dbg!(cset.name(), cset.whatwg_name());
        aztec_encoder::encode_with_charset(contents, ecc_percent, layers, cset)?
    } else {
        aztec_encoder::encode(contents, ecc_percent, layers)?
    };
    renderRXingResult(&aztec, width, height)
}

fn renderRXingResult(code: &AztecCode, width: u32, height: u32) -> Result<BitMatrix> {
    let input = code.getMatrix();

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
    Ok(output)
}
