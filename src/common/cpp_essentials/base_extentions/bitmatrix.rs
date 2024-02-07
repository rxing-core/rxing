
use crate::common::BitMatrix;
use crate::common::Result;
use crate::point_f;
use crate::Point;

impl BitMatrix {
    pub fn Deflate(
        &self,
        width: u32,
        height: u32,
        top: f32,
        left: f32,
        subSampling: f32,
    ) -> Result<Self> {
        let mut result = BitMatrix::new(width, height)?;

        for y in 0..result.height() {
            // for (int y = 0; y < result.height(); y++) {
            let yOffset = top + y as f32 * subSampling;
            for x in 0..result.width() {
                // for (int x = 0; x < result.width(); x++) {
                if self.get_point(point_f(left + x as f32 * subSampling, yOffset)) {
                    result.set(x, y);
                }
            }
        }

        Ok(result)
    }

    pub fn getTopLeftOnBitWithPosition(&self, left: &mut u32, top: &mut u32) -> bool {
        let Some(Point { x, y }) = self.getTopLeftOnBit() else {
            return false;
        };
        *left = x as u32;
        *top = y as u32;

        true
    }

    pub fn getBottomRightOnBitWithPosition(&self, right: &mut u32, bottom: &mut u32) -> bool {
        let Some(Point { x, y }) = self.getBottomRightOnBit() else {
            return false;
        };
        *right = x as u32;
        *bottom = y as u32;

        true
    }

    pub fn findBoundingBox(
        &self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
        minSize: u32,
    ) -> (bool, u32, u32, u32, u32) {
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;

        let mut right = 0;
        let mut bottom = 0;
        if !self.getTopLeftOnBitWithPosition(&mut left, &mut top)
            || !self.getBottomRightOnBitWithPosition(&mut right, &mut bottom)
            || bottom - top + 1 < minSize
        {
            return (false, left, top, width, height);
        }

        for y in top..=bottom {
            // for (int y = top; y <= bottom; y++ ) {
            for x in 0..left {
                // for (int x = 0; x < left; ++x){
                if self.get(x, y) {
                    left = x;
                    break;
                }
            }
            for x in (right..(self.width() - 1)).rev() {
                // for (int x = _width-1; x > right; x--){
                if self.get(x, y) {
                    right = x;
                    break;
                }
            }
        }

        width = right - left + 1;
        height = bottom - top + 1;

        (
            width >= minSize && height >= minSize,
            left,
            top,
            width,
            height,
        )
    }
}
