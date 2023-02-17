use crate::{
    common::{BitMatrix, DetectorRXingResult},
    Point,
};

pub struct DatamatrixDetectorResult(BitMatrix, Vec<Point>);

impl DatamatrixDetectorResult {
    pub fn new(bits: BitMatrix, points: Vec<Point>) -> Self {
        Self(bits, points)
    }
}

impl DetectorRXingResult for DatamatrixDetectorResult {
    fn getBits(&self) -> &BitMatrix {
        &self.0
    }

    fn getPoints(&self) -> &[Point] {
        &self.1
    }
}
