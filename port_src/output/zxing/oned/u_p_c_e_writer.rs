/*
 * Copyright 2009 ZXing authors
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
 * This object renders an UPC-E code as a {@link BitMatrix}.
 *
 * @author 0979097955s@gmail.com (RX)
 */

 const CODE_WIDTH: i32 = // start guard
3 + // bars
(7 * 6) + // end guard
6;
pub struct UPCEWriter {
    super: UPCEANWriter;
}

impl UPCEWriter {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::UPC_E);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let length: i32 = contents.length();
        match length {
              7 => 
                 {
                    // No check digit present, calculate it and add it
                     let mut check: i32;
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        check = UPCEANReader::get_standard_u_p_c_e_a_n_checksum(&UPCEReader::convert_u_p_c_eto_u_p_c_a(&contents));
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( fe: &FormatException) {
                            throw IllegalArgumentException::new(fe);
                        }  0 => break
                    }

                    contents += check;
                    break;
                }
              8 => 
                 {
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        if !UPCEANReader::check_standard_u_p_c_e_a_n_checksum(&UPCEReader::convert_u_p_c_eto_u_p_c_a(&contents)) {
                            throw IllegalArgumentException::new("Contents do not pass checksum");
                        }
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( ignored: &FormatException) {
                            throw IllegalArgumentException::new("Illegal contents");
                        }  0 => break
                    }

                    break;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("Requested contents should be 7 or 8 digits long, but got {}", length));
                }
        }
        check_numeric(&contents);
         let first_digit: i32 = Character::digit(&contents.char_at(0), 10);
        if first_digit != 0 && first_digit != 1 {
            throw IllegalArgumentException::new("Number system must be 0 or 1");
        }
         let check_digit: i32 = Character::digit(&contents.char_at(7), 10);
         let parities: i32 = UPCEReader.NUMSYS_AND_CHECK_DIGIT_PATTERNS[first_digit][check_digit];
         let result: [bool; CODE_WIDTH] = [false; CODE_WIDTH];
         let mut pos: i32 = append_pattern(&result, 0, UPCEANReader.START_END_PATTERN, true);
         {
             let mut i: i32 = 1;
            while i <= 6 {
                {
                     let mut digit: i32 = Character::digit(&contents.char_at(i), 10);
                    if (parities >> (6 - i) & 1) == 1 {
                        digit += 10;
                    }
                    pos += append_pattern(&result, pos, UPCEANReader.L_AND_G_PATTERNS[digit], false);
                }
                i += 1;
             }
         }

        append_pattern(&result, pos, UPCEANReader.END_PATTERN, false);
        return result;
    }
}

