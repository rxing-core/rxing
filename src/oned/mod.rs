#[cfg(feature = "decoders")]
mod one_d_reader;

#[cfg(feature = "decoders")] // there is no rss writer
pub mod rss;

#[cfg(feature = "decoders")]
pub use one_d_reader::*;

mod ean_manufacturer_org_support;
pub use ean_manufacturer_org_support::*;

#[cfg(feature = "decoders")]
mod coda_bar_reader;
#[cfg(feature = "decoders")]
pub use coda_bar_reader::*;

#[cfg(feature = "decoders")]
mod code_39_reader;
#[cfg(feature = "decoders")]
pub use code_39_reader::*;

#[cfg(feature = "decoders")]
mod multi_format_one_d_reader;
#[cfg(feature = "decoders")]
pub use multi_format_one_d_reader::*;

#[cfg(feature = "decoders")]
mod code_93_reader;
#[cfg(feature = "decoders")]
pub use code_93_reader::*;

#[cfg(feature = "decoders")]
mod code_128_reader;
#[cfg(feature = "decoders")]
pub use code_128_reader::*;

#[cfg(feature = "decoders")]
mod itf_reader;
#[cfg(feature = "decoders")]
pub use itf_reader::*;

#[cfg(feature = "decoders")]
mod telepen_reader;
#[cfg(feature = "decoders")]
pub use telepen_reader::*;

#[cfg(feature = "decoders")]
mod upc_ean_reader;
#[cfg(feature = "decoders")]
pub use upc_ean_reader::*;

#[cfg(feature = "decoders")]
mod upc_ean_extension_2_support;
#[cfg(feature = "decoders")]
mod upc_ean_extension_5_support;
#[cfg(feature = "decoders")]
mod upc_ean_extension_support;

#[cfg(feature = "decoders")]
pub use upc_ean_extension_2_support::*;
#[cfg(feature = "decoders")]
pub use upc_ean_extension_5_support::*;
#[cfg(feature = "decoders")]
pub use upc_ean_extension_support::*;

#[cfg(feature = "decoders")]
mod ean_8_reader;
#[cfg(feature = "decoders")]
pub use ean_8_reader::*;

#[cfg(feature = "decoders")]
mod ean_13_reader;
#[cfg(feature = "decoders")]
pub use ean_13_reader::*;

#[cfg(feature = "decoders")]
mod upc_a_reader;
#[cfg(feature = "decoders")]
pub use upc_a_reader::*;

#[cfg(feature = "decoders")]
mod upc_e_reader;
#[cfg(feature = "decoders")]
pub use upc_e_reader::*;

#[cfg(feature = "encoders")]
mod one_d_code_writer;
#[cfg(feature = "encoders")]
pub use one_d_code_writer::*;

#[cfg(feature = "encoders")]
mod coda_bar_writer;
#[cfg(feature = "encoders")]
pub use coda_bar_writer::*;

#[cfg(feature = "decoders")]
mod multi_format_upc_ean_reader;
#[cfg(feature = "decoders")]
pub use multi_format_upc_ean_reader::*;

#[cfg(feature = "encoders")]
mod code_39_writer;
#[cfg(feature = "encoders")]
pub use code_39_writer::*;

#[cfg(feature = "encoders")]
mod code_93_writer;
#[cfg(feature = "encoders")]
pub use code_93_writer::*;

#[cfg(feature = "encoders")]
mod itf_writer;
#[cfg(feature = "encoders")]
pub use itf_writer::*;

#[cfg(feature = "encoders")]
mod code_128_writer;
#[cfg(feature = "encoders")]
pub use code_128_writer::*;

#[cfg(test)]
#[cfg(all(feature = "encoders", feature = "decoders"))]
mod code_128_writer_test_tase;

#[cfg(test)]
#[cfg(all(feature = "encoders", feature = "decoders"))]
mod code_128_reader_test_case;

#[cfg(feature = "encoders")]
mod upc_a_writer;
#[cfg(feature = "encoders")]
pub use upc_a_writer::*;

#[cfg(feature = "encoders")]
mod ean_13_writer;
#[cfg(feature = "encoders")]
pub use ean_13_writer::*;

#[cfg(feature = "encoders")]
mod telepen_writer;
#[cfg(feature = "encoders")]
pub use telepen_writer::*;

#[cfg(feature = "encoders")]
mod upc_ean_writer;
#[cfg(feature = "encoders")]
pub use upc_ean_writer::*;

#[cfg(feature = "encoders")]
mod ean_8_writer;
#[cfg(feature = "encoders")]
pub use ean_8_writer::*;

#[cfg(feature = "encoders")]
mod upc_e_writer;
#[cfg(feature = "encoders")]
pub use upc_e_writer::*;

mod telepen_common;

#[cfg(feature = "decoders")] // This is fine as CPP only reads and never writes
pub mod cpp;

mod oned_constants;

pub(crate) mod upcean_common;
