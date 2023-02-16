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

use std::rc::Rc;

use crate::{
    common::{BitMatrix, Result},
    point, Exceptions, Point, ResultPoint,
};

/**
 * @author Guenther Grau
 */
#[derive(Clone)]
pub struct BoundingBox {
    image: Rc<BitMatrix>,
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
        image: Rc<BitMatrix>,
        topLeft: Option<Point>,
        bottomLeft: Option<Point>,
        topRight: Option<Point>,
        bottomRight: Option<Point>,
    ) -> Result<BoundingBox> {
        let leftUnspecified = topLeft.is_none() || bottomLeft.is_none();
        let rightUnspecified = topRight.is_none() || bottomRight.is_none();
        if leftUnspecified && rightUnspecified {
            return Err(Exceptions::NotFoundException(None));
        }

        let newTopLeft;
        let newBottomLeft;
        let newTopRight;
        let newBottomRight;

        if leftUnspecified {
            newTopRight = topRight.ok_or(Exceptions::IllegalStateException(None))?;
            newBottomRight = bottomRight.ok_or(Exceptions::IllegalStateException(None))?;
            newTopLeft = point(0.0, newTopRight.getY());
            newBottomLeft = point(0.0, newBottomRight.getY());
        } else if rightUnspecified {
            newTopLeft = topLeft.ok_or(Exceptions::IllegalStateException(None))?;
            newBottomLeft = bottomLeft.ok_or(Exceptions::IllegalStateException(None))?;
            newTopRight = point(image.getWidth() as f32 - 1.0, newTopLeft.getY());
            newBottomRight = point(image.getWidth() as f32 - 1.0, newBottomLeft.getY());
        } else {
            newTopLeft = topLeft.ok_or(Exceptions::IllegalStateException(None))?;
            newTopRight = topRight.ok_or(Exceptions::IllegalStateException(None))?;
            newBottomLeft = bottomLeft.ok_or(Exceptions::IllegalStateException(None))?;
            newBottomRight = bottomRight.ok_or(Exceptions::IllegalStateException(None))?;
        }

        Ok(BoundingBox {
            image,
            minX: newTopLeft.getX().min(newBottomLeft.getX()) as u32,
            maxX: newTopRight.getX().max(newBottomRight.getX()) as u32,
            minY: newTopLeft.getY().min(newTopRight.getY()) as u32,
            maxY: newBottomLeft.getY().max(newBottomRight.getY()) as u32,
            topLeft: newTopLeft,
            bottomLeft: newBottomLeft,
            topRight: newTopRight,
            bottomRight: newBottomRight,
        })
    }

    pub fn from_other(boundingBox: Rc<BoundingBox>) -> BoundingBox {
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
            return Ok(rightBox
                .as_ref()
                .ok_or(Exceptions::IllegalStateException(None))?
                .clone());
        }
        if rightBox.is_none() {
            return Ok(leftBox
                .as_ref()
                .ok_or(Exceptions::IllegalStateException(None))?
                .clone());
        }
        let leftBox = leftBox.ok_or(Exceptions::IllegalStateException(None))?;
        let rightBox = rightBox.ok_or(Exceptions::IllegalStateException(None))?;

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
            let mut newMinY = top.getY() - missingStartRows as f32;
            if newMinY < 0.0 {
                newMinY = 0.0;
            }
            let newTop = point(top.getX(), newMinY);
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
            let mut newMaxY = bottom.getY() as u32 + missingEndRows;
            if newMaxY >= self.image.getHeight() {
                newMaxY = self.image.getHeight() - 1;
            }
            let newBottom = point(bottom.getX(), newMaxY as f32);
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
