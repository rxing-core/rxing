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

use std::io::Read;

use crate::{RXingResultMetadataType, RXingResultMetadataValue};

use rxing_one_d_proc_derive::OneDReader;

use crate::common::{BitArray, Result};
use crate::oned::telepen_common;
use crate::Exceptions;
use crate::RXingResult;
use crate::{point_f, BarcodeFormat};

use super::OneDReader;

/**
 * <p>Decodes Telepen barcodes.</p>
 *
 * @author Chris Wood
 */
#[derive(OneDReader)]
pub struct TelepenReader {
    // Keep some instance variables to avoid reallocations
    counters: Box<[u32]>,
    counterLength: usize,
}

impl Default for TelepenReader {
    fn default() -> Self {
        Self {
            counters: Box::new([0; 80]),
            counterLength: 0,
        }
    }
}

impl OneDReader for TelepenReader {
    fn decode_row(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodeHints,
    ) -> Result<crate::RXingResult> {
        self.counters.fill(0);
        self.setCounters(row, (row.get_size() as f32 * 0.001) as u32)?;

        let startOffset = self.findStartPattern()? as usize;
        let end = self.findEndPattern(startOffset)? as usize;

        let theCounters = &self.counters;
        let mut maxBar = 0;
        let mut minBar = u32::MAX;

        // 0 position value will be whitespace prior to the barcode beginning
        let mut j = startOffset;

        // Calculate a median bar / gap width by establishing the smallest and
        // largest gaps.
        while j <= end {
            let currentCounter = theCounters[j];
            minBar = u32::min(currentCounter, minBar);
            maxBar = u32::max(currentCounter, maxBar);

            j += 1;
        }

        // Calculate median value as float.
        let mut thresholdBar = (minBar + maxBar) as f64 / 2.0;

        // Lean very slightly toward thicker bars.
        thresholdBar = thresholdBar - (thresholdBar / 10.0);

        // Start of the barcode is always a black bar.
        let mut isBlack = true;

        let mut pattern: Vec<u32> = vec![0; self.counterLength];
        let mut patternLength: usize = 0;

        pattern.fill(0);

        j = startOffset;

        // Categorise each value into narrow or wide black or white. Each
        // permutation is signified by an integer value.
        //
        //    0 = Narrow White
        //    1 = Narrow Black
        //    2 = Wide White
        //    3 = Wide Black
        //
        // Wide elements are 3x the width of narrow, thus:
        //
        //    Black narrow: B
        //    Black wide:   BBB
        //    White narrow: .
        //    White wide:   ...

        while j <= end {
            let currentCounter = theCounters[j];
            if (currentCounter as f64) < thresholdBar {
                if isBlack {
                    // Narrow black: B
                    pattern[patternLength] = 1;
                } else {
                    // Narrow white: .
                    pattern[patternLength] = 0;
                }
            } else if isBlack {
                // Wide black: BBB
                pattern[patternLength] = 3;
            } else {
                // Wide white: ...
                pattern[patternLength] = 2;
            }

            patternLength += 1;
            j += 1;
            isBlack = !isBlack;
        }

        let mut bits: BitArray = BitArray::new();
        let mut state = 0;
        j = 0;

        // Convert narrow-wide sequence into bit array.
        while j < patternLength - 1 {
            if pattern[j] == 3 && pattern[j + 1] == 2 {
                // BBB... = 010
                bits.appendBit(false);
                bits.appendBit(true);
                bits.appendBit(false);
            } else if pattern[j] == 3 && pattern[j + 1] == 0 {
                // BBB. = 00
                bits.appendBit(false);
                bits.appendBit(false);
            } else if pattern[j] == 1 && pattern[j + 1] == 2 && state == 0 {
                // B... = 01
                bits.appendBit(false);
                bits.appendBit(true);
                state = 1;
            } else if pattern[j] == 1 && pattern[j + 1] == 2 && state == 1 {
                // B... = 10
                bits.appendBit(true);
                bits.appendBit(false);
                state = 0;
            } else if pattern[j] == 1 && pattern[j + 1] == 0 {
                // B. = 1
                bits.appendBit(true);
            }

            j += 2;
        }

        let byteLength = bits.getSizeInBytes();

        // Any Telepen barcode will be longer than two bytes.
        if byteLength < 3 {
            return Err(Exceptions::NOT_FOUND);
        }

        let mut bytes: Vec<u8> = vec![0; byteLength];
        // bits.toBytes(0, bytes.as_mut_slice(), 0, byteLength);
        bits.read_exact(&mut bytes)
            .map_err(|_| Exceptions::ILLEGAL_STATE)?;

        j = 0;

        // Tweak our byte array to clean things up a little.
        while j < byteLength {
            // Telepen is little-endian, so need to swap the
            // bits around for each byte to be correct.
            bytes[j] = bytes[j].reverse_bits();

            // The first bit in the byte can always be disregarded
            // as the highest ASCII decimal value is 127. It might be
            // set because it is used as a parity bit during encoding
            // (ensuring that there is an equal number of 1 bits in the
            // byte).
            if bytes[j] >= 128 {
                bytes[j] -= 128;
            }

            j += 1;
        }

        // First character should be _ which is decimal 95.
        if bytes[0] != 95 {
            return Err(Exceptions::NOT_FOUND);
        }

        // Last character should be z which is decimal 122.
        if bytes[byteLength - 1] != 122 {
            return Err(Exceptions::NOT_FOUND);
        }

        // Content bytes
        let contentBytes = bytes[1..byteLength - 2].to_vec();
        let mut contentString = String::from_utf8_lossy(&contentBytes).to_string();

        // Penultimate byte is a block check character.
        let check = bytes[byteLength - 2];

        let checksum = telepen_common::calculate_checksum(&contentString);

        // Validate checksum
        if check != checksum as u8 {
            return Err(Exceptions::NOT_FOUND);
        }

        if matches!(hints.TelepenAsNumeric, Some(true)) {
            contentString = telepen_common::ascii_to_numeric(&contentString);
        }

        let mut runningCount = 0;
        runningCount += self.counters.iter().take(startOffset).sum::<u32>();
        let left: f32 = runningCount as f32;

        runningCount += self
            .counters
            .iter()
            .skip(startOffset)
            .take(self.counterLength - startOffset - end)
            .sum::<u32>();

        let right: f32 = runningCount as f32;

        let mut result = RXingResult::new(
            &contentString,
            bytes,
            vec![
                point_f(left, rowNumber as f32),
                point_f(right, rowNumber as f32),
            ],
            BarcodeFormat::TELEPEN,
        );

        result.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier("]B0".to_owned()),
        );

        Ok(result)
    }
}
impl TelepenReader {
    pub fn new() -> Self {
        Self {
            counters: Box::new([0; 80]), //Vec::with_capacity(80),
            counterLength: 0,
        }
    }

    /**
     * Records the size of all runs of white and black pixels, starting with white.
     * This is just like recordPattern, except it records all the counters, and
     * uses our builtin "counters" member for storage.
     * @param row row to count from
     */
    fn setCounters(&mut self, row: &BitArray, minToleratedWidth: u32) -> Result<()> {
        self.counterLength = 0;

        let mut i = 1;
        let end = row.get_size();
        let mut currentColor = false;
        let mut count = 1;

        // Move to first white pixel
        while i < end && row.get(i) {
            i += 1;
        }

        while i < end {
            if row.get(i) == currentColor {
                count += 1;
            } else {
                if count >= minToleratedWidth || self.counterLength == 0 {
                    self.counterAppend(count);
                } else {
                    // Noise from previous bar. Treat it as the
                    // previous colour.
                    self.counters[self.counterLength - 1] += count;
                }

                count = 1;
                currentColor = !currentColor;
            }

            i += 1;
        }

        if count >= minToleratedWidth {
            self.counterAppend(count);
        } else {
            // Noise from previous bar. Treat it as the
            // previous colour.
            self.counters[self.counterLength - 1] += count;
        }

        Ok(())
    }

    fn counterAppend(&mut self, e: u32) {
        self.counters[self.counterLength] = e;
        self.counterLength += 1;
        if self.counterLength >= self.counters.len() {
            let mut temp = vec![0; self.counterLength * 2];
            temp[0..self.counterLength].clone_from_slice(&self.counters[..]);
            self.counters = temp.into_boxed_slice();
        }
    }

    fn findStartPattern(&mut self) -> Result<u32> {
        if self.counterLength <= 20 {
            return Err(Exceptions::NOT_FOUND);
        }

        let mut i = 0;
        while i < self.counterLength - 20 {
            // Read next 20 in sequence. All 20 must be either between 28% and 38%
            // of biggest, or between 90% to 100% of biggest.
            let mut j = 0;
            let mut maxBar: f32 = 0.0;
            let mut minBar: f32 = f32::MAX;

            while i + j < self.counterLength && j < 20 {
                if (self.counters[i + j] as f32) > maxBar {
                    maxBar = self.counters[i + j] as f32;
                }

                if (self.counters[i + j] as f32) < minBar {
                    minBar = self.counters[i + j] as f32;
                }

                j += 1;
            }

            j = 0;

            let median = minBar + maxBar / 2.0;
            let mut passed = true;

            // First 10 items must be:
            //    N-N-N-N-N-N-N-N-N-N-W
            while i + j < self.counterLength && j < 11 {
                if j < 10 {
                    // Narrow
                    if (self.counters[i + j] as f32) > median {
                        passed = false;
                        break;
                    } else if j == 10 {
                        // Wide
                        if (self.counters[i + j] as f32) < median {
                            passed = false;
                            break;
                        }
                    }
                }

                j += 1;
            }

            if !passed {
                i += 1;
                continue;
            }

            return Ok(i as u32);
        }
        Err(Exceptions::NOT_FOUND)
    }

    fn findEndPattern(&mut self, start: usize) -> Result<u32> {
        const TOLERANCE: f32 = 0.5;

        let mut i = start;
        while i < self.counterLength {
            // Read next 20 in sequence. All 20 must be either between 28% and 38%
            // of biggest, or between 90% to 100% of biggest.
            let mut j = 0;
            let mut maxBar: f32 = 0.0;

            while i + j < self.counterLength && j < 20 {
                if (self.counters[i + j] as f32) > maxBar {
                    maxBar = self.counters[i + j] as f32;
                }

                j += 1;
            }

            i = start + 20;

            while i < self.counterLength {
                if (self.counters[i] as f32) > (maxBar * (1.0 + TOLERANCE)) {
                    return Ok((i - 1) as u32);
                }

                i += 1;
            }
        }
        Ok((self.counterLength - 1) as u32)
    }
}
