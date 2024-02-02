/*
 * Copyright 2009 ZXing authors
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

use std::rc::Rc;

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel};
use imageproc::geometric_transformations::rotate_about_center;

use crate::common::Result;
use crate::LuminanceSource;

// const MINUS_45_IN_RADIANS: f32 = -0.7853981633974483; // Math.toRadians(-45.0)
const MINUS_45_IN_RADIANS: f32 = std::f32::consts::FRAC_PI_4;

/**
 * This LuminanceSource implementation is meant for J2SE clients and our blackbox unit tests.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author code@elektrowolle.de (Wolfgang Jung)
 */
pub struct BufferedImageLuminanceSource {
    // extends LuminanceSource {
    image: Rc<DynamicImage>,
    width: usize,
    height: usize,
    left: u32,
    top: u32,
}

impl BufferedImageLuminanceSource {
    pub fn new(image: DynamicImage) -> Self {
        let w = image.width();
        let h = image.height();
        Self::with_details(image, 0, 0, w as usize, h as usize)
    }

    pub fn with_details(
        image: DynamicImage,
        left: u32,
        top: u32,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            image: Rc::new(build_local_grey_image(image)),
            width,
            height,
            left,
            top,
        }
    }
}

impl LuminanceSource for BufferedImageLuminanceSource {
    fn get_row(&self, y: usize) -> Vec<u8> {
        let width = self.get_width(); // - self.left as usize;

        let pixels: Vec<u8> = || -> Option<Vec<u8>> {
            Some(
                self.image
                    .as_luma8()?
                    .rows()
                    .nth(y + self.top as usize)?
                    .skip(self.left as usize)
                    .take(width)
                    .map(|&p| p.0[0])
                    .collect(),
            )
        }()
        .unwrap_or_default();

        pixels
    }

    fn get_column(&self, x: usize) -> Vec<u8> {
        let width = self.get_height(); // - self.left as usize;

        let pixels: Vec<u8> = || -> Option<Vec<u8>> {
            Some(
                self.image
                    .as_luma8()?
                    .rows()
                    .skip(self.top as usize)
                    .fold(Vec::default(), |mut acc, e| {
                        acc.push(
                            e.into_iter()
                                .nth(self.left as usize + x)
                                .unwrap_or(&Luma([0_u8])),
                        );
                        acc
                    })
                    .iter()
                    .map(|&p| p.0[0])
                    .collect(),
            )
        }()
        .unwrap_or_default();

        pixels
    }

    fn get_matrix(&self) -> Vec<u8> {
        if self.height == self.image.height() as usize && self.width == self.image.width() as usize
        {
            return self.image.as_bytes().to_vec();
        }
        let skip = self.top * self.image.width();
        let row_skip = self.left;
        let total_row_take = self.width;
        let total_rows_to_take = self.image.width() * self.height as u32;

        let unmanaged = self
            .image
            .as_bytes()
            .iter()
            .skip(skip as usize) // get to the row we want
            .take(total_rows_to_take as usize)
            .collect::<Vec<&u8>>(); // get all the rows we want to look at

        let data = unmanaged
            .chunks_exact(self.image.width() as usize) // Get rows
            .flat_map(|f| {
                f.iter()
                    .skip(row_skip as usize)
                    .take(total_row_take)
                    .copied()
            }) // flatten this all out
            .copied() // copy it over so that it's u8
            .collect(); // collect into a vec

        data
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn is_crop_supported(&self) -> bool {
        true
    }

    fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> Result<Self> {
        Ok(Self {
            image: self.image.clone(),
            width,
            height,
            left: self.left + left as u32,
            top: self.top + top as u32,
        })
    }

    fn is_rotate_supported(&self) -> bool {
        true
    }

    fn invert(&mut self) {
        let mut img = (*self.image).clone();
        img.invert();
        self.image = Rc::new(img);
    }

    fn rotate_counter_clockwise(&self) -> Result<Self> {
        let img = self.image.rotate270();
        Ok(Self {
            width: img.width() as usize,
            height: img.height() as usize,
            image: Rc::new(img),
            left: 0,
            top: 0,
        })
    }

    fn rotate_counter_clockwise_45(&self) -> Result<Self> {
        let img = rotate_about_center(
            &self.image.to_luma8(),
            MINUS_45_IN_RADIANS,
            imageproc::geometric_transformations::Interpolation::Nearest,
            Luma([u8::MAX / 2; 1]),
        );

        let new_img = DynamicImage::from(img);

        Ok(Self {
            width: new_img.width() as usize,
            height: new_img.height() as usize,
            image: Rc::new(new_img),
            left: 0,
            top: 0,
        })
    }

    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        self.image.get_pixel(x as u32, y as u32).to_luma().0[0]
    }
}
