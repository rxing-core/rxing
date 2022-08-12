/*
 * Copyright 2007 ZXing authors
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
 * Parses strings of digits that represent a UPC code.
 * 
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct ProductResultParser {
    super: ResultParser;
}

impl ProductResultParser {

    // Treat all UPC and EAN variants as UPCs, in the sense that they are all product barcodes.
    pub fn  parse(&self,  result: &Result) -> ProductParsedResult  {
         let format: BarcodeFormat = result.get_barcode_format();
        if !(format == BarcodeFormat::UPC_A || format == BarcodeFormat::UPC_E || format == BarcodeFormat::EAN_8 || format == BarcodeFormat::EAN_13) {
            return null;
        }
         let raw_text: String = get_massaged_text(result);
        if !is_string_of_digits(&raw_text, &raw_text.length()) {
            return null;
        }
        // Not actually checking the checksum again here    
         let normalized_product_i_d: String;
        // Expand UPC-E for purposes of searching
        if format == BarcodeFormat::UPC_E && raw_text.length() == 8 {
            normalized_product_i_d = UPCEReader::convert_u_p_c_eto_u_p_c_a(&raw_text);
        } else {
            normalized_product_i_d = raw_text;
        }
        return ProductParsedResult::new(&raw_text, &normalized_product_i_d);
    }
}

