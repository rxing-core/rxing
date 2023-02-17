use crate::{
    common::{BitMatrix, DetectorRXingResult},
    Point,
};

pub struct QRCodeDetectorResult {
    bit_source: BitMatrix,
    result_points: Vec<Point>,
}

impl QRCodeDetectorResult {
    pub fn new(bit_source: BitMatrix, result_points: Vec<Point>) -> Self {
        Self {
            bit_source,
            result_points,
        }
    }
}

impl DetectorRXingResult for QRCodeDetectorResult {
    fn getBits(&self) -> &crate::common::BitMatrix {
        &self.bit_source
    }

    fn getPoints(&self) -> &[crate::Point] {
        &self.result_points
    }
}
