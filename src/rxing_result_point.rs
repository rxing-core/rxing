use std::fmt;

use crate::ResultPoint;
use std::hash::Hash;

/**
 * <p>Encapsulates a point of interest in an image containing a barcode. Typically, this
 * would be the location of a finder pattern or the corner of the barcode, for example.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct RXingResultPoint {
    pub(crate) x: f32,
    pub(crate) y: f32,
}
impl Hash for RXingResultPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_string().hash(state);
        self.y.to_string().hash(state);
    }
}
impl PartialEq for RXingResultPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x.to_string() == other.x.to_string() && self.y.to_string() == other.y.to_string()
    }
}
impl Eq for RXingResultPoint {}
impl RXingResultPoint {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl ResultPoint for RXingResultPoint {
    fn getX(&self) -> f32 {
        self.x
    }

    fn getY(&self) -> f32 {
        self.y
    }

    fn into_rxing_result_point(self) -> RXingResultPoint {
        self
    }
}

impl fmt::Display for RXingResultPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
