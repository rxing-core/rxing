use std::{fmt, iter::Sum};

use crate::ResultPoint;
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/**
 * <p>Encapsulates a point of interest in an image containing a barcode. Typically, this
 * would be the location of a finder pattern or the corner of the barcode, for example.</p>
 *
 * @author Sean Owen
 */
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        self.x == other.x && self.y == other.y
    }
}
impl Eq for RXingResultPoint {}

impl RXingResultPoint {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn with_single(x: f32) -> Self {
        Self { x, y: x }
    }
}

impl std::ops::AddAssign for RXingResultPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<'a> Sum<&'a RXingResultPoint> for RXingResultPoint {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, &p| acc + p)
    }
}

impl ResultPoint for RXingResultPoint {
    fn getX(&self) -> f32 {
        self.x
    }

    fn getY(&self) -> f32 {
        self.y
    }

    fn into_rxing_result_point(self) -> Self {
        self
    }
}

impl fmt::Display for RXingResultPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Sub for RXingResultPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Neg for RXingResultPoint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl std::ops::Add for RXingResultPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Mul for RXingResultPoint {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Mul<f32> for RXingResultPoint {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Mul<i32> for RXingResultPoint {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs as f32, self.y * rhs as f32)
    }
}

impl std::ops::Mul<RXingResultPoint> for i32 {
    type Output = RXingResultPoint;

    fn mul(self, rhs: RXingResultPoint) -> Self::Output {
        Self::Output::new(rhs.x * self as f32, rhs.y * self as f32)
    }
}

impl std::ops::Mul<RXingResultPoint> for f32 {
    type Output = RXingResultPoint;

    fn mul(self, rhs: RXingResultPoint) -> Self::Output {
        Self::Output::new(rhs.x * self, rhs.y * self)
    }
}

impl std::ops::Div<f32> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs)
    }
}

impl RXingResultPoint {
    pub fn dot(self, p: Self) -> f32 {
        self.x * p.x + self.y * p.y
    }

    pub fn cross(self, p: Self) -> f32 {
        self.x * p.y - p.x * self.y
    }

    /// L1 norm
    pub fn sumAbsComponent(self) -> f32 {
        self.x.abs() + self.y.abs()
    }

    /// L2 norm
    pub fn length(self) -> f32 {
        Self::dot(self, self).sqrt()
    }

    /// L-inf norm
    pub fn maxAbsComponent(self) -> f32 {
        f32::max(self.x.abs(), self.y.abs())
    }

    pub fn distance(self, p: Self) -> f32 {
        Self::length(self - p)
    }

    /// Calculate a floating point pixel coordinate representing the 'center' of the pixel.
    /// This is sort of the inverse operation of the PointI(PointF) conversion constructor.
    /// See also the documentation of the GridSampler API.
    #[inline(always)]
    pub fn centered(self) -> Self {
        Self::new(self.x.floor() + 0.5, self.y.floor() + 0.5)
    }

    pub fn middle(self, p: Self) -> Self {
        (self + p) / 2.0
    }

    pub fn normalized(self) -> Self {
        self / Self::length(self)
    }

    pub fn bresenhamDirection(self) -> Self {
        self / Self::maxAbsComponent(self)
    }

    pub fn mainDirection(self) -> Self {
        if self.x.abs() > self.y.abs() {
            Self::new(self.x, 0.0)
        } else {
            Self::new(0.0, self.y)
        }
    }
}
