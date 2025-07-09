# 🚧 Rust Torrent Client

A **BitTorrent client** implementation in Rust, aiming to support downloading files using the BitTorrent protocol. This project is a work-in-progress and currently in its **very early phase**—core components are being built from the ground up! 🦀

📝 **This project is a Rust rewrite of the original [Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client).**

## 📈 Project Status

The current implementation status and roadmap are tracked in [docs/CHECKPOINT.md](md/checkpoint.md). Please refer to it for up-to-date progress and details on what is implemented.

For the project's long-term goals and planned features, see [goals.md](md/goals.md).

## 📁 Project Structure

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
        └── handshake.rs
```

## 🙏 Acknowledgments

- Inspired by the original [Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client) ([piyushgupta53/go-torrent-client](https://github.com/piyushgupta53/go-torrent-client))
- [BitTorrent Protocol Specification](https://wiki.theory.org/BitTorrentSpecification)
- [Bencode Specification](https://wiki.theory.org/BitTorrentSpecification#Bencoding)

> ⚠️ **Note:** This project is not yet ready for general use. Contributions and feedback are welcome as the project evolves!
