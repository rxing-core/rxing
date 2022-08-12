/*
 * Copyright (C) 2012 ZXing authors
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
 * @see UPCEANExtension5Support
 */
struct UPCEANExtension2Support {

     let decode_middle_counters: [i32; 4] = [0; 4];

     let decode_row_string_buffer: StringBuilder = StringBuilder::new();
}

impl UPCEANExtension2Support {

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
         let check_parity: i32 = 0;
         {
             let mut x: i32 = 0;
            while x < 2 && row_offset < end {
                {
                     let best_match: i32 = UPCEANReader::decode_digit(row, &counters, row_offset, UPCEANReader.L_AND_G_PATTERNS);
                    result_string.append(('0' + best_match % 10) as char);
                    for  let counter: i32 in counters {
                        row_offset += counter;
                    }
                    if best_match >= 10 {
                        check_parity |= 1 << (1 - x);
                    }
                    if x != 1 {
                        // Read off separator if not last
                        row_offset = row.get_next_set(row_offset);
                        row_offset = row.get_next_unset(row_offset);
                    }
                }
                x += 1;
             }
         }

        if result_string.length() != 2 {
            throw NotFoundException::get_not_found_instance();
        }
        if Integer::parse_int(&result_string.to_string()) % 4 != check_parity {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(row_offset);
    }

    /**
   * @param raw raw content of extension
   * @return formatted interpretation of raw content as a {@link Map} mapping
   *  one {@link ResultMetadataType} to appropriate value, or {@code null} if not known
   */
    fn  parse_extension_string( raw: &String) -> Map<ResultMetadataType, Object>  {
        if raw.length() != 2 {
            return null;
        }
         let result: Map<ResultMetadataType, Object> = EnumMap<>::new(ResultMetadataType.class);
        result.put(ResultMetadataType::ISSUE_NUMBER, &Integer::value_of(&raw));
        return result;
    }
}

