/*
 * Copyright (C) 2010 ZXing authors
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
 * @see UPCEANExtension2Support
 */

 const CHECK_DIGIT_ENCODINGS: vec![Vec<i32>; 10] = vec![0x18, 0x14, 0x12, 0x11, 0x0C, 0x06, 0x03, 0x0A, 0x09, 0x05, ]
;
struct UPCEANExtension5Support {

     let decode_middle_counters: [i32; 4] = [0; 4];

     let decode_row_string_buffer: StringBuilder = StringBuilder::new();
}

impl UPCEANExtension5Support {

    fn  decode_row(&self,  row_number: i32,  row: &BitArray,  extension_start_range: &Vec<i32>) -> /*  throws NotFoundException */Result<Result, Rc<Exception>>   {
         let result: StringBuilder = self.decode_row_string_buffer;
        result.set_length(0);
         let end: i32 = self.decode_middle(row, &extension_start_range, &result);
         let result_string: String = result.to_string();
         let extension_data: Map<ResultMetadataType, Object> = ::parse_extension_string(&result_string);
         let extension_result: Result = Result::new(&result_string, null,  : vec![ResultPoint; 2] = vec![ResultPoint::new((extension_start_range[0] + extension_start_range[1]) / 2.0f, row_number), ResultPoint::new(end, row_number), ]
        , BarcodeFormat::UPC_EAN_EXTENSION);
        if extension_data != null {
            extension_result.put_all_metadata(&extension_data);
        }
        return Ok(extension_result);
    }

    fn  decode_middle(&self,  row: &BitArray,  start_range: &Vec<i32>,  result_string: &StringBuilder) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
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
            while x < 5 && row_offset < end {
                {
                     let best_match: i32 = UPCEANReader::decode_digit(row, &counters, row_offset, UPCEANReader.L_AND_G_PATTERNS);
                    result_string.append(('0' + best_match % 10) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                    if best_match >= 10 {
                        lg_pattern_found |= 1 << (4 - x);
                    }
                    if x != 4 {
                        // Read off separator if not last
                        row_offset = row.get_next_set(row_offset);
                        row_offset = row.get_next_unset(row_offset);
                    }
                }
                x += 1;
             }
         }

        if result_string.length() != 5 {
            throw NotFoundException::get_not_found_instance();
        }
         let check_digit: i32 = ::determine_check_digit(lg_pattern_found);
        if ::extension_checksum(&result_string.to_string()) != check_digit {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(row_offset);
    }

    fn  extension_checksum( s: &CharSequence) -> i32  {
         let length: i32 = s.length();
         let mut sum: i32 = 0;
         {
             let mut i: i32 = length - 2;
            while i >= 0 {
                {
                    sum += s.char_at(i) - '0';
                }
                i -= 2;
             }
         }

        sum *= 3;
         {
             let mut i: i32 = length - 1;
            while i >= 0 {
                {
                    sum += s.char_at(i) - '0';
                }
                i -= 2;
             }
         }

        sum *= 3;
        return sum % 10;
    }

    fn  determine_check_digit( lg_pattern_found: i32) -> /*  throws NotFoundException */Result<i32, Rc<Exception>>   {
         {
             let mut d: i32 = 0;
            while d < 10 {
                {
                    if lg_pattern_found == CHECK_DIGIT_ENCODINGS[d] {
                        return Ok(d);
                    }
                }
                d += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    /**
   * @param raw raw content of extension
   * @return formatted interpretation of raw content as a {@link Map} mapping
   *  one {@link ResultMetadataType} to appropriate value, or {@code null} if not known
   */
    fn  parse_extension_string( raw: &String) -> Map<ResultMetadataType, Object>  {
        if raw.length() != 5 {
            return null;
        }
         let value: Object = ::parse_extension5_string(&raw);
        if value == null {
            return null;
        }
         let result: Map<ResultMetadataType, Object> = EnumMap<>::new(ResultMetadataType.class);
        result.put(ResultMetadataType::SUGGESTED_PRICE, &value);
        return result;
    }

    fn  parse_extension5_string( raw: &String) -> String  {
         let mut currency: String;
        match raw.char_at(0) {
              '0' => 
                 {
                    currency = "Â£";
                    break;
                }
              '5' => 
                 {
                    currency = "$";
                    break;
                }
              '9' => 
                 {
                    // Reference: http://www.jollytech.com
                    match raw {
                          "90000" => 
                             {
                                // No suggested retail price
                                return null;
                            }
                          "99991" => 
                             {
                                // Complementary
                                return "0.00";
                            }
                          "99990" => 
                             {
                                return "Used";
                            }
                    }
                    // Otherwise... unknown currency?
                    currency = "";
                    break;
                }
            _ => 
                 {
                    currency = "";
                    break;
                }
        }
         let raw_amount: i32 = Integer::parse_int(&raw.substring(1));
         let units_string: String = String::value_of(raw_amount / 100);
         let hundredths: i32 = raw_amount % 100;
         let hundredths_string: String =  if hundredths < 10 { format!("0{}", hundredths) } else { String::value_of(hundredths) };
        return format!("{}{}.{}", currency, units_string, hundredths_string);
    }
}

