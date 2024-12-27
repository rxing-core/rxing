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
    aztec::AztecWriter,
    common::Result,
    datamatrix::DataMatrixWriter,
    oned::{
        CodaBarWriter, Code128Writer, Code39Writer, Code93Writer, EAN13Writer, EAN8Writer,
        ITFWriter, TelepenWriter, UPCAWriter, UPCEWriter,
    },
    pdf417::PDF417Writer,
    qrcode::QRCodeWriter,
    BarcodeFormat, EncodeHints, Exceptions, Writer,
};

/**
 * This is a factory class which finds the appropriate Writer subclass for the BarcodeFormat
 * requested and encodes the barcode with the supplied contents.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Default)]
pub struct MultiFormatWriter;

impl Writer for MultiFormatWriter {
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
        let writer: Box<dyn Writer> = match format {
            BarcodeFormat::EAN_8 => Box::<EAN8Writer>::default(),
            BarcodeFormat::UPC_E => Box::<UPCEWriter>::default(),
            BarcodeFormat::EAN_13 => Box::<EAN13Writer>::default(),
            BarcodeFormat::UPC_A => Box::<UPCAWriter>::default(),
            BarcodeFormat::QR_CODE => Box::<QRCodeWriter>::default(),
            BarcodeFormat::CODE_39 => Box::<Code39Writer>::default(),
            BarcodeFormat::CODE_93 => Box::<Code93Writer>::default(),
            BarcodeFormat::CODE_128 => Box::<Code128Writer>::default(),
            BarcodeFormat::ITF => Box::<ITFWriter>::default(),
            BarcodeFormat::PDF_417 => Box::<PDF417Writer>::default(),
            BarcodeFormat::CODABAR => Box::<CodaBarWriter>::default(),
            BarcodeFormat::DATA_MATRIX => Box::<DataMatrixWriter>::default(),
            BarcodeFormat::TELEPEN => Box::<TelepenWriter>::default(),
            BarcodeFormat::AZTEC => Box::<AztecWriter>::default(),
            _ => {
                return Err(Exceptions::illegal_argument_with(format!(
                    "No encoder available for format {format:?}"
                )))
            }
        };

        writer.encode_with_hints(contents, format, width, height, hints)
    }
}
