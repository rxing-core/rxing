use multimap::MultiMap;

use crate::{
    common::{
        cpp_essentials::{
            BitMatrixCursorTrait, ConcentricPattern, Direction, EdgeTracer, FindLeftGuard,
            FixedPattern, GetPatternRow, GetPatternRowTP, IsPattern, LocateConcentricPattern,
            PatternRow, PatternType, PatternView, ReadSymmetricPattern, RegressionLine,
            RegressionLineTrait,
        },
        BitMatrix,
    },
    point, Point,
};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct FinderPatternSet {
    bl: ConcentricPattern,
    tl: ConcentricPattern,
    tr: ConcentricPattern,
}

pub type FinderPatterns = Vec<ConcentricPattern>;
pub type FinderPatternSets = Vec<FinderPatternSet>;

const PATTERN: FixedPattern<5, 7, false> = FixedPattern::new([1, 1, 3, 1, 1]);

pub fn FindFinderPatterns(image: &BitMatrix, tryHarder: bool) -> FinderPatterns {
    const MIN_SKIP: u32 = 3; // 1 pixel/module times 3 modules/center
    const MAX_MODULES_FAST: u32 = 20 * 4 + 17; // support up to version 20 for mobile clients

    // Let's assume that the maximum version QR Code we support takes up 1/4 the height of the
    // image, and then account for the center being 3 modules in size. This gives the smallest
    // number of pixels the center could be, so skip this often. When trying harder, look for all
    // QR versions regardless of how dense they are.
    let height = image.height();
    let mut skip = (3 * height) / (4 * MAX_MODULES_FAST);
    if (skip < MIN_SKIP || tryHarder) {
        skip = MIN_SKIP;
    }

    let mut res: Vec<ConcentricPattern> = Vec::new();
    let mut y = skip - 1;

    while y < height {
        // for (int y = skip - 1; y < height; y += skip) {
        let mut row = PatternRow::default();
        GetPatternRowTP(image, y, &mut row, false);
        let mut next: PatternView = PatternView::new(&row);

        while {
            let next = FindLeftGuard(&next, 0, &PATTERN, 0.5).unwrap();
            next.isValid()
        } {
            let p = point(
                next.pixelsInFront() as f32
                    + next[0] as f32
                    + next[1] as f32
                    + next[2] as f32 / 2.0,
                y as f32 + 0.5,
            );

            // make sure p is not 'inside' an already found pattern area
            if res
                .iter()
                .find(|old| Point::distance(p, old.p) < (old.size as f32) / 2.0)
                .is_none()
            {
                // if (FindIf(res, [p](const auto& old) { return distance(p, old) < old.size / 2; }) == res.end()) {
                let pattern = LocateConcentricPattern::<false, 5, 7>(
                    image,
                    &PATTERN.into(),
                    p,
                    next.sum::<u16>() as i32 * 3,
                ); // 3 for very skewed samples
                   //    Reduce(next) * 3); // 3 for very skewed samples
                if (pattern.is_some()) {
                    // log(*pattern, 3);
                    assert!(image.get_point(pattern.as_ref().unwrap().p));
                    res.push(pattern.unwrap());
                }
            }

            next.skipPair();
            next.skipPair();
            next.extend();
        }

        y += skip;
    }

    res
}

/**
 * @brief GenerateFinderPatternSets
 * @param patterns list of ConcentricPattern objects, i.e. found finder pattern squares
 * @return list of plausible finder pattern sets, sorted by decreasing plausibility
 */
pub fn GenerateFinderPatternSets(patterns: &mut FinderPatterns) -> FinderPatternSets {
    patterns.sort_by_key(|p| p.size);
    // std::sort(patterns.begin(), patterns.end(), [](const auto& a, const auto& b) { return a.size < b.size; });

    let mut sets: MultiMap<String, FinderPatternSet> = MultiMap::new();
    let squaredDistance = |a: ConcentricPattern, b: ConcentricPattern| {
        // The scaling of the distance by the b/a size ratio is a very coarse compensation for the shortening effect of
        // the camera projection on slanted symbols. The fact that the size of the finder pattern is proportional to the
        // distance from the camera is used here. This approximation only works if a < b < 2*a (see below).
        // Test image: fix-finderpattern-order.jpg
        ConcentricPattern::dot((a - b), (a - b)) as f64
            * (((b).size as f64) / ((a).size as f64)).powi(2) //std::pow(double(b.size) / a.size, 2)
    };

    let cosUpper: f64 = (45.0_f64 / 180.0 * 3.1415).cos(); // TODO: use c++20 std::numbers::pi_v
    let cosLower: f64 = (135.0_f64 / 180.0 * 3.1415).cos();

    let nbPatterns = (patterns).len();
    for i in 0..(nbPatterns - 2) {
        // for (int i = 0; i < nbPatterns - 2; i++) {
        for j in (i + 1)..(nbPatterns - 1) {
            // for (int j = i + 1; j < nbPatterns - 1; j++) {
            for k in (j + 1)..(nbPatterns - 0) {
                // for (int k = j + 1; k < nbPatterns - 0; k++) {
                let mut a = &patterns[i];
                let mut b = &patterns[j];
                let mut c = &patterns[k];
                // if the pattern sizes are too different to be part of the same symbol, skip this
                // and the rest of the innermost loop (sorted list)
                if (c.size > a.size * 2) {
                    break;
                }

                // Orders the three points in an order [A,B,C] such that AB is less than AC
                // and BC is less than AC, and the angle between BC and BA is less than 180 degrees.

                let mut distAB2 = squaredDistance(*a, *b);
                let mut distBC2 = squaredDistance(*b, *c);
                let mut distAC2 = squaredDistance(*a, *c);

                if (distBC2 >= distAB2 && distBC2 >= distAC2) {
                    std::mem::swap(&mut a, &mut b);
                    std::mem::swap(&mut distBC2, &mut distAC2);
                } else if (distAB2 >= distAC2 && distAB2 >= distBC2) {
                    std::mem::swap(&mut b, &mut c);
                    std::mem::swap(&mut distAB2, &mut distAC2);
                }

                let distAB = (distAB2.sqrt());
                let distBC = (distBC2).sqrt();

                // Make sure distAB and distBC don't differ more than reasonable
                // TODO: make sure the constant 2 is not to conservative for reasonably tilted symbols
                if (distAB > 2.0 * distBC || distBC > 2.0 * distAB) {
                    continue;
                }

                // Estimate the module count and ignore this set if it can not result in a valid decoding
                let moduleCount = (distAB + distBC)
                    / (2.0 * (a.size + b.size + c.size) as f64 / (3.0 * 7.0))
                    + 7.0;
                if (moduleCount < 21.0 * 0.9 || moduleCount > 177.0 * 1.5)
                // moduleCount may be overestimated, see above
                {
                    continue;
                }

                // Make sure the angle between AB and BC does not deviate from 90° by more than 45°
                let cosAB_BC = (distAB2 + distBC2 - distAC2) / (2.0 * distAB * distBC);
                if ((cosAB_BC.is_nan()) || cosAB_BC > cosUpper || cosAB_BC < cosLower) {
                    continue;
                }

                // a^2 + b^2 = c^2 (Pythagorean theorem), and a = b (isosceles triangle).
                // Since any right triangle satisfies the formula c^2 - b^2 - a^2 = 0,
                // we need to check both two equal sides separately.
                // The value of |c^2 - 2 * b^2| + |c^2 - 2 * a^2| increases as dissimilarity
                // from isosceles right triangle.
                let d: f64 = ((distAC2 - 2.0 * distAB2).abs() + (distAC2 - 2.0 * distBC2).abs());

                // Use cross product to figure out whether A and C are correct or flipped.
                // This asks whether BC x BA has a positive z component, which is the arrangement
                // we want for A, B, C. If it's negative then swap A and C.
                if (ConcentricPattern::cross(*c - *b, *a - *b) < 0.0) {
                    std::mem::swap(&mut a, &mut c);
                }

                // arbitrarily limit the number of potential sets
                // (this has performance implications while limiting the maximal number of detected symbols)
                sets.insert(
                    d.to_string(),
                    FinderPatternSet {
                        bl: *a,
                        tl: *b,
                        tr: *c,
                    },
                );
                // const setSizeLimit : usize = 256;
                // if (sets.len() < setSizeLimit || sets.crbegin().first > d) {
                // 	sets.emplace(d, FinderPatternSet{a, b, c});
                // 	if (sets.len() > setSizeLimit)
                // 		{sets.erase(std::prev(sets.end()));}
                // }
            }
        }
    }

    // convert from multimap to vector
    let mut res: FinderPatternSets = Vec::with_capacity(sets.len());

    for (k, v) in sets {
        // for (auto& [d, s] : sets)
        res.extend(v);
    }

    res
}

pub fn EstimateModuleSize(image: &BitMatrix, a: ConcentricPattern, b: ConcentricPattern) -> f64 {
    let mut cur = EdgeTracer::new(image, a.p, b.p - a.p);
    assert!(cur.isBlack());

    let pattern = ReadSymmetricPattern::<5, _>(&mut cur, a.size * 2);

    if pattern.is_none() {
        return -1.0;
    }

    let pattern = pattern.unwrap();

    if (!(IsPattern(
        &PatternView::new(&PatternRow::new(pattern.to_vec())),
        &PATTERN,
        None,
        0.0,
        0.0,
        Some(true),
    ) != 0.0))
    {
        return -1.0;
    }

    (2 * pattern.iter().sum::<PatternType>() - pattern[0] - pattern[4]) as f64 / 12.0
        * cur.d().length() as f64
    //  (2 * Reduce(*pattern) - (*pattern)[0] - (*pattern)[4]) / 12.0 * length(cur.d)
}

pub struct DimensionEstimate {
    dim: i32,
    ms: f64,
    err: i32,
}

impl Default for DimensionEstimate {
    fn default() -> Self {
        Self {
            dim: 0,
            ms: 0.0,
            err: 4,
        }
    }
}

pub fn EstimateDimension(
    image: &BitMatrix,
    a: ConcentricPattern,
    b: ConcentricPattern,
) -> DimensionEstimate {
    let ms_a = EstimateModuleSize(image, a, b);
    let ms_b = EstimateModuleSize(image, b, a);

    if (ms_a < 0.0 || ms_b < 0.0) {
        return DimensionEstimate::default();
    }

    let moduleSize = (ms_a + ms_b) / 2.0;

    let dimension = ((ConcentricPattern::distance(a, b) as f64 / moduleSize).round() as i32 + 7);
    let error = 1 - (dimension % 4);

    DimensionEstimate {
        dim: dimension + error,
        ms: moduleSize,
        err: (error).abs(),
    }
}

pub fn TraceLine(image: &BitMatrix, p: Point, d: Point, edge: i32) -> impl RegressionLineTrait {
    let mut cur = EdgeTracer::new(image, p, d - p);
    let mut line = RegressionLine::default();
    line.setDirectionInward(cur.back());

    // collect points inside the black line -> backup on 3rd edge
    cur.stepToEdge(Some(edge), Some(0), Some(edge == 3));
    if (edge == 3) {
        cur.turnBack();
    }

    let mut curI = EdgeTracer::new(image, (cur.p), (Point::mainDirection(cur.d())));
    // make sure curI positioned such that the white->black edge is directly behind
    // Test image: fix-traceline.jpg
    while (!bool::from(curI.edgeAtBack())) {
        if (curI.edgeAtLeft().into()) {
            curI.turnRight();
        } else if (curI.edgeAtRight().into()) {
            curI.turnLeft();
        } else {
            curI.step(Some(-1.0));
        }
    }

    for dir in [Direction::Left, Direction::Right] {
        // for (auto dir : {Direction::LEFT, Direction::RIGHT}) {
        let mut c = EdgeTracer::new(image, curI.p, curI.direction(dir));
        let stepCount = (Point::maxAbsComponent(cur.p - p)) as i32;
        loop {
            line.add(Point::centered(c.p));

            if !(--stepCount > 0 && c.stepAlongEdge(dir, Some(true))) {
                break;
            }
        } //while (--stepCount > 0 && c.stepAlongEdge(dir, true));
    }

    line.evaluate_max_distance(Some(1.0), Some(true));

    line
}
