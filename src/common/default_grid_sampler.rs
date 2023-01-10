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

// import com.google.zxing.NotFoundException;

use crate::Exceptions;

use super::{BitMatrix, GridSampler, PerspectiveTransform};

/**
 * @author Sean Owen
 */
#[derive(Default)]
pub struct DefaultGridSampler;

impl GridSampler for DefaultGridSampler {
    fn sample_grid_detailed(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        p1ToX: f32,
        p1ToY: f32,
        p2ToX: f32,
        p2ToY: f32,
        p3ToX: f32,
        p3ToY: f32,
        p4ToX: f32,
        p4ToY: f32,
        p1FromX: f32,
        p1FromY: f32,
        p2FromX: f32,
        p2FromY: f32,
        p3FromX: f32,
        p3FromY: f32,
        p4FromX: f32,
        p4FromY: f32,
    ) -> Result<BitMatrix, Exceptions> {
        let transform = PerspectiveTransform::quadrilateralToQuadrilateral(
            p1ToX, p1ToY, p2ToX, p2ToY, p3ToX, p3ToY, p4ToX, p4ToY, p1FromX, p1FromY, p2FromX,
            p2FromY, p3FromX, p3FromY, p4FromX, p4FromY,
        );

        self.sample_grid(image, dimensionX, dimensionY, &transform)
    }

    fn sample_grid(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        transform: &PerspectiveTransform,
    ) -> Result<BitMatrix, Exceptions> {
        if dimensionX == 0 || dimensionY == 0 {
            return Err(Exceptions::NotFoundException(None));
        }
        let mut bits = BitMatrix::new(dimensionX, dimensionY)?;
        let mut points = vec![0_f32; 2 * dimensionX as usize];
        for y in 0..dimensionY {
            //   for (int y = 0; y < dimensionY; y++) {
            let max = points.len();
            let i_value = y as f32 + 0.5f32;
            let mut x = 0;
            while x < max {
                // for (int x = 0; x < max; x += 2) {
                points[x] = (x as f32 / 2.0) + 0.5f32;
                points[x + 1] = i_value;
                x += 2;
            }
            transform.transform_points_single(&mut points);
            // Quick check to see if points transformed to something inside the image;
            // sufficient to check the endpoints
            self.checkAndNudgePoints(image, &mut points)?;
            // try {
            let mut x = 0;
            while x < max {
                //   for (int x = 0; x < max; x += 2) {
                if points[x].floor() as u32 >= image.getWidth()
                    || points[x + 1].floor() as u32 >= image.getHeight()
                {
                    return Err(Exceptions::NotFoundException(Some(
                        "index out of bounds, see documentation in file for explanation".to_owned(),
                    )));
                }
                if image.get(points[x].floor() as u32, points[x + 1].floor() as u32) {
                    // Black(-ish) pixel
                    bits.set(x as u32 / 2, y);
                }
                x += 2;
            }
            // } catch (ArrayIndexOutOfBoundsException aioobe) {
            //   // This feels wrong, but, sometimes if the finder patterns are misidentified, the resulting
            //   // transform gets "twisted" such that it maps a straight line of points to a set of points
            //   // whose endpoints are in bounds, but others are not. There is probably some mathematical
            //   // way to detect this about the transformation that I don't know yet.
            //   // This results in an ugly runtime exception despite our clever checks above -- can't have
            //   // that. We could check each point's coordinates but that feels duplicative. We settle for
            //   // catching and wrapping ArrayIndexOutOfBoundsException.
            //   throw NotFoundException.getNotFoundInstance();
            // }
        }
        Ok(bits)
    }
}
