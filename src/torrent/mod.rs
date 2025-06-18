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
}

pub type TorrentResult<T> = std::result::Result<T, TorrentError>;
