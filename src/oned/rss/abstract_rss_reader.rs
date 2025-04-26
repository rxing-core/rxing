/*
 * Copyright (C) 2010 ZXing authors
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

use crate::{
    common::Result,
    oned::{one_d_reader, OneDReader},
    Exceptions,
};

/**
 * Superclass of {@link OneDReader} implementations that read barcodes in the RSS family
 * of formats.
 */
pub trait AbstractRSSReaderTrait: OneDReader {
    const MAX_AVG_VARIANCE: f32 = 0.2;
    const MAX_INDIVIDUAL_VARIANCE: f32 = 0.45;

    const MIN_FINDER_PATTERN_RATIO: f32 = 9.5 / 12.0;
    const MAX_FINDER_PATTERN_RATIO: f32 = 12.5 / 14.0;

    fn parseFinderValue(counters: &[u32], finderPatterns: &[[u32; 4]]) -> Result<u32> {
        for (value, pattern) in finderPatterns.iter().enumerate() {
            if one_d_reader::pattern_match_variance(
                counters,
                pattern,
                Self::MAX_INDIVIDUAL_VARIANCE,
            ) < Self::MAX_AVG_VARIANCE
            {
                return Ok(value as u32);
            }
        }
        Err(Exceptions::NOT_FOUND)
    }

    fn increment(array: &mut [u32], errors: &[f32]) {
        let mut index = 0;
        let mut biggestError = errors[0];
        for (i, error) in errors.iter().enumerate().take(array.len()).skip(1) {
            if *error > biggestError {
                biggestError = *error;
                index = i;
            }
        }
        array[index] += 1;
    }

    fn decrement(array: &mut [u32], errors: &[f32]) {
        let mut index = 0;
        let mut biggestError = errors[0];
        for (i, error) in errors.iter().enumerate().take(array.len()).skip(1) {
            if *error < biggestError {
                biggestError = *error;
                index = i;
            }
        }
        array[index] -= 1;
    }

    fn isFinderPattern(counters: &[u32]) -> bool {
        let firstTwoSum = counters[0] + counters[1];
        let sum = firstTwoSum + counters[2] + counters[3];
        let ratio: f32 = (firstTwoSum as f32) / (sum as f32);
        if ratio >= Self::MIN_FINDER_PATTERN_RATIO && ratio <= Self::MAX_FINDER_PATTERN_RATIO {
            // passes ratio test in spec, but see if the counts are unreasonable
            let mut minCounter = u32::MAX;
            let mut maxCounter = u32::MIN;

            for counter in counters {
                maxCounter = std::cmp::max(*counter, maxCounter);
                minCounter = std::cmp::min(*counter, minCounter);
            }
            return maxCounter < 10 * minCounter;
        }
        false
    }
}
