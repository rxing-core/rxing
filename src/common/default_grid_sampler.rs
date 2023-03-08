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

use crate::common::Result;
use crate::{point, Exceptions, Point};

use super::{BitMatrix, GridSampler, SamplerControl};

/**
 * @author Sean Owen
 */
#[derive(Default)]
pub struct DefaultGridSampler;

impl GridSampler for DefaultGridSampler {
    fn sample_grid(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        controls: &[SamplerControl],
    ) -> Result<BitMatrix> {
        if dimensionX <= 0 || dimensionY <= 0 {
            return Err(Exceptions::NOT_FOUND);
        }

        for SamplerControl { p0, p1, transform } in controls {
            // To deal with remaining examples (see #251 and #267) of "numercial instabilities" that have not been
            // prevented with the Quadrilateral.h:IsConvex() check, we check for all boundary points of the grid to
            // be inside.
            let isInside = |p: Point| -> bool {
                return image.is_in(transform.transform_point(p.centered()));
            };
            for y in (p0.y as i32)..(p1.y as i32) {
                // for (int y = y0; y < y1; ++y)
                if !isInside(point(p0.x, y as f32)) || !isInside(point(p1.x - 1.0, y as f32)) {
                    return Err(Exceptions::NOT_FOUND);
                }
            }
            for x in (p0.x as i32)..(p1.x as i32) {
                // for (int x = x0; x < x1; ++x)
                if !isInside(point(x as f32, p0.y)) || !isInside(point(x as f32, p1.y - 1.0)) {
                    return Err(Exceptions::NOT_FOUND);
                }
            }
        }

        let mut bits = BitMatrix::new(dimensionX, dimensionY)?;
        for SamplerControl { p0, p1, transform } in controls {
            // for (auto&& [x0, x1, y0, y1, mod2Pix] : rois) {
            for y in (p0.y as i32)..(p1.y as i32) {
                // for (int y = y0; y < y1; ++y)
                for x in (p0.x as i32)..(p1.x as i32) {
                    // for (int x = x0; x < x1; ++x) {
                    let p = transform.transform_point(Point::from((x, y)).centered()); //mod2Pix(centered(PointI{x, y}));

                    if image.get_point(p) {
                        bits.set(x as u32, y as u32);
                    }
                }
            }
        }

        // dbg!(image.to_string());
        // dbg!(bits.to_string());

        let projectCorner = |p: Point| -> Point {
            for SamplerControl { p0, p1, transform } in controls {
                if p0.x <= p.x && p.x <= p1.x && p0.y <= p.y && p.y <= p1.y {
                    return transform.transform_point(p) + point(0.5, 0.5);
                }
            }
            Point::default()
        };

        let _tl = projectCorner(point(0.0, 0.0));
        let _tr = projectCorner(Point::from((dimensionX, 0)));
        let _bl = projectCorner(Point::from((dimensionX, dimensionY)));
        let _br = projectCorner(Point::from((0, dimensionX)));

        Ok(bits)
    }
}
