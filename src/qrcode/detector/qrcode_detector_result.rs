use crate::{
    common::{BitMatrix, DetectorRXingResult},
    RXingResultPoint, ResultPoint,
};

pub struct QRCodeDetectorResult {
    bit_source: BitMatrix,
    result_points: Vec<RXingResultPoint>,
}

impl QRCodeDetectorResult {
    pub fn new(bit_source: BitMatrix, result_points: Vec<RXingResultPoint>) -> Self {
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

    fn getPoints(&self) -> &Vec<crate::RXingResultPoint> {
        &self.result_points
    }
}
