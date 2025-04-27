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

use std::borrow::Cow;

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
    image: DynamicImage,
    width: usize,
    height: usize,
}

impl BufferedImageLuminanceSource {
    pub fn new(image: DynamicImage) -> Self {
        let width = image.width() as usize;
        let height = image.height() as usize;
        // Self::with_details(image, 0, 0, w as usize, h as usize)
        Self {
            image: build_local_grey_image(image),
            width,
            height,
        }
    }
}

impl LuminanceSource for BufferedImageLuminanceSource {
    const SUPPORTS_CROP: bool = true;
    const SUPPORTS_ROTATION: bool = true;

    fn get_row(&self, y: usize) -> Option<Cow<[u8]>> {
        let buf = self.image.as_luma8()?;

        let width = self.get_width();
        let stride = buf.width() as usize; // full row length in pixels
        let start = y
            .checked_mul(stride) // guard against overflow
            .and_then(|off| off.checked_add(0))
            .unwrap_or(0);

        // Make sure we don’t go past the end
        if start + width > buf.as_raw().len() {
            return None;
        }

        // Copy the exact sub-slice in one memcpy
        Some(Cow::Borrowed(&buf.as_raw()[start..start + width]))
    }

    fn get_column(&self, x: usize) -> Vec<u8> {
        let pixels: Vec<u8> = || -> Option<Vec<u8>> {
            Some(self.image.as_luma8()?.rows().fold(
                Vec::with_capacity(self.get_height()),
                |mut acc, e| {
                    let pix = e.into_iter().nth(x).unwrap_or(&Luma([0_u8]));
                    acc.push(pix.0[0]);
                    acc
                },
            ))
        }()
        .unwrap_or_default();

        pixels
    }

    fn get_matrix(&self) -> Vec<u8> {
        // if self.height == self.image.height() as usize && self.width == self.image.width() as usize
        // {
        self.image.as_bytes().to_vec()
        // }
        // let skip = self.image.width();
        // let row_skip = 0;
        // let total_row_take = self.width;
        // let total_rows_to_take = self.image.width() * self.height as u32;

        // let unmanaged = self
        //     .image
        //     .as_bytes()
        //     .iter()
        //     .skip(skip as usize) // get to the row we want
        //     .take(total_rows_to_take as usize)
        //     .collect::<Vec<&u8>>(); // get all the rows we want to look at

        // let data = unmanaged
        //     .chunks_exact(self.image.width() as usize) // Get rows
        //     .flat_map(|f| {
        //         f.iter()
        //             .skip(row_skip as usize)
        //             .take(total_row_take)
        //             .copied()
        //     }) // flatten this all out
        //     .copied() // copy it over so that it's u8
        //     .collect(); // collect into a vec

        // data
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> Result<Self> {
        Ok(Self {
            image: self
                .image
                .crop_imm(left as u32, top as u32, width as u32, height as u32),
            width,
            height,
        })
    }

    fn invert(&mut self) {
        self.image.invert()
    }

    fn rotate_counter_clockwise(&self) -> Result<Self> {
        let img = self.image.rotate270();
        Ok(Self {
            width: img.width() as usize,
            height: img.height() as usize,
            image: img,
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
            image: new_img,
        })
    }

    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        self.image.get_pixel(x as u32, y as u32).to_luma().0[0]
    }
}

fn build_local_grey_image(source: DynamicImage) -> DynamicImage {
    let raster = match source {
        DynamicImage::ImageLuma8(img) => img,
        DynamicImage::ImageLumaA8(img) => {
            let mut raster: ImageBuffer<_, Vec<_>> = ImageBuffer::new(img.width(), img.height());

            for (x, y, new_pixel) in raster.enumerate_pixels_mut() {
                let pixel = img.get_pixel(x, y);
                let [luma, alpha] = pixel.0;
                if alpha == 0 {
                    // white, so we know its luminance is 255
                    *new_pixel = Luma([0xFF])
                } else {
                    *new_pixel = Luma([luma.saturating_mul(alpha)])
                }
            }

            raster
        }
        // DynamicImage::ImageRgb8(_) => todo!(),
        // DynamicImage::ImageRgba8(_) => todo!(),
        DynamicImage::ImageLuma16(img) => {
            let mut raster: ImageBuffer<_, Vec<_>> = ImageBuffer::new(img.width(), img.height());

            for (x, y, new_pixel) in raster.enumerate_pixels_mut() {
                let pixel = img.get_pixel(x, y);
                let [luma] = pixel.0;

                *new_pixel = Luma([(luma / u8::MAX as u16) as u8])
            }

            raster
        }
        DynamicImage::ImageLumaA16(img) => {
            let mut raster: ImageBuffer<_, Vec<_>> = ImageBuffer::new(img.width(), img.height());

            for (x, y, new_pixel) in raster.enumerate_pixels_mut() {
                let pixel = img.get_pixel(x, y);
                let [luma, alpha] = pixel.0;
                if alpha == 0 {
                    // white, so we know its luminance is 255
                    *new_pixel = Luma([0xFF])
                } else {
                    *new_pixel = Luma([((luma.saturating_mul(alpha)) / u8::MAX as u16) as u8])
                }
            }

            raster
        }
        // DynamicImage::ImageRgb16(_) => todo!(),
        // DynamicImage::ImageRgba16(_) => todo!(),
        // DynamicImage::ImageRgb32F(_) => todo!(),
        // DynamicImage::ImageRgba32F(_) => todo!(),
        _ => {
            let img = source.to_rgba8();

            let mut raster: ImageBuffer<_, Vec<_>> =
                ImageBuffer::new(source.width(), source.height());

            for (x, y, new_pixel) in raster.enumerate_pixels_mut() {
                let pixel = img.get_pixel(x, y);
                let [red, green, blue, alpha] = pixel.0;
                if alpha == 0 {
                    // white, so we know its luminance is 255
                    *new_pixel = Luma([0xFF])
                } else {
                    // .299R + 0.587G + 0.114B (YUV/YIQ for PAL and NTSC),
                    // (306*R) >> 10 is approximately equal to R*0.299, and so on.
                    // 0x200 >> 10 is 0.5, it implements rounding.
                    *new_pixel = Luma([((306 * (red as u64)
                        + 601 * (green as u64)
                        + 117 * (blue as u64)
                        + 0x200)
                        >> 10) as u8])
                }
            }
            raster
        }
    };

    DynamicImage::from(raster)
}
