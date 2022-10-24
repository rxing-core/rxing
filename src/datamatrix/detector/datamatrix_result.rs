use crate::{common::{DetectorRXingResult, BitMatrix}, RXingResultPoint};

pub struct DatamatrixDetectorResult(BitMatrix,Vec<RXingResultPoint>);

impl DatamatrixDetectorResult {
    pub fn new(bits:BitMatrix, points:Vec<RXingResultPoint>)->Self {
        Self(bits,points)
    }
}

impl DetectorRXingResult for DatamatrixDetectorResult {
    fn getBits(&self) -> &BitMatrix {
        &self.0
    }

    fn getPoints(&self) -> &Vec<RXingResultPoint> {
        &self.1
    }
}