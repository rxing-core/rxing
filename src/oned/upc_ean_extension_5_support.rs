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

use std::collections::HashMap;

use crate::{
    common::{BitArray, Result},
    point, BarcodeFormat, Exceptions, RXingResult, RXingResultMetadataType,
    RXingResultMetadataValue,
};

use super::{upc_ean_reader, UPCEANReader, STAND_IN};

/**
 * @see UPCEANExtension2Support
 */
#[derive(Default)]
pub struct UPCEANExtension5Support;

impl UPCEANExtension5Support {
    const CHECK_DIGIT_ENCODINGS: [usize; 10] =
        [0x18, 0x14, 0x12, 0x11, 0x0C, 0x06, 0x03, 0x0A, 0x09, 0x05];

    pub fn decodeRow(
        &self,
        rowNumber: u32,
        row: &BitArray,
        extensionStartRange: &[usize; 2],
    ) -> Result<RXingResult> {
        let mut result = String::new();

        let end = Self::decodeMiddle(row, extensionStartRange, &mut result)?;

        let resultString = result;
        let extensionData = Self::parseExtensionString(&resultString);

        let mut extensionRXingResult = RXingResult::new(
            &resultString,
            Vec::new(),
            vec![
                point(
                    (extensionStartRange[0] + extensionStartRange[1]) as f32 / 2.0,
                    rowNumber as f32,
                ),
                point(end as f32, rowNumber as f32),
            ],
            BarcodeFormat::UPC_EAN_EXTENSION,
        );

        if let Some(ed) = extensionData {
            extensionRXingResult.putAllMetadata(ed);
        }

        Ok(extensionRXingResult)
    }

    fn decodeMiddle(
        row: &BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<u32> {
        let mut counters = [0_u32; 4];
        let end = row.getSize();
        let mut rowOffset = startRange[1];

        let mut lgPatternFound = 0;

        let mut x = 0;
        while x < 5 && rowOffset < end {
            let bestMatch = STAND_IN.decodeDigit(
                row,
                &mut counters,
                rowOffset,
                &upc_ean_reader::L_AND_G_PATTERNS,
            )?;
            resultString
                .push(char::from_u32('0' as u32 + bestMatch as u32 % 10).ok_or(Exceptions::PARSE)?);

            rowOffset += counters.iter().sum::<u32>() as usize;

            if bestMatch >= 10 {
                lgPatternFound |= 1 << (4 - x);
            }
            if x != 4 {
                // Read off separator if not last
                rowOffset = row.getNextSet(rowOffset);
                rowOffset = row.getNextUnset(rowOffset);
            }

            x += 1;
        }

        if resultString.chars().count() != 5 {
            return Err(Exceptions::NOT_FOUND);
        }

        let checkDigit = Self::determineCheckDigit(lgPatternFound)?;
        if Self::extensionChecksum(resultString).ok_or(Exceptions::ILLEGAL_ARGUMENT)?
            != checkDigit as u32
        {
            return Err(Exceptions::NOT_FOUND);
        }

        Ok(rowOffset as u32)
    }

    fn extensionChecksum(s: &str) -> Option<u32> {
        let length = s.chars().count();
        let mut sum = 0;
        let mut i = length as isize - 2;
        while i >= 0 {
            // for (int i = length - 2; i >= 0; i -= 2) {
            sum += s.chars().nth(i as usize)? as u32 - '0' as u32;

            i -= 2;
        }
        sum *= 3;

        let mut i = length as isize - 1;
        while i >= 0 {
            // for (int i = length - 1; i >= 0; i -= 2) {
            sum += s.chars().nth(i as usize)? as u32 - '0' as u32;

            i -= 2;
        }
        sum *= 3;
        Some(sum % 10)
    }

    fn determineCheckDigit(lgPatternFound: usize) -> Result<usize> {
        for d in 0..10 {
            if lgPatternFound == Self::CHECK_DIGIT_ENCODINGS[d] {
                return Ok(d);
            }
        }
        Err(Exceptions::NOT_FOUND)
    }

    /**
     * @param raw raw content of extension
     * @return formatted interpretation of raw content as a {@link Map} mapping
     *  one {@link RXingResultMetadataType} to appropriate value, or {@code null} if not known
     */
    fn parseExtensionString(
        raw: &str,
    ) -> Option<HashMap<RXingResultMetadataType, RXingResultMetadataValue>> {
        if raw.chars().count() != 5 {
            return None;
        }
        let Some(value) = Self::parseExtension5String(raw) else {
            return None;
        };

        let mut result = HashMap::new();
        result.insert(
            RXingResultMetadataType::SUGGESTED_PRICE,
            RXingResultMetadataValue::SuggestedPrice(value),
        );

        Some(result)
    }

    fn parseExtension5String(raw: &str) -> Option<String> {
        let currency = match raw.chars().next()? {
            '0' => "£",
            '5' => "$",
            '9' => {
                // Reference: http://www.jollytech.com
                match raw {
                    "90000" =>
                    // No suggested retail price
                    {
                        return None
                    }
                    "99991" =>
                    // Complementary
                    {
                        return Some("0.00".to_string())
                    }
                    "99990" => return Some("Used".to_owned()),
                    _ => {}
                }
                // Otherwise... unknown currency?
                ""
            }
            _ => "",
        };

        let rawAmount = raw[1..].parse::<i32>().ok()?;
        let unitsString = (rawAmount / 100).to_string();
        let hundredths = rawAmount % 100;
        let hundredthsString = if hundredths < 10 {
            format!("0{hundredths}")
        } else {
            hundredths.to_string()
        };

        Some(format!("{currency}{unitsString}.{hundredthsString}"))
    }
}
