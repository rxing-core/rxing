#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod aztec;
pub mod client;
pub mod common;
mod exceptions;
pub mod maxicode;
pub mod qrcode;

use std::{collections::HashMap, rc::Rc};

pub use exceptions::Exceptions;

#[cfg(feature = "image")]
mod buffered_image_luminance_source;

#[cfg(feature = "image")]
pub use buffered_image_luminance_source::*;

#[cfg(test)]
mod PlanarYUVLuminanceSourceTestCase;

#[cfg(test)]
mod rgb_luminance_source_test_case;

pub type EncodingHintDictionary = HashMap<EncodeHintType, EncodeHintValue>;
pub type DecodingHintDictionary = HashMap<DecodeHintType, DecodeHintValue>;
pub type MetadataDictionary = HashMap<RXingResultMetadataType, RXingResultMetadataValue>;

mod barcode_format;
pub use barcode_format::*;

mod encode_hints;
pub use encode_hints::*;

/// Callback which is invoked when a possible result point (significant
/// point in the barcode image such as a corner) is found.
pub type PointCallback = Rc<dyn Fn(Point)>;

/** Temporary type to ease refactoring and keep backwards-compatibility */
pub type RXingResultPointCallback = PointCallback;

mod decode_hints;
pub use decode_hints::*;

mod writer;
pub use writer::*;

mod reader;
pub use reader::*;

mod rxing_result_metadata;
pub use rxing_result_metadata::*;

mod rxing_result;
pub use rxing_result::*;

mod result_point;
pub use result_point::*;

pub mod result_point_utils;

mod rxing_result_point;
pub use rxing_result_point::*;

mod dimension;
pub use dimension::*;

mod binarizer;
pub use binarizer::*;

mod binary_bitmap;
pub use binary_bitmap::*;

mod luminance_source;
pub use luminance_source::*;

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
mod multi_use_multi_format_reader;
pub use multi_use_multi_format_reader::*;

mod multi_format_reader;
pub use multi_format_reader::*;

// Simple methods to help detect barcodes in common situations
pub mod helpers;

mod luma_luma_source;
pub use luma_luma_source::*;

#[cfg(feature = "svg_read")]
mod svg_luminance_source;
#[cfg(feature = "svg_read")]
pub use svg_luminance_source::*;
