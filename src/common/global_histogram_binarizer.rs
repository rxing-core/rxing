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

// package com.google.zxing.common;

// import com.google.zxing.Binarizer;
// import com.google.zxing.LuminanceSource;
// import com.google.zxing.NotFoundException;

use std::borrow::Cow;

use once_cell::unsync::OnceCell;

use crate::common::Result;
use crate::{Binarizer, Exceptions, LuminanceSource};

use super::{BitArray, BitMatrix};

const LUMINANCE_BITS: usize = 5;
const LUMINANCE_SHIFT: usize = 8 - LUMINANCE_BITS;
const LUMINANCE_BUCKETS: usize = 1 << LUMINANCE_BITS;

/**
 * This Binarizer implementation uses the old ZXing global histogram approach. It is suitable
 * for low-end mobile devices which don't have enough CPU or memory to use a local thresholding
 * algorithm. However, because it picks a global black point, it cannot handle difficult shadows
 * and gradients.
 *
 * Faster mobile devices and all desktop applications should probably use HybridBinarizer instead.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
pub struct GlobalHistogramBinarizer<LS: LuminanceSource> {
    //_luminances: Vec<u8>,
    width: usize,
    height: usize,
    source: LS,
    black_matrix: OnceCell<BitMatrix>,
    black_row_cache: Vec<OnceCell<BitArray>>,
}

impl<LS: LuminanceSource> Binarizer for GlobalHistogramBinarizer<LS> {
    type Source = LS;

    fn get_luminance_source(&self) -> &Self::Source {
        &self.source
    }

    // Applies simple sharpening to the row data to improve performance of the 1D Readers.
    fn get_black_row(&self, y: usize) -> Result<Cow<BitArray>> {
        let row = self.black_row_cache[y].get_or_try_init(|| {
            let source = self.get_luminance_source();
            let width = source.get_width();
            let mut row = BitArray::with_size(width);

            // self.initArrays(width);
            let localLuminances = source.get_row(y);
            let mut localBuckets = [0; LUMINANCE_BUCKETS]; //self.buckets.clone();
            for x in 0..width {
                // for (int x = 0; x < width; x++) {
                localBuckets[((localLuminances[x]) >> LUMINANCE_SHIFT) as usize] += 1;
            }
            let blackPoint = Self::estimateBlackPoint(&localBuckets)?;

            if width < 3 {
                // Special case for very small images
                for (x, lum) in localLuminances.iter().enumerate().take(width) {
                    // for x in 0..width {
                    //   for (int x = 0; x < width; x++) {
                    if (*lum as u32) < blackPoint {
                        row.set(x);
                    }
                }
            } else {
                let mut left = localLuminances[0]; // & 0xff;
                let mut center = localLuminances[1]; // & 0xff;
                for x in 1..width - 1 {
                    //   for (int x = 1; x < width - 1; x++) {
                    let right = localLuminances[x + 1];
                    // A simple -1 4 -1 box filter with a weight of 2.
                    if ((center as i64 * 4) - left as i64 - right as i64) / 2 < blackPoint as i64 {
                        row.set(x);
                    }
                    left = center;
                    center = right;
                }
            }

            Ok(row)
        })?;

        Ok(Cow::Borrowed(row))
    }

    // Does not sharpen the data, as this call is intended to only be used by 2D Readers.
    fn get_black_matrix(&self) -> Result<&BitMatrix> {
        let matrix = self
            .black_matrix
            .get_or_try_init(|| Self::build_black_matrix(&self.source))?;
        Ok(matrix)
    }

    fn create_binarizer(&self, source: LS) -> Self {
        Self::new(source)
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

impl<LS: LuminanceSource> GlobalHistogramBinarizer<LS> {
    // const EMPTY: [u8; 0] = [0; 0];

    pub fn new(source: LS) -> Self {
        Self {
            //_luminances: vec![0; source.getWidth()],
            width: source.get_width(),
            height: source.get_height(),
            black_matrix: OnceCell::new(),
            black_row_cache: vec![OnceCell::default(); source.get_height()],
            source,
        }
    }

    fn build_black_matrix(source: &LS) -> Result<BitMatrix> {
        // let source = source.getLuminanceSource();
        let width = source.get_width();
        let height = source.get_height();
        let mut matrix = BitMatrix::new(width as u32, height as u32)?;

        // Quickly calculates the histogram by sampling four rows from the image. This proved to be
        // more robust on the blackbox tests than sampling a diagonal as we used to do.
        // self.initArrays(width);
        let mut localBuckets = [0; LUMINANCE_BUCKETS]; //self.buckets.clone();
        for y in 1..5 {
            // for (int y = 1; y < 5; y++) {
            let row = height * y / 5;
            let localLuminances = source.get_row(row);
            let right = (width * 4) / 5;
            for x in (width / 5)..right {
                let pixel = localLuminances[x];
                localBuckets[(pixel >> LUMINANCE_SHIFT) as usize] += 1;
            }
        }
        let blackPoint = Self::estimateBlackPoint(&localBuckets)?;

        // We delay reading the entire image luminance until the black point estimation succeeds.
        // Although we end up reading four rows twice, it is consistent with our motto of
        // "fail quickly" which is necessary for continuous scanning.
        let localLuminances = source.get_matrix();
        for y in 0..height {
            let offset = y * width;
            for x in 0..width {
                let pixel = localLuminances[offset + x];
                if (pixel as u32) < blackPoint {
                    matrix.set(x as u32, y as u32);
                }
            }
        }

        Ok(matrix)
    }

    fn estimateBlackPoint(buckets: &[u32]) -> Result<u32> {
        // Find the tallest peak in the histogram.
        let numBuckets = buckets.len();
        let mut maxBucketCount = 0;
        let mut firstPeak = 0;
        let mut firstPeakSize = 0;
        for (x, bucket) in buckets.iter().enumerate().take(numBuckets) {
            // for x in 0..numBuckets {
            // for (int x = 0; x < numBuckets; x++) {
            if *bucket > firstPeakSize {
                firstPeak = x;
                firstPeakSize = *bucket;
            }
            if *bucket > maxBucketCount {
                maxBucketCount = *bucket;
            }
        }

        // Find the second-tallest peak which is somewhat far from the tallest peak.
        let mut secondPeak = 0;
        let mut secondPeakScore = 0;
        for (x, bucket) in buckets.iter().enumerate().take(numBuckets) {
            // for x in 0..numBuckets {
            // for (int x = 0; x < numBuckets; x++) {
            let distanceToBiggest = (x as i32 - firstPeak as i32).unsigned_abs();
            // Encourage more distant second peaks by multiplying by square of distance.
            let score = *bucket * distanceToBiggest * distanceToBiggest;
            if score > secondPeakScore {
                secondPeak = x;
                secondPeakScore = score;
            }
        }

        // Make sure firstPeak corresponds to the black peak.
        if firstPeak > secondPeak {
            std::mem::swap(&mut firstPeak, &mut secondPeak);
        }

        // If there is too little contrast in the image to pick a meaningful black point, throw rather
        // than waste time trying to decode the image, and risk false positives.
        if secondPeak - firstPeak <= numBuckets / 16 {
            return Err(Exceptions::not_found_with(
                "secondPeak - firstPeak <= numBuckets / 16 ",
            ));
        }

        // Find a valley between them that is low and closer to the white peak.
        let mut bestValley = secondPeak as isize - 1;
        let mut bestValleyScore = -1;
        let mut x = secondPeak as isize;
        while x > firstPeak as isize {
            // for (int x = secondPeak - 1; x > firstPeak; x--) {
            let fromFirst = x - firstPeak as isize;
            let score = fromFirst
                * fromFirst
                * (secondPeak as isize - x)
                * (maxBucketCount - buckets[x as usize]) as isize;
            if score as i32 > bestValleyScore {
                bestValley = x;
                bestValleyScore = score as i32;
            }
            x -= 1;
        }

        Ok((bestValley as u32) << LUMINANCE_SHIFT)
    }
}
