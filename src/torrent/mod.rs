//! Torrent file parsing and error handling logic.
//!
//! This module provides types and error handling for working with .torrent files, including parsing, validation, and error reporting.
use serde_bencode;
use thiserror::Error;
pub mod file;
pub mod info_hash;
#[derive(Debug, Error)]
pub enum TorrentError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Bencode decoding error: {0}")]
    Bencode(#[from] crate::bencode::BencodeError),

    #[error("Invalid torrent file format: {0}")]
    InvalidFormat(String),

    #[error("Missing or invalid field: {0}")]
    MissingField(String),

    #[error("Invalid data type for field: {0}")]
    InvalidFieldType(String),

    #[error("Invalid pieces hash length")]
    InvalidPiecesHashLength,

    #[error("Path conversion error: {0}")]
    PathConversion(String),

    #[error("Date parse error")]
    DateParseError,

    #[error("Url Parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Bencode deserialization error: {0}")]
    BencodeDe(#[from] serde_bencode::Error),

    #[error("Handshake failed: Invalid protocol identifier")]
    HandshakeInvalidProtocol,

    #[error("Handshake failed: Info hash mismatch")]
    HandshakeInfoHashMismatch,

    #[error("Handshake timed out")]
    HandshakeTimeout,
}

/// Result type for torrent operations derived from `std::result`
pub type TorrentResult<T> = std::result::Result<T, TorrentError>;
