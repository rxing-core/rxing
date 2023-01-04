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

// package com.google.zxing.client.result;

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.RXingResult;
// import com.google.zxing.oned.UPCEReader;

use crate::{BarcodeFormat, RXingResult};

use super::{ParsedClientResult, ProductParsedRXingResult, ResultParser};

/**
 * Parses strings of digits that represent a UPC code.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    // Treat all UPC and EAN variants as UPCs, in the sense that they are all product barcodes.

    let format = result.getBarcodeFormat();
    if !(format == &BarcodeFormat::UPC_A
        || format == &BarcodeFormat::UPC_E
        || format == &BarcodeFormat::EAN_8
        || format == &BarcodeFormat::EAN_13)
    {
        return None;
    }
    let rawText = ResultParser::getMassagedText(result);
    if !ResultParser::isStringOfDigits(&rawText, rawText.len()) {
        return None;
    }
    // Not actually checking the checksum again here

    let normalizedProductID=
    // Expand UPC-E for purposes of searching
    if format == &BarcodeFormat::UPC_E && rawText.len() == 8 {
        // unimplemented!("UPCEReader is required to parse this");
         crate::oned::convertUPCEtoUPCA(&rawText)
    } else {
         rawText.clone()
    };

    Some(ParsedClientResult::ProductResult(
        ProductParsedRXingResult::with_normalized_id(rawText, normalizedProductID),
    ))
}
