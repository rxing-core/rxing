use crate::{common::BitMatrix, Point};

use super::{util::opposite, Direction, Value};

/**
 * @brief The BitMatrixCursor represents a current position inside an image and current direction it can advance towards.
 *
 * The current position and direction is a PointT<T>. So depending on the type it can be used to traverse the image
 * in a Bresenham style (PointF) or in a discrete way (step only horizontal/vertical/diagonal (PointI)).
 */
pub trait BitMatrixCursorTrait {
    // const BitMatrix* img;

    // POINT p; // current position
    // POINT d; // current direction

    // BitMatrixCursor(const BitMatrix& image, POINT p, POINT d) : img(&image), p(p) { setDirection(d); }

    fn testAt(&self, p: Point) -> Value; //const
                                         // {
                                         // 	return img->isIn(p) ? Value{img->get(p)} : Value{};
                                         // }

    fn blackAt(&self, pos: Point) -> bool {
        self.testAt(pos).isBlack()
    }
    fn whiteAt(&self, pos: Point) -> bool {
        self.testAt(pos).isWhite()
    }

    fn isIn(&self, p: Point) -> bool; // { return img->isIn(p); }
    fn isInSelf(&self) -> bool; // { return self.isIn(p); }
    fn isBlack(&self) -> bool; // { return blackAt(p); }
    fn isWhite(&self) -> bool; // { return whiteAt(p); }

    fn front(&self) -> &Point; //{ return d; }
    fn back(&self) -> Point; // { return {-d.x, -d.y}; }
    fn left(&self) -> Point; //{ return {d.y, -d.x}; }
    fn right(&self) -> Point; //{ return {-d.y, d.x}; }
    fn direction(&self, dir: Direction) -> Point {
        self.right() * Into::<i32>::into(dir)
    }

    fn turnBack(&mut self); // noexcept { d = back(); }
    fn turnLeft(&mut self); //noexcept { d = left(); }
    fn turnRight(&mut self); //noexcept { d = right(); }
    fn turn(&mut self, dir: Direction); //noexcept { d = direction(dir); }

    fn edgeAt_point(&self, d: Point) -> Value;
    // {
    // 	Value v = testAt(p);
    // 	return testAt(p + d) != v ? v : Value();
    // }

    fn edgeAtFront(&self) -> Value {
        return self.edgeAt_point(*self.front());
    }
    fn edgeAtBack(&self) -> Value {
        self.edgeAt_point(self.back())
    }
    fn edgeAtLeft(&self) -> Value {
        self.edgeAt_point(self.left())
    }
    fn edgeAtRight(&self) -> Value {
        self.edgeAt_point(self.right())
    }
    fn edgeAt_direction(&self, dir: Direction) -> Value {
        self.edgeAt_point(self.direction(dir))
    }

    fn setDirection(&mut self, dir: Point); // { d = bresenhamDirection(dir); }
                                            // fn setDirection(&self, dir: Point);// { d = dir; }

    fn step(&mut self, s: Option<f32>) -> bool; // DEF to 1
                                                // {
                                                // 	p += s * d;
                                                // 	return isIn(p);
                                                // }

    fn movedBy<T: BitMatrixCursorTrait>(self, d: Point) -> Self;
    fn turnedBack(&self) -> Self; // { return {*img, p, back()}; }
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
        let skipCorner = skipCorner.unwrap_or_default();

        if !self.edgeAt_direction(dir).isValid() {
            self.turn(dir);
        } else if self.edgeAtFront().isValid() {
            self.turn(opposite(dir));
            if self.edgeAtFront().isValid() {
                self.turn(opposite(dir));
                if self.edgeAtFront().isValid() {
                    return false;
                }
            }
        }

        let mut ret = self.step(None);

        if ret && skipCorner && !self.edgeAt_direction(dir).isValid() {
            self.turn(dir);
            ret = self.step(None);
        }

        ret
    }

    fn countEdges(&mut self, range: i32) -> i32 {
        let mut res = 0;
        let mut range = range;

        let mut steps;

        while {
            steps = if range == 0 {
                0
            } else {
                self.stepToEdge(Some(1), Some(range), None)
            };
            steps > 0
        } {
            range -= steps;
            res += 1;
        }

        res
    }

    fn p(&self) -> Point;

    fn d(&self) -> Point;

    fn img(&self) -> &BitMatrix;

    fn readPattern<const LEN: usize, T: TryFrom<i32> + Default + Copy + Clone>(
        &mut self,
        range: Option<i32>,
    ) -> Option<[T; LEN]> {
        let range = range.unwrap_or(0);
        let mut res = [T::default(); LEN];
        for i in res.iter_mut() {
            *i = self
                .stepToEdge(Some(1), Some(range), None)
                .try_into()
                .ok()?;
        }
        Some(res)
    }

    fn readPatternFromBlack<const LEN: usize, T: TryFrom<i32> + Default + Copy + Clone>(
        &mut self,
        maxWhitePrefix: i32,
        range: Option<i32>,
    ) -> Option<[T; LEN]> {
        let range = range.unwrap_or(0);
        if maxWhitePrefix != 0
            && self.isWhite()
            && !self.stepToEdge(Some(1), Some(maxWhitePrefix), None) > 0
        {
            return None;
        }
        // return readPattern<ARRAY>(range);
        self.readPattern::<LEN, _>(Some(range))
    }
}
