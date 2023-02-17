use crate::common::Result;
use crate::{Exceptions, Point};

use super::{DMRegressionLine, Direction, RegressionLine};

#[inline(always)]
pub fn float_min<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        b
    } else {
        a
    }
}

#[inline(always)]
pub fn float_max<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}

#[inline(always)]
pub fn intersect(l1: &DMRegressionLine, l2: &DMRegressionLine) -> Result<Point> {
    if !(l1.isValid() && l2.isValid()) {
        return Err(Exceptions::illegalState);
    }
    let d = l1.a * l2.b - l1.b * l2.a;
    let x = (l1.c * l2.b - l1.b * l2.c) / d;
    let y = (l1.a * l2.c - l1.c * l2.a) / d;
    Ok(Point { x, y })
}

#[allow(dead_code)]
#[inline(always)]
pub fn opposite(dir: Direction) -> Direction {
    if dir == Direction::Left {
        Direction::Right
    } else {
        Direction::Left
    }
}
