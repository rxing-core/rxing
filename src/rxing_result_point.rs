
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
pub struct PointT<T> {
    pub x: T,
    pub y: T,
}
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[derive(Debug, Clone, Copy, Default)]
// pub struct Point {
//     pub(crate) x: f32,
//     pub(crate) y: f32,
// }

pub type PointF = PointT<f32>;
pub type PointI = PointT<u32>;
pub type Point = PointF;

impl Into<PointI> for Point {
    fn into(self) -> PointI {
        PointI {
            x: self.x.floor() as u32,
            y: self.y.floor() as u32,
        }
    }
}

impl Into<Point> for PointI {
    fn into(self) -> Point {
        Point {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

/** An alias for `Point::new`. */
pub fn point(x: f32, y: f32) -> Point {
    Point::new(x, y)
}

pub fn point_g<T: TryInto<f32>>(x: T, y: T) -> Option<Point> {
    Some(Point::new(x.try_into().ok()?, y.try_into().ok()?))
}

pub fn point_i<T: Into<i64>>(x: T, y: T) -> Point {
    Point::new(x.into() as f32, y.into() as f32)
}

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

impl<T> PointT<T>
where
    T: Copy,
{
    pub const fn new(x: T, y: T) -> PointT<T> {
        PointT { x: x, y: y }
    }
    pub const fn with_single(x: T) -> Self {
        Self { x, y: x }
    }
}

impl<T> std::ops::AddAssign for PointT<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> std::ops::SubAssign for PointT<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<'a, T> Sum<&'a PointT<T>> for PointT<T>
where
    T: std::ops::Add<Output = T> + 'a + Default,
    PointT<T>: std::ops::Add<Output = PointT<T>> + Copy,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, &p| acc + p)
    }
}

/** This impl is temporary and is there to ease refactoring. */
impl<T> ResultPoint for PointT<T>
where
    T: Into<f32> + Copy,
{
    fn getX(&self) -> f32 {
        self.x.into()
    }

    fn getY(&self) -> f32 {
        self.y.into()
    }

    fn to_rxing_result_point(&self) -> PointT<f32> {
        PointT {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<T> fmt::Display for PointT<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T> std::ops::Sub for PointT<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> std::ops::Neg for PointT<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T> std::ops::Add for PointT<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> std::ops::Add<f32> for PointT<T>
where
    T: Into<f32> + std::ops::Add<f32, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl<T> std::ops::Add<PointT<T>> for f32
where
    T: std::ops::Add<f32, Output = f32>,
{
    type Output = Point;

    fn add(self, rhs: PointT<T>) -> Self::Output {
        Point::new(rhs.x + self, rhs.y + self)
    }
}

impl<T> std::ops::Mul for PointT<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
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

impl std::ops::Mul<u32> for Point {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
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

impl std::ops::Mul<Point> for u32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output::new(rhs.x * self as f32, rhs.y * self as f32)
    }
}

impl<T> PointT<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + num::traits::real::Real,
    PointT<T>: std::ops::Div<T, Output = PointT<T>>,
{
    pub fn dot(self, p: Self) -> T {
        self.x * p.x + self.y * p.y
    }

    pub fn cross(self, p: Self) -> T {
        self.x * p.y - p.x * self.y
    }

    /// L1 norm
    pub fn sumAbsComponent(self) -> T {
        self.x.abs() + self.y.abs()
    }

    /// L2 norm
    pub fn length(self) -> T {
        self.x.hypot(self.y)
    }

    /// L-inf norm
    pub fn maxAbsComponent(self) -> T {
        self.x.abs().max(self.y.abs())
        // f32::max(self.x.abs(), self.y.abs())
    }

    pub fn squaredDistance(self, p: Self) -> T {
        let diff = self - p;
        diff.x * diff.x + diff.y * diff.y
    }

    pub fn distance(self, p: Self) -> T {
        (self - p).length()
    }

    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn fold<U, F: Fn(T, T) -> U>(self, f: F) -> U {
        f(self.x, self.y)
    }

    pub fn middle(self, p: Self) -> Self
    where
        T: From<u8>,
    {
        (self + p) / 2.into()
    }

    pub fn normalized(self) -> Self {
        self / Self::length(self)
    }

    pub fn bresenhamDirection(self) -> Self {
        self / Self::maxAbsComponent(self)
    }

    pub fn mainDirection(self) -> Self
    where
        T: From<u8>,
    {
        if self.x.abs() > self.y.abs() {
            Self::new(self.x, 0.into())
        } else {
            Self::new(0.into(), self.y)
        }
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Calculate a floating point pixel coordinate representing the 'center' of the pixel.
    /// This is sort of the inverse operation of the PointI(PointF) conversion constructor.
    /// See also the documentation of the GridSampler API.
    #[inline(always)]
    pub fn centered(self) -> PointT<f32>
    where
        T: Into<f32>,
    {
        PointT::new(self.x.floor().into() + 0.5, self.y.floor().into() + 0.5)
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x as f32, y as f32)
    }
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Self {
        Self::new(x as f32, y as f32)
    }
}

impl From<(f32, f32)> for PointI {
    fn from((x, y): (f32, f32)) -> Self {
        PointI {
            x: x.floor() as u32,
            y: y.floor() as u32,
        }
    }
}

impl<T> From<(T, T)> for PointT<T> {
    fn from((x, y): (T, T)) -> PointT<T> {
        PointT { x, y }
    }
}

impl<T> From<&(T, T)> for PointT<T>
where
    T: Copy,
{
    fn from(&(x, y): &(T, T)) -> PointT<T> {
        PointT { x, y }
    }
}

impl<T> From<T> for PointT<T>
where
    T: Copy,
{
    fn from(value: T) -> Self {
        Self::with_single(value)
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
