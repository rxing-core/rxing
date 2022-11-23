/*
 * Copyright 2007 ZXing authors
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
    aztec::AztecReader, maxicode::MaxiCodeReader, qrcode::QRCodeReader, BarcodeFormat,
    BinaryBitmap, DecodeHintType, DecodeHintValue, DecodingHintDictionary, Exceptions, RXingResult,
    Reader,
};

/**
 * MultiFormatReader is a convenience class and the main entry point into the library for most uses.
 * By default it attempts to decode all barcode formats that the library supports. Optionally, you
 * can provide a hints object to request different behavior, for example only decoding QR codes.
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct MultiFormatReader {
    hints: DecodingHintDictionary,
    readers: Vec<Box<dyn Reader>>,
}

impl Reader for MultiFormatReader {
    /**
     * This version of decode honors the intent of Reader.decode(BinaryBitmap) in that it
     * passes null as a hint to the decoders. However, that makes it inefficient to call repeatedly.
     * Use setHints() followed by decodeWithState() for continuous scan applications.
     *
     * @param image The pixel data to decode
     * @return The contents of the image
     * @throws NotFoundException Any errors which occurred
     */
    fn decode(
        &mut self,
        image: &crate::BinaryBitmap,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        self.setHints(&HashMap::new());
        self.decodeInternal(image)
    }

    /**
     * Decode an image using the hints provided. Does not honor existing state.
     *
     * @param image The pixel data to decode
     * @param hints The hints to use, clearing the previous state.
     * @return The contents of the image
     * @throws NotFoundException Any errors which occurred
     */
    fn decode_with_hints(
        &mut self,
        image: &crate::BinaryBitmap,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        self.setHints(hints);
        self.decodeInternal(image)
    }

    fn reset(&mut self) {
        // if (readers != null) {
        for reader in self.readers.iter_mut() {
            reader.reset();
        }
        // }
    }
}

impl MultiFormatReader {
    const EMPTY_READER_ARRAY: Vec<Box<dyn Reader>> = Vec::new();

    /**
     * Decode an image using the state set up by calling setHints() previously. Continuous scan
     * clients will get a <b>large</b> speed increase by using this instead of decode().
     *
     * @param image The pixel data to decode
     * @return The contents of the image
     * @throws NotFoundException Any errors which occurred
     */
    pub fn decodeWithState(&mut self, image: &BinaryBitmap) -> Result<RXingResult, Exceptions> {
        // Make sure to set up the default state so we don't crash
        if self.readers.is_empty() {
            self.setHints(&HashMap::new());
        }
        self.decodeInternal(image)
    }

    /**
     * This method adds state to the MultiFormatReader. By setting the hints once, subsequent calls
     * to decodeWithState(image) can reuse the same set of readers without reallocating memory. This
     * is important for performance in continuous scan clients.
     *
     * @param hints The set of hints to use for subsequent calls to decode(image)
     */
    pub fn setHints(&mut self, hints: &DecodingHintDictionary) {
        self.hints = hints.clone(); // {hint} else {HashMap::new()};

        let tryHarder = self.hints.contains_key(&DecodeHintType::TRY_HARDER);
        //@SuppressWarnings("unchecked")
        let formats = hints.get(&DecodeHintType::POSSIBLE_FORMATS);
        let mut readers: Vec<Box<dyn Reader>> = Vec::new();
        if let Some(DecodeHintValue::PossibleFormats(formats)) = formats {
            let addOneDReader = formats.contains(&BarcodeFormat::UPC_A)
                || formats.contains(&BarcodeFormat::UPC_E)
                || formats.contains(&BarcodeFormat::EAN_13)
                || formats.contains(&BarcodeFormat::EAN_8)
                || formats.contains(&BarcodeFormat::CODABAR)
                || formats.contains(&BarcodeFormat::CODE_39)
                || formats.contains(&BarcodeFormat::CODE_93)
                || formats.contains(&BarcodeFormat::CODE_128)
                || formats.contains(&BarcodeFormat::ITF)
                || formats.contains(&BarcodeFormat::RSS_14)
                || formats.contains(&BarcodeFormat::RSS_EXPANDED);
            // Put 1D readers upfront in "normal" mode
            if addOneDReader && !tryHarder {
                unimplemented!("");
                // readers.push(new MultiFormatOneDReader(hints));
            }
            if formats.contains(&BarcodeFormat::QR_CODE) {
                readers.push(Box::new(QRCodeReader {}));
            }
            if formats.contains(&BarcodeFormat::DATA_MATRIX) {
                unimplemented!("");
                // readers.push(DataMatrixReader{});
            }
            if formats.contains(&BarcodeFormat::AZTEC) {
                readers.push(Box::new(AztecReader {}));
            }
            if formats.contains(&BarcodeFormat::PDF_417) {
                unimplemented!("");
                // readers.push(new PDF417Reader());
            }
            if formats.contains(&BarcodeFormat::MAXICODE) {
                readers.push(Box::new(MaxiCodeReader {}));
            }
            // At end in "try harder" mode
            if addOneDReader && tryHarder {
                unimplemented!("");
                // readers.push( MultiFormatOneDReader::new(hints));
            }
        }
        if readers.is_empty() {
            if !tryHarder {
                // readers.push( MultiFormatOneDReader::new(hints));
                unimplemented!("");
            }

            readers.push(Box::new(QRCodeReader {}));
            // readers.push( Box::new(DataMatrixReader{}));
            readers.push(Box::new(AztecReader {}));
            // readers.push( PDF417Reader());
            readers.push(Box::new(MaxiCodeReader {}));
            // unimplemented!("");

            if tryHarder {
                // readers.push( Box::new(MultiFormatOneDReader::new(hints)));
                unimplemented!("");
            }
        }
        self.readers = Vec::new(); //readers.toArray(EMPTY_READER_ARRAY);
    }

    pub fn decodeInternal(&mut self, image: &BinaryBitmap) -> Result<RXingResult, Exceptions> {
        if !self.readers.is_empty() {
            for reader in self.readers.iter_mut() {
                // I'm not sure how to model this in rust
                // if (Thread.currentThread().isInterrupted()) {
                //   throw NotFoundException.getNotFoundInstance();
                // }
                //try {
                let res = reader.decode_with_hints(image, &self.hints);
                if res.is_ok() {
                    return res;
                }
                //} catch (ReaderException re) {
                // continue
                //}
            }
            if self.hints.contains_key(&DecodeHintType::ALSO_INVERTED) {
                // Calling all readers again with inverted image
                let mut image = image.clone();
                image.getBlackMatrixMut().flip_self();
                for reader in self.readers.iter_mut() {
                    // if (Thread.currentThread().isInterrupted()) {
                    //   throw NotFoundException.getNotFoundInstance();
                    // }
                    let res = reader.decode_with_hints(&image, &self.hints);
                    if res.is_ok() {
                        return res;
                    }
                    // try {
                    //   return reader.decode(image, hints);
                    // } catch (ReaderException re) {
                    //   // continue
                    // }
                }
            }
        }
        return Err(Exceptions::NotFoundException("".to_owned()));
    }
}
