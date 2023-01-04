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

use super::OneDReader;
use crate::{BarcodeFormat, Exceptions};
use rxing_one_d_proc_derive::{EANReader, OneDReader};

use super::upc_ean_reader;
use super::UPCEANReader;

/**
 * <p>Implements decoding of the EAN-8 format.</p>
 *
 * @author Sean Owen
 */
#[derive(OneDReader, EANReader, Default)]
pub struct EAN8Reader;

impl UPCEANReader for EAN8Reader {
    fn getBarcodeFormat(&self) -> crate::BarcodeFormat {
        BarcodeFormat::EAN_8
    }

    fn decodeMiddle(
        &self,
        row: &crate::common::BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<usize, Exceptions> {
        let mut counters = [0_u32; 4]; //decodeMiddleCounters;
                                       // counters[0] = 0;
                                       // counters[1] = 0;
                                       // counters[2] = 0;
                                       // counters[3] = 0;
        let end = row.getSize();
        let mut rowOffset = startRange[1];

        let mut x = 0;
        while x < 4 && rowOffset < end {
            // for (int x = 0; x < 4 && rowOffset < end; x++) {
            let bestMatch =
                self.decodeDigit(row, &mut counters, rowOffset, &upc_ean_reader::L_PATTERNS)?;
            resultString.push(char::from_u32('0' as u32 + bestMatch as u32).unwrap());
            // for (int counter : counters) {
            //   rowOffset += counter;
            // }

            rowOffset += counters.iter().sum::<u32>() as usize;

            x += 1;
        }

        let middleRange =
            self.findGuardPattern(row, rowOffset, true, &upc_ean_reader::MIDDLE_PATTERN)?;
        rowOffset = middleRange[1];

        let mut x = 0;
        while x < 4 && rowOffset < end {
            // for (int x = 0; x < 4 && rowOffset < end; x++) {
            let bestMatch =
                self.decodeDigit(row, &mut counters, rowOffset, &upc_ean_reader::L_PATTERNS)?;
            resultString.push(char::from_u32('0' as u32 + bestMatch as u32).unwrap());
            // for (int counter : counters) {
            //   rowOffset += counter;
            // }
            rowOffset += counters.iter().sum::<u32>() as usize;
            x += 1;
        }

        Ok(rowOffset)
    }
}
