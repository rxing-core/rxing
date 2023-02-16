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

use crate::{point, Point, ResultPoint};

/**
 * <p>Encapsulates a finder pattern, which are the three square patterns found in
 * the corners of QR Codes. It also encapsulates a count of similar finder patterns,
 * as a convenience to the finder's bookkeeping.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FinderPattern {
    estimatedModuleSize: f32,
    count: usize,
    point: Point,
}

impl ResultPoint for FinderPattern {
    fn getX(&self) -> f32 {
        self.point.x
    }

    fn getY(&self) -> f32 {
        self.point.y
    }

    fn to_rxing_result_point(&self) -> Point {
        self.point
    }
}

impl From<&FinderPattern> for Point {
    fn from(value: &FinderPattern) -> Self {
        value.point
    }
}

impl From<FinderPattern> for Point {
    fn from(value: FinderPattern) -> Self {
        value.point
    }
}

impl FinderPattern {
    pub fn new(posX: f32, posY: f32, estimatedModuleSize: f32) -> Self {
        Self::private_new(posX, posY, estimatedModuleSize, 1)
    }

    fn private_new(posX: f32, posY: f32, estimatedModuleSize: f32, count: usize) -> Self {
        Self {
            estimatedModuleSize,
            count,
            point: point(posX, posY),
        }
    }

    pub fn getEstimatedModuleSize(&self) -> f32 {
        self.estimatedModuleSize
    }

    pub fn getCount(&self) -> usize {
        self.count
    }

    /**
     * <p>Determines if this finder pattern "about equals" a finder pattern at the stated
     * position and size -- meaning, it is at nearly the same center with nearly the same size.</p>
     */
    pub fn aboutEquals(&self, moduleSize: f32, i: f32, j: f32) -> bool {
        if (i - self.getY()).abs() <= moduleSize && (j - self.getX()).abs() <= moduleSize {
            let moduleSizeDiff = (moduleSize - self.estimatedModuleSize).abs();
            moduleSizeDiff <= 1.0 || moduleSizeDiff <= self.estimatedModuleSize
        } else {
            false
        }
    }

    /**
     * Combines this object's current estimate of a finder pattern position and module size
     * with a new estimate. It returns a new {@code FinderPattern} containing a weighted average
     * based on count.
     */
    pub fn combineEstimate(&self, i: f32, j: f32, newModuleSize: f32) -> FinderPattern {
        let combinedCount = self.count as f32 + 1.0;
        let combinedX = (self.count as f32 * self.getX() + j) / combinedCount;
        let combinedY = (self.count as f32 * self.getY() + i) / combinedCount;
        let combinedModuleSize =
            (self.count as f32 * self.estimatedModuleSize + newModuleSize) / combinedCount;
        FinderPattern::private_new(
            combinedX,
            combinedY,
            combinedModuleSize,
            combinedCount.round() as usize,
        )
    }
}
