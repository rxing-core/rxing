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
 * @author Sean Owen
 */
pub struct DefaultGridSampler {
    super: GridSampler;
}

impl DefaultGridSampler {

    pub fn  sample_grid(&self,  image: &BitMatrix,  dimension_x: i32,  dimension_y: i32,  p1_to_x: f32,  p1_to_y: f32,  p2_to_x: f32,  p2_to_y: f32,  p3_to_x: f32,  p3_to_y: f32,  p4_to_x: f32,  p4_to_y: f32,  p1_from_x: f32,  p1_from_y: f32,  p2_from_x: f32,  p2_from_y: f32,  p3_from_x: f32,  p3_from_y: f32,  p4_from_x: f32,  p4_from_y: f32) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
         let transform: PerspectiveTransform = PerspectiveTransform::quadrilateral_to_quadrilateral(p1_to_x, p1_to_y, p2_to_x, p2_to_y, p3_to_x, p3_to_y, p4_to_x, p4_to_y, p1_from_x, p1_from_y, p2_from_x, p2_from_y, p3_from_x, p3_from_y, p4_from_x, p4_from_y);
        return Ok(self.sample_grid(image, dimension_x, dimension_y, transform));
    }

    pub fn  sample_grid(&self,  image: &BitMatrix,  dimension_x: i32,  dimension_y: i32,  transform: &PerspectiveTransform) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
        if dimension_x <= 0 || dimension_y <= 0 {
            throw NotFoundException::get_not_found_instance();
        }
         let bits: BitMatrix = BitMatrix::new(dimension_x, dimension_y);
         let mut points: [f32; 2.0 * dimension_x] = [0.0; 2.0 * dimension_x];
         {
             let mut y: i32 = 0;
            while y < dimension_y {
                {
                     let max: i32 = points.len();
                     let i_value: f32 = y + 0.5f;
                     {
                         let mut x: i32 = 0;
                        while x < max {
                            {
                                points[x] = (x / 2.0) as f32 + 0.5f;
                                points[x + 1] = i_value;
                            }
                            x += 2;
                         }
                     }

                    transform.transform_points(&points);
                    // Quick check to see if points transformed to something inside the image;
                    // sufficient to check the endpoints
                    check_and_nudge_points(image, &points);
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                         {
                             let mut x: i32 = 0;
                            while x < max {
                                {
                                    if image.get(points[x] as i32, points[x + 1] as i32) {
                                        // Black(-ish) pixel
                                        bits.set(x / 2, y);
                                    }
                                }
                                x += 2;
                             }
                         }

                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( aioobe: &ArrayIndexOutOfBoundsException) {
                            throw NotFoundException::get_not_found_instance();
                        }  0 => break
                    }

                }
                y += 1;
             }
         }

        return Ok(bits);
    }
}

