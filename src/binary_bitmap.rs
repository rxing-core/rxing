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

use std::{fmt, rc::Rc, marker::PhantomData};

use crate::{
    common::{BitArray, BitMatrix},
    Binarizer, Exceptions, LuminanceSource,
};

/**
 * This class is the core bitmap class used by ZXing to represent 1 bit data. Reader objects
 * accept a BinaryBitmap and attempt to decode it.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Clone)]
pub struct BinaryBitmap<L:LuminanceSource,B:Binarizer<L>> {
    binarizer: Rc<B>,
    matrix: BitMatrix,
    pd_l: PhantomData<L>
}

impl<L:LuminanceSource,B:Binarizer<L>> BinaryBitmap<L,B> {
    pub fn new(binarizer: B) -> Self {
        Self {
            matrix: binarizer.getBlackMatrix().unwrap(),
            binarizer: Rc::new(binarizer),
            pd_l: PhantomData
        }
    }

    /**
     * @return The width of the bitmap.
     */
    pub fn getWidth(&self) -> usize {
        return self.binarizer.getWidth();
    }

    /**
     * @return The height of the bitmap.
     */
    pub fn getHeight(&self) -> usize {
        return self.binarizer.getHeight();
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
    pub fn getBlackRow(&self, y: usize, row: &mut BitArray) -> Result<BitArray, Exceptions> {
        return self.binarizer.getBlackRow(y, row);
    }

    /**
     * Converts a 2D array of luminance data to 1 bit. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    pub fn getBlackMatrixMut(&mut self) -> &mut BitMatrix {
        // The matrix is created on demand the first time it is requested, then cached. There are two
        // reasons for this:
        // 1. This work will never be done if the caller only installs 1D Reader objects, or if a
        //    1D Reader finds a barcode before the 2D Readers run.
        // 2. This work will only be done once even if the caller installs multiple 2D Readers.
        &mut self.matrix
    }

    /**
     * Converts a 2D array of luminance data to 1 bit. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    pub fn getBlackMatrix(&self) -> &BitMatrix {
        // The matrix is created on demand the first time it is requested, then cached. There are two
        // reasons for this:
        // 1. This work will never be done if the caller only installs 1D Reader objects, or if a
        //    1D Reader finds a barcode before the 2D Readers run.
        // 2. This work will only be done once even if the caller installs multiple 2D Readers.
        &self.matrix
    }

    /**
     * @return Whether this bitmap can be cropped.
     */
    pub fn isCropSupported(&self) -> bool {
        let b = &self.binarizer;
        let r = &b.getLuminanceSource();
        let isCropOk = r.isCropSupported();
        return isCropOk;
    }

    /**
     * Returns a new object with cropped image data. Implementations may keep a reference to the
     * original data rather than a copy. Only callable if isCropSupported() is true.
     *
     * @param left The left coordinate, which must be in [0,getWidth())
     * @param top The top coordinate, which must be in [0,getHeight())
     * @param width The width of the rectangle to crop.
     * @param height The height of the rectangle to crop.
     * @return A cropped version of this object.
     */
    pub fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> BinaryBitmap<L,B> {
        let newSource = self
            .binarizer
            .getLuminanceSource()
            .crop(left, top, width, height);
        return BinaryBitmap::new(
            self.binarizer
                .createBinarizer(newSource.expect("new lum source expected")),
        );
    }

    /**
     * @return Whether this bitmap supports counter-clockwise rotation.
     */
    pub fn isRotateSupported(&self) -> bool {
        return self.binarizer.getLuminanceSource().isRotateSupported();
    }

    /**
     * Returns a new object with rotated image data by 90 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * @return A rotated version of this object.
     */
    pub fn rotateCounterClockwise(&self) -> BinaryBitmap<L,B> {
        let newSource = self.binarizer.getLuminanceSource().rotateCounterClockwise();
        return BinaryBitmap::new(
            self.binarizer
                .createBinarizer(newSource.expect("new lum source expected")),
        );
    }

    /**
     * Returns a new object with rotated image data by 45 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * @return A rotated version of this object.
     */
    pub fn rotateCounterClockwise45(&self) -> BinaryBitmap<L,B> {
        let newSource = self
            .binarizer
            .getLuminanceSource()
            .rotateCounterClockwise45();
        return BinaryBitmap::new(
            self.binarizer
                .createBinarizer(newSource.expect("new lum source expected")),
        );
    }
}

impl<L:LuminanceSource,B:Binarizer<L>> fmt::Display for BinaryBitmap<L,B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.getBlackMatrix())
    }
}
