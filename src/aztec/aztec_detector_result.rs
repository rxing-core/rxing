/*
 * Copyright 2010 ZXing authors
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

// package com.google.zxing.aztec;

// import com.google.zxing.Point;
// import com.google.zxing.common.BitMatrix;
// import com.google.zxing.common.DetectorRXingResult;

use crate::{
    common::{BitMatrix, DetectorRXingResult},
    Point,
};

/**
 * <p>Extends {@link DetectorRXingResult} with more information specific to the Aztec format,
 * like the number of layers and whether it's compact.</p>
 *
 * @author Sean Owen
 */
pub struct AztecDetectorRXingResult {
    bits: BitMatrix,
    points: [Point; 4],
    compact: bool,
    nbDatablocks: u32,
    nbLayers: u32,
}

impl DetectorRXingResult for AztecDetectorRXingResult {
    fn getBits(&self) -> &BitMatrix {
        &self.bits
    }

    fn getPoints(&self) -> &[Point] {
        &self.points
    }
}

impl AztecDetectorRXingResult {
    pub fn new(
        bits: BitMatrix,
        points: [Point; 4],
        compact: bool,
        nbDatablocks: u32,
        nbLayers: u32,
    ) -> Self {
        Self {
            bits,
            points,
            compact,
            nbDatablocks,
            nbLayers,
        }
    }

    pub const fn getNbLayers(&self) -> u32 {
        self.nbLayers
    }

    pub fn getNbDatablocks(&self) -> u32 {
        self.nbDatablocks
    }

    pub const fn isCompact(&self) -> bool {
        self.compact
    }
}
