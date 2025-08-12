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

use once_cell::sync::OnceCell;

use crate::common::Result;
use crate::{Binarizer, LuminanceSource};

use super::{BitArray, BitMatrix, GlobalHistogramBinarizer};

/**
 * This class implements a local thresholding algorithm, which while slower than the
 * GlobalHistogramBinarizer, is fairly efficient for what it does. It is designed for
 * high frequency images of barcodes with black data on white backgrounds. For this application,
 * it does a much better job than a global blackpoint with severe shadows and gradients.
 * However it tends to produce artifacts on lower frequency images and is therefore not
 * a good general purpose binarizer for uses outside ZXing.
 *
 * This class extends GlobalHistogramBinarizer, using the older histogram approach for 1D readers,
 * and the newer local approach for 2D readers. 1D decoding using a per-row histogram is already
 * inherently local, and only fails for horizontal gradients. We can revisit that problem later,
 * but for now it was not a win to use local blocks for 1D.
 *
 * This Binarizer is the default for the unit tests and the recommended class for library users.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct HybridBinarizer<LS: LuminanceSource> {
    //width: usize,
    //height: usize,
    //source: Box<dyn LuminanceSource>,
    ghb: GlobalHistogramBinarizer<LS>,
    black_matrix: OnceCell<BitMatrix>,
}
impl<LS: LuminanceSource> Binarizer for HybridBinarizer<LS> {
    type Source = LS;

    fn get_luminance_source(&self) -> &LS {
        self.ghb.get_luminance_source()
    }

    fn get_black_row(&self, y: usize) -> Result<Cow<'_, BitArray>> {
        self.ghb.get_black_row(y)
    }

    fn get_black_line(&self, l: usize, lt: super::LineOrientation) -> Result<Cow<'_, BitArray>> {
        self.ghb.get_black_line(l, lt)
    }

    /**
     * Calculates the final BitMatrix once for all requests. This could be called once from the
     * constructor instead, but there are some advantages to doing it lazily, such as making
     * profiling easier, and not doing heavy lifting when callers don't expect it.
     */
    fn get_black_matrix(&self) -> Result<&BitMatrix> {
        let matrix = self
            .black_matrix
            .get_or_try_init(|| Self::calculateBlackMatrix(&self.ghb))?;
        Ok(matrix)
    }

    fn create_binarizer(&self, source: LS) -> Self {
        Self::new(source)
    }

    fn get_width(&self) -> usize {
        self.ghb.get_width()
    }

    fn get_height(&self) -> usize {
        self.ghb.get_height()
    }

    fn get_black_row_from_matrix(&self, y: usize) -> Result<Cow<'_, BitArray>> {
        if let Some(matrix) = self.black_matrix.get() {
            Ok(Cow::Owned(matrix.getRow(y as u32)))
        } else {
            self.get_black_row(y)
        }
    }
}

// This class uses 5x5 blocks to compute local luminance, where each block is 8x8 pixels.
// So this is the smallest dimension in each axis we can accept.
const BLOCK_SIZE_POWER: usize = 3;
const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_POWER; // ...0100...00
const BLOCK_SIZE_MASK: usize = BLOCK_SIZE - 1; // ...0011...11
const MINIMUM_DIMENSION: usize = BLOCK_SIZE * 5;
const MIN_DYNAMIC_RANGE: usize = 24;

impl<LS: LuminanceSource> HybridBinarizer<LS> {
    pub fn new(source: LS) -> Self {
        let ghb = GlobalHistogramBinarizer::new(source);
        Self {
            black_matrix: OnceCell::new(),
            ghb,
        }
    }

    fn calculateBlackMatrix<LS2: LuminanceSource>(
        ghb: &GlobalHistogramBinarizer<LS2>,
    ) -> Result<BitMatrix> {
        // let matrix;
        let source = ghb.get_luminance_source();
        let width = source.get_width();
        let height = source.get_height();

        //  dbg!(matrix.to_string());
        if width >= MINIMUM_DIMENSION && height >= MINIMUM_DIMENSION {
            let luminances = source.get_matrix();
            let mut sub_width = width >> BLOCK_SIZE_POWER;
            if (width & BLOCK_SIZE_MASK) != 0 {
                sub_width += 1;
            }
            let mut sub_height = height >> BLOCK_SIZE_POWER;
            if (height & BLOCK_SIZE_MASK) != 0 {
                sub_height += 1;
            }
            let black_points = Self::calculateBlackPoints(
                &luminances,
                sub_width as u32,
                sub_height as u32,
                width as u32,
                height as u32,
            );

            let mut new_matrix = BitMatrix::new(width as u32, height as u32)?;
            Self::calculateThresholdForBlock(
                &luminances,
                sub_width as u32,
                sub_height as u32,
                width as u32,
                height as u32,
                &black_points,
                &mut new_matrix,
            );
            Ok(new_matrix)
        } else {
            // If the image is too small, fall back to the global histogram approach.
            let m = ghb.get_black_matrix()?;
            Ok(m.clone())
        }
    }

    /**
     * For each block in the image, calculate the average black point using a 5x5 grid
     * of the blocks around it. Also handles the corner cases (fractional blocks are computed based
     * on the last pixels in the row/column which are also used in the previous block).
     */
    fn calculateThresholdForBlock(
        luminances: &[u8],
        sub_width: u32,
        sub_height: u32,
        width: u32,
        height: u32,
        black_points: &[Vec<u32>],
        matrix: &mut BitMatrix,
    ) {
        let maxYOffset = height - BLOCK_SIZE as u32;
        let maxXOffset = width - BLOCK_SIZE as u32;
        for y in 0..sub_height {
            // for (int y = 0; y < subHeight; y++) {
            let yoffset = u32::min(y << BLOCK_SIZE_POWER, maxYOffset);

            let top = u32::clamp(y, 2, sub_height - 3); // Self::cap(y, sub_height - 3);
            for x in 0..sub_width {
                //   for (int x = 0; x < subWidth; x++) {
                let xoffset = u32::min(x << BLOCK_SIZE_POWER, maxXOffset);

                let left = u32::clamp(x, 2, sub_width - 3); //Self::cap(x, sub_width - 3);
                let mut sum = 0;
                for z in -2..=2 {
                    // for (int z = -2; z <= 2; z++) {
                    let blackRow = &black_points[(top as i32 + z) as usize];
                    sum += blackRow[(left - 2) as usize]
                        + blackRow[(left - 1) as usize]
                        + blackRow[left as usize]
                        + blackRow[(left + 1) as usize]
                        + blackRow[(left + 2) as usize];
                }
                let average = sum / 25;
                Self::thresholdBlock(luminances, xoffset, yoffset, average, width, matrix);
            }
        }
    }

    /**
     * Applies a single threshold to a block of pixels.
     */
    fn thresholdBlock(
        luminances: &[u8],
        xoffset: u32,
        yoffset: u32,
        threshold: u32,
        stride: u32,
        matrix: &mut BitMatrix,
    ) {
        let mut offset = yoffset * stride + xoffset;
        for y in 0..BLOCK_SIZE {
            // for (int y = 0, offset = yoffset * stride + xoffset; y < HybridBinarizer::BLOCK_SIZE; y++, offset += stride) {
            for x in 0..BLOCK_SIZE {
                //   for (int x = 0; x < HybridBinarizer::BLOCK_SIZE; x++) {
                // Comparison needs to be <= so that black == 0 pixels are black even if the threshold is 0.
                if luminances[offset as usize + x] as u32 <= threshold {
                    matrix.set(xoffset + x as u32, yoffset + y as u32);
                }
            }
            offset += stride;
        }
    }

    /**
     * Calculates a single black point for each block of pixels and saves it away.
     * See the following thread for a discussion of this algorithm:
     *  http://groups.google.com/group/zxing/browse_thread/thread/d06efa2c35a7ddc0
     */
    fn calculateBlackPoints(
        luminances: &[u8],
        subWidth: u32,
        subHeight: u32,
        width: u32,
        height: u32,
    ) -> Vec<Vec<u32>> {
        let maxYOffset = height as usize - BLOCK_SIZE;
        let maxXOffset = width as usize - BLOCK_SIZE;
        let mut blackPoints = vec![vec![0; subWidth as usize]; subHeight as usize];
        for y in 0..subHeight {
            // for (int y = 0; y < subHeight; y++) {
            let yoffset = u32::min(y << BLOCK_SIZE_POWER, maxYOffset as u32);

            for x in 0..subWidth {
                //   for (int x = 0; x < subWidth; x++) {
                let xoffset = u32::min(x << BLOCK_SIZE_POWER, maxXOffset as u32);

                let mut sum: u32 = 0;
                let mut min = u8::MAX;
                let mut max = u8::MIN;

                let mut offset = yoffset * width + xoffset;
                let mut yy = 0;
                while yy < BLOCK_SIZE {
                    for &pixel in &luminances[offset as usize..offset as usize + BLOCK_SIZE] {
                        sum += pixel as u32;
                        // still looking for good contrast
                        min = min.min(pixel);
                        max = max.max(pixel);
                    }
                    // short-circuit min/max tests once dynamic range is met
                    if (max - min) as usize > MIN_DYNAMIC_RANGE {
                        // finish the rest of the rows quickly
                        offset += width;
                        yy += 1;
                        while yy < BLOCK_SIZE {
                            sum += luminances[offset as usize..offset as usize + BLOCK_SIZE]
                                .iter()
                                .map(|&b| b as u32)
                                .sum::<u32>();
                            yy += 1;
                            offset += width;
                        }
                        break;
                    }
                    yy += 1;
                    offset += width;
                }

                // The default estimate is the average of the values in the block.
                let mut average = sum >> (BLOCK_SIZE_POWER * 2);
                if (max - min) as usize <= MIN_DYNAMIC_RANGE {
                    // If variation within the block is low, assume this is a block with only light or only
                    // dark pixels. In that case we do not want to use the average, as it would divide this
                    // low contrast area into black and white pixels, essentially creating data out of noise.
                    //
                    // The default assumption is that the block is light/background. Since no estimate for
                    // the level of dark pixels exists locally, use half the min for the block.
                    average = min as u32 / 2;

                    if y > 0 && x > 0 {
                        // Correct the "white background" assumption for blocks that have neighbors by comparing
                        // the pixels in this block to the previously calculated black points. This is based on
                        // the fact that dark barcode symbology is always surrounded by some amount of light
                        // background for which reasonable black point estimates were made. The bp estimated at
                        // the boundaries is used for the interior.

                        // The (min < bp) is arbitrary but works better than other heuristics that were tried.
                        let average_neighbor_black_point: u32 = (blackPoints[y as usize - 1]
                            [x as usize]
                            + (2 * blackPoints[y as usize][x as usize - 1])
                            + blackPoints[y as usize - 1][x as usize - 1])
                            / 4;
                        if (min as u32) < average_neighbor_black_point {
                            average = average_neighbor_black_point;
                        }
                    }
                }
                blackPoints[y as usize][x as usize] = average;
            }
        }
        blackPoints
    }
}
