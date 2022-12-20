/*
 * Copyright 2009 ZXing authors
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
    common::{detector::MathUtils, BitArray},
    oned::{one_d_reader, OneDReader},
    BarcodeFormat, DecodeHintType, DecodingHintDictionary, Exceptions, RXingResult,
    RXingResultMetadataType, RXingResultMetadataValue, RXingResultPoint, Reader, ResultPoint,
};

use super::{
    rss_utils, AbstractRSSReaderTrait, DataCharacter, DataCharacterTrait, FinderPattern, Pair,
};

/**
 * Decodes RSS-14, including truncated and stacked variants. See ISO/IEC 24724:2006.
 */
pub struct RSS14Reader {
    possibleLeftPairs: Vec<Pair>,
    possibleRightPairs: Vec<Pair>,
    decodeFinderCounters: [u32; 4],
    dataCharacterCounters: [u32; 8],
    oddRoundingErrors: [f32; 4],
    evenRoundingErrors: [f32; 4],
    oddCounts: [u32; 4],
    evenCounts: [u32; 4],
}

impl AbstractRSSReaderTrait for RSS14Reader {}

impl OneDReader for RSS14Reader {
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        let mut row = row.clone();
        let leftPair = self.decodePair(&row, false, rowNumber, hints);
        Self::addOrTally(&mut self.possibleLeftPairs, leftPair);
        row.reverse();
        let rightPair = self.decodePair(&row, true, rowNumber, hints);
        Self::addOrTally(&mut self.possibleRightPairs, rightPair);
        row.reverse();
        for left in &self.possibleLeftPairs {
            // for (Pair left : possibleLeftPairs) {
            if left.getCount() > 1 {
                for right in &self.possibleRightPairs {
                    // for (Pair right : possibleRightPairs) {
                    if right.getCount() > 1 && self.checkChecksum(&left, &right) {
                        return Ok(self.constructRXingResult(&left, &right));
                    }
                }
            }
        }
        return Err(Exceptions::NotFoundException("".to_owned()));
    }
}
impl Reader for RSS14Reader {
    fn decode(&mut self, image: &crate::BinaryBitmap) -> Result<crate::RXingResult, Exceptions> {
        self.decode_with_hints(image, &HashMap::new())
    }

    // Note that we don't try rotation without the try harder flag, even if rotation was supported.
    fn decode_with_hints(
        &mut self,
        image: &crate::BinaryBitmap,
        hints: &DecodingHintDictionary,
    ) -> Result<crate::RXingResult, Exceptions> {
        if let Ok(res) = self.doDecode(image, hints) {
            Ok(res)
        } else {
            let tryHarder = hints.contains_key(&DecodeHintType::TRY_HARDER);
            if tryHarder && image.isRotateSupported() {
                let rotatedImage = image.rotateCounterClockwise();
                let mut result = self.doDecode(&rotatedImage, hints)?;
                // Record that we found it rotated 90 degrees CCW / 270 degrees CW
                let metadata = result.getRXingResultMetadata();
                let mut orientation = 270;
                if metadata.contains_key(&RXingResultMetadataType::ORIENTATION) {
                    // But if we found it reversed in doDecode(), add in that result here:
                    orientation = (orientation
                        + if let Some(crate::RXingResultMetadataValue::Orientation(or)) =
                            metadata.get(&RXingResultMetadataType::ORIENTATION)
                        {
                            *or
                        } else {
                            0
                        })
                        % 360;
                }
                result.putMetadata(
                    RXingResultMetadataType::ORIENTATION,
                    RXingResultMetadataValue::Orientation(orientation),
                );
                // Update result points
                // let points = result.getRXingResultPoints();
                // if points != null {
                let height = rotatedImage.getHeight();
                // for point in result.getRXingResultPointsMut().iter_mut() {
                let total_points = result.getRXingResultPoints().len();
                let points = result.getRXingResultPointsMut();
                for i in 0..total_points {
                    // for (int i = 0; i < points.length; i++) {
                    points[i] = RXingResultPoint::new(
                        height as f32 - points[i].getY() - 1.0,
                        points[i].getX(),
                    );
                }
                // }

                Ok(result)
            } else {
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
        }
    }

    fn reset(&mut self) {
        self.possibleLeftPairs.clear();
        self.possibleRightPairs.clear();
    }
}

impl RSS14Reader {
    const OUTSIDE_EVEN_TOTAL_SUBSET: [u32; 5] = [1, 10, 34, 70, 126];
    const INSIDE_ODD_TOTAL_SUBSET: [u32; 4] = [4, 20, 48, 81];
    const OUTSIDE_GSUM: [u32; 5] = [0, 161, 961, 2015, 2715];
    const INSIDE_GSUM: [u32; 4] = [0, 336, 1036, 1516];
    const OUTSIDE_ODD_WIDEST: [u32; 5] = [8, 6, 4, 3, 1];
    const INSIDE_ODD_WIDEST: [u32; 4] = [2, 4, 6, 8];

    const FINDER_PATTERNS: [[u32; 4]; 9] = [
        [3, 8, 2, 1],
        [3, 5, 5, 1],
        [3, 3, 7, 1],
        [3, 1, 9, 1],
        [2, 7, 4, 1],
        [2, 5, 6, 1],
        [2, 3, 8, 1],
        [1, 5, 7, 1],
        [1, 3, 9, 1],
    ];

    pub fn new() -> Self {
        Self {
            possibleLeftPairs: Vec::new(),
            possibleRightPairs: Vec::new(),
            decodeFinderCounters: [0u32; 4],
            dataCharacterCounters: [0u32; 8],
            oddRoundingErrors: [0.0; 4],
            evenRoundingErrors: [0.0; 4],
            oddCounts: [0u32; 4],  //vec![0u32;4],
            evenCounts: [0u32; 4], //vec![0u32;4],
        }
    }

    fn addOrTally(possiblePairs: &mut Vec<Pair>, pair: Option<Pair>) {
        let Some(pair) = pair else {
      return;
    };

        let mut found = false;
        for other in possiblePairs.iter_mut() {
            // for (Pair other : possiblePairs) {
            if other.getValue() == pair.getValue() {
                other.incrementCount();
                found = true;
                break;
            }
        }
        if !found {
            possiblePairs.push(pair);
        }
    }

    fn constructRXingResult(&self, leftPair: &Pair, rightPair: &Pair) -> RXingResult {
        let symbolValue: u64 = 4537077 * leftPair.getValue() as u64 + rightPair.getValue() as u64;
        let text = symbolValue.to_string();

        let mut buffer = String::with_capacity(14);
        let mut i = 13 - text.chars().count() as isize;
        while i > 0 {
            // for (int i = 13 - text.length(); i > 0; i--) {
            buffer.push('0');

            i -= 1;
        }
        buffer.push_str(&text);

        let mut checkDigit = 0;
        for i in 0..13 {
            // for (int i = 0; i < 13; i++) {
            let digit = buffer.chars().nth(i).unwrap() as u32 - '0' as u32;
            checkDigit += if (i & 0x01) == 0 { 3 * digit } else { digit };
        }
        checkDigit = 10 - (checkDigit % 10);
        if checkDigit == 10 {
            checkDigit = 0;
        }
        buffer.push_str(&checkDigit.to_string());

        let leftPoints = leftPair.getFinderPattern().getRXingResultPoints();
        let rightPoints = rightPair.getFinderPattern().getRXingResultPoints();
        let mut result = RXingResult::new(
            &buffer,
            Vec::new(),
            vec![leftPoints[0], leftPoints[1], rightPoints[0], rightPoints[1]],
            BarcodeFormat::RSS_14,
        );

        result.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier("]e0".to_owned()),
        );

        result
    }

    fn checkChecksum(&self, leftPair: &Pair, rightPair: &Pair) -> bool {
        let checkValue = (leftPair.getChecksumPortion() + 16 * rightPair.getChecksumPortion()) % 79;
        let mut targetCheckValue =
            9 * leftPair.getFinderPattern().getValue() + rightPair.getFinderPattern().getValue();
        if targetCheckValue > 72 {
            targetCheckValue -= 1;
        }
        if targetCheckValue > 8 {
            targetCheckValue -= 1;
        }
        checkValue == targetCheckValue
    }

    fn decodePair(
        &mut self,
        row: &BitArray,
        right: bool,
        rowNumber: u32,
        hints: &DecodingHintDictionary,
    ) -> Option<Pair> {
        let pos_pair = || -> Result<Pair, Exceptions> {
            let startEnd = self.findFinderPattern(row, right)?;
            let pattern = self.parseFoundFinderPattern(row, rowNumber, right, &startEnd)?;

            // RXingResultPointCallback resultPointCallback = hints == null ? null :
            //     (RXingResultPointCallback) hints.get(DecodeHintType.NEED_RESULT_POINT_CALLBACK);

            // if (resultPointCallback != null) {
            //   startEnd = pattern.getStartEnd();
            //   float center = (startEnd[0] + startEnd[1] - 1) / 2.0f;
            //   if (right) {
            //     // row is actually reversed
            //     center = row.getSize() - 1 - center;
            //   }
            //   resultPointCallback.foundPossibleRXingResultPoint(new RXingResultPoint(center, rowNumber));
            // }

            let outside = self.decodeDataCharacter(row, &pattern, true)?;
            let inside = self.decodeDataCharacter(row, &pattern, false)?;

            // todo!("must add callback");

            Ok(Pair::new(
                1597 * outside.getValue() + inside.getValue(),
                outside.getChecksumPortion() + 4 * inside.getChecksumPortion(),
                pattern,
            ))
        }();
        if pos_pair.is_err() {
            None
        } else {
            Some(pos_pair.unwrap())
        }
    }

    fn decodeDataCharacter(
        &mut self,
        row: &BitArray,
        pattern: &FinderPattern,
        outsideChar: bool,
    ) -> Result<DataCharacter, Exceptions> {
        let counters = &mut self.dataCharacterCounters; //[0_u32; 8]; //self.getDataCharacterCounters();
                                                        //Arrays.fill(counters, 0);
                                                        // counters.fill(0);
        counters.fill(0);

        if outsideChar {
            one_d_reader::recordPatternInReverse(row, pattern.getStartEnd()[0], &mut counters[..])?;
        } else {
            one_d_reader::recordPattern(row, pattern.getStartEnd()[1], &mut counters[..])?;
            // reverse it
            let mut i = 0;
            let mut j = counters.len() - 1;
            while i < j {
                // for (int i = 0, j = counters.length - 1; i < j; i++, j--) {

                counters.swap(i, j);

                i += 1;
                j -= 1;
            }
        }

        let numModules = if outsideChar { 16 } else { 15 };

        let elementWidth: f32 = counters.iter().sum::<u32>() as f32 / numModules as f32;

        // let  oddCounts = &mut self.oddCounts;//[0u32; 4]; //self.getOddCounts();
        // let  evenCounts = //&mut self.evenCounts;//[0u32; 4]; // self.getEvenCounts();
        // let  oddRoundingErrors = //&mut self.oddRoundingErrors;//[0f32; 4]; // self.getOddRoundingErrors();
        // let  evenRoundingErrors = &mut self.evenRoundingErrors;//[0f32; 4]; // self.getEvenRoundingErrors();

        for i in 0..counters.len() {
            // for (int i = 0; i < counters.length; i++) {
            let value: f32 = counters[i] as f32 / elementWidth;
            let mut count = (value + 0.5) as u32; // Round
            if count < 1 {
                count = 1;
            } else if count > 8 {
                count = 8;
            }
            let offset = i / 2;
            if (i & 0x01) == 0 {
                self.oddCounts[offset] = count;
                self.oddRoundingErrors[offset] = value - count as f32;
            } else {
                self.evenCounts[offset] = count;
                self.evenRoundingErrors[offset] = value - count as f32;
            }
        }

        self.adjustOddEvenCounts(outsideChar, numModules)?;

        let mut oddSum = 0;
        let mut oddChecksumPortion = 0;
        for i in (0..self.oddCounts.len()).rev() {
            // for (int i = oddCounts.length - 1; i >= 0; i--) {
            oddChecksumPortion *= 9;
            oddChecksumPortion += &self.oddCounts[i];
            oddSum += &self.oddCounts[i];
        }
        let mut evenChecksumPortion = 0;
        let mut evenSum = 0;
        for i in (0..self.evenCounts.len()).rev() {
            // for (int i = evenCounts.length - 1; i >= 0; i--) {
            evenChecksumPortion *= 9;
            evenChecksumPortion += self.evenCounts[i];
            evenSum += self.evenCounts[i];
        }
        let checksumPortion = oddChecksumPortion + 3 * evenChecksumPortion;

        if outsideChar {
            if (oddSum & 0x01) != 0 || oddSum > 12 || oddSum < 4 {
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
            let group = ((12 - oddSum) / 2) as usize;
            let oddWidest = Self::OUTSIDE_ODD_WIDEST[group];
            let evenWidest = 9 - oddWidest;
            let vOdd = rss_utils::getRSSvalue(&self.oddCounts, oddWidest, false);
            let vEven = rss_utils::getRSSvalue(&self.evenCounts, evenWidest, true);
            let tEven = Self::OUTSIDE_EVEN_TOTAL_SUBSET[group];
            let gSum = Self::OUTSIDE_GSUM[group];
            return Ok(DataCharacter::new(
                vOdd * tEven + vEven + gSum,
                checksumPortion,
            ));
        } else {
            if (evenSum & 0x01) != 0 || evenSum > 10 || evenSum < 4 {
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
            let group = ((10 - evenSum) / 2) as usize;
            let oddWidest = Self::INSIDE_ODD_WIDEST[group];
            let evenWidest = 9 - oddWidest;
            let vOdd = rss_utils::getRSSvalue(&self.oddCounts, oddWidest, true);
            let vEven = rss_utils::getRSSvalue(&self.evenCounts, evenWidest, false);
            let tOdd = Self::INSIDE_ODD_TOTAL_SUBSET[group];
            let gSum = Self::INSIDE_GSUM[group];
            return Ok(DataCharacter::new(
                vEven * tOdd + vOdd + gSum,
                checksumPortion,
            ));
        }
    }

    fn findFinderPattern(
        &mut self,
        row: &BitArray,
        rightFinderPattern: bool,
    ) -> Result<[usize; 2], Exceptions> {
        // let  counters = self.getDecodeFinderCounters();
        // counters[0] = 0;
        // counters[1] = 0;
        // counters[2] = 0;
        // counters[3] = 0;
        let counters = &mut self.decodeFinderCounters;
        counters.fill(0);

        let width = row.getSize();
        let mut isWhite = false;
        let mut rowOffset = 0;
        while rowOffset < width {
            isWhite = !row.get(rowOffset);
            if rightFinderPattern == isWhite {
                // Will encounter white first when searching for right finder pattern
                break;
            }
            rowOffset += 1;
        }

        let mut counterPosition = 0;
        let mut patternStart = rowOffset;
        for x in rowOffset..width {
            // for (int x = rowOffset; x < width; x++) {
            if row.get(x) != isWhite {
                counters[counterPosition] += 1;
            } else {
                if counterPosition == 3 {
                    if Self::isFinderPattern(counters) {
                        return Ok([patternStart, x]);
                    }
                    patternStart += (counters[0] + counters[1]) as usize;
                    counters[0] = counters[2];
                    counters[1] = counters[3];
                    counters[2] = 0;
                    counters[3] = 0;
                    counterPosition -= 1;
                } else {
                    counterPosition += 1;
                }
                counters[counterPosition] = 1;
                isWhite = !isWhite;
            }
        }
        return Err(Exceptions::NotFoundException("".to_owned()));
    }

    fn parseFoundFinderPattern(
        &mut self,
        row: &BitArray,
        rowNumber: u32,
        right: bool,
        startEnd: &[usize],
    ) -> Result<FinderPattern, Exceptions> {
        // Actually we found elements 2-5
        let firstIsBlack = row.get(startEnd[0]);
        let mut firstElementStart = startEnd[0] as isize - 1;
        // Locate element 1
        while firstElementStart >= 0 && firstIsBlack != row.get(firstElementStart as usize) {
            firstElementStart -= 1;
        }
        firstElementStart += 1;
        let firstCounter = startEnd[0] - firstElementStart as usize;
        let mut counters = &mut self.decodeFinderCounters;
        let counter_len = counters.len();
        let slc = counters[0..counter_len - 1].to_vec();
        counters[1..counter_len].copy_from_slice(&slc);
        // Make 'counters' hold 1-4
        // let counters = self.getDecodeFinderCounters();
        // System.arraycopy(counters, 0, counters, 1, counters.length - 1);
        // counters.fill(0);

        counters[0] = firstCounter as u32;
        let value = Self::parseFinderValue(counters, &Self::FINDER_PATTERNS)?;
        let mut start = firstElementStart as usize;
        let mut end = startEnd[1];
        if right {
            // row is actually reversed
            start = row.getSize() - 1 - start;
            end = row.getSize() - 1 - end;
        }

        Ok(FinderPattern::new(
            value,
            [firstElementStart as usize, startEnd[1]],
            start,
            end,
            rowNumber,
        ))
    }

    fn adjustOddEvenCounts(
        &mut self,
        outsideChar: bool,
        numModules: u32,
    ) -> Result<(), Exceptions> {
        let oddSum = self.oddCounts.iter().sum::<u32>(); //MathUtils.sum(getOddCounts());
        let evenSum = self.evenCounts.iter().sum::<u32>(); //MathUtils.sum(getEvenCounts());

        let mut incrementOdd = false;
        let mut decrementOdd = false;
        let mut incrementEven = false;
        let mut decrementEven = false;

        if outsideChar {
            if oddSum > 12 {
                decrementOdd = true;
            } else if oddSum < 4 {
                incrementOdd = true;
            }
            if evenSum > 12 {
                decrementEven = true;
            } else if evenSum < 4 {
                incrementEven = true;
            }
        } else {
            if oddSum > 11 {
                decrementOdd = true;
            } else if oddSum < 5 {
                incrementOdd = true;
            }
            if evenSum > 10 {
                decrementEven = true;
            } else if evenSum < 4 {
                incrementEven = true;
            }
        }

        let mismatch = oddSum as i32 + evenSum as i32 - numModules as i32;
        let oddParityBad = (oddSum & 0x01) == (if outsideChar { 1 } else { 0 });
        let evenParityBad = (evenSum & 0x01) == 1;
        /*if (mismatch == 2) {
          if (!(oddParityBad && evenParityBad)) {
            throw ReaderException.getInstance();
          }
          decrementOdd = true;
          decrementEven = true;
        } else if (mismatch == -2) {
          if (!(oddParityBad && evenParityBad)) {
            throw ReaderException.getInstance();
          }
          incrementOdd = true;
          incrementEven = true;
        } else */
        match mismatch {
            1 => {
                if oddParityBad {
                    if evenParityBad {
                        return Err(Exceptions::NotFoundException("".to_owned()));
                    }
                    decrementOdd = true;
                } else {
                    if !evenParityBad {
                        return Err(Exceptions::NotFoundException("".to_owned()));
                    }
                    decrementEven = true;
                }
            }
            -1 => {
                if oddParityBad {
                    if evenParityBad {
                        return Err(Exceptions::NotFoundException("".to_owned()));
                    }
                    incrementOdd = true;
                } else {
                    if !evenParityBad {
                        return Err(Exceptions::NotFoundException("".to_owned()));
                    }
                    incrementEven = true;
                }
            }
            0 => {
                if oddParityBad {
                    if !evenParityBad {
                        return Err(Exceptions::NotFoundException("".to_owned()));
                    }
                    // Both bad
                    if oddSum < evenSum {
                        incrementOdd = true;
                        decrementEven = true;
                    } else {
                        decrementOdd = true;
                        incrementEven = true;
                    }
                } else {
                    if evenParityBad {
                        return Err(Exceptions::NotFoundException("".to_owned()));
                    }
                    // Nothing to do!
                }
            }
            _ => return Err(Exceptions::NotFoundException("".to_owned())),
        }

        if incrementOdd {
            if decrementOdd {
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
            Self::increment(&mut self.oddCounts, &self.oddRoundingErrors);
        }
        if decrementOdd {
            Self::decrement(&mut self.oddCounts, &self.oddRoundingErrors);
        }
        if incrementEven {
            if decrementEven {
                return Err(Exceptions::NotFoundException("".to_owned()));
            }
            Self::increment(&mut self.evenCounts, &self.evenRoundingErrors);
        }
        if decrementEven {
            Self::decrement(&mut self.evenCounts, &self.evenRoundingErrors);
        }
        Ok(())
    }
}
