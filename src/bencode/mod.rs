use std::collections::HashMap;
use thiserror::Error;
pub mod decoder;
pub mod encoder;

/**
 * Represents a Bencode value, which is the core data structure for the Bencode encoding format.
 * Bencode is commonly used in BitTorrent protocol for encoding metadata.
 *
 * The enum contains four variants that represent all possible Bencode data types:
 *
 * 1. String(Vec<u8>):
 *    - Represents a byte string in Bencode
 *    - Stored as raw bytes (Vec<u8>) rather than UTF-8 strings
 *    - Example: "4:spam" in Bencode becomes String(b"spam")
 *
 * 2. Integer(i64):
 *    - Represents a 64-bit signed integer
 *    - Bencode integers are prefixed with 'i' and suffixed with 'e'
 *    - Example: "i42e" in Bencode becomes Integer(42)
 *
 * 3. List(Vec<BencodeValue>):
 *    - Represents an ordered sequence of Bencode values
 *    - Lists are prefixed with 'l' and suffixed with 'e'
 *    - Example: "l4:spami42ee" becomes List([String(b"spam"), Integer(42)])
 *
 * 4. Dict(HashMap<Vec<u8>, BencodeValue>):
 *    - Represents a key-value mapping where keys are byte strings
 *    - Dictionaries are prefixed with 'd' and suffixed with 'e'
 *    - Keys must be strings and are stored as Vec<u8>
 *    - Example: "d3:foo3:bare" becomes Dict({b"foo" => String(b"bar")})
 */
#[derive(Debug, PartialEq, Clone)]
pub enum BencodeValue {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeValue>),
    Dict(HashMap<Vec<u8>, BencodeValue>),
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
