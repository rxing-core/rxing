use crate::common::Result;
use crate::{Exceptions, Point};

use super::{Direction, RegressionLineTrait};

#[inline(always)]
pub fn intersect<T: RegressionLineTrait, T2: RegressionLineTrait>(
    l1: &T,
    l2: &T2,
) -> Result<Point> {
    if !(l1.isValid() && l2.isValid()) {
        return Err(Exceptions::ILLEGAL_STATE);
    }
    let d = l1.a() * l2.b() - l1.b() * l2.a();
    let x = (l1.c() * l2.b() - l1.b() * l2.c()) / d;
    let y = (l1.a() * l2.c() - l1.c() * l2.a()) / d;
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

#[inline(always)]
pub fn UpdateMinMax<T: Ord + Copy>(min: &mut T, max: &mut T, val: T) {
    *min = std::cmp::min(*min, val);
    *max = std::cmp::max(*max, val);
}

#[inline(always)]
pub fn UpdateMinMaxFloat(min: &mut f64, max: &mut f64, val: f64) {
    *min = f64::min(*min, val);
    *max = f64::max(*max, val);
}
