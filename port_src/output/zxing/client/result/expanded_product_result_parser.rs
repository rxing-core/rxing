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
// package com::google::zxing::client::result;

/**
 * Parses strings of digits that represent a RSS Extended code.
 * 
 * @author Antonio Manuel Benjumea Conde, Servinform, S.A.
 * @author AgustÃ­n Delgado, Servinform, S.A.
 */
pub struct ExpandedProductResultParser {
    super: ResultParser;
}

impl ExpandedProductResultParser {

    pub fn  parse(&self,  result: &Result) -> ExpandedProductParsedResult  {
         let format: BarcodeFormat = result.get_barcode_format();
        if format != BarcodeFormat::RSS_EXPANDED {
            // ExtendedProductParsedResult NOT created. Not a RSS Expanded barcode
            return null;
        }
         let raw_text: String = get_massaged_text(result);
         let product_i_d: String = null;
         let mut sscc: String = null;
         let lot_number: String = null;
         let production_date: String = null;
         let packaging_date: String = null;
         let best_before_date: String = null;
         let expiration_date: String = null;
         let mut weight: String = null;
         let weight_type: String = null;
         let weight_increment: String = null;
         let mut price: String = null;
         let price_increment: String = null;
         let price_currency: String = null;
         let uncommon_a_is: Map<String, String> = HashMap<>::new();
         let mut i: i32 = 0;
        while i < raw_text.length() {
             let ai: String = ::find_a_ivalue(i, &raw_text);
            if ai == null {
                // ExtendedProductParsedResult NOT created. Not match with RSS Expanded pattern
                return null;
            }
            i += ai.length() + 2;
             let value: String = ::find_value(i, &raw_text);
            i += value.length();
            match ai {
                  "00" => 
                     {
                        sscc = value;
                        break;
                    }
                  "01" => 
                     {
                        product_i_d = value;
                        break;
                    }
                  "10" => 
                     {
                        lot_number = value;
                        break;
                    }
                  "11" => 
                     {
                        production_date = value;
                        break;
                    }
                  "13" => 
                     {
                        packaging_date = value;
                        break;
                    }
                  "15" => 
                     {
                        best_before_date = value;
                        break;
                    }
                  "17" => 
                     {
                        expiration_date = value;
                        break;
                    }
                  "3100" => 
                     {
                    }
                  "3101" => 
                     {
                    }
                  "3102" => 
                     {
                    }
                  "3103" => 
                     {
                    }
                  "3104" => 
                     {
                    }
                  "3105" => 
                     {
                    }
                  "3106" => 
                     {
                    }
                  "3107" => 
                     {
                    }
                  "3108" => 
                     {
                    }
                  "3109" => 
                     {
                        weight = value;
                        weight_type = ExpandedProductParsedResult::KILOGRAM;
                        weight_increment = ai.substring(3);
                        break;
                    }
                  "3200" => 
                     {
                    }
                  "3201" => 
                     {
                    }
                  "3202" => 
                     {
                    }
                  "3203" => 
                     {
                    }
                  "3204" => 
                     {
                    }
                  "3205" => 
                     {
                    }
                  "3206" => 
                     {
                    }
                  "3207" => 
                     {
                    }
                  "3208" => 
                     {
                    }
                  "3209" => 
                     {
                        weight = value;
                        weight_type = ExpandedProductParsedResult::POUND;
                        weight_increment = ai.substring(3);
                        break;
                    }
                  "3920" => 
                     {
                    }
                  "3921" => 
                     {
                    }
                  "3922" => 
                     {
                    }
                  "3923" => 
                     {
                        price = value;
                        price_increment = ai.substring(3);
                        break;
                    }
                  "3930" => 
                     {
                    }
                  "3931" => 
                     {
                    }
                  "3932" => 
                     {
                    }
                  "3933" => 
                     {
                        if value.length() < 4 {
                            // ExtendedProductParsedResult NOT created. Not match with RSS Expanded pattern
                            return null;
                        }
                        price = value.substring(3);
                        price_currency = value.substring(0, 3);
                        price_increment = ai.substring(3);
                        break;
                    }
                _ => 
                     {
                        // No match with common AIs
                        uncommon_a_is.put(&ai, &value);
                        break;
                    }
            }
        }
        return ExpandedProductParsedResult::new(&raw_text, &product_i_d, &sscc, &lot_number, &production_date, &packaging_date, &best_before_date, &expiration_date, &weight, &weight_type, &weight_increment, &price, &price_increment, &price_currency, &uncommon_a_is);
    }

    fn  find_a_ivalue( i: i32,  raw_text: &String) -> String  {
         let c: char = raw_text.char_at(i);
        // First character must be a open parenthesis.If not, ERROR
        if c != '(' {
            return null;
        }
         let raw_text_aux: CharSequence = raw_text.substring(i + 1);
         let buf: StringBuilder = StringBuilder::new();
         {
             let mut index: i32 = 0;
            while index < raw_text_aux.length() {
                {
                     let current_char: char = raw_text_aux.char_at(index);
                    if current_char == ')' {
                        return buf.to_string();
                    }
                    if current_char < '0' || current_char > '9' {
                        return null;
                    }
                    buf.append(current_char);
                }
                index += 1;
             }
         }

        return buf.to_string();
    }

    fn  find_value( i: i32,  raw_text: &String) -> String  {
         let buf: StringBuilder = StringBuilder::new();
         let raw_text_aux: String = raw_text.substring(i);
         {
             let mut index: i32 = 0;
            while index < raw_text_aux.length() {
                {
                     let c: char = raw_text_aux.char_at(index);
                    if c == '(' {
                        // with the iteration
                        if ::find_a_ivalue(index, &raw_text_aux) != null {
                            break;
                        }
                        buf.append('(');
                    } else {
                        buf.append(c);
                    }
                }
                index += 1;
             }
         }

        return buf.to_string();
    }
}

