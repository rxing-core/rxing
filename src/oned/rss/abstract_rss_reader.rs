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

    // private final int[] decodeFinderCounters;
    // private final int[] dataCharacterCounters;
    // private final float[] oddRoundingErrors;
    // private final float[] evenRoundingErrors;
    // private final int[] oddCounts;
    // private final int[] evenCounts;

    // protected AbstractRSSReader() {
    //   decodeFinderCounters = new int[4];
    //   dataCharacterCounters = new int[8];
    //   oddRoundingErrors = new float[4];
    //   evenRoundingErrors = new float[4];
    //   oddCounts = new int[dataCharacterCounters.length / 2];
    //   evenCounts = new int[dataCharacterCounters.length / 2];
    // }

    fn parseFinderValue(counters: &[u32], finderPatterns: &[[u32; 4]]) -> Result<u32, Exceptions> {
        for value in 0..finderPatterns.len() {
            // for (int value = 0; value < finderPatterns.length; value++) {
            if one_d_reader::patternMatchVariance(
                counters,
                &finderPatterns[value],
                Self::MAX_INDIVIDUAL_VARIANCE,
            ) < Self::MAX_AVG_VARIANCE
            {
                return Ok(value as u32);
            }
        }
        Err(Exceptions::NotFoundException("".to_owned()))
    }

    /**
     * @param array values to sum
     * @return sum of values
     * @deprecated call {@link MathUtils#sum(int[])}
     */
    #[deprecated]
    fn count(array: &[u32]) -> u32 {
        array.iter().sum::<u32>()
    }

    fn increment(array: &mut [u32], errors: &[f32]) {
        let mut index = 0;
        let mut biggestError = errors[0];
        for i in 1..array.len() {
            // for (int i = 1; i < array.length; i++) {
            if errors[i] > biggestError {
                biggestError = errors[i];
                index = i;
            }
        }
        array[index] += 1;
    }

    fn decrement(array: &mut [u32], errors: &[f32]) {
        let mut index = 0;
        let mut biggestError = errors[0];
        for i in 1..array.len() {
            // for (int i = 1; i < array.length; i++) {
            if errors[i] < biggestError {
                biggestError = errors[i];
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
                // for (int counter : counters) {
                if *counter > maxCounter {
                    maxCounter = *counter;
                }
                if *counter < minCounter {
                    minCounter = *counter;
                }
            }
            return maxCounter < 10 * minCounter;
        }
        return false;
    }
}
