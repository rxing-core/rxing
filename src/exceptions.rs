use std::fmt;

#[derive(Debug)]
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
    ReaderDecodeException()
}