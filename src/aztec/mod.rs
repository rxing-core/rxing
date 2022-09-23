mod AztecDetectorResult;
mod aztec_reader;
mod aztec_writer;
pub mod decoder;
pub mod detector;
pub mod encoder;

pub use aztec_reader::*;
pub use aztec_writer::*;

#[cfg(test)]
mod DecoderTest;
#[cfg(test)]
mod EncoderTest;
#[cfg(test)]
mod DetectorTest;

mod shared_test_methods;