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

use std::{borrow::Cow, rc::Rc};

use once_cell::unsync::OnceCell;

use crate::Result;
use crate::{binarizer::Binarizer, Exceptions, LuminanceSource};

use super::{BitArray, BitMatrix};

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
pub struct GlobalHistogramBinarizer {
    //_luminances: Vec<u8>,
    width: usize,
    height: usize,
    source: Box<dyn LuminanceSource>,
    black_matrix: OnceCell<BitMatrix>,
    black_row_cache: Vec<OnceCell<BitArray>>,
}

impl Binarizer for GlobalHistogramBinarizer {
    fn getLuminanceSource(&self) -> &Box<dyn LuminanceSource> {
        &self.source
    }

    // Applies simple sharpening to the row data to improve performance of the 1D Readers.
    fn getBlackRow(&self, y: usize) -> Result<Cow<BitArray>> {
        let row = self.black_row_cache[y].get_or_try_init(|| {
            let source = self.getLuminanceSource();
            let width = source.getWidth();
            let mut row = BitArray::with_size(width);

            // self.initArrays(width);
            let localLuminances = source.getRow(y);
            let mut localBuckets = [0; GlobalHistogramBinarizer::LUMINANCE_BUCKETS]; //self.buckets.clone();
            for x in 0..width {
                // for (int x = 0; x < width; x++) {
                localBuckets[((localLuminances[x]) >> GlobalHistogramBinarizer::LUMINANCE_SHIFT)
                    as usize] += 1;
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
    fn getBlackMatrix(&self) -> Result<&BitMatrix> {
        let matrix = self
            .black_matrix
            .get_or_try_init(|| Self::build_black_matrix(&self.source))?;
        Ok(matrix)
    }

    fn createBinarizer(&self, source: Box<dyn crate::LuminanceSource>) -> Rc<dyn Binarizer> {
        Rc::new(GlobalHistogramBinarizer::new(source))
    }

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
    }
}

impl GlobalHistogramBinarizer {
    const LUMINANCE_BITS: usize = 5;
    const LUMINANCE_SHIFT: usize = 8 - GlobalHistogramBinarizer::LUMINANCE_BITS;
    const LUMINANCE_BUCKETS: usize = 1 << GlobalHistogramBinarizer::LUMINANCE_BITS;
    // const EMPTY: [u8; 0] = [0; 0];

    pub fn new(source: Box<dyn LuminanceSource>) -> Self {
        Self {
            //_luminances: vec![0; source.getWidth()],
            width: source.getWidth(),
            height: source.getHeight(),
            black_matrix: OnceCell::new(),
            black_row_cache: vec![OnceCell::default(); source.getHeight()],
            source,
        }
    }

    fn build_black_matrix(source: &Box<dyn LuminanceSource>) -> Result<BitMatrix> {
        // let source = source.getLuminanceSource();
        let width = source.getWidth();
        let height = source.getHeight();
        let mut matrix = BitMatrix::new(width as u32, height as u32)?;

        // Quickly calculates the histogram by sampling four rows from the image. This proved to be
        // more robust on the blackbox tests than sampling a diagonal as we used to do.
        // self.initArrays(width);
        let mut localBuckets = [0; GlobalHistogramBinarizer::LUMINANCE_BUCKETS]; //self.buckets.clone();
        for y in 1..5 {
            // for (int y = 1; y < 5; y++) {
            let row = height * y / 5;
            let localLuminances = source.getRow(row);
            let right = (width * 4) / 5;
            let mut x = width / 5;
            while x < right {
                //   for (int x = width / 5; x < right; x++) {
                let pixel = localLuminances[x];
                localBuckets[(pixel >> GlobalHistogramBinarizer::LUMINANCE_SHIFT) as usize] += 1;
                x += 1;
            }
        }
        let blackPoint = Self::estimateBlackPoint(&localBuckets)?;

        // We delay reading the entire image luminance until the black point estimation succeeds.
        // Although we end up reading four rows twice, it is consistent with our motto of
        // "fail quickly" which is necessary for continuous scanning.
        let localLuminances = source.getMatrix();
        for y in 0..height {
            // for (int y = 0; y < height; y++) {
            let offset = y * width;
            for x in 0..width {
                //   for (int x = 0; x < width; x++) {
                let pixel = localLuminances[offset + x];
                if (pixel as u32) < blackPoint {
                    matrix.set(x as u32, y as u32);
                }
            }
        }

        Ok(matrix)
    }

    // fn initArrays(&mut self, luminanceSize: usize) {
    //     // if self.luminances.len() < luminanceSize {
    //     //     self.luminances = ;
    //     // }
    //     // // for x in 0..GlobalHistogramBinarizer::LUMINANCE_BUCKETS {
    //     //     // for (int x = 0; x < LUMINANCE_BUCKETS; x++) {
    //     //     self.buckets[x] = 0;
    //     // }
    // }

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
        let mut bestValley = secondPeak - 1;
        let mut bestValleyScore = -1;
        let mut x = secondPeak;
        while x > firstPeak {
            // for (int x = secondPeak - 1; x > firstPeak; x--) {
            let fromFirst = x - firstPeak;
            let score =
                fromFirst * fromFirst * (secondPeak - x) * (maxBucketCount - buckets[x]) as usize;
            if score as i32 > bestValleyScore {
                bestValley = x;
                bestValleyScore = score as i32;
            }
            x -= 1;
        }

        Ok((bestValley as u32) << GlobalHistogramBinarizer::LUMINANCE_SHIFT)
    }
}
