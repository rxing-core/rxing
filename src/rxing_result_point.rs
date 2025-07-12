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
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, PartialOrd)]
pub struct PointT<T> {
    pub x: T,
    pub y: T,
}

pub type PointF = PointT<f32>;
pub type PointI = PointT<i32>;
pub type PointU = PointT<u32>;
pub type Point = PointF;

impl From<Point> for PointI {
    fn from(val: Point) -> Self {
        PointI {
            x: val.x.floor() as i32,
            y: val.y.floor() as i32,
        }
    }
}

impl From<PointI> for Point {
    fn from(val: PointI) -> Self {
        Point {
            x: val.x as f32,
            y: val.y as f32,
        }
    }
}

impl From<Point> for PointU {
    fn from(val: Point) -> Self {
        PointU {
            x: val.x.floor() as u32,
            y: val.y.floor() as u32,
        }
    }
}

impl From<PointU> for Point {
    fn from(val: PointU) -> Self {
        Point {
            x: val.x as f32,
            y: val.y as f32,
        }
    }
}

/** An alias for `Point::new`. */
#[inline]
pub const fn point<T>(x: T, y: T) -> PointT<T>
where
    T: Copy,
{
    PointT::new(x, y)
}
#[inline]
pub fn point_i<T: Into<i64>>(x: T, y: T) -> Point {
    Point::new(x.into() as f32, y.into() as f32)
}

impl<T> Eq for PointT<T> where T: PartialEq {}

impl<T> PointT<T>
where
    T: Copy,
{
    pub const fn new(x: T, y: T) -> PointT<T> {
        PointT { x, y }
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
    fn get_x(&self) -> f32 {
        self.x.into()
    }

    fn get_y(&self) -> f32 {
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

    /**
     * Returns the z component of the cross product between vectors BC and BA.
     */
    pub fn crossProductZ(a: PointT<T>, b: PointT<T>, c: PointT<T>) -> T {
        ((c.x - b.x) * (a.y - b.y)) - ((c.y - b.y) * (a.x - b.x))
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
            x: x.floor() as i32,
            y: y.floor() as i32,
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

#[cfg(test)]
mod point_tests {
    use super::{point, point_i, Point, PointF, PointI, PointT, PointU, ResultPoint};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // --- Constructors, Default, new, with_single, aliases -----------------

    #[test]
    fn test_new_and_with_single() {
        let p: PointT<i32> = PointT::new(3, -4);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, -4);

        let q: PointT<i32> = PointT::with_single(7);
        assert_eq!(q, PointT { x: 7, y: 7 });

        let r = point(1.1f32, 2.2f32);
        assert_eq!(r, PointF { x: 1.1, y: 2.2 });

        let s = point_i(5, -6);
        assert_eq!(s, PointF::new(5.0, -6.0));
    }

    #[test]
    fn test_default() {
        let p: PointI = PointI::default();
        assert_eq!(p, PointI { x: 0, y: 0 });
    }

    // --- From conversions --------------------------------------------------

    #[test]
    fn test_from_point_to_int_and_uint() {
        let pf = PointF::new(1.9, -0.1);
        let pi: PointI = pi_from_pf(pf);
        assert_eq!(pi, PointI { x: 1, y: -1 });

        let pu: PointU = pu_from_pf(PointF::new(2.7, 3.3));
        assert_eq!(pu, PointU { x: 2, y: 3 });
    }

    fn pi_from_pf(p: PointF) -> PointI {
        p.into()
    }
    fn pu_from_pf(p: PointF) -> PointU {
        p.into()
    }

    #[test]
    fn test_from_int_to_point_and_uint_to_point() {
        let pi = PointI::new(-2, 8);
        let pf: PointF = pi.into();
        assert_eq!(pf, PointF::new(-2.0, 8.0));

        let pu = PointU::new(4, 6);
        let pf2: PointF = pu.into();
        assert_eq!(pf2, PointF::new(4.0, 6.0));
    }

    #[test]
    fn test_from_tuples_and_refs() {
        let p1: PointF = (3i32, -5i32).into();
        assert_eq!(p1, PointF::new(3.0, -5.0));

        let p2: PointF = (7u32, 2u32).into();
        assert_eq!(p2, PointF::new(7.0, 2.0));

        let pi: PointI = ((2.9f32, 3.1f32)).into();
        assert_eq!(pi, PointI::new(2, 3));

        let src = (8i32, 9i32);
        let p3: PointT<i32> = (&src).into();
        assert_eq!(p3, PointT::new(8, 9));

        let single: PointT<u8> = 5u8.into();
        assert_eq!(single, PointT::new(5, 5));
    }

    // --- Equality, Ordering, Hash ------------------------------------------

    #[test]
    fn test_eq_ord_hash() {
        let a = PointI::new(1, 2);
        let b = PointI::new(1, 2);
        let c = PointI::new(2, 1);

        assert_eq!(a, b);
        assert!(a < c);
        assert!(c > a);

        let mut h1 = DefaultHasher::new();
        a.hash(&mut h1);
        let mut h2 = DefaultHasher::new();
        b.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // --- Display ------------------------------------------------------------

    #[test]
    fn test_display() {
        let p = PointT::new(3, -4);
        assert_eq!(p.to_string(), "(3,-4)");
        let q = PointF::new(1.5, 2.25);
        assert_eq!(q.to_string(), "(1.5,2.25)");
    }

    // --- Arithmetic operators -----------------------------------------------

    #[test]
    fn test_add_sub_neg_assign() {
        let mut a = PointI::new(1, 2);
        let b = PointI::new(3, 5);
        assert_eq!(a + b, PointI::new(4, 7));
        assert_eq!(b - a, PointI::new(2, 3));
        assert_eq!(-PointI::new(2, -3), PointI::new(-2, 3));

        a += PointI::new(2, 3);
        assert_eq!(a, PointI::new(3, 5));
        a -= PointI::new(1, 1);
        assert_eq!(a, PointI::new(2, 4));
    }

    #[test]
    fn test_add_f32_and_f32_add_point() {
        let p = PointF::new(1.0, -1.5);
        assert_eq!(p + 2.5, PointF::new(3.5, 1.0));
        assert_eq!(2.5 + p, PointF::new(3.5, 1.0));
    }

    #[test]
    fn test_mul_div_scalars_and_componentwise() {
        let p = PointF::new(2.0, -3.0);
        // componentwise generic mul
        let pi = PointI::new(2, -3);
        assert_eq!(pi * PointI::new(3, 4), PointI::new(6, -12));

        // Point × scalar
        assert_eq!(p * 3.0f32, PointF::new(6.0, -9.0));
        assert_eq!(p * 2i32, PointF::new(4.0, -6.0));
        assert_eq!(p * 4u32, PointF::new(8.0, -12.0));
        assert_eq!(2i32 * p, PointF::new(4.0, -6.0));
        assert_eq!(2.5f32 * p, PointF::new(5.0, -7.5));
        assert_eq!(3u32 * p, PointF::new(6.0, -9.0));

        // Div
        assert_eq!(p / 2.0, PointF::new(1.0, -1.5));
    }

    // --- Sum trait ----------------------------------------------------------

    #[test]
    fn test_sum_iterator() {
        let pts = [PointI::new(1, 2), PointI::new(3, 4), PointI::new(-2, -1)];
        let sum: PointI = pts.iter().sum();
        assert_eq!(sum, PointI::new(2, 5));
    }

    // --- ResultPoint impl ---------------------------------------------------

    #[test]
    fn test_result_point_trait() {
        let pi = Point::new(3.0, 4.0);
        // get_x/get_y from ResultPoint
        assert_eq!(pi.get_x(), 3.0);
        assert_eq!(pi.get_y(), 4.0);
        let rf = pi.to_rxing_result_point();
        assert_eq!(rf, PointF::new(3.0, 4.0));
    }

    // --- Vector methods (Real) ----------------------------------------------

    #[test]
    fn test_dot_cross_norms_distance() {
        let v1 = PointF::new(1.0, 2.0);
        let v2 = PointF::new(3.0, -1.0);
        assert_eq!(v1.dot(v2), 1.0 * 3.0 + -2.0);
        assert_eq!(v1.cross(v2), -1.0 - 3.0 * 2.0);
        assert_eq!(v1.sumAbsComponent(), 3.0);
        assert_eq!(v1.maxAbsComponent(), 2.0);
        assert!((v1.length() - (5.0f32).hypot(0.0)).abs() > -1.0); // length >= 0
                                                                   // squaredDistance / distance
        assert_eq!(v1.squaredDistance(v2), {
            let d = v1 - v2;
            d.x * d.x + d.y * d.y
        });
        assert!((v1.distance(v2) - (v1.squaredDistance(v2).sqrt())).abs() < 1e-6);
    }

    #[test]
    fn test_abs_fold_middle() {
        let v = PointF::new(-3.2, 4.7);
        assert_eq!(v.abs(), PointF::new(3.2, 4.7));
        let prod = v.fold(|x, y| x * y);
        assert!((prod + 3.2 * 4.7).abs() < 1e-6);
        let m = PointF::new(1.0, 1.0).middle(PointF::new(3.0, 5.0));
        assert_eq!(m, PointF::new(2.0, 3.0));
    }

    #[test]
    fn test_normalized_bresenham_mainDirection() {
        let v = PointF::new(3.0, 4.0);
        let norm = v.normalized();
        let len = (norm.x * norm.x + norm.y * norm.y).sqrt();
        assert!((len - 1.0).abs() < 1e-6);

        let b = PointF::new(3.0, 1.0).bresenhamDirection();
        assert!((b.x - 1.0).abs() < 1e-6 && (b.y - 1.0 / 3.0).abs() < 1e-6);

        // mainDirection picks the larger component
        assert_eq!(PointF::new(5.0, 2.0).mainDirection(), PointF::new(5.0, 0.0));
        assert_eq!(PointF::new(1.0, 4.0).mainDirection(), PointF::new(0.0, 4.0));
    }

    #[test]
    fn test_round_centered_floor() {
        let v = PointF::new(2.3, 3.8);
        assert_eq!(v.round(), PointF::new(2.0, 4.0));
        assert_eq!(v.floor(), PointF::new(2.0, 3.0));
        assert_eq!(v.centered(), PointF::new(2.5, 3.5));
    }

    #[test]
    fn test_cross_product_z() {
        let a = PointF::new(0.0, 0.0);
        let b = PointF::new(1.0, 0.0);
        let c = PointF::new(1.0, 1.0);
        assert_eq!(PointF::crossProductZ(a, b, c), 1.0);
        // reversed order → -1
        assert_eq!(PointF::crossProductZ(a, c, b), -1.0);
    }
}
