# 🎯 Project Goals & Planned Features

This document outlines the long-term goals and planned features for the client.

---

## 🌟 Project Goals

- **Modern, type-safe BitTorrent client in Rust**
- **Robust bencode parser/encoder** for all BitTorrent metadata
- **Torrent file parsing** (single-file & multi-file)
- **Reliable tracker communication** (HTTP/HTTPS)
- **Peer discovery and handshake**
- **Efficient piece/block management and download**
- **Cross-platform compatibility**
- **Clean, well-documented, and testable codebase**

---

## 🛣️ Planned Features

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

---

> For current implementation status, see [docs/CHECKPOINT.md](docs/checkpoint.md). 