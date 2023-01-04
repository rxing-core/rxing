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
    datamatrix::DataMatrixWriter,
    oned::{
        Code128Writer, Code39Writer, Code93Writer, EAN13Writer, EAN8Writer, ITFWriter, UPCAWriter,
        UPCEWriter,
    },
    pdf417::PDF417Writer,
    qrcode::QRCodeWriter,
    BarcodeFormat, Exceptions, Writer,
};

/**
 * This is a factory class which finds the appropriate Writer subclass for the BarcodeFormat
 * requested and encodes the barcode with the supplied contents.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct MultiFormatWriter;

impl Writer for MultiFormatWriter {
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
            BarcodeFormat::CODABAR => Box::<Code128Writer>::default(),
            BarcodeFormat::DATA_MATRIX => Box::<DataMatrixWriter>::default(),
            BarcodeFormat::AZTEC => Box::<AztecWriter>::default(),
            _ => {
                return Err(Exceptions::IllegalArgumentException(Some(format!(
                    "No encoder available for format {:?}",
                    format
                ))))
            }
        };

        writer.encode_with_hints(contents, format, width, height, hints)
    }
}

//   @Override
//   public BitMatrix encode(String contents,
//                           BarcodeFormat format,
//                           int width,
//                           int height) throws WriterException {
//     return encode(contents, format, width, height, null);
//   }

//   @Override
//   public BitMatrix encode(String contents,
//                           BarcodeFormat format,
//                           int width, int height,
//                           Map<EncodeHintType,?> hints) throws WriterException {

//     Writer writer;
//     switch (format) {
//       case EAN_8:
//         writer = new EAN8Writer();
//         break;
//       case UPC_E:
//         writer = new UPCEWriter();
//         break;
//       case EAN_13:
//         writer = new EAN13Writer();
//         break;
//       case UPC_A:
//         writer = new UPCAWriter();
//         break;
//       case QR_CODE:
//         writer = new QRCodeWriter();
//         break;
//       case CODE_39:
//         writer = new Code39Writer();
//         break;
//       case CODE_93:
//         writer = new Code93Writer();
//         break;
//       case CODE_128:
//         writer = new Code128Writer();
//         break;
//       case ITF:
//         writer = new ITFWriter();
//         break;
//       case PDF_417:
//         writer = new PDF417Writer();
//         break;
//       case CODABAR:
//         writer = new CodaBarWriter();
//         break;
//       case DATA_MATRIX:
//         writer = new DataMatrixWriter();
//         break;
//       case AZTEC:
//         writer = new AztecWriter();
//         break;
//       default:
//         throw new IllegalArgumentException("No encoder available for format " + format);
//     }
//     return writer.encode(contents, format, width, height, hints);
//   }

// }
