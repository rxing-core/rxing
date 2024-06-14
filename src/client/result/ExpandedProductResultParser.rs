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

/*
 * These authors would like to acknowledge the Spanish Ministry of Industry,
 * Tourism and Trade, for the support in the project TSI020301-2008-2
 * "PIRAmIDE: Personalizable Interactions with Resources on AmI-enabled
 * Mobile Dynamic Environments", led by Treelogic
 * ( http://www.treelogic.com/ ):
 *
 *   http://www.piramidepse.com/
 */

// package com.google.zxing.client.result;

// import java.util.HashMap;
// import java.util.Map;

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.RXingResult;

use std::collections::HashMap;

use crate::BarcodeFormat;

use super::{ExpandedProductParsedRXingResult, ParsedClientResult, ResultParser};

/**
 * Parses strings of digits that represent a RSS Extended code.
 *
 * @author Antonio Manuel Benjumea Conde, Servinform, S.A.
 * @author Agustín Delgado, Servinform, S.A.
 */
pub fn parse(result: &crate::RXingResult) -> Option<super::ParsedClientResult> {
    let barcode_format = result.getBarcodeFormat();
    if barcode_format != &BarcodeFormat::RSS_EXPANDED {
        // ExtendedProductParsedRXingResult NOT created. Not a RSS Expanded barcode
        return None;
    }
    let rawText = ResultParser::getMassagedText(result);

    let mut productID: String = String::default(); // = null;
    let mut sscc: String = String::default();
    let mut lotNumber: String = String::default();
    let mut productionDate: String = String::default();
    let mut packagingDate: String = String::default();
    let mut bestBeforeDate: String = String::default();
    let mut expirationDate: String = String::default();
    let mut weight: String = String::default();
    let mut weightType: String = String::default();
    let mut weightIncrement: String = String::default();
    let mut price: String = String::default();
    let mut priceIncrement: String = String::default();
    let mut priceCurrency: String = String::default();
    let mut uncommonAIs = HashMap::new();

    let mut i = 0;

    while i < rawText.len() {
        let ai = findAIvalue(i, &rawText)?;
        // if ai == null {
        // Error. Code doesn't match with RSS expanded pattern
        // ExtendedProductParsedRXingResult NOT created. Not match with RSS Expanded pattern
        // return None;
        // }
        i += ai.len() + 2;
        let value = findValue(i, &rawText)?;
        i += value.len();
        match ai.as_str() {
            "00" => sscc = value,
            "01" => productID = value,
            "10" => lotNumber = value,
            "11" => productionDate = value,
            "13" => packagingDate = value,
            "15" => bestBeforeDate = value,
            "17" => expirationDate = value,
            "3100" | "3101" | "3102" | "3103" | "3104" | "3105" | "3106" | "3107" | "3108"
            | "3109" => {
                weight = value;
                weightType = ExpandedProductParsedRXingResult::KILOGRAM.into();
                ai[3..].clone_into(&mut weightIncrement)
            }
            "3200" | "3201" | "3202" | "3203" | "3204" | "3205" | "3206" | "3207" | "3208"
            | "3209" => {
                weight = value;
                weightType = ExpandedProductParsedRXingResult::POUND.into();
                ai[3..].clone_into(&mut weightIncrement)
            }
            "3920" | "3921" | "3922" | "3923" => {
                price = value;
                ai[3..].clone_into(&mut priceIncrement)
            }
            "3930" | "3931" | "3932" | "3933" => {
                if value.len() < 4 {
                    // The value must have more of 3 symbols (3 for currency and
                    // 1 at least for the price)
                    // ExtendedProductParsedRXingResult NOT created. Not match with RSS Expanded pattern
                    return None;
                }
                value[3..].clone_into(&mut price);
                value[0..3].clone_into(&mut priceCurrency);
                ai[3..].clone_into(&mut priceIncrement);
            }
            _ => {
                // No match with common AIs
                uncommonAIs.insert(ai, value);
            }
        };

        // switch (ai) {
        // case "00":
        //   sscc = value;
        //   break;
        // case "01":
        //   productID = value;
        //   break;
        // case "10":
        //   lotNumber = value;
        //   break;
        // case "11":
        //   productionDate = value;
        //   break;
        // case "13":
        //   packagingDate = value;
        //   break;
        // case "15":
        //   bestBeforeDate = value;
        //   break;
        // case "17":
        //   expirationDate = value;
        //   break;
        // case "3100":
        // case "3101":
        // case "3102":
        // case "3103":
        // case "3104":
        // case "3105":
        // case "3106":
        // case "3107":
        // case "3108":
        // case "3109":
        //   weight = value;
        //   weightType = ExpandedProductParsedRXingResult.KILOGRAM;
        //   weightIncrement = ai.substring(3);
        //   break;
        // case "3200":
        // case "3201":
        // case "3202":
        // case "3203":
        // case "3204":
        // case "3205":
        // case "3206":
        // case "3207":
        // case "3208":
        // case "3209":
        //   weight = value;
        //   weightType = ExpandedProductParsedRXingResult.POUND;
        //   weightIncrement = ai.substring(3);
        //   break;
        // case "3920":
        // case "3921":
        // case "3922":
        // case "3923":
        //   price = value;
        //   priceIncrement = ai.substring(3);
        //   break;
        // case "3930":
        // case "3931":
        // case "3932":
        // case "3933":
        //   if (value.length() < 4) {
        //     // The value must have more of 3 symbols (3 for currency and
        //     // 1 at least for the price)
        //     // ExtendedProductParsedRXingResult NOT created. Not match with RSS Expanded pattern
        //     return null;
        //   }
        //   price = value.substring(3);
        //   priceCurrency = value.substring(0, 3);
        //   priceIncrement = ai.substring(3);
        //   break;
        //   default:
        //     // No match with common AIs
        //     uncommonAIs.put(ai, value);
        //     break;
        // }
    }

    Some(ParsedClientResult::ExpandedProductResult(
        ExpandedProductParsedRXingResult::new(
            rawText,
            productID,
            sscc,
            lotNumber,
            productionDate,
            packagingDate,
            bestBeforeDate,
            expirationDate,
            weight,
            weightType,
            weightIncrement,
            price,
            priceIncrement,
            priceCurrency,
            uncommonAIs,
        ),
    ))

    // return new ExpandedProductParsedRXingResult(rawText,
    //                                        productID,
    //                                        sscc,
    //                                        lotNumber,
    //                                        productionDate,
    //                                        packagingDate,
    //                                        bestBeforeDate,
    //                                        expirationDate,
    //                                        weight,
    //                                        weightType,
    //                                        weightIncrement,
    //                                        price,
    //                                        priceIncrement,
    //                                        priceCurrency,
    //                                        uncommonAIs);
}

fn findAIvalue(i: usize, rawText: &str) -> Option<String> {
    let c = rawText.chars().nth(i)?;
    // First character must be a open parenthesis.If not, ERROR
    if c != '(' {
        return None;
    }

    let rawTextAux = &rawText[i + 1..];

    let mut buf = String::new();
    for index in 0..rawTextAux.len() {
        // for (int index = 0; index < rawTextAux.length(); index++) {
        let currentChar = rawTextAux.chars().nth(index)?;
        if currentChar == ')' {
            return Some(buf);
        }
        if !currentChar.is_ascii_digit() {
            return None;
        }
        buf.push(currentChar);
    }

    Some(buf)
}

fn findValue(i: usize, rawText: &str) -> Option<String> {
    let mut buf = String::new();
    let rawTextAux = &rawText[i..];

    for index in 0..rawTextAux.len() {
        // for (int index = 0; index < rawTextAux.length(); index++) {
        let c = rawTextAux.chars().nth(index)?;
        if c == '(' {
            // We look for a new AI. If it doesn't exist (ERROR), we continue
            // with the iteration
            if findAIvalue(index, rawTextAux).is_some() {
                break;
            }
            // if findAIvalue(index, rawTextAux) != null {
            //   break;
            // }
            buf.push('(');
        } else {
            buf.push(c);
        }
    }
    Some(buf)
}
