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
    common::BitMatrix, BarcodeFormat, EncodeHintType, EncodeHintValue, Exceptions, Writer,
};

use super::{
    decoder::ErrorCorrectionLevel,
    encoder::{encoder, QRCode},
};

const QUIET_ZONE_SIZE: i32 = 4;

/**
 * This object renders a QR Code as a BitMatrix 2D array of greyscale values.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct QRCodeWriter; // {

// private static final int QUIET_ZONE_SIZE = 4;
//}

impl Writer for QRCodeWriter {
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
        if (contents.is_empty()) {
            return Err(Exceptions::IllegalArgumentException(
                "Found empty contents".to_owned(),
            ));
            // throw new IllegalArgumentException("Found empty contents");
        }

        if format != &BarcodeFormat::QR_CODE {
            return Err(Exceptions::IllegalArgumentException(format!(
                "Can only encode QR_CODE, but got {:?}",
                format
            )));
            // throw new IllegalArgumentException("Can only encode QR_CODE, but got " + format);
        }

        if (width < 0 || height < 0) {
            return Err(Exceptions::IllegalArgumentException(format!(
                "Requested dimensions are too small: {}x{}",
                width, height
            )));
            // throw new IllegalArgumentException("Requested dimensions are too small: " + width + 'x' +
            //     height);
        }

        let mut errorCorrectionLevel = ErrorCorrectionLevel::L;
        let mut quietZone = QUIET_ZONE_SIZE;
        // if (hints != null) {
        if hints.contains_key(&EncodeHintType::ERROR_CORRECTION) {
            if let EncodeHintValue::ErrorCorrection(ec_level) =
                hints.get(&EncodeHintType::ERROR_CORRECTION).unwrap()
            {
                errorCorrectionLevel = ec_level.parse()?;
            }
            // errorCorrectionLevel = ErrorCorrectionLevel.valueOf(hints.get(EncodeHintType.ERROR_CORRECTION).toString());
        }
        if hints.contains_key(&EncodeHintType::MARGIN) {
            if let EncodeHintValue::Margin(margin) = hints.get(&EncodeHintType::MARGIN).unwrap() {
                quietZone = margin.parse().unwrap();
                // quietZone = Integer.parseInt(hints.get(EncodeHintType.MARGIN).toString());
            }

            // quietZone = Integer.parseInt(hints.get(EncodeHintType.MARGIN).toString());
        }
        // }

        let code = encoder::encode_with_hints(contents, errorCorrectionLevel, &hints)?;

        Self::renderRXingResult(&code, width, height, quietZone)
    }
}

impl QRCodeWriter {
    // Note that the input matrix uses 0 == white, 1 == black, while the output matrix uses
    // 0 == black, 255 == white (i.e. an 8 bit greyscale bitmap).
    fn renderRXingResult(
        code: &QRCode,
        width: i32,
        height: i32,
        quietZone: i32,
    ) -> Result<BitMatrix, Exceptions> {
        let input = code.getMatrix();
        if input.is_none() {
            return Err(Exceptions::IllegalStateException(
                "matrix is empty".to_owned(),
            ));
            // throw new IllegalStateException();
        }

        let input = input.as_ref().unwrap();

        let inputWidth = input.getWidth() as i32;
        let inputHeight = input.getHeight() as i32;
        let qrWidth = inputWidth + (quietZone * 2);
        let qrHeight = inputHeight + (quietZone * 2);
        let outputWidth = width.max(qrWidth);
        let outputHeight = height.max(qrHeight);

        let multiple = (outputWidth / qrWidth).min(outputHeight / qrHeight);
        // Padding includes both the quiet zone and the extra white pixels to accommodate the requested
        // dimensions. For example, if input is 25x25 the QR will be 33x33 including the quiet zone.
        // If the requested size is 200x160, the multiple will be 4, for a QR of 132x132. These will
        // handle all the padding from 100x100 (the actual QR) up to 200x160.
        let leftPadding = (outputWidth - (inputWidth * multiple)) / 2;
        let topPadding = (outputHeight - (inputHeight * multiple)) / 2;

        let mut output = BitMatrix::new(outputWidth as u32, outputHeight as u32)?;

        let mut inputY = 0;
        let mut outputY = topPadding;
        while inputY < inputHeight {
            // for (int inputY = 0, outputY = topPadding; inputY < inputHeight; inputY++, outputY += multiple) {
            // Write the contents of this row of the barcode
            let mut inputX = 0;
            let mut outputX = leftPadding;
            while inputX < inputWidth {
                // for (int inputX = 0, outputX = leftPadding; inputX < inputWidth; inputX++, outputX += multiple) {
                if input.get(inputX as u32, inputY as u32) == 1 {
                    output.setRegion(
                        outputX as u32,
                        outputY as u32,
                        multiple as u32,
                        multiple as u32,
                    );
                }

                inputX += 1;
                outputX += multiple;
            }

            inputY += 1;
            outputY += multiple;
        }

        Ok(output)
    }
}
