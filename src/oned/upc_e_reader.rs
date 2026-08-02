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

use super::{OneDReader, UPCEANReader};
use crate::{
    BarcodeFormat, Error,
    common::Result,
    oned::upcean_common::{checkStandardUPCEANChecksum, convertUPCEtoUPCA},
};
use rxing_one_d_proc_derive::{EANReader, OneDReader};

use super::oned_constants::upc_e::*;
use super::oned_constants::upc_ean_shared::L_AND_G_PATTERNS;

/**
 * <p>Implements decoding of the UPC-E format.</p>
 * <p><a href="http://www.barcodeisland.com/upce.phtml">This</a> is a great reference for
 * UPC-E information.</p>
 *
 * @author Sean Owen
 */
#[derive(OneDReader, EANReader, Default)]
pub struct UPCEReader;

impl UPCEANReader for UPCEReader {
    fn getBarcodeFormat(&self) -> crate::BarcodeFormat {
        BarcodeFormat::UPC_E
    }

    fn decodeMiddle(
        &self,
        row: &crate::common::BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<usize> {
        let mut counters = [0_u32; 4];

        let end = row.get_size();
        let mut rowOffset = startRange[1];

        let mut lgPatternFound = 0;

        let mut x = 0;
        while x < 6 && rowOffset < end {
            let bestMatch = self.decodeDigit(row, &mut counters, rowOffset, &L_AND_G_PATTERNS)?;
            resultString.push(
                char::from_u32('0' as u32 + bestMatch as u32 % 10)
                    .ok_or(Error::format_with("could not convert digit to char"))?,
            );
            rowOffset += counters.iter().sum::<u32>() as usize;

            if bestMatch >= 10 {
                lgPatternFound |= 1 << (5 - x);
            }

            x += 1;
        }

        Self::determineNumSysAndCheckDigit(resultString, lgPatternFound)?;

        Ok(rowOffset)
    }

    fn checkChecksum(&self, s: &str) -> Result<bool> {
        checkStandardUPCEANChecksum(
            &convertUPCEtoUPCA(s).ok_or(Error::checksum_with("upc_e checksum error"))?,
        )
    }

    fn decodeEnd(&self, row: &crate::common::BitArray, endStart: usize) -> Result<[usize; 2]> {
        self.findGuardPattern(row, endStart, true, &MIDDLE_END_PATTERN)
    }
}

impl UPCEReader {
    fn determineNumSysAndCheckDigit(
        resultString: &mut String,
        lgPatternFound: usize,
    ) -> Result<()> {
        for numSys in 0..=1 {
            for d in 0..10 {
                if lgPatternFound == NUMSYS_AND_CHECK_DIGIT_PATTERNS[numSys][d] {
                    resultString.insert(
                        0,
                        char::from_u32('0' as u32 + numSys as u32)
                            .ok_or(Error::format_with("could not convert digit to char"))?,
                    );
                    resultString.push(
                        char::from_u32('0' as u32 + d as u32)
                            .ok_or(Error::format_with("could not convert digit to char"))?,
                    );
                    return Ok(());
                }
            }
        }
        Err(Error::NOT_FOUND)
    }
}
