/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// package com.google.zxing.common;

// import org.junit.Assert;
// import org.junit.Test;

use crate::point_f;

/**
 * @author Sean Owen
 */
// public final class PerspectiveTransformTestCase extends Assert {
use super::{PerspectiveTransform, Quadrilateral};

static EPSILON: f32 = 1.0E-4_f32;

#[test]
fn test_square_to_quadrilateral() {
    let square = Quadrilateral::new(
        point_f(2.0, 3.0),
        point_f(10.0, 3.0),
        point_f(16.0, 15.0),
        point_f(4.0, 9.0),
    );
    let pt = PerspectiveTransform::squareToQuadrilateral(square);
    assert_point_equals(2.0, 3.0, 0.0, 0.0, &pt);
    assert_point_equals(10.0, 4.0, 1.0, 0.0, &pt);
    assert_point_equals(4.0, 9.0, 0.0, 1.0, &pt);
    assert_point_equals(16.0, 15.0, 1.0, 1.0, &pt);
    assert_point_equals(6.535211, 6.8873234, 0.5, 0.5, &pt);
    assert_point_equals(48.0, 42.42857, 1.5, 1.5, &pt);
}

#[test]
fn test_quadrilateral_to_quadrilateral() {
    let quad1 = Quadrilateral::new(
        point_f(2.0, 3.0),
        point_f(10.0, 4.0),
        point_f(16.0, 15.0),
        point_f(4.0, 9.0),
    );
    let quad2 = Quadrilateral::new(
        point_f(103.0, 110.0),
        point_f(300.0, 120.0),
        point_f(290.0, 270.0),
        point_f(150.0, 280.0),
    );
    let pt = PerspectiveTransform::quadrilateralToQuadrilateral(quad1, quad2).expect("transform");

    assert_point_equals(103.0, 110.0, 2.0, 3.0, &pt);
    assert_point_equals(300.0, 120.0, 10.0, 4.0, &pt);
    assert_point_equals(290.0, 270.0, 16.0, 15.0, &pt);
    assert_point_equals(150.0, 280.0, 4.0, 9.0, &pt);
    assert_point_equals(7.1516876, -64.60185, 0.5, 0.5, &pt);
    assert_point_equals(328.09116, 334.16385, 50.0, 50.0, &pt);
}

fn assert_point_equals(
    expected_x: f32,
    expected_y: f32,
    source_x: f32,
    source_y: f32,
    pt: &PerspectiveTransform,
) {
    let mut points = [point_f(source_x, source_y)];
    pt.transform_points_single(&mut points);
    assert!(
        (expected_x - points[0].x < EPSILON || points[0].x - expected_x < EPSILON),
        "{} - {}",
        expected_x,
        points[0].x
    );
    assert!(
        (expected_y - points[0].y < EPSILON || points[0].y - expected_y < EPSILON),
        "{} - {}",
        expected_y,
        points[0].y
    );

    // assert_eq!( expectedX, points[0]);
    // assert_eq!( expectedY, points[1]);
}
