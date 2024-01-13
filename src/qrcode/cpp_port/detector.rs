use crate::{
    common::{
        cpp_essentials::{
            CenterOfRing, DMRegressionLine, FindConcentricPatternCorners, FindLeftGuardBy, Matrix,
        },
        DefaultGridSampler, GridSampler, Result, SamplerControl,
    },
    point, point_i,
    qrcode::{
        decoder::{FormatInformation, Version, VersionRef},
        detector::QRCodeDetectorResult,
    },
    Exceptions, PointF,
};
use multimap::MultiMap;
use num::Integer;

use crate::{
    common::{
        cpp_essentials::{
            BitMatrixCursorTrait, ConcentricPattern, Direction, EdgeTracer, FixedPattern,
            GetPatternRowTP, IsPattern, LocateConcentricPattern, PatternRow, PatternType,
            PatternView, ReadSymmetricPattern, RegressionLine, RegressionLineTrait,
        },
        BitMatrix, PerspectiveTransform, Quadrilateral,
    },
    point_f, Point, PointI,
};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct FinderPatternSet {
    pub bl: ConcentricPattern,
    pub tl: ConcentricPattern,
    pub tr: ConcentricPattern,
}

pub type FinderPatterns = Vec<ConcentricPattern>;
pub type FinderPatternSets = Vec<FinderPatternSet>;

const LEN: usize = 5;
const SUM: usize = 7;
const PATTERN: FixedPattern<LEN, SUM, false> = FixedPattern::new([1, 1, 3, 1, 1]);
const SUBPATTERN_RMQR: FixedPattern<5, 5, false> = FixedPattern::new([1, 1, 1, 1, 1]);
const CORNER_EDGE_RMQR: FixedPattern<2, 4, false> = FixedPattern::new([3, 1]);
const E2E: bool = true;

fn FindPattern(view: PatternView<'_>) -> Result<PatternView<'_>> {
    FindLeftGuardBy::<LEN, _>(
        view,
        LEN,
        |view: &PatternView, spaceInPixel: Option<f32>| {
            // perform a fast plausability test for 1:1:3:1:1 pattern
            if view[2] < 2 as PatternType * std::cmp::max(view[0], view[4])
                || view[2] < std::cmp::max(view[1], view[3])
            {
                return false;
            }
            IsPattern::<E2E, 5, 7, false>(view, &PATTERN, spaceInPixel, 0.1, 0.0) != 0.0
        },
    )
}

/// Locate the finder patterns for the symbol.
/// This function can panic
pub fn FindFinderPatterns(image: &BitMatrix, tryHarder: bool) -> FinderPatterns {
    const MIN_SKIP: u32 = 3; // 1 pixel/module times 3 modules/center
    const MAX_MODULES_FAST: u32 = 20 * 4 + 17; // support up to version 20 for mobile clients

    // Let's assume that the maximum version QR Code we support takes up 1/4 the height of the
    // image, and then account for the center being 3 modules in size. This gives the smallest
    // number of pixels the center could be, so skip this often. When trying harder, look for all
    // QR versions regardless of how dense they are.
    let height = image.height();
    let mut skip = (3 * height) / (4 * MAX_MODULES_FAST);
    if skip < MIN_SKIP || tryHarder {
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
            if let Ok(up_next) = FindPattern(next) {
                next = up_next;
                next.isValid()
            } else {
                false
            }
        } {
            let p = point_f(
                next.pixelsInFront() as f32
                    + next[0] as f32
                    + next[1] as f32
                    + next[2] as f32 / 2.0,
                y as f32 + 0.5,
            );

            // make sure p is not 'inside' an already found pattern area
            if !res
                .iter()
                .any(|old| Point::distance(p, old.p) < (old.size as f32) / 2.0)
            {
                // if (FindIf(res, [p](const auto& old) { return distance(p, old) < old.size / 2; }) == res.end()) {
                let pattern = LocateConcentricPattern::<E2E, 5, 7>(
                    image,
                    &PATTERN.into(),
                    p,
                    next.iter().sum::<u16>() as i32 * 3,
                ); // 3 for very skewed samples
                   //    Reduce(next) * 3); // 3 for very skewed samples
                if let Some(p) = pattern {
                    // log(*pattern, 3);
                    // assert!(image.get_point(pattern.as_ref().unwrap().p));
                    res.push(p);
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
        ConcentricPattern::dot(a - b, a - b) as f64
            * (((b).size as f64) / ((a).size as f64)).powi(2) //std::pow(double(b.size) / a.size, 2)
    };

    let cosUpper: f64 = (45.0_f64 / 180.0 * std::f64::consts::PI).cos(); // TODO: use c++20 std::numbers::pi_v
    let cosLower: f64 = (135.0_f64 / 180.0 * std::f64::consts::PI).cos();

    let nbPatterns = (patterns).len();

    if nbPatterns < 2 {
        return FinderPatternSets::default();
    }

    for i in 0..(nbPatterns - 2) {
        // for (int i = 0; i < nbPatterns - 2; i++) {
        for j in (i + 1)..(nbPatterns - 1) {
            // for (int j = i + 1; j < nbPatterns - 1; j++) {
            for k in (j + 1)..nbPatterns {
                // for (int k = j + 1; k < nbPatterns - 0; k++) {
                let mut a = &patterns[i];
                let mut b = &patterns[j];
                let mut c = &patterns[k];
                // if the pattern sizes are too different to be part of the same symbol, skip this
                // and the rest of the innermost loop (sorted list)
                if c.size > a.size * 2 {
                    break;
                }

                // Orders the three points in an order [A,B,C] such that AB is less than AC
                // and BC is less than AC, and the angle between BC and BA is less than 180 degrees.

                let mut distAB2 = squaredDistance(*a, *b);
                let mut distBC2 = squaredDistance(*b, *c);
                let mut distAC2 = squaredDistance(*a, *c);

                if distBC2 >= distAB2 && distBC2 >= distAC2 {
                    std::mem::swap(&mut a, &mut b);
                    std::mem::swap(&mut distBC2, &mut distAC2);
                } else if distAB2 >= distAC2 && distAB2 >= distBC2 {
                    std::mem::swap(&mut b, &mut c);
                    std::mem::swap(&mut distAB2, &mut distAC2);
                }

                let distAB = distAB2.sqrt();
                let distBC = (distBC2).sqrt();

                // Make sure distAB and distBC don't differ more than reasonable
                // TODO: make sure the constant 2 is not to conservative for reasonably tilted symbols
                if distAB > 2.0 * distBC || distBC > 2.0 * distAB {
                    continue;
                }

                // Estimate the module count and ignore this set if it can not result in a valid decoding
                let moduleCount = (distAB + distBC)
                    / (2.0 * (a.size + b.size + c.size) as f64 / (3.0 * 7.0))
                    + 7.0;
                if !(21.0 * 0.9..=177.0 * 1.5).contains(&moduleCount)
                // moduleCount may be overestimated, see above
                {
                    continue;
                }

                // Make sure the angle between AB and BC does not deviate from 90° by more than 45°
                let cosAB_BC = (distAB2 + distBC2 - distAC2) / (2.0 * distAB * distBC);
                if (cosAB_BC.is_nan()) || cosAB_BC > cosUpper || cosAB_BC < cosLower {
                    continue;
                }

                // a^2 + b^2 = c^2 (Pythagorean theorem), and a = b (isosceles triangle).
                // Since any right triangle satisfies the formula c^2 - b^2 - a^2 = 0,
                // we need to check both two equal sides separately.
                // The value of |c^2 - 2 * b^2| + |c^2 - 2 * a^2| increases as dissimilarity
                // from isosceles right triangle.
                let d: f64 = (distAC2 - 2.0 * distAB2).abs() + (distAC2 - 2.0 * distBC2).abs();

                // Use cross product to figure out whether A and C are correct or flipped.
                // This asks whether BC x BA has a positive z component, which is the arrangement
                // we want for A, B, C. If it's negative then swap A and C.
                if ConcentricPattern::cross(*c - *b, *a - *b) < 0.0 {
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

    for (_, v) in sets {
        // for (auto& [d, s] : sets)
        res.extend(v);
    }

    res.sort_by_key(|i| i.bl.size);

    res
}

pub fn EstimateModuleSize(image: &BitMatrix, a: ConcentricPattern, b: ConcentricPattern) -> f64 {
    let mut cur = EdgeTracer::new(image, a.p, b.p - a.p);
    if !cur.isBlack() {
        return -1.0;
    }
    assert!(cur.isBlack());

    let pattern = ReadSymmetricPattern::<5, _>(&mut cur, a.size * 2);

    if pattern.is_none() {
        return -1.0;
    }

    let pattern = pattern.unwrap();

    if !(IsPattern::<E2E, 5, 7, false>(
        &PatternView::new(&PatternRow::new(pattern.to_vec())),
        &PATTERN,
        None,
        0.0,
        0.0,
    ) != 0.0)
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

    if ms_a < 0.0 || ms_b < 0.0 {
        return DimensionEstimate::default();
    }

    let moduleSize = (ms_a + ms_b) / 2.0;

    let dimension = (ConcentricPattern::distance(a, b) as f64 / moduleSize).round() as i32 + 7;
    let error = 1 - (dimension % 4);

    DimensionEstimate {
        dim: dimension + error,
        ms: moduleSize,
        err: (error).abs(),
    }
}

/// This function can panic
pub fn TraceLine(image: &BitMatrix, p: Point, d: Point, edge: i32) -> impl RegressionLineTrait {
    let mut cur = EdgeTracer::new(image, p, d - p);
    let mut line = RegressionLine::default();
    line.setDirectionInward(cur.back());

    // collect points inside the black line -> backup on 3rd edge
    cur.stepToEdge(Some(edge), Some(0), Some(edge == 3));
    if edge == 3 {
        cur.turnBack();
    }

    let mut curI = EdgeTracer::new(image, cur.p, Point::mainDirection(cur.d()));
    // make sure curI positioned such that the white->black edge is directly behind
    // Test image: fix-traceline.jpg
    while !bool::from(curI.edgeAtBack()) {
        if curI.edgeAtLeft().into() {
            curI.turnRight();
        } else if curI.edgeAtRight().into() {
            curI.turnLeft();
        } else {
            curI.step(Some(-1.0));
        }
    }

    for dir in [Direction::Left, Direction::Right] {
        // for (auto dir : {Direction::LEFT, Direction::RIGHT}) {
        let mut c = EdgeTracer::new(image, curI.p, curI.direction(dir));
        let mut stepCount = (Point::maxAbsComponent(cur.p - p)) as i32;
        loop {
            line.add(Point::centered(c.p))
                .expect("could not add point on line");

            stepCount -= 1;
            if !(stepCount > 0 && c.stepAlongEdge(dir, Some(true))) {
                break;
            }
        } //while (--stepCount > 0 && c.stepAlongEdge(dir, true));
    }

    line.evaluate_max_distance(Some(1.0), Some(true));

    line
}

// estimate how tilted the symbol is (return value between 1 and 2, see also above)
pub fn EstimateTilt(fp: &FinderPatternSet) -> f64 {
    let min = [fp.bl.size, fp.tl.size, fp.tr.size]
        .iter()
        .min()
        .copied()
        .unwrap_or(i32::MAX);
    let max = [fp.bl.size, fp.tl.size, fp.tr.size]
        .iter()
        .max()
        .copied()
        .unwrap_or(i32::MIN);

    (max as f64) / (min as f64)
}

pub fn Mod2Pix(
    dimension: i32,
    brOffset: Point,
    pix: Quadrilateral,
) -> Result<PerspectiveTransform> {
    let mut quad = Quadrilateral::rectangle(dimension, dimension, Some(3.5));
    // let quad = Rectangle(dimension, dimension, 3.5);
    quad[2] -= brOffset;

    PerspectiveTransform::quadrilateralToQuadrilateral(quad, pix)
    // return {quad, pix};
}

pub fn LocateAlignmentPattern(
    image: &BitMatrix,
    moduleSize: i32,
    estimate: Point,
) -> Option<Point> {
    // log(estimate, 2);

    for d in [
        point_f(0.0, 0.0),
        point_f(0.0, -1.0),
        point_f(0.0, 1.0),
        point_f(-1.0, 0.0),
        point_f(1.0, 0.0),
        point_f(-1.0, -1.0),
        point_f(1.0, -1.0),
        point_f(1.0, 1.0),
        point_f(-1.0, 1.0),
    ] {
        // 	for (auto d : {PointF{0, 0}, {0, -1}, {0, 1}, {-1, 0}, {1, 0}, {-1, -1}, {1, -1}, {1, 1}, {-1, 1},
        // #if 1
        // 				   }) {
        // #else
        // 				   {0, -2}, {0, 2}, {-2, 0}, {2, 0}, {-1, -2}, {1, -2}, {-1, 2}, {1, 2}, {-2, -1}, {-2, 1}, {2, -1}, {2, 1}}) {
        // #endif
        let cor = CenterOfRing(
            image,
            (estimate + moduleSize as f32 * 2.25 * d).floor(),
            moduleSize * 3,
            1,
            false,
        );

        // if we did not land on a black pixel the concentric pattern finder will fail
        if cor.is_none() || !image.get_point(cor.unwrap()) {
            continue;
        }

        if let Some(cor1) = CenterOfRing(image, cor.unwrap().floor(), moduleSize, 1, true) {
            if let Some(cor2) = CenterOfRing(image, cor.unwrap().floor(), moduleSize * 3, -2, true)
            {
                if Point::distance(cor1, cor2) < moduleSize as f32 / 2.0 {
                    let res = (cor1 + cor2) / 2.0;
                    // log(res, 3);
                    return Some(res);
                }
            }
        }
    }

    None
}

pub fn ReadVersion(
    image: &BitMatrix,
    dimension: u32,
    mod2Pix: PerspectiveTransform,
) -> Result<VersionRef> {
    let mut bits = [0; 2]; //

    for mirror in [false, true] {
        // Read top-right/bottom-left version info: 3 wide by 6 tall (depending on mirrored)
        let mut versionBits = 0;
        for y in (0..=5).rev() {
            // for (int y = 5; y >= 0; --y)
            for x in ((dimension - 11)..=(dimension - 9)).rev() {
                // for (int x = dimension - 9; x >= dimension - 11; --x) {
                let mod_ = if mirror { point_i(y, x) } else { point_i(x, y) };
                let pix = mod2Pix.transform_point((mod_).centered());
                if !image.is_in(pix) {
                    versionBits = -1;
                } else {
                    AppendBit(&mut versionBits, image.get_point(pix));
                }
                // log(pix, 3);
            }
            bits[usize::from(mirror)] = versionBits;
        }
    }

    Version::DecodeVersionInformation(bits[0], bits[1])
}

pub fn AppendBit(val: &mut i32, bit: bool) {
    *val <<= 1;

    *val |= i32::from(bit)
}

pub fn SampleQR(image: &BitMatrix, fp: &FinderPatternSet) -> Result<QRCodeDetectorResult> {
    let top = EstimateDimension(image, fp.tl, fp.tr);
    let left = EstimateDimension(image, fp.tl, fp.bl);

    if top.dim == 0 && left.dim == 0 {
        return Err(Exceptions::NOT_FOUND);
    }

    let best = if top.err == left.err {
        if top.dim > left.dim {
            top
        } else {
            left
        }
    } else if top.err < left.err {
        top
    } else {
        left
    };
    let mut dimension = best.dim;
    let moduleSize = (best.ms + 1.0) as i32;

    let mut br = ConcentricPattern {
        p: point_f(-1.0, -1.0),
        size: 0,
    };
    let mut brOffset = point_i(3, 3);

    // Everything except version 1 (21 modules) has an alignment pattern. Estimate the center of that by intersecting
    // line extensions of the 1 module wide square around the finder patterns. This could also help with detecting
    // slanted symbols of version 1.

    // generate 4 lines: outer and inner edge of the 1 module wide black line between the two outer and the inner
    // (tl) finder pattern
    let bl2 = TraceLine(image, fp.bl.p, fp.tl.p, 2);
    let bl3 = TraceLine(image, fp.bl.p, fp.tl.p, 3);
    let tr2 = TraceLine(image, fp.tr.p, fp.tl.p, 2);
    let tr3 = TraceLine(image, fp.tr.p, fp.tl.p, 3);

    if bl2.isValid() && tr2.isValid() && bl3.isValid() && tr3.isValid() {
        // intersect both outer and inner line pairs and take the center point between the two intersection points
        let brInter = (DMRegressionLine::intersect(&bl2, &tr2).ok_or(Exceptions::NOT_FOUND)?
            + DMRegressionLine::intersect(&bl3, &tr3).ok_or(Exceptions::NOT_FOUND)?)
            / 2.0;
        // log(brInter, 3);

        if dimension > 21 {
            if let Some(brCP) = LocateAlignmentPattern(image, moduleSize, brInter) {
                br = brCP.into();
            }
        }

        // if the symbol is tilted or the resolution of the RegressionLines is sufficient, use their intersection
        // as the best estimate (see discussion in #199 and test image estimate-tilt.jpg )
        if !image.is_in(br.p)
            && (EstimateTilt(fp) > 1.1
                || (bl2.isHighRes() && bl3.isHighRes() && tr2.isHighRes() && tr3.isHighRes()))
        {
            br = brInter.into();
        }
    }

    // otherwise the simple estimation used by upstream is used as a best guess fallback
    if !image.is_in(br.p) {
        br = fp.tr - fp.tl + fp.bl;
        brOffset = point_i(0, 0);
    }

    // log(br, 3);
    let mut mod2Pix = Mod2Pix(
        dimension,
        brOffset,
        Quadrilateral::from([fp.tl.p, fp.tr.p, br.p, fp.bl.p]),
    )?;

    if dimension >= Version::DimensionOfVersion(7, false) as i32 {
        let version = ReadVersion(image, dimension as u32, mod2Pix);

        // if the version bits are garbage -> discard the detection
        if version.is_err()
            || (version.as_ref().unwrap().getDimensionForVersion() as i32 - dimension).abs() > 8
        {
            /*return DetectorResult();*/
            return Err(Exceptions::NOT_FOUND);
        }
        if version.as_ref().unwrap().getDimensionForVersion() as i32 != dimension {
            // printf("update dimension: %d -> %d\n", dimension, version.dimension());
            dimension = version.as_ref().unwrap().getDimensionForVersion() as i32;
            mod2Pix = Mod2Pix(
                dimension,
                brOffset,
                Quadrilateral::from([fp.tl.p, fp.tr.p, br.p, fp.bl.p]),
            )?;
        }
        // #if 1
        let apM = version.as_ref().unwrap().getAlignmentPatternCenters(); // alignment pattern positions in modules
        let mut apP = Matrix::new(apM.len(), apM.len())?; // found/guessed alignment pattern positions in pixels
        let N = (apM.len()) - 1;

        // project the alignment pattern at module coordinates x/y to pixel coordinate based on current mod2Pix
        let projectM2P = |x, y, mod2Pix: &PerspectiveTransform| {
            mod2Pix.transform_point(Point::centered(point_i(apM[x], apM[y])))
        };

        let mut findInnerCornerOfConcentricPattern = |x, y, fp: ConcentricPattern| {
            let pc = apP.set(x, y, projectM2P(x, y, &mod2Pix));
            if let Some(fpQuad) = FindConcentricPatternCorners(image, fp.p, fp.size, 2) {
                for c in fpQuad.0 {
                    if Point::distance(c, pc) < (fp.size as f32) / 2.0 {
                        apP.set(x, y, c);
                    }
                }
            }
        };

        findInnerCornerOfConcentricPattern(0, 0, fp.tl);
        findInnerCornerOfConcentricPattern(0, N, fp.bl);
        findInnerCornerOfConcentricPattern(N, 0, fp.tr);

        let bestGuessAPP = |x, y, apP: &Matrix<Point>| {
            if let Some(p) = apP.get(x, y)
            // if (auto p = apP(x, y))
            {
                return p;
            }
            projectM2P(x, y, &mod2Pix)
        };

        for y in 0..=N {
            // for (int y = 0; y <= N; ++y)
            for x in 0..=N {
                // for (int x = 0; x <= N; ++x) {
                if apP.get(x, y).is_some() {
                    continue;
                }

                let guessed = if x * y == 0 {
                    bestGuessAPP(x, y, &apP)
                } else {
                    bestGuessAPP(x - 1, y, &apP) + bestGuessAPP(x, y - 1, &apP)
                        - bestGuessAPP(x - 1, y - 1, &apP)
                };
                if let Some(found) = LocateAlignmentPattern(image, moduleSize, guessed)
                // if (auto found = LocateAlignmentPattern(image, moduleSize, guessed))
                {
                    apP.set(x, y, found);
                }
            }
        }

        // go over the whole set of alignment patters again and try to fill any remaining gap by using available neighbors as guides
        for y in 0..=N {
            // for (int y = 0; y <= N; ++y) {
            for x in 0..=N {
                // for (int x = 0; x <= N; ++x) {
                if apP.get(x, y).is_some() {
                    continue;
                }

                // find the two closest valid alignment pattern pixel positions both horizontally and vertically
                let mut hori = Vec::new();
                let mut verti = Vec::new();
                let mut i = 2;
                while i < 2 * N + 2 && hori.len() < 2 {
                    let xi = x as isize + i as isize / 2 * (if i % 2 != 0 { 1 } else { -1 });
                    if 0 <= xi && xi <= N as isize && apP.get(xi as usize, y).is_some() {
                        hori.push(
                            apP.get(xi as usize, y)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                        );
                    }
                    i += 1;
                }
                // for (int i = 2; i < 2 * N + 2 && Size(hori) < 2; ++i) {
                // 	let xi = x + i / 2 * (i%2 ? 1 : -1);
                // 	if (0 <= xi && xi <= N && apP(xi, y))
                // 		{hori.push_back(*apP(xi, y));}
                // }
                let mut i = 2;
                while i < 2 * N + 2 && verti.len() < 2 {
                    let yi = y as isize + i as isize / 2 * (if i % 2 != 0 { 1 } else { -1 });
                    if 0 <= yi && yi <= N as isize && apP.get(x, yi as usize).is_some() {
                        verti.push(
                            apP.get(x, yi as usize)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                        );
                    }
                    i += 1;
                }
                // for (int i = 2; i < 2 * N + 2 && Size(verti) < 2; ++i) {
                // 	let yi = y + i / 2 * (i%2 ? 1 : -1);
                // 	if (0 <= yi && yi <= N && apP(x, yi))
                // 		{verti.push_back(*apP(x, yi));}
                // }

                // if we found 2 each, intersect the two lines that are formed by connecting the point pairs
                if (hori.len()) == 2 && (verti.len()) == 2 {
                    let guessed = RegressionLine::intersect(
                        &DMRegressionLine::new(hori[0], hori[1]),
                        &DMRegressionLine::new(verti[0], verti[1]),
                    )
                    .ok_or(Exceptions::ILLEGAL_STATE)?;
                    let found = LocateAlignmentPattern(image, moduleSize, guessed);
                    // search again near that intersection and if the search fails, use the intersection
                    // if (!found.is_some()) {printf("location guessed at %dx%d\n", x, y)};
                    apP.set(x, y, if let Some(f) = found { f } else { guessed });
                }
            }
        }

        if let Some(c) = apP.get(N, N)
        // if (auto c = apP.get(N, N))
        {
            mod2Pix = Mod2Pix(
                dimension,
                point_i(3, 3),
                Quadrilateral::from([fp.tl.p, fp.tr.p, c, fp.bl.p]),
            )?;
        }

        // go over the whole set of alignment patters again and fill any remaining gaps by a projection based on an updated mod2Pix
        // projection. This works if the symbol is flat, wich is a reasonable fall-back assumption.
        for y in 0..=N {
            // for (int y = 0; y <= N; ++y) {
            for x in 0..=N {
                // for (int x = 0; x <= N; ++x) {
                if apP.get(x, y).is_some() {
                    continue;
                }

                // printf("locate failed at %dx%d\n", x, y);
                apP.set(x, y, projectM2P(x, y, &mod2Pix));
            }
        }

        // assemble a list of region-of-interests based on the found alignment pattern pixel positions

        let mut rois = Vec::new();
        for y in 0..N {
            // for (int y = 0; y < N; ++y){
            for x in 0..N {
                // for (int x = 0; x < N; ++x) {
                let x0 = apM[x];
                let x1 = apM[x + 1];
                let y0 = apM[y];
                let y1 = apM[y + 1];
                rois.push(SamplerControl {
                    p0: point_i(x0 - u32::from(x == 0) * 6, y0 - u32::from(y == 0) * 6),
                    p1: point_i(
                        x1 + u32::from(x == N - 1) * 7,
                        y1 + u32::from(y == N - 1) * 7,
                    ),
                    transform: PerspectiveTransform::quadrilateralToQuadrilateral(
                        Quadrilateral::rectangle_from_xy(
                            x0 as f32, x1 as f32, y0 as f32, y1 as f32, None,
                        ),
                        Quadrilateral::from([
                            apP.get(x, y).unwrap(),
                            apP.get(x + 1, y).unwrap(),
                            apP.get(x + 1, y + 1).unwrap(),
                            apP.get(x, y + 1).unwrap(),
                        ]),
                    )?,
                });
            }
        }
        let grid_sampler = DefaultGridSampler;
        let (sampled, rp) =
            grid_sampler.sample_grid(image, dimension as u32, dimension as u32, &rois)?;
        let result = QRCodeDetectorResult::new(sampled, rp.to_vec());
        return Ok(result);
        //  grid_sampler.sample_grid(image, dimension, dimension, &rois);
        // #endif
    }

    let grid_sampler = DefaultGridSampler;
    let (sampled, rps) = grid_sampler.sample_grid(
        image,
        dimension as u32,
        dimension as u32,
        &[SamplerControl {
            p1: point_i(dimension as u32, dimension as u32),
            p0: point_i(0, 0),
            transform: mod2Pix,
        }],
    )?;
    let result = QRCodeDetectorResult::new(sampled, rps.to_vec());
    Ok(result)
    // return SampleGrid(image, dimension, dimension, mod2Pix);
}

/**
* This method detects a code in a "pure" image -- that is, pure monochrome image
* which contains only an unrotated, unskewed, image of a code, with some white border
* around it. This is a specialized method that works exceptionally fast in this special
* case.
*/
pub fn DetectPureQR(image: &BitMatrix) -> Result<QRCodeDetectorResult> {
    type Pattern = [PatternType; 5];

    // #ifdef PRINT_DEBUG
    // 	SaveAsPBM(image, "weg.pbm");
    // #endif

    let MIN_MODULES: u32 = Version::DimensionOfVersion(1, false);
    let MAX_MODULES: u32 = Version::DimensionOfVersion(40, false);

    let (found, left, top, width, height) = image.findBoundingBox(0, 0, 0, 0, MIN_MODULES);

    if !found || (width as i32 - height as i32).abs() > 1 {
        return Err(Exceptions::NOT_FOUND);
    }
    let right = left + width - 1;
    let bottom = top + height - 1;

    let tl = point_i(left, top);
    let tr = point_i(right, top);
    let bl = point_i(left, bottom);
    let mut diagonal: Pattern = Default::default();
    // allow corners be moved one pixel inside to accommodate for possible aliasing artifacts
    for [p, d] in [
        [tl, point_i(1, 1)],
        [tr, point_f(-1.0, 1.0)],
        [bl, point_f(1.0, -1.0)],
    ] {
        diagonal = EdgeTracer::new(image, p, d)
            .readPatternFromBlack(1, Some((width / 3 + 1) as i32))
            .ok_or(Exceptions::NOT_FOUND)?;

        let diag_hld = diagonal.to_vec().into();
        let view = PatternView::new(&diag_hld);
        if !(IsPattern::<E2E, 5, 7, false>(&view, &PATTERN, None, 0.0, 0.0) != 0.0) {
            return Err(Exceptions::NOT_FOUND);
        }
    }

    let fpWidth = diagonal.iter().sum::<u16>() as i32; //Reduce(diagonal);
    let dimension = EstimateDimension(
        image,
        ConcentricPattern {
            p: tl + fpWidth as f32 / 2.0 * point_i(1, 1),
            size: fpWidth,
        },
        ConcentricPattern {
            p: tr + fpWidth as f32 / 2.0 * point_f(-1.0, 1.0),
            size: fpWidth,
        },
    )
    .dim;

    let moduleSize: f32 = ((width) as f32) / dimension as f32;
    if dimension < MIN_MODULES as i32
        || dimension > MAX_MODULES as i32
        || !image.is_in(point_f(
            left as f32 + moduleSize / 2.0 + (dimension - 1) as f32 * moduleSize,
            top as f32 + moduleSize / 2.0 + (dimension - 1) as f32 * moduleSize,
        ))
    {
        return Err(Exceptions::NOT_FOUND);
    }

    // #ifdef PRINT_DEBUG
    // 	LogMatrix log;
    // 	LogMatrixWriter lmw(log, image, 5, "grid2.pnm");
    // 	for (int y = 0; y < dimension; y++)
    // 		for (int x = 0; x < dimension; x++)
    // 			log(PointF(left + (x + .5f) * moduleSize, top + (y + .5f) * moduleSize));
    // #endif

    // Now just read off the bits (this is a crop + subsample)
    Ok(QRCodeDetectorResult::new(
        image.Deflate(
            dimension as u32,
            dimension as u32,
            top as f32 + moduleSize / 2.0,
            left as f32 + moduleSize / 2.0,
            moduleSize,
        )?,
        vec![
            point_i(left, top),
            point_i(right, top),
            point_i(right, bottom),
            point_i(left, bottom),
        ],
    ))

    // return {Deflate(image, dimension, dimension, top + moduleSize / 2, left + moduleSize / 2, moduleSize),
    // 		{{left, top}, {right, top}, {right, bottom}, {left, bottom}}};
}

pub fn DetectPureMQR(image: &BitMatrix) -> Result<QRCodeDetectorResult> {
    type Pattern = [PatternType; 5];

    const MIN_MODULES: u32 = Version::DimensionOfVersion(1, true);
    const MAX_MODULES: u32 = Version::DimensionOfVersion(4, true);

    let (found, left, top, width, height) = image.findBoundingBox(0, 0, 0, 0, MIN_MODULES);

    // int left, top, width, height;
    if !found || (width as i32 - height as i32).abs() > 1 {
        return Err(Exceptions::NOT_FOUND);
    }
    let right = left + width - 1;
    let bottom = top + height - 1;

    // allow corners be moved one pixel inside to accommodate for possible aliasing artifacts
    let diagonal: Pattern = EdgeTracer::new(image, point_i(left, top), point_i(1, 1))
        .readPatternFromBlack(1, None)
        .ok_or(Exceptions::ILLEGAL_STATE)?;
    let diag_hld = diagonal.to_vec().into();
    let view = PatternView::new(&diag_hld);
    if !(IsPattern::<E2E, 5, 7, false>(&view, &PATTERN, None, 0.0, 0.0) != 0.0) {
        return Err(Exceptions::NOT_FOUND);
    }

    let fpWidth = diagonal.into_iter().sum::<u16>();
    let moduleSize: f32 = (fpWidth as f32) / 7.0;
    let dimension = (width as f32 / moduleSize).floor() as u32;

    if dimension < MIN_MODULES
        || dimension > MAX_MODULES
        || !image.is_in(point_f(
            left as f32 + moduleSize / 2.0 + (dimension - 1) as f32 * moduleSize,
            top as f32 + moduleSize / 2.0 + (dimension - 1) as f32 * moduleSize,
        ))
    {
        return Err(Exceptions::NOT_FOUND);
    }

    // #ifdef PRINT_DEBUG
    // 	LogMatrix log;
    // 	LogMatrixWriter lmw(log, image, 5, "grid2.pnm");
    // 	for (int y = 0; y < dimension; y++)
    // 		for (int x = 0; x < dimension; x++)
    // 			log(PointF(left + (x + .5f) * moduleSize, top + (y + .5f) * moduleSize));
    // #endif

    // Now just read off the bits (this is a crop + subsample)
    Ok(QRCodeDetectorResult::new(
        image.Deflate(
            dimension,
            dimension,
            top as f32 + moduleSize / 2.0,
            left as f32 + moduleSize / 2.0,
            moduleSize,
        )?,
        vec![
            point_i(left, top),
            point_i(right, top),
            point_i(right, bottom),
            point_i(left, bottom),
        ],
    ))
    // return {Deflate(image, dimension, dimension, top + moduleSize / 2, left + moduleSize / 2, moduleSize),
    // 		{{left, top}, {right, top}, {right, bottom}, {left, bottom}}};
}

pub fn DetectPureRMQR(image: &BitMatrix) -> Result<QRCodeDetectorResult> {
    type Pattern = [PatternType; 5]; //std::array<PatternView::value_type, PATTERN.size()>;
    type SubPattern = [PatternType; 5]; //std::array<PatternView::value_type, SUBPATTERN_RMQR.size()>;
    type CornerEdgePattern = [PatternType; 2]; //std::array<PatternView::value_type, CORNER_EDGE_RMQR.size()>;

    // #ifdef PRINT_DEBUG
    // 	SaveAsPBM(image, "weg.pbm");
    // #endif

    const MIN_MODULES: u32 = 7;
    const MIN_MODULES_W: u32 = 27;
    const MIN_MODULES_H: u32 = 7;
    const MAX_MODULES_W: u32 = 139;
    const MAX_MODULES_H: u32 = 17;

    let (found, left, top, width, height) = image.findBoundingBox(0, 0, 0, 0, MIN_MODULES);

    if (!found) {
        return Err(Exceptions::NOT_FOUND);
    }
    let right = left + width - 1;
    let bottom = top + height - 1;

    let tl = point_i(left, top);
    let tr = point_i(right, top);
    let br = point_i(right, bottom);
    let bl = point_i(left, bottom);

    // allow corners be moved one pixel inside to accommodate for possible aliasing artifacts
    let diagonal: Pattern = EdgeTracer::new(image, tl, point_i(1, 1))
        .readPatternFromBlack(1, None)
        .ok_or(Exceptions::ILLEGAL_STATE)?;
    let diag_hld = diagonal.to_vec().into();
    let view = PatternView::new(&diag_hld);
    if !(IsPattern::<E2E, 5, 7, false>(&view, &PATTERN, None, 0.0, 0.0) != 0.0) {
        return Err(Exceptions::NOT_FOUND);
    }

    // Finder sub pattern
    let mut subdiagonal: SubPattern = EdgeTracer::new(image, br, point_i(-1, -1))
        .readPatternFromBlack(1, None)
        .ok_or(Exceptions::ILLEGAL_STATE)?;
    if (subdiagonal.len() == 5 && subdiagonal[4] > subdiagonal[3]) {
        // Sub pattern has no separator so can run off along the diagonal
        subdiagonal[4] = subdiagonal[3]; // Hack it back to previous
    }
    let subdiagonal_hld = subdiagonal.to_vec().into();
    let view = PatternView::new(&subdiagonal_hld);
    if !(IsPattern::<E2E, 5, 5, false>(&view, &SUBPATTERN_RMQR, None, 0.0, 0.0) != 0.0) {
        return Err(Exceptions::NOT_FOUND);
    }

    // Horizontal corner finder patterns (for vertical ones see below)
    for (p, d) in [(tr, point_i(-1, 0)), (bl, point_i(1, 0))] {
        // for (auto [p, d] : {std::pair(tr, PointI{-1, 0}), {bl, {1, 0}}}) {
        let corner: CornerEdgePattern = EdgeTracer::new(image, p, d)
            .readPatternFromBlack(1, None)
            .ok_or(Exceptions::ILLEGAL_STATE)?;
        let corner_hld = corner.to_vec().into();
        let view = PatternView::new(&corner_hld);
        if !(IsPattern::<E2E, 2, 4, false>(&view, &CORNER_EDGE_RMQR, None, 0.0, 0.0) != 0.0) {
            {
                return Err(Exceptions::NOT_FOUND);
            }
        }
    }

    let fpWidth = (diagonal).into_iter().sum::<u16>();
    let moduleSize = (fpWidth as f32) / 7.0;
    let dimW = (width as f32 / moduleSize as f32).floor() as u32;
    let dimH = (height as f32 / moduleSize as f32).floor() as u32;

    if (dimW == dimH
        || dimW.is_even()
        || dimH.is_even()
        || dimW < MIN_MODULES_W
        || dimW > MAX_MODULES_W
        || dimH < MIN_MODULES_H
        || dimH > MAX_MODULES_H
        || !image.is_in(point_f(
            left as f32 + moduleSize / 2.0 + (dimW as f32 - 1.0) * moduleSize,
            top as f32 + moduleSize / 2.0 + (dimH as f32 - 1.0) * moduleSize,
        )))
    {
        return Err(Exceptions::NOT_FOUND);
    }

    // Vertical corner finder patterns
    if (dimH > 7) {
        // None for R7
        let corner: CornerEdgePattern = EdgeTracer::new(image, tr, point_i(0, 1))
            .readPatternFromBlack(1, None)
            .ok_or(Exceptions::ILLEGAL_STATE)?;
        let corner_hld = corner.to_vec().into();
        let view = PatternView::new(&corner_hld);
        if !(IsPattern::<E2E, 2, 4, false>(&view, &CORNER_EDGE_RMQR, None, 0.0, 0.0) != 0.0) {
            return Err(Exceptions::NOT_FOUND);
        }
        if (dimH > 9) {
            // No bottom left for R9
            let corner: CornerEdgePattern = EdgeTracer::new(image, bl, point_i(0, -1))
                .readPatternFromBlack(1, None)
                .ok_or(Exceptions::ILLEGAL_STATE)?;
            let corner_hld = corner.to_vec().into();
            let view = PatternView::new(&corner_hld);
            if !(IsPattern::<E2E, 2, 4, false>(&view, &CORNER_EDGE_RMQR, None, 0.0, 0.0) != 0.0) {
                return Err(Exceptions::NOT_FOUND);
            }
        }
    }

    // #ifdef PRINT_DEBUG
    // 	LogMatrix log;
    // 	LogMatrixWriter lmw(log, image, 5, "grid2.pnm");
    // 	for (int y = 0; y < dimH; y++)
    // 		for (int x = 0; x < dimW; x++)
    // 			log(PointF(left + (x + .5f) * moduleSize, top + (y + .5f) * moduleSize));
    // #endif

    // Now just read off the bits (this is a crop + subsample)
    Ok(QRCodeDetectorResult::new(
        image.Deflate(
            dimW,
            dimH,
            top as f32 + moduleSize / 2.0,
            left as f32 + moduleSize / 2.0,
            moduleSize,
        )?,
        vec![tl, tr, br, bl],
    ))
    // return {Deflate(image, dimW, dimH, top + moduleSize / 2, left + moduleSize / 2, moduleSize), {tl, tr, br, bl}};
}

pub fn SampleMQR(image: &BitMatrix, fp: ConcentricPattern) -> Result<QRCodeDetectorResult> {
    let Some(fpQuad) = FindConcentricPatternCorners(image, fp.p, fp.size, 2) else {
        return Err(Exceptions::NOT_FOUND);
    };

    let srcQuad = Quadrilateral::rectangle(7, 7, Some(0.5));

    // #if defined(_MSVC_LANG) // TODO: see MSVC issue https://developercommunity.visualstudio.com/t/constexpr-object-is-unable-to-be-used-as/10035065
    // 	static
    // #else
    // 	constexpr
    // #endif
    let FORMAT_INFO_COORDS: [Point; 17] = [
        point_i(0, 8),
        point_i(1, 8),
        point_i(2, 8),
        point_i(3, 8),
        point_i(4, 8),
        point_i(5, 8),
        point_i(6, 8),
        point_i(7, 8),
        point_i(8, 8),
        point_i(8, 7),
        point_i(8, 6),
        point_i(8, 5),
        point_i(8, 4),
        point_i(8, 3),
        point_i(8, 2),
        point_i(8, 1),
        point_i(8, 0),
    ];

    let mut bestFI = FormatInformation::default();
    let mut bestPT = PerspectiveTransform::quadrilateralToQuadrilateral(
        srcQuad,
        fpQuad.rotated_corners(Some(0), None),
    )?;

    for i in 0..4 {
        // for (int i = 0; i < 4; ++i) {
        let mod2Pix = PerspectiveTransform::quadrilateralToQuadrilateral(
            srcQuad,
            fpQuad.rotated_corners(Some(i), None),
        )?;

        let check = |i, checkOne: bool| {
            let p = mod2Pix.transform_point(Point::centered(FORMAT_INFO_COORDS[i]));
            image.is_in(p) && (!checkOne || image.get_point(p))
        };

        // check that we see both innermost timing pattern modules
        if !check(0, true) || !check(8, false) || !check(16, true) {
            continue;
        }

        let mut formatInfoBits = 0;
        for info_coord in FORMAT_INFO_COORDS.iter().take(15 + 1).skip(1)
        // for i in 1..=15
        // for (int i = 1; i <= 15; ++i)
        {
            AppendBit(
                &mut formatInfoBits,
                image.get_point(mod2Pix.transform_point(Point::centered(*info_coord))),
            );
        }

        let fi = FormatInformation::DecodeMQR(formatInfoBits as u32);
        if fi.hammingDistance < bestFI.hammingDistance {
            bestFI = fi;
            bestPT = mod2Pix;
        }
    }

    if !bestFI.isValid() {
        return Err(Exceptions::NOT_FOUND);
    }

    let dim: u32 = Version::DimensionOfVersion(bestFI.microVersion, true);

    // check that we are in fact not looking at a corner of a non-micro QRCode symbol
    // we accept at most 1/3rd black pixels in the quite zone (in a QRCode symbol we expect about 1/2).
    let mut blackPixels = 0;
    for i in 0..dim {
        // for (int i = 0; i < dim; ++i) {
        let px = bestPT.transform_point(Point::centered(point_i(i, dim)));
        let py = bestPT.transform_point(Point::centered(point_i(dim, i)));
        blackPixels += u32::from(image.is_in(px) && image.get_point(px))
            + u32::from(image.is_in(py) && image.get_point(py));
    }
    if blackPixels > 2 * dim / 3 {
        return Err(Exceptions::NOT_FOUND);
    }

    let grid_sampler = DefaultGridSampler;
    let (sample, rps) = grid_sampler.sample_grid(
        image,
        dim,
        dim,
        &[SamplerControl {
            p1: point_i(dim, dim),
            p0: point_i(0, 0),
            transform: bestPT,
        }],
    )?;
    Ok(QRCodeDetectorResult::new(sample, rps.to_vec()))

    //  SampleGrid(image, dim, dim, bestPT)
}

pub fn SampleRMQR(image: &BitMatrix, fp: ConcentricPattern) -> Result<QRCodeDetectorResult> {
    // TODO proper
    let Some(fpQuad) = FindConcentricPatternCorners(image, fp.p, fp.size, 2) else {
        return Err(Exceptions::NOT_FOUND);
    };

    let srcQuad = Quadrilateral::rectangle(7, 7, Some(0.5));

    let FORMAT_INFO_EDGE_COORDS: [Point; 4] =
        [point_i(8, 0), point_i(9, 0), point_i(10, 0), point_i(11, 0)];
    let FORMAT_INFO_COORDS: [Point; 18] = [
        point_i(8, 1),
        point_i(8, 2),
        point_i(8, 3),
        point_i(8, 4),
        point_i(8, 5),
        point_i(9, 1),
        point_i(9, 2),
        point_i(9, 3),
        point_i(9, 4),
        point_i(9, 5),
        point_i(10, 1),
        point_i(10, 2),
        point_i(10, 3),
        point_i(10, 4),
        point_i(10, 5),
        point_i(11, 1),
        point_i(11, 2),
        point_i(11, 3),
    ];

    let mut bestFI: FormatInformation = FormatInformation::default();
    let mut bestPT: PerspectiveTransform = PerspectiveTransform::default();

    for i in 0..4 {
        // for (int i = 0; i < 4; ++i) {
        let mod2Pix = PerspectiveTransform::quadrilateralToQuadrilateral(
            srcQuad,
            fpQuad.rotated_corners(Some(i), None),
        )?;

        let check = |i: usize, on: bool| {
            let p = mod2Pix.transform_point(Point::centered(FORMAT_INFO_EDGE_COORDS[i]));
            image.is_in(p) && image.get_point(p) == on
        };

        // check that we see top edge timing pattern modules
        if (!check(0, true) || !check(1, false) || !check(2, true) || !check(3, false)) {
            continue;
        }

        let mut formatInfoBits = 0;
        for i in 0..FORMAT_INFO_COORDS.len() {
            // for (int i = 0; i < Size(FORMAT_INFO_COORDS); ++i)
            AppendBit(
                &mut formatInfoBits,
                image.get_point(mod2Pix.transform_point(Point::centered(FORMAT_INFO_COORDS[i]))),
            );
        }

        let fi = FormatInformation::DecodeRMQR(formatInfoBits as u32, 0 /*formatInfoBits2*/);
        if (fi.hammingDistance < bestFI.hammingDistance) {
            bestFI = fi;
            bestPT = mod2Pix;
        }
    }

    if (!bestFI.isValid()) {
        return Err(Exceptions::NOT_FOUND);
    }

    let dim = Version::DimensionOfVersionRMQR(bestFI.rMQRVersion as u32 + 1);

    let grid_sampler = DefaultGridSampler;
    let (sample, rps) = grid_sampler.sample_grid(
        image,
        dim.x as u32,
        dim.y as u32,
        &[SamplerControl {
            p1: point_i(dim.x, dim.y),
            p0: point_i(0, 0),
            transform: bestPT,
        }],
    )?;
    Ok(QRCodeDetectorResult::new(sample, rps.to_vec()))
    //  SampleGrid(image, dim.x, dim.y, bestPT)
}
