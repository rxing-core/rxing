macro_rules! CHECK {
    ($A:expr) => {
        if (!($A)) {
            continue;
        }
    };
}

/*
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use crate::{
    common::{
        BitMatrix, DefaultGridSampler, DetectorRXingResult, GridSampler, PerspectiveTransform,
    },
    datamatrix::detector::{zxing_cpp_detector::Quadrilateral, DatamatrixDetectorResult},
    qrcode::encoder::ByteMatrix,
    result_point_utils::distance,
    Exceptions, RXingResultPoint, ResultPoint,
};

trait RegressionLine {
    //     points: Vec<RXingResultPoint>,
    //     direction_inward: RXingResultPoint,

    // }
    // impl RegressionLine {
    // std::vector<PointF> _points;
    // PointF _directionInward;
    // PointF::value_t a = NAN, b = NAN, c = NAN;

    fn intersect<T: RegressionLine, T2: RegressionLine>(&self, l1: &T, l2: &T2)
        -> RXingResultPoint;

    //  fn evaluate_begin_end(&self, begin:&RXingResultPoint, end:&RXingResultPoint) -> bool;// {
    // {
    // 	let mean = std::accumulate(begin, end, PointF()) / std::distance(begin, end);
    // 	PointF::value_t sumXX = 0, sumYY = 0, sumXY = 0;
    // 	for (auto p = begin; p != end; ++p) {
    // 		auto d = *p - mean;
    // 		sumXX += d.x * d.x;
    // 		sumYY += d.y * d.y;
    // 		sumXY += d.x * d.y;
    // 	}
    // 	if (sumYY >= sumXX) {
    // 		auto l = std::sqrt(sumYY * sumYY + sumXY * sumXY);
    // 		a = +sumYY / l;
    // 		b = -sumXY / l;
    // 	} else {
    // 		auto l = std::sqrt(sumXX * sumXX + sumXY * sumXY);
    // 		a = +sumXY / l;
    // 		b = -sumXX / l;
    // 	}
    // 	if (dot(_directionInward, normal()) < 0) {
    // 		a = -a;
    // 		b = -b;
    // 	}
    // 	c = dot(normal(), mean); // (a*mean.x + b*mean.y);
    // 	return dot(_directionInward, normal()) > 0.5f; // angle between original and new direction is at most 60 degree
    // }

    fn evaluate(&mut self, points: &[RXingResultPoint]) -> bool; // { return self.evaluate_begin_end(&points.front(), &points.back() + 1); }
    fn evaluateSelf(&mut self) -> bool;

    fn distance(&self, a: &RXingResultPoint, b: &RXingResultPoint) -> f32 {
        return crate::result_point_utils::distance(a, b);
    }

    // RegressionLine() { _points.reserve(16); } // arbitrary but plausible start size (tiny performance improvement)

    // template<typename T> RegressionLine(PointT<T> a, PointT<T> b)
    // {
    // 	evaluate(std::vector{a, b});
    // }

    // template<typename T> RegressionLine(const PointT<T>* b, const PointT<T>* e)
    // {
    // 	evaluate(b, e);
    // }

    fn points(&self) -> &[RXingResultPoint]; //const { return _points; }
    fn length(&self) -> u32; //const { return _points.size() >= 2 ? int(distance(_points.front(), _points.back())) : 0; }
    fn isValid(&self) -> bool; //const { return !std::isnan(a); }
    fn normal(&self) -> RXingResultPoint; //const { return isValid() ? PointF(a, b) : _directionInward; }
    fn signedDistance(&self, p: &RXingResultPoint) -> f32; //const { return dot(normal(), p) - c; }
    fn distance_single(&self, p: &RXingResultPoint) -> f32; //const { return std::abs(signedDistance(PointF(p))); }
    fn project(&self, p: &RXingResultPoint) -> RXingResultPoint {
        *p - self.normal() * self.signedDistance(p)
    }

    fn reset(&mut self);
    // {
    // 	_points.clear();
    // 	_directionInward = {};
    // 	a = b = c = NAN;
    // }

    fn add(&mut self, p: &RXingResultPoint); //{
                                             // 	assert(_directionInward != PointF());
                                             // 	_points.push_back(p);
                                             // 	if (_points.size() == 1)
                                             // 		c = dot(normal(), p);
                                             // }

    fn pop_back(&mut self); // { _points.pop_back(); }

    fn setDirectionInward(&mut self, d: &RXingResultPoint); //{ _directionInward = normalized(d); }

    // fn evaluate(&self, double maxSignedDist = -1, bool updatePoints = false) -> bool
    fn evaluate_max_distance(
        &mut self,
        maxSignedDist: Option<f64>,
        updatePoints: Option<bool>,
    ) -> bool;
    // 	{
    // 		bool ret = evaluate(_points);
    // 		if (maxSignedDist > 0) {
    // 			auto points = _points;
    // 			while (true) {
    // 				auto old_points_size = points.size();
    // 				// remove points that are further 'inside' than maxSignedDist or further 'outside' than 2 x maxSignedDist
    // 				auto end = std::remove_if(points.begin(), points.end(), [this, maxSignedDist](auto p) {
    // 					auto sd = this->signedDistance(p);
    //                     return sd > maxSignedDist || sd < -2 * maxSignedDist;
    // 				});
    // 				points.erase(end, points.end());
    // 				if (old_points_size == points.size())
    // 					break;
    // // #ifdef PRINT_DEBUG
    // // 				printf("removed %zu points\n", old_points_size - points.size());
    // // #endif
    // 				ret = evaluate(points);
    // 			}

    // 			if (updatePoints)
    // 				_points = std::move(points);
    // 		}
    // 		return ret;
    // 	}

    fn isHighRes(&self) -> bool; //const
                                 // {
                                 // 	PointF min = _points.front(), max = _points.front();
                                 // 	for (auto p : _points) {
                                 // 		min.x = std::min(min.x, p.x);
                                 // 		min.y = std::min(min.y, p.y);
                                 // 		max.x = std::max(max.x, p.x);
                                 // 		max.y = std::max(max.y, p.y);
                                 // 	}
                                 // 	auto diff  = max - min;
                                 // 	auto len   = maxAbsComponent(diff);
                                 // 	auto steps = std::min(std::abs(diff.x), std::abs(diff.y));
                                 // 	// due to aliasing we get bad extrapolations if the line is short and too close to vertical/horizontal
                                 // 	return steps > 2 || len > 50;
                                 // }
}

#[inline(always)]
fn intersect(l1: &DMRegressionLine, l2: &DMRegressionLine) -> RXingResultPoint {
    assert!(l1.isValid() && l2.isValid());
    let d = l1.a * l2.b - l1.b * l2.a;
    let x = (l1.c * l2.b - l1.b * l2.c) / d;
    let y = (l1.a * l2.c - l1.c * l2.a) / d;
    RXingResultPoint { x, y }
}

/**
* The following code is the 'new' one implemented by Axel Waggershauser and is working completely different.
* It is performing something like a (back) trace search along edges through the bit matrix, first looking for
* the 'L'-pattern, then tracing the black/white borders at the top/right. Advantages over the old code are:
*  * works with lower resolution scans (around 2 pixel per module), due to sub-pixel precision grid placement
*  * works with real-world codes that have just one module wide quiet-zone (which is perfectly in spec)
*/

#[derive(Default, Clone)]
struct DMRegressionLine {
    points: Vec<RXingResultPoint>,
    direction_inward: RXingResultPoint,
    a: f32,
    b: f32,
    c: f32,
    // std::vector<PointF> _points;
    // PointF _directionInward;
    // PointF::value_t a = NAN, b = NAN, c = NAN;
}
impl RegressionLine for DMRegressionLine {
    fn intersect<T: RegressionLine, T2: RegressionLine>(
        &self,
        l1: &T,
        l2: &T2,
    ) -> RXingResultPoint {
        todo!()
    }

    fn points(&self) -> &[RXingResultPoint] {
        &self.points
    }

    fn length(&self) -> u32 {
        self.points.len() as u32
    }

    fn isValid(&self) -> bool {
        !self.a.is_nan()
    }

    fn normal(&self) -> RXingResultPoint {
        if self.isValid() {
            RXingResultPoint {
                x: self.a,
                y: self.b,
            }
        } else {
            self.direction_inward
        }
    }

    fn signedDistance(&self, p: &RXingResultPoint) -> f32 {
        RXingResultPoint::dot(self.normal(), *p) - self.c
    }

    fn distance_single(&self, p: &RXingResultPoint) -> f32 {
        (self.signedDistance(p)).abs()
    }

    fn reset(&mut self) {
        self.points.clear();
        self.direction_inward = RXingResultPoint { x: 0.0, y: 0.0 };
        self.a = f32::NAN;
        self.b = f32::NAN;
        self.c = f32::NAN;
    }

    fn add(&mut self, p: &RXingResultPoint) {
        // assert(self.direction_inward != RXingResultPoint::default());
        self.points.push(*p);
        if (self.points.len() == 1) {
            self.c = RXingResultPoint::dot(self.normal(), *p);
        }
    }

    fn pop_back(&mut self) {
        self.points.pop();
    }

    fn setDirectionInward(&mut self, d: &RXingResultPoint) {
        self.direction_inward = RXingResultPoint::normalized(*d);
    }

    fn evaluate_max_distance(
        &mut self,
        maxSignedDist: Option<f64>,
        updatePoints: Option<bool>,
    ) -> bool {
        let maxSignedDist = if let Some(m) = maxSignedDist { m } else { -1.0 };
        let updatePoints = if let Some(u) = updatePoints { u } else { false };

        let mut ret = self.evaluateSelf();
        if (maxSignedDist > 0.0) {
            let mut points = self.points.clone();
            loop {
                let old_points_size = points.len();
                // remove points that are further 'inside' than maxSignedDist or further 'outside' than 2 x maxSignedDist
                // auto end = std::remove_if(points.begin(), points.end(), [this, maxSignedDist](auto p) {
                // 	auto sd = this->signedDistance(p);
                //     return sd > maxSignedDist || sd < -2 * maxSignedDist;
                // });
                // points.erase(end, points.end());
                points.retain(|p| {
                    let sd = self.signedDistance(p) as f64;
                    !(sd > maxSignedDist || sd < -2.0 * maxSignedDist)
                });
                if (old_points_size == points.len()) {
                    break;
                }
                // #ifdef PRINT_DEBUG
                // 				printf("removed %zu points\n", old_points_size - points.size());
                // #endif
                ret = self.evaluate(&points);
            }

            if (updatePoints) {
                self.points = points;
            }
        }
        return ret;
    }

    fn isHighRes(&self) -> bool {
        let Some(mut min) = self.points.first().copied() else { return false };
        let Some(mut max) = self.points.first().copied() else { return false };
        for p in &self.points {
            min.x = float_min(min.x, p.x);
            min.y = float_min(min.y, p.y);
            max.x = float_max(max.x, p.x);
            max.y = float_max(max.y, p.y);
        }
        let diff = max - min;
        let len = RXingResultPoint::maxAbsComponent(&diff);
        let steps = float_min((diff.x).abs(), (diff.y).abs());
        // due to aliasing we get bad extrapolations if the line is short and too close to vertical/horizontal
        return steps > 2.0 || len > 50.0;
    }

    fn evaluate(&mut self, points: &[RXingResultPoint]) -> bool {
        let mean = points.iter().sum::<RXingResultPoint>() / points.len() as f32;

        let mut sumXX = 0.0;
        let mut sumYY = 0.0;
        let mut sumXY = 0.0;
        for p in points {
            // for (auto p = begin; p != end; ++p) {
            let d = *p - mean;
            sumXX += d.x * d.x;
            sumYY += d.y * d.y;
            sumXY += d.x * d.y;
        }
        if (sumYY >= sumXX) {
            let l = (sumYY * sumYY + sumXY * sumXY).sqrt();
            self.a = sumYY / l;
            self.b = -sumXY / l;
        } else {
            let l = (sumXX * sumXX + sumXY * sumXY).sqrt();
            self.a = sumXY / l;
            self.b = -sumXX / l;
        }
        if RXingResultPoint::dot(self.direction_inward, self.normal()) < 0.0 {
            // if (dot(_directionInward, normal()) < 0) {
            self.a = -self.a;
            self.b = -self.b;
        }
        self.c = RXingResultPoint::dot(self.normal(), mean); // (a*mean.x + b*mean.y);
        return RXingResultPoint::dot(self.direction_inward, self.normal()) > 0.5;
        // angle between original and new direction is at most 60 degree
    }

    fn evaluateSelf(&mut self) -> bool {
        let mean = self.points.iter().sum::<RXingResultPoint>() / self.points.len() as f32;

        let mut sumXX = 0.0;
        let mut sumYY = 0.0;
        let mut sumXY = 0.0;
        for p in &self.points {
            // for (auto p = begin; p != end; ++p) {
            let d = *p - mean;
            sumXX += d.x * d.x;
            sumYY += d.y * d.y;
            sumXY += d.x * d.y;
        }
        if (sumYY >= sumXX) {
            let l = (sumYY * sumYY + sumXY * sumXY).sqrt();
            self.a = sumYY / l;
            self.b = -sumXY / l;
        } else {
            let l = (sumXX * sumXX + sumXY * sumXY).sqrt();
            self.a = sumXY / l;
            self.b = -sumXX / l;
        }
        if RXingResultPoint::dot(self.direction_inward, self.normal()) < 0.0 {
            // if (dot(_directionInward, normal()) < 0) {
            self.a = -self.a;
            self.b = -self.b;
        }
        self.c = RXingResultPoint::dot(self.normal(), mean); // (a*mean.x + b*mean.y);
        return RXingResultPoint::dot(self.direction_inward, self.normal()) > 0.5;
        // angle between original and new direction is at most 60 degree
    }
}

impl DMRegressionLine {
    // template <typename Container, typename Filter>
    fn average<'a, T>(c: &'a [f64], f: T) -> f64
    where
        T: Fn(f64) -> bool,
    {
        let mut sum: f64 = 0.0;
        let mut num = 0;
        for v in c {
            // for (const auto& v : c)
            if (f(*v)) {
                sum += *v;
                num += 1;
            }
        }
        sum / num as f64
    }
    fn reverse(&mut self) {
        self.points.reverse();
    }

    fn modules(&mut self, beg: &RXingResultPoint, end: &RXingResultPoint) -> f64 {
        // assert(_points.size() > 3);

        // re-evaluate and filter out all points too far away. required for the gapSizes calculation.
        self.evaluate_max_distance(Some(1.0), Some(true));

        // std::vector<double> gapSizes, modSizes;
        let mut gapSizes: Vec<f64> = Vec::new();
        let mut modSizes = Vec::new();

        gapSizes.reserve(self.points.len());

        // calculate the distance between the points projected onto the regression line
        for i in 1..self.points.len() {
            // for (size_t i = 1; i < _points.size(); ++i)
            gapSizes.push(self.distance(
                &self.project(&self.points[i]),
                &self.project(&self.points[i - 1]),
            ) as f64);
        }

        // calculate the (expected average) distance of two adjacent pixels
        let unitPixelDist = RXingResultPoint::length(RXingResultPoint::bresenhamDirection(
            &(*self.points.last().clone().unwrap() - *self.points.first().clone().unwrap()),
        )) as f64;

        // calculate the width of 2 modules (first black pixel to first black pixel)
        let mut sumFront: f64 =
            (self.distance(beg, &self.project(&self.points[0])) as f64 - unitPixelDist) as f64;
        let mut sumBack: f64 = 0.0; // (last black pixel to last black pixel)
        for dist in gapSizes {
            // for (auto dist : gapSizes) {
            if (dist > 1.9 * unitPixelDist) {
                modSizes.push(std::mem::take(&mut sumBack));
            }
            sumFront += dist;
            sumBack += dist;
            if (dist > 1.9 * unitPixelDist) {
                modSizes.push(std::mem::take(&mut sumFront));
            }
        }

        modSizes.push(
            sumFront
                + self.distance(end, &self.project(&self.points.last().clone().unwrap())) as f64,
        );
        modSizes[0] = 0.0; // the first element is an invalid sumBack value, would be pop_front() if vector supported this
        let lineLength = self.distance(beg, end) as f64 - unitPixelDist;
        let mut meanModSize = Self::average(&modSizes, |_: f64| true);
        // let meanModSize = average(modSizes, [](double){ return true; });
        // #ifdef PRINT_DEBUG
        // 		printf("unit pixel dist: %.1f\n", unitPixelDist);
        // 		printf("lineLength: %.1f, meanModSize: %.1f, gaps: %lu\n", lineLength, meanModSize, modSizes.size());
        // #endif
        for i in 0..2 {
            // for (int i = 0; i < 2; ++i)
            meanModSize = Self::average(&modSizes, |dist: f64| {
                (dist - meanModSize).abs() < meanModSize / (2 + i) as f64
            });
            // meanModSize = average(modSizes, [=](double dist) { return std::abs(dist - meanModSize) < meanModSize / (2 + i); });
        }
        // #ifdef PRINT_DEBUG
        // 		printf("post filter meanModSize: %.1f\n", meanModSize);
        // #endif
        return lineLength / meanModSize;
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    LEFT = -1,
    RIGHT = 1,
}

impl From<Direction> for i32 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::LEFT => -1,
            Direction::RIGHT => 1,
        }
    }
}

#[inline(always)]
fn opposite(dir: Direction) -> Direction {
    if dir == Direction::LEFT {
        Direction::RIGHT
    } else {
        Direction::LEFT
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Value {
    INVALID = -1,
    WHITE = 0,
    BLACK = 1,
}
impl Value {
    pub fn isBlack(&self) -> bool {
        self == &Value::BLACK
    }
    pub fn isWhite(&self) -> bool {
        self == &Value::WHITE
    }
    pub fn isValid(&self) -> bool {
        self != &Value::INVALID
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        match value {
            true => Value::BLACK,
            false => Value::WHITE,
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::INVALID => false,
            Value::WHITE => true,
            Value::BLACK => true,
        }
    }
}

/**
 * @brief The BitMatrixCursor represents a current position inside an image and current direction it can advance towards.
 *
 * The current position and direction is a PointT<T>. So depending on the type it can be used to traverse the image
 * in a Bresenham style (PointF) or in a discrete way (step only horizontal/vertical/diagonal (PointI)).
 */
trait BitMatrixCursor {
    // const BitMatrix* img;

    // POINT p; // current position
    // POINT d; // current direction

    // BitMatrixCursor(const BitMatrix& image, POINT p, POINT d) : img(&image), p(p) { setDirection(d); }

    fn testAt(&self, p: &RXingResultPoint) -> Value; //const
                                                     // {
                                                     // 	return img->isIn(p) ? Value{img->get(p)} : Value{};
                                                     // }

    fn blackAt(&self, pos: &RXingResultPoint) -> bool {
        return self.testAt(pos).isBlack();
    }
    fn whiteAt(&self, pos: &RXingResultPoint) -> bool {
        return self.testAt(pos).isWhite();
    }

    fn isIn(&self, p: &RXingResultPoint) -> bool; // { return img->isIn(p); }
    fn isInSelf(&self) -> bool; // { return self.isIn(p); }
    fn isBlack(&self) -> bool; // { return blackAt(p); }
    fn isWhite(&self) -> bool; // { return whiteAt(p); }

    fn front(&self) -> &RXingResultPoint; //{ return d; }
    fn back(&self) -> RXingResultPoint; // { return {-d.x, -d.y}; }
    fn left(&self) -> RXingResultPoint; //{ return {d.y, -d.x}; }
    fn right(&self) -> RXingResultPoint; //{ return {-d.y, d.x}; }
    fn direction(&self, dir: Direction) -> RXingResultPoint {
        return self.right() * Into::<i32>::into(dir);
    }

    fn turnBack(&mut self); // noexcept { d = back(); }
    fn turnLeft(&mut self); //noexcept { d = left(); }
    fn turnRight(&mut self); //noexcept { d = right(); }
    fn turn(&mut self, dir: Direction); //noexcept { d = direction(dir); }

    fn edgeAt_point(&self, d: &RXingResultPoint) -> Value;
    // {
    // 	Value v = testAt(p);
    // 	return testAt(p + d) != v ? v : Value();
    // }

    fn edgeAtFront(&self) -> Value {
        return self.edgeAt_point(self.front());
    }
    fn edgeAtBack(&self) -> Value {
        return self.edgeAt_point(&self.back());
    }
    fn edgeAtLeft(&self) -> Value {
        return self.edgeAt_point(&self.left());
    }
    fn edgeAtRight(&self) -> Value {
        return self.edgeAt_point(&self.right());
    }
    fn edgeAt_direction(&self, dir: Direction) -> Value {
        return self.edgeAt_point(&self.direction(dir));
    }

    fn setDirection(&mut self, dir: &RXingResultPoint); // { d = bresenhamDirection(dir); }
                                                        // fn setDirection(&self, dir:&RXingResultPoint);// { d = dir; }

    fn step(&mut self, s: Option<f32>) -> bool; // DEF to 1
                                                // {
                                                // 	p += s * d;
                                                // 	return isIn(p);
                                                // }

    fn movedBy<T: BitMatrixCursor>(self, d: &RXingResultPoint) -> Self;
    // {
    // 	auto res = *this;
    // 	res.p += d;
    // 	return res;
    // }

    /**
     * @brief stepToEdge advances cursor to one step behind the next (or n-th) edge.
     * @param nth number of edges to pass
     * @param range max number of steps to take
     * @param backup whether or not to backup one step so we land in front of the edge
     * @return number of steps taken or 0 if moved outside of range/image
     */
    fn stepToEdge(&mut self, nth: Option<i32>, range: Option<i32>, backup: Option<bool>) -> i32;
    // fn stepToEdge(&self, int nth = 1, int range = 0, bool backup = false) -> i32
    // {
    // 	// TODO: provide an alternative and faster out-of-bounds check than isIn() inside testAt()
    // 	int steps = 0;
    // 	auto lv = testAt(p);

    // 	while (nth && (!range || steps < range) && lv.isValid()) {
    // 		++steps;
    // 		auto v = testAt(p + steps * d);
    // 		if (lv != v) {
    // 			lv = v;
    // 			--nth;
    // 		}
    // 	}
    // 	if (backup)
    // 		--steps;
    // 	p += steps * d;
    // 	return steps * (nth == 0);
    // }

    fn stepAlongEdge(&mut self, dir: Direction, skipCorner: Option<bool>) -> bool
// fn stepAlongEdge(&self,  dir:Direction, skipCorner:Option<bool> = false) -> bool
    {
        let skipCorner = if let Some(sc) = skipCorner { sc } else { false };

        if (false == self.edgeAt_direction(dir).into()) {
            self.turn(dir);
        } else if (true == self.edgeAtFront().into()) {
            self.turn(opposite(dir));
            if (true == self.edgeAtFront().into()) {
                self.turn(opposite(dir));
                if (true == self.edgeAtFront().into()) {
                    return false;
                }
            }
        }

        let mut ret = self.step(None);

        if (ret && skipCorner && false == self.edgeAt_direction(dir).into()) {
            self.turn(dir);
            ret = self.step(None);
        }

        return ret;
    }

    fn countEdges(&mut self, range: Option<i32>) -> i32 {
        let mut range = if let Some(r) = range { r } else { 0 };
        let mut res = 0;

        let mut steps = self.stepToEdge(Some(1), Some(range), None);

        while (steps > 0) {
            range -= steps;
            res += 1;
            steps = self.stepToEdge(Some(1), Some(range), None);
        }

        return res;
    }

    // template<typename ARRAY>
    // ARRAY readPattern(int range = 0)
    // {
    // 	ARRAY res;
    // 	for (auto& i : res)
    // 		i = stepToEdge(1, range);
    // 	return res;
    // }

    // template<typename ARRAY>
    // ARRAY readPatternFromBlack(int maxWhitePrefix, int range = 0)
    // {
    // 	if (maxWhitePrefix && isWhite() && !stepToEdge(1, maxWhitePrefix))
    // 		return {};
    // 	return readPattern<ARRAY>(range);
    // }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum StepResult {
    FOUND,
    OPEN_END,
    CLOSED_END,
}

#[derive(Clone)]
struct EdgeTracer<'a> {
    img: &'a BitMatrix,

    p: RXingResultPoint, // current position
    d: RXingResultPoint, // current direction

    // pub history: Option<&'a mut ByteMatrix>, // = nullptr;
    pub history: Option<Rc<RefCell<ByteMatrix>>>,
    pub state: i32,
    // const BitMatrix* img;

    // POINT p; // current position
    // POINT d; // current direction
}

// impl<'a> Clone for EdgeTracer<'_> {
//     fn clone(&self) -> Self {
//         if let Some(history) = self.history {
//             Self { img: self.img, p: self.p.clone(), d: self.d.clone(), history: Some(history), state: self.state.clone() }
//         }else {
//         Self { img: self.img, p: self.p.clone(), d: self.d.clone(), history: None, state: self.state.clone() }
//         }
//     }
// }

impl<'a> BitMatrixCursor for EdgeTracer<'_> {
    fn testAt(&self, p: &RXingResultPoint) -> Value {
        if self.img.isIn(p, 0) {
            Value::from(self.img.get_point(p))
        } else {
            Value::INVALID
        }
    }

    fn isIn(&self, p: &RXingResultPoint) -> bool {
        self.img.isIn(p, 0)
    }

    fn isInSelf(&self) -> bool {
        self.isIn(&self.p)
    }

    fn isBlack(&self) -> bool {
        self.blackAt(&self.p)
    }

    fn isWhite(&self) -> bool {
        self.whiteAt(&self.p)
    }

    fn front(&self) -> &RXingResultPoint {
        &self.d
    }

    fn back(&self) -> RXingResultPoint {
        RXingResultPoint {
            x: -self.d.x,
            y: -self.d.y,
        }
    }

    fn left(&self) -> RXingResultPoint {
        RXingResultPoint {
            x: self.d.y,
            y: -self.d.x,
        }
    }

    fn right(&self) -> RXingResultPoint {
        RXingResultPoint {
            x: -self.d.y,
            y: self.d.x,
        }
    }

    fn turnBack(&mut self) {
        self.d = self.back()
    }

    fn turnLeft(&mut self) {
        self.d = self.left()
    }

    fn turnRight(&mut self) {
        self.d = self.right()
    }

    fn turn(&mut self, dir: Direction) {
        self.d = self.direction(dir)
    }

    fn edgeAt_point(&self, d: &RXingResultPoint) -> Value {
        let v = self.testAt(&self.p);
        if self.testAt(&(self.p + *d)) != v {
            v
        } else {
            Value::INVALID
        }
    }

    fn setDirection(&mut self, dir: &RXingResultPoint) {
        self.d = RXingResultPoint::bresenhamDirection(dir)
    }

    fn step(&mut self, s: Option<f32>) -> bool {
        let s = if let Some(s) = s { s } else { 1.0 };
        self.p += self.d * s;
        return self.isIn(&self.p);
    }

    fn movedBy<T: BitMatrixCursor>(self, d: &RXingResultPoint) -> Self {
        let mut res = self;
        res.p += *d;

        res
    }

    /**
     * @brief stepToEdge advances cursor to one step behind the next (or n-th) edge.
     * @param nth number of edges to pass
     * @param range max number of steps to take
     * @param backup whether or not to backup one step so we land in front of the edge
     * @return number of steps taken or 0 if moved outside of range/image
     */
    fn stepToEdge(&mut self, nth: Option<i32>, range: Option<i32>, backup: Option<bool>) -> i32 {
        let mut nth = if let Some(nth) = nth { nth } else { 1 };
        let range = if let Some(r) = range { r } else { 0 };
        let backup = if let Some(b) = backup { b } else { false };
        // TODO: provide an alternative and faster out-of-bounds check than isIn() inside testAt()
        let mut steps = 0;
        let mut lv = self.testAt(&self.p);

        while nth > 0 && (!(range > 0) || steps < range) && lv.isValid() {
            steps += 1;
            let v = self.testAt(&(self.p + steps * self.d));
            if (lv != v) {
                lv = v;
                nth -= 1;
            }
        }
        if (backup) {
            steps -= 1;
        }
        self.p += self.d * steps;
        return steps * i32::from(nth == 0);
    }
}

impl<'a> EdgeTracer<'_> {
    pub fn new(image: &'a BitMatrix, p: RXingResultPoint, d: RXingResultPoint) -> EdgeTracer<'a> {
        // : img(&image), p(p) { setDirection(d); }
        EdgeTracer {
            img: image,
            p,
            d,
            history: None,
            state: 0,
        }
    }

    fn traceStep(
        &mut self,
        dEdge: &RXingResultPoint,
        maxStepSize: i32,
        goodDirection: bool,
    ) -> StepResult {
        let dEdge = RXingResultPoint::mainDirection(*dEdge);
        for breadth in 1..=(if maxStepSize == 1 {
            2
        } else {
            (if goodDirection { 1 } else { 3 })
        }) {
            // for (int breadth = 1; breadth <= (maxStepSize == 1 ? 2 : (goodDirection ? 1 : 3)); ++breadth)
            for step in 1..=maxStepSize {
                // for (int step = 1; step <= maxStepSize; ++step)
                for i in 0..=(2 * (step / 4 + 1) * breadth) {
                    // for (int i = 0; i <= 2*(step/4+1) * breadth; ++i) {
                    let mut pEdge = self.p
                        + step * self.d
                        + (if i & 1 > 0 { (i + 1) / 2 } else { -i / 2 }) * dEdge;
                    // dbg!(pEdge);

                    if (!self.blackAt(&(pEdge + dEdge))) {
                        continue;
                    }

                    // found black pixel -> go 'outward' until we hit the b/w border
                    for j in 0..(std::cmp::max(maxStepSize, 3)) {
                        // for (int j = 0; j < std::max(maxStepSize, 3) && isIn(pEdge); ++j) {
                        if (self.whiteAt(&pEdge)) {
                            // if we are not making any progress, we still have another endless loop bug
                            assert!(self.p != RXingResultPoint::centered(&pEdge));
                            self.p = RXingResultPoint::centered(&pEdge);

                            // if (self.history && maxStepSize == 1) {
                            if let Some(history) = &self.history {
                                if (maxStepSize == 1) {
                                    if (history.borrow().get(self.p.x as u32, self.p.y as u32)
                                        == self.state as u8)
                                    {
                                        return StepResult::CLOSED_END;
                                    }
                                    history.borrow_mut().set(
                                        self.p.x as u32,
                                        self.p.y as u32,
                                        self.state as u8,
                                    );
                                }
                            }

                            return StepResult::FOUND;
                        }
                        pEdge = pEdge - dEdge;
                        if (self.blackAt(&(pEdge - self.d))) {
                            pEdge = pEdge - self.d;
                        }
                        // dbg!(pEdge);

                        if !self.isIn(&pEdge) {
                            break;
                        }
                    }
                    // no valid b/w border found within reasonable range
                    return StepResult::CLOSED_END;
                }
            }
        }
        return StepResult::OPEN_END;
    }

    pub fn updateDirectionFromOrigin(&mut self, origin: &RXingResultPoint) -> bool {
        let old_d = self.d;
        self.setDirection(&(self.p - origin));
        // if the new direction is pointing "backward", i.e. angle(new, old) > 90 deg -> break
        if (RXingResultPoint::dot(self.d, old_d) < 0.0) {
            return false;
        }
        // make sure d stays in the same quadrant to prevent an infinite loop
        if ((self.d.x).abs() == (self.d.y).abs()) {
            self.d = RXingResultPoint::mainDirection(old_d)
                + 0.99 * (self.d - RXingResultPoint::mainDirection(old_d));
        } else if (RXingResultPoint::mainDirection(self.d)
            != RXingResultPoint::mainDirection(old_d))
        {
            self.d = RXingResultPoint::mainDirection(old_d)
                + 0.99 * RXingResultPoint::mainDirection(self.d);
        }
        return true;
    }

    pub fn traceLine<T: RegressionLine>(&mut self, dEdge: &RXingResultPoint, line: &mut T) -> bool {
        line.setDirectionInward(dEdge);
        loop {
            // log(self.p);
            line.add(&self.p);
            if (line.points().len() % 50 == 10) {
                if (!line.evaluate_max_distance(None, None)) {
                    return false;
                }
                if (!self.updateDirectionFromOrigin(
                    &(self.p - line.project(&self.p) + **line.points().first().as_ref().unwrap()),
                )) {
                    return false;
                }
            }
            let stepResult = self.traceStep(dEdge, 1, line.isValid());
            if (stepResult != StepResult::FOUND) {
                return stepResult == StepResult::OPEN_END && line.points().len() > 1;
            }
        } // while (true);
    }

    pub fn traceGaps<T: RegressionLine>(
        &mut self,
        dEdge: &RXingResultPoint,
        line: &mut T,
        maxStepSize: i32,
        finishLine: &T,
    ) -> bool {
        let mut maxStepSize = maxStepSize;
        line.setDirectionInward(dEdge);
        let mut gaps = 0;
        loop {
            // detect an endless loop (lack of progress). if encountered, please report.
            assert!(line.points().is_empty() || &&self.p != line.points().last().as_ref().unwrap());
            if (!line.points().is_empty() && &&self.p == line.points().last().as_ref().unwrap()) {
                return false;
            }
            // log(p);

            // if we drifted too far outside of the code, break
            if (line.isValid()
                && line.signedDistance(&self.p) < -5.0
                && (!line.evaluate_max_distance(None, None) || line.signedDistance(&self.p) < -5.0))
            {
                return false;
            }

            // if we are drifting towards the inside of the code, pull the current position back out onto the line
            if (line.isValid() && line.signedDistance(&self.p) > 3.0) {
                // The current direction d and the line we are tracing are supposed to be roughly parallel.
                // In case the 'go outward' step in traceStep lead us astray, we might end up with a line
                // that is almost perpendicular to d. Then the back-projection below can result in an
                // endless loop. Break if the angle between d and line is greater than 45 deg.
                if ((RXingResultPoint::dot(RXingResultPoint::normalized(self.d), line.normal()))
                    .abs()
                    > 0.7)
                // thresh is approx. sin(45 deg)
                {
                    return false;
                }

                let mut np = line.project(&self.p);
                // make sure we are making progress even when back-projecting:
                // consider a 90deg corner, rotated 45deg. we step away perpendicular from the line and get
                // back projected where we left off the line.
                // The 'while' instead of 'if' was introduced to fix the issue with #245. It turns out that
                // np can actually be behind the projection of the last line point and we need 2 steps in d
                // to prevent a dead lock. see #245.png
                while (RXingResultPoint::distance(
                    np,
                    line.project(line.points().last().as_ref().unwrap()),
                ) < 1.0)
                {
                    np = np + self.d;
                }
                self.p = RXingResultPoint::centered(&np);
            } else {
                let stepLengthInMainDir = if line.points().is_empty() {
                    0.0
                } else {
                    RXingResultPoint::dot(
                        RXingResultPoint::mainDirection(self.d),
                        (self.p - line.points().last().unwrap()),
                    )
                };
                line.add(&self.p);

                if (stepLengthInMainDir > 1.0) {
                    gaps += 1;
                    if (gaps >= 2 || line.points().len() > 5) {
                        if (!line.evaluate_max_distance(Some(1.5), None)) {
                            return false;
                        }
                        if (!self.updateDirectionFromOrigin(
                            &(self.p - line.project(&self.p) + *line.points().first().unwrap()),
                        )) {
                            return false;
                        }
                        // check if the first half of the top-line trace is complete.
                        // the minimum code size is 10x10 -> every code has at least 4 gaps
                        //TODO: maybe switch to termination condition based on bottom line length to get a better
                        // finishLine for the right line trace
                        if (!finishLine.isValid() && gaps == 4) {
                            // undo the last insert, it will be inserted again after the restart
                            line.pop_back();
                            gaps -= 1;
                            return true;
                        }
                    }
                } else if (gaps == 0 && line.points().len() >= (2 * maxStepSize) as usize) {
                    return false;
                } // no point in following a line that has no gaps
            }

            if (finishLine.isValid()) {
                maxStepSize =
                    std::cmp::min(maxStepSize, (finishLine.signedDistance(&self.p)) as i32);
            }

            let stepResult = self.traceStep(dEdge, maxStepSize, line.isValid());

            if (stepResult != StepResult::FOUND)
            // we are successful iff we found an open end across a valid finishLine
            {
                return stepResult == StepResult::OPEN_END
                    && finishLine.isValid()
                    && (finishLine.signedDistance(&self.p)) as i32 <= maxStepSize + 1;
            }
        } //while (true);
    }

    pub fn traceCorner(&mut self, dir: &mut RXingResultPoint, corner: &RXingResultPoint) -> bool {
        self.step(None);
        // log(p);
        let corner = self.p;
        std::mem::swap(&mut self.d, dir);
        self.traceStep(&(-1.0 * dir), 2, false);
        // #ifdef PRINT_DEBUG
        // 		printf("turn: %.0f x %.0f -> %.2f, %.2f\n", p.x, p.y, d.x, d.y);
        // #endif
        return self.isIn(&corner) && self.isIn(&self.p);
    }
}

fn Scan(
    startTracer: &mut EdgeTracer,
    lines: &mut [DMRegressionLine; 4],
) -> Result<DatamatrixDetectorResult, Exceptions> {
    while (startTracer.step(None)) {
        //log(startTracer.p);

        // continue until we cross from black into white
        if (!startTracer.edgeAtBack().isWhite()) {
            continue;
        }

        let mut tl = RXingResultPoint::default();
        let mut bl = RXingResultPoint::default();
        let mut br = RXingResultPoint::default();
        let mut tr = RXingResultPoint::default();

        for l in lines.iter_mut() {
            l.reset();
        }

        let [lineL, lineB, lineR, lineT] = lines;

        // for l in lines {
        //     l.reset();
        // }

        // #ifdef PRINT_DEBUG
        // 		SCOPE_EXIT([&] {
        // 			for (auto& l : lines)
        // 				log(l.points());
        // 		});
        // # define CHECK(A) if (!(A)) { printf("broke at %d\n", __LINE__); continue; }
        // #else
        // # define CHECK(A) if(!(A)) continue
        // #endif

        let mut t = startTracer.clone();

        // follow left leg upwards
        t.turnRight();
        t.state = 1;
        CHECK!(t.traceLine(&t.right(), lineL));
        CHECK!(t.traceCorner(&mut t.right(), &tl));
        lineL.reverse();
        let mut tlTracer = t;

        // follow left leg downwards
        t = startTracer.clone();
        t.state = 1;
        t.setDirection(&tlTracer.right());
        CHECK!(t.traceLine(&t.left(), lineL));
        if (!lineL.isValid()) {
            t.updateDirectionFromOrigin(&tl);
        }
        let up = t.back();
        CHECK!(t.traceCorner(&mut t.left(), &bl));

        // follow bottom leg right
        t.state = 2;
        CHECK!(t.traceLine(&t.left(), lineB));
        if (!lineB.isValid()) {
            t.updateDirectionFromOrigin(&bl);
        }
        let right = *t.front();
        CHECK!(t.traceCorner(&mut t.left(), &br));

        let lenL = distance(&tl, &bl) - 1.0;
        let lenB = distance(&bl, &br) - 1.0;
        CHECK!(lenL >= 8.0 && lenB >= 10.0 && lenB >= lenL / 4.0 && lenB <= lenL * 18.0);

        let mut maxStepSize: i32 = (lenB / 5.0 + 1.0) as i32; // datamatrix bottom dim is at least 10

        // at this point we found a plausible L-shape and are now looking for the b/w pattern at the top and right:
        // follow top row right 'half way' (4 gaps), see traceGaps break condition with 'invalid' line
        tlTracer.setDirection(&right);
        CHECK!(tlTracer.traceGaps(
            &tlTracer.right(),
            lineT,
            maxStepSize,
            &DMRegressionLine::default()
        ));

        maxStepSize = std::cmp::min(lineT.length() as i32 / 3, (lenL / 5.0) as i32) * 2;

        // follow up until we reach the top line
        t.setDirection(&up);
        t.state = 3;
        CHECK!(t.traceGaps(&t.left(), lineR, maxStepSize, lineT));
        CHECK!(t.traceCorner(&mut t.left(), &tr));

        let lenT = distance(&tl, &tr) - 1.0;
        let lenR = distance(&tr, &br) - 1.0;

        CHECK!(
            (lenT - lenB).abs() / lenB < 0.5
                && (lenR - lenL).abs() / lenL < 0.5
                && lineT.points().len() >= 5
                && lineR.points().len() >= 5
        );

        // continue top row right until we cross the right line
        CHECK!(tlTracer.traceGaps(&tlTracer.right(), lineT, maxStepSize, lineR));

        // #ifdef PRINT_DEBUG
        // 		printf("L: %.1f, %.1f ^ %.1f, %.1f > %.1f, %.1f (%d : %d : %d : %d)\n", bl.x, bl.y,
        // 			   tl.x - bl.x, tl.y - bl.y, br.x - bl.x, br.y - bl.y, (int)lenL, (int)lenB, (int)lenT, (int)lenR);
        // #endif

        // for l in [lineL, lineB, lineT, lineR] {
        //     l.evaluate_max_distance(Some(1.0), None);
        // }
        lineL.evaluate_max_distance(Some(1.0), None);
        lineB.evaluate_max_distance(Some(1.0), None);
        lineT.evaluate_max_distance(Some(1.0), None);
        lineR.evaluate_max_distance(Some(1.0), None);

        // find the bounding box corners of the code with sub-pixel precision by intersecting the 4 border lines
        bl = intersect(lineB, lineL);
        tl = intersect(lineT, lineL);
        tr = intersect(lineT, lineR);
        br = intersect(lineB, lineR);

        let mut dimT: i32 = 0;
        let mut dimR: i32 = 0;
        let mut fracT: f64 = 0.0;
        let mut fracR: f64 = 0.0;
        let splitDouble = |d: f64, i: &mut i32, f: &mut f64| {
            *i = if d.is_normal() { (d + 0.5) as i32 } else { 0 };
            *f = if d.is_normal() {
                (d - *i as f64).abs()
            } else {
                f64::INFINITY
            };
        };
        splitDouble(lineT.modules(&tl, &tr), &mut dimT, &mut fracT);
        splitDouble(lineR.modules(&br, &tr), &mut dimR, &mut fracR);

        // #ifdef PRINT_DEBUG
        // 		printf("L: %.1f, %.1f ^ %.1f, %.1f > %.1f, %.1f ^> %.1f, %.1f\n", bl.x, bl.y,
        // 			   tl.x - bl.x, tl.y - bl.y, br.x - bl.x, br.y - bl.y, tr.x, tr.y);
        // 		printf("dim: %d x %d\n", dimT, dimR);
        // #endif

        // if we have an almost square (invalid rectangular) data matrix dimension, we try to parse it by assuming a
        // square. we use the dimension that is closer to an integral value. all valid rectangular symbols differ in
        // their dimension by at least 10 (here 5, see doubling below). Note: this is currently not required for the
        // black-box tests to complete.
        if ((dimT - dimR).abs() < 5) {
            dimR = if fracR < fracT { dimR } else { dimT };
            dimT = dimR;
        }

        // the dimension is 2x the number of black/white transitions
        dimT *= 2;
        dimR *= 2;

        CHECK!(dimT >= 10 && dimT <= 144 && dimR >= 8 && dimR <= 144);

        let movedTowardsBy = |a: &RXingResultPoint,
                              b1: &RXingResultPoint,
                              b2: &RXingResultPoint,
                              d: f32|
         -> RXingResultPoint {
            *a + d * RXingResultPoint::normalized(
                RXingResultPoint::normalized(*b1 - *a) + RXingResultPoint::normalized(*b2 - *a),
            )
        };

        // shrink shape by half a pixel to go from center of white pixel outside of code to the edge between white and black
        let sourcePoints = Quadrilateral::with_points(
            movedTowardsBy(&tl, &tr, &bl, 0.5),
            // move the tr point a little less because the jagged top and right line tend to be statistically slightly
            // inclined toward the center anyway.
            movedTowardsBy(&tr, &br, &tl, 0.3),
            movedTowardsBy(&br, &bl, &tr, 0.5),
            movedTowardsBy(&bl, &tl, &br, 0.5),
        );

        let grid_sampler = DefaultGridSampler::default();
        // let transform = PerspectiveTransform::quadrilateralToQuadrilateral(x0, y0, x1, y1, x2, y2, x3, y3, x0p, y0p, x1p, y1p, x2p, y2p, x3p, y3p);

        let res = grid_sampler.sample_grid_detailed(
            startTracer.img,
            dimT as u32,
            dimR as u32,
            0.0,
            0.0,
            dimT as f32,
            0.0,
            dimT as f32,
            dimR as f32,
            0.0,
            dimR as f32,
            sourcePoints.topLeft().getX(),
            sourcePoints.topLeft().getY(),
            sourcePoints.topRight().getX(),
            sourcePoints.topRight().getY(),
            sourcePoints.bottomRight().getX(),
            sourcePoints.bottomRight().getY(),
            sourcePoints.bottomLeft().getX(),
            sourcePoints.bottomLeft().getY(),
        );

        // let res = grid_sampler.sample_grid(startTracer.img, dimT as u32, dimR as u32, &transform);

        // let res = SampleGrid(*startTracer.img, dimT, dimR, PerspectiveTransform(Rectangle(dimT, dimR, 0), sourcePoints));

        CHECK!(!res.is_ok());

        return Ok(DatamatrixDetectorResult::new(
            res.unwrap(),
            sourcePoints.points().to_vec(),
        ));
    }

    Err(Exceptions::NotFoundException(None))
}

pub fn detect(
    image: &BitMatrix,
    tryHarder: bool,
    tryRotate: bool,
) -> Result<DatamatrixDetectorResult, Exceptions> {
    // #ifdef PRINT_DEBUG
    // 	LogMatrixWriter lmw(log, image, 1, "dm-log.pnm");
    // //	tryRotate = tryHarder = false;
    // #endif

    // disable expensive multi-line scan to detect off-center symbols for now
    // #ifndef __cpp_impl_coroutine
    // 	tryHarder = false;
    // #endif

    // a history log to remember where the tracing already passed by to prevent a later trace from doing the same work twice
    let mut history = None;
    if (tryHarder) {
        history = Some(Rc::new(RefCell::new(ByteMatrix::new(
            image.getWidth(),
            image.getHeight(),
        ))));
    }

    // instantiate RegressionLine objects outside of Scan function to prevent repetitive std::vector allocations
    let mut lines = [
        DMRegressionLine::default(),
        DMRegressionLine::default(),
        DMRegressionLine::default(),
        DMRegressionLine::default(),
    ]; // [DMRegressionLine::default();4];

    const minSymbolSize: u32 = 8 * 2; // minimum realistic size in pixel: 8 modules x 2 pixels per module

    for dir in [
        RXingResultPoint { x: -1.0, y: 0.0 },
        RXingResultPoint { x: 1.0, y: 0.0 },
        RXingResultPoint { x: 0.0, y: -1.0 },
        RXingResultPoint { x: 0.0, y: 1.0 },
    ] {
        // for (auto dir : {PointF(-1, 0), PointF(1, 0), PointF(0, -1), PointF(0, 1)}) {
        let center = RXingResultPoint {
            x: (image.getWidth() / 2) as f32,
            y: (image.getHeight() / 2) as f32,
        }; //PointF(image.width() / 2, image.height() / 2);
        let startPos =
            RXingResultPoint::centered(&(center - center * dir + minSymbolSize as i32 / 2 * dir));

        if let Some(history) = &mut history {
            history.borrow_mut().clear(0);
            // history.clear(0);
        }

        let mut i = 1;
        loop {
            // for (int i = 1;; ++i) {
            // EdgeTracer  tracer(image, startPos, dir);
            let mut tracer = EdgeTracer::new(image, startPos, dir);
            tracer.p +=
                i / 2 * minSymbolSize as i32 * (if (i & 1) != 0 { -1 } else { 1 }) * tracer.right();
            if (tryHarder) {
                // tracer.history = history.as_mut();
                tracer.history = history.clone();
                // if let Some(history) = &history {
                // 	tracer.history = history;
                // }
                // tracer.history = &history;
            }

            if (!tracer.isInSelf()) {
                break;
            }

            // #ifdef __cpp_impl_coroutine
            // 			DetectorResult res;
            // 			while (res = Scan(tracer, lines), res.isValid())
            // 				co_yield std::move(res);
            // #else
            if let Ok(res) = Scan(&mut tracer, &mut lines) {
                // if res.isValid(){
                return Ok(res);
                // }
            }

            // if (auto res = Scan(tracer, lines); res.isValid())
            // 	{return res;}
            // #endif

            if (!tryHarder) {
                break;
            } // only test center lines
            i += 1;
        }

        if (!tryRotate) {
            break;
        } // only test left direction
    }

    // #ifndef __cpp_impl_coroutine
    Err(Exceptions::NotFoundException(None))
    // #endif
}

#[inline(always)]
fn float_min<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        b
    } else {
        a
    }
}

#[inline(always)]
fn float_max<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}
