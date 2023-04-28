use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{BitMatrix, Result},
    qrcode::encoder::ByteMatrix,
    Exceptions, Point,
};

use super::{BitMatrixCursorTrait, Direction, RegressionLineTrait, StepResult, Value};

#[derive(Clone)]
pub struct EdgeTracer<'a> {
    pub(crate) img: &'a BitMatrix,

    pub(crate) p: Point, // current position
    d: Point,            // current direction

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

impl BitMatrixCursorTrait for EdgeTracer<'_> {
    fn testAt(&self, p: Point) -> Value {
        if self.img.isIn(p, 0) {
            Value::from(self.img.get_point(p))
        } else {
            Value::Invalid
        }
    }

    fn isIn(&self, p: Point) -> bool {
        self.img.isIn(p, 0)
    }

    fn isInSelf(&self) -> bool {
        self.isIn(self.p)
    }

    fn isBlack(&self) -> bool {
        self.blackAt(self.p)
    }

    fn isWhite(&self) -> bool {
        self.whiteAt(self.p)
    }

    fn front(&self) -> &Point {
        &self.d
    }

    fn back(&self) -> Point {
        Point {
            x: -self.d.x,
            y: -self.d.y,
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.d.y,
            y: -self.d.x,
        }
    }

    fn right(&self) -> Point {
        Point {
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

    fn edgeAt_point(&self, d: Point) -> Value {
        let v = self.testAt(self.p);
        if self.testAt(self.p + d) != v {
            v
        } else {
            Value::Invalid
        }
    }

    fn setDirection(&mut self, dir: Point) {
        self.d = dir.bresenhamDirection();
    }

    fn step(&mut self, s: Option<f32>) -> bool {
        let s = if let Some(s) = s { s } else { 1.0 };
        self.p += self.d * s;
        self.isIn(self.p)
    }

    fn movedBy<T: BitMatrixCursorTrait>(self, d: Point) -> Self {
        let mut res = self;
        res.p += d;

        res
    }

    fn turnedBack(&self) -> Self {
        let mut res = self.clone();
        res.d = res.back();

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
        let mut nth = nth.unwrap_or(1); //if let Some(nth) = nth { nth } else { 1 };
        let range = range.unwrap_or(0); //if let Some(r) = range { r } else { 0 };
        let backup = backup.unwrap_or(false); //if let Some(b) = backup { b } else { false };
                                              // TODO: provide an alternative and faster out-of-bounds check than isIn() inside testAt()
        let mut steps = 0;
        let mut lv = self.testAt(self.p);

        while nth > 0 && (range <= 0 || steps < range) && lv.isValid() {
            steps += 1;
            let v = self.testAt(self.p + steps * self.d);
            if lv != v {
                lv = v;
                nth -= 1;
            }
        }
        if backup {
            steps -= 1;
        }
        self.p += self.d * steps;
        steps * i32::from(nth == 0)
    }

    fn p(&self) -> Point {
        self.p
    }

    fn d(&self) -> Point {
        self.d
    }

    fn img(&self) -> &BitMatrix {
        self.img
    }
}

impl<'a> EdgeTracer<'_> {
    pub fn new(image: &'a BitMatrix, p: Point, d: Point) -> EdgeTracer<'a> {
        // : img(&image), p(p) { setDirection(d); }
        EdgeTracer {
            img: image,
            p,
            d: Point::bresenhamDirection(d), //d,
            history: None,
            state: 0,
        }
    }

    fn traceStep(
        &mut self,
        dEdge: Point,
        maxStepSize: i32,
        goodDirection: bool,
    ) -> Result<StepResult> {
        let dEdge = Point::mainDirection(dEdge);
        for breadth in 1..=(if maxStepSize == 1 {
            2
        } else if goodDirection {
            1
        } else {
            3
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

                    if !self.blackAt(pEdge + dEdge) {
                        continue;
                    }

                    // found black pixel -> go 'outward' until we hit the b/w border
                    for _j in 0..(std::cmp::max(maxStepSize, 3)) {
                        // for (int j = 0; j < std::max(maxStepSize, 3) && isIn(pEdge); ++j) {
                        if self.whiteAt(pEdge) {
                            // if we are not making any progress, we still have another endless loop bug
                            if self.p == pEdge.centered() {
                                return Err(Exceptions::ILLEGAL_STATE);
                            }
                            self.p = pEdge.centered();

                            // if (self.history && maxStepSize == 1) {
                            if let Some(history) = &self.history {
                                if maxStepSize == 1 {
                                    if history.borrow().get(self.p.x as u32, self.p.y as u32)
                                        == self.state as u8
                                    {
                                        return Ok(StepResult::ClosedEnd);
                                    }
                                    history.borrow_mut().set(
                                        self.p.x as u32,
                                        self.p.y as u32,
                                        self.state as u8,
                                    );
                                }
                            }

                            return Ok(StepResult::Found);
                        }
                        pEdge -= dEdge;
                        if self.blackAt(pEdge - self.d) {
                            pEdge -= self.d;
                        }
                        // dbg!(pEdge);

                        if !self.isIn(pEdge) {
                            break;
                        }
                    }
                    // no valid b/w border found within reasonable range
                    return Ok(StepResult::ClosedEnd);
                }
            }
        }
        Ok(StepResult::OpenEnd)
    }

    pub fn updateDirectionFromOrigin(&mut self, origin: Point) -> bool {
        let old_d = self.d;
        self.setDirection(self.p - origin);
        // if the new direction is pointing "backward", i.e. angle(new, old) > 90 deg -> break
        if Point::dot(self.d, old_d) < 0.0 {
            return false;
        }
        // make sure d stays in the same quadrant to prevent an infinite loop
        if (self.d.x).abs() == (self.d.y).abs() {
            self.d = Point::mainDirection(old_d) + 0.99 * (self.d - Point::mainDirection(old_d));
        } else if Point::mainDirection(self.d) != Point::mainDirection(old_d) {
            self.d = Point::mainDirection(old_d) + 0.99 * Point::mainDirection(self.d);
        }
        true
    }

    pub fn traceLine<T: RegressionLineTrait>(
        &mut self,
        dEdge: Point,
        line: &mut T,
    ) -> Result<bool> {
        line.setDirectionInward(dEdge);
        loop {
            // log(self.p);
            line.add(self.p)?;
            if line.points().len() % 50 == 10 {
                if !line.evaluate_max_distance(None, None) {
                    return Ok(false);
                }
                if !self.updateDirectionFromOrigin(
                    self.p - line.project(self.p)
                        + **line
                            .points()
                            .first()
                            .as_ref()
                            .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                ) {
                    return Ok(false);
                }
            }
            let stepResult = self.traceStep(dEdge, 1, line.isValid())?;
            if stepResult != StepResult::Found {
                return Ok(stepResult == StepResult::OpenEnd && line.points().len() > 1);
            }
        } // while (true);
    }

    pub fn traceGaps<T: RegressionLineTrait>(
        &mut self,
        dEdge: Point,
        line: &mut T,
        maxStepSize: i32,
        finishLine: &mut T,
    ) -> Result<bool> {
        let mut maxStepSize = maxStepSize;
        line.setDirectionInward(dEdge);
        let mut gaps = 0;
        loop {
            // detect an endless loop (lack of progress). if encountered, please report.
            if !(line.points().is_empty()
                || &&self.p
                    != line
                        .points()
                        .last()
                        .as_ref()
                        .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?)
            {
                return Err(Exceptions::ILLEGAL_STATE);
            }
            if !line.points().is_empty()
                && &&self.p
                    == line
                        .points()
                        .last()
                        .as_ref()
                        .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
            {
                return Ok(false);
            }
            // log(p);

            // if we drifted too far outside of the code, break
            if line.isValid()
                && line.signedDistance(self.p) < -5.0
                && (!line.evaluate_max_distance(None, None) || line.signedDistance(self.p) < -5.0)
            {
                return Ok(false);
            }

            // if we are drifting towards the inside of the code, pull the current position back out onto the line
            if line.isValid() && line.signedDistance(self.p) > 3.0 {
                // The current direction d and the line we are tracing are supposed to be roughly parallel.
                // In case the 'go outward' step in traceStep lead us astray, we might end up with a line
                // that is almost perpendicular to d. Then the back-projection below can result in an
                // endless loop. Break if the angle between d and line is greater than 45 deg.
                if (Point::dot(Point::normalized(self.d), line.normal())).abs() > 0.7
                // thresh is approx. sin(45 deg)
                {
                    return Ok(false);
                }

                // re-evaluate line with all the points up to here before projecting
                if !line.evaluate_max_distance(Some(1.5), None) {
                    return Ok(false);
                }

                let mut np = line.project(self.p);
                // make sure we are making progress even when back-projecting:
                // consider a 90deg corner, rotated 45deg. we step away perpendicular from the line and get
                // back projected where we left off the line.
                // The 'while' instead of 'if' was introduced to fix the issue with #245. It turns out that
                // np can actually be behind the projection of the last line point and we need 2 steps in d
                // to prevent a dead lock. see #245.png
                while Point::distance(
                    np,
                    line.project(
                        line.points()
                            .last()
                            .copied()
                            .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                    ),
                ) < 1.0
                {
                    np += self.d;
                }
                self.p = Point::centered(np);
            } else {
                let stepLengthInMainDir = if line.points().is_empty() {
                    0.0
                } else {
                    Point::dot(
                        Point::mainDirection(self.d),
                        self.p
                            - line
                                .points()
                                .last()
                                .copied()
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                    )
                };
                line.add(self.p)?;

                if stepLengthInMainDir > 1.0 {
                    gaps += 1;
                    if gaps >= 2 || line.points().len() > 5 {
                        if !line.evaluate_max_distance(Some(1.5), None) {
                            return Ok(false);
                        }
                        if !self.updateDirectionFromOrigin(
                            self.p - line.project(self.p)
                                + line
                                    .points()
                                    .first()
                                    .copied()
                                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                        ) {
                            return Ok(false);
                        }
                        // check if the first half of the top-line trace is complete.
                        // the minimum code size is 10x10 -> every code has at least 4 gaps
                        //TODO: maybe switch to termination condition based on bottom line length to get a better
                        // finishLine for the right line trace
                        if !finishLine.isValid() && gaps == 4 {
                            // undo the last insert, it will be inserted again after the restart
                            line.pop_back();
                            // gaps -= 1;
                            return Ok(true);
                        }
                    }
                } else if gaps == 0 && line.points().len() >= (2 * maxStepSize) as usize {
                    return Ok(false);
                } // no point in following a line that has no gaps
            }

            if finishLine.isValid() {
                maxStepSize =
                    std::cmp::min(maxStepSize, (finishLine.signedDistance(self.p)) as i32);
            }

            let stepResult = self.traceStep(dEdge, maxStepSize, line.isValid())?;

            if stepResult != StepResult::Found
            // we are successful iff we found an open end across a valid finishLine
            {
                return Ok(stepResult == StepResult::OpenEnd
                    && finishLine.isValid()
                    && (finishLine.signedDistance(self.p)) as i32 <= maxStepSize + 1);
            }
        } //while (true);
    }

    pub fn traceCorner(&mut self, dir: &mut Point, corner: &mut Point) -> Result<bool> {
        self.step(None);
        // log(p);
        *corner = self.p;
        std::mem::swap(&mut self.d, dir);
        self.traceStep(-1.0 * (*dir), 2, false)?;
        // #ifdef PRINT_DEBUG
        // 		printf("turn: %.0f x %.0f -> %.2f, %.2f\n", p.x, p.y, d.x, d.y);
        // #endif
        Ok(self.isIn(*corner) && self.isIn(self.p))
    }
}
