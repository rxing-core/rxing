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

use rxing_one_d_proc_derive::OneDReader;

use crate::common::{BitArray, Result};
use crate::DecodeHintValue;
use crate::Exceptions;
use crate::RXingResult;
use crate::{point, BarcodeFormat};

use super::OneDReader;

/**
 * <p>Decodes Codabar barcodes.</p>
 *
 * @author Bas Vijfwinkel
 * @author David Walker
 */
#[derive(OneDReader)]
pub struct CodaBarReader {
    // Keep some instance variables to avoid reallocations
    decodeRowRXingResult: String,
    counters: Vec<u32>,
    counterLength: usize,
}

impl Default for CodaBarReader {
    fn default() -> Self {
        Self {
            decodeRowRXingResult: String::with_capacity(20),
            counters: vec![0; 80],
            counterLength: 0,
        }
    }
}

impl OneDReader for CodaBarReader {
    fn decode_row(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult> {
        self.counters.fill(0);
        // Arrays.fill(counters, 0);
        self.setCounters(row)?;
        let startOffset = self.findStartPattern()? as usize;
        let mut nextStart = startOffset;

        self.decodeRowRXingResult.clear();
        loop {
            let charOffset = self.toNarrowWidePattern(nextStart);
            if charOffset == -1 {
                return Err(Exceptions::NOT_FOUND);
            }
            // Hack: We store the position in the alphabet table into a
            // StringBuilder, so that we can access the decoded patterns in
            // validatePattern. We'll translate to the actual characters later.
            self.decodeRowRXingResult
                .push(char::from_u32(charOffset as u32).ok_or(Exceptions::PARSE)?);
            nextStart += 8;
            // Stop as soon as we see the end character.
            if self.decodeRowRXingResult.chars().count() > 1
                && Self::arrayContains(
                    &Self::STARTEND_ENCODING,
                    Self::ALPHABET[charOffset as usize],
                )
            {
                break;
            }

            // no fixed end pattern so keep on reading while data is available
            if nextStart >= self.counterLength {
                break;
            }
        }
        // Look for whitespace after pattern:
        let trailingWhitespace = self.counters[nextStart - 1];
        let mut lastPatternSize = 0;
        for i in -8..-1 {
            lastPatternSize += self.counters[(nextStart as isize + i) as usize];
        }

        // We need to see whitespace equal to 50% of the last pattern size,
        // otherwise this is probably a false positive. The exception is if we are
        // at the end of the row. (I.e. the barcode barely fits.)
        if nextStart < self.counterLength && trailingWhitespace < lastPatternSize / 2 {
            return Err(Exceptions::NOT_FOUND);
        }

        self.validatePattern(startOffset)?;

        // Translate character table offsets to actual characters.
        for i in 0..self.decodeRowRXingResult.chars().count() {
            // for (int i = 0; i < decodeRowRXingResult.length(); i++) {
            self.decodeRowRXingResult.replace_range(
                i..=i,
                &Self::ALPHABET[self
                    .decodeRowRXingResult
                    .chars()
                    .nth(i)
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                    as usize]
                    .to_string(),
            );
        }
        // Ensure a valid start and end character
        let startchar = self
            .decodeRowRXingResult
            .chars()
            .next()
            .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
        if !Self::arrayContains(&Self::STARTEND_ENCODING, startchar) {
            return Err(Exceptions::NOT_FOUND);
        }
        let endchar = self
            .decodeRowRXingResult
            .chars()
            .nth(self.decodeRowRXingResult.chars().count() - 1)
            .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?;
        if !Self::arrayContains(&Self::STARTEND_ENCODING, endchar) {
            return Err(Exceptions::NOT_FOUND);
        }

        // remove stop/start characters character and check if a long enough string is contained
        if (self.decodeRowRXingResult.chars().count()) <= Self::MIN_CHARACTER_LENGTH as usize {
            // Almost surely a false positive ( start + stop + at least 1 character)
            return Err(Exceptions::NOT_FOUND);
        }

        if !matches!(
            hints.get(&DecodeHintType::RETURN_CODABAR_START_END),
            Some(DecodeHintValue::ReturnCodabarStartEnd(true))
        ) {
            self.decodeRowRXingResult =
                self.decodeRowRXingResult[1..self.decodeRowRXingResult.len() - 1].to_owned();
        }

        let mut runningCount = 0;
        runningCount += self.counters.iter().take(startOffset).sum::<u32>();
        // for i in 0..startOffset {
        //     runningCount += self.counters[i];
        // }
        let left: f32 = runningCount as f32;
        runningCount += self
            .counters
            .iter()
            .skip(startOffset)
            .take(nextStart)
            .sum::<u32>();
        // for i in startOffset..(nextStart - 1) {
        //     runningCount += self.counters[i];
        // }
        let right: f32 = runningCount as f32;

        let mut result = RXingResult::new(
            &self.decodeRowRXingResult,
            Vec::new(),
            vec![
                point(left, rowNumber as f32),
                point(right, rowNumber as f32),
            ],
            BarcodeFormat::CODABAR,
        );

        result.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier("]F0".to_owned()),
        );

        Ok(result)
    }
}
impl CodaBarReader {
    // These values are critical for determining how permissive the decoding
    // will be. All stripe sizes must be within the window these define, as
    // compared to the average stripe size.
    pub const MAX_ACCEPTABLE: f32 = 2.0;
    pub const PADDING: f32 = 1.5;

    // const ALPHABET_STRING : &str= "0123456789-$:/.+ABCD";
    pub const ALPHABET: [char; 20] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '$', ':', '/', '.', '+', 'A', 'B',
        'C', 'D',
    ];

    /**
     * These represent the encodings of characters, as patterns of wide and narrow bars. The 7 least-significant bits of
     * each int correspond to the pattern of wide and narrow, with 1s representing "wide" and 0s representing narrow.
     */
    pub const CHARACTER_ENCODINGS: [u32; 20] = [
        0x003, 0x006, 0x009, 0x060, 0x012, 0x042, 0x021, 0x024, 0x030, 0x048, // 0-9
        0x00c, 0x018, 0x045, 0x051, 0x054, 0x015, 0x01A, 0x029, 0x00B, 0x00E, // -$:/.+ABCD
    ];

    // minimal number of characters that should be present (including start and stop characters)
    // under normal circumstances this should be set to 3, but can be set higher
    // as a last-ditch attempt to reduce false positives.
    pub const MIN_CHARACTER_LENGTH: u32 = 3;

    // official start and end patterns
    pub const STARTEND_ENCODING: [char; 4] = ['A', 'B', 'C', 'D'];
    // some Codabar generator allow the Codabar string to be closed by every
    // character. This will cause lots of false positives!

    // some industries use a checksum standard but this is not part of the original Codabar standard
    // for more information see : http://www.mecsw.com/specs/codabar.html

    pub fn new() -> Self {
        Self {
            decodeRowRXingResult: String::with_capacity(20),
            counters: vec![0; 80], //Vec::with_capacity(80),
            counterLength: 0,
        }
    }

    fn validatePattern(&self, start: usize) -> Result<()> {
        // First, sum up the total size of our four categories of stripe sizes;
        let mut sizes = [0, 0, 0, 0];
        let mut counts = [0, 0, 0, 0];
        let end = self.decodeRowRXingResult.chars().count() - 1;

        // We break out of this loop in the middle, in order to handle
        // inter-character spaces properly.
        let mut pos = start;
        for i in 0..=end {
            // for (int i = 0; i <= end; i++) {
            let mut pattern = Self::CHARACTER_ENCODINGS[self
                .decodeRowRXingResult
                .chars()
                .nth(i)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                as usize];
            for j in (0_usize..=6).rev() {
                // Even j = bars, while odd j = spaces. Categories 2 and 3 are for
                // long stripes, while 0 and 1 are for short stripes.
                let category = (j & 1) + ((pattern as usize) & 1) * 2;
                sizes[category] += self.counters[(pos + j)];
                counts[category] += 1;
                pattern >>= 1;
            }
            // We ignore the inter-character space - it could be of any size.
            pos += 8;
        }

        // Calculate our allowable size thresholds using fixed-point math.
        let mut maxes = [0.0; 4]; //new float[4];
        let mut mins = [0.0; 4]; //new float[4];
                                 // Define the threshold of acceptability to be the midpoint between the
                                 // average small stripe and the average large stripe. No stripe lengths
                                 // should be on the "wrong" side of that line.
        for i in 0..2 {
            // for (int i = 0; i < 2; i++) {
            mins[i] = 0.0; // Accept arbitrarily small "short" stripes.
            mins[i + 2] = ((sizes[i] as f32) / (counts[i] as f32)
                + (sizes[i + 2] as f32) / (counts[i + 2] as f32))
                / 2.0;
            maxes[i] = mins[i + 2];
            maxes[i + 2] = ((sizes[i + 2] as f32) * Self::MAX_ACCEPTABLE + Self::PADDING)
                / (counts[i + 2] as f32);
        }

        // Now verify that all of the stripes are within the thresholds.
        pos = start;
        for i in 0..=end {
            // for (int i = 0; i <= end; i++) {
            let mut pattern = Self::CHARACTER_ENCODINGS[self
                .decodeRowRXingResult
                .chars()
                .nth(i)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                as usize];
            for j in (0..=6).rev() {
                // Even j = bars, while odd j = spaces. Categories 2 and 3 are for
                // long stripes, while 0 and 1 are for short stripes.
                let category = (j & 1) + ((pattern as usize) & 1) * 2;
                let size = self.counters[(pos + j)];
                if (size as f32) < mins[category] || (size as f32) > maxes[category] {
                    return Err(Exceptions::NOT_FOUND);
                }
                pattern >>= 1;
            }
            pos += 8;
        }
        Ok(())
    }

    /**
     * Records the size of all runs of white and black pixels, starting with white.
     * This is just like recordPattern, except it records all the counters, and
     * uses our builtin "counters" member for storage.
     * @param row row to count from
     */
    fn setCounters(&mut self, row: &BitArray) -> Result<()> {
        self.counterLength = 0;
        // Start from the first white bit.
        let mut i = row.getNextUnset(0);
        let end = row.get_size();
        if i >= end {
            return Err(Exceptions::NOT_FOUND);
        }
        let mut isWhite = true;
        let mut count = 0;
        while i < end {
            if row.get(i) != isWhite {
                count += 1;
            } else {
                self.counterAppend(count);
                count = 1;
                isWhite = !isWhite;
            }
            i += 1;
        }
        self.counterAppend(count);
        Ok(())
    }

    fn counterAppend(&mut self, e: u32) {
        self.counters[self.counterLength] = e;
        self.counterLength += 1;
        if self.counterLength >= self.counters.len() {
            let mut temp = vec![0; self.counterLength * 2];
            temp[0..self.counterLength].clone_from_slice(&self.counters[..]);
            self.counters = temp;
        }
    }

    fn findStartPattern(&mut self) -> Result<u32> {
        let mut i = 1;
        while i < self.counterLength {
            // for (int i = 1; i < counterLength; i += 2) {
            let charOffset = self.toNarrowWidePattern(i);
            if charOffset != -1
                && Self::arrayContains(
                    &Self::STARTEND_ENCODING,
                    Self::ALPHABET[charOffset as usize],
                )
            {
                // Look for whitespace before start pattern, >= 50% of width of start pattern
                // We make an exception if the whitespace is the first element.
                let mut patternSize = 0;
                for j in i..(i + 7) {
                    patternSize += self.counters[j];
                }
                if i == 1 || self.counters[i - 1] >= patternSize / 2 {
                    return Ok(i as u32);
                }
            }

            i += 2;
        }
        Err(Exceptions::NOT_FOUND)
    }

    pub fn arrayContains(array: &[char], key: char) -> bool {
        array.contains(&key)
    }

    // Assumes that counters[position] is a bar.
    fn toNarrowWidePattern(&mut self, position: usize) -> i32 {
        let end = position + 7;
        if end >= self.counterLength {
            return -1;
        }

        let theCounters = &self.counters;

        let mut maxBar = 0;
        let mut minBar = u32::MAX;
        let mut j = position;
        while j < end {
            let currentCounter = theCounters[j];
            if currentCounter < minBar {
                minBar = currentCounter;
            }
            if currentCounter > maxBar {
                maxBar = currentCounter;
            }

            j += 2;
        }
        let thresholdBar = (minBar + maxBar) / 2;

        let mut maxSpace = 0;
        let mut minSpace = u32::MAX;
        let mut j = position + 1;
        while j < end {
            let currentCounter = theCounters[j];
            minSpace = std::cmp::min(currentCounter, minSpace);
            maxSpace = std::cmp::max(currentCounter, maxSpace);

            j += 2;
        }
        let thresholdSpace = (minSpace + maxSpace) / 2;

        let mut bitmask = 1 << 7;
        let mut pattern = 0;
        for i in 0..7 {
            let threshold = if (i & 1) == 0 {
                thresholdBar
            } else {
                thresholdSpace
            };
            bitmask >>= 1;
            if theCounters[position + i] > threshold {
                pattern |= bitmask;
            }
        }

        for i in 0..Self::CHARACTER_ENCODINGS.len() {
            if Self::CHARACTER_ENCODINGS[i] == pattern {
                return i as i32;
            }
        }
        -1
    }
}
