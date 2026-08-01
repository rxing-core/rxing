#[cfg(feature = "decoders")]
pub mod detector;
pub mod reedsolomon;

#[cfg(feature = "decoders")]
use crate::Point;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
#[cfg(feature = "decoders")]
mod StringUtilsTestCase;

#[cfg(test)]
mod BitArrayTestCase;

#[cfg(test)]
#[cfg(feature = "decoders")]
mod hybrid_binarizer_test_case;

#[cfg(test)]
#[cfg(feature = "decoders")]
mod adaptive_threshold_binarizer_test_case;

#[cfg(test)]
pub(crate) mod bit_matrix_test_case;

#[cfg(test)]
mod BitSourceTestCase;

#[cfg(test)]
#[cfg(feature = "decoders")]
mod PerspectiveTransformTestCase;

#[cfg(feature = "decoders")]
pub mod string_utils;

mod bit_array;
pub use bit_array::*;

pub type Result<T, E = crate::Error> = std::result::Result<T, E>;

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
#[cfg(feature = "decoders")]
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

#[cfg(feature = "decoders")]
mod perspective_transform;
#[cfg(feature = "decoders")]
pub use perspective_transform::*;

#[cfg(feature = "decoders")]
mod decoder_rxing_result;
#[cfg(feature = "decoders")]
pub use decoder_rxing_result::*;

mod bit_source_builder;
pub use bit_source_builder::*;

#[cfg(feature = "decoders")]
mod grid_sampler;
#[cfg(feature = "decoders")]
pub use grid_sampler::*;

#[cfg(feature = "decoders")]
mod default_grid_sampler;
#[cfg(feature = "decoders")]
pub use default_grid_sampler::*;

mod character_set;
pub use character_set::*;

#[cfg(feature = "decoders")]
mod eci_string_builder;
#[cfg(feature = "decoders")]
pub use eci_string_builder::*;

mod eci_encoder_set;
pub use eci_encoder_set::*;

mod minimal_eci_input;
pub use minimal_eci_input::*;

#[cfg(feature = "decoders")]
mod global_histogram_binarizer;
#[cfg(feature = "decoders")]
pub use global_histogram_binarizer::*;

#[cfg(feature = "decoders")]
mod hybrid_binarizer;
#[cfg(feature = "decoders")]
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

#[cfg(all(feature = "image", feature = "decoders"))]
mod adaptive_threshold_binarizer;
#[cfg(all(feature = "image", feature = "decoders"))]
pub use adaptive_threshold_binarizer::*;

pub type BitFieldBaseType = usize;
pub const BIT_FIELD_BASE_BITS: usize = BitFieldBaseType::BITS as usize;
pub const BIT_FIELD_SHIFT_BITS: usize = BIT_FIELD_BASE_BITS - 1;

#[cfg(feature = "experimental_features")]
mod bitmatrix_sources;

#[cfg(feature = "decoders")]
mod pattern_reader;
