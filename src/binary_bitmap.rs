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

use std::{
    borrow::{Borrow, Cow},
    fmt,
};

use once_cell::sync::OnceCell;

use crate::{
    Binarizer, LuminanceSource,
    common::{BitArray, BitMatrix, LineOrientation, Result},
};

/**
 * This class is the core bitmap class used by ZXing to represent 1 bit data. Reader objects
 * accept a BinaryBitmap and attempt to decode it.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct BinaryBitmap<B: Binarizer> {
    binarizer: B,
    pub(crate) matrix: OnceCell<BitMatrix>,
    inverted: bool,
}

impl<B: Binarizer> BinaryBitmap<B> {
    pub fn new(binarizer: B) -> Self {
        Self {
            matrix: OnceCell::new(),
            binarizer,
            inverted: false,
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
    pub fn get_black_row(&self, y: usize) -> Result<Cow<'_, BitArray>> {
        if self.inverted {
            let matrix = self.matrix.get().expect("invert() builds the matrix");
            // Match the binarizer path's contract: an out-of-range row is an
            // error, not a panic from indexing the flipped matrix.
            if y >= matrix.height() as usize {
                return Err(crate::Error::INDEX_OUT_OF_BOUNDS);
            }
            Ok(Cow::Owned(matrix.getRow(y as u32)))
        } else {
            self.binarizer.get_black_row(y)
        }
    }

    /// Get a row or column of the image
    pub fn get_black_line(&self, l: usize, lt: LineOrientation) -> Result<Cow<'_, BitArray>> {
        if self.inverted {
            let matrix = self.matrix.get().expect("invert() builds the matrix");
            let bound = match lt {
                LineOrientation::Row => matrix.height(),
                LineOrientation::Column => matrix.width(),
            };
            if l >= bound as usize {
                return Err(crate::Error::INDEX_OUT_OF_BOUNDS);
            }
            let line = match lt {
                LineOrientation::Row => matrix.getRow(l as u32),
                LineOrientation::Column => matrix.getCol(l as u32),
            };
            Ok(Cow::Owned(line))
        } else {
            self.binarizer.get_black_line(l, lt)
        }
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
    /// Fallback used when the binarizer cannot produce a matrix at all.
    fn empty_matrix(width: usize, height: usize) -> BitMatrix {
        if width == 0 || height == 0 {
            BitMatrix::new(1, 1).unwrap()
        } else {
            BitMatrix::new(width as u32, height as u32).unwrap()
        }
    }

    /**
     * Mutable access to the black matrix. Copy-on-write: the first call copies
     * the binarizer's cached matrix into this bitmap so that local mutations
     * (inversion, morphological closing) never corrupt the binarizer's cache.
     */
    pub fn get_black_matrix_mut(&mut self) -> &mut BitMatrix {
        if self.matrix.get().is_none() {
            let built = match self.binarizer.get_black_matrix() {
                Ok(matrix) => matrix.clone(),
                Err(_) => Self::empty_matrix(self.get_width(), self.get_height()),
            };
            let _ = self.matrix.set(built);
        }
        self.matrix.get_mut().expect("populated above")
    }

    /**
     * Converts a 2D array of luminance data to 1 bit. The matrix is computed
     * lazily by the binarizer and cached there; this method borrows that cache
     * directly (no copy) unless this bitmap holds a locally-modified copy
     * (after `invert()` or `close()`), which then takes precedence.
     */
    pub fn get_black_matrix(&self) -> &BitMatrix {
        if let Some(matrix) = self.matrix.get() {
            return matrix;
        }
        match self.binarizer.get_black_matrix() {
            Ok(matrix) => matrix,
            Err(_) => self
                .matrix
                .get_or_init(|| Self::empty_matrix(self.get_width(), self.get_height())),
        }
    }

    /// Switch between the normal and the inverted view of the image.
    ///
    /// Flips the cached black matrix (building it first if necessary) and
    /// routes `get_black_row` / `get_black_line` through the flipped matrix,
    /// so 1D readers see the inverted image too. Calling it again restores
    /// the original view.
    pub fn invert(&mut self) {
        self.get_black_matrix_mut().flip_self();
        self.inverted = !self.inverted;
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
        let mut cropped = BinaryBitmap::new(
            self.binarizer
                .create_binarizer(newSource.expect("new lum source expected")),
        );
        // The binarizer works on the (un-inverted) luminance source, so a derived
        // bitmap starts un-inverted; carry over our inverted view so a decoder
        // that crops during an AlsoInverted retry still sees the inverted image.
        if self.inverted {
            cropped.invert();
        }
        cropped
    }

    /**
     * @return Whether this bitmap supports counter-clockwise rotation.
     */
    pub fn is_rotate_supported(&self) -> bool {
        self.binarizer.get_luminance_source().is_rotate_supported()
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
        let mut rotated = BinaryBitmap::new(
            self.binarizer
                .create_binarizer(newSource.expect("new lum source expected")),
        );
        // Carry over our inverted view (see `crop`): the 1D TryHarder path
        // rotates the bitmap during an AlsoInverted retry and must keep scanning
        // the inverted image.
        if self.inverted {
            rotated.invert();
        }
        rotated
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
        let mut rotated = BinaryBitmap::new(
            self.binarizer
                .create_binarizer(newSource.expect("new lum source expected")),
        );
        // Carry over our inverted view (see `crop`).
        if self.inverted {
            rotated.invert();
        }
        rotated
    }

    pub fn get_source(&self) -> &B::Source {
        self.binarizer.get_luminance_source()
    }

    pub fn get_binarizer(&self) -> &B {
        &self.binarizer
    }
}

impl<B: Binarizer> fmt::Display for BinaryBitmap<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.matrix.borrow())
    }
}
