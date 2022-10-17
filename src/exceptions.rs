use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum Exceptions {
    IllegalArgumentException(String),
    UnsupportedOperationException(String),
    IllegalStateException(String),
    ArithmeticException(String),
    NotFoundException(String),
    FormatException(String),
    ChecksumException(String),
    ReaderException(String),
    WriterException(String),
    ReedSolomonException(String),
    IndexOutOfBoundsException(String),
    RuntimeException(String),
    ParseException(String),
    ReaderDecodeException(),
}

impl fmt::Display for Exceptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exceptions::IllegalArgumentException(a) => {
                write!(f, "IllegalArgumentException - {}", a)
            }
            Exceptions::UnsupportedOperationException(a) => {
                write!(f, "UnsupportedOperationException - {}", a)
            }
            Exceptions::IllegalStateException(a) => write!(f, "IllegalStateException - {}", a),
            Exceptions::ArithmeticException(a) => write!(f, "ArithmeticException - {}", a),
            Exceptions::NotFoundException(a) => write!(f, "NotFoundException - {}", a),
            Exceptions::FormatException(a) => write!(f, "FormatException - {}", a),
            Exceptions::ChecksumException(a) => write!(f, "ChecksumException - {}", a),
            Exceptions::ReaderException(a) => write!(f, "ReaderException - {}", a),
            Exceptions::WriterException(a) => write!(f, "WriterException - {}", a),
            Exceptions::ReedSolomonException(a) => write!(f, "ReedSolomonException - {}", a),
            Exceptions::IndexOutOfBoundsException(a) => {
                write!(f, "IndexOutOfBoundsException - {}", a)
            }
            Exceptions::RuntimeException(a) => write!(f, "RuntimeException - {}", a),
            Exceptions::ParseException(a) => write!(f, "ParseException - {}", a),
            Exceptions::ReaderDecodeException() => write!(f, "ReaderDecodeException - -"),
        }
    }
}

impl Error for Exceptions {}
