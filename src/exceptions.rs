use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("IllegalStateException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    IllegalState(String),
    #[error("ArithmeticException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Arithmetic(String),
    #[error("ChecksumException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Checksum(String),
    #[error("WriterException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    ReedSolomon(String),
    #[error("IndexOutOfBoundsException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    IndexOutOfBounds(String),
    #[error("RuntimeException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Runtime(String),
    #[error("ParseException{}", if .0.is_empty() { String::new()  } else { format!(" - {}", .0) })]
    Parse(String),

    /// The caller supplied invalid input — bad hint, unsupported
    /// format, out-of-range dimension.
    #[error("invalid input for {field}: {value}")]
    InvalidInput {
        field: &'static str,
        value: String,

        #[cfg_attr(
            feature = "serde",
            serde(serialize_with = "serde_helpers::opt_as_string")
        )]
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// No barcode present. Expected during scanning — try another
    /// reader, orientation, or region.
    #[error("not found")]
    NotFound,

    /// A barcode was located but its structure is invalid.
    #[error("malformed barcode: {message}")]
    Format {
        message: Cow<'static, str>,

        #[cfg_attr(
            feature = "serde",
            serde(serialize_with = "serde_helpers::opt_as_string")
        )]
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// General IO errors, found exclusivly in the helpers module
    #[cfg(feature = "image")]
    #[error("could not read image")]
    #[cfg_attr(feature = "serde", serde(serialize_with = "serde_helpers::as_string"))]
    ImageIo(#[from] image::ImageError),

    /// General file IO errors, found exclusivly in the helpers module
    #[error("could not read file")]
    #[cfg_attr(feature = "serde", serde(serialize_with = "serde_helpers::as_string"))]
    Io(#[from] std::io::Error),

    /// An internal invariant was violated. Always an rxing bug.
    #[error("internal error: {0}")]
    Internal(Cow<'static, str>),
}

impl Error {
    pub const ILLEGAL_STATE: Self = Self::IllegalState(String::new());
    pub fn illegal_state_with<I: Into<String>>(x: I) -> Self {
        Self::IllegalState(x.into())
    }

    pub const ARITHMETIC: Self = Self::Arithmetic(String::new());
    pub fn arithmetic_with<I: Into<String>>(x: I) -> Self {
        Self::Arithmetic(x.into())
    }

    pub const NOT_FOUND: Self = Self::NotFound;

    pub const CHECKSUM: Self = Self::Checksum(String::new());
    pub fn checksum_with<I: Into<String>>(x: I) -> Self {
        Self::Checksum(x.into())
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
}

// 1. Create a tiny helper module to stringify non-serializable fields
#[cfg(feature = "serde")]
mod serde_helpers {
    use serde::Serializer;
    use std::fmt::Display;

    // Serializes any type that implements Display (like std::io::Error) as a string
    pub fn as_string<T: Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    // Serializes an Option containing a dynamic Error as a string (or null)
    pub fn opt_as_string<T: Display + ?Sized, S: Serializer>(
        value: &Option<Box<T>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(err) => serializer.serialize_str(&err.to_string()),
            None => serializer.serialize_none(),
        }
    }
}
