/*
 * Copyright (C) 2012 ZXing authors
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
    common::BitArray, BarcodeFormat, Exceptions, RXingResult, RXingResultMetadataType,
    RXingResultMetadataValue, RXingResultPoint,
};

use super::{upc_ean_reader, STAND_IN, UPCEANReader};

/**
 * @see UPCEANExtension5Support
 */
pub struct UPCEANExtension2Support {
    decodeMiddleCounters: [u32; 4],
}
impl Default for UPCEANExtension2Support {
    fn default() -> Self {
        Self {
            decodeMiddleCounters: Default::default(),
        }
    }
}

impl UPCEANExtension2Support {
    pub fn decodeRow(
        &self,
        rowNumber: u32,
        row: &BitArray,
        extensionStartRange: &[u32; 3],
    ) -> Result<RXingResult, Exceptions> {
        let mut result = String::new();
        let end = self.decodeMiddle(row, extensionStartRange, &mut result)?;

        let resultString = result;
        let extensionData = Self::parseExtensionString(&resultString);

        let mut extensionRXingResult = RXingResult::new(
            &resultString,
            Vec::new(),
            vec![
                RXingResultPoint::new(
                    (extensionStartRange[0] + extensionStartRange[1]) as f32 / 2.0,
                    rowNumber as f32,
                ),
                RXingResultPoint::new(end as f32, rowNumber as f32),
            ],
            BarcodeFormat::UPC_EAN_EXTENSION,
        );
        if let Some(ed) = extensionData {
            extensionRXingResult.putAllMetadata(ed);
        }

        Ok(extensionRXingResult)
    }

    fn decodeMiddle(
        &self,
        row: &BitArray,
        startRange: &[u32; 3],
        resultString: &mut String,
    ) -> Result<u32, Exceptions> {
        let mut counters = self.decodeMiddleCounters;
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
        let end = row.getSize();
        let mut rowOffset = startRange[1] as usize;

        let mut checkParity = 0;

        let mut x = 0;
        while x < 2 && rowOffset < end {
            // for (int x = 0; x < 2 && rowOffset < end; x++) {
            let bestMatch = STAND_IN.decodeDigit(
                row,
                &mut counters,
                rowOffset,
                &upc_ean_reader::L_AND_G_PATTERNS,
            )?;
            resultString.push(char::from_u32('0' as u32 + bestMatch as u32 % 10).unwrap());
            // for counter in counters {
            // // for (int counter : counters) {
            //   rowOffset += counter;
            // }

            rowOffset += counters.iter().sum::<u32>() as usize;

            if bestMatch >= 10 {
                checkParity |= 1 << (1 - x);
            }
            if x != 1 {
                // Read off separator if not last
                rowOffset = row.getNextSet(rowOffset);
                rowOffset = row.getNextUnset(rowOffset);
            }
            x += 1;
        }

        if resultString.chars().count() != 2 {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        if resultString.parse::<u32>().unwrap() % 4 != checkParity {
            // if (Integer.parseInt(resultString.toString()) % 4 != checkParity) {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        Ok(rowOffset as u32)
    }

    /**
     * @param raw raw content of extension
     * @return formatted interpretation of raw content as a {@link Map} mapping
     *  one {@link RXingResultMetadataType} to appropriate value, or {@code null} if not known
     */
    fn parseExtensionString(
        raw: &str,
    ) -> Option<HashMap<RXingResultMetadataType, RXingResultMetadataValue>> {
        if raw.chars().count() != 2 {
            return None;
        }
        let mut result = HashMap::new();

        result.insert(
            RXingResultMetadataType::ISSUE_NUMBER,
            RXingResultMetadataValue::IssueNumber(raw.parse::<i32>().unwrap()),
        );

        Some(result)
    }
}
