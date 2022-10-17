/*
 * Copyright 2014 ZXing authors
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

// import org.junit.Assert;
// import org.junit.Test;

// /**
//  * Tests {@link RGBLuminanceSource}.
//  */
// public final class RGBLuminanceSourceTestCase extends Assert {

use crate::{RGBLuminanceSource, LuminanceSource};

  const SRC_DATA : [u32;9] = [0x000000, 0x7F7F7F, 0xFFFFFF,
  0xFF0000, 0x00FF00, 0x0000FF,
  0x0000FF, 0x00FF00, 0xFF0000];

  #[test]
  fn testCrop() {
    let SOURCE = RGBLuminanceSource::new_with_width_height_pixels(3,3,&SRC_DATA.to_vec());

    assert!(SOURCE.isCropSupported());
    let cropped = SOURCE.crop(1, 1, 1, 1).unwrap();
    assert_eq!(1, cropped.getHeight());
    assert_eq!(1, cropped.getWidth());
    assert_eq!(vec![ 0x7F ], cropped.getRow(0, &vec![0;0]));
  }

  #[test]
  fn testMatrix() {
    let SOURCE = RGBLuminanceSource::new_with_width_height_pixels(3,3,&SRC_DATA.to_vec());

    assert_eq!(vec![ 0x00, 0x7F,  0xFF, 0x3F, 0x7F, 0x3F, 0x3F, 0x7F, 0x3F ],
                      SOURCE.getMatrix());
    let croppedFullWidth = SOURCE.crop(0, 1, 3, 2).unwrap();
    assert_eq!(vec![ 0x3F, 0x7F, 0x3F, 0x3F, 0x7F, 0x3F ],
                      croppedFullWidth.getMatrix());
    let croppedCorner = SOURCE.crop(1, 1, 2, 2).unwrap();
    assert_eq!(vec![ 0x7F, 0x3F, 0x7F, 0x3F ],
                      croppedCorner.getMatrix());
  }

  #[test]
  fn testGetRow() {
    let SOURCE = RGBLuminanceSource::new_with_width_height_pixels(3,3,&SRC_DATA.to_vec());

    assert_eq!(vec![ 0x3F, 0x7F, 0x3F ], SOURCE.getRow(2, &vec![0;3]));
  }

  // #[test]
  // fn testToString() {
  //   let SOURCE = RGBLuminanceSource::new_with_width_height_pixels(3,3,&src_data.to_vec());

  //   assert_eq!("#+ \n#+#\n#+#\n", SOURCE.toString());
  // }

// }
