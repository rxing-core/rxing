use crate::{
    common::{
        cpp_essentials::{
            Direction, FixedPattern, IsPattern, PatternRow, PatternType, PatternView,
        },
        BitMatrix, Quadrilateral,
    },
    point_f, Point,
};

use super::{
    BitMatrixCursorTrait, EdgeTracer, FastEdgeToEdgeCounter, Pattern, RegressionLine,
    RegressionLineTrait, UpdateMinMax, UpdateMinMaxFloat,
};

pub fn CenterFromEnd<const N: usize, T: Into<f32> + std::iter::Sum<T> + Copy>(
    pattern: &[T; N],
    end: f32,
) -> f32 {
    if N == 5 {
        let a: f32 = pattern[4].into() + pattern[3].into() + pattern[2].into() / 2.0;
        let b: f32 =
            pattern[4].into() + (pattern[3].into() + pattern[2].into() + pattern[1].into()) / 2.0;
        let c: f32 = (pattern[4].into()
            + pattern[3].into()
            + pattern[2].into()
            + pattern[1].into()
            + pattern[0].into())
            / 2.0;
        end - (2.0 * a + b + c) / 4.0
    } else if N == 3 {
        let a: f32 = pattern[2].into() + pattern[1].into() / 2.0;
        let b: f32 = (pattern[2].into() + pattern[1].into() + pattern[0].into()) / 2.0;
        end - (2.0 * a + b) / 3.0
    } else {
        // aztec
        let a: f32 =
            pattern.iter().skip(N / 2 + 1).copied().sum::<T>().into() + pattern[N / 2].into() / 2.0;
        // let a = std::accumulate(pattern.begin() + (N/2 + 1), pattern.end(), pattern[N/2] / 2.0);
        end - a
    }
}

pub fn ReadSymmetricPattern<const N: usize, Cursor: BitMatrixCursorTrait>(
    cur: &mut Cursor,
    range: i32,
) -> Option<Pattern<N>> {
    assert!(N % 2 == 1);

    assert!(range > 0);

    let mut range = range;

    let mut res: Pattern<N> = [0; N];
    let s_2 = res.len() as isize / 2;
    let mut cuo = cur.turnedBack();

    let mut next = |cur: &mut Cursor, i: isize| {
        let v = cur.stepToEdge(Some(1), Some(range), None);
        res[(s_2 + i) as usize] = (res[(s_2 + i) as usize] as i32 + v) as u16;
        // res[(s_2 + i) as usize] += v;
        if range != 0 {
            range -= v;
        }

        v
    };

    for i in 0..=s_2 {
        // for (int i = 0; i <= s_2; ++i) {
        if !next(cur, i) == 0 || !next(&mut cuo, -i) == 0 {
            return None;
        }
    }
    res[s_2 as usize] -= 1; // the starting pixel has been counted twice, fix this

    Some(res)
}

// default for RELAXED_THRESHOLD should be false
pub fn CheckSymmetricPattern<
    const E2E: bool,
    const LEN: usize,
    const SUM: usize,
    T: BitMatrixCursorTrait,
>(
    cur: &mut T,
    pattern: &Pattern<LEN>,
    range: i32,
    updatePosition: bool,
) -> i32 {
    let mut range = range;

    let mut curFwd: FastEdgeToEdgeCounter = FastEdgeToEdgeCounter::new(cur);
    let binding = cur.turnedBack();
    let mut curBwd: FastEdgeToEdgeCounter = FastEdgeToEdgeCounter::new(&binding);

    let centerFwd = curFwd.stepToNextEdge(range as u32) as i32;
    if centerFwd == 0 {
        return 0;
    }
    let centerBwd = curBwd.stepToNextEdge(range as u32) as i32;
    if centerBwd == 0 {
        return 0;
    }

    assert!(range > 0);
    let mut res: PatternRow = PatternRow::new(vec![0; LEN]);
    let s_2 = (res.len()) / 2;
    res[s_2] = (centerFwd + centerBwd - 1) as u16; // -1 because the starting pixel is counted twice
    range -= res[s_2] as i32;

    let mut next = |cur: &mut FastEdgeToEdgeCounter, i: isize| {
        let v = cur.stepToNextEdge(range as u32) as i32;
        res[(s_2 as isize + i) as usize] = v as u16;
        range -= v;

        v
    };

    for i in 1..=s_2 {
        // for (int i = 1; i <= s_2; ++i) {
        if next(&mut curFwd, i as isize) == 0 || next(&mut curBwd, -(i as isize)) == 0 {
            return 0;
        }
    }

    if IsPattern::<E2E, LEN, SUM, false>(
        &PatternView::new(&res),
        &FixedPattern::<LEN, SUM, false>::with_reference(pattern),
        None,
        0.0,
        0.0,
    ) == 0.0
    {
        return 0;
    }

    if updatePosition {
        cur.step(Some((res[s_2] as i32 / 2 - (centerBwd - 1)) as f32));
    }

    res.into_iter().sum::<PatternType>() as i32
}

pub fn AverageEdgePixels<T: BitMatrixCursorTrait>(
    cur: &mut T,
    range: i32,
    numOfEdges: u32,
) -> Option<Point> {
    let mut sum = Point::default();

    for _i in 0..numOfEdges {
        // for (int i = 0; i < numOfEdges; ++i) {
        if !cur.isInSelf() {
            return None;
        }
        cur.stepToEdge(Some(1), Some(range), None);
        sum += cur.p().centered() + (cur.p() + cur.back()).centered()
        // sum += centered(cur.p) + centered(cur.p + cur.back());
        // log(cur.p + cur.back(), 2);
    }
    Some(sum / (2 * numOfEdges) as f32)
}

pub fn CenterOfDoubleCross(
    image: &BitMatrix,
    center: Point,
    range: i32,
    numOfEdges: u32,
) -> Option<Point> {
    let mut sum = Point::default();

    for d in [
        point_f(0.0, 1.0),
        point_f(1.0, 0.0),
        point_f(1.0, 1.0),
        point_f(1.0, -1.0),
    ] {
        // for (auto d : {PointI{0, 1}, {1, 0}, {1, 1}, {1, -1}}) {
        let avr1 = AverageEdgePixels(&mut EdgeTracer::new(image, center, d), range, numOfEdges)?;
        let avr2 = AverageEdgePixels(&mut EdgeTracer::new(image, center, -d), range, numOfEdges)?;

        sum += avr1 + avr2;
    }
    Some(sum / 8.0)
}

pub fn CenterOfRing(
    image: &BitMatrix,
    center: Point,
    range: i32,
    nth: i32,
    requireCircle: bool,
) -> Option<Point> {
    // range is the approximate width/height of the nth ring, if nth>1 then it would be plausible to limit the search radius
    // to approximately range / 2 * sqrt(2) == range * 0.75 but it turned out to be too limiting with realworld/noisy data.
    let radius = range;
    let inner = nth < 0;
    let nth = nth.abs();
    // log(center, 3);
    let mut cur = EdgeTracer::new(image, center, point_f(0.0, 1.0));
    if cur.stepToEdge(Some(nth), Some(radius), Some(inner)) == 0 {
        return None;
    }
    cur.turnRight(); // move clock wise and keep edge on the right/left depending on backup
    let edgeDir = if inner {
        Direction::Left
    } else {
        Direction::Right
    };

    let mut neighbourMask = 0;
    let start = cur.p();
    let mut sum = Point::default();
    let mut n = 0;
    loop {
        // log(cur.p, 4);
        sum += cur.p().centered();
        n += 1;

        // find out if we come full circle around the center. 8 bits have to be set in the end.
        neighbourMask |= 1
            << (4.0
                + Point::dot(
                    Point::floor(Point::bresenhamDirection(cur.p() - center)),
                    point_f(1.0, 3.0),
                )) as u32;

        if !cur.stepAlongEdge(edgeDir, None) {
            return None;
        }

        // use L-inf norm, simply because it is a lot faster than L2-norm and sufficiently accurate
        if Point::maxAbsComponent(cur.p - center) > radius as f32
            || center == cur.p
            || n > 4 * 2 * range
        {
            return None;
        }

        if !(cur.p != start) {
            break;
        }
    } //while (cur.p != start);

    if requireCircle && neighbourMask != 0b111101111 {
        return None;
    }

    Some(sum / n as f32)
}

pub fn CenterOfRings(
    image: &BitMatrix,
    center: Point,
    range: i32,
    numOfRings: u32,
) -> Option<Point> {
    let mut n = 1;
    let mut sum = center;
    for i in 2..(numOfRings + 1) {
        // for (int i = 1; i < numOfRings; ++i) {
        let c = CenterOfRing(image, center.floor(), range, i as i32, true)?;

        if c == Point::default() {
            if n == 1 {
                return None;
            } else {
                return Some(sum / n as f32);
            }
        } else if Point::distance(c, center) > range as f32 / numOfRings as f32 / 2.0 {
            return None;
        }

        sum += c;
        n += 1;
    }
    Some(sum / n as f32)
}

pub fn CollectRingPoints(
    image: &BitMatrix,
    center: Point,
    range: i32,
    edgeIndex: i32,
    backup: bool,
) -> Vec<Point> {
    let centerI = center.floor();
    let radius = range;
    let mut cur = EdgeTracer::new(image, centerI, point_f(0.0, 1.0));
    if cur.stepToEdge(Some(edgeIndex), Some(radius), Some(backup)) == 0 {
        return Vec::default();
    }
    cur.turnRight(); // move clock wise and keep edge on the right/left depending on backup
    let edgeDir = if backup {
        Direction::Left
    } else {
        Direction::Right
    };

    let mut neighbourMask = 0;
    let start = cur.p();
    let mut points = Vec::<Point>::with_capacity(4 * range as usize);

    loop {
        // log(cur.p, 4);
        points.push(cur.p().centered());

        // find out if we come full circle around the center. 8 bits have to be set in the end.
        neighbourMask |= 1
            << (4.0
                + Point::dot(
                    Point::round(Point::bresenhamDirection(cur.p - centerI)),
                    point_f(1.0, 3.0),
                )) as u32;

        if !cur.stepAlongEdge(edgeDir, None) {
            return Vec::default();
        }

        // use L-inf norm, simply because it is a lot faster than L2-norm and sufficiently accurate
        if Point::maxAbsComponent(cur.p - centerI) > radius as f32
            || centerI == cur.p
            || (points).len() > 4 * 2 * range as usize
        {
            return Vec::default();
        }

        if !(cur.p != start) {
            break;
        }
    } //while (cur.p != start);

    if neighbourMask != 0b111101111 {
        return Vec::default();
    }

    points
}

pub fn FitQadrilateralToPoints(center: Point, points: &mut [Point]) -> Option<Quadrilateral> {
    // rotate points such that the first one is the furthest away from the center (hence, a corner)
    let max_by_pred = |a: &&Point, b: &&Point| {
        let da = Point::distance(**a, center);
        let db = Point::distance(**b, center);
        da.partial_cmp(&db).unwrap()
    };

    let max = points.iter().max_by(max_by_pred)?;

    let pos = points.iter().position(|e| e == max)?;

    points.rotate_left(pos);

    let mut corners = [Point::default(); 4];
    corners[0] = points[0];
    // find the oposite corner by looking for the farthest point near the oposite point
    corners[2] = *points[(points.len() * 3 / 8)..=(points.len() * 5 / 8)]
        .iter()
        .max_by(max_by_pred)?;
    // corners[2] = std::max_element(&points[Size(points) * 3 / 8], &points[Size(points) * 5 / 8], dist2Center);
    // find the two in between corners by looking for the points farthest from the long diagonal
    let l = RegressionLine::with_two_points(corners[0], corners[2]);

    let diagonal_max_by_pred = |p1: &Point, p2: &Point| {
        let d1 = l.distance_single(*p1);
        let d2 = l.distance_single(*p2);
        d1.partial_cmp(&d2).unwrap()
    };
    corners[1] = points[(points.len() / 8)..=(points.len() * 3 / 8)]
        .iter()
        .copied()
        .max_by(diagonal_max_by_pred)?;
    // corners[1] = std::max_element(&points[Size(points) * 1 / 8], &points[Size(points) * 3 / 8], dist2Diagonal);
    corners[3] = points[(points.len() * 5 / 8)..=(points.len() * 7 / 8)]
        .iter()
        .copied()
        .max_by(diagonal_max_by_pred)?;
    // corners[3] = std::max_element(&points[Size(points) * 5 / 8], &points[Size(points) * 7 / 8], dist2Diagonal);

    let corner_positions = [
        0,
        points.iter().position(|p| *p == corners[1])?,
        points.iter().position(|p| *p == corners[2])?,
        points.iter().position(|p| *p == corners[3])?,
    ];

    let try_get_range = |a: usize, b: usize| -> Option<&[Point]> {
        if a > b {
            None
        }
        // Added for Issue #36 where array is sometimes out of bounds
        else if a + 1 >= points.len() || b >= points.len() {
            if a + 1 >= points.len() {
                None
            } else {
                Some(&points[a..])
            }
        }
        // Added for Issue #36 where a sometimes equals b
        else if a == b {
            Some(&points[a..b])
        } else {
            Some(&points[a + 1..b])
        }
    };

    let lines = [
        RegressionLine::with_point_slice(try_get_range(corner_positions[0], corner_positions[1])?),
        RegressionLine::with_point_slice(try_get_range(corner_positions[1], corner_positions[2])?),
        RegressionLine::with_point_slice(try_get_range(corner_positions[2], corner_positions[3])?),
        RegressionLine::with_point_slice(try_get_range(corner_positions[3], points.len())?),
    ];

    if lines.iter().any(|line| !line.isValid()) {
        return None;
    }

    let beg: [usize; 4] = [
        corner_positions[0] + 1,
        corner_positions[1] + 1,
        corner_positions[2] + 1,
        corner_positions[3] + 1,
    ];
    let end: [usize; 4] = [
        corner_positions[1],
        corner_positions[2],
        corner_positions[3],
        points.len(),
    ];

    // check if all points belonging to each line segment are sufficiently close to that line
    for i in 0..4 {
        // for (int i = 0; i < 4; ++i){
        for p in &points[beg[i]..end[i]] {
            // for (const PointF* p = beg[i]; p != end[i]; ++p) {
            let len = (end[i] - beg[i]) as f64; //std::distance(beg[i], end[i]);
            if len > 3.0 && (lines[i].distance_single(*p) as f64) > (len / 8.0).clamp(1.0, 8.0) {
                // #ifdef PRINT_DEBUG
                // 				printf("%d: %.2f > %.2f @ %.fx%.f\n", i, lines[i].distance(*p), std::distance(beg[i], end[i]) / 1., p->x, p->y);
                // #endif
                return None;
            }
        }
    }

    let mut res = Quadrilateral::default();
    for i in 0..4 {
        // for (int i = 0; i < 4; ++i) {
        res[i] = RegressionLine::intersect(&lines[i], &lines[(i + 1) % 4])?;
    }

    Some(res)
}

pub fn QuadrilateralIsPlausibleSquare(q: &Quadrilateral, lineIndex: usize) -> bool {
    let mut m;

    m = Point::distance(q[0], q[3]) as f64; //M = distance(q[0], q[3]);
    let mut M = m;

    for i in 1..4 {
        // for (int i = 1; i < 4; ++i)

        UpdateMinMaxFloat(&mut m, &mut M, Point::distance(q[i - 1], q[i]) as f64);
    }

    m >= (lineIndex * 2) as f64 && m > M / 3.0
}

pub fn FitSquareToPoints(
    image: &BitMatrix,
    center: Point,
    range: i32,
    lineIndex: i32,
    backup: bool,
) -> Option<Quadrilateral> {
    let mut points = CollectRingPoints(image, center, range, lineIndex, backup);
    if points.is_empty() {
        return None;
    }

    let res = FitQadrilateralToPoints(center, &mut points)?;
    if !QuadrilateralIsPlausibleSquare(&res, (lineIndex - i32::from(backup)) as usize) {
        return None;
    }

    Some(res)
}

pub fn FindConcentricPatternCorners(
    image: &BitMatrix,
    center: Point,
    range: i32,
    lineIndex: i32,
) -> Option<Quadrilateral> {
    let innerCorners = FitSquareToPoints(image, center, range, lineIndex, false)?;

    let outerCorners = FitSquareToPoints(image, center, range, lineIndex + 1, true)?;

    let res = Quadrilateral::blend(&innerCorners, &outerCorners);

    Some(res)
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct ConcentricPattern {
    pub p: Point,
    pub size: i32,
}

impl std::ops::Sub for ConcentricPattern {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_p = self.p - rhs.p;
        Self {
            p: new_p,
            size: self.size,
        }
    }
}

impl std::ops::Add for ConcentricPattern {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_p = self.p + rhs.p;
        Self {
            p: new_p,
            size: self.size,
        }
    }
}

impl From<Point> for ConcentricPattern {
    fn from(value: Point) -> Self {
        Self { p: value, size: 0 }
    }
}

impl ConcentricPattern {
    pub fn dot(self, other: ConcentricPattern) -> f32 {
        Point::dot(self.p, other.p)
    }

    pub fn cross(self, other: ConcentricPattern) -> f32 {
        Point::cross(self.p, other.p)
    }

    pub fn distance(self, other: ConcentricPattern) -> f32 {
        Point::distance(self.p, other.p)
    }
}

pub fn LocateConcentricPattern<const E2E: bool, const LEN: usize, const SUM: usize>(
    image: &BitMatrix,
    pattern: &Pattern<LEN>,
    center: Point,
    range: i32,
) -> Option<ConcentricPattern> {
    let mut cur = EdgeTracer::new(image, center.floor(), Point::default());
    let mut minSpread = image.getWidth() as i32;
    let mut maxSpread = 0_i32;

    // TODO: setting maxError to 1 can subtantially help with detecting symbols with low print quality resulting in damaged
    // finder patterns, but it sutantially increases the runtime (approx. 20% slower for the falsepositive images).
    let mut maxError = 0;
    for d in [point_f(0.0, 1.0), point_f(1.0, 0.0)] {
        // for (auto d : {PointI{0, 1}, {1, 0}}) {
        cur.setDirection(d); // THIS COULD POSSIBLY BE WRONG, WE MIGHT MEAN TO CLONE cur EACH RUN?

        let spread = CheckSymmetricPattern::<E2E, LEN, SUM, _>(&mut cur, pattern, range, true);
        if spread != 0 {
            UpdateMinMax(&mut minSpread, &mut maxSpread, spread);
        } else {
            maxError -= 1;
            if maxError < 0 {
                return None;
            }
        }
    }

    //#if 1
    for d in [point_f(1.0, 1.0), point_f(1.0, -1.0)] {
        // for (auto d : {PointI{1, 1}, {1, -1}}) {
        cur.setDirection(d); // THIS COULD POSSIBLY BE WRONG, WE MIGHT MEAN TO CLONE cur EACH RUN?
        let spread = CheckSymmetricPattern::<E2E, LEN, SUM, _>(&mut cur, pattern, range * 2, false);
        if spread != 0 {
            UpdateMinMax(&mut minSpread, &mut maxSpread, spread);
        } else {
            maxError -= 1;
            if maxError < 0 {
                return None;
            }
        }
    }
    //#endif

    if maxSpread > 5 * minSpread {
        return None;
    }

    let newCenter = FinetuneConcentricPatternCenter(image, cur.p(), range, pattern.len() as u32)?;

    Some(ConcentricPattern {
        p: newCenter,
        size: (maxSpread + minSpread) / 2,
    })
}

pub fn FinetuneConcentricPatternCenter(
    image: &BitMatrix,
    center: Point,
    range: i32,
    finderPatternSize: u32,
) -> Option<Point> {
    // make sure we have at least one path of white around the center
    if let Some(res1) = CenterOfRing(image, center.floor(), range, 1, true) {
        if !image.get_point(res1) {
            return None;
        }
        // and then either at least one more ring around that
        if let Some(res2) = CenterOfRings(image, res1, range, finderPatternSize / 2) {
            return if image.get_point(res2) {
                Some(res2)
            } else {
                None
            };
        }
        // or the center can be approximated by a square
        if FitSquareToPoints(image, res1, range, 1, false).is_some() {
            return Some(res1);
        }
        // TODO: this is currently only keeping #258 alive, evaluate if still worth it
        if let Some(res2) =
            CenterOfDoubleCross(image, res1.floor(), range, finderPatternSize / 2 + 1)
        {
            return if image.get_point(res2) {
                Some(res2)
            } else {
                None
            };
        }
    }
    None
}
