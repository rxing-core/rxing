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
 * The purpose of this class hierarchy is to abstract different bitmap implementations across
 * platforms into a standard interface for requesting greyscale luminance values. The interface
 * only provides immutable methods; therefore crop and rotation create copies. This is to ensure
 * that one Reader does not modify the original luminance source and leave it in an unknown state
 * for other Readers in the chain.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct LuminanceSource {

     let width: i32;

     let height: i32;
}

impl LuminanceSource {

    pub fn new( width: i32,  height: i32) -> LuminanceSource {
        let .width = width;
        let .height = height;
    }

    /**
   * Fetches one row of luminance data from the underlying platform's bitmap. Values range from
   * 0 (black) to 255 (white). Because Java does not have an unsigned byte type, callers will have
   * to bitwise and with 0xff for each value. It is preferable for implementations of this method
   * to only fetch this row rather than the whole image, since no 2D Readers may be installed and
   * getMatrix() may never be called.
   *
   * @param y The row to fetch, which must be in [0,getHeight())
   * @param row An optional preallocated array. If null or too small, it will be ignored.
   *            Always use the returned object, and ignore the .length of the array.
   * @return An array containing the luminance data.
   */
    pub fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8> ;

    /**
   * Fetches luminance data for the underlying bitmap. Values should be fetched using:
   * {@code int luminance = array[y * width + x] & 0xff}
   *
   * @return A row-major 2D array of luminance values. Do not use result.length as it may be
   *         larger than width * height bytes on some platforms. Do not modify the contents
   *         of the result.
   */
    pub fn  get_matrix(&self) -> Vec<i8> ;

    /**
   * @return The width of the bitmap.
   */
    pub fn  get_width(&self) -> i32  {
        return self.width;
    }

    /**
   * @return The height of the bitmap.
   */
    pub fn  get_height(&self) -> i32  {
        return self.height;
    }

    /**
   * @return Whether this subclass supports cropping.
   */
    pub fn  is_crop_supported(&self) -> bool  {
        return false;
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
    pub fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> LuminanceSource  {
        throw UnsupportedOperationException::new("This luminance source does not support cropping.");
    }

    /**
   * @return Whether this subclass supports counter-clockwise rotation.
   */
    pub fn  is_rotate_supported(&self) -> bool  {
        return false;
    }

    /**
   * @return a wrapper of this {@code LuminanceSource} which inverts the luminances it returns -- black becomes
   *  white and vice versa, and each value becomes (255-value).
   */
    pub fn  invert(&self) -> LuminanceSource  {
        return InvertedLuminanceSource::new(self);
    }

    /**
   * Returns a new object with rotated image data by 90 degrees counterclockwise.
   * Only callable if {@link #isRotateSupported()} is true.
   *
   * @return A rotated version of this object.
   */
    pub fn  rotate_counter_clockwise(&self) -> LuminanceSource  {
        throw UnsupportedOperationException::new("This luminance source does not support rotation by 90 degrees.");
    }

    /**
   * Returns a new object with rotated image data by 45 degrees counterclockwise.
   * Only callable if {@link #isRotateSupported()} is true.
   *
   * @return A rotated version of this object.
   */
    pub fn  rotate_counter_clockwise45(&self) -> LuminanceSource  {
        throw UnsupportedOperationException::new("This luminance source does not support rotation by 45 degrees.");
    }

    pub fn  to_string(&self) -> String  {
         let mut row: [i8; self.width] = [0; self.width];
         let result: StringBuilder = StringBuilder::new(self.height * (self.width + 1));
         {
             let mut y: i32 = 0;
            while y < self.height {
                {
                    row = self.get_row(y, &row);
                     {
                         let mut x: i32 = 0;
                        while x < self.width {
                            {
                                 let luminance: i32 = row[x] & 0xFF;
                                 let mut c: char;
                                if luminance < 0x40 {
                                    c = '#';
                                } else if luminance < 0x80 {
                                    c = '+';
                                } else if luminance < 0xC0 {
                                    c = '.';
                                } else {
                                    c = ' ';
                                }
                                result.append(c);
                            }
                            x += 1;
                         }
                     }

                    result.append('\n');
                }
                y += 1;
             }
         }

        return result.to_string();
    }
}

