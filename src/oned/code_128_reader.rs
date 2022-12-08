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

use one_d_reader_derive::OneDReader;

use crate::{common::BitArray, BarcodeFormat, Exceptions, RXingResult};

use super::OneDReader;

/**
 * <p>Decodes Code 128 barcodes.</p>
 *
 * @author Sean Owen
 */
#[derive(OneDReader)]
pub struct Code128Reader;

impl OneDReader for Code128Reader {
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
        let convertFNC1 = hints.contains_key(&DecodeHintType::ASSUME_GS1);

        let mut symbologyModifier = 0;

        let startPatternInfo = Self::findStartPattern(row)?;
        let startCode = startPatternInfo[2] as u8;

        let mut rawCodes: Vec<u8> = Vec::with_capacity(20); //new ArrayList<>(20);
        rawCodes.push(startCode);

        let mut codeSet = match startCode {
            // switch (startCode) {
            CODE_START_A => CODE_CODE_A,
            CODE_START_B => CODE_CODE_B,
            CODE_START_C => CODE_CODE_C,
            _ => return Err(Exceptions::FormatException("".to_owned())),
        };

        let mut done = false;
        let mut isNextShifted = false;

        let mut result = String::with_capacity(20); //new StringBuilder(20);

        let mut lastStart = startPatternInfo[0];
        let mut nextStart = startPatternInfo[1];
        let mut counters = [0_u32; 6]; //new int[6];

        let mut lastCode = 0;
        let mut code = 0;
        let mut checksumTotal = startCode as usize;
        let mut multiplier = 0;
        let mut lastCharacterWasPrintable = true;
        let mut upperMode = false;
        let mut shiftUpperMode = false;

        while !done {
            let unshift = isNextShifted;
            isNextShifted = false;

            // Save off last code
            lastCode = code;

            // Decode another code from image
            code = Self::decodeCode(row, &mut counters, nextStart)?;

            rawCodes.push(code);

            // Remember whether the last code was printable or not (excluding CODE_STOP)
            if code != CODE_STOP {
                lastCharacterWasPrintable = true;
            }

            // Add to checksum computation (if not CODE_STOP of course)
            if code != CODE_STOP {
                multiplier += 1;
                checksumTotal += multiplier as usize * code as usize;
            }

            // Advance to where the next code will to start
            lastStart = nextStart;

            nextStart += counters.iter().sum::<u32>() as usize;

            // for counter in counters {
            // // for (int counter : counters) {
            //   nextStart += counter;
            // }

            // Take care of illegal start codes
            match code {
                CODE_START_A | CODE_START_B | CODE_START_C => {
                    return Err(Exceptions::FormatException("".to_owned()))
                }
                _ => {}
            }

            match codeSet {
                CODE_CODE_A => {
                    if code < 64 {
                        if shiftUpperMode == upperMode {
                            result.push((b' ' + code) as char);
                        } else {
                            result.push((b' ' + code + 128) as char);
                        }
                        shiftUpperMode = false;
                    } else if code < 96 {
                        if shiftUpperMode == upperMode {
                            result.push((code - 64) as char);
                        } else {
                            result.push((code + 64) as char);
                        }
                        shiftUpperMode = false;
                    } else {
                        // Don't let CODE_STOP, which always appears, affect whether whether we think the last
                        // code was printable or not.
                        if code != CODE_STOP {
                            lastCharacterWasPrintable = false;
                        }
                        match code {
                            CODE_FNC_1 => {
                                if result.chars().count() == 0 {
                                    // FNC1 at first or second character determines the symbology
                                    symbologyModifier = 1;
                                } else if result.chars().count() == 1 {
                                    symbologyModifier = 2;
                                }
                                if convertFNC1 {
                                    if result.chars().count() == 0 {
                                        // GS1 specification 5.4.3.7. and 5.4.6.4. If the first char after the start code
                                        // is FNC1 then this is GS1-128. We add the symbology identifier.
                                        result.push_str("]C1");
                                    } else {
                                        // GS1 specification 5.4.7.5. Every subsequent FNC1 is returned as ASCII 29 (GS)
                                        result.push(29 as char);
                                    }
                                }
                            }
                            CODE_FNC_2 => symbologyModifier = 4,
                            CODE_FNC_3 =>
                                // do nothing?
                                {}
                            CODE_FNC_4_A => {
                                if !upperMode && shiftUpperMode {
                                    upperMode = true;
                                    shiftUpperMode = false;
                                } else if upperMode && shiftUpperMode {
                                    upperMode = false;
                                    shiftUpperMode = false;
                                } else {
                                    shiftUpperMode = true;
                                }
                            }
                            CODE_SHIFT => {
                                isNextShifted = true;
                                codeSet = CODE_CODE_B;
                            }
                            CODE_CODE_B => codeSet = CODE_CODE_B,
                            CODE_CODE_C => codeSet = CODE_CODE_C,
                            CODE_STOP => done = true,
                            _ => {}
                        }
                    }
                }
                CODE_CODE_B => {
                    if code < 96 {
                        if shiftUpperMode == upperMode {
                            result.push((b' ' + code) as char);
                        } else {
                            result.push((b' ' + code + 128) as char);
                        }
                        shiftUpperMode = false;
                    } else {
                        if code != CODE_STOP {
                            lastCharacterWasPrintable = false;
                        }
                        match code {
                            CODE_FNC_1 => {
                                if result.chars().count() == 0 {
                                    // FNC1 at first or second character determines the symbology
                                    symbologyModifier = 1;
                                } else if result.chars().count() == 1 {
                                    symbologyModifier = 2;
                                }
                                if convertFNC1 {
                                    if result.chars().count() == 0 {
                                        // GS1 specification 5.4.3.7. and 5.4.6.4. If the first char after the start code
                                        // is FNC1 then this is GS1-128. We add the symbology identifier.
                                        result.push_str("]C1");
                                    } else {
                                        // GS1 specification 5.4.7.5. Every subsequent FNC1 is returned as ASCII 29 (GS)
                                        result.push(29 as char);
                                    }
                                }
                            }
                            CODE_FNC_2 => symbologyModifier = 4,
                            CODE_FNC_3 =>
                                // do nothing?
                                {}
                            CODE_FNC_4_B => {
                                if !upperMode && shiftUpperMode {
                                    upperMode = true;
                                    shiftUpperMode = false;
                                } else if upperMode && shiftUpperMode {
                                    upperMode = false;
                                    shiftUpperMode = false;
                                } else {
                                    shiftUpperMode = true;
                                }
                            }
                            CODE_SHIFT => {
                                isNextShifted = true;
                                codeSet = CODE_CODE_A;
                            }
                            CODE_CODE_A => codeSet = CODE_CODE_A,

                            CODE_CODE_C => codeSet = CODE_CODE_C,

                            CODE_STOP => done = true,

                            _ => {}
                        }
                    }
                }
                CODE_CODE_C => {
                    if code < 100 {
                        if code < 10 {
                            result.push('0');
                        }
                        result.push_str(&code.to_string());
                    } else {
                        if code != CODE_STOP {
                            lastCharacterWasPrintable = false;
                        }
                        match code {
                            CODE_FNC_1 => {
                                if result.chars().count() == 0 {
                                    // FNC1 at first or second character determines the symbology
                                    symbologyModifier = 1;
                                } else if result.chars().count() == 1 {
                                    symbologyModifier = 2;
                                }
                                if convertFNC1 {
                                    if result.chars().count() == 0 {
                                        // GS1 specification 5.4.3.7. and 5.4.6.4. If the first char after the start code
                                        // is FNC1 then this is GS1-128. We add the symbology identifier.
                                        result.push_str("]C1");
                                    } else {
                                        // GS1 specification 5.4.7.5. Every subsequent FNC1 is returned as ASCII 29 (GS)
                                        result.push(29 as char);
                                    }
                                }
                            }
                            CODE_CODE_A => codeSet = CODE_CODE_A,

                            CODE_CODE_B => codeSet = CODE_CODE_B,

                            CODE_STOP => done = true,

                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            // Unshift back to another code set if we were shifted
            if unshift {
                codeSet = if codeSet == CODE_CODE_A {
                    CODE_CODE_B
                } else {
                    CODE_CODE_A
                };
            }
        }

        let lastPatternSize = nextStart - lastStart;

        // Check for ample whitespace following pattern, but, to do this we first need to remember that
        // we fudged decoding CODE_STOP since it actually has 7 bars, not 6. There is a black bar left
        // to read off. Would be slightly better to properly read. Here we just skip it:
        nextStart = row.getNextUnset(nextStart);
        if !row.isRange(
            nextStart,
            row.getSize().min(nextStart + (nextStart - lastStart) / 2),
            false,
        )? {
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        // Pull out from sum the value of the penultimate check code
        checksumTotal -= multiplier as usize * lastCode as usize;
        // lastCode is the checksum then:
        if (checksumTotal % 103) as u8 != lastCode {
            return Err(Exceptions::ChecksumException("".to_owned()));
        }

        // Need to pull out the check digits from string
        let resultLength = result.chars().count();
        if resultLength == 0 {
            // false positive
            return Err(Exceptions::NotFoundException("".to_owned()));
        }

        // Only bother if the result had at least one character, and if the checksum digit happened to
        // be a printable character. If it was just interpreted as a control code, nothing to remove.
        if resultLength > 0 && lastCharacterWasPrintable {
            let len_trim = if codeSet == CODE_CODE_C {
                resultLength - 2
            } else {
                resultLength - 1
            };
            let new_str = result.chars().take(len_trim).collect();
            result = new_str;
            // if codeSet == CODE_CODE_C {
            //   result.delete(resultLength - 2, resultLength);
            // } else {
            //   result.delete(resultLength - 1, resultLength);
            // }
        }

        let left: f32 = (startPatternInfo[1] + startPatternInfo[0]) as f32 / 2.0;
        let right: f32 = lastStart as f32 + lastPatternSize as f32 / 2.0;

        let rawCodesSize = rawCodes.len();
        let mut rawBytes = vec![0u8; rawCodesSize];
        for i in 0..rawCodesSize {
            // for (int i = 0; i < rawCodesSize; i++) {
            rawBytes[i] = *rawCodes.get(i).unwrap();
        }
        let mut resultObject = RXingResult::new(
            &result,
            rawBytes,
            vec![
                RXingResultPoint::new(left, rowNumber as f32),
                RXingResultPoint::new(right, rowNumber as f32),
            ],
            BarcodeFormat::CODE_128,
        );

        resultObject.putMetadata(
            RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            RXingResultMetadataValue::SymbologyIdentifier(format!("]C{}", symbologyModifier)),
        );

        Ok(resultObject)
    }
}
impl Code128Reader {
    fn findStartPattern(row: &BitArray) -> Result<[usize; 3], Exceptions> {
        let width = row.getSize();
        let rowOffset = row.getNextSet(0);

        let mut counterPosition = 0;
        let mut counters = [0_u32; 6];
        let mut patternStart = rowOffset;
        let mut isWhite = false;
        let patternLength = counters.len();

        for i in rowOffset..width {
            // for (int i = rowOffset; i < width; i++) {
            if row.get(i) != isWhite {
                counters[counterPosition] += 1;
            } else {
                if counterPosition == patternLength - 1 {
                    let mut bestVariance = MAX_AVG_VARIANCE;
                    let mut bestMatch = -1_isize;
                    for startCode in CODE_START_A..=CODE_START_C {
                        // for (int startCode = CODE_START_A; startCode <= CODE_START_C; startCode++) {
                        let variance = Self::patternMatchVariance(
                            &counters,
                            &CODE_PATTERNS[startCode as usize],
                            MAX_INDIVIDUAL_VARIANCE,
                        );
                        if variance < bestVariance {
                            bestVariance = variance;
                            bestMatch = startCode as isize;
                        }
                    }
                    // Look for whitespace before start pattern, >= 50% of width of start pattern
                    if bestMatch >= 0
                        && row.isRange(
                            0.max(patternStart as isize - (i as isize - patternStart as isize) / 2)
                                as usize,
                            patternStart,
                            false,
                        )?
                    {
                        return Ok([patternStart, i, bestMatch as usize]);
                    }
                    patternStart += (counters[0] + counters[1]) as usize;
                    let slc = &counters[2..(counterPosition - 1 + 2)].to_vec();
                    counters[0..(counterPosition - 1)].copy_from_slice(slc);
                    // System.arraycopy(counters, 2, counters, 0, counterPosition - 1);
                    counters[counterPosition - 1] = 0;
                    counters[counterPosition] = 0;
                    counterPosition -= 1;
                } else {
                    counterPosition += 1;
                }
                counters[counterPosition] = 1;
                isWhite = !isWhite;
            }
        }

        Err(Exceptions::NotFoundException("".to_owned()))
    }

    fn decodeCode(
        row: &BitArray,
        counters: &mut [u32; 6],
        rowOffset: usize,
    ) -> Result<u8, Exceptions> {
        Self::recordPattern(row, rowOffset, counters)?;
        let mut bestVariance = MAX_AVG_VARIANCE; // worst variance we'll accept
        let mut bestMatch = -1_isize;
        for d in 0..CODE_PATTERNS.len() {
            // for (int d = 0; d < CODE_PATTERNS.len(); d++) {
            let pattern = &CODE_PATTERNS[d];
            let variance = Self::patternMatchVariance(counters, &pattern, MAX_INDIVIDUAL_VARIANCE);
            if variance < bestVariance {
                bestVariance = variance;
                bestMatch = d as isize;
            }
        }
        // TODO We're overlooking the fact that the STOP pattern has 7 values, not 6.
        if bestMatch >= 0 {
            Ok(bestMatch as u8)
        } else {
            Err(Exceptions::NotFoundException("".to_owned()))
        }
    }
}

use lazy_static::lazy_static;

lazy_static! {

static ref CODE_PATTERNS: [Vec<u32>; 107] = [
    vec![2, 1, 2, 2, 2, 2], // 0
    vec![2, 2, 2, 1, 2, 2],
    vec![2, 2, 2, 2, 2, 1],
    vec![1, 2, 1, 2, 2, 3],
    vec![1, 2, 1, 3, 2, 2],
    vec![1, 3, 1, 2, 2, 2], // 5
    vec![1, 2, 2, 2, 1, 3],
    vec![1, 2, 2, 3, 1, 2],
    vec![1, 3, 2, 2, 1, 2],
    vec![2, 2, 1, 2, 1, 3],
    vec![2, 2, 1, 3, 1, 2], // 10
    vec![2, 3, 1, 2, 1, 2],
    vec![1, 1, 2, 2, 3, 2],
    vec![1, 2, 2, 1, 3, 2],
    vec![1, 2, 2, 2, 3, 1],
    vec![1, 1, 3, 2, 2, 2], // 15
    vec![1, 2, 3, 1, 2, 2],
    vec![1, 2, 3, 2, 2, 1],
    vec![2, 2, 3, 2, 1, 1],
    vec![2, 2, 1, 1, 3, 2],
    vec![2, 2, 1, 2, 3, 1], // 20
    vec![2, 1, 3, 2, 1, 2],
    vec![2, 2, 3, 1, 1, 2],
    vec![3, 1, 2, 1, 3, 1],
    vec![3, 1, 1, 2, 2, 2],
    vec![3, 2, 1, 1, 2, 2], // 25
    vec![3, 2, 1, 2, 2, 1],
    vec![3, 1, 2, 2, 1, 2],
    vec![3, 2, 2, 1, 1, 2],
    vec![3, 2, 2, 2, 1, 1],
    vec![2, 1, 2, 1, 2, 3], // 30
    vec![2, 1, 2, 3, 2, 1],
    vec![2, 3, 2, 1, 2, 1],
    vec![1, 1, 1, 3, 2, 3],
    vec![1, 3, 1, 1, 2, 3],
    vec![1, 3, 1, 3, 2, 1], // 35
    vec![1, 1, 2, 3, 1, 3],
    vec![1, 3, 2, 1, 1, 3],
    vec![1, 3, 2, 3, 1, 1],
    vec![2, 1, 1, 3, 1, 3],
    vec![2, 3, 1, 1, 1, 3], // 40
    vec![2, 3, 1, 3, 1, 1],
    vec![1, 1, 2, 1, 3, 3],
    vec![1, 1, 2, 3, 3, 1],
    vec![1, 3, 2, 1, 3, 1],
    vec![1, 1, 3, 1, 2, 3], // 45
    vec![1, 1, 3, 3, 2, 1],
    vec![1, 3, 3, 1, 2, 1],
    vec![3, 1, 3, 1, 2, 1],
    vec![2, 1, 1, 3, 3, 1],
    vec![2, 3, 1, 1, 3, 1], // 50
    vec![2, 1, 3, 1, 1, 3],
    vec![2, 1, 3, 3, 1, 1],
    vec![2, 1, 3, 1, 3, 1],
    vec![3, 1, 1, 1, 2, 3],
    vec![3, 1, 1, 3, 2, 1], // 55
    vec![3, 3, 1, 1, 2, 1],
    vec![3, 1, 2, 1, 1, 3],
    vec![3, 1, 2, 3, 1, 1],
    vec![3, 3, 2, 1, 1, 1],
    vec![3, 1, 4, 1, 1, 1], // 60
    vec![2, 2, 1, 4, 1, 1],
    vec![4, 3, 1, 1, 1, 1],
    vec![1, 1, 1, 2, 2, 4],
    vec![1, 1, 1, 4, 2, 2],
    vec![1, 2, 1, 1, 2, 4], // 65
    vec![1, 2, 1, 4, 2, 1],
    vec![1, 4, 1, 1, 2, 2],
    vec![1, 4, 1, 2, 2, 1],
    vec![1, 1, 2, 2, 1, 4],
    vec![1, 1, 2, 4, 1, 2], // 70
    vec![1, 2, 2, 1, 1, 4],
    vec![1, 2, 2, 4, 1, 1],
    vec![1, 4, 2, 1, 1, 2],
    vec![1, 4, 2, 2, 1, 1],
    vec![2, 4, 1, 2, 1, 1], // 75
    vec![2, 2, 1, 1, 1, 4],
    vec![4, 1, 3, 1, 1, 1],
    vec![2, 4, 1, 1, 1, 2],
    vec![1, 3, 4, 1, 1, 1],
    vec![1, 1, 1, 2, 4, 2], // 80
    vec![1, 2, 1, 1, 4, 2],
    vec![1, 2, 1, 2, 4, 1],
    vec![1, 1, 4, 2, 1, 2],
    vec![1, 2, 4, 1, 1, 2],
    vec![1, 2, 4, 2, 1, 1], // 85
    vec![4, 1, 1, 2, 1, 2],
    vec![4, 2, 1, 1, 1, 2],
    vec![4, 2, 1, 2, 1, 1],
    vec![2, 1, 2, 1, 4, 1],
    vec![2, 1, 4, 1, 2, 1], // 90
    vec![4, 1, 2, 1, 2, 1],
    vec![1, 1, 1, 1, 4, 3],
    vec![1, 1, 1, 3, 4, 1],
    vec![1, 3, 1, 1, 4, 1],
    vec![1, 1, 4, 1, 1, 3], // 95
    vec![1, 1, 4, 3, 1, 1],
    vec![4, 1, 1, 1, 1, 3],
    vec![4, 1, 1, 3, 1, 1],
    vec![1, 1, 3, 1, 4, 1],
    vec![1, 1, 4, 1, 3, 1], // 100
    vec![3, 1, 1, 1, 4, 1],
    vec![4, 1, 1, 1, 3, 1],
    vec![2, 1, 1, 4, 1, 2],
    vec![2, 1, 1, 2, 1, 4],
    vec![2, 1, 1, 2, 3, 2], // 105
    vec![2, 3, 3, 1, 1, 1, 2],
];
}

const MAX_AVG_VARIANCE: f32 = 0.25;
const MAX_INDIVIDUAL_VARIANCE: f32 = 0.7;

const CODE_SHIFT: u8 = 98;

const CODE_CODE_C: u8 = 99;
const CODE_CODE_B: u8 = 100;
const CODE_CODE_A: u8 = 101;

const CODE_FNC_1: u8 = 102;
const CODE_FNC_2: u8 = 97;
const CODE_FNC_3: u8 = 96;
const CODE_FNC_4_A: u8 = 101;
const CODE_FNC_4_B: u8 = 100;

const CODE_START_A: u8 = 103;
const CODE_START_B: u8 = 104;
const CODE_START_C: u8 = 105;
const CODE_STOP: u8 = 106;
