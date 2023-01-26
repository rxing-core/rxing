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
    fn sum<I: Iterator<Item = &'a RXingResultPoint>>(iter: I) -> Self {
        let mut add = RXingResultPoint { x: 0.0, y: 0.0 };
        for n in iter {
            add += *n;
        }
        add
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

impl std::ops::Sub<RXingResultPoint> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<&RXingResultPoint> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Neg for RXingResultPoint {
    type Output = RXingResultPoint;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Add<RXingResultPoint> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn add(self, rhs: RXingResultPoint) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<RXingResultPoint> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn mul(self, rhs: RXingResultPoint) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<i32> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

impl std::ops::Mul<RXingResultPoint> for i32 {
    type Output = RXingResultPoint;

    fn mul(self, rhs: RXingResultPoint) -> Self::Output {
        RXingResultPoint {
            x: rhs.x * self as f32,
            y: rhs.y * self as f32,
        }
    }
}

impl std::ops::Mul<RXingResultPoint> for f32 {
    type Output = RXingResultPoint;

    fn mul(self, rhs: RXingResultPoint) -> Self::Output {
        RXingResultPoint {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl std::ops::Mul<&RXingResultPoint> for f32 {
    type Output = RXingResultPoint;

    fn mul(self, rhs: &RXingResultPoint) -> Self::Output {
        RXingResultPoint {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl std::ops::Mul<&mut RXingResultPoint> for f32 {
    type Output = RXingResultPoint;

    fn mul(self, rhs: &mut RXingResultPoint) -> Self::Output {
        RXingResultPoint {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl std::ops::Div<f32> for RXingResultPoint {
    type Output = RXingResultPoint;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl RXingResultPoint {
    pub fn dot(a: RXingResultPoint, b: RXingResultPoint) -> f32 {
        a.x * b.x + a.y * b.y
    }

    pub fn cross(a: &RXingResultPoint, b: &RXingResultPoint) -> f32 {
        a.x * b.y - b.x * a.y
    }

    /// L1 norm
    pub fn sumAbsComponent(p: &RXingResultPoint) -> f32 {
        (p.x).abs() + (p.y).abs()
    }

    /// L2 norm
    pub fn length(p: RXingResultPoint) -> f32 {
        (Self::dot(p, p)).sqrt()
    }

    /// L-inf norm
    pub fn maxAbsComponent(p: &RXingResultPoint) -> f32 {
        let a = (p.x).abs();
        let b = (p.y).abs();

        if a > b {
            a
        } else {
            b
        }

        // return std::cmp::max((p.x).abs(), (p.y).abs());
    }

    pub fn distance(a: RXingResultPoint, b: RXingResultPoint) -> f32 {
        Self::length(a - b)
    }

    /// Calculate a floating point pixel coordinate representing the 'center' of the pixel.
    /// This is sort of the inverse operation of the PointI(PointF) conversion constructor.
    /// See also the documentation of the GridSampler API.
    #[inline(always)]
    pub fn centered(p: &RXingResultPoint) -> RXingResultPoint {
        RXingResultPoint {
            x: (p.x).floor() + 0.5,
            y: (p.y).floor() + 0.5,
        }
    }

    pub fn normalized(d: RXingResultPoint) -> RXingResultPoint {
        d / Self::length(d)
    }

    pub fn bresenhamDirection(d: &RXingResultPoint) -> RXingResultPoint {
        *d / Self::maxAbsComponent(d)
    }

    pub fn mainDirection(d: RXingResultPoint) -> RXingResultPoint {
        if (d.x).abs() > (d.y).abs() {
            Self::new(d.x, 0.0)
        } else {
            Self::new(0.0, d.y)
        }
    }
}
