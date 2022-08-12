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
 * This object extends LuminanceSource around an array of YUV data returned from the camera driver,
 * with the option to crop to a rectangle within the full data. This can be used to exclude
 * superfluous pixels around the perimeter and speed up decoding.
 *
 * It works for any pixel format where the Y channel is planar and appears first, including
 * YCbCr_420_SP and YCbCr_422_SP.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

 const THUMBNAIL_SCALE_FACTOR: i32 = 2;
pub struct PlanarYUVLuminanceSource {
    super: LuminanceSource;

     let yuv_data: Vec<i8>;

     let data_width: i32;

     let data_height: i32;

     let left: i32;

     let top: i32;
}

impl PlanarYUVLuminanceSource {

    pub fn new( yuv_data: &Vec<i8>,  data_width: i32,  data_height: i32,  left: i32,  top: i32,  width: i32,  height: i32,  reverse_horizontal: bool) -> PlanarYUVLuminanceSource {
        super(width, height);
        if left + width > data_width || top + height > data_height {
            throw IllegalArgumentException::new("Crop rectangle does not fit within image data.");
        }
        let .yuvData = yuv_data;
        let .dataWidth = data_width;
        let .dataHeight = data_height;
        let .left = left;
        let .top = top;
        if reverse_horizontal {
            self.reverse_horizontal(width, height);
        }
    }

    pub fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8>  {
        if y < 0 || y >= get_height() {
            throw IllegalArgumentException::new(format!("Requested row is outside the image: {}", y));
        }
         let width: i32 = get_width();
        if row == null || row.len() < width {
            row = : [i8; width] = [0; width];
        }
         let offset: i32 = (y + self.top) * self.data_width + self.left;
        System::arraycopy(&self.yuv_data, offset, &row, 0, width);
        return row;
    }

    pub fn  get_matrix(&self) -> Vec<i8>  {
         let width: i32 = get_width();
         let height: i32 = get_height();
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.data_width && height == self.data_height {
            return self.yuv_data;
        }
         let area: i32 = width * height;
         let matrix: [i8; area] = [0; area];
         let input_offset: i32 = self.top * self.data_width + self.left;
        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.data_width {
            System::arraycopy(&self.yuv_data, input_offset, &matrix, 0, area);
            return matrix;
        }
        // Otherwise copy one cropped row at a time.
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let output_offset: i32 = y * width;
                    System::arraycopy(&self.yuv_data, input_offset, &matrix, output_offset, width);
                    input_offset += self.data_width;
                }
                y += 1;
             }
         }

        return matrix;
    }

    pub fn  is_crop_supported(&self) -> bool  {
        return true;
    }

    pub fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> LuminanceSource  {
        return PlanarYUVLuminanceSource::new(&self.yuv_data, self.data_width, self.data_height, self.left + left, self.top + top, width, height, false);
    }

    pub fn  render_thumbnail(&self) -> Vec<i32>  {
         let width: i32 = get_width() / THUMBNAIL_SCALE_FACTOR;
         let height: i32 = get_height() / THUMBNAIL_SCALE_FACTOR;
         let mut pixels: [i32; width * height] = [0; width * height];
         let yuv: Vec<i8> = self.yuv_data;
         let input_offset: i32 = self.top * self.data_width + self.left;
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let output_offset: i32 = y * width;
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                 let grey: i32 = yuv[input_offset + x * THUMBNAIL_SCALE_FACTOR] & 0xff;
                                pixels[output_offset + x] = 0xFF000000 | (grey * 0x00010101);
                            }
                            x += 1;
                         }
                     }

                    input_offset += self.data_width * THUMBNAIL_SCALE_FACTOR;
                }
                y += 1;
             }
         }

        return pixels;
    }

    /**
   * @return width of image from {@link #renderThumbnail()}
   */
    pub fn  get_thumbnail_width(&self) -> i32  {
        return get_width() / THUMBNAIL_SCALE_FACTOR;
    }

    /**
   * @return height of image from {@link #renderThumbnail()}
   */
    pub fn  get_thumbnail_height(&self) -> i32  {
        return get_height() / THUMBNAIL_SCALE_FACTOR;
    }

    fn  reverse_horizontal(&self,  width: i32,  height: i32)   {
         let yuv_data: Vec<i8> = self.yuvData;
         {
             let mut y: i32 = 0, let row_start: i32 = self.top * self.data_width + self.left;
            while y < height {
                {
                     let middle: i32 = row_start + width / 2;
                     {
                         let mut x1: i32 = row_start, let mut x2: i32 = row_start + width - 1;
                        while x1 < middle {
                            {
                                 let temp: i8 = yuv_data[x1];
                                yuv_data[x1] = yuv_data[x2];
                                yuv_data[x2] = temp;
                            }
                            x1 += 1;
                            x2 -= 1;
                         }
                     }

                }
                y += 1;
                row_start += self.data_width;
             }
         }

    }
}

