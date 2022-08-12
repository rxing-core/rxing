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
 * <p>Implements decoding of the EAN-13 format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */

// For an EAN-13 barcode, the first digit is represented by the parities used
// to encode the next six digits, according to the table below. For example,
// if the barcode is 5 123456 789012 then the value of the first digit is
// signified by using odd for '1', even for '2', even for '3', odd for '4',
// odd for '5', and even for '6'. See http://en.wikipedia.org/wiki/EAN-13
//
//                Parity of next 6 digits
//    Digit   0     1     2     3     4     5
//       0    Odd   Odd   Odd   Odd   Odd   Odd
//       1    Odd   Odd   Even  Odd   Even  Even
//       2    Odd   Odd   Even  Even  Odd   Even
//       3    Odd   Odd   Even  Even  Even  Odd
//       4    Odd   Even  Odd   Odd   Even  Even
//       5    Odd   Even  Even  Odd   Odd   Even
//       6    Odd   Even  Even  Even  Odd   Odd
//       7    Odd   Even  Odd   Even  Odd   Even
//       8    Odd   Even  Odd   Even  Even  Odd
//       9    Odd   Even  Even  Odd   Even  Odd
//
// Note that the encoding for '0' uses the same parity as a UPC barcode. Hence
// a UPC barcode can be converted to an EAN-13 barcode by prepending a 0.
//
// The encoding is represented by the following array, which is a bit pattern
// using Odd = 0 and Even = 1. For example, 5 is represented by:
//
//              Odd Even Even Odd Odd Even
// in binary:
//                0    1    1   0   0    1   == 0x19
//
 const FIRST_DIGIT_ENCODINGS: vec![Vec<i32>; 10] = vec![0x00, 0x0B, 0x0D, 0xE, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A, ]
;
pub struct EAN13Reader {
    super: UPCEANReader;

     let decode_middle_counters: Vec<i32>;
}

impl EAN13Reader {

    pub fn new() -> EAN13Reader {
        decode_middle_counters = : [i32; 4] = [0; 4];
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.decode_middle_counters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let end: i32 = row.get_size();
         let row_offset: i32 = start_range[1];
         let lg_pattern_found: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < 6 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_AND_G_PATTERNS);
                    result_string.append(('0' + best_match % 10) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                    if best_match >= 10 {
                        lg_pattern_found |= 1 << (5 - x);
                    }
                }
                x += 1;
             }
         }

        ::determine_first_digit(&result_string, lg_pattern_found);
         let middle_range: Vec<i32> = find_guard_pattern(row, row_offset, true, MIDDLE_PATTERN);
        row_offset = middle_range[1];
         {
             let mut x: i32 = 0;
            while x < 6 && row_offset < end {
                {
                     let best_match: i32 = decode_digit(row, &counters, row_offset, L_PATTERNS);
                    result_string.append(('0' + best_match) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                }
                x += 1;
             }
         }

        return Ok(row_offset);
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::EAN_13;
    }

    /**
   * Based on pattern of odd-even ('L' and 'G') patterns used to encoded the explicitly-encoded
   * digits in a barcode, determines the implicitly encoded first digit and adds it to the
   * result string.
   *
   * @param resultString string to insert decoded first digit into
   * @param lgPatternFound int whose bits indicates the pattern of odd/even L/G patterns used to
   *  encode digits
   * @throws NotFoundException if first digit cannot be determined
   */
    fn  determine_first_digit( result_string: &StringBuilder,  lg_pattern_found: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         {
             let mut d: i32 = 0;
            while d < 10 {
                {
                    if lg_pattern_found == FIRST_DIGIT_ENCODINGS[d] {
                        result_string.insert(0, ('0' + d) as char);
                        return;
                    }
                }
                d += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }
}

