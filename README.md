# blockchain-compression

**Lossless compression for blockchain data -- up to 60:1 ratios with Zstandard and custom dictionaries, built in Rust.**

[![Crates.io](https://img.shields.io/crates/v/blockchain-compression)](https://crates.io/crates/blockchain-compression)
[![docs.rs](https://img.shields.io/docsrs/blockchain-compression)](https://docs.rs/blockchain-compression)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/cryptuon/blockchain-compression/actions/workflows/ci.yml/badge.svg)](https://github.com/cryptuon/blockchain-compression/actions)

---

## Why blockchain-compression?

Blockchain data is full of repeated structures -- program IDs, public keys, signatures, common amounts -- that general-purpose compressors don't exploit. This library uses Zstandard with custom dictionaries trained on blockchain-specific byte patterns to achieve dramatically better compression ratios while guaranteeing perfect data integrity.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
blockchain-compression = { version = "0.1", features = ["zstd"] }
```

Compress and decompress:

```rust
use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

let compressed = compressor.compress(data)?;
let decompressed = compressor.decompress(&compressed)?;
assert_eq!(data, decompressed.as_slice());
```

## Features

- **Blockchain-optimized dictionaries** -- custom Zstandard dictionaries for Solana program IDs, signatures, and common patterns
- **6 compression presets** -- from real-time (`FastCompression`) to archival (`MaxCompression`)
- **Multiple backends** -- Zstandard, DEFLATE, and LZ4 support via feature flags
- **100% lossless** -- rigorous roundtrip validation on every operation
- **Thread-safe** -- safe for concurrent usage across threads
- **Trait-based architecture** -- composable `CompressionStrategy` trait for custom implementations

## Presets

| Preset | Zstd Level | Compression Ratio | Best For |
|--------|-----------|-------------------|----------|
| `FastCompression` | 3 | 5--15:1 | Real-time processing |
| `Transactions` | 3 | 10--30:1 | Transaction data |
| `Instructions` | 6 | 10--25:1 | Program instructions |
| `Accounts` | 6 | 15--40:1 | Account state snapshots |
| `Mixed` | 6 | 12--35:1 | General-purpose |
| `MaxCompression` | 19 | 20--60:1 | Archival storage |

## Installation

Enable the compression backend you need:

```toml
# Zstandard (recommended)
blockchain-compression = { version = "0.1", features = ["zstd"] }

# DEFLATE
blockchain-compression = { version = "0.1", features = ["deflate"] }

# LZ4
blockchain-compression = { version = "0.1", features = ["lz4"] }
```

## Building and Testing

```bash
# Build
cargo build --features zstd

# Run tests
cargo test --features zstd

# Run examples
cargo run --example basic_usage --features zstd
cargo run --example performance_benchmark --features zstd

# Benchmarks
cargo bench --features zstd
```

## Documentation

- [API Reference](https://docs.rs/blockchain-compression) -- full Rustdoc on docs.rs
- [User Guide](documentation/) -- mkdocs-based guides and tutorials
- [API Details](docs/API.md) -- in-repo API documentation
- [Examples](examples/) -- runnable code examples

## License

MIT
