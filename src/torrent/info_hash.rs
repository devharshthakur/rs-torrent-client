use crate::bencode;
use crate::bencode::BencodeValue;
use anyhow::Ok;
use anyhow::Result;
use sha1::Digest;
use sha1::Sha1;
use std::collections::HashMap;

/// Calculates the SHA-1 hash of a bencode-encoded info dictionary.
///
/// This function takes a bencode dictionary containing torrent metadata and:
/// 1. Encodes the dictionary into bencode format
/// 2. Calculates the SHA-1 hash of the encoded data
/// 3. Returns the 20-byte hash as a fixed-size array
///
/// # Arguments
/// * `info_dict` - A HashMap containing the torrent's info dictionary
///
/// # Returns
/// * `Result<[u8;20]>` - A 20-byte array containing the SHA-1 hash, or an error if encoding fails
///
/// # Example
/// ```
/// let info_dict = HashMap::new();
/// let hash = calculate_info_hash(&info_dict)?;
/// assert_eq!(hash.len(), 20);
/// ```
pub fn calculate_info_hash(info_dict: &HashMap<Vec<u8>, BencodeValue>) -> Result<[u8; 20]> {
    let mut buffer = Vec::new();
    bencode::encoder::encode(&mut buffer, &BencodeValue::Dict(info_dict.clone()))?;

    let mut hasher = Sha1::new();
    hasher.update(&buffer);

    let result = hasher.finalize();
    let mut info_hash = [0u8; 20];
    info_hash.copy_from_slice(&result[..]);

    Ok(info_hash)
}
