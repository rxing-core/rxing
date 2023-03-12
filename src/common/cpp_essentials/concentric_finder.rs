use crate::{
    common::{
        cpp_essentials::{
            Direction, FixedPattern, IsPattern, PatternRow, PatternType, PatternView,
        },
        BitMatrix, Quadrilateral,
    },
    point, Point,
};

use super::{
    BitMatrixCursor, DMRegressionLine, EdgeTracer, FastEdgeToEdgeCounter, Pattern, RegressionLine,
};

pub fn CenterFromEnd<const N: usize, T: Into<f32> + std::iter::Sum<T> + Copy>(
    pattern: &[T; N],
    end: f32,
) -> f32 {
    if (N == 5) {
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
    } else if (N == 3) {
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

pub fn ReadSymmetricPattern<const N: usize, Cursor: BitMatrixCursor>(
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
        if (range != 0) {
            range -= v;
        }

        v
    };

    for i in 0..=s_2 {
        // for (int i = 0; i <= s_2; ++i) {
        if (!next(cur, i) != 0 || !next(&mut cuo, -i) != 0) {
            return None;
        }
    }
    res[s_2 as usize] -= 1; // the starting pixel has been counted twice, fix this

    Some(res)
}

// default for RELAXED_THRESHOLD should be false
pub fn CheckSymmetricPattern<
    const RELAXED_THRESHOLD: bool,
    const LEN: usize,
    const SUM: usize,
    T: BitMatrixCursor,
>(
    cur: &mut T,
    pattern: &Pattern<LEN>,
    range: i32,
    updatePosition: bool,
) -> i32 {
    let mut range = range;

    let curFwd: FastEdgeToEdgeCounter = FastEdgeToEdgeCounter::new(cur);
    let curBwd: FastEdgeToEdgeCounter = FastEdgeToEdgeCounter::new(&cur.turnedBack());

    let centerFwd = curFwd.stepToNextEdge(range);
    if (!(centerFwd != 0)) {
        return 0;
    }
    let centerBwd = curBwd.stepToNextEdge(range);
    if (!(centerBwd != 0)) {
        return 0;
    }

    assert!(range > 0);
    let mut res: PatternRow = PatternRow::new(vec![0; LEN]);
    let s_2 = (res.len()) / 2;
    res[s_2] = (centerFwd + centerBwd - 1) as u16; // -1 because the starting pixel is counted twice
    range -= res[s_2] as i32;

    let mut next = |cur: &FastEdgeToEdgeCounter, i: isize| {
        let v = cur.stepToNextEdge(range);
        res[(s_2 as isize + i) as usize] = v as u16;
        range -= v;

        v
    };

    for i in 1..=s_2 {
        // for (int i = 1; i <= s_2; ++i) {
        if (!(next(&curFwd, i as isize) != 0) || !(next(&curBwd, -(i as isize)) != 0)) {
            return 0;
        }
    }

    if (!(IsPattern(
        &PatternView::new(&res),
        &FixedPattern::<LEN, SUM, false>::with_reference(pattern),
        None,
        0.0,
        0.0,
        Some(RELAXED_THRESHOLD),
    ) != 0.0))
    {
        return 0;
    }

    if (updatePosition) {
        cur.step(Some((res[s_2] as i32 / 2 - (centerBwd as i32 - 1)) as f32));
    }

    res.into_iter().sum::<PatternType>() as i32
}

pub fn AverageEdgePixels<T: BitMatrixCursor>(
    cur: &mut T,
    range: i32,
    numOfEdges: u32,
) -> Option<Point> {
    let mut sum = Point::default();

    for i in 0..numOfEdges {
        // for (int i = 0; i < numOfEdges; ++i) {
        if (!cur.isInSelf()) {
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
        point(0.0, 1.0),
        point(1.0, 0.0),
        point(1.0, 1.0),
        point(1.0, -1.0),
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
    let mut cur = EdgeTracer::new(image, center, point(0.0, 1.0));
    if (!(cur.stepToEdge(Some(nth), Some(radius), Some(inner)) != 0)) {
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
        neighbourMask |= (1
            << (4.0 + Point::dot(Point::bresenhamDirection(cur.p() - center), point(1.0, 3.0)))
                as u32);

        if (!cur.stepAlongEdge(edgeDir, None)) {
            return None;
        }

        // use L-inf norm, simply because it is a lot faster than L2-norm and sufficiently accurate
        if (Point::maxAbsComponent(cur.p - center) > radius as f32
            || center == cur.p
            || n > 4 * 2 * range)
        {
            return None;
        }

        if !(cur.p != start) {
            break;
        }
    } //while (cur.p != start);

    if (requireCircle && neighbourMask != 0b111101111) {
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
    let mut n = numOfRings;
    let mut sum = numOfRings * center;
    for i in 1..numOfRings {
        // for (int i = 1; i < numOfRings; ++i) {
        let c = CenterOfRing(image, center, range, i as i32 + 1, false)?;

        // TODO: decide whether this wheighting depending on distance to the center is worth it
        let weight = numOfRings - i;
        sum += weight * c;
        n += weight;
    }
    Some(sum / n as f32)
}

pub fn FinetuneConcentricPatternCenter(
    image: &BitMatrix,
    center: Point,
    range: i32,
    finderPatternSize: u32,
) -> Option<Point> {
    // make sure we have at least one path of white around the center
    let res = CenterOfRing(image, center, range, 1, false)?;

    let center = res;

    let mut res = CenterOfRings(image, center, range, finderPatternSize / 2);

    if (res.is_none() || !image.get_point(res?)) {
        res = CenterOfDoubleCross(image, (center), range, finderPatternSize / 2 + 1);
    }
    if (res.is_none() || !image.get_point(res?)) {
        res = Some(center);
    }
    if (res.is_none() || !image.get_point(res?)) {
        return None;
    }

    res
}

pub fn CollectRingPoints(
    image: &BitMatrix,
    center: Point,
    range: i32,
    edgeIndex: i32,
    backup: bool,
) -> Vec<Point> {
    let centerI = center.round();
    let radius = range;
    let mut cur = EdgeTracer::new(image, centerI, point(0.0, 1.0));
    if (!(cur.stepToEdge(Some(edgeIndex), Some(radius), Some(backup)) != 0)) {
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
        points.push((cur.p().centered()));

        // find out if we come full circle around the center. 8 bits have to be set in the end.
        neighbourMask |= (1
            << (4.0 + Point::dot(Point::bresenhamDirection(cur.p - centerI), point(1.0, 3.0)))
                as u32);

        if (!cur.stepAlongEdge(edgeDir, None)) {
            return Vec::default();
        }

        // use L-inf norm, simply because it is a lot faster than L2-norm and sufficiently accurate
        if (Point::maxAbsComponent(cur.p - centerI) > radius as f32
            || centerI == cur.p
            || (points).len() > 4 * 2 * range as usize)
        {
            return Vec::default();
        }

        if !(cur.p != start) {
            break;
        }
    } //while (cur.p != start);

    if (neighbourMask != 0b111101111) {
        return Vec::default();
    }

    points
}

pub fn FitQadrilateralToPoints(center: Point, points: &mut [Point]) -> Option<Quadrilateral> {
    let dist2Center = |a, b| Point::distance(a, center) < Point::distance(b, center);
    // rotate points such that the first one is the furthest away from the center (hence, a corner)

    let max_by_pred = |a: &Point, b: &Point| {
        if dist2Center(*a, *b) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    };

    let max = points.iter().copied().max_by(max_by_pred)?;

    let pos = points.iter().position(|e| *e == max)?;

    points.rotate_left(pos);
    // std::rotate(points.begin(), std::max_element(points.begin(), points.end(), dist2Center), points.end());

    let mut corners = [Point::default(); 4];
    corners[0] = points[0];
    // find the oposite corner by looking for the farthest point near the oposite point
    points[(points.len() * 3 / 8)..=(points.len() * 5 / 8)]
        .iter()
        .copied()
        .max_by(max_by_pred)?;
    // corners[2] = std::max_element(&points[Size(points) * 3 / 8], &points[Size(points) * 5 / 8], dist2Center);
    // find the two in between corners by looking for the points farthest from the long diagonal
    let l = DMRegressionLine::with_two_points(corners[0], corners[2]);
    let dist2Diagonal = /*[l = RegressionLine(*corners[0], *corners[2])]*/| a,  b| {  l.distance_single(a) < l.distance_single(b) };

    let diagonal_max_by_pred = |p1: &Point, p2: &Point| {
        if dist2Diagonal(*p1, *p2) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    };
    corners[1] = points[(points.len() * 1 / 8)..=(points.len() * 3 / 8)]
        .iter()
        .copied()
        .max_by(diagonal_max_by_pred)?;
    // corners[1] = std::max_element(&points[Size(points) * 1 / 8], &points[Size(points) * 3 / 8], dist2Diagonal);
    corners[3] = points[(points.len() * 5 / 8)..=(points.len() * 7 / 8)]
        .iter()
        .copied()
        .max_by(diagonal_max_by_pred)?;
    // corners[3] = std::max_element(&points[Size(points) * 5 / 8], &points[Size(points) * 7 / 8], dist2Diagonal);

    let lines = [
        DMRegressionLine::with_two_points((corners[0] + 1.0), corners[1]),
        DMRegressionLine::with_two_points((corners[1] + 1.0), corners[2]),
        DMRegressionLine::with_two_points((corners[2] + 1.0), corners[3]),
        DMRegressionLine::with_two_points((corners[3] + 1.0), (*points.last()? + 1.0)),
    ];
    // std::array lines{RegressionLine{corners[0] + 1, corners[1]}, RegressionLine{corners[1] + 1, corners[2]},
    // 				 RegressionLine{corners[2] + 1, corners[3]}, RegressionLine{corners[3] + 1, &points.back() + 1}};
    if lines.iter().any(|line| !line.isValid()) {
        return None;
    }

    let mut res = Quadrilateral::default();
    for i in 0..4 {
        // for (int i = 0; i < 4; ++i) {
        res[i] = DMRegressionLine::intersect(&lines[i], &lines[(i + 1) % 4])?;
    }

    Some(res)
}

pub fn QuadrilateralIsPlausibleSquare(q: &Quadrilateral, lineIndex: usize) -> bool {
    let mut m = f64::default();
    // let mut M = f64::default();

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
    if (points.is_empty()) {
        return None;
    }

    let res = FitQadrilateralToPoints(center, &mut points)?;
    if (!QuadrilateralIsPlausibleSquare(&res, (lineIndex - i32::from(backup)) as usize)) {
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

    // for  p in innerCorners{
    // 	log(p, 3);}

    // for  p in outerCorners{
    // 	log(p, 3);}

    // for  p in res{
    // 	log(p, 3);}

    Some(res)
}

#[derive(Default)]
pub struct ConcentricPattern {
    p: Point,
    size: usize,
}

 pub fn LocateConcentricPattern<const RELAXED_THRESHOLD:bool, const LEN: usize,
 const SUM: usize,
 T: BitMatrixCursor>(  image:&BitMatrix,  pattern:&Pattern<LEN>,  center:Point,  range:i32) -> Option<ConcentricPattern>
{
	let mut cur = EdgeTracer::new(image, center, Point::default());
	let mut minSpread = image.getWidth() as i32;
    let mut maxSpread = 0_i32;
    for d in [point(0.0,1.0), point(1.0,0.0)] {
	// for (auto d : {PointI{0, 1}, {1, 0}}) {
        cur.setDirection(d); // THIS COULD POSSIBLY BE WRONG, WE MIGHT MEAN TO CLONE cur EACH RUN?
		let spread = CheckSymmetricPattern(&mut cur, pattern, range, true);
		if (!(spread != 0))
			{return None}
		UpdateMinMax(&mut minSpread, &mut maxSpread, spread);
	}

//#if 1
for d in [point(1.0,1.0), point(1.0,-1.0)] {
	// for (auto d : {PointI{1, 1}, {1, -1}}) {
        cur.setDirection(d);// THIS COULD POSSIBLY BE WRONG, WE MIGHT MEAN TO CLONE cur EACH RUN?
		let spread = CheckSymmetricPattern(&mut cur, pattern, range * 2, false);
		if (!(spread != 0))
			{return None}
		UpdateMinMax(&mut minSpread, &mut maxSpread, spread);
	}
//#endif

	if (maxSpread > 5 * minSpread)
		{return None}

	let newCenter = FinetuneConcentricPatternCenter(image, cur.p(), range, pattern.len() as u32)?;

	 Some(ConcentricPattern{*newCenter, (maxSpread + minSpread) / 2})
}

fn UpdateMinMax<T: Ord + Copy>(min: &mut T, max: &mut T, val: T) {
    *min = std::cmp::min(*min, val);
    *max = std::cmp::max(*max, val);
}

fn UpdateMinMaxFloat(min: &mut f64, max: &mut f64, val: f64) {
    *min = f64::min(*min, val);
    *max = f64::max(*max, val);
}
