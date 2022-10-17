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

use crate::{Binarizer, Exceptions, LuminanceSource};

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
    luminances: Vec<u8>,
    buckets: Vec<u32>,
    width: usize,
    height: usize,
    source: Box<dyn LuminanceSource>,
}

impl Binarizer for GlobalHistogramBinarizer {
    fn getLuminanceSource(&self) -> &Box<dyn LuminanceSource> {
        &self.source
    }

    // Applies simple sharpening to the row data to improve performance of the 1D Readers.
    fn getBlackRow(&self, y: usize, row: &mut BitArray) -> Result<BitArray, Exceptions> {
        let source = self.getLuminanceSource();
        let width = source.getWidth();
        let mut row = if row.getSize() < width {
            BitArray::with_size(width)
        } else {
            let mut z = row.clone();
            z.clear();
            z
        };

        // self.initArrays(width);
        let localLuminances = source.getRow(y, &self.luminances);
        let mut localBuckets = self.buckets.clone();
        for x in 0..width {
            // for (int x = 0; x < width; x++) {
            localBuckets
                [((localLuminances[x]) >> GlobalHistogramBinarizer::LUMINANCE_SHIFT) as usize] += 1;
        }
        let blackPoint = self.estimateBlackPoint(&localBuckets)?;

        if width < 3 {
            // Special case for very small images
            for x in 0..width {
                //   for (int x = 0; x < width; x++) {
                if (localLuminances[x] as u32) < blackPoint {
                    row.set(x);
                }
            }
        } else {
            let mut left = localLuminances[0]; // & 0xff;
            let mut center = localLuminances[1]; // & 0xff;
            for x in 1..width - 1 {
                //   for (int x = 1; x < width - 1; x++) {
                let right = localLuminances[x + 1] & 0xff;
                // A simple -1 4 -1 box filter with a weight of 2.
                if ((center * 4) - left - right) as u32 / 2 < blackPoint {
                    row.set(x);
                }
                left = center;
                center = right;
            }
        }
        Ok(row)
    }

    // Does not sharpen the data, as this call is intended to only be used by 2D Readers.
    fn getBlackMatrix(&self) -> Result<BitMatrix, Exceptions> {
        let source = self.getLuminanceSource();
        let width = source.getWidth();
        let height = source.getHeight();
        let mut matrix = BitMatrix::new(width as u32, height as u32)?;

        // Quickly calculates the histogram by sampling four rows from the image. This proved to be
        // more robust on the blackbox tests than sampling a diagonal as we used to do.
        // self.initArrays(width);
        let mut localBuckets = self.buckets.clone();
        for y in 1..5 {
            // for (int y = 1; y < 5; y++) {
            let row = height * y / 5;
            let localLuminances = source.getRow(row, &self.luminances);
            let right = (width * 4) / 5;
            let mut x = width / 5;
            while x < right {
                //   for (int x = width / 5; x < right; x++) {
                let pixel = localLuminances[x];
                localBuckets[(pixel >> GlobalHistogramBinarizer::LUMINANCE_SHIFT) as usize] += 1;
                x += 1;
            }
        }
        let blackPoint = self.estimateBlackPoint(&localBuckets)?;

        // We delay reading the entire image luminance until the black point estimation succeeds.
        // Although we end up reading four rows twice, it is consistent with our motto of
        // "fail quickly" which is necessary for continuous scanning.
        let localLuminances = source.getMatrix();
        for y in 0..height {
            // for (int y = 0; y < height; y++) {
            let offset = y * width;
            for x in 0..width {
                //   for (int x = 0; x < width; x++) {
                let pixel = localLuminances[offset + x] & 0xff;
                if (pixel as u32) < blackPoint {
                    matrix.set(x as u32, y as u32);
                }
            }
        }

        Ok(matrix)
    }

    fn createBinarizer(&self, source: Box<dyn crate::LuminanceSource>) -> Box<dyn Binarizer> {
        return Box::new(GlobalHistogramBinarizer::new(source));
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
            luminances: vec![0; source.getWidth()],
            buckets: vec![0; GlobalHistogramBinarizer::LUMINANCE_BUCKETS],
            width: source.getWidth(),
            height: source.getHeight(),
            source: source,
        }
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

    fn estimateBlackPoint(&self, buckets: &[u32]) -> Result<u32, Exceptions> {
        // Find the tallest peak in the histogram.
        let numBuckets = buckets.len();
        let mut maxBucketCount = 0;
        let mut firstPeak = 0;
        let mut firstPeakSize = 0;
        for x in 0..numBuckets {
            // for (int x = 0; x < numBuckets; x++) {
            if buckets[x] > firstPeakSize {
                firstPeak = x;
                firstPeakSize = buckets[x];
            }
            if buckets[x] > maxBucketCount {
                maxBucketCount = buckets[x];
            }
        }

        // Find the second-tallest peak which is somewhat far from the tallest peak.
        let mut secondPeak = 0;
        let mut secondPeakScore = 0;
        for x in 0..numBuckets {
            // for (int x = 0; x < numBuckets; x++) {
            let distanceToBiggest = x - firstPeak;
            // Encourage more distant second peaks by multiplying by square of distance.
            let score = buckets[x] * distanceToBiggest as u32 * distanceToBiggest as u32;
            if score > secondPeakScore {
                secondPeak = x;
                secondPeakScore = score;
            }
        }

        // Make sure firstPeak corresponds to the black peak.
        if firstPeak > secondPeak {
            let temp = firstPeak;
            firstPeak = secondPeak;
            secondPeak = temp;
        }

        // If there is too little contrast in the image to pick a meaningful black point, throw rather
        // than waste time trying to decode the image, and risk false positives.
        if secondPeak - firstPeak <= numBuckets / 16 {
            return Err(Exceptions::NotFoundException(
                "secondPeak - firstPeak <= numBuckets / 16 ".to_owned(),
            ));
        }

        // Find a valley between them that is low and closer to the white peak.
        let mut bestValley = secondPeak - 1;
        let mut bestValleyScore = -1i32;
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
