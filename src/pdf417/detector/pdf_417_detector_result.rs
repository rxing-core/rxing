/*
 * Copyright 2007 ZXing authors
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

use crate::{common::BitMatrix, Point};

/**
 * @author Guenther Grau
 */
pub struct PDF417DetectorRXingResult {
    bits: BitMatrix,
    points: Vec<[Option<Point>; 8]>,
    rotation: u32,
}

impl PDF417DetectorRXingResult {
    pub fn with_rotation(bits: BitMatrix, points: Vec<[Option<Point>; 8]>, rotation: u32) -> Self {
        Self {
            bits,
            points,
            rotation,
        }
    }

    pub fn new(bits: BitMatrix, points: Vec<[Option<Point>; 8]>) -> Self {
        Self::with_rotation(bits, points, 0)
    }

    pub fn getBits(&self) -> &BitMatrix {
        &self.bits
    }

    pub fn getPoints(&self) -> &Vec<[Option<Point>; 8]> {
        &self.points
    }

    pub fn getRotation(&self) -> u32 {
        self.rotation
    }
}
