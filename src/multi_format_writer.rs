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

use crate::{aztec::AztecWriter, qrcode::QRCodeWriter, BarcodeFormat, Exceptions, Writer};

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
            BarcodeFormat::EAN_8 => unimplemented!(""),
            // writer =  EAN8Writer(),
            BarcodeFormat::UPC_E => unimplemented!(""),
            // writer =  UPCEWriter(),
            BarcodeFormat::EAN_13 => unimplemented!(""),
            // writer =  EAN13Writer(),
            BarcodeFormat::UPC_A => unimplemented!(""),
            // writer =  UPCAWriter(),
            BarcodeFormat::QR_CODE => Box::new(QRCodeWriter {}),
            BarcodeFormat::CODE_39 => unimplemented!(""),
            // writer =  Code39Writer(),
            BarcodeFormat::CODE_93 => unimplemented!(""),
            // writer =  Code93Writer(),
            BarcodeFormat::CODE_128 => unimplemented!(""),
            // writer =  Code128Writer(),
            BarcodeFormat::ITF => unimplemented!(""),
            // writer =  ITFWriter(),
            BarcodeFormat::PDF_417 => unimplemented!(""),
            // writer =  PDF417Writer(),
            BarcodeFormat::CODABAR => unimplemented!(""),
            // writer =  CodaBarWriter(),
            BarcodeFormat::DATA_MATRIX => unimplemented!(""),
            // DataMatrixWriter{},
            BarcodeFormat::AZTEC => Box::new(AztecWriter {}),
            _ => {
                return Err(Exceptions::IllegalArgumentException(format!(
                    "No encoder available for format {:?}",
                    format
                )))
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
