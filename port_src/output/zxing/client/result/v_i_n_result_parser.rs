/*
 * Copyright 2014 ZXing authors
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
// package com::google::zxing::client::result;

/**
 * Detects a result that is likely a vehicle identification number.
 *
 * @author Sean Owen
 */

 const IOQ: Pattern = Pattern::compile("[IOQ]");

 const AZ09: Pattern = Pattern::compile("[A-Z0-9]{17}");
pub struct VINResultParser {
    super: ResultParser;
}

impl VINResultParser {

    pub fn  parse(&self,  result: &Result) -> VINParsedResult  {
        if result.get_barcode_format() != BarcodeFormat::CODE_39 {
            return null;
        }
         let raw_text: String = result.get_text();
        raw_text = IOQ::matcher(&raw_text)::replace_all("")::trim();
        if !AZ09::matcher(&raw_text)::matches() {
            return null;
        }
        let tryResult1 = 0;
        'try1: loop {
        {
            if !::check_checksum(&raw_text) {
                return null;
            }
             let wmi: String = raw_text.substring(0, 3);
            return VINParsedResult::new(&raw_text, &wmi, &raw_text.substring(3, 9), &raw_text.substring(9, 17), &::country_code(&wmi), &raw_text.substring(3, 8), &::model_year(&raw_text.char_at(9)), &raw_text.char_at(10), &raw_text.substring(11));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( iae: &IllegalArgumentException) {
                return null;
            }  0 => break
        }

    }

    fn  check_checksum( vin: &CharSequence) -> bool  {
         let mut sum: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < vin.length() {
                {
                    sum += ::vin_position_weight(i + 1) * ::vin_char_value(&vin.char_at(i));
                }
                i += 1;
             }
         }

         let check_char: char = vin.char_at(8);
         let expected_check_char: char = self.check_char(sum % 11);
        return check_char == expected_check_char;
    }

    fn  vin_char_value( c: char) -> i32  {
        if c >= 'A' && c <= 'I' {
            return (c - 'A') + 1;
        }
        if c >= 'J' && c <= 'R' {
            return (c - 'J') + 1;
        }
        if c >= 'S' && c <= 'Z' {
            return (c - 'S') + 2;
        }
        if c >= '0' && c <= '9' {
            return c - '0';
        }
        throw IllegalArgumentException::new();
    }

    fn  vin_position_weight( position: i32) -> i32  {
        if position >= 1 && position <= 7 {
            return 9 - position;
        }
        if position == 8 {
            return 10;
        }
        if position == 9 {
            return 0;
        }
        if position >= 10 && position <= 17 {
            return 19 - position;
        }
        throw IllegalArgumentException::new();
    }

    fn  check_char( remainder: i32) -> char  {
        if remainder < 10 {
            return ('0' + remainder) as char;
        }
        if remainder == 10 {
            return 'X';
        }
        throw IllegalArgumentException::new();
    }

    fn  model_year( c: char) -> i32  {
        if c >= 'E' && c <= 'H' {
            return (c - 'E') + 1984;
        }
        if c >= 'J' && c <= 'N' {
            return (c - 'J') + 1988;
        }
        if c == 'P' {
            return 1993;
        }
        if c >= 'R' && c <= 'T' {
            return (c - 'R') + 1994;
        }
        if c >= 'V' && c <= 'Y' {
            return (c - 'V') + 1997;
        }
        if c >= '1' && c <= '9' {
            return (c - '1') + 2001;
        }
        if c >= 'A' && c <= 'D' {
            return (c - 'A') + 2010;
        }
        throw IllegalArgumentException::new();
    }

    fn  country_code( wmi: &CharSequence) -> String  {
         let c1: char = wmi.char_at(0);
         let c2: char = wmi.char_at(1);
        match c1 {
              '1' => 
                 {
                }
              '4' => 
                 {
                }
              '5' => 
                 {
                    return "US";
                }
              '2' => 
                 {
                    return "CA";
                }
              '3' => 
                 {
                    if c2 >= 'A' && c2 <= 'W' {
                        return "MX";
                    }
                    break;
                }
              '9' => 
                 {
                    if (c2 >= 'A' && c2 <= 'E') || (c2 >= '3' && c2 <= '9') {
                        return "BR";
                    }
                    break;
                }
              'J' => 
                 {
                    if c2 >= 'A' && c2 <= 'T' {
                        return "JP";
                    }
                    break;
                }
              'K' => 
                 {
                    if c2 >= 'L' && c2 <= 'R' {
                        return "KO";
                    }
                    break;
                }
              'L' => 
                 {
                    return "CN";
                }
              'M' => 
                 {
                    if c2 >= 'A' && c2 <= 'E' {
                        return "IN";
                    }
                    break;
                }
              'S' => 
                 {
                    if c2 >= 'A' && c2 <= 'M' {
                        return "UK";
                    }
                    if c2 >= 'N' && c2 <= 'T' {
                        return "DE";
                    }
                    break;
                }
              'V' => 
                 {
                    if c2 >= 'F' && c2 <= 'R' {
                        return "FR";
                    }
                    if c2 >= 'S' && c2 <= 'W' {
                        return "ES";
                    }
                    break;
                }
              'W' => 
                 {
                    return "DE";
                }
              'X' => 
                 {
                    if c2 == '0' || (c2 >= '3' && c2 <= '9') {
                        return "RU";
                    }
                    break;
                }
              'Z' => 
                 {
                    if c2 >= 'A' && c2 <= 'R' {
                        return "IT";
                    }
                    break;
                }
        }
        return null;
    }
}

