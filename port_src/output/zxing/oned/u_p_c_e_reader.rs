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
 * <p>Implements decoding of the UPC-E format.</p>
 * <p><a href="http://www.barcodeisland.com/upce.phtml">This</a> is a great reference for
 * UPC-E information.</p>
 *
 * @author Sean Owen
 */

/**
   * The pattern that marks the middle, and end, of a UPC-E pattern.
   * There is no "second half" to a UPC-E barcode.
   */
 const MIDDLE_END_PATTERN: vec![Vec<i32>; 6] = vec![1, 1, 1, 1, 1, 1, ]
;

// For an UPC-E barcode, the final digit is represented by the parities used
// to encode the middle six digits, according to the table below.
//
//                Parity of next 6 digits
//    Digit   0     1     2     3     4     5
//       0    Even   Even  Even Odd  Odd   Odd
//       1    Even   Even  Odd  Even Odd   Odd
//       2    Even   Even  Odd  Odd  Even  Odd
//       3    Even   Even  Odd  Odd  Odd   Even
//       4    Even   Odd   Even Even Odd   Odd
//       5    Even   Odd   Odd  Even Even  Odd
//       6    Even   Odd   Odd  Odd  Even  Even
//       7    Even   Odd   Even Odd  Even  Odd
//       8    Even   Odd   Even Odd  Odd   Even
//       9    Even   Odd   Odd  Even Odd   Even
//
// The encoding is represented by the following array, which is a bit pattern
// using Odd = 0 and Even = 1. For example, 5 is represented by:
//
//              Odd Even Even Odd Odd Even
// in binary:
//                0    1    1   0   0    1   == 0x19
//
/**
   * See {@link #L_AND_G_PATTERNS}; these values similarly represent patterns of
   * even-odd parity encodings of digits that imply both the number system (0 or 1)
   * used, and the check digit.
   */
 const NUMSYS_AND_CHECK_DIGIT_PATTERNS: vec![vec![Vec<Vec<i32>>; 10]; 2] = vec![vec![0x38, 0x34, 0x32, 0x31, 0x2C, 0x26, 0x23, 0x2A, 0x29, 0x25, ]
, vec![0x07, 0x0B, 0x0D, 0x0E, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A, ]
, ]
;
pub struct UPCEReader {
    super: UPCEANReader;

     let decode_middle_counters: Vec<i32>;
}

impl UPCEReader {

    pub fn new() -> UPCEReader {
        decode_middle_counters = : [i32; 4] = [0; 4];
    }

    pub fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
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
                    result.append(('0' + best_match % 10) as char);
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

        ::determine_num_sys_and_check_digit(&result, lg_pattern_found);
        return Ok(row_offset);
    }

    pub fn  decode_end(&self,  row: &BitArray,  end_start: i32) -> /*  throws NotFoundException */Result<Vec<i32>, Rc<Exception>>   {
        return Ok(find_guard_pattern(row, end_start, true, &MIDDLE_END_PATTERN));
    }

    pub fn  check_checksum(&self,  s: &String) -> /*  throws FormatException */Result<bool, Rc<Exception>>   {
        return Ok(super.check_checksum(&::convert_u_p_c_eto_u_p_c_a(&s)));
    }

    fn  determine_num_sys_and_check_digit( result_string: &StringBuilder,  lg_pattern_found: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         {
             let num_sys: i32 = 0;
            while num_sys <= 1 {
                {
                     {
                         let mut d: i32 = 0;
                        while d < 10 {
                            {
                                if lg_pattern_found == NUMSYS_AND_CHECK_DIGIT_PATTERNS[num_sys][d] {
                                    result_string.insert(0, ('0' + num_sys) as char);
                                    result_string.append(('0' + d) as char);
                                    return;
                                }
                            }
                            d += 1;
                         }
                     }

                }
                num_sys += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    fn  get_barcode_format(&self) -> BarcodeFormat  {
        return BarcodeFormat::UPC_E;
    }

    /**
   * Expands a UPC-E value back into its full, equivalent UPC-A code value.
   *
   * @param upce UPC-E code as string of digits
   * @return equivalent UPC-A code as string of digits
   */
    pub fn  convert_u_p_c_eto_u_p_c_a( upce: &String) -> String  {
         let upce_chars: [Option<char>; 6] = [None; 6];
        upce.get_chars(1, 7, &upce_chars, 0);
         let result: StringBuilder = StringBuilder::new(12);
        result.append(&upce.char_at(0));
         let last_char: char = upce_chars[5];
        match last_char {
              '0' => 
                 {
                }
              '1' => 
                 {
                }
              '2' => 
                 {
                    result.append(&upce_chars, 0, 2);
                    result.append(last_char);
                    result.append("0000");
                    result.append(&upce_chars, 2, 3);
                    break;
                }
              '3' => 
                 {
                    result.append(&upce_chars, 0, 3);
                    result.append("00000");
                    result.append(&upce_chars, 3, 2);
                    break;
                }
              '4' => 
                 {
                    result.append(&upce_chars, 0, 4);
                    result.append("00000");
                    result.append(upce_chars[4]);
                    break;
                }
            _ => 
                 {
                    result.append(&upce_chars, 0, 5);
                    result.append("0000");
                    result.append(last_char);
                    break;
                }
        }
        // Only append check digit in conversion if supplied
        if upce.length() >= 8 {
            result.append(&upce.char_at(7));
        }
        return result.to_string();
    }
}

