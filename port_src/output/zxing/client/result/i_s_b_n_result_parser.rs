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
// package com::google::zxing::client::result;

/**
 * Parses strings of digits that represent a ISBN.
 * 
 * @author jbreiden@google.com (Jeff Breidenbach)
 */
pub struct ISBNResultParser {
    super: ResultParser;
}

impl ISBNResultParser {

    /**
   * See <a href="http://www.bisg.org/isbn-13/for.dummies.html">ISBN-13 For Dummies</a>
   */
    pub fn  parse(&self,  result: &Result) -> ISBNParsedResult  {
         let format: BarcodeFormat = result.get_barcode_format();
        if format != BarcodeFormat::EAN_13 {
            return null;
        }
         let raw_text: String = get_massaged_text(result);
         let length: i32 = raw_text.length();
        if length != 13 {
            return null;
        }
        if !raw_text.starts_with("978") && !raw_text.starts_with("979") {
            return null;
        }
        return ISBNParsedResult::new(&raw_text);
    }
}

