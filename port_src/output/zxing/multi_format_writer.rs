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
// package com::google::zxing;

/**
 * This is a factory class which finds the appropriate Writer subclass for the BarcodeFormat
 * requested and encodes the barcode with the supplied contents.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Writer)]
pub struct MultiFormatWriter {
}

impl MultiFormatWriter {

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> /*  throws WriterException */Result<BitMatrix, Rc<Exception>>   {
        return Ok(self.encode(&contents, format, width, height, null));
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> /*  throws WriterException */Result<BitMatrix, Rc<Exception>>   {
         let mut writer: Writer;
        match format {
              EAN_8 => 
                 {
                    writer = EAN8Writer::new();
                    break;
                }
              UPC_E => 
                 {
                    writer = UPCEWriter::new();
                    break;
                }
              EAN_13 => 
                 {
                    writer = EAN13Writer::new();
                    break;
                }
              UPC_A => 
                 {
                    writer = UPCAWriter::new();
                    break;
                }
              QR_CODE => 
                 {
                    writer = QRCodeWriter::new();
                    break;
                }
              CODE_39 => 
                 {
                    writer = Code39Writer::new();
                    break;
                }
              CODE_93 => 
                 {
                    writer = Code93Writer::new();
                    break;
                }
              CODE_128 => 
                 {
                    writer = Code128Writer::new();
                    break;
                }
              ITF => 
                 {
                    writer = ITFWriter::new();
                    break;
                }
              PDF_417 => 
                 {
                    writer = PDF417Writer::new();
                    break;
                }
              CODABAR => 
                 {
                    writer = CodaBarWriter::new();
                    break;
                }
              DATA_MATRIX => 
                 {
                    writer = DataMatrixWriter::new();
                    break;
                }
              AZTEC => 
                 {
                    writer = AztecWriter::new();
                    break;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("No encoder available for format {}", format));
                }
        }
        return Ok(writer.encode(&contents, format, width, height, &hints));
    }
}

