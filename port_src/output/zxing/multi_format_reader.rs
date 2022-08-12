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
// package com::google::zxing;

/**
 * MultiFormatReader is a convenience class and the main entry point into the library for most uses.
 * By default it attempts to decode all barcode formats that the library supports. Optionally, you
 * can provide a hints object to request different behavior, for example only decoding QR codes.
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 */

 const EMPTY_READER_ARRAY: [Option<Reader>; 0] = [None; 0];
#[derive(Reader)]
pub struct MultiFormatReader {

     let hints: Map<DecodeHintType, ?>;

     let readers: Vec<Reader>;
}

impl MultiFormatReader {

    /**
   * This version of decode honors the intent of Reader.decode(BinaryBitmap) in that it
   * passes null as a hint to the decoders. However, that makes it inefficient to call repeatedly.
   * Use setHints() followed by decodeWithState() for continuous scan applications.
   *
   * @param image The pixel data to decode
   * @return The contents of the image
   * @throws NotFoundException Any errors which occurred
   */
    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        self.set_hints(null);
        return Ok(self.decode_internal(image));
    }

    /**
   * Decode an image using the hints provided. Does not honor existing state.
   *
   * @param image The pixel data to decode
   * @param hints The hints to use, clearing the previous state.
   * @return The contents of the image
   * @throws NotFoundException Any errors which occurred
   */
    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        self.set_hints(&hints);
        return Ok(self.decode_internal(image));
    }

    /**
   * Decode an image using the state set up by calling setHints() previously. Continuous scan
   * clients will get a <b>large</b> speed increase by using this instead of decode().
   *
   * @param image The pixel data to decode
   * @return The contents of the image
   * @throws NotFoundException Any errors which occurred
   */
    pub fn  decode_with_state(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        // Make sure to set up the default state so we don't crash
        if self.readers == null {
            self.set_hints(null);
        }
        return Ok(self.decode_internal(image));
    }

    /**
   * This method adds state to the MultiFormatReader. By setting the hints once, subsequent calls
   * to decodeWithState(image) can reuse the same set of readers without reallocating memory. This
   * is important for performance in continuous scan clients.
   *
   * @param hints The set of hints to use for subsequent calls to decode(image)
   */
    pub fn  set_hints(&self,  hints: &Map<DecodeHintType, ?>)   {
        self.hints = hints;
         let try_harder: bool = hints != null && hints.contains_key(DecodeHintType::TRY_HARDER);
         let formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
         let mut readers: Collection<Reader> = ArrayList<>::new();
        if formats != null {
             let add_one_d_reader: bool = formats.contains(BarcodeFormat::UPC_A) || formats.contains(BarcodeFormat::UPC_E) || formats.contains(BarcodeFormat::EAN_13) || formats.contains(BarcodeFormat::EAN_8) || formats.contains(BarcodeFormat::CODABAR) || formats.contains(BarcodeFormat::CODE_39) || formats.contains(BarcodeFormat::CODE_93) || formats.contains(BarcodeFormat::CODE_128) || formats.contains(BarcodeFormat::ITF) || formats.contains(BarcodeFormat::RSS_14) || formats.contains(BarcodeFormat::RSS_EXPANDED);
            // Put 1D readers upfront in "normal" mode
            if add_one_d_reader && !try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
            if formats.contains(BarcodeFormat::QR_CODE) {
                readers.add(QRCodeReader::new());
            }
            if formats.contains(BarcodeFormat::DATA_MATRIX) {
                readers.add(DataMatrixReader::new());
            }
            if formats.contains(BarcodeFormat::AZTEC) {
                readers.add(AztecReader::new());
            }
            if formats.contains(BarcodeFormat::PDF_417) {
                readers.add(PDF417Reader::new());
            }
            if formats.contains(BarcodeFormat::MAXICODE) {
                readers.add(MaxiCodeReader::new());
            }
            // At end in "try harder" mode
            if add_one_d_reader && try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
        }
        if readers.is_empty() {
            if !try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
            readers.add(QRCodeReader::new());
            readers.add(DataMatrixReader::new());
            readers.add(AztecReader::new());
            readers.add(PDF417Reader::new());
            readers.add(MaxiCodeReader::new());
            if try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
        }
        self.readers = readers.to_array(EMPTY_READER_ARRAY);
    }

    pub fn  reset(&self)   {
        if self.readers != null {
            for  let reader: Reader in self.readers {
                reader.reset();
            }
        }
    }

    fn  decode_internal(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        if self.readers != null {
            for  let reader: Reader in self.readers {
                if Thread::current_thread()::is_interrupted() {
                    throw NotFoundException::get_not_found_instance();
                }
                let tryResult1 = 0;
                'try1: loop {
                {
                    return Ok(reader.decode(image, &self.hints));
                }
                break 'try1
                }
                match tryResult1 {
                     catch ( re: &ReaderException) {
                    }  0 => break
                }

            }
            if self.hints != null && self.hints.contains_key(DecodeHintType::ALSO_INVERTED) {
                // Calling all readers again with inverted image
                image.get_black_matrix().flip();
                for  let reader: Reader in self.readers {
                    if Thread::current_thread()::is_interrupted() {
                        throw NotFoundException::get_not_found_instance();
                    }
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        return Ok(reader.decode(image, &self.hints));
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( re: &ReaderException) {
                        }  0 => break
                    }

                }
            }
        }
        throw NotFoundException::get_not_found_instance();
    }
}

