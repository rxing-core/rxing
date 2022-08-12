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
// package com::google::zxing::oned;

/**
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */

 const EMPTY_ONED_ARRAY: [Option<OneDReader>; 0] = [None; 0];
pub struct MultiFormatOneDReader {
    super: OneDReader;

     let readers: Vec<OneDReader>;
}

impl MultiFormatOneDReader {

    pub fn new( hints: &Map<DecodeHintType, ?>) -> MultiFormatOneDReader {
         let possible_formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
         let use_code39_check_digit: bool = hints != null && hints.get(DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT) != null;
         let mut readers: Collection<OneDReader> = ArrayList<>::new();
        if possible_formats != null {
            if possible_formats.contains(BarcodeFormat::EAN_13) || possible_formats.contains(BarcodeFormat::UPC_A) || possible_formats.contains(BarcodeFormat::EAN_8) || possible_formats.contains(BarcodeFormat::UPC_E) {
                readers.add(MultiFormatUPCEANReader::new(&hints));
            }
            if possible_formats.contains(BarcodeFormat::CODE_39) {
                readers.add(Code39Reader::new(use_code39_check_digit));
            }
            if possible_formats.contains(BarcodeFormat::CODE_93) {
                readers.add(Code93Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::CODE_128) {
                readers.add(Code128Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::ITF) {
                readers.add(ITFReader::new());
            }
            if possible_formats.contains(BarcodeFormat::CODABAR) {
                readers.add(CodaBarReader::new());
            }
            if possible_formats.contains(BarcodeFormat::RSS_14) {
                readers.add(RSS14Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::RSS_EXPANDED) {
                readers.add(RSSExpandedReader::new());
            }
        }
        if readers.is_empty() {
            readers.add(MultiFormatUPCEANReader::new(&hints));
            readers.add(Code39Reader::new());
            readers.add(CodaBarReader::new());
            readers.add(Code93Reader::new());
            readers.add(Code128Reader::new());
            readers.add(ITFReader::new());
            readers.add(RSS14Reader::new());
            readers.add(RSSExpandedReader::new());
        }
        let .readers = readers.to_array(EMPTY_ONED_ARRAY);
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        for  let reader: OneDReader in self.readers {
            let tryResult1 = 0;
            'try1: loop {
            {
                return Ok(reader.decode_row(row_number, row, &hints));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( re: &ReaderException) {
                }  0 => break
            }

        }
        throw NotFoundException::get_not_found_instance();
    }

    pub fn  reset(&self)   {
        for  let reader: Reader in self.readers {
            reader.reset();
        }
    }
}

