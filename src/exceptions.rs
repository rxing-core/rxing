#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Exceptions {
    #[error("IllegalArgumentException{}", if .0.is_empty() { String::new() } else { format!(" - {}", .0) })]
    IllegalArgumentException(String),
    #[error("UnsupportedOperationException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    UnsupportedOperationException(String),
    #[error("IllegalStateException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    IllegalStateException(String),
    #[error("ArithmeticException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    ArithmeticException(String),
    #[error("NotFoundException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    NotFoundException(String),
    #[error("FormatException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    FormatException(String),
    #[error("ChecksumException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    ChecksumException(String),
    #[error("ReaderException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    ReaderException(String),
    #[error("WriterException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    WriterException(String),
    #[error("ReedSolomonException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    ReedSolomonException(String),
    #[error("IndexOutOfBoundsException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    IndexOutOfBoundsException(String),
    #[error("RuntimeException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    RuntimeException(String),
    #[error("ParseException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    ParseException(String),
    #[error("ReaderDecodeException")]
    ReaderDecodeException(),
}

impl Exceptions {
    pub const ILLEGAL_ARGUMENT: Self = Self::IllegalArgumentException(String::new());
    pub fn illegal_argument_with<I: Into<String>>(x: I) -> Self {
        Self::IllegalArgumentException(x.into())
    }

    pub const UNSUPPORTED_OPERATION: Self = Self::UnsupportedOperationException(String::new());
    pub fn unsupported_operation_with<I: Into<String>>(x: I) -> Self {
        Self::UnsupportedOperationException(x.into())
    }

    pub const ILLEGAL_STATE: Self = Self::IllegalStateException(String::new());
    pub fn illegal_state_with<I: Into<String>>(x: I) -> Self {
        Self::IllegalStateException(x.into())
    }

    pub const ARITHMETIC: Self = Self::ArithmeticException(String::new());
    pub fn arithmetic_with<I: Into<String>>(x: I) -> Self {
        Self::ArithmeticException(x.into())
    }

    pub const NOT_FOUND: Self = Self::NotFoundException(String::new());
    pub fn not_found_with<I: Into<String>>(x: I) -> Self {
        Self::NotFoundException(x.into())
    }

    pub const FORMAT: Self = Self::FormatException(String::new());
    pub fn format_with<I: Into<String>>(x: I) -> Self {
        Self::FormatException(x.into())
    }

    pub const CHECKSUM: Self = Self::ChecksumException(String::new());
    pub fn checksum_with<I: Into<String>>(x: I) -> Self {
        Self::ChecksumException(x.into())
    }

    pub const READER: Self = Self::ReaderException(String::new());
    pub fn reader_with<I: Into<String>>(x: I) -> Self {
        Self::ReaderException(x.into())
    }

    pub const WRITER: Self = Self::WriterException(String::new());
    pub fn writer_with<I: Into<String>>(x: I) -> Self {
        Self::WriterException(x.into())
    }

    pub const REED_SOLOMON: Self = Self::ReedSolomonException(String::new());
    pub fn reed_solomon_with<I: Into<String>>(x: I) -> Self {
        Self::ReedSolomonException(x.into())
    }

    pub const INDEX_OUT_OF_BOUNDS: Self = Self::IndexOutOfBoundsException(String::new());
    pub fn index_out_of_bounds_with<I: Into<String>>(x: I) -> Self {
        Self::IndexOutOfBoundsException(x.into())
    }

    pub const RUNTIME: Self = Self::RuntimeException(String::new());
    pub fn runtime_with<I: Into<String>>(x: I) -> Self {
        Self::RuntimeException(x.into())
    }

    pub const PARSE: Self = Self::ParseException(String::new());
    pub fn parse_with<I: Into<String>>(x: I) -> Self {
        Self::ParseException(x.into())
    }
}
