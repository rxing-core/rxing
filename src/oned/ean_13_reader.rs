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

use std::marker::PhantomData;

use one_d_proc_derive::{EANReader, OneDReader};

use super::UPCEANReader;

use super::upc_ean_reader;
use super::OneDReader;

use crate::BarcodeFormat;
use crate::Exceptions;

/**
 * <p>Implements decoding of the EAN-13 format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */
#[derive(OneDReader, EANReader)]
pub struct EAN13Reader<L:LuminanceSource,B:Binarizer<L>>(PhantomData<L>,PhantomData<B>);
impl<L:LuminanceSource,B:Binarizer<L>> UPCEANReader<L,B> for EAN13Reader<L,B> {
    fn getBarcodeFormat(&self) -> crate::BarcodeFormat {
        BarcodeFormat::EAN_13
    }

    fn decodeMiddle(
        &self,
        row: &crate::common::BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<usize, crate::Exceptions> {
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
            let bestMatch = self.decodeDigit(
                row,
                &mut counters,
                rowOffset,
                &upc_ean_reader::L_AND_G_PATTERNS,
            )?;
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

        Self::determineFirstDigit(resultString, lgPatternFound)?;

        let middleRange =
            self.findGuardPattern(row, rowOffset, true, &upc_ean_reader::MIDDLE_PATTERN)?;
        rowOffset = middleRange[1];

        let mut x = 0;

        while x < 6 && rowOffset < end {
            // for (int x = 0; x < 6 && rowOffset < end; x++) {
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
impl<L:LuminanceSource,B:Binarizer<L>> EAN13Reader<L,B> {
    // For an EAN-13 barcode, the first digit is represented by the parities used
    // to encode the next six digits, according to the table below. For example,
    // if the barcode is 5 123456 789012 then the value of the first digit is
    // signified by using odd for '1', even for '2', even for '3', odd for '4',
    // odd for '5', and even for '6'. See http://en.wikipedia.org/wiki/EAN-13
    //
    //                Parity of next 6 digits
    //    Digit   0     1     2     3     4     5
    //       0    Odd   Odd   Odd   Odd   Odd   Odd
    //       1    Odd   Odd   Even  Odd   Even  Even
    //       2    Odd   Odd   Even  Even  Odd   Even
    //       3    Odd   Odd   Even  Even  Even  Odd
    //       4    Odd   Even  Odd   Odd   Even  Even
    //       5    Odd   Even  Even  Odd   Odd   Even
    //       6    Odd   Even  Even  Even  Odd   Odd
    //       7    Odd   Even  Odd   Even  Odd   Even
    //       8    Odd   Even  Odd   Even  Even  Odd
    //       9    Odd   Even  Even  Odd   Even  Odd
    //
    // Note that the encoding for '0' uses the same parity as a UPC barcode. Hence
    // a UPC barcode can be converted to an EAN-13 barcode by prepending a 0.
    //
    // The encoding is represented by the following array, which is a bit pattern
    // using Odd = 0 and Even = 1. For example, 5 is represented by:
    //
    //              Odd Even Even Odd Odd Even
    // in binary:
    //                0    1    1   0   0    1   == 0x19
    //
    pub const FIRST_DIGIT_ENCODINGS: [usize; 10] =
        [0x00, 0x0B, 0x0D, 0xE, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A];

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
    fn determineFirstDigit(
        resultString: &mut String,
        lgPatternFound: usize,
    ) -> Result<(), Exceptions> {
        for d in 0..10 {
            // for (int d = 0; d < 10; d++) {
            if lgPatternFound == Self::FIRST_DIGIT_ENCODINGS[d] {
                resultString.insert(0, char::from_u32('0' as u32 + d as u32).unwrap());
                return Ok(());
            }
        }
        return Err(Exceptions::NotFoundException("".to_owned()));
    }
}

impl<L:LuminanceSource,B:Binarizer<L>> Default for EAN13Reader<L,B> {
    fn default() -> Self {
        Self (PhantomData,PhantomData)
    }
}
