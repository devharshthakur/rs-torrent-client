//! Torrent file structure and parsing logic.
//!
//! This module defines the TorrentFile struct and related helpers for parsing, validating, and working with .torrent file metadata.
use crate::bencode::BencodeValue;
use crate::torrent::info_hash;

use super::TorrentError;
use anyhow::Ok;
use anyhow::Result;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Debug, PartialEq, Clone)]
pub struct TorrentFile {
    pub announce: String,
    pub announce_list: Vec<Vec<String>>,
    pub creation_date: Option<SystemTime>,
    pub comment: String,
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

/**
Parses the concatenated SHA-1 hashes from the `pieces` string.

# Torrent Protocol Context:
The `pieces` key within the `info` dictionary is a single string (byte sequence)
that is a concatenation of 20-byte SHA-1 hashes for each piece of the torrent.
This function breaks that long string into individual 20-byte hash arrays.
The order of these hashes is crucial as it corresponds directly to the piece index.

# How it works:
1. Takes a byte slice (`&[u8]`) which is the raw `pieces` data.
2. Validates that the length of this byte slice is a multiple of 20 (since each hash is 20 bytes).
3. Iterates through the byte slice, taking 20 bytes at a time.
4. Copies each 20-byte chunk into a fixed-size `[u8; 20]` array.
5. Collects these arrays into a `Vec<[u8; 20]>`.
*/
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

/**
Parses the announce list from the torrent file.

# How it works:
1. Takes a `BencodeValue` which is expected to be a list of lists of strings.
2. Iterates through each tier of trackers, converting them into a `Vec<Vec<String>>`.
3. Validates that each tracker URL is a valid UTF-8 string.
4. Returns an error if the structure is invalid.
*/
fn parse_announce_list(value: BencodeValue) -> Result<Vec<Vec<String>>> {
    match value {
        BencodeValue::List(tiers) => {
            let mut result = Vec::new();
            for tier in tiers {
                match tier {
                    BencodeValue::List(trackers) => {
                        let mut tier_vec = Vec::new();
                        for tracker in trackers {
                            match tracker {
                                BencodeValue::String(s) => {
                                    tier_vec.push(String::from_utf8(s).map_err(|e| {
                                        TorrentError::InvalidFormat(format!(
                                            "Invalid tracker URL (not UTF-8): {}",
                                            e
                                        ))
                                    })?);
                                }
                                _ => {
                                    return Err(TorrentError::InvalidFormat(
                                        "Tracker URL not a string".to_string(),
                                    )
                                    .into());
                                }
                            }
                        }
                        result.push(tier_vec);
                    }
                    _ => {
                        return Err(TorrentError::InvalidFormat(
                            "Announce tier not a list".to_string(),
                        )
                        .into());
                    }
                }
            }
            Ok(result)
        }
        _ => Err(TorrentError::InvalidFormat("Announce-list not a list".to_string()).into()),
    }
}

/**
Parses the info dictionary from the torrent file.

# How it works:
1. Extracts required fields such as `piece_length`, `pieces`, `private`, `name`, `length`, and `files`.
2. Validates the structure and content of each field.
3. Constructs an `InfoDict` struct with the parsed data.
4. Returns an error if any required field is missing or invalid.
*/
fn parse_info_dict(value: BencodeValue) -> Result<InfoDict> {
    // Step 1: Validate that the input is a dictionary
    let dict = match value {
        BencodeValue::Dict(d) => d,
        _ => {
            return Err(TorrentError::InvalidFormat("Info is not a dictionary".to_string()).into());
        }
    };

    // Step 2: Extract and validate piece_length (required field)
    let piece_length = match dict.get(&b"piece length".to_vec()) {
        Some(BencodeValue::Integer(i)) => *i,
        _ => return Err(TorrentError::MissingField("piece length".to_string()).into()),
    };

    // Step 3: Extract and validate pieces bytes (required field)
    let pieces_bytes = match dict.get(&b"pieces".to_vec()) {
        Some(BencodeValue::String(s)) => s.clone(),
        _ => return Err(TorrentError::MissingField("pieces".to_string()).into()),
    };

    // Step 4: Extract private flag (optional field, defaults to false)
    let private = match dict.get(&b"private".to_vec()) {
        Some(BencodeValue::Integer(1)) => true,
        _ => false,
    };

    // Step 5: Extract and validate name (required field, must be UTF-8)
    let name = match dict.get(&b"name".to_vec()) {
        Some(BencodeValue::String(s)) => String::from_utf8(s.clone())
            .map_err(|e| TorrentError::InvalidFormat(format!("Invalid name (not UTF-8): {}", e)))?,
        _ => return Err(TorrentError::MissingField("name".to_string()).into()),
    };

    // Step 6: Extract length (optional field for single-file torrents)
    let length = match dict.get(&b"length".to_vec()) {
        Some(BencodeValue::Integer(i)) => Some(*i),
        _ => None,
    };

    // Step 7: Parse files list (optional field for multi-file torrents)
    let files = match dict.get(&b"files".to_vec()) {
        Some(BencodeValue::List(list)) => {
            let mut files_vec = Vec::new();

            // Iterate through each file entry in the list
            for file_val in list {
                // Step 7a: Validate that each file entry is a dictionary
                if let BencodeValue::Dict(file_dict) = file_val {
                    // Step 7b: Extract file length (required for each file)
                    let length = match file_dict.get(&b"length".to_vec()) {
                        Some(BencodeValue::Integer(i)) => *i,
                        _ => {
                            return Err(
                                TorrentError::MissingField("file length".to_string()).into()
                            );
                        }
                    };

                    // Step 7c: Extract and validate file path (required for each file)
                    let path = match file_dict.get(&b"path".to_vec()) {
                        Some(BencodeValue::List(path_list)) => {
                            let mut path_vec = Vec::new();

                            // Step 7d: Process each path component
                            for p in path_list {
                                if let BencodeValue::String(s) = p {
                                    // Convert path component from bytes to UTF-8 string
                                    path_vec.push(String::from_utf8(s.clone()).map_err(|e| {
                                        TorrentError::InvalidFormat(format!(
                                            "Invalid file path (not UTF-8): {}",
                                            e
                                        ))
                                    })?);
                                } else {
                                    return Err(TorrentError::InvalidFormat(
                                        "File path component not a string".to_string(),
                                    )
                                    .into());
                                }
                            }
                            path_vec
                        }
                        _ => return Err(TorrentError::MissingField("file path".to_string()).into()),
                    };

                    // Step 7e: Create FileDict and add to files vector
                    files_vec.push(FileDict { length, path });
                } else {
                    return Err(
                        TorrentError::InvalidFormat("File entry not a dict".to_string()).into(),
                    );
                }
            }
            files_vec
        }
        _ => Vec::new(), // No files list means single-file torrent
    };

    // Step 8: Determine if this is a directory (multi-file) torrent
    let is_directory = !files.is_empty();

    // Step 9: Construct and return the InfoDict
    Ok(InfoDict {
        piece_length,
        pieces: pieces_bytes,
        private,
        name,
        length,
        files,
        is_directory,
    })
}

impl TorrentFile {
    /**
    Returns the total length of all files in the torrent.

    # How it works:
    1. Checks if the torrent is a single-file or multi-file torrent.
    2. For single-file torrents, returns the `length` field.
    3. For multi-file torrents, sums the `length` of each file in the `files` list.
    */
    pub fn total_length(&self) -> i64 {
        if !self.info.is_directory {
            self.info.length.unwrap_or(0)
        } else {
            self.info.files.iter().map(|f| f.length).sum()
        }
    }
    /**
    Returns the total number of pieces in the torrent.

    # How it works:
    1. Derived from the number of 20-byte hashes in `pieces_hash`.
    */
    pub fn num_pieces(&self) -> usize {
        self.pieces_hash.len()
    }

    /**
    Returns the size of a specific piece in the torrent.

    # How it works:
    1. Checks if the piece index is valid.
    2. For all pieces except the last one, returns the `piece_length`.
    3. For the last piece, calculates the size based on the remaining data.
    */
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
    /**
    Returns a vector of file paths that contain data for a specific piece in the torrent.

    # How it works:
    1. Calculates the byte range (start and end) of the requested piece.
    2. For single-file torrents, returns the main file path if the piece is valid.
    3. For multi-file torrents, checks each file's byte range against the piece range
       and returns paths of files that overlap with the piece.
    */
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

    /**
    Parses the torrent file data into a `TorrentFile` struct.

    # How it works:
    1. Extracts the `announce`, `info`, `announce_list`, `creation_date`, `comment`, `created_by`, and `encoding` fields.
    2. Validates the structure and content of each field.
    3. Calculates the `info_hash` and `pieces_hash`.
    4. Constructs a `TorrentFile` struct with the parsed data.
    5. Returns an error if any required field is missing or invalid.
    */
    #[tracing::instrument(level = "debug")]
    pub fn parse(data: BencodeValue) -> Result<TorrentFile> {
        let mut dict = match data {
            BencodeValue::Dict(d) => d,
            _ => {
                return Err(
                    TorrentError::InvalidFormat("Root is not a dictionary".to_string()).into(),
                );
            }
        };

        let announce_bytes = dict
            .remove(&b"announce".to_vec())
            .ok_or(TorrentError::MissingField("announce".to_string()))?;

        let announce = match announce_bytes {
            BencodeValue::String(s) => String::from_utf8(s).map_err(|e| {
                TorrentError::InvalidFormat(format!("Invalid announce Url (not UTF-8): {}", e))
            })?,
            _ => return Err(TorrentError::MissingField("announce(not string)".to_string()).into()),
        };

        let info_dict_value = dict
            .remove(&b"info".to_vec())
            .ok_or(TorrentError::MissingField("info".to_string()))?;

        let info_dict_map = match &info_dict_value {
            BencodeValue::Dict(d) => d.clone(),
            _ => return Err(TorrentError::InvalidFormat("info is not a dict".to_string()).into()),
        };

        let info = parse_info_dict(info_dict_value)?;

        let announce_list =
            if let Some(announce_list_value) = dict.remove(&b"announce-list".to_vec()) {
                parse_announce_list(announce_list_value)?
            } else {
                Vec::new()
            };

        let creation_date = if let Some(data_value) = dict.remove(&b"creation data".to_vec()) {
            match data_value {
                BencodeValue::Integer(timestamp) => {
                    let secs = timestamp
                        .try_into()
                        .map_err(|_| TorrentError::DateParseError)?;
                    let duration = std::time::Duration::from_secs(secs);
                    Some(UNIX_EPOCH + duration)
                }
                _ => {
                    return Err(TorrentError::InvalidFormat(
                        "Creation date not an integer".to_string(),
                    )
                    .into());
                }
            }
        } else {
            None
        };

        let comment = match dict.remove(&b"comment".to_vec()) {
            Some(BencodeValue::String(s)) => String::from_utf8(s).unwrap_or_default(),
            _ => String::new(),
        };
        let created_by = match dict.remove(&b"created by".to_vec()) {
            Some(BencodeValue::String(s)) => String::from_utf8(s).unwrap_or_default(),
            _ => String::new(),
        };
        let encoding = match dict.remove(&b"encoding".to_vec()) {
            Some(BencodeValue::String(s)) => String::from_utf8(s).unwrap_or_default(),
            _ => String::new(),
        };

        let info_hash = info_hash::calculate_info_hash(&info_dict_map)?;
        let pieces_hash = parse_pieces(&info.pieces)?;

        Ok(TorrentFile {
            announce,
            announce_list,
            creation_date,
            comment,
            created_by,
            encoding,
            info,
            info_hash,
            pieces_hash,
        })
    }
}
