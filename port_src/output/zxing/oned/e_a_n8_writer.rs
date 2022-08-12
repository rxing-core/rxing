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
 * This object renders an EAN8 code as a {@link BitMatrix}.
 *
 * @author aripollak@gmail.com (Ari Pollak)
 */

 const CODE_WIDTH: i32 = // start guard
3 + // left bars
(7 * 4) + // middle guard
5 + // right bars
(7 * 4) + // end guard
3;
pub struct EAN8Writer {
    super: UPCEANWriter;
}

impl EAN8Writer {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::EAN_8);
    }

    /**
   * @return a byte array of horizontal pixels (false = white, true = black)
   */
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
                        check = UPCEANReader::get_standard_u_p_c_e_a_n_checksum(&contents);
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
                        if !UPCEANReader::check_standard_u_p_c_e_a_n_checksum(&contents) {
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
         let result: [bool; CODE_WIDTH] = [false; CODE_WIDTH];
         let mut pos: i32 = 0;
        pos += append_pattern(&result, pos, UPCEANReader.START_END_PATTERN, true);
         {
             let mut i: i32 = 0;
            while i <= 3 {
                {
                     let digit: i32 = Character::digit(&contents.char_at(i), 10);
                    pos += append_pattern(&result, pos, UPCEANReader.L_PATTERNS[digit], false);
                }
                i += 1;
             }
         }

        pos += append_pattern(&result, pos, UPCEANReader.MIDDLE_PATTERN, false);
         {
             let mut i: i32 = 4;
            while i <= 7 {
                {
                     let digit: i32 = Character::digit(&contents.char_at(i), 10);
                    pos += append_pattern(&result, pos, UPCEANReader.L_PATTERNS[digit], true);
                }
                i += 1;
             }
         }

        append_pattern(&result, pos, UPCEANReader.START_END_PATTERN, true);
        return result;
    }
}

