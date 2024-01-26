pub mod detector;
pub mod reedsolomon;

use crate::Point;

#[cfg(test)]
mod StringUtilsTestCase;

#[cfg(test)]
mod BitArrayTestCase;

#[cfg(test)]
pub(crate) mod bit_matrix_test_case;

#[cfg(test)]
mod BitSourceTestCase;

#[cfg(test)]
mod PerspectiveTransformTestCase;

mod string_utils;
pub use string_utils::*;

mod bit_array;
pub use bit_array::*;

pub type Result<T, E = crate::Exceptions> = std::result::Result<T, E>;

/*
 * Copyright 2007 ZXing authors
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

// package com.google.zxing.common;

// import com.google.zxing.Point;

/**
 * <p>Encapsulates the result of detecting a barcode in an image. This includes the raw
 * matrix of black/white pixels corresponding to the barcode, and possibly points of interest
 * in the image, like the location of finder patterns or corners of the barcode in the image.</p>
 *
 * @author Sean Owen
 */
pub trait DetectorRXingResult {
    fn getBits(&self) -> &BitMatrix;

    fn getPoints(&self) -> &[Point];
}

// pub struct DetectorRXingResult {
//     bits: BitMatrix,
//     points: Vec<Point>,
// }

mod bit_matrix;
pub use bit_matrix::*;

mod eci_input;
pub use eci_input::*;

mod bit_source;
pub use bit_source::*;

mod perspective_transform;
pub use perspective_transform::*;

mod decoder_rxing_result;
pub use decoder_rxing_result::*;

mod bit_source_builder;
pub use bit_source_builder::*;

mod grid_sampler;
pub use grid_sampler::*;

mod default_grid_sampler;
pub use default_grid_sampler::*;

mod character_set;
pub use character_set::*;

mod eci_string_builder;
pub use eci_string_builder::*;

mod eci_encoder_set;
pub use eci_encoder_set::*;

mod minimal_eci_input;
pub use minimal_eci_input::*;

mod global_histogram_binarizer;
pub use global_histogram_binarizer::*;

mod hybrid_binarizer;
pub use hybrid_binarizer::*;

mod eci;
pub use eci::*;

mod quad;
pub use quad::*;

pub mod cpp_essentials;

mod line_orientation;
pub use line_orientation::LineOrientation;

#[cfg(feature = "otsu_level")]
mod otsu_level_binarizer;
#[cfg(feature = "otsu_level")]
pub use otsu_level_binarizer::*;
