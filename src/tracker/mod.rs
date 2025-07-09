//! Tracker client logic for the BitTorrent protocol.
//!
//! This module implements communication with BitTorrent trackers, including:
//! - Announce requests and responses
//! - Peer parsing (compact and non-compact)
//! - Peer ID generation and URL encoding helpers
//!
//! Used by the client to discover peers for a torrent.
use crate::torrent::file::TorrentFile;
use anyhow::{Ok, Result};
use rand::Rng;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};

// Represents a client communicating with a bittorent tracker
#[derive(Debug)]
pub struct Client {
    peer_id: [u8; 20],
    port: u16,
}

// Contains the parameters for a tracker announce request
#[derive(Debug)]
pub struct AnnounceRequest {
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    pub port: u16,
    pub uploaded: i64,
    pub downloaded: i64,
    pub compact: bool,
    pub left: i64,
}

/// Represents a peer recieved from the tracker.
#[derive(Debug, Clone, Deserialize)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
}

/// Contains the parsed response from a tracker.
#[derive(Debug, Deserialize)]
pub struct AnnounceResponse {
    pub interval: i64,
    pub peers: Vec<Peer>, // A list of peers that client can connect to.
}

#[derive(Debug, Deserialize)]
struct TrackerResponse {
    #[serde(default)]
    interval: i64,
    #[serde(default)]
    peers: Peers,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Peers {
    Compact(Vec<u8>),
    NonCompact(Vec<PeerDict>),
}

impl Default for Peers {
    fn default() -> Self {
        Peers::Compact(Vec::new())
    }
}

#[derive(Debug, Deserialize)]
struct PeerDict {
    ip: String,
    port: u16,
}

impl Client {
    pub fn new(port: u16) -> Result<Self> {
        Ok(Self {
            peer_id: generate_peer_id()?,
            port,
        })
    }
    /// Sends an announce request to the tracker to get a list of peers.
    #[tracing::instrument(skip(self, torrent), level = "debug")]
    pub async fn announce(&self, torrent: &TorrentFile) -> Result<AnnounceResponse> {
        let request = AnnounceRequest {
            info_hash: torrent.info_hash,
            peer_id: self.peer_id,
            port: self.port,
            uploaded: 0,
            downloaded: 0,
            compact: true,
            left: torrent.total_length(),
        };
        // Build url with query paramters
        let mut url = url::Url::parse(&torrent.announce)?;
        let params = [
            ("info_hash", url_encode(&request.info_hash)),
            ("peer_id", url_encode(&request.peer_id)),
            ("port", request.port.to_string()),
            ("uploaded", request.uploaded.to_string()),
            ("downloaded", request.downloaded.to_string()),
            ("left", request.left.to_string()),
            ("compact", (request.compact as i32).to_string()),
        ];
        url.query_pairs_mut().extend_pairs(&params).finish();
        tracing::debug!(?url, "Making announce request to tracker");
        let response = reqwest::get(url).await?;
        let response_bytes = response.bytes().await?;
        Self::parse_announce_response(&response_bytes)
    }

    /// Parses the response from a BitTorrent tracker announce request.
    ///
    /// This function takes a byte slice containing the bencoded tracker response,
    /// deserializes it into a `TrackerResponse` struct, and then extracts the list of peers.
    /// It handles both the compact and non-compact peer list formats as specified by the BitTorrent protocol:
    /// - **Compact format**: The "peers" field is a byte string where each peer is represented by 6 bytes
    ///   (4 bytes for the IPv4 address and 2 bytes for the port, in network byte order).
    /// - **Non-compact format**: The "peers" field is a list of dictionaries, each containing "ip" and "port".
    ///
    /// # Arguments
    /// * `bytes` - A byte slice containing the bencoded tracker response.
    ///
    /// # Returns
    /// * `Result<AnnounceResponse>` - The parsed announce response, including the interval and the list of peers.
    ///
    /// # Example
    /// ```
    /// let response_bytes = ...; // bytes from tracker
    /// let announce_response = Client::parse_announce_response(&response_bytes)?;
    /// println!("Interval: {}", announce_response.interval);
    /// for peer in announce_response.peers {
    ///     println!("Peer: {}:{}", peer.ip, peer.port);
    /// }
    /// ```
    fn parse_announce_response(bytes: &[u8]) -> Result<AnnounceResponse> {
        // 1. Deserialize the tracker response from bencoded bytes
        let tracker_response: TrackerResponse = serde_bencode::from_bytes(bytes)?;

        // 2. Parse the peers field, handling both compact and non-compact forms
        let peers = match tracker_response.peers {
            // 2a. Compact: each peer is 6 bytes (4 for IP, 2 for port)
            Peers::Compact(bytes) => bytes
                .chunks_exact(6)
                .map(|chunk| {
                    let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
                    let port = u16::from_be_bytes([chunk[4], chunk[5]]);
                    Peer {
                        ip: IpAddr::V4(ip),
                        port,
                    }
                })
                .collect(),
            // 2b. Non-compact: each peer is a dictionary with "ip" and "port"
            Peers::NonCompact(dicts) => dicts
                .into_iter()
                .filter_map(|dict| {
                    dict.ip.parse::<IpAddr>().ok().map(|ip| Peer {
                        ip,
                        port: dict.port,
                    })
                })
                .collect(),
        };

        // 3. Return the parsed announce response
        Ok(AnnounceResponse {
            interval: tracker_response.interval,
            peers,
        })
    }
}
/// Generates a unique peer ID for this client.
///
/// This function creates a 20-byte peer ID that follows the BitTorrent protocol specification.
/// The peer ID consists of:
/// - A 9-byte prefix identifying the client ("-RT0001-") RT = Rust torrent
/// - 11 random bytes to ensure uniqueness
///
/// # Returns
/// * `Result<[u8; 20]>` - A 20-byte peer ID, or an error if generation fails
///
/// # Example
/// ```rust
/// let peer_id = generate_peer_id()?;
/// assert_eq!(peer_id.len(), 20);
/// assert_eq!(&peer_id[..9], b"-GT0001-");
///
fn generate_peer_id() -> Result<[u8; 20]> {
    let mut peer_id = [0u8; 20];
    let prefix = b"-RT0001-";
    peer_id[..prefix.len()].copy_from_slice(prefix);
    let mut rng = rand::rng();
    rng.fill(&mut peer_id[prefix.len()..]);
    Ok(peer_id)
}

/// URL-encodes a byte slice according to RFC 3986.
///
/// This function performs percent-encoding of bytes that are not in the unreserved
/// character set. Unreserved characters (a-z, A-Z, 0-9, -, ., _, ~) are left as-is,
/// while all other characters are encoded as %XX where XX is the hexadecimal
/// representation of the byte value.
///
/// # Arguments
/// * `bytes` - The byte slice to URL-encode
///
/// # Returns
/// * `String` - The URL-encoded string
///
/// # Example
/// ```rust
/// let bytes = b"Hello World!";
/// let encoded = url_encode(bytes);
/// assert_eq!(encoded, "Hello%20World%21");
/// ```
fn url_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 3);
    for &byte in bytes {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}
