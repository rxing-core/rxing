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

use rxing_one_d_proc_derive::{EANReader, OneDReader};

use super::UPCEANReader;

use super::OneDReader;
use super::oned_constants::ean_13::FIRST_DIGIT_ENCODINGS;
use super::oned_constants::upc_ean_shared::*;

use crate::BarcodeFormat;
use crate::Error;
use crate::common::Result;

/**
 * <p>Implements decoding of the EAN-13 format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */
#[derive(OneDReader, EANReader, Default)]
pub struct EAN13Reader;
impl UPCEANReader for EAN13Reader {
    fn getBarcodeFormat(&self) -> crate::BarcodeFormat {
        BarcodeFormat::EAN_13
    }

    fn decodeMiddle(
        &self,
        row: &crate::common::BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<usize> {
        let mut counters = [0_u32; 4]; //decodeMiddleCounters;
        // counters[0] = 0;
        // counters[1] = 0;
        // counters[2] = 0;
        // counters[3] = 0;
        let end = row.get_size();
        let mut rowOffset = startRange[1];

        let mut lgPatternFound = 0;

        let mut x = 0;

        while x < 6 && rowOffset < end {
            // for (int x = 0; x < 6 && rowOffset < end; x++) {
            let bestMatch = self.decodeDigit(row, &mut counters, rowOffset, &L_AND_G_PATTERNS)?;
            resultString
                .push(char::from_u32('0' as u32 + bestMatch as u32 % 10).ok_or(Error::PARSE)?);

            rowOffset += counters.iter().sum::<u32>() as usize;

            if bestMatch >= 10 {
                lgPatternFound |= 1 << (5 - x);
            }

            x += 1;
        }

        Self::determineFirstDigit(resultString, lgPatternFound)?;

        let middleRange = self.findGuardPattern(row, rowOffset, true, &MIDDLE_PATTERN)?;
        rowOffset = middleRange[1];

        let mut x = 0;

        while x < 6 && rowOffset < end {
            let bestMatch = self.decodeDigit(row, &mut counters, rowOffset, &L_PATTERNS)?;
            resultString.push(char::from_u32('0' as u32 + bestMatch as u32).ok_or(Error::PARSE)?);

            rowOffset += counters.iter().sum::<u32>() as usize;

            x += 1;
        }

        Ok(rowOffset)
    }
}
impl EAN13Reader {
    /**
     * Based on pattern of odd-even ('L' and 'G') patterns used to encoded the explicitly-encoded
     * digits in a barcode, determines the implicitly encoded first digit and adds it to the
     * result string.
     *
     * @param resultString string to insert decoded first digit into
     * @param lgPatternFound int whose bits indicates the pattern of odd/even L/G patterns used to
     *  encode digits
     * @throws NotFoundException if first digit cannot be determined
     */
    fn determineFirstDigit(resultString: &mut String, lgPatternFound: usize) -> Result<()> {
        for (d, &fde) in FIRST_DIGIT_ENCODINGS.iter().enumerate() {
            // for (int d = 0; d < 10; d++) {
            if lgPatternFound == fde {
                resultString.insert(
                    0,
                    char::from_u32('0' as u32 + d as u32).ok_or(Error::PARSE)?,
                );
                return Ok(());
            }
        }
        Err(Error::NOT_FOUND)
    }
}
