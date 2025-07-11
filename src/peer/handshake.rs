//! This module implements the BitTorrent peer handshake protocol.
//!
//! The handshake is the first message exchanged between two peers. It verifies
//! that both peers are participating in the same torrent (via info_hash) and
//! establishes basic protocol compatibility.
//!
use crate::torrent::TorrentError;
use anyhow::Result;
use std::io::Read;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::stream;
use tokio::time::timeout;
use tracing::instrument;

/** Represents a BitTorrent handshake message as defined in the BitTorrent protocol.

A handshake is the first message exchanged between peers and contains:
- Protocol identifier ("BitTorrent protocol")
- Reserved bytes for protocol extensions
- Info hash identifying the torrent
- Peer ID identifying the client */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake {
    pub protocol_len: u8,
    pub protocol: [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    /** Creates a new handshake with the specified info hash and peer ID.

    Arguments:
    - info_hash - The SHA-1 hash of the torrent's info dictionary
    - peer_id - A unique identifier for this client

    Returns:
    A new Handshake instance with default protocol settings */
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            protocol_len: 19,
            protocol: *b"BitTorrent protocol",
            reserved: [0u8; 8],
            info_hash,
            peer_id,
        }
    }

    /** Serializes the handshake into a 68-byte array as per the BitTorrent protocol.

    The serialized format is:
    - 1 byte: protocol string length (19)
    - 19 bytes: protocol string ("BitTorrent protocol")
    - 8 bytes: reserved bytes (all zeros)
    - 20 bytes: info hash
    - 20 bytes: peer ID

    Returns:
    A 68-byte array containing the serialized handshake */
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

    /**
    Asynchronously deserializes a handshake from a `tokio::net::TcpStream`.

    This function reads exactly 68 bytes from the provided TCP stream,
    parses them into a `Handshake` struct, and performs basic validation
    on the protocol length and string.

    # Arguments
    - `stream` - A mutable reference to a `tokio::net::TcpStream` from which
      the handshake bytes will be read.

    # Returns
    - `Result<Self>` - A `Result` which is:
      - `Ok(Handshake)`: If the handshake was successfully read and parsed.
      - `Err(anyhow::Error)`: If an I/O error occurs (e.g., connection closed,
        timeout), or if the handshake format is invalid (e.g., incorrect protocol length
        or protocol string).

    # Errors
    This function can return an `anyhow::Error` if:
    - An underlying I/O error occurs during reading (e.g., `io::ErrorKind::UnexpectedEof`).
    - `protocol_len` is not 19.
    - The protocol string is not "BitTorrent protocol".
    */

    #[instrument(level = "trace", skip(stream))]
    pub async fn read(stream: &mut TcpStream) -> Result<Self> {
        // Step 1: Read the first byte (protocol_len) asynchronously.
        let mut length_buf = [0u8; 1];
        stream.read_exact(&mut length_buf).await?;
        let protocol_len = length_buf[0];

        // Step 2: Validate the protocol length.
        if protocol_len != 19 {
            return Err(anyhow::anyhow!("Invalid protocol length: {}", protocol_len));
        }

        // Step 3: Read the remaining 67 bytes of the handshake asynchronously.
        let mut buf = [0u8; 67];
        stream.read_exact(&mut buf).await?;

        // Step 4: Extract and validate the protocol string from the buffer.
        let mut protocol = [0u8; 19];
        protocol.copy_from_slice(&buf[0..19]);
        if &protocol != b"BitTorrent protocol" {
            return Err(anyhow::anyhow!("Invalid protocol: {:?}", protocol));
        }

        // Step 5: Extract reserved bytes, info_hash, and peer_id from the buffer.
        let mut reserved = [0u8; 8];
        reserved.copy_from_slice(&buf[19..27]);

        let mut info_hash = [0u8; 20];
        info_hash.copy_from_slice(&buf[27..47]);

        let mut peer_id = [0u8; 20];
        peer_id.copy_from_slice(&buf[47..67]);

        // Step 6: Construct and return the Handshake struct.
        Ok(Self {
            protocol_len,
            protocol,
            reserved,
            info_hash,
            peer_id,
        })
    }

    /** Validates that this handshake's info hash matches the expected value.

    Arguments:
    - expected_info_hash - The expected SHA-1 hash of the torrent's info dictionary

    Returns:
    A Result that is Ok if the info hashes match, or an error if they don't

    Errors:
    Returns an error if the info hash in this handshake doesn't match the expected value */
    pub fn validate(&self, expected_info_hash: [u8; 20]) -> Result<()> {
        if self.info_hash != expected_info_hash {
            return Err(anyhow::anyhow!(
                "Info hash mismatch: got {:?}, expected {:?}",
                self.info_hash,
                expected_info_hash
            ));
        }
        Ok(())
    }

    /// Performs a complete handshake with a peer over a TCP connection.
    ///
    /// This is an `async` function because it involves network I/O.
    ///
    /// # How it works
    /// 1. Establishes a TCP connection to the peer's address (IP:port).
    /// 2. Creates and serializes our handshake message.
    /// 3. Sends the serialized handshake over the connection.
    /// 4. Reads and deserializes the peer's handshake response.
    /// 5. Validates the received handshake against the expected info_hash.
    /// 6. Returns the peer's handshake if successful.
    ///
    /// # Arguments
    /// * `peer_addr` - The peer's "IP:port" string (e.g., "127.0.0.1:6881").
    /// * `info_hash` - The torrent's info hash.
    /// * `peer_id` - Our client's peer ID.
    ///
    /// # Returns
    /// * `Result<Handshake>` - The peer's handshake if successful, or an error.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The connection fails
    /// - The peer's handshake is invalid
    /// - The protocol length is not 19
    /// - The protocol string is not "BitTorrent protocol"
    pub async fn do_handshake(
        peer_addr: String,
        info_hash: [u8; 20],
        peer_id: [u8; 20],
    ) -> Result<Handshake> {
        // 1. Establish TCP connection
        let mut stream = TcpStream::connect(peer_addr).await?;

        // 2. Create and serialize our handshake
        let our_handshake = Handshake::new(info_hash, peer_id);
        let serialized = our_handshake.serialize();

        // 3. Send the handshake
        stream.write_all(&serialized).await?;

        // 4. Read and deserialize the peer's handshake response

        // Read protocol length (should be 19)
        let mut protocol_len_buf = [0u8; 1];
        stream.read_exact(&mut protocol_len_buf).await?;
        let protocol_len = protocol_len_buf[0];
        if protocol_len != 19 {
            return Err(anyhow::anyhow!("Invalid protocol length: {}", protocol_len));
        }

        // Read the next 67 bytes (protocol string, reserved, info_hash, peer_id)
        let mut buf = [0u8; 67];
        stream.read_exact(&mut buf).await?;

        // Extract protocol string
        let mut protocol = [0u8; 19];
        protocol.copy_from_slice(&buf[0..19]);
        if &protocol != b"BitTorrent protocol" {
            return Err(anyhow::anyhow!("Invalid protocol {:?}", protocol));
        }

        // Extract reserved bytes
        let mut reserved = [0u8; 8];
        reserved.copy_from_slice(&buf[19..27]);

        // Extract info_hash
        let mut peer_info_hash = [0u8; 20];
        peer_info_hash.copy_from_slice(&buf[27..47]);

        // Extract peer_id
        let mut peer_peer_id = [0u8; 20];
        peer_peer_id.copy_from_slice(&buf[47..67]);

        // 5. Validate the received handshake's info_hash
        if peer_info_hash != info_hash {
            return Err(anyhow::anyhow!(
                "Info hash mismatch: got {:?}, expected {:?}",
                peer_info_hash,
                info_hash
            ));
        }

        // 6. Return the peer's handshake
        Ok(Handshake {
            info_hash: peer_info_hash,
            peer_id: peer_peer_id,
            protocol_len,
            protocol: *b"BitTorrent protocol",
            reserved,
        })
    }
}
