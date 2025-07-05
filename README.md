# 🚧 Rust Torrent Client

A **BitTorrent client** implementation in Rust, aiming to support downloading files using the BitTorrent protocol. This project is a work-in-progress and currently in its **very early phase**—core components are being built from the ground up! 🦀


📝 **This project is a Rust rewrite of the original [Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client).**


## ✨ Project Goals

- **Bencode Encoding/Decoding**
  - Full support for all Bencode types:
    - Strings
    - Integers
    - Lists
    - Dictionaries
  - Robust error handling and validation

- **Torrent File Processing**
  - Parse `.torrent` files:
    - Single-file
    - Multi-file
  - Info hash calculation
  - Piece hash extraction

- **Peer Discovery & Communication**
  - HTTP tracker support
  - Peer handshake & message protocol

- **Download Functionality**
  - Piece/block management
  - Concurrent downloads
  - File assembly & storage management

## 📁 (Tentative) Project Structure

```
.
├── src/                # Main source code
├── bencode/            # Bencode encoding/decoding
├── torrent/            # Torrent file processing
├── tracker/            # Tracker protocol implementation
├── peer/               # Peer communication
├── download/           # Download management
└── docs/               # Documentation & checkpoints
```

## 🚀 Status

This project is **actively under development** and is in its initial stages. Check out [docs/CHECKPOINT.md](docs/CHECKPOINT.md) for the latest progress and implementation details.

## 🛣️ Planned Features

- Magnet link support
- Metadata exchange protocol
- DHT (Distributed Hash Table) support

## 🙏 Acknowledgments

- Inspired by the original [Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client) ([piyushgupta53/go-torrent-client](https://github.com/piyushgupta53/go-torrent-client))
- [BitTorrent Protocol Specification](https://wiki.theory.org/BitTorrentSpecification)
- [Bencode Specification](https://wiki.theory.org/BitTorrentSpecification#Bencoding)

---

> ⚠️ **Note:** This project is not yet ready for general use. Contributions and feedback are welcome as the project evolves!
