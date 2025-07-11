# Context for AI agents for `rs-torrent-client` project

## Project Overview

`rs-torrent-client` is a modern, type-safe BitTorrent client written in Rust. It aims to provide a robust, extensible, and well-documented implementation of the BitTorrent protocol, focusing on correctness, safety, and maintainability. The project is a Rust rewrite of the original [Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client).

- **Status:** Early phase, core components under active development
- **Main repo:** https://github.com/devharshthakur/rs-torrent-client

## High-Level Architecture

```
src/
  ├── main.rs         # Binary entry point
  ├── lib.rs          # Library root, re-exports modules
  ├── bencode/        # Bencode encoding/decoding logic
  │     ├── mod.rs
  │     ├── encoder.rs
  │     └── decoder.rs
  ├── torrent/        # Torrent file parsing, info hash, piece hashes
  │     ├── mod.rs
  │     ├── file.rs
  │     └── info_hash.rs
  ├── tracker/        # Tracker client logic
  │     └── mod.rs
  └── peer/           # Peer handshake and communication (WIP)
        ├── mod.rs
        ├── handshake.rs
        └── message.rs
```

## Core Modules & Responsibilities

### 1. Bencode (`src/bencode/`)

- **Purpose:** Implements the Bencode encoding/decoding required by the BitTorrent protocol.
- **Key Types:**
  - `BencodeValue` enum: String, Integer, List, Dict
  - `BencodeError`, `BencodeResult`
- **Features:**
  - Full support for all Bencode types
  - Robust error handling and validation

### 2. Torrent (`src/torrent/`)

- **Purpose:** Handles parsing and validation of `.torrent` files, including metadata extraction and info hash calculation.
- **Key Types:**
  - `TorrentFile`: Represents a parsed torrent file (announce, announce-list, creation date, comment, created_by, encoding, info, info_hash, pieces_hash)
  - `InfoDict`, `FileDict`
  - `TorrentError`, `TorrentResult`
- **Features:**
  - Single-file and multi-file torrent support
  - Piece hash extraction
  - Info hash calculation (SHA-1 of bencoded info dict)

### 3. Tracker (`src/tracker/`)

- **Purpose:** Implements HTTP tracker communication for peer discovery.
- **Key Types:**
  - `Client`: Handles announce requests
  - `AnnounceRequest`, `AnnounceResponse`, `Peer`
- **Features:**
  - HTTP tracker announce request/response
  - Peer parsing (compact and non-compact)
  - Peer ID generation and URL encoding

### 4. Peer (`src/peer/`)

- **Purpose:** Implements peer-to-peer communication, including the BitTorrent handshake protocol and message exchange (WIP).
- **Key Types:**
  - `Handshake`: Handles handshake serialization, deserialization, and validation
  - `MessageId` (stub)
- **Features:**
  - Handshake protocol (connect, validate info hash, exchange peer IDs)
  - Message protocol (in progress)

## Current Implementation Status (as of July **12**, 2025)

- **Bencode Encoding/Decoding:** Complete
- **Torrent File Processing:** Complete
- **Tracker Client:** Complete
- **Peer Module:** Structure and handshake logic present, message protocol WIP

See `md/checkpoint.md` for detailed status.

## Project Goals & Planned Features

### Goals

- Modern, type-safe BitTorrent client in Rust
- Robust bencode parser/encoder
- Torrent file parsing (single-file & multi-file)
- Reliable tracker communication (HTTP/HTTPS)
- Peer discovery and handshake
- Efficient piece/block management and download
- Cross-platform compatibility
- Clean, well-documented, and testable codebase

### Planned Features

- Magnet link support
- Metadata exchange protocol (BEP 9)
- DHT (Distributed Hash Table) support
- uTP (Micro Transport Protocol) support
- Piece/block verification and error recovery
- Download queue and prioritization
- File assembly and storage management
- CLI interface for torrent management
- Optional: Web UI for monitoring
- Advanced peer management (choking/unchoking, optimistic unchoke)
- Rate limiting and bandwidth management
- IPv6 support
- Comprehensive test suite

## How to Extend or Use

- **Library:** Import via `lib.rs` for programmatic use
- **Binary:** Run via `main.rs` (currently prints a startup message)
- **Modules:** Each module is self-contained and documented for ease of extension

## References

- [BitTorrent Protocol Specification](https://wiki.theory.org/BitTorrentSpecification)
- [Bencode Specification](https://wiki.theory.org/BitTorrentSpecification#Bencoding)
- [Original Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client)

---

> This context file is intended for AI agents to quickly understand the architecture, status, and direction of the `rs-torrent-client` project.
