#[cfg(feature = "decoders")]
pub mod data_mask;
#[cfg(feature = "decoders")]
pub mod decoder;
#[cfg(feature = "decoders")]
pub mod detector;
#[cfg(feature = "decoders")]
mod qr_cpp_reader;

#[cfg(feature = "decoders")]
pub use qr_cpp_reader::QrReader;

#[cfg(feature = "decoders")]
pub mod bitmatrix_parser;

mod qr_type;
pub use qr_type::Type;

#[cfg(test)]
#[cfg(feature = "decoders")]
mod test;
