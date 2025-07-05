use crate::bencode::BencodeValue;
use crate::torrent::file::TorrentFile;
use crate::torrent::TorrentError;
use anyhow::Result;
use rand::Rng;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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
}

/// Represents a peer recieved from the tracker.
#[derive(Debug, Clone, Deserialize)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
}

/// Contains the parsed response from a tracker.
#[derive(Debug, Deserialize)]
pub struct AnnouceResponse {
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
