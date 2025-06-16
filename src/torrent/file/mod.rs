use super::TorrentError;
use super::TorrentResult;
use crate::bencode::BencodeValue;
use anyhow::Ok;
use anyhow::Result;
use clap::builder::Str;
use sha1::Digest;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tracing::instrument;

#[derive(Debug, PartialEq, Clone)]
pub struct TorrentFile {
    pub announce: String,
    pub announce_list: Vec<Vec<String>>,
    pub creation_date: Option<SystemTime>,
    pub commnet: String,
    pub created_by: String,
    pub encoding: String,
    pub info: InfoDict,
    pub info_hash: [u8; 20],
    pub pieces_hash: Vec<[u8; 20]>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfoDict {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: bool,
    pub name: String,
    pub length: Option<i64>,
    pub files: Vec<FileDict>,
    pub is_directory: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileDict {
    pub length: i64,
    pub path: Vec<String>,
}

/// Helper function to parse the concatenated SHA-1 hashes from the `pieces` string.
///
/// # Torrent Protocol Context:
/// The `pieces` key within the `info` dictionary is a single string (byte sequence)
/// that is a concatenation of 20-byte SHA-1 hashes for each piece of the torrent.
/// This function breaks that long string into individual 20-byte hash arrays.
/// The order of these hashes is crucial as it corresponds directly to the piece index.
///
/// # How it works:
/// 1. Takes a byte slice (`&[u8]`) which is the raw `pieces` data.
/// 2. Validates that the length of this byte slice is a multiple of 20 (since each hash is 20 bytes).
/// 3. Iterates through the byte slice, taking 20 bytes at a time.
/// 4. Copies each 20-byte chunk into a fixed-size `[u8; 20]` array.
/// 5. Collects these arrays into a `Vec<[u8; 20]>`.
#[tracing::instrument(level = "trace")]
fn parse_pieces(pieces_bytes: &[u8]) -> Result<Vec<[u8; 20]>> {
    if pieces_bytes.len() % 20 != 0 {
        return Err(TorrentError::InvalidPiecesHashLength.into());
    }
    let num_pieces = pieces_bytes.len() / 20;
    let mut hashes = Vec::with_capacity(num_pieces);
    for i in 0..num_pieces {
        let start = i * 20;
        let end = start + 20;

        let mut hash_array = [0u8; 20];
        hash_array.copy_from_slice(&pieces_bytes[start..end]);
        hashes.push(hash_array);
    }
    Ok(hashes)
}

impl TorrentFile {
    fn calculate_sha1_hash(data: &[u8]) -> [u8; 20] {
        sha1::Sha1::digest(data).into()
    }
    /// Returns the total length of all files in the torrent.
    pub fn total_length(&self) -> i64 {
        if !self.info.is_directory {
            self.info.length.unwrap_or(0)
        } else {
            self.info.files.iter().map(|f| f.length).sum()
        }
    }
    /// Returns the total number of pieces in the torrent.
    /// Derived from the number of 20-byte hashes in `pieces_hash`.
    pub fn num_pieces(&self) -> usize {
        self.pieces_hash.len()
    }

    pub fn piece_size(&self, index: usize) -> i64 {
        if index >= self.num_pieces() {
            return 0;
        }
        if index < self.num_pieces() - 1 {
            self.info.piece_length
        } else {
            let total_length = self.total_length();
            let full_pieces_length = (self.num_pieces() - 1) as i64 * self.info.piece_length;
            let last_piece_size = total_length - full_pieces_length;

            if last_piece_size == 0 && self.num_pieces() > 0 {
                self.info.piece_length
            } else {
                last_piece_size
            }
        }
    }

    /// Returns a vector of file paths that contain data for a specific piece in the torrent.
    ///
    /// This function determines which files contain data for a given piece index by:
    /// 1. Calculating the byte range (start and end) of the requested piece
    /// 2. For single-file torrents, returns the main file path if the piece is valid
    /// 3. For multi-file torrents, checks each file's byte range against the piece range
    ///    and returns paths of files that overlap with the piece
    ///
    /// # Arguments
    /// * `index` - The index of the piece to find files for
    ///
    /// # Returns
    /// A vector of `PathBuf` containing the paths of files that contain data for the specified piece.
    /// Returns an empty vector if the piece index is invalid.
    pub fn file_paths_for_piece(&self, index: usize) -> Vec<PathBuf> {
        if index >= self.num_pieces() {
            return Vec::new();
        }
        let piece_start = (index as i64) * self.info.piece_length;
        let piece_end = piece_start + self.piece_size(index);
        let mut current_data_position: i64 = 0;
        let mut result_paths = Vec::new();
        if !self.info.is_directory {
            result_paths.push(PathBuf::from(&self.info.name));
        } else {
            for file_info in &self.info.files {
                let file_start = current_data_position;
                let file_end = file_start + file_info.length;
                if file_end > piece_start && file_start < piece_end {
                    let mut full_path = PathBuf::from(&self.info.name);
                    for componet in &file_info.path {
                        full_path = full_path.join(componet)
                    }
                    result_paths.push(full_path);
                }
                current_data_position = file_end;
            }
        }
        result_paths
    }
}
