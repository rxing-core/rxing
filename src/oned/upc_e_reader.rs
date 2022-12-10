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

use super::{OneDReader, UPCEANReader, L_AND_G_PATTERNS};
use crate::{BarcodeFormat, Exceptions};
use one_d_reader_derive::{EANReader, OneDReader};

/**
 * <p>Implements decoding of the UPC-E format.</p>
 * <p><a href="http://www.barcodeisland.com/upce.phtml">This</a> is a great reference for
 * UPC-E information.</p>
 *
 * @author Sean Owen
 */
#[derive(OneDReader, EANReader)]
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
    ) -> Result<usize, Exceptions> {
        let mut counters = [0_u32; 4]; //decodeMiddleCounters;
                                       // counters[0] = 0;
                                       // counters[1] = 0;
                                       // counters[2] = 0;
                                       // counters[3] = 0;
        let end = row.getSize();
        let mut rowOffset = startRange[1];

        let mut lgPatternFound = 0;

        let mut x = 0;
        while x < 6 && rowOffset < end {
            // for (int x = 0; x < 6 && rowOffset < end; x++) {
            let bestMatch = self.decodeDigit(row, &mut counters, rowOffset, &L_AND_G_PATTERNS)?;
            resultString.push(char::from_u32('0' as u32 + bestMatch as u32 % 10).unwrap());
            // for (int counter : counters) {
            //   rowOffset += counter;
            // }
            rowOffset += counters.iter().sum::<u32>() as usize;

            if bestMatch >= 10 {
                lgPatternFound |= 1 << (5 - x);
            }

            x += 1;
        }

        Self::determineNumSysAndCheckDigit(resultString, lgPatternFound)?;

        Ok(rowOffset)
    }

    fn checkChecksum(&self, s: &str) -> Result<bool, Exceptions> {
        self.checkStandardUPCEANChecksum(&convertUPCEtoUPCA(s))
    }

    fn decodeEnd(&self, row: &crate::common::BitArray, endStart: usize) -> Result<[usize; 2], Exceptions>
    {
        self.findGuardPattern(row, endStart, true, &Self::MIDDLE_END_PATTERN)
    }
}

impl Default for UPCEReader {
    fn default() -> Self {
        Self {}
    }
}

impl UPCEReader {
    /**
     * The pattern that marks the middle, and end, of a UPC-E pattern.
     * There is no "second half" to a UPC-E barcode.
     */
    const MIDDLE_END_PATTERN: [u32; 6] = [1, 1, 1, 1, 1, 1];

    // For an UPC-E barcode, the final digit is represented by the parities used
    // to encode the middle six digits, according to the table below.
    //
    //                Parity of next 6 digits
    //    Digit   0     1     2     3     4     5
    //       0    Even   Even  Even Odd  Odd   Odd
    //       1    Even   Even  Odd  Even Odd   Odd
    //       2    Even   Even  Odd  Odd  Even  Odd
    //       3    Even   Even  Odd  Odd  Odd   Even
    //       4    Even   Odd   Even Even Odd   Odd
    //       5    Even   Odd   Odd  Even Even  Odd
    //       6    Even   Odd   Odd  Odd  Even  Even
    //       7    Even   Odd   Even Odd  Even  Odd
    //       8    Even   Odd   Even Odd  Odd   Even
    //       9    Even   Odd   Odd  Even Odd   Even
    //
    // The encoding is represented by the following array, which is a bit pattern
    // using Odd = 0 and Even = 1. For example, 5 is represented by:
    //
    //              Odd Even Even Odd Odd Even
    // in binary:
    //                0    1    1   0   0    1   == 0x19
    //

    /**
     * See {@link #L_AND_G_PATTERNS}; these values similarly represent patterns of
     * even-odd parity encodings of digits that imply both the number system (0 or 1)
     * used, and the check digit.
     */
    const NUMSYS_AND_CHECK_DIGIT_PATTERNS: [[usize; 10]; 2] = [
        [0x38, 0x34, 0x32, 0x31, 0x2C, 0x26, 0x23, 0x2A, 0x29, 0x25],
        [0x07, 0x0B, 0x0D, 0x0E, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A],
    ];

    fn determineNumSysAndCheckDigit(
        resultString: &mut String,
        lgPatternFound: usize,
    ) -> Result<(), Exceptions> {
        for numSys in 0..=1 {
            // for (int numSys = 0; numSys <= 1; numSys++) {
            for d in 0..10 {
                // for (int d = 0; d < 10; d++) {
                if lgPatternFound == Self::NUMSYS_AND_CHECK_DIGIT_PATTERNS[numSys][d] {
                    resultString.insert(0, char::from_u32('0' as u32 + numSys as u32).unwrap());
                    resultString.push(char::from_u32('0' as u32 + d as u32).unwrap());
                    return Ok(());
                }
            }
        }
        return Err(Exceptions::NotFoundException("".to_owned()));
    }
}

/**
 * Expands a UPC-E value back into its full, equivalent UPC-A code value.
 *
 * @param upce UPC-E code as string of digits
 * @return equivalent UPC-A code as string of digits
 */
pub fn convertUPCEtoUPCA(upce: &str) -> String {
    let upceChars = &upce[1..8]; //['0';6];//new char[6];
                                 //upce.getChars(1, 7, upceChars, 0);
    let mut result = String::with_capacity(12); //new StringBuilder(12);
    result.push(upce.chars().nth(0).unwrap());
    let lastChar = upceChars.chars().nth(5).unwrap();
    match lastChar {
        '0' | '1' | '2' => {
            result.push_str(&upceChars[0..2]);
            // result.push(upceChars, 0, 2);
            result.push(lastChar);
            result.push_str("0000");
            result.push_str(&upceChars[2..3 + 2]);
        }
        '3' => {
            result.push_str(&upceChars[0..3]);
            result.push_str("00000");
            result.push_str(&upceChars[3..2 + 3]);
        }
        '4' => {
            result.push_str(&upceChars[0..4]);
            result.push_str("00000");
            result.push(upceChars.chars().nth(4).unwrap());
        }
        _ => {
            result.push_str(&upceChars[0..5]);
            result.push_str("0000");
            result.push(lastChar);
        }
    }
    // Only append check digit in conversion if supplied
    if upce.chars().count() >= 8 {
        result.push(upce.chars().nth(7).unwrap());
    }

    result
}
