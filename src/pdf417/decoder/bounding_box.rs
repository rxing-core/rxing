/*
 * Copyright 2013 ZXing authors
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

use std::sync::Arc;

use crate::{
    common::{BitMatrix, Result},
    point_f, Exceptions, Point,
};

/**
 * @author Guenther Grau
 */
#[derive(Clone)]
pub struct BoundingBox {
    image: Arc<BitMatrix>,
    topLeft: Point,
    bottomLeft: Point,
    topRight: Point,
    bottomRight: Point,
    minX: u32,
    maxX: u32,
    minY: u32,
    maxY: u32,
}
impl BoundingBox {
    pub fn new(
        image: Arc<BitMatrix>,
        topLeft: Option<Point>,
        bottomLeft: Option<Point>,
        topRight: Option<Point>,
        bottomRight: Option<Point>,
    ) -> Result<BoundingBox> {
        let leftUnspecified = topLeft.is_none() || bottomLeft.is_none();
        let rightUnspecified = topRight.is_none() || bottomRight.is_none();
        if leftUnspecified && rightUnspecified {
            return Err(Exceptions::NOT_FOUND);
        }

        let newTopLeft;
        let newBottomLeft;
        let newTopRight;
        let newBottomRight;

        if leftUnspecified {
            newTopRight = topRight.ok_or(Exceptions::ILLEGAL_STATE)?;
            newBottomRight = bottomRight.ok_or(Exceptions::ILLEGAL_STATE)?;
            newTopLeft = point_f(0.0, newTopRight.y);
            newBottomLeft = point_f(0.0, newBottomRight.y);
        } else if rightUnspecified {
            newTopLeft = topLeft.ok_or(Exceptions::ILLEGAL_STATE)?;
            newBottomLeft = bottomLeft.ok_or(Exceptions::ILLEGAL_STATE)?;
            newTopRight = point_f(image.getWidth() as f32 - 1.0, newTopLeft.y);
            newBottomRight = point_f(image.getWidth() as f32 - 1.0, newBottomLeft.y);
        } else {
            newTopLeft = topLeft.ok_or(Exceptions::ILLEGAL_STATE)?;
            newTopRight = topRight.ok_or(Exceptions::ILLEGAL_STATE)?;
            newBottomLeft = bottomLeft.ok_or(Exceptions::ILLEGAL_STATE)?;
            newBottomRight = bottomRight.ok_or(Exceptions::ILLEGAL_STATE)?;
        }

        Ok(BoundingBox {
            image,
            minX: newTopLeft.x.min(newBottomLeft.x) as u32,
            maxX: newTopRight.x.max(newBottomRight.x) as u32,
            minY: newTopLeft.y.min(newTopRight.y) as u32,
            maxY: newBottomLeft.y.max(newBottomRight.y) as u32,
            topLeft: newTopLeft,
            bottomLeft: newBottomLeft,
            topRight: newTopRight,
            bottomRight: newBottomRight,
        })
    }

    pub fn from_other(boundingBox: Arc<BoundingBox>) -> BoundingBox {
        BoundingBox {
            image: boundingBox.image.clone(),
            topLeft: boundingBox.topLeft,
            bottomLeft: boundingBox.bottomLeft,
            topRight: boundingBox.topRight,
            bottomRight: boundingBox.bottomRight,
            minX: boundingBox.minX,
            maxX: boundingBox.maxX,
            minY: boundingBox.minY,
            maxY: boundingBox.maxY,
        }
    }

    pub fn merge(
        leftBox: Option<BoundingBox>,
        rightBox: Option<BoundingBox>,
    ) -> Result<BoundingBox> {
        if leftBox.is_none() {
            return Ok(rightBox.as_ref().ok_or(Exceptions::ILLEGAL_STATE)?.clone());
        }
        if rightBox.is_none() {
            return Ok(leftBox.as_ref().ok_or(Exceptions::ILLEGAL_STATE)?.clone());
        }
        let leftBox = leftBox.ok_or(Exceptions::ILLEGAL_STATE)?;
        let rightBox = rightBox.ok_or(Exceptions::ILLEGAL_STATE)?;

        BoundingBox::new(
            leftBox.image,
            Some(leftBox.topLeft),
            Some(leftBox.bottomLeft),
            Some(rightBox.topRight),
            Some(rightBox.bottomRight),
        )
    }

    pub fn addMissingRows(
        &self,
        missingStartRows: u32,
        missingEndRows: u32,
        isLeft: bool,
    ) -> Result<BoundingBox> {
        let mut newTopLeft = self.topLeft;
        let mut newBottomLeft = self.bottomLeft;
        let mut newTopRight = self.topRight;
        let mut newBottomRight = self.bottomRight;

        if missingStartRows > 0 {
            let top = if isLeft { self.topLeft } else { self.topRight };
            let mut newMinY = top.y - missingStartRows as f32;
            if newMinY < 0.0 {
                newMinY = 0.0;
            }
            let newTop = point_f(top.x, newMinY);
            if isLeft {
                newTopLeft = newTop;
            } else {
                newTopRight = newTop;
            }
        }

        if missingEndRows > 0 {
            let bottom = if isLeft {
                self.bottomLeft
            } else {
                self.bottomRight
            };
            let mut newMaxY = bottom.y as u32 + missingEndRows;
            if newMaxY >= self.image.getHeight() {
                newMaxY = self.image.getHeight() - 1;
            }
            let newBottom = point_f(bottom.x, newMaxY as f32);
            if isLeft {
                newBottomLeft = newBottom;
            } else {
                newBottomRight = newBottom;
            }
        }

        BoundingBox::new(
            self.image.clone(),
            Some(newTopLeft),
            Some(newBottomLeft),
            Some(newTopRight),
            Some(newBottomRight),
        )
    }

    pub fn getMinX(&self) -> u32 {
        self.minX
    }

    pub fn getMaxX(&self) -> u32 {
        self.maxX
    }

    pub fn getMinY(&self) -> u32 {
        self.minY
    }

    pub fn getMaxY(&self) -> u32 {
        self.maxY
    }

    pub fn getTopLeft(&self) -> &Point {
        &self.topLeft
    }

    pub fn getTopRight(&self) -> &Point {
        &self.topRight
    }

    pub fn getBottomLeft(&self) -> &Point {
        &self.bottomLeft
    }

    pub fn getBottomRight(&self) -> &Point {
        &self.bottomRight
    }
}
