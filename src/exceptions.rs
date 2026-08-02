use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// A barcode checksum verification or error-correction (e.g. Reed-Solomon) failed.
    #[error("checksum error: {message}")]
    Checksum {
        message: Cow<'static, str>,

        #[cfg_attr(
            feature = "serde",
            serde(serialize_with = "serde_helpers::opt_as_string")
        )]
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

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
    pub const NOT_FOUND: Self = Self::NotFound;

    pub const CHECKSUM: Self = Self::Checksum {
        message: Cow::Borrowed("checksum verification failed"),
        source: None,
    };
    pub fn checksum_with<I: Into<Cow<'static, str>>>(x: I) -> Self {
        Self::Checksum {
            message: x.into(),
            source: None,
        }
    }
    pub fn checksum_with_source<
        I: Into<Cow<'static, str>>,
        E: std::error::Error + Send + Sync + 'static,
    >(
        msg: I,
        source: E,
    ) -> Self {
        Self::Checksum {
            message: msg.into(),
            source: Some(Box::new(source)),
        }
    }

    pub const FORMAT: Self = Self::Format {
        message: Cow::Borrowed("malformed barcode"),
        source: None,
    };
    pub fn format_with<I: Into<Cow<'static, str>>>(x: I) -> Self {
        Self::Format {
            message: x.into(),
            source: None,
        }
    }
    pub fn format_with_source<
        I: Into<Cow<'static, str>>,
        E: std::error::Error + Send + Sync + 'static,
    >(
        msg: I,
        source: E,
    ) -> Self {
        Self::Format {
            message: msg.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn invalid_input_with<V: Into<String>>(field: &'static str, value: V) -> Self {
        Self::InvalidInput {
            field,
            value: value.into(),
            cause: None,
        }
    }
    pub fn invalid_input_with_cause<
        V: Into<String>,
        E: std::error::Error + Send + Sync + 'static,
    >(
        field: &'static str,
        value: V,
        cause: E,
    ) -> Self {
        Self::InvalidInput {
            field,
            value: value.into(),
            cause: Some(Box::new(cause)),
        }
    }

    pub const INTERNAL: Self = Self::Internal(Cow::Borrowed("internal error"));
    pub fn internal_with<I: Into<Cow<'static, str>>>(x: I) -> Self {
        Self::Internal(x.into())
    }
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
