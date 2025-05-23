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

use crate::{common::Result, Point};
use crate::{point, Exceptions};

use super::{BitMatrix, PerspectiveTransform, Quadrilateral};

/**
 * Implementations of this class can, given locations of finder patterns for a QR code in an
 * image, sample the right points in the image to reconstruct the QR code, accounting for
 * perspective distortion. It is abstracted since it is relatively expensive and should be allowed
 * to take advantage of platform-specific optimized implementations, like Sun's Java Advanced
 * Imaging library, but which may not be available in other environments such as J2ME, and vice
 * versa.
 *
 * The implementation used can be controlled by calling {@link #setGridSampler(GridSampler)}
 * with an instance of a class which implements this interface.
 *
 * @author Sean Owen
 */
pub trait GridSampler {
    //   /**
    //    * Sets the implementation of GridSampler used by the library. One global
    //    * instance is stored, which may sound problematic. But, the implementation provided
    //    * ought to be appropriate for the entire platform, and all uses of this library
    //    * in the whole lifetime of the JVM. For instance, an Android activity can swap in
    //    * an implementation that takes advantage of native platform libraries.
    //    *
    //    * @param newGridSampler The platform-specific object to install.
    //    */
    //   public static void setGridSampler(GridSampler newGridSampler) {
    //     gridSampler = newGridSampler;
    //   }

    //   /**
    //    * @return the current implementation of GridSampler
    //    */
    //   public static GridSampler getInstance() {
    //     return gridSampler;
    //   }

    /**
     * Samples an image for a rectangular matrix of bits of the given dimension. The sampling
     * transformation is determined by the coordinates of 4 points, in the original and transformed
     * image space.
     *
     * @param image image to sample
     * @param dimensionX width of {@link BitMatrix} to sample from image
     * @param dimensionY height of {@link BitMatrix} to sample from image
     * @param p1ToX point 1 preimage X
     * @param p1ToY point 1 preimage Y
     * @param p2ToX point 2 preimage X
     * @param p2ToY point 2 preimage Y
     * @param p3ToX point 3 preimage X
     * @param p3ToY point 3 preimage Y
     * @param p4ToX point 4 preimage X
     * @param p4ToY point 4 preimage Y
     * @param p1FromX point 1 image X
     * @param p1FromY point 1 image Y
     * @param p2FromX point 2 image X
     * @param p2FromY point 2 image Y
     * @param p3FromX point 3 image X
     * @param p3FromY point 3 image Y
     * @param p4FromX point 4 image X
     * @param p4FromY point 4 image Y
     * @return {@link BitMatrix} representing a grid of points sampled from the image within a region
     *   defined by the "from" parameters
     * @throws NotFoundException if image can't be sampled, for example, if the transformation defined
     *   by the given points is invalid or results in sampling outside the image boundaries
     */
    #[allow(clippy::too_many_arguments)]
    fn sample_grid_detailed(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        dst: Quadrilateral,
        src: Quadrilateral,
    ) -> Result<(BitMatrix, [Point; 4])> {
        let transform = PerspectiveTransform::quadrilateralToQuadrilateral(dst, src)?;

        self.sample_grid(
            image,
            dimensionX,
            dimensionY,
            &[SamplerControl::new(dimensionX, dimensionY, transform)],
        )
    }

    fn sample_grid(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        controls: &[SamplerControl],
    ) -> Result<(BitMatrix, [Point; 4])> {
        if dimensionX == 0 || dimensionY == 0 {
            return Err(Exceptions::NOT_FOUND);
        }
        let mut bits = BitMatrix::new(dimensionX, dimensionY)?;
        let mut points = vec![Point::default(); dimensionX as usize];
        for y in 0..dimensionY {
            //   for (int y = 0; y < dimensionY; y++) {
            let i_value = y as f32 + 0.5;

            for (x, point) in points.iter_mut().enumerate() {
                point.x = (x as f32) + 0.5;
                point.y = i_value;
            }

            controls
                .iter()
                .for_each(|control| control.transform.transform_points_single(&mut points));
            // controls
            //     .first()
            //     .unwrap()
            //     .transform
            //     .transform_points_single(&mut points);
            // Quick check to see if points transformed to something inside the image;
            // sufficient to check the endpoints
            self.checkAndNudgePoints(image, &mut points)?;
            // try {
            for (x, point) in points.iter().enumerate() {
                // for x in 0..points.len() {
                if image.try_get(point.x as u32, point.y as u32).ok_or(
                    Exceptions::not_found_with(
                        "index out of bounds, see documentation in file for explanation",
                    ),
                )? {
                    // Black(-ish) pixel
                    bits.set(x as u32, y);
                }
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
        // dbg!(bits.to_string());

        Ok((bits, [Point::default(); 4]))
    }

    /**
     * <p>Checks a set of points that have been transformed to sample points on an image against
     * the image's dimensions to see if the point are even within the image.</p>
     *
     * <p>This method will actually "nudge" the endpoints back onto the image if they are found to be
     * barely (less than 1 pixel) off the image. This accounts for imperfect detection of finder
     * patterns in an image where the QR Code runs all the way to the image border.</p>
     *
     * <p>For efficiency, the method will check points from either end of the line until one is found
     * to be within the image. Because the set of points are assumed to be linear, this is valid.</p>
     *
     * @param image image into which the points should map
     * @param points actual points in x1,y1,...,xn,yn form
     * @throws NotFoundException if an endpoint is lies outside the image boundaries
     */
    fn checkAndNudgePoints(&self, image: &BitMatrix, points: &mut [Point]) -> Result<()> {
        let width = image.getWidth();
        let height = image.getHeight();
        // Check and nudge points from start until we see some that are OK:
        let mut nudged;
        let max_offset = points.len() - 1; // points.length must be even
        for point in points.iter_mut().take(max_offset) {
            let (x, y) = (point.x as i32, point.y as i32);
            if x < -1 || x > width as i32 || y < -1 || y > height as i32 {
                return Err(Exceptions::NOT_FOUND);
            }
            nudged = false;
            if x == -1 {
                point.x = 0.0;
                nudged = true;
            } else if x == width as i32 {
                point.x = width as f32 - 1.0;
                nudged = true;
            }
            if y == -1 {
                point.y = 0.0;
                nudged = true;
            } else if y == height as i32 {
                point.y = height as f32 - 1.0;
                nudged = true;
            }
            if !nudged {
                break;
            }
        }
        // Check and nudge points from end:
        for point in points.iter_mut().rev().take(max_offset).rev() {
            let (x, y) = (point.x as i32, point.y as i32);
            if x < -1 || x > width as i32 || y < -1 || y > height as i32 {
                return Err(Exceptions::NOT_FOUND);
            }
            nudged = false;
            if x == -1 {
                point.x = 0.0;
                nudged = true;
            } else if x == width as i32 {
                point.x = width as f32 - 1.0;
                nudged = true;
            }
            if y == -1 {
                point.y = 0.0;
                nudged = true;
            } else if y == height as i32 {
                point.y = height as f32 - 1.0;
                nudged = true;
            }
            if !nudged {
                break;
            }
        }
        Ok(())
    }
}

pub struct SamplerControl {
    pub p0: Point,
    pub p1: Point,
    pub transform: PerspectiveTransform,
}

impl SamplerControl {
    pub fn new(width: u32, height: u32, transform: PerspectiveTransform) -> Self {
        Self {
            p0: point(0.0, 0.0),
            p1: point(width as f32, height as f32),
            transform,
        }
    }
}
