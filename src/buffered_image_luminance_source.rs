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

use image::{DynamicImage, ImageBuffer, Luma};
use imageproc::geometric_transformations::{Interpolation, rotate_about_center};

use crate::common::Result;
use crate::{Luma8LuminanceSource, LuminanceSource};

// const MINUS_45_IN_RADIANS: f32 = -0.7853981633974483; // Math.toRadians(-45.0)
const MINUS_45_IN_RADIANS: f32 = std::f32::consts::FRAC_PI_4;

/**
 * This LuminanceSource implementation is meant for J2SE clients and our blackbox unit tests.
 *
 * The image is converted to greyscale once at construction; everything else
 * delegates to [`Luma8LuminanceSource`], inheriting its zero-copy cropping and
 * shared buffers.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author code@elektrowolle.de (Wolfgang Jung)
 */
#[derive(Debug, Clone)]
pub struct BufferedImageLuminanceSource {
    source: Luma8LuminanceSource,
}

impl BufferedImageLuminanceSource {
    pub fn new(image: DynamicImage) -> Self {
        let grey = build_local_grey_image(image);
        let (width, height) = grey.dimensions();
        let source = Luma8LuminanceSource::new(grey.into_raw(), width, height)
            .expect("image dimensions match its buffer by construction");
        Self { source }
    }
}

impl LuminanceSource for BufferedImageLuminanceSource {
    const SUPPORTS_CROP: bool = true;
    const SUPPORTS_ROTATION: bool = true;

    fn get_row(&'_ self, y: usize) -> Option<Cow<'_, [u8]>> {
        self.source.get_row(y)
    }

    fn get_column(&self, x: usize) -> Cow<'_, [u8]> {
        self.source.get_column(x)
    }

    fn get_matrix(&self) -> Cow<'_, [u8]> {
        self.source.get_matrix()
    }

    fn get_width(&self) -> usize {
        self.source.get_width()
    }

    fn get_height(&self) -> usize {
        self.source.get_height()
    }

    fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> Result<Self> {
        Ok(Self {
            source: self.source.crop(left, top, width, height)?,
        })
    }

    fn invert(&mut self) {
        self.source.invert()
    }

    fn rotate_counter_clockwise(&self) -> Result<Self> {
        Ok(Self {
            source: self.source.rotate_counter_clockwise()?,
        })
    }

    fn rotate_counter_clockwise_45(&self) -> Result<Self> {
        // A raster resampling operation, so this one genuinely materializes.
        let (width, height) = (self.get_width() as u32, self.get_height() as u32);
        let buffer = ImageBuffer::from_raw(width, height, self.source.get_matrix().into_owned())
            .ok_or_else(|| {
                crate::Error::illegal_argument_with("matrix does not match its dimensions")
            })?;
        let rotated = rotate_about_center(
            &buffer,
            MINUS_45_IN_RADIANS,
            Interpolation::Nearest,
            Luma([u8::MAX / 2; 1]),
        );
        Ok(Self {
            source: Luma8LuminanceSource::new(rotated.into_raw(), width, height)?,
        })
    }

    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        self.source.get_luma8_point(x, y)
    }
}

fn build_local_grey_image(source: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    match source {
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
                    // ZXing reference: alpha is otherwise ignored
                    *new_pixel = Luma([luma])
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

                *new_pixel = Luma([(luma >> 8) as u8])
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
                    // ZXing reference: alpha is otherwise ignored
                    *new_pixel = Luma([(luma >> 8) as u8])
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
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use image::{DynamicImage, ImageBuffer};

    use crate::{BufferedImageLuminanceSource, LuminanceSource};

    fn luma_4x4() -> BufferedImageLuminanceSource {
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_raw(4, 4, (0..16).collect()).unwrap());
        BufferedImageLuminanceSource::new(img)
    }

    #[test]
    fn conversion_passes_luma8_through() {
        let src = luma_4x4();
        assert_eq!(src.get_width(), 4);
        assert_eq!(src.get_height(), 4);
        assert_eq!(&*src.get_matrix(), &(0..16).collect::<Vec<u8>>()[..]);
    }

    /// ZXing reference semantics (BufferedImageLuminanceSource.java, 3.5.1):
    /// alpha == 0 → white, otherwise alpha is ignored and the luma passes
    /// through unchanged — consistent with the RGBA arm below.
    #[test]
    fn conversion_luma_a8_matches_zxing_reference() {
        let img = DynamicImage::ImageLumaA8(
            ImageBuffer::from_raw(4, 1, vec![50, 0, 50, 255, 0, 255, 2, 3]).unwrap(),
        );
        let src = BufferedImageLuminanceSource::new(img);
        assert_eq!(&*src.get_matrix(), &[255, 50, 0, 2]);
    }

    /// ZXing reference semantics: 16-bit luma scales to 8-bit with `>> 8`
    /// (Java's TYPE_USHORT_GRAY → getRGB conversion).
    #[test]
    fn conversion_luma16_matches_zxing_reference() {
        let img = DynamicImage::ImageLuma16(
            ImageBuffer::from_raw(3, 1, vec![0xFFFFu16, 0x0000, 0x7F00]).unwrap(),
        );
        let src = BufferedImageLuminanceSource::new(img);
        assert_eq!(&*src.get_matrix(), &[255, 0, 127]);
    }

    /// ZXing reference semantics for 16-bit grey+alpha: alpha == 0 → white,
    /// otherwise `luma >> 8`.
    #[test]
    fn conversion_luma_a16_matches_zxing_reference() {
        #[rustfmt::skip]
        let pixels = vec![
            0xFFFFu16, 0x0000, // transparent → white
            0xFFFF, 0xFFFF,    // opaque white
            0x0000, 0xFFFF,    // opaque black
            0x7F00, 0x0101,    // semi-transparent mid-grey → 127
        ];
        let img = DynamicImage::ImageLumaA16(ImageBuffer::from_raw(4, 1, pixels).unwrap());
        let src = BufferedImageLuminanceSource::new(img);
        assert_eq!(&*src.get_matrix(), &[255, 255, 0, 127]);
    }

    /// End-to-end guard for the LumaA8 fix: a grey-toned (60/200), fully
    /// opaque QR code must survive the greyscale conversion and decode. Under
    /// the old `saturating_mul` conversion every pixel ≥ 1 saturated to white
    /// and this failed.
    #[test]
    #[cfg(all(feature = "encoders", feature = "decoders", feature = "qrcode"))]
    fn grey_toned_opaque_luma_a8_decodes() {
        use crate::common::HybridBinarizer;
        use crate::{
            BarcodeFormat, BinaryBitmap, DecodeHints, MultiFormatReader, MultiFormatWriter, Reader,
            Writer,
        };

        let content = "grey on grey";
        let bits = MultiFormatWriter
            .encode(content, &BarcodeFormat::QR_CODE, 128, 128)
            .expect("encode succeeds");
        let (w, h) = (bits.width(), bits.height());
        let mut pixels = Vec::with_capacity((w * h * 2) as usize);
        for y in 0..h {
            for x in 0..w {
                pixels.extend([if bits.get(x, y) { 60u8 } else { 200 }, 255]);
            }
        }
        let img = DynamicImage::ImageLumaA8(ImageBuffer::from_raw(w, h, pixels).unwrap());

        let mut bitmap =
            BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img)));
        let result = MultiFormatReader::default()
            .decode_with_hints(&mut bitmap, &DecodeHints::default())
            .expect("grey-toned opaque QR must decode");
        assert_eq!(result.getText(), content);
    }

    /// Pins current RGBA semantics: alpha == 0 → white, else the fixed-point
    /// YUV weights `(306R + 601G + 117B + 0x200) >> 10`.
    #[test]
    fn conversion_pins_rgba_semantics() {
        #[rustfmt::skip]
        let pixels = vec![
            0, 0, 0, 0,         // transparent → white
            255, 255, 255, 255, // white
            0, 0, 0, 255,       // black
            255, 0, 0, 255,     // red → (306*255 + 0x200) >> 10 == 76
        ];
        let img = DynamicImage::ImageRgba8(ImageBuffer::from_raw(4, 1, pixels).unwrap());
        let src = BufferedImageLuminanceSource::new(img);
        assert_eq!(&*src.get_matrix(), &[255, 255, 0, 76]);
    }

    #[test]
    fn row_column_and_point_accessors_agree() {
        let src = luma_4x4();
        assert_eq!(&*src.get_row(1).expect("row in bounds"), &[4, 5, 6, 7]);
        assert_eq!(&*src.get_column(2), &[2, 6, 10, 14]);
        assert_eq!(src.get_luma8_point(3, 2), 11);
    }

    #[test]
    fn invert_inverts_all_accessors() {
        let img =
            DynamicImage::ImageLuma8(ImageBuffer::from_raw(2, 2, vec![0, 10, 100, 255]).unwrap());
        let mut src = BufferedImageLuminanceSource::new(img);
        src.invert();
        assert_eq!(&*src.get_matrix(), &[255, 245, 155, 0]);
        assert_eq!(src.get_luma8_point(0, 0), 255);
        assert_eq!(&*src.get_column(1), &[245, 0]);
    }

    #[test]
    fn crop_and_rotate_have_correct_values() {
        let src = luma_4x4();
        let cropped = src.crop(1, 1, 2, 2).expect("crop");
        assert_eq!(&*cropped.get_matrix(), &[5, 6, 9, 10]);

        let rotated = src.rotate_counter_clockwise().expect("rotate");
        // CCW: first row of the rotated image is the last column, top to bottom
        assert_eq!(
            &*rotated.get_row(0).expect("row in bounds"),
            &[3, 7, 11, 15]
        );
    }

    /// Pins the exact output of the current 45° rotation (imageproc
    /// `rotate_about_center`, nearest-neighbour, grey fill) on a small
    /// asymmetric input, so the refactor can be verified byte-identical.
    #[test]
    fn rotate_45_pins_current_output() {
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_raw(3, 3, (0..9).collect()).unwrap());
        let src = BufferedImageLuminanceSource::new(img);
        let rotated = src.rotate_counter_clockwise_45().expect("rotate 45");
        assert_eq!(rotated.get_width(), 3);
        assert_eq!(rotated.get_height(), 3);
        assert_eq!(&*rotated.get_matrix(), &[127, 3, 1, 6, 7, 5, 127, 8, 8]);
    }

    #[test]
    fn crop_is_zero_copy() {
        let src = luma_4x4();
        let cropped = src.crop(1, 1, 2, 2).expect("crop");

        let Cow::Borrowed(parent) = src.get_matrix() else {
            panic!("full-view matrix must be Cow::Borrowed");
        };
        let Some(Cow::Borrowed(row)) = cropped.get_row(0) else {
            panic!("cropped row must be Cow::Borrowed");
        };

        let base = parent.as_ptr() as usize;
        let p = row.as_ptr() as usize;
        assert!(
            p >= base && p < base + parent.len(),
            "crop must share the parent's buffer, not copy the region"
        );
    }
}
