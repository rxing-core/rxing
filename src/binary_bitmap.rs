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

use std::{borrow::Cow, fmt};

use crate::{
    common::{BitArray, BitMatrix, LineOrientation, Result},
    Binarizer, LuminanceSource,
};

/**
 * This class is the core bitmap class used by ZXing to represent 1 bit data. Reader objects
 * accept a BinaryBitmap and attempt to decode it.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

pub struct BinaryBitmap<B: Binarizer> {
    binarizer: B,
    pub(crate) matrix: Option<BitMatrix>,
}

impl<B: Binarizer> BinaryBitmap<B> {
    pub fn new(binarizer: B) -> Self {
        Self {
            matrix: None,
            binarizer,
        }
    }

    /**
     * @return The width of the bitmap.
     */
    pub fn get_width(&self) -> usize {
        self.binarizer.get_width()
    }

    /**
     * @return The height of the bitmap.
     */
    pub fn get_height(&self) -> usize {
        self.binarizer.get_height()
    }

    /**
     * Converts one row of luminance data to 1 bit data. May actually do the conversion, or return
     * cached data. Callers should assume this method is expensive and call it as seldom as possible.
     * This method is intended for decoding 1D barcodes and may choose to apply sharpening.
     *
     * @param y The row to fetch, which must be in [0, bitmap height)
     * @param row An optional preallocated array. If null or too small, it will be ignored.
     *            If used, the Binarizer will call BitArray.clear(). Always use the returned object.
     * @return The array of bits for this row (true means black).
     * @throws NotFoundException if row can't be binarized
     */
    pub fn get_black_row(&self, y: usize) -> Result<Cow<BitArray>> {
        self.binarizer.get_black_row(y)
    }

    /// Get a row or column of the image
    pub fn get_black_line(&self, l: usize, lt: LineOrientation) -> Result<Cow<BitArray>> {
        self.binarizer.get_black_line(l, lt)
    }

    /**
     * Converts a 2D array of luminance data to 1 bit. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * Panics if the binarizer cannot be created.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    pub fn get_black_matrix_mut(&mut self) -> &mut BitMatrix {
        // The matrix is created on demand the first time it is requested, then cached. There are two
        // reasons for this:
        // 1. This work will never be done if the caller only installs 1D Reader objects, or if a
        //    1D Reader finds a barcode before the 2D Readers run.
        // 2. This work will only be done once even if the caller installs multiple 2D Readers.
        if self.matrix.is_none() {
            self.matrix = Some(self.binarizer.get_black_matrix().unwrap().clone());
        }
        self.matrix.as_mut().unwrap()
    }

    /**
     * Converts a 2D array of luminance data to 1 bit. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * Panics if the binarizer cannot be created.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    pub fn get_black_matrix(&mut self) -> &BitMatrix {
        // The matrix is created on demand the first time it is requested, then cached. There are two
        // reasons for this:
        // 1. This work will never be done if the caller only installs 1D Reader objects, or if a
        //    1D Reader finds a barcode before the 2D Readers run.
        // 2. This work will only be done once even if the caller installs multiple 2D Readers.
        if self.matrix.is_none() {
            self.matrix = Some(match self.binarizer.get_black_matrix() {
                Ok(a) => a.clone(),
                Err(_) => {
                    BitMatrix::new(self.get_width() as u32, self.get_height() as u32).unwrap()
                }
            })
            // self.binarizer.get_black_matrix().unwrap_or_else( |_| BitMatrix::new(self.get_width() as u32, self.get_height() as u32).unwrap()).clone())
        }
        self.matrix.as_ref().unwrap()
    }

    /**
     * @return Whether this bitmap can be cropped.
     */
    pub fn is_crop_supported(&self) -> bool {
        self.binarizer.get_luminance_source().is_crop_supported()
    }

    /**
     * Returns a new object with cropped image data. Implementations may keep a reference to the
     * original data rather than a copy. Only callable if isCropSupported() is true.
     *
     * Panics if the binarizer cannot be created.
     *
     * @param left The left coordinate, which must be in [0,getWidth())
     * @param top The top coordinate, which must be in [0,getHeight())
     * @param width The width of the rectangle to crop.
     * @param height The height of the rectangle to crop.
     * @return A cropped version of this object.
     */
    pub fn crop(&mut self, left: usize, top: usize, width: usize, height: usize) -> Self {
        let newSource = self
            .binarizer
            .get_luminance_source()
            .crop(left, top, width, height);
        BinaryBitmap::new(
            self.binarizer
                .create_binarizer(newSource.expect("new lum source expected")),
        )
    }

    /**
     * @return Whether this bitmap supports counter-clockwise rotation.
     */
    pub fn is_rotate_supported(&self) -> bool {
        return self.binarizer.get_luminance_source().is_rotate_supported();
    }

    /**
     * Returns a new object with rotated image data by 90 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * Panics if the binarizer cannot be created.
     *
     * @return A rotated version of this object.
     */
    pub fn rotate_counter_clockwise(&mut self) -> Self {
        let newSource = self
            .binarizer
            .get_luminance_source()
            .rotate_counter_clockwise();
        BinaryBitmap::new(
            self.binarizer
                .create_binarizer(newSource.expect("new lum source expected")),
        )
    }

    /**
     * Returns a new object with rotated image data by 45 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * Panics if the binarizer cannot be created.
     *
     * @return A rotated version of this object.
     */
    pub fn rotate_counter_clockwise_45(&self) -> Self {
        let newSource = self
            .binarizer
            .get_luminance_source()
            .rotate_counter_clockwise_45();
        BinaryBitmap::new(
            self.binarizer
                .create_binarizer(newSource.expect("new lum source expected")),
        )
    }

    pub fn get_source(&self) -> &B::Source {
        &self.binarizer.get_luminance_source()
    }

    pub fn get_binarizer(&self) -> &B {
        &self.binarizer
    }
}

impl<B: Binarizer> fmt::Display for BinaryBitmap<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.matrix)
    }
}
