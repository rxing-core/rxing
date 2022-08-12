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
// package com::google::zxing::common;

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

 let grid_sampler: GridSampler = DefaultGridSampler::new();
pub struct GridSampler {
}

impl GridSampler {

    /**
   * Sets the implementation of GridSampler used by the library. One global
   * instance is stored, which may sound problematic. But, the implementation provided
   * ought to be appropriate for the entire platform, and all uses of this library
   * in the whole lifetime of the JVM. For instance, an Android activity can swap in
   * an implementation that takes advantage of native platform libraries.
   *
   * @param newGridSampler The platform-specific object to install.
   */
    pub fn  set_grid_sampler( new_grid_sampler: &GridSampler)   {
        grid_sampler = new_grid_sampler;
    }

    /**
   * @return the current implementation of GridSampler
   */
    pub fn  get_instance() -> GridSampler  {
        return grid_sampler;
    }

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
    pub fn  sample_grid(&self,  image: &BitMatrix,  dimension_x: i32,  dimension_y: i32,  p1_to_x: f32,  p1_to_y: f32,  p2_to_x: f32,  p2_to_y: f32,  p3_to_x: f32,  p3_to_y: f32,  p4_to_x: f32,  p4_to_y: f32,  p1_from_x: f32,  p1_from_y: f32,  p2_from_x: f32,  p2_from_y: f32,  p3_from_x: f32,  p3_from_y: f32,  p4_from_x: f32,  p4_from_y: f32) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>  ;

    pub fn  sample_grid(&self,  image: &BitMatrix,  dimension_x: i32,  dimension_y: i32,  transform: &PerspectiveTransform) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>  ;

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
    pub fn  check_and_nudge_points( image: &BitMatrix,  points: &Vec<f32>)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         let width: i32 = image.get_width();
         let height: i32 = image.get_height();
        // Check and nudge points from start until we see some that are OK:
         let mut nudged: bool = true;
        // points.length must be even
         let max_offset: i32 = points.len() - 1;
         {
             let mut offset: i32 = 0;
            while offset < max_offset && nudged {
                {
                     let x: i32 = points[offset] as i32;
                     let y: i32 = points[offset + 1] as i32;
                    if x < -1 || x > width || y < -1 || y > height {
                        throw NotFoundException::get_not_found_instance();
                    }
                    nudged = false;
                    if x == -1 {
                        points[offset] = 0.0f;
                        nudged = true;
                    } else if x == width {
                        points[offset] = width - 1.0;
                        nudged = true;
                    }
                    if y == -1 {
                        points[offset + 1] = 0.0f;
                        nudged = true;
                    } else if y == height {
                        points[offset + 1] = height - 1.0;
                        nudged = true;
                    }
                }
                offset += 2;
             }
         }

        // Check and nudge points from end:
        nudged = true;
         {
             let mut offset: i32 = points.len() - 2;
            while offset >= 0 && nudged {
                {
                     let x: i32 = points[offset] as i32;
                     let y: i32 = points[offset + 1] as i32;
                    if x < -1 || x > width || y < -1 || y > height {
                        throw NotFoundException::get_not_found_instance();
                    }
                    nudged = false;
                    if x == -1 {
                        points[offset] = 0.0f;
                        nudged = true;
                    } else if x == width {
                        points[offset] = width - 1.0;
                        nudged = true;
                    }
                    if y == -1 {
                        points[offset + 1] = 0.0f;
                        nudged = true;
                    } else if y == height {
                        points[offset + 1] = height - 1.0;
                        nudged = true;
                    }
                }
                offset -= 2;
             }
         }

    }
}

