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

//Point

use crate::{Point, ResultPoint};

/**
 * <p>Encapsulates an alignment pattern, which are the smaller square patterns found in
 * all but the simplest QR Codes.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlignmentPattern {
    estimatedModuleSize: f32,
    point: (f32, f32),
}

impl ResultPoint for AlignmentPattern {
    fn getX(&self) -> f32 {
        self.point.0
    }

    fn getY(&self) -> f32 {
        self.point.1
    }

    fn to_rxing_result_point(&self) -> Point {
        Point {
            x: self.point.0,
            y: self.point.1,
        }
    }
}

impl AlignmentPattern {
    pub fn new(posX: f32, posY: f32, estimatedModuleSize: f32) -> Self {
        Self {
            estimatedModuleSize,
            point: (posX, posY),
        }
    }

    /**
     * <p>Determines if this alignment pattern "about equals" an alignment pattern at the stated
     * position and size -- meaning, it is at nearly the same center with nearly the same size.</p>
     */
    pub fn aboutEquals(&self, moduleSize: f32, i: f32, j: f32) -> bool {
        if (i - self.getY()).abs() <= moduleSize && (j - self.getX()).abs() <= moduleSize {
            let moduleSizeDiff = (moduleSize - self.estimatedModuleSize).abs();
            return moduleSizeDiff <= 1.0 || moduleSizeDiff <= self.estimatedModuleSize;
        }
        false
    }

    /**
     * Combines this object's current estimate of a finder pattern position and module size
     * with a new estimate. It returns a new {@code FinderPattern} containing an average of the two.
     */
    pub fn combineEstimate(&self, i: f32, j: f32, newModuleSize: f32) -> AlignmentPattern {
        let combinedX = (self.getX() + j) / 2.0;
        let combinedY = (self.getY() + i) / 2.0;
        let combinedModuleSize = (self.estimatedModuleSize + newModuleSize) / 2.0;
        AlignmentPattern::new(combinedX, combinedY, combinedModuleSize)
    }
}
