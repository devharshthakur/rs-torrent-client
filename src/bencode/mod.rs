use std::collections::HashMap;
use thiserror::Error;
pub mod decoder;
pub mod encoder;

/**
* Represents a Bencode value.
* This enum acts as a type-safe equivalent to Go's `interface{}`
  for Bencode data structures.
*/
#[derive(Debug, PartialEq, Clone)]
pub enum BencodeValue {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeValue>),
    Dicts(HashMap<Vec<u8>, BencodeValue>),
}

/// Custom error type for Bencode operations.
#[derive(Debug, Error)]
pub enum BencodeError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid bencode format:{0}")]
    InvalidFormat(String),

    #[error("Invalid integer format")]
    InvalidInteger,

    #[error("Invalid String length")]
    InvalidStringLength,

    #[error("Unexpected end of input")]
    UnexpectedEOI,

    #[error("Cannot encode type")]
    CannotEncodeType(&'static str),

    #[error("Dictionary keys must be strings")]
    DictKeyNotString,
}

pub type BencodeResult<T> = std::result::Result<T, BencodeError>;
