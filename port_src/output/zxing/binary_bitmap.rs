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
// package com::google::zxing;

/**
 * This class is the core bitmap class used by ZXing to represent 1 bit data. Reader objects
 * accept a BinaryBitmap and attempt to decode it.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct BinaryBitmap {

     let binarizer: Binarizer;

     let mut matrix: BitMatrix;
}

impl BinaryBitmap {

    pub fn new( binarizer: &Binarizer) -> BinaryBitmap {
        if binarizer == null {
            throw IllegalArgumentException::new("Binarizer must be non-null.");
        }
        let .binarizer = binarizer;
    }

    /**
   * @return The width of the bitmap.
   */
    pub fn  get_width(&self) -> i32  {
        return self.binarizer.get_width();
    }

    /**
   * @return The height of the bitmap.
   */
    pub fn  get_height(&self) -> i32  {
        return self.binarizer.get_height();
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
    pub fn  get_black_row(&self,  y: i32,  row: &BitArray) -> /*  throws NotFoundException */Result<BitArray, Rc<Exception>>   {
        return Ok(self.binarizer.get_black_row(y, row));
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
    pub fn  get_black_matrix(&self) -> /*  throws NotFoundException */Result<BitMatrix, Rc<Exception>>   {
        // 2. This work will only be done once even if the caller installs multiple 2D Readers.
        if self.matrix == null {
            self.matrix = self.binarizer.get_black_matrix();
        }
        return Ok(self.matrix);
    }

    /**
   * @return Whether this bitmap can be cropped.
   */
    pub fn  is_crop_supported(&self) -> bool  {
        return self.binarizer.get_luminance_source().is_crop_supported();
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
    pub fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> BinaryBitmap  {
         let new_source: LuminanceSource = self.binarizer.get_luminance_source().crop(left, top, width, height);
        return BinaryBitmap::new(&self.binarizer.create_binarizer(new_source));
    }

    /**
   * @return Whether this bitmap supports counter-clockwise rotation.
   */
    pub fn  is_rotate_supported(&self) -> bool  {
        return self.binarizer.get_luminance_source().is_rotate_supported();
    }

    /**
   * Returns a new object with rotated image data by 90 degrees counterclockwise.
   * Only callable if {@link #isRotateSupported()} is true.
   *
   * @return A rotated version of this object.
   */
    pub fn  rotate_counter_clockwise(&self) -> BinaryBitmap  {
         let new_source: LuminanceSource = self.binarizer.get_luminance_source().rotate_counter_clockwise();
        return BinaryBitmap::new(&self.binarizer.create_binarizer(new_source));
    }

    /**
   * Returns a new object with rotated image data by 45 degrees counterclockwise.
   * Only callable if {@link #isRotateSupported()} is true.
   *
   * @return A rotated version of this object.
   */
    pub fn  rotate_counter_clockwise45(&self) -> BinaryBitmap  {
         let new_source: LuminanceSource = self.binarizer.get_luminance_source().rotate_counter_clockwise45();
        return BinaryBitmap::new(&self.binarizer.create_binarizer(new_source));
    }

    pub fn  to_string(&self) -> String  {
        let tryResult1 = 0;
        'try1: loop {
        {
            return self.get_black_matrix().to_string();
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &NotFoundException) {
                return "";
            }  0 => break
        }

    }
}

