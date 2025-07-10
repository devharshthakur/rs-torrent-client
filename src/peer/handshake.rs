use crate::torrent::TorrentError;
use anyhow::Result;
use std::io::Read;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::instrument;

#[derive(Debug, PartialEq, Eq)]
pub struct Handshake {
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    pub protocol_len: u8,
    pub protocol: [u8; 19],
    pub reserved: [u8; 8],
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            info_hash,
            peer_id,
            protocol_len: 19,
            protocol: *b"BitTorrent protocol",
            reserved: [0u8; 8],
        }
    }

    /// Serializes the handshake into a 68-byte array as per the BitTorrent protocol.
    #[instrument(level = "trace")]
    pub fn serialize(&self) -> [u8; 68] {
        let mut buf = [0u8; 68];
        buf[0] = self.protocol_len;
        buf[1..20].copy_from_slice(&self.protocol);
        buf[20..28].copy_from_slice(&[0u8; 8]); // reserved bytes
        buf[28..48].copy_from_slice(&self.info_hash);
        buf[48..68].copy_from_slice(&self.peer_id);
        buf
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        // Read and validate protocol length
        let mut length_buf = [0u8; 1];
        reader.read_exact(&mut length_buf)?;
        let protocol_len = length_buf[0];
        if protocol_len != 19 {
            return Err(anyhow::anyhow!("Invalid protocol length: {}", protocol_len));
        }
        //  Read the remaining 67 bytes
        let mut buf = [0u8; 67];
        reader.read_exact(&mut buf)?;

        // Extract and validate protocol string
        let mut protocol = [0u8; 19];
        protocol.copy_from_slice(&buf[0..19]);
        if &protocol != b"BitTorrent protocol" {
            return Err(anyhow::anyhow!("Invalid protocol {:?}", protocol));
        }

        // Extract reserved, info_hash and peer_id
        let mut reserved = [0u8; 8];
        reserved.copy_from_slice(&buf[19..27]);

        let mut info_hash = [0u8; 20];
        info_hash.copy_from_slice(&buf[27..47]);

        let mut peer_id = [0u8; 20];
        peer_id.copy_from_slice(&buf[47..67]);

        Ok(Self {
            info_hash,
            peer_id,
            protocol_len,
            protocol: *b"BitTorrent protocol",
            reserved,
        })
    }
}
