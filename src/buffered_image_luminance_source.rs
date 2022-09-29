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

// package com.google.zxing;

// import java.awt.Graphics2D;
// import java.awt.geom.AffineTransform;
// import java.awt.image.BufferedImage;
// import java.awt.image.WritableRaster;

use image::{DynamicImage, Luma, GenericImage, EncodableLayout};
use imageproc::geometric_transformations::rotate_about_center;

use crate::LuminanceSource;

const MINUS_45_IN_RADIANS: f32 = -0.7853981633974483; // Math.toRadians(-45.0)

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
        

        Self {
            image: DynamicImage::from(image.to_luma8()),
            width: width,
            height: height,
            left: left,
            top: top,
        }
    }
}

impl LuminanceSource for BufferedImageLuminanceSource {
    fn getRow(&self, y: usize, row: &Vec<u8>) -> Vec<u8> {
        let width = self.getWidth();

        let mut row = if row.len() >= width {
            row.to_vec()
        } else {
            vec![0; width]
        };

        let pixels: Vec<u8> = self
            .image
            .clone()
            .into_luma8()
            .rows()
            .nth(y)
            .unwrap()
            .map(|&p| p.0[0])
            .collect();

        // The underlying raster of image consists of bytes with the luminance values
        row[..width].clone_from_slice(&pixels[..]);

        return row;
    }

    fn getMatrix(&self) -> Vec<u8> {
        return self.image.as_bytes().to_vec();
    }

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
    }

    fn invert(&mut self) {
        self.image.invert();
    }

    fn isCropSupported(&self) -> bool {
        return true;
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>, crate::exceptions::Exceptions> {
        return Ok(Box::new(BufferedImageLuminanceSource::with_details(
            self.image
                .crop_imm(left as u32, top as u32, width as u32, height as u32),
            self.left + left as u32,
            self.top + top as u32,
            width,
            height,
        )));
    }

    fn isRotateSupported(&self) -> bool {
        return true;
    }

    fn rotateCounterClockwise(
        &self,
    ) -> Result<Box<dyn LuminanceSource>, crate::exceptions::Exceptions> {
        Ok(Box::new(BufferedImageLuminanceSource::new(
            self.image.rotate270(),
        )))
    }

    fn rotateCounterClockwise45(
        &self,
    ) -> Result<Box<dyn LuminanceSource>, crate::exceptions::Exceptions> {
        let img = rotate_about_center(
            &self.image.to_luma8(),
            MINUS_45_IN_RADIANS,
            imageproc::geometric_transformations::Interpolation::Nearest,
            Luma([255; 1]),
        );

        let new_img = DynamicImage::from(img);

        Ok(Box::new(BufferedImageLuminanceSource::new(new_img)))
    }
}
