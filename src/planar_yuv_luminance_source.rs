// /*
//  * Copyright 2013 ZXing authors
//  *
//  * Licensed under the Apache License, Version 2.0 (the "License");
//  * you may not use this file except in compliance with the License.
//  * You may obtain a copy of the License at
//  *
//  *      http://www.apache.org/licenses/LICENSE-2.0
//  *
//  * Unless required by applicable law or agreed to in writing, software
//  * distributed under the License is distributed on an "AS IS" BASIS,
//  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  * See the License for the specific language governing permissions and
//  * limitations under the License.
//  */
// //package com.google.zxing;

// /**
//  * A wrapper implementation of {@link LuminanceSource} which inverts the luminances it returns -- black becomes
//  * white and vice versa, and each value becomes (255-value).
//  *
//  * @author Sean Owen
//  */
// pub struct InvertedLuminanceSource {
//     width: usize,
//     height: usize,
//     delegate: Box<dyn LuminanceSource>,
// }

// impl InvertedLuminanceSource {
//     pub fn new_with_delegate(delegate: Box<dyn LuminanceSource>) -> Self {
//         Self {
//             width: delegate.getWidth(),
//             height: delegate.getHeight(),
//             delegate,
//         }
//     }
// }

// impl LuminanceSource for InvertedLuminanceSource {
//     fn getRow(&self, y: usize, row: &Vec<u8>) -> Vec<u8> {
//         let mut new_row = self.delegate.getRow(y, row);
//         let width = self.getWidth();
//         for i in 0..width {
//             //for (int i = 0; i < width; i++) {
//             new_row[i] = 255 - (new_row[i] & 0xFF);
//         }
//         return new_row;
//     }

//     fn getMatrix(&self) -> Vec<u8> {
//         let matrix = self.delegate.getMatrix();
//         let length = self.getWidth() * self.getHeight();
//         let mut invertedMatrix = Vec::with_capacity(length);
//         for i in 0..length {
//             //for (int i = 0; i < length; i++) {
//             invertedMatrix[i] = 255 - (matrix[i] & 0xFF);
//         }
//         return invertedMatrix;
//     }

//     fn getWidth(&self) -> usize {
//         self.width
//     }

//     fn getHeight(&self) -> usize {
//         self.height
//     }

//     fn isCropSupported(&self) -> bool {
//         return self.delegate.isCropSupported();
//     }

//     fn crop(
//         &self,
//         left: usize,
//         top: usize,
//         width: usize,
//         height: usize,
//     ) -> Result<Box<dyn LuminanceSource>, UnsupportedOperationException> {
//         let crop = self.delegate.crop(left, top, width, height)?;
//         return Ok(Box::new(InvertedLuminanceSource::new_with_delegate(crop)));
//     }

//     fn isRotateSupported(&self) -> bool {
//         return self.delegate.isRotateSupported();
//     }

//     /**
//      * @return original delegate {@link LuminanceSource} since invert undoes itself
//      */
//     fn invert(&self) -> Box<dyn LuminanceSource> {
//         return self.delegate;
//     }

//     fn rotateCounterClockwise(
//         &self,
//     ) -> Result<Box<dyn LuminanceSource>, UnsupportedOperationException> {
//         let rot = self.delegate.rotateCounterClockwise()?;
//         return Ok(Box::new(InvertedLuminanceSource::new_with_delegate(rot)));
//     }

//     fn rotateCounterClockwise45(
//         &self,
//     ) -> Result<Box<dyn LuminanceSource>, UnsupportedOperationException> {
//         let rot_45 = self.delegate.rotateCounterClockwise45()?;
//         return Ok(Box::new(InvertedLuminanceSource::new_with_delegate(rot_45)));
//     }
// }

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

//package com.google.zxing;

use crate::{Exceptions, LuminanceSource};

const THUMBNAIL_SCALE_FACTOR: usize = 2;

/**
 * This object extends LuminanceSource around an array of YUV data returned from the camera driver,
 * with the option to crop to a rectangle within the full data. This can be used to exclude
 * superfluous pixels around the perimeter and speed up decoding.
 *
 * It works for any pixel format where the Y channel is planar and appears first, including
 * YCbCr_420_SP and YCbCr_422_SP.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Debug, Clone)]
pub struct PlanarYUVLuminanceSource {
    yuv_data: Vec<u8>,
    data_width: usize,
    data_height: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    invert: bool,
}

impl PlanarYUVLuminanceSource {
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_all(
        yuv_data: Vec<u8>,
        data_width: usize,
        data_height: usize,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
        reverse_horizontal: bool,
        inverted: bool,
    ) -> Result<Self, Exceptions> {
        if left + width > data_width || top + height > data_height {
            return Err(Exceptions::illegalArgument(
                "Crop rectangle does not fit within image data.".to_owned(),
            ));
        }

        let mut new_s: Self = Self {
            yuv_data,
            data_width,
            data_height,
            left,
            top,
            width,
            height,
            invert: inverted,
        };

        if reverse_horizontal {
            new_s.reverseHorizontal(width, height);
        }

        Ok(new_s)
    }

    pub fn renderThumbnail(&self) -> Vec<u8> {
        let width = self.getWidth() / THUMBNAIL_SCALE_FACTOR;
        let height = self.getHeight() / THUMBNAIL_SCALE_FACTOR;
        let mut pixels = vec![0; width * height];
        let yuv = &self.yuv_data;
        let mut input_offset = self.top * self.data_width + self.left;

        for y in 0..height {
            let output_offset = y * width;
            for x in 0..width {
                let grey = yuv[input_offset + x * THUMBNAIL_SCALE_FACTOR];
                pixels[output_offset + x] = (0xFF000000 | (grey as u32 * 0x00010101)) as u8;
            }
            input_offset += self.data_width * THUMBNAIL_SCALE_FACTOR;
        }
        pixels
    }

    /**
     * @return width of image from {@link #renderThumbnail()}
     */
    pub fn getThumbnailWidth(&self) -> usize {
        self.getWidth() / THUMBNAIL_SCALE_FACTOR
    }

    /**
     * @return height of image from {@link #renderThumbnail()}
     */
    pub fn getThumbnailHeight(&self) -> usize {
        self.getHeight() / THUMBNAIL_SCALE_FACTOR
    }

    fn reverseHorizontal(&mut self, width: usize, height: usize) {
        let mut rowStart = self.top * self.data_width + self.left;
        for _y in 0..height {
            let middle = rowStart + width / 2;
            let mut x2 = rowStart + width - 1;
            for x1 in rowStart..middle {
                self.yuv_data.swap(x1, x2);
                x2 -= 1;
            }
            rowStart += self.data_width;
        }
    }
}

impl LuminanceSource for PlanarYUVLuminanceSource {
    fn getRow(&self, y: usize) -> Vec<u8> {
        if y >= self.getHeight() {
            // //throw new IllegalArgumentException("Requested row is outside the image: " + y);
            // panic!("Requested row is outside the image: {y}");
            return Vec::new();
        }
        let width = self.getWidth();

        let offset = (y + self.top) * self.data_width + self.left;

        let mut row = vec![0; width];

        row[..width].clone_from_slice(&self.yuv_data[offset..width + offset]);
        if self.invert {
            row = self.invert_block_of_bytes(row);
        }
        row
    }

    fn getMatrix(&self) -> Vec<u8> {
        let width = self.getWidth();
        let height = self.getHeight();

        // If the caller asks for the entire underlying image, save the copy and give them the
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.data_width && height == self.data_height {
            let mut v = self.yuv_data.clone();
            if self.invert {
                v = self.invert_block_of_bytes(v);
            }
            return v;
        }

        let area = width * height;
        let mut matrix = vec![0; area];
        let mut inputOffset = self.top * self.data_width + self.left;

        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.data_width {
            matrix[0..area].clone_from_slice(&self.yuv_data[inputOffset..area]);
            if self.invert {
                matrix = self.invert_block_of_bytes(matrix);
            }
            return matrix;
        }

        // Otherwise copy one cropped row at a time.
        for y in 0..height {
            let outputOffset = y * width;
            matrix[outputOffset..outputOffset + width]
                .clone_from_slice(&self.yuv_data[inputOffset..inputOffset + width]);
            inputOffset += self.data_width;
        }

        if self.invert {
            matrix = self.invert_block_of_bytes(matrix);
        }

        matrix
    }

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
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
    ) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        match PlanarYUVLuminanceSource::new_with_all(
            self.yuv_data.clone(),
            self.data_width,
            self.data_height,
            self.left + left,
            self.top + top,
            width,
            height,
            false,
            self.invert,
        ) {
            Ok(new) => Ok(Box::new(new)),
            Err(_err) => Err(Exceptions::unsupportedOperationEmpty()),
        }
    }

    fn isRotateSupported(&self) -> bool {
        false
    }

    fn invert(&mut self) {
        self.invert = !self.invert;
    }
}
