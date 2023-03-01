#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use rxing_common as common;

pub mod aztec;
pub mod client;
pub mod maxicode;
pub mod qrcode;

pub use rxing_common::exceptions;
pub use rxing_common::exceptions::Exceptions;

#[cfg(feature = "image")]
mod buffered_image_luminance_source;

#[cfg(feature = "image")]
pub use buffered_image_luminance_source::*;

#[cfg(test)]
mod PlanarYUVLuminanceSourceTestCase;

#[cfg(test)]
mod rgb_luminance_source_test_case;

pub use rxing_common::PointCallback;

pub use rxing_common::barcode_format::*;

pub use rxing_common::encode_hints::*;

/** Temporary type to ease refactoring and keep backwards-compatibility */
pub type RXingResultPointCallback = PointCallback;

pub use rxing_common::decode_hints::*;

mod writer;
pub use writer::*;

mod reader;
pub use reader::*;

pub use rxing_common::rxing_result_metadata::*;

mod rxing_result;
pub use rxing_result::*;

pub use rxing_common::result_point::*;

pub use rxing_common::result_point_utils;

pub use rxing_common::rxing_result_point::*;

pub use rxing_common::dimension::*;

pub use rxing_common::binarizer::*;

mod binary_bitmap;
pub use binary_bitmap::*;

pub use rxing_common::luminance_source::*;

mod planar_yuv_luminance_source;
pub use planar_yuv_luminance_source::*;

mod rgb_luminance_source;
pub use rgb_luminance_source::*;

pub mod datamatrix;
pub mod multi;
pub mod oned;
pub mod pdf417;

mod multi_format_writer;
pub use multi_format_writer::*;

mod multi_format_reader;
pub use multi_format_reader::*;

// Simple methods to help detect barcodes in common situations
pub mod helpers;

mod luma_luma_source;
pub use luma_luma_source::*;

pub use rxing_common::DecodingHintDictionary;
pub use rxing_common::EncodingHintDictionary;

pub use rxing_common::result_point::ResultPoint;

#[cfg(feature = "svg_read")]
mod svg_luminance_source;
#[cfg(feature = "svg_read")]
pub use svg_luminance_source::*;
