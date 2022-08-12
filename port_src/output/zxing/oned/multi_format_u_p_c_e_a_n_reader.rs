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
 * <p>A reader that can read all available UPC/EAN formats. If a caller wants to try to
 * read all such formats, it is most efficient to use this implementation rather than invoke
 * individual readers.</p>
 *
 * @author Sean Owen
 */

 const EMPTY_READER_ARRAY: [Option<UPCEANReader>; 0] = [None; 0];
pub struct MultiFormatUPCEANReader {
    super: OneDReader;

     let readers: Vec<UPCEANReader>;
}

impl MultiFormatUPCEANReader {

    pub fn new( hints: &Map<DecodeHintType, ?>) -> MultiFormatUPCEANReader {
         let possible_formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
         let mut readers: Collection<UPCEANReader> = ArrayList<>::new();
        if possible_formats != null {
            if possible_formats.contains(BarcodeFormat::EAN_13) {
                readers.add(EAN13Reader::new());
            } else if possible_formats.contains(BarcodeFormat::UPC_A) {
                readers.add(UPCAReader::new());
            }
            if possible_formats.contains(BarcodeFormat::EAN_8) {
                readers.add(EAN8Reader::new());
            }
            if possible_formats.contains(BarcodeFormat::UPC_E) {
                readers.add(UPCEReader::new());
            }
        }
        if readers.is_empty() {
            readers.add(EAN13Reader::new());
            // UPC-A is covered by EAN-13
            readers.add(EAN8Reader::new());
            readers.add(UPCEReader::new());
        }
        let .readers = readers.to_array(EMPTY_READER_ARRAY);
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
        // Compute this location once and reuse it on multiple implementations
         let start_guard_pattern: Vec<i32> = UPCEANReader::find_start_guard_pattern(row);
        for  let reader: UPCEANReader in self.readers {
            let tryResult1 = 0;
            'try1: loop {
            {
                 let result: Result = reader.decode_row(row_number, row, &start_guard_pattern, &hints);
                // Special case: a 12-digit code encoded in UPC-A is identical to a "0"
                // followed by those 12 digits encoded as EAN-13. Each will recognize such a code,
                // UPC-A as a 12-digit string and EAN-13 as a 13-digit string starting with "0".
                // Individually these are correct and their readers will both read such a code
                // and correctly call it EAN-13, or UPC-A, respectively.
                //
                // In this case, if we've been looking for both types, we'd like to call it
                // a UPC-A code. But for efficiency we only run the EAN-13 decoder to also read
                // UPC-A. So we special case it here, and convert an EAN-13 result to a UPC-A
                // result if appropriate.
                //
                // But, don't return UPC-A if UPC-A was not a requested format!
                 let ean13_may_be_u_p_c_a: bool = result.get_barcode_format() == BarcodeFormat::EAN_13 && result.get_text().char_at(0) == '0';
                 let possible_formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
                 let can_return_u_p_c_a: bool = possible_formats == null || possible_formats.contains(BarcodeFormat::UPC_A);
                if ean13_may_be_u_p_c_a && can_return_u_p_c_a {
                    // Transfer the metadata across
                     let result_u_p_c_a: Result = Result::new(&result.get_text().substring(1), &result.get_raw_bytes(), &result.get_result_points(), BarcodeFormat::UPC_A);
                    result_u_p_c_a.put_all_metadata(&result.get_result_metadata());
                    return Ok(result_u_p_c_a);
                }
                return Ok(result);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( ignored: &ReaderException) {
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

