use crate::{Exceptions, RXingResultPoint};

use super::{
    util::{float_max, float_min},
    RegressionLine,
};

#[derive(Clone)]
pub struct DMRegressionLine {
    points: Vec<RXingResultPoint>,
    direction_inward: RXingResultPoint,
    pub(super) a: f32,
    pub(super) b: f32,
    pub(super) c: f32,
    // std::vector<PointF> _points;
    // PointF _directionInward;
    // PointF::value_t a = NAN, b = NAN, c = NAN;
}

impl Default for DMRegressionLine {
    fn default() -> Self {
        Self {
            points: Default::default(),
            direction_inward: Default::default(),
            a: f32::NAN,
            b: f32::NAN,
            c: f32::NAN,
        }
    }
}

impl RegressionLine for DMRegressionLine {
    fn points(&self) -> &[RXingResultPoint] {
        &self.points
    }

    fn length(&self) -> u32 {
        if self.points.len() >= 2 {
            RXingResultPoint::distance(*self.points.first().unwrap(), *self.points.last().unwrap())
                as u32
        } else {
            0
        }
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

    fn add(&mut self, p: &RXingResultPoint) -> Result<(), Exceptions> {
        if self.direction_inward == RXingResultPoint::default() {
            return Err(Exceptions::illegalState);
        }
        self.points.push(*p);
        if self.points.len() == 1 {
            self.c = RXingResultPoint::dot(self.normal(), *p);
        }
        Ok(())
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
        if maxSignedDist > 0.0 {
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
                if old_points_size == points.len() {
                    break;
                }
                // #ifdef PRINT_DEBUG
                // 				printf("removed %zu points\n", old_points_size - points.size());
                // #endif
                ret = self.evaluate(&points);
            }

            if updatePoints {
                self.points = points;
            }
        }
        ret
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
        steps > 2.0 || len > 50.0
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
        if sumYY >= sumXX {
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
        RXingResultPoint::dot(self.direction_inward, self.normal()) > 0.5
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
        if sumYY >= sumXX {
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
        RXingResultPoint::dot(self.direction_inward, self.normal()) > 0.5
        // angle between original and new direction is at most 60 degree
    }
}

impl DMRegressionLine {
    // template <typename Container, typename Filter>
    fn average<T>(c: &[f64], f: T) -> f64
    where
        T: Fn(f64) -> bool,
    {
        let mut sum: f64 = 0.0;
        let mut num = 0;
        for v in c {
            // for (const auto& v : c)
            if f(*v) {
                sum += *v;
                num += 1;
            }
        }
        sum / num as f64
    }

    pub fn reverse(&mut self) {
        self.points.reverse();
    }

    pub fn modules(
        &mut self,
        beg: &RXingResultPoint,
        end: &RXingResultPoint,
    ) -> Result<f64, Exceptions> {
        if self.points.len() <= 3 {
            return Err(Exceptions::illegalState);
        }

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
            &(*self.points.last().ok_or(Exceptions::indexOutOfBounds)?
                - *self.points.first().ok_or(Exceptions::indexOutOfBounds)?),
        )) as f64;

        // calculate the width of 2 modules (first black pixel to first black pixel)
        let mut sumFront: f64 =
            self.distance(beg, &self.project(&self.points[0])) as f64 - unitPixelDist;
        let mut sumBack: f64 = 0.0; // (last black pixel to last black pixel)
        for dist in gapSizes {
            // for (auto dist : gapSizes) {
            if dist > 1.9 * unitPixelDist {
                modSizes.push(std::mem::take(&mut sumBack));
            }
            sumFront += dist;
            sumBack += dist;
            if dist > 1.9 * unitPixelDist {
                modSizes.push(std::mem::take(&mut sumFront));
            }
        }

        modSizes.push(
            sumFront
                + self.distance(
                    end,
                    &self.project(self.points.last().ok_or(Exceptions::indexOutOfBounds)?),
                ) as f64,
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
        Ok(lineLength / meanModSize)
    }
}
