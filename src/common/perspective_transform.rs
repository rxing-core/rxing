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

use std::ops::Mul;

use crate::point;

use super::Quadrilateral;

/**
 * <p>This class implements a perspective transform in two dimensions. Given four source and four
 * destination points, it will compute the transformation implied between them. The code is based
 * directly upon section 3.4.2 of George Wolberg's "Digital Image Warping"; see pages 54-56.</p>
 *
 * @author Sean Owen
 */
pub struct PerspectiveTransform {
    a11: f32,
    a12: f32,
    a13: f32,
    a21: f32,
    a22: f32,
    a23: f32,
    a31: f32,
    a32: f32,
    a33: f32,
}

impl PerspectiveTransform {
    #[allow(clippy::too_many_arguments)]
    fn new(
        a11: f32,
        a21: f32,
        a31: f32,
        a12: f32,
        a22: f32,
        a32: f32,
        a13: f32,
        a23: f32,
        a33: f32,
    ) -> Self {
        Self {
            a11,
            a12,
            a13,
            a21,
            a22,
            a23,
            a31,
            a32,
            a33,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn quadrilateralToQuadrilateral(
        dst: Quadrilateral,
        src: Quadrilateral
    ) -> Self {

        // let q_to_s = PerspectiveTransform::quadrilateralToSquare(x0, y0, x1, y1, x2, y2, x3, y3);
        // let s_to_q =
        //     PerspectiveTransform::squareToQuadrilateral(x0p, y0p, x1p, y1p, x2p, y2p, x3p, y3p);

        let q_to_s = PerspectiveTransform::quadrilateralToSquare(dst);
        let s_to_q =
            PerspectiveTransform::squareToQuadrilateral(src);
        s_to_q * q_to_s
    }

    pub fn transform_points_single(&self, points: &mut [f32]) {
        let a11 = self.a11;
        let a12 = self.a12;
        let a13 = self.a13;
        let a21 = self.a21;
        let a22 = self.a22;
        let a23 = self.a23;
        let a31 = self.a31;
        let a32 = self.a32;
        let a33 = self.a33;
        let maxI = points.len() - 1; // points.length must be even
        let mut i = 0;
        while i < maxI {
            // for (int i = 0; i < maxI; i += 2) {
            let x = points[i];
            let y = points[i + 1];
            let denominator = a13 * x + a23 * y + a33;
            points[i] = (a11 * x + a21 * y + a31) / denominator;
            points[i + 1] = (a12 * x + a22 * y + a32) / denominator;
            i += 2;
        }
    }

    pub fn transform_points_double(&self, x_values: &mut [f32], y_valuess: &mut [f32]) {
        let n = x_values.len();
        // for i in 0..n {
        for (x, y) in x_values.iter_mut().zip(y_valuess.iter_mut()).take(n) {
            // for (int i = 0; i < n; i++) {
            let denominator = self.a13 * *x + self.a23 * *y + self.a33;
            *x = (self.a11 * *x + self.a21 * *y + self.a31) / denominator;
            *y = (self.a12 * *x + self.a22 * *y + self.a32) / denominator;
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn squareToQuadrilateral(
        square: Quadrilateral
    ) -> Self {
        let [p0, p1, p2, p3 ] = square.0;
        
        let d3 = p0 - p1 + p2 - p3;
        if d3 == point(0.0,0.0) {
            // Affine
            PerspectiveTransform::new(p1.x - p0.x, p2.x - p1.x, p0.x, p1.y - p0.y, p2.y - p1.y, p0.y, 0.0, 0.0, 1.0)
        } else {

let d1 = p1 - p2;
let d2 = p3 - p2;

            let denominator = d1.cross(d2);
            let a13 = (d3.x * d2.y - d2.x * d3.y) / denominator;
            let a23 = (d1.x * d3.y - d3.x * d1.y) / denominator;
            PerspectiveTransform::new(
                p1.x - p0.x + a13 * p1.x,
                p3.x - p0.x + a23 * p3.x,
                p0.x,
                p1.y - p0.y + a13 * p1.y,
                p3.y - p0.y + a23 * p3.y,
                p0.y,
                a13,
                a23,
                1.0,
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn quadrilateralToSquare(
        quad: Quadrilateral
    ) -> Self {
        // Here, the adjoint serves as the inverse
        PerspectiveTransform::squareToQuadrilateral(quad).buildAdjoint()
    }

    fn buildAdjoint(&self) -> Self {
        // Adjoint is the transpose of the cofactor matrix:
        PerspectiveTransform::new(
            self.a22 * self.a33 - self.a23 * self.a32,
            self.a23 * self.a31 - self.a21 * self.a33,
            self.a21 * self.a32 - self.a22 * self.a31,
            self.a13 * self.a32 - self.a12 * self.a33,
            self.a11 * self.a33 - self.a13 * self.a31,
            self.a12 * self.a31 - self.a11 * self.a32,
            self.a12 * self.a23 - self.a13 * self.a22,
            self.a13 * self.a21 - self.a11 * self.a23,
            self.a11 * self.a22 - self.a12 * self.a21,
        )
    }
}

impl Mul for PerspectiveTransform {
    type Output = PerspectiveTransform;

    fn mul(self, rhs: Self) -> Self::Output {
        PerspectiveTransform::new(
            self.a11 * rhs.a11 + self.a21 * rhs.a12 + self.a31 * rhs.a13,
            self.a11 * rhs.a21 + self.a21 * rhs.a22 + self.a31 * rhs.a23,
            self.a11 * rhs.a31 + self.a21 * rhs.a32 + self.a31 * rhs.a33,
            self.a12 * rhs.a11 + self.a22 * rhs.a12 + self.a32 * rhs.a13,
            self.a12 * rhs.a21 + self.a22 * rhs.a22 + self.a32 * rhs.a23,
            self.a12 * rhs.a31 + self.a22 * rhs.a32 + self.a32 * rhs.a33,
            self.a13 * rhs.a11 + self.a23 * rhs.a12 + self.a33 * rhs.a13,
            self.a13 * rhs.a21 + self.a23 * rhs.a22 + self.a33 * rhs.a23,
            self.a13 * rhs.a31 + self.a23 * rhs.a32 + self.a33 * rhs.a33,
        )
    }
}