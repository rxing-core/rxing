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
// package com::google::zxing::oned::rss::expanded::decoders;

/**
 * @author Pablo OrduÃ±a, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */

 const TWO_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();

 const THREE_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();

 const THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();

 const FOUR_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();
struct FieldParser {
}

impl FieldParser {

    static {
        TWO_DIGIT_DATA_LENGTH::put("00", &DataLength::fixed(18));
        TWO_DIGIT_DATA_LENGTH::put("01", &DataLength::fixed(14));
        TWO_DIGIT_DATA_LENGTH::put("02", &DataLength::fixed(14));
        TWO_DIGIT_DATA_LENGTH::put("10", &DataLength::variable(20));
        TWO_DIGIT_DATA_LENGTH::put("11", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("12", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("13", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("15", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("17", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("20", &DataLength::fixed(2));
        TWO_DIGIT_DATA_LENGTH::put("21", &DataLength::variable(20));
        TWO_DIGIT_DATA_LENGTH::put("22", &DataLength::variable(29));
        TWO_DIGIT_DATA_LENGTH::put("30", &DataLength::variable(8));
        TWO_DIGIT_DATA_LENGTH::put("37", &DataLength::variable(8));
        //internal company codes
         {
             let mut i: i32 = 90;
            while i <= 99 {
                {
                    TWO_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::variable(30));
                }
                i += 1;
             }
         }

    }

    static {
        THREE_DIGIT_DATA_LENGTH::put("240", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("241", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("242", &DataLength::variable(6));
        THREE_DIGIT_DATA_LENGTH::put("250", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("251", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("253", &DataLength::variable(17));
        THREE_DIGIT_DATA_LENGTH::put("254", &DataLength::variable(20));
        THREE_DIGIT_DATA_LENGTH::put("400", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("401", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("402", &DataLength::fixed(17));
        THREE_DIGIT_DATA_LENGTH::put("403", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("410", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("411", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("412", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("413", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("414", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("420", &DataLength::variable(20));
        THREE_DIGIT_DATA_LENGTH::put("421", &DataLength::variable(15));
        THREE_DIGIT_DATA_LENGTH::put("422", &DataLength::fixed(3));
        THREE_DIGIT_DATA_LENGTH::put("423", &DataLength::variable(15));
        THREE_DIGIT_DATA_LENGTH::put("424", &DataLength::fixed(3));
        THREE_DIGIT_DATA_LENGTH::put("425", &DataLength::fixed(3));
        THREE_DIGIT_DATA_LENGTH::put("426", &DataLength::fixed(3));
    }

    static {
         {
             let mut i: i32 = 310;
            while i <= 316 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 320;
            while i <= 336 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 340;
            while i <= 357 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 360;
            while i <= 369 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("390", &DataLength::variable(15));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("391", &DataLength::variable(18));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("392", &DataLength::variable(15));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("393", &DataLength::variable(18));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("703", &DataLength::variable(30));
    }

    static {
        FOUR_DIGIT_DATA_LENGTH::put("7001", &DataLength::fixed(13));
        FOUR_DIGIT_DATA_LENGTH::put("7002", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("7003", &DataLength::fixed(10));
        FOUR_DIGIT_DATA_LENGTH::put("8001", &DataLength::fixed(14));
        FOUR_DIGIT_DATA_LENGTH::put("8002", &DataLength::variable(20));
        FOUR_DIGIT_DATA_LENGTH::put("8003", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("8004", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("8005", &DataLength::fixed(6));
        FOUR_DIGIT_DATA_LENGTH::put("8006", &DataLength::fixed(18));
        FOUR_DIGIT_DATA_LENGTH::put("8007", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("8008", &DataLength::variable(12));
        FOUR_DIGIT_DATA_LENGTH::put("8018", &DataLength::fixed(18));
        FOUR_DIGIT_DATA_LENGTH::put("8020", &DataLength::variable(25));
        FOUR_DIGIT_DATA_LENGTH::put("8100", &DataLength::fixed(6));
        FOUR_DIGIT_DATA_LENGTH::put("8101", &DataLength::fixed(10));
        FOUR_DIGIT_DATA_LENGTH::put("8102", &DataLength::fixed(2));
        FOUR_DIGIT_DATA_LENGTH::put("8110", &DataLength::variable(70));
        FOUR_DIGIT_DATA_LENGTH::put("8200", &DataLength::variable(70));
    }

    fn new() -> FieldParser {
    }

    fn  parse_fields_in_general_purpose( raw_information: &String) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if raw_information.is_empty() {
            return Ok(null);
        }
        if raw_information.length() < 2 {
            throw NotFoundException::get_not_found_instance();
        }
         let two_digit_data_length: DataLength = TWO_DIGIT_DATA_LENGTH::get(&raw_information.substring(0, 2));
        if two_digit_data_length != null {
            if two_digit_data_length.variable {
                return Ok(::process_variable_a_i(2, two_digit_data_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(2, two_digit_data_length.len(), &raw_information));
        }
        if raw_information.length() < 3 {
            throw NotFoundException::get_not_found_instance();
        }
         let first_three_digits: String = raw_information.substring(0, 3);
         let three_digit_data_length: DataLength = THREE_DIGIT_DATA_LENGTH::get(&first_three_digits);
        if three_digit_data_length != null {
            if three_digit_data_length.variable {
                return Ok(::process_variable_a_i(3, three_digit_data_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(3, three_digit_data_length.len(), &raw_information));
        }
        if raw_information.length() < 4 {
            throw NotFoundException::get_not_found_instance();
        }
         let three_digit_plus_digit_data_length: DataLength = THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::get(&first_three_digits);
        if three_digit_plus_digit_data_length != null {
            if three_digit_plus_digit_data_length.variable {
                return Ok(::process_variable_a_i(4, three_digit_plus_digit_data_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(4, three_digit_plus_digit_data_length.len(), &raw_information));
        }
         let first_four_digit_length: DataLength = FOUR_DIGIT_DATA_LENGTH::get(&raw_information.substring(0, 4));
        if first_four_digit_length != null {
            if first_four_digit_length.variable {
                return Ok(::process_variable_a_i(4, first_four_digit_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(4, first_four_digit_length.len(), &raw_information));
        }
        throw NotFoundException::get_not_found_instance();
    }

    fn  process_fixed_a_i( ai_size: i32,  field_size: i32,  raw_information: &String) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if raw_information.length() < ai_size {
            throw NotFoundException::get_not_found_instance();
        }
         let ai: String = raw_information.substring(0, ai_size);
        if raw_information.length() < ai_size + field_size {
            throw NotFoundException::get_not_found_instance();
        }
         let field: String = raw_information.substring(ai_size, ai_size + field_size);
         let remaining: String = raw_information.substring(ai_size + field_size);
         let result: String = format!("({}){}", ai, field);
         let parsed_a_i: String = ::parse_fields_in_general_purpose(&remaining);
        return Ok( if parsed_a_i == null { result } else { format!("{}{}", result, parsed_a_i) });
    }

    fn  process_variable_a_i( ai_size: i32,  variable_field_size: i32,  raw_information: &String) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
         let ai: String = raw_information.substring(0, ai_size);
         let max_size: i32 = Math::min(&raw_information.length(), ai_size + variable_field_size);
         let field: String = raw_information.substring(ai_size, max_size);
         let remaining: String = raw_information.substring(max_size);
         let result: String = format!("({}){}", ai, field);
         let parsed_a_i: String = ::parse_fields_in_general_purpose(&remaining);
        return Ok( if parsed_a_i == null { result } else { format!("{}{}", result, parsed_a_i) });
    }

    struct DataLength {

         let variable: bool;

         let length: i32;
    }
    
    impl DataLength {

        fn new( variable: bool,  length: i32) -> DataLength {
            let .variable = variable;
            let .len() = length;
        }

        fn  fixed( length: i32) -> DataLength  {
            return DataLength::new(false, length);
        }

        fn  variable( length: i32) -> DataLength  {
            return DataLength::new(true, length);
        }
    }

}

