use std::{fmt, iter::Sum};

use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ResultPoint;

/**
 * <p>Encapsulates a point of interest in an image containing a barcode. Typically, this
 * would be the location of a finder pattern or the corner of the barcode, for example.</p>
 *
 * @author Sean Owen
 */
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

/** An alias for `Point::new`. */
pub fn point(x: f32, y: f32) -> Point {
    Point::new(x, y)
}

/** Currently necessary because the external OneDReader proc macro uses it. */
pub type RXingResultPoint = Point;

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_string().hash(state);
        self.y.to_string().hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn with_single(x: f32) -> Self {
        Self { x, y: x }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<'a> Sum<&'a Point> for Point {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, &p| acc + p)
    }
}

/** This impl is temporary and is there to ease refactoring. */
impl ResultPoint for Point {
    fn getX(&self) -> f32 {
        self.x
    }

    fn getY(&self) -> f32 {
        self.y
    }

    fn to_rxing_result_point(&self) -> Self {
        *self
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs as f32, self.y * rhs as f32)
    }
}

impl std::ops::Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output::new(rhs.x * self as f32, rhs.y * self as f32)
    }
}

impl std::ops::Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output::new(rhs.x * self, rhs.y * self)
    }
}

impl std::ops::Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs)
    }
}

impl Point {
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
        self.x.hypot(self.y)
    }

    /// L-inf norm
    pub fn maxAbsComponent(self) -> f32 {
        f32::max(self.x.abs(), self.y.abs())
    }

    pub fn squaredDistance(self, p: Self) -> f32 {
        let diff = self - p;
        diff.x * diff.x + diff.y * diff.y
    }

    pub fn distance(self, p: Self) -> f32 {
        (self - p).length()
    }

    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn fold<U, F: Fn(f32, f32) -> U>(self, f: F) -> U {
        f(self.x, self.y)
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

impl From<&(f32, f32)> for Point {
    fn from(&(x, y): &(f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn testDistance() {
        assert_eq!(
            (8.0f32).sqrt(),
            Point::new(1.0, 2.0).distance(Point::new(3.0, 4.0))
        );
        assert_eq!(0.0, Point::new(1.0, 2.0).distance(Point::new(1.0, 2.0)));

        assert_eq!(
            (8.0f32).sqrt(),
            Point::new(1.0, 2.0).distance(Point::new(3.0, 4.0))
        );
        assert_eq!(0.0, Point::new(1.0, 2.0).distance(Point::new(1.0, 2.0)));
    }
}
