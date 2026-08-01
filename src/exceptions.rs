#[cfg(feature = "serde")]
use serde::{Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("IllegalArgumentException{}", if .0.is_empty() { String::new() } else { format!(" - {}", .0) })]
    IllegalArgument(String),
    #[error("UnsupportedOperationException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    UnsupportedOperation(String),
    #[error("IllegalStateException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    IllegalState(String),
    #[error("ArithmeticException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Arithmetic(String),
    #[error("NotFoundException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    NotFound(String),
    #[error("FormatException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Format(String),
    #[error("ChecksumException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Checksum(String),
    #[error("WriterException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Writer(String),
    #[error("ReedSolomonException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    ReedSolomon(String),
    #[error("IndexOutOfBoundsException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    IndexOutOfBounds(String),
    #[error("RuntimeException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Runtime(String),
    #[error("ParseException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Parse(String),
}

impl Error {
    pub const ILLEGAL_ARGUMENT: Self = Self::IllegalArgument(String::new());
    pub fn illegal_argument_with<I: Into<String>>(x: I) -> Self {
        Self::IllegalArgument(x.into())
    }

    pub const UNSUPPORTED_OPERATION: Self = Self::UnsupportedOperation(String::new());
    pub fn unsupported_operation_with<I: Into<String>>(x: I) -> Self {
        Self::UnsupportedOperation(x.into())
    }

    pub const ILLEGAL_STATE: Self = Self::IllegalState(String::new());
    pub fn illegal_state_with<I: Into<String>>(x: I) -> Self {
        Self::IllegalState(x.into())
    }

    pub const ARITHMETIC: Self = Self::Arithmetic(String::new());
    pub fn arithmetic_with<I: Into<String>>(x: I) -> Self {
        Self::Arithmetic(x.into())
    }

    pub const NOT_FOUND: Self = Self::NotFound(String::new());
    pub fn not_found_with<I: Into<String>>(x: I) -> Self {
        Self::NotFound(x.into())
    }

    pub const FORMAT: Self = Self::Format(String::new());
    pub fn format_with<I: Into<String>>(x: I) -> Self {
        Self::Format(x.into())
    }

    pub const CHECKSUM: Self = Self::Checksum(String::new());
    pub fn checksum_with<I: Into<String>>(x: I) -> Self {
        Self::Checksum(x.into())
    }

    pub const WRITER: Self = Self::Writer(String::new());
    pub fn writer_with<I: Into<String>>(x: I) -> Self {
        Self::Writer(x.into())
    }

    pub const REED_SOLOMON: Self = Self::ReedSolomon(String::new());
    pub fn reed_solomon_with<I: Into<String>>(x: I) -> Self {
        Self::ReedSolomon(x.into())
    }

    pub const INDEX_OUT_OF_BOUNDS: Self = Self::IndexOutOfBounds(String::new());
    pub fn index_out_of_bounds_with<I: Into<String>>(x: I) -> Self {
        Self::IndexOutOfBounds(x.into())
    }

    pub const RUNTIME: Self = Self::Runtime(String::new());
    pub fn runtime_with<I: Into<String>>(x: I) -> Self {
        Self::Runtime(x.into())
    }

    pub const PARSE: Self = Self::Parse(String::new());
    pub fn parse_with<I: Into<String>>(x: I) -> Self {
        Self::Parse(x.into())
    }
}
