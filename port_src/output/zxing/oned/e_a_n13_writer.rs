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
 * This object renders an EAN13 code as a {@link BitMatrix}.
 *
 * @author aripollak@gmail.com (Ari Pollak)
 */

 const CODE_WIDTH: i32 = // start guard
3 + // left bars
(7 * 6) + // middle guard
5 + // right bars
(7 * 6) + // end guard
3;
pub struct EAN13Writer {
    super: UPCEANWriter;
}

impl EAN13Writer {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::EAN_13);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let length: i32 = contents.length();
        match length {
              12 => 
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
              13 => 
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
                    throw IllegalArgumentException::new(format!("Requested contents should be 12 or 13 digits long, but got {}", length));
                }
        }
        check_numeric(&contents);
         let first_digit: i32 = Character::digit(&contents.char_at(0), 10);
         let parities: i32 = EAN13Reader.FIRST_DIGIT_ENCODINGS[first_digit];
         let result: [bool; CODE_WIDTH] = [false; CODE_WIDTH];
         let mut pos: i32 = 0;
        pos += append_pattern(&result, pos, UPCEANReader.START_END_PATTERN, true);
        // See EAN13Reader for a description of how the first digit & left bars are encoded
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

        pos += append_pattern(&result, pos, UPCEANReader.MIDDLE_PATTERN, false);
         {
             let mut i: i32 = 7;
            while i <= 12 {
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

