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

use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{BitMatrix, DefaultGridSampler, GridSampler, Result},
    datamatrix::detector::{
        zxing_cpp_detector::{util::intersect, BitMatrixCursor, Quadrilateral, RegressionLine},
        DatamatrixDetectorResult,
    },
    qrcode::encoder::ByteMatrix,
    result_point_utils::distance,
    Exceptions, Point, ResultPoint,
};

use super::{DMRegressionLine, EdgeTracer};

/**
* The following code is the 'new' one implemented by Axel Waggershauser and is working completely different.
* It is performing something like a (back) trace search along edges through the bit matrix, first looking for
* the 'L'-pattern, then tracing the black/white borders at the top/right. Advantages over the old code are:
*  * works with lower resolution scans (around 2 pixel per module), due to sub-pixel precision grid placement
*  * works with real-world codes that have just one module wide quiet-zone (which is perfectly in spec)
*/

fn Scan(
    startTracer: &mut EdgeTracer,
    lines: &mut [DMRegressionLine; 4],
) -> Result<DatamatrixDetectorResult> {
    while startTracer.step(None) {
        //log(startTracer.p);

        // continue until we cross from black into white
        if !startTracer.edgeAtBack().isWhite() {
            continue;
        }

        let mut tl = Point::default();
        let mut bl = Point::default();
        let mut br = Point::default();
        let mut tr = Point::default();

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
        CHECK!(t.traceLine(t.right(), lineL)?);
        CHECK!(t.traceCorner(&mut t.right(), &mut tl)?);
        lineL.reverse();
        let mut tlTracer = t;

        // follow left leg downwards
        t = startTracer.clone();
        t.state = 1;
        t.setDirection(tlTracer.right());
        CHECK!(t.traceLine(t.left(), lineL)?);
        if !lineL.isValid() {
            t.updateDirectionFromOrigin(tl);
        }
        let up = t.back();
        CHECK!(t.traceCorner(&mut t.left(), &mut bl)?);

        // follow bottom leg right
        t.state = 2;
        CHECK!(t.traceLine(t.left(), lineB)?);
        if !lineB.isValid() {
            t.updateDirectionFromOrigin(bl);
        }
        let right = *t.front();
        CHECK!(t.traceCorner(&mut t.left(), &mut br)?);

        let lenL = distance(&tl, &bl) - 1.0;
        let lenB = distance(&bl, &br) - 1.0;
        CHECK!(lenL >= 8.0 && lenB >= 10.0 && lenB >= lenL / 4.0 && lenB <= lenL * 18.0);

        let mut maxStepSize: i32 = (lenB / 5.0 + 1.0) as i32; // datamatrix bottom dim is at least 10

        // at this point we found a plausible L-shape and are now looking for the b/w pattern at the top and right:
        // follow top row right 'half way' (4 gaps), see traceGaps break condition with 'invalid' line
        tlTracer.setDirection(right);
        CHECK!(tlTracer.traceGaps(
            tlTracer.right(),
            lineT,
            maxStepSize,
            &mut DMRegressionLine::default()
        )?);

        // let a = lineT.length() as i32 / 3;
        // let b = (lenL / 5.0) as i32;

        // maxStepSize = std::cmp::min(a,  b) * 2;
        maxStepSize = std::cmp::min(lineT.length() as i32 / 3, (lenL / 5.0) as i32) * 2;

        // follow up until we reach the top line
        t.setDirection(up);
        t.state = 3;
        CHECK!(t.traceGaps(t.left(), lineR, maxStepSize, lineT)?);
        CHECK!(t.traceCorner(&mut t.left(), &mut tr)?);

        let lenT = distance(&tl, &tr) - 1.0;
        let lenR = distance(&tr, &br) - 1.0;

        CHECK!(
            (lenT - lenB).abs() / lenB < 0.5
                && (lenR - lenL).abs() / lenL < 0.5
                && lineT.points().len() >= 5
                && lineR.points().len() >= 5
        );

        // continue top row right until we cross the right line
        CHECK!(tlTracer.traceGaps(tlTracer.right(), lineT, maxStepSize, lineR)?);

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
        bl = intersect(lineB, lineL)?;
        tl = intersect(lineT, lineL)?;
        tr = intersect(lineT, lineR)?;
        br = intersect(lineB, lineR)?;

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
        splitDouble(lineT.modules(tl, tr)?, &mut dimT, &mut fracT);
        splitDouble(lineR.modules(br, tr)?, &mut dimR, &mut fracR);

        // #ifdef PRINT_DEBUG
        // 		printf("L: %.1f, %.1f ^ %.1f, %.1f > %.1f, %.1f ^> %.1f, %.1f\n", bl.x, bl.y,
        // 			   tl.x - bl.x, tl.y - bl.y, br.x - bl.x, br.y - bl.y, tr.x, tr.y);
        // 		printf("dim: %d x %d\n", dimT, dimR);
        // #endif

        // if we have an almost square (invalid rectangular) data matrix dimension, we try to parse it by assuming a
        // square. we use the dimension that is closer to an integral value. all valid rectangular symbols differ in
        // their dimension by at least 10 (here 5, see doubling below). Note: this is currently not required for the
        // black-box tests to complete.
        if (dimT - dimR).abs() < 5 {
            dimR = if fracR < fracT { dimR } else { dimT };
            dimT = dimR;
        }

        // the dimension is 2x the number of black/white transitions
        dimT *= 2;
        dimR *= 2;

        CHECK!((10..=144).contains(&dimT) && (8..=144).contains(&dimR));

        let movedTowardsBy = |a: Point, b1: Point, b2: Point, d: f32| -> Point {
            a + d * Point::normalized(Point::normalized(b1 - a) + Point::normalized(b2 - a))
        };

        // shrink shape by half a pixel to go from center of white pixel outside of code to the edge between white and black
        let sourcePoints = Quadrilateral::with_points(
            movedTowardsBy(tl, tr, bl, 0.5),
            // move the tr point a little less because the jagged top and right line tend to be statistically slightly
            // inclined toward the center anyway.
            movedTowardsBy(tr, br, tl, 0.3),
            movedTowardsBy(br, bl, tr, 0.5),
            movedTowardsBy(bl, tl, br, 0.5),
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

        CHECK!(res.is_ok());

        return Ok(DatamatrixDetectorResult::new(
            res?,
            sourcePoints.points().to_vec(),
        ));
    }

    Err(Exceptions::NotFoundException(None))
}

pub fn detect(
    image: &BitMatrix,
    tryHarder: bool,
    tryRotate: bool,
) -> Result<DatamatrixDetectorResult> {
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
    if tryHarder {
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

    const MIN_SYMBOL_SIZE: u32 = 8 * 2; // minimum realistic size in pixel: 8 modules x 2 pixels per module

    for dir in [
        Point { x: -1.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.0, y: -1.0 },
        Point { x: 0.0, y: 1.0 },
    ] {
        // for (auto dir : {PointF(-1, 0), PointF(1, 0), PointF(0, -1), PointF(0, 1)}) {
        let center = Point {
            x: (image.getWidth() / 2) as f32,
            y: (image.getHeight() / 2) as f32,
        }; //PointF(image.width() / 2, image.height() / 2);
        let startPos = Point::centered(center - center * dir + MIN_SYMBOL_SIZE as i32 / 2 * dir);

        if let Some(history) = &mut history {
            history.borrow_mut().clear(0);
            // history.clear(0);
        }

        let mut i = 1;
        loop {
            // for (int i = 1;; ++i) {
            // EdgeTracer  tracer(image, startPos, dir);
            let mut tracer = EdgeTracer::new(image, startPos, dir);
            tracer.p += i / 2
                * MIN_SYMBOL_SIZE as i32
                * (if (i & 1) != 0 { -1 } else { 1 })
                * tracer.right();
            if tryHarder {
                // tracer.history = history.as_mut();
                tracer.history = history.clone();
                // if let Some(history) = &history {
                // 	tracer.history = history;
                // }
                // tracer.history = &history;
            }

            if !tracer.isInSelf() {
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

            if !tryHarder {
                break;
            } // only test center lines
            i += 1;
        }

        if !tryRotate {
            break;
        } // only test left direction
    }

    // #ifndef __cpp_impl_coroutine
    Err(Exceptions::NotFoundException(None))
    // #endif
}
