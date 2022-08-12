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
 * <p>Implements decoding of the UPC-A format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
pub struct UPCAReader {
    super: UPCEANReader;

     let ean13_reader: UPCEANReader = EAN13Reader::new();
}

impl UPCAReader {

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  start_guard_range: &Vec<i32>,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode_row(row_number, row, &start_guard_range, &hints)));
    }

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode_row(row_number, row, &hints)));
    }

    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode(image)));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(::maybe_return_result(&self.ean13_reader.decode(image, &hints)));
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::UPC_A;
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
        return Ok(self.ean13_reader.decode_middle(row, &start_range, &result_string));
    }

    fn  maybe_return_result( result: &Result) -> /*  throws FormatException */Result<Result, Rc<Exception>>   {
         let text: String = result.get_text();
        if text.char_at(0) == '0' {
             let upca_result: Result = Result::new(&text.substring(1), null, &result.get_result_points(), BarcodeFormat::UPC_A);
            if result.get_result_metadata() != null {
                upca_result.put_all_metadata(&result.get_result_metadata());
            }
            return Ok(upca_result);
        } else {
            throw FormatException::get_format_instance();
        }
    }
}

