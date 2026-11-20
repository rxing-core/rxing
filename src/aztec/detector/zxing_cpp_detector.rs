/*
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{
    Point,
    common::{
        BitMatrix, PerspectiveTransform, Quadrilateral,
        cpp_essentials::{FindConcentricPatternCorners, FixedPattern, LocateConcentricPattern},
    },
    point,
};

const E2E: bool = true;

const COMPACT_BULLSEYE: FixedPattern<7, 7> = FixedPattern::new([1, 1, 1, 1, 1, 1, 1]);
const COMPACT_NB_CENTER_LAYERS: u32 = 5;

const FULL_BULLSEYE: FixedPattern<11, 11> = FixedPattern::new([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
const FULL_NB_CENTER_LAYERS: u32 = 7;

pub struct BullseyeMatch {
    pub corners: Quadrilateral,
    pub compact: bool,
}

/// Locates the Aztec bullseye near `seed`, returning the corners just outside its outermost
/// (reference) ring
pub fn locate_bullseye(image: &BitMatrix, seed: Point) -> Option<BullseyeMatch> {
    // Smallest real Aztec symbol (compact, 1 layer) is 15x15 modules; anything smaller can't
    // possibly contain a bullseye, and would otherwise drive `range` to 0 below, which
    // `LocateConcentricPattern`'s internals assert against.
    const MIN_IMAGE_SIZE: u32 = 15;
    if image.getWidth() < MIN_IMAGE_SIZE || image.getHeight() < MIN_IMAGE_SIZE {
        return None;
    }

    let range = (image.getWidth().min(image.getHeight()) / 2) as i32;

    if let Some(m) = try_pattern::<11, 11>(
        image,
        seed,
        range,
        &FULL_BULLSEYE.into(),
        FULL_NB_CENTER_LAYERS,
        false,
    ) {
        return Some(m);
    }

    try_pattern::<7, 7>(
        image,
        seed,
        range,
        &COMPACT_BULLSEYE.into(),
        COMPACT_NB_CENTER_LAYERS,
        true,
    )
}

fn try_pattern<const LEN: usize, const SUM: usize>(
    image: &BitMatrix,
    seed: Point,
    range: i32,
    pattern: &[u16; LEN],
    nb_center_layers: u32,
    compact: bool,
) -> Option<BullseyeMatch> {
    let found = LocateConcentricPattern::<E2E, LEN, SUM>(image, pattern, seed, range)?;

    let module_size = found.size as f32 / LEN as f32;
    let half_side = nb_center_layers as f32 * module_size;
    let center = found.p;

    let corners = Quadrilateral::new(
        point(center.x + half_side, center.y - half_side), // top right
        point(center.x + half_side, center.y + half_side), // bottom right
        point(center.x - half_side, center.y + half_side), // bottom left
        point(center.x - half_side, center.y - half_side), // top left
    );

    Some(BullseyeMatch { corners, compact })
}

/// A candidate bullseye found by [`find_bullseye_candidates`]: its (approximate) center and the
/// pixel width of its inner 7-module (compact bullseye sized) pattern.
#[derive(Clone, Copy, Debug)]
pub struct BullseyeCandidate {
    pub p: Point,
    pub size: i32,
}

/// Checks that the 7 bars `view[1..8]` look like a `1:1:1:1:1:1:1` Aztec center pattern, with
/// `view[0]` and `view[8]` being the (black) ring just outside of it.
///
/// Instead of a strict per-module tolerance check, this compares the sums of each pair of
/// neighbouring bars (which are robust against a binarizer that biases black vs. white widths),
/// like zxing-cpp's `IsAztecCenterPattern`.
fn is_aztec_center_pattern(view: &[u16; 9]) -> bool {
    let inner = &view[1..8];
    let mut m = inner[0] as i32 + inner[1] as i32;
    let mut mx = m;
    for i in 1..inner.len() - 1 {
        let v = inner[i] as i32 + inner[i + 1] as i32;
        m = m.min(v);
        mx = mx.max(v);
    }
    let mid = inner[3] as i32;
    mx <= m * 4 / 3 + 1 && view[0] as i32 >= mid - 2 && view[8] as i32 >= mid - 2
}

/// Steps from `p` in direction `d` until the color changes, at most `range` steps. Returns the
/// number of steps taken (0 if the image border or `range` was hit before an edge).
fn step_to_next_edge(image: &BitMatrix, p: &mut (i32, i32), d: (i32, i32), range: i32) -> i32 {
    let color = image.get(p.0 as u32, p.1 as u32);
    let (w, h) = (image.getWidth() as i32, image.getHeight() as i32);
    let mut steps = 0;
    loop {
        let np = (p.0 + d.0, p.1 + d.1);
        if np.0 < 0 || np.1 < 0 || np.0 >= w || np.1 >= h || steps >= range {
            return 0;
        }
        *p = np;
        steps += 1;
        if image.get(np.0 as u32, np.1 as u32) != color {
            return steps;
        }
    }
}

/// Reads the bullseye's bar widths through `center` in direction `d` (and its opposite), checks
/// them with [`is_aztec_center_pattern`] and returns the width of the inner 7 bars (0 if it's
/// not a plausible center pattern). If `update` is set, `center` is moved to the middle of the
/// center module along `d`.
fn check_symmetric_aztec_center_pattern(
    image: &BitMatrix,
    center: &mut (i32, i32),
    d: (i32, i32),
    range: i32,
    update: bool,
) -> i32 {
    // tilted symbols may have a larger vertical than horizontal range
    let range = range * 2;
    if !image.get(center.0 as u32, center.1 as u32) {
        return 0;
    }
    let mut fwd = *center;
    let mut bwd = *center;
    let back = (-d.0, -d.1);
    let center_fwd = step_to_next_edge(image, &mut fwd, d, range);
    if center_fwd == 0 {
        return 0;
    }
    let center_bwd = step_to_next_edge(image, &mut bwd, back, range);
    if center_bwd == 0 {
        return 0;
    }

    let mut view = [0u16; 9];
    view[4] = (center_fwd + center_bwd - 1) as u16;
    let mut used = view[4] as i32;
    for i in 1..=4 {
        let f = step_to_next_edge(image, &mut fwd, d, range - used);
        if f == 0 {
            return 0;
        }
        used += f;
        let b = step_to_next_edge(image, &mut bwd, back, range - used);
        if b == 0 {
            return 0;
        }
        used += b;
        view[4 + i] = f as u16;
        view[4 - i] = b as u16;
    }

    if !is_aztec_center_pattern(&view) {
        return 0;
    }

    if update {
        // the first step of `center_fwd` landed on the far side of the edge
        let shift = (center_fwd - center_bwd) / 2;
        center.0 += shift * d.0;
        center.1 += shift * d.1;
    }

    view[1..8].iter().map(|&v| v as i32).sum()
}

fn locate_aztec_center(image: &BitMatrix, p: Point, spread: i32) -> Option<BullseyeCandidate> {
    let mut c = (p.x as i32, p.y as i32);
    let mut min_spread = i32::MAX;
    let mut max_spread = 0;
    for (d, update) in [
        ((0, 1), true),
        ((1, 0), true),
        ((1, 1), false),
        ((1, -1), false),
    ] {
        let s = check_symmetric_aztec_center_pattern(image, &mut c, d, spread, update);
        if s == 0 {
            return None;
        }
        min_spread = min_spread.min(s);
        max_spread = max_spread.max(s);
    }
    Some(BullseyeCandidate {
        p: point(c.0 as f32 + 0.5, c.1 as f32 + 0.5),
        size: (min_spread + max_spread) / 2,
    })
}

/// Scans the image row by row for Aztec center (bullseye) patterns, like zxing-cpp's
/// `AZDetector`'s `FindFinderPatterns`.
pub fn find_bullseye_candidates(image: &BitMatrix) -> Vec<BullseyeCandidate> {
    let mut res: Vec<BullseyeCandidate> = Vec::new();
    let (width, height) = (image.getWidth(), image.getHeight());
    if width < 15 || height < 15 {
        return res;
    }

    let skip = (height / 2 / 100).clamp(1, 5);
    let margin = 1;

    let mut runs: Vec<u16> = Vec::with_capacity(width as usize / 2);
    let mut y = margin;
    while y < height - margin {
        // run lengths, always starting with a (possibly empty) white run
        runs.clear();
        let mut color = false;
        let mut count: u16 = 0;
        for x in 0..width {
            let c = image.get(x, y);
            if c != color {
                runs.push(count);
                count = 0;
                color = c;
            }
            count += 1;
        }
        runs.push(count);

        // the center pattern starts with white (an even index) and is preceded and followed by
        // black runs, so we need an odd-index black run in front
        if runs.len() < 10 {
            y += skip;
            continue;
        }
        let mut i = 2;
        let mut x0: u32 = runs[0] as u32 + runs[1] as u32;
        while i + 8 <= runs.len() {
            let mut view = [0u16; 9];
            view.copy_from_slice(&runs[i - 1..i + 8]);
            if is_aztec_center_pattern(&view) {
                let pattern_width: i32 = runs[i..i + 7].iter().map(|&v| v as i32).sum();
                let px = x0 as f32
                    + runs[i] as f32
                    + runs[i + 1] as f32
                    + runs[i + 2] as f32
                    + runs[i + 3] as f32 / 2.0;
                let p = point(px, y as f32 + 0.5);

                // make sure p is not 'inside' an already found pattern area
                let known = res
                    .iter()
                    .rev()
                    .any(|old| Point::distance(p, old.p) < old.size as f32 / 2.0);
                if !known
                    && let Some(c) = locate_aztec_center(image, p, pattern_width)
                    && !res
                        .iter()
                        .any(|old| Point::distance(c.p, old.p) < old.size as f32 / 2.0)
                {
                    res.push(c);
                }
            }
            x0 += runs[i] as u32 + runs[i + 1] as u32;
            i += 2;
        }

        y += skip;
    }

    res
}

/// Fits the actual (possibly rotated / perspective-distorted) corners of the bullseye found at
/// `candidate` and returns the candidate interpretations to try, each as `(compact, corners)`
/// where `corners` are the module centers just outside the bullseye in
/// [topRight, bottomRight, bottomLeft, topLeft] order (in the symbol's own, not yet
/// rotation-corrected, coordinate frame).
pub fn fitted_bullseye_corners(
    image: &BitMatrix,
    candidate: &BullseyeCandidate,
) -> Vec<(bool, [Point; 4])> {
    // Perspective transform from module coordinates (relative to the center module) to pixels,
    // fitted to the white ring `r` modules out from the center module.
    let fit = |r: i32, range: i32| -> Option<PerspectiveTransform> {
        let quad = FindConcentricPatternCorners(image, candidate.p, range, r)?;
        let r = r as f32;
        let src = Quadrilateral::new(point(-r, -r), point(r, -r), point(r, r), point(-r, r));
        PerspectiveTransform::quadrilateralToQuadrilateral(src, quad).ok()
    };
    // the module centers just outside a bullseye of the given radius
    let corners = |mod2pix: &PerspectiveTransform, radius: f32| {
        [
            mod2pix.transform_point(point(radius, -radius)),
            mod2pix.transform_point(point(radius, radius)),
            mod2pix.transform_point(point(-radius, radius)),
            mod2pix.transform_point(point(-radius, -radius)),
        ]
    };

    let mut res = Vec::new();
    let Some(inner) = fit(3, candidate.size) else {
        return res;
    };
    // compact symbol (bullseye radius 5)
    res.push((true, corners(&inner, 5.0)));
    // full symbol (bullseye radius 7): the white ring 5 modules out gives a larger, hence more
    // accurate, base for the perspective transform, if it's there
    if let Some(outer) = fit(5, candidate.size * 2) {
        res.push((false, corners(&outer, 7.0)));
    }
    res.push((false, corners(&inner, 7.0)));
    res
}
