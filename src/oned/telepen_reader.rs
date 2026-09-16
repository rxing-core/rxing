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

use crate::Exceptions;
use crate::RXingResult;
use crate::common::{BitArray, Result};
use crate::oned::telepen_common;
use crate::{BarcodeFormat, point};

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
                point(left, rowNumber as f32),
                point(right, rowNumber as f32),
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

        if count >= minToleratedWidth || self.counterLength == 0 {
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

            // Midpoint between the narrowest and widest element in the window.
            // Note this is `(min + max) / 2`, not `min + max / 2`: the latter
            // sits at three quarters of the way up for a min of zero and lets
            // genuinely wide elements pass as narrow.
            // A window of uniform width carries no narrow/wide distinction, so
            // both comparisons below pass vacuously and any flat run of
            // elements -- the signature of noise rather than a symbol --
            // matches the start pattern.
            if maxBar <= minBar {
                i += 1;
                continue;
            }

            let median = (minBar + maxBar) / 2.0;
            let mut passed = true;

            // The start pattern is 11 elements:
            //    N-N-N-N-N-N-N-N-N-N-W
            // The window must hold all 11 for the check to mean anything; a
            // truncated tail is not a start pattern.
            if i + 11 > self.counterLength {
                break;
            }

            while j < 11 {
                if j < 10 {
                    // Narrow
                    if (self.counters[i + j] as f32) > median {
                        passed = false;
                        break;
                    }
                } else {
                    // Wide. Previously this arm was nested inside `j < 10` as
                    // an `else if j == 10`, which is unreachable, so the wide
                    // element was never checked at all and any run of ten
                    // narrow elements matched.
                    if (self.counters[i + j] as f32) < median {
                        passed = false;
                        break;
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
                    let end = i - 1;
                    self.checkStopPattern(start, end)?;
                    return Ok(end as u32);
                }

                i += 1;
            }
        }

        let end = self.counterLength - 1;
        self.checkStopPattern(start, end)?;
        Ok(end as u32)
    }

    /// Verifies the symbol ends on something shaped like a Telepen stop.
    ///
    /// `findEndPattern` locates the end of the symbol by looking for a quiet
    /// zone -- an element half again wider than anything nearby -- which any
    /// sufficiently isolated dark run satisfies. It performs no structural
    /// check, so on a picture with no Telepen symbol in it the end lands on
    /// arbitrary structure and the character-level guards downstream (`_`
    /// first, `z` last, checksum) are the only thing between noise and a
    /// decode.
    ///
    /// Telepen's stop character is `z` plus the reversed start, which ends in
    /// a run of narrow elements. That trailing run is what is checked here.
    /// The wide elements preceding it are deliberately not: how many survive
    /// binarisation varies with the trailing quiet zone, and requiring an
    /// exact mirror of the start pattern rejects real symbols in
    /// `test_resources/blackbox/telepen-1`.
    ///
    /// Elements are classified against the midpoint of the region being
    /// decoded, the same rule `decode_row` applies a few lines later, so this
    /// rejects nothing the subsequent narrow/wide categorisation would have
    /// accepted.
    fn checkStopPattern(&self, start: usize, end: usize) -> Result<()> {
        // The stop elements, plus at least the 11 of the start pattern.
        if end <= start || end - start < 21 || end >= self.counterLength {
            return Err(Exceptions::NOT_FOUND);
        }

        let region = &self.counters[start..=end];
        let minBar = *region.iter().min().unwrap_or(&0) as f32;
        let maxBar = *region.iter().max().unwrap_or(&0) as f32;

        // A region of uniform width carries no narrow/wide distinction at all,
        // so there is nothing here that could be a Telepen symbol.
        if maxBar <= minBar {
            return Err(Exceptions::NOT_FOUND);
        }

        let median = (minBar + maxBar) / 2.0;

        // Telepen's stop character ends in a run of narrow elements. Every
        // symbol in the test corpus ends with at least nine, preceded by wide
        // elements; the count is checked rather than the full mirror of the
        // start pattern because the wide side varies with how the trailing
        // quiet zone is binarised.
        const TRAILING_NARROW: usize = 9;
        for k in (end + 1 - TRAILING_NARROW)..=end {
            if (self.counters[k] as f32) > median {
                return Err(Exceptions::NOT_FOUND);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DecodeHints;

    /// Builds a row from alternating white/black runs, starting with white.
    fn row_from_runs(runs: &[u32]) -> BitArray {
        let total: u32 = runs.iter().sum();
        let mut row = BitArray::with_size(total as usize + 2);
        let mut at = 1usize;
        let mut black = false;
        for run in runs {
            if black {
                for k in at..at + *run as usize {
                    row.set(k);
                }
            }
            at += *run as usize;
            black = !black;
        }
        row
    }

    /// The start pattern is ten narrow elements followed by a wide one. The
    /// wide check used to sit in an `else if j == 10` nested inside `if j < 10`,
    /// which is unreachable, so only the ten narrow elements were ever tested
    /// and any run of ten narrow elements was accepted as a start pattern.
    ///
    /// This row is all-narrow: there is no wide eleventh element anywhere, so
    /// there is no start pattern in it. Before the fix `findStartPattern`
    /// returned `Ok(0)`.
    #[test]
    fn uniform_narrow_run_is_not_a_start_pattern() {
        let mut reader = TelepenReader::new();
        reader.setCounters(&row_from_runs(&[2; 60]), 1).unwrap();

        assert!(
            matches!(
                reader.findStartPattern(),
                Err(Exceptions::NotFoundException(_))
            ),
            "a run of uniform narrow elements has no wide element to end the \
             start pattern, so it must not match one"
        );
    }

    /// The companion to the above: a genuine start pattern -- ten narrow then
    /// one wide -- must still be found, at its correct offset.
    #[test]
    fn genuine_start_pattern_is_found() {
        let mut runs = vec![2u32; 10];
        runs.push(6);
        runs.extend(std::iter::repeat(2).take(40));

        let mut reader = TelepenReader::new();
        reader.setCounters(&row_from_runs(&runs), 1).unwrap();

        assert_eq!(reader.findStartPattern().unwrap(), 0);
    }

    /// `findStartPattern` classified elements against `minBar + maxBar / 2.0`,
    /// which is the midpoint only when `minBar` is zero; otherwise it sits far
    /// too high and admits genuinely wide elements as narrow. With min 2 and
    /// max 6 the buggy expression gives 5.0 rather than 4.0, so an element of
    /// width 5 -- clearly wide against a narrow width of 2 -- passed as narrow.
    #[test]
    fn wide_element_is_not_admitted_as_narrow_by_a_skewed_midpoint() {
        // Nine narrow, then a wide element in the tenth narrow slot.
        let mut runs = vec![2u32; 9];
        runs.push(5);
        runs.push(6);
        runs.extend(std::iter::repeat(2).take(40));

        let mut reader = TelepenReader::new();
        reader.setCounters(&row_from_runs(&runs), 1).unwrap();

        // Offset 0 must be rejected: its tenth element is wide.
        assert_ne!(
            reader.findStartPattern().ok(),
            Some(0),
            "an element of width 5 against a narrow width of 2 is wide, and \
             must not satisfy a narrow slot of the start pattern"
        );
    }

    /// The stop side had no structural check at all: `findEndPattern` locates
    /// the end by looking for a quiet zone and accepts whatever precedes it.
    /// A region that ends in wide elements is not a Telepen stop.
    #[test]
    fn region_not_ending_in_narrow_elements_is_rejected() {
        // A valid-looking start, then a body that ends wide.
        let mut runs = vec![2u32; 10];
        runs.push(6);
        runs.extend(std::iter::repeat(2).take(20));
        runs.extend(std::iter::repeat(6).take(10));

        let mut reader = TelepenReader::new();
        reader.setCounters(&row_from_runs(&runs), 1).unwrap();

        let start = reader.findStartPattern().unwrap() as usize;
        assert!(
            matches!(
                reader.findEndPattern(start),
                Err(Exceptions::NotFoundException(_))
            ),
            "a region ending in wide elements has no Telepen stop pattern"
        );
    }

    /// Regression test: an entirely-black row makes the "move to first white
    /// pixel" scan consume the whole row, so `setCounters` never records a
    /// transition and `counterLength` stays 0. The final noise-merge then
    /// evaluated `self.counters[self.counterLength - 1]`, underflowing to
    /// `usize::MAX` and panicking with "index out of bounds". The row must be
    /// at least 2000 wide so `minToleratedWidth` (0.1% of the width) is >= 2,
    /// making the trailing `count` of 1 fall into the noise branch.
    #[test]
    fn all_black_row_returns_not_found_instead_of_panicking() {
        let mut row = BitArray::with_size(2048);
        for i in 0..row.get_size() {
            row.set(i);
        }

        let mut reader = TelepenReader::new();
        let result = reader.decode_row(0, &row, &DecodeHints::default());

        assert!(matches!(result, Err(Exceptions::NotFoundException(_))));
    }
}
