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

use image::{DynamicImage, ImageBuffer, Luma};
use imageproc::geometric_transformations::rotate_about_center;

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
        // if image.getType() == BufferedImage.TYPE_BYTE_GRAY {
        //   this.image = image;
        // } else {
        //   int sourceWidth = image.getWidth();
        //   int sourceHeight = image.getHeight();
        //   if (left + width > sourceWidth || top + height > sourceHeight) {
        //     throw new IllegalArgumentException("Crop rectangle does not fit within image data.");
        //   }

        //   this.image = new BufferedImage(sourceWidth, sourceHeight, BufferedImage.TYPE_BYTE_GRAY);

        //   WritableRaster raster = this.image.getRaster();
        //   int[] buffer = new int[width];
        //   for (int y = top; y < top + height; y++) {
        //     image.getRGB(left, y, width, 1, buffer, 0, sourceWidth);
        //     for (int x = 0; x < width; x++) {
        //       int pixel = buffer[x];

        //       // The color of fully-transparent pixels is irrelevant. They are often, technically, fully-transparent
        //       // black (0 alpha, and then 0 RGB). They are often used, of course as the "white" area in a
        //       // barcode image. Force any such pixel to be white:
        //       if ((pixel & 0xFF000000) == 0) {
        //         // white, so we know its luminance is 255
        //         buffer[x] = 0xFF;
        //       } else {
        //         // .299R + 0.587G + 0.114B (YUV/YIQ for PAL and NTSC),
        //         // (306*R) >> 10 is approximately equal to R*0.299, and so on.
        //         // 0x200 >> 10 is 0.5, it implements rounding.
        //         buffer[x] =
        //           (306 * ((pixel >> 16) & 0xFF) +
        //             601 * ((pixel >> 8) & 0xFF) +
        //             117 * (pixel & 0xFF) +
        //             0x200) >> 10;
        //       }
        //     }
        //     raster.setPixels(left, y, width, 1, buffer);
        //   }

        // }

        let img = image.to_rgba8();

        let mut raster: ImageBuffer<_, Vec<_>> = ImageBuffer::new(image.width(), image.height());

        for x in 0..image.width() {
            for y in 0..image.height() {
                let pixel = img.get_pixel(x, y);
                let [red, green, blue, alpha] = pixel.0;
                if alpha == 0 {
                    // white, so we know its luminance is 255
                    raster.put_pixel(x, y, Luma([0xFF]))
                } else {
                    // .299R + 0.587G + 0.114B (YUV/YIQ for PAL and NTSC),
                    // (306*R) >> 10 is approximately equal to R*0.299, and so on.
                    // 0x200 >> 10 is 0.5, it implements rounding.
                    raster.put_pixel(
                        x,
                        y,
                        Luma([((306 * (red as u64)
                            + 601 * (green as u64)
                            + 117 * (blue as u64)
                            + 0x200)
                            >> 10) as u8]),
                    );
                }
            }
        }

        // for pixel in img.pixels() {
        //     // The color of fully-transparent pixels is irrelevant. They are often, technically, fully-transparent
        //       // black (0 alpha, and then 0 RGB). They are often used, of course as the "white" area in a
        //       // barcode image. Force any such pixel to be white:
        //       let [red,green,blue,alpha] = pixel.0;
        //       if (alpha & 0xFF) == 0 {
        //         // white, so we know its luminance is 255
        //         raster.push(0xFF);
        //         // buffer[x] = 0xFF;
        //       } else {
        //         // .299R + 0.587G + 0.114B (YUV/YIQ for PAL and NTSC),
        //         // (306*R) >> 10 is approximately equal to R*0.299, and so on.
        //         // 0x200 >> 10 is 0.5, it implements rounding.
        //         raster.push((
        //           (306 * ((red as u32 >> 16) & 0xFF) +
        //             601 * ((green as u32 >> 8) & 0xFF) +
        //             117 * (blue as u32 & 0xFF) +
        //             0x200) >> 10) as u8);
        //       }
        //     }

        // let ib:ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(image.width()   , image.height(), raster).unwrap();

        Self {
            image: Rc::new(DynamicImage::from(raster)),
            width,
            height,
            left,
            top,
        }

        // Self {
        //     image: DynamicImage::from(image.to_luma8()),
        //     width: width,
        //     height: height,
        //     left: left,
        //     top: top,
        // }
    }
}

impl LuminanceSource for BufferedImageLuminanceSource {
    fn getRow(&self, y: usize) -> Vec<u8> {
        let width = self.getWidth(); // - self.left as usize;

        let pixels: Vec<u8> = self
            .image
            .as_luma8()
            .unwrap()
            .rows()
            .nth(y + self.top as usize)
            .unwrap()
            .skip(self.left as usize)
            .take(width)
            .map(|&p| p.0[0])
            .collect();

        // The underlying raster of image consists of bytes with the luminance values
        // row[..width].clone_from_slice(&pixels[..]);

        pixels
    }

    fn getMatrix(&self) -> Vec<u8> {
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
            .chunks(self.image.width() as usize)
            .into_iter() // Get rows
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

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
    }

    fn invert(&mut self) {
        // self.image.borrow_mut().invert();
        let mut img = (*self.image).clone();
        img.invert();
        self.image = Rc::new(img);
    }

    fn isCropSupported(&self) -> bool {
        true
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>, crate::exceptions::Exceptions> {
        //  Ok(Box::new(BufferedImageLuminanceSource::with_details(
        //     self.image
        //         .crop_imm(left as u32, top as u32, width as u32, height as u32),
        //     self.left + left as u32,
        //     self.top + top as u32,
        //     width,
        //     height,
        // )))
        Ok(Box::new(Self {
            // image: self
            //     .image
            //     .crop_imm(left as u32, top as u32, width as u32, height as u32),
            image: self.image.clone(),
            width,
            height,
            left: self.left + left as u32,
            top: self.top + top as u32,
        }))
    }

    fn isRotateSupported(&self) -> bool {
        true
    }

    fn rotateCounterClockwise(
        &self,
    ) -> Result<Box<dyn LuminanceSource>, crate::exceptions::Exceptions> {
        // Ok(Box::new(BufferedImageLuminanceSource::new(
        //     self.image.rotate270(),
        // )))
        let img = self.image.rotate270();
        Ok(Box::new(Self {
            width: img.width() as usize,
            height: img.height() as usize,
            image: Rc::new(img),
            left: 0,
            top: 0,
        }))
    }

    fn rotateCounterClockwise45(
        &self,
    ) -> Result<Box<dyn LuminanceSource>, crate::exceptions::Exceptions> {
        let img = rotate_about_center(
            &self.image.to_luma8(),
            MINUS_45_IN_RADIANS,
            imageproc::geometric_transformations::Interpolation::Nearest,
            Luma([u8::MAX / 2; 1]),
        );

        let new_img = DynamicImage::from(img);

        // Ok(Box::new(BufferedImageLuminanceSource::new(new_img)))
        Ok(Box::new(Self {
            width: new_img.width() as usize,
            height: new_img.height() as usize,
            image: Rc::new(new_img),
            left: 0,
            top: 0,
        }))
    }
}
