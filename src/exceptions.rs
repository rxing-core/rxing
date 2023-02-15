use std::{error::Error, fmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum Exceptions {
    IllegalArgumentException(Option<String>),
    UnsupportedOperationException(Option<String>),
    IllegalStateException(Option<String>),
    ArithmeticException(Option<String>),
    NotFoundException(Option<String>),
    FormatException(Option<String>),
    ChecksumException(Option<String>),
    ReaderException(Option<String>),
    WriterException(Option<String>),
    ReedSolomonException(Option<String>),
    IndexOutOfBoundsException(Option<String>),
    RuntimeException(Option<String>),
    ParseException(Option<String>),
    ReaderDecodeException(),
}

impl Exceptions {
    pub fn illegalArgument<I: Into<String>>(x: I) -> Self {
        Self::IllegalArgumentException(Some(x.into()))
    }

    pub fn illegalArgumentEmpty() -> Self {
        Self::IllegalArgumentException(None)
    }

    pub fn unsupportedOperation<I: Into<String>>(x: I) -> Self {
        Self::UnsupportedOperationException(Some(x.into()))
    }

    pub fn unsupportedOperationEmpty() -> Self {
        Self::UnsupportedOperationException(None)
    }

    pub fn illegalState<I: Into<String>>(x: I) -> Self {
        Self::IllegalStateException(Some(x.into()))
    }

    pub fn illegalStateEmpty() -> Self {
        Self::IllegalStateException(None)
    }

    pub fn arithmetic<I: Into<String>>(x: I) -> Self {
        Self::ArithmeticException(Some(x.into()))
    }

    pub fn arithmeticEmpty() -> Self {
        Self::ArithmeticException(None)
    }

    pub fn notFound<I: Into<String>>(x: I) -> Self {
        Self::NotFoundException(Some(x.into()))
    }

    pub fn notFoundEmpty() -> Self {
        Self::NotFoundException(None)
    }

    pub fn format<I: Into<String>>(x: I) -> Self {
        Self::FormatException(Some(x.into()))
    }

    pub fn formatEmpty() -> Self {
        Self::FormatException(None)
    }

    pub fn checksum<I: Into<String>>(x: I) -> Self {
        Self::ChecksumException(Some(x.into()))
    }

    pub fn checksumEmpty() -> Self {
        Self::ChecksumException(None)
    }

    pub fn reader<I: Into<String>>(x: I) -> Self {
        Self::ReaderException(Some(x.into()))
    }

    pub fn readerEmpty() -> Self {
        Self::ReaderException(None)
    }

    pub fn writer<I: Into<String>>(x: I) -> Self {
        Self::WriterException(Some(x.into()))
    }

    pub fn writerEmpty() -> Self {
        Self::WriterException(None)
    }

    pub fn reedSolomon<I: Into<String>>(x: I) -> Self {
        Self::ReedSolomonException(Some(x.into()))
    }

    pub fn reedSolomonEmpty() -> Self {
        Self::ReedSolomonException(None)
    }

    pub fn indexOutOfBounds<I: Into<String>>(x: I) -> Self {
        Self::IndexOutOfBoundsException(Some(x.into()))
    }

    pub fn indexOutOfBoundsEmpty() -> Self {
        Self::IndexOutOfBoundsException(None)
    }

    pub fn runtime<I: Into<String>>(x: I) -> Self {
        Self::RuntimeException(Some(x.into()))
    }

    pub fn runtimeEmpty() -> Self {
        Self::RuntimeException(None)
    }

    pub fn parse<I: Into<String>>(x: I) -> Self {
        Self::ParseException(Some(x.into()))
    }

    pub fn parseEmpty() -> Self {
        Self::ParseException(None)
    }
}

impl fmt::Display for Exceptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exceptions::IllegalArgumentException(Some(a)) => {
                write!(f, "IllegalArgumentException - {a}")
            }

            Exceptions::UnsupportedOperationException(Some(a)) => {
                write!(f, "UnsupportedOperationException - {a}")
            }

            Exceptions::IllegalStateException(Some(a)) => {
                write!(f, "IllegalStateException - {a}")
            }
            Exceptions::ArithmeticException(Some(a)) => write!(f, "ArithmeticException - {a}"),
            Exceptions::NotFoundException(Some(a)) => write!(f, "NotFoundException - {a}"),
            Exceptions::FormatException(Some(a)) => write!(f, "FormatException - {a}"),
            Exceptions::ChecksumException(Some(a)) => write!(f, "ChecksumException - {a}"),
            Exceptions::ReaderException(Some(a)) => write!(f, "ReaderException - {a}"),
            Exceptions::WriterException(Some(a)) => write!(f, "WriterException - {a}"),
            Exceptions::ReedSolomonException(Some(a)) => write!(f, "ReedSolomonException - {a}"),
            Exceptions::IndexOutOfBoundsException(Some(a)) => {
                write!(f, "IndexOutOfBoundsException - {a}")
            }

            Exceptions::RuntimeException(Some(a)) => write!(f, "RuntimeException - {a}"),
            Exceptions::ParseException(Some(a)) => write!(f, "ParseException - {a}"),

            Exceptions::IllegalArgumentException(None) => write!(f, "IllegalArgumentException"),

            Exceptions::UnsupportedOperationException(None) => {
                write!(f, "UnsupportedOperationException")
            }
            Exceptions::IllegalStateException(None) => write!(f, "IllegalStateException"),
            Exceptions::ArithmeticException(None) => write!(f, "ArithmeticException"),
            Exceptions::NotFoundException(None) => write!(f, "NotFoundException"),
            Exceptions::FormatException(None) => write!(f, "FormatException"),
            Exceptions::ChecksumException(None) => write!(f, "ChecksumException"),
            Exceptions::ReaderException(None) => write!(f, "ReaderException"),
            Exceptions::WriterException(None) => write!(f, "WriterException"),
            Exceptions::ReedSolomonException(None) => write!(f, "ReedSolomonException"),
            Exceptions::IndexOutOfBoundsException(None) => write!(f, "IndexOutOfBoundsException"),

            Exceptions::RuntimeException(None) => write!(f, "RuntimeException"),
            Exceptions::ParseException(None) => write!(f, "ParseException"),

            Exceptions::ReaderDecodeException() => write!(f, "ReaderDecodeException"),
        }
    }
}

impl Error for Exceptions {}
