# Installation

## Basic Setup

Add `blockchain-compression` to your `Cargo.toml`:

```toml
[dependencies]
blockchain-compression = { version = "0.1", features = ["zstd"] }
```

## Feature Flags

The library uses feature flags to control which compression backends are compiled:

| Feature | Backend | Default | Description |
|---------|---------|---------|-------------|
| `deflate` | DEFLATE (flate2) | Yes | General-purpose compression, widely compatible |
| `zstd` | Zstandard | No | Recommended for blockchain data -- best ratios |
| `lz4` | LZ4 | No | Fastest compression/decompression speed |

### Recommended: Zstandard

For blockchain data, use the `zstd` feature. It provides the best compression ratios with custom dictionary support:

```toml
blockchain-compression = { version = "0.1", features = ["zstd"] }
```

### Multiple Backends

You can enable multiple backends simultaneously:

```toml
blockchain-compression = { version = "0.1", features = ["zstd", "deflate", "lz4"] }
```

### Default Feature

The `deflate` feature is enabled by default. To use only `zstd`:

```toml
blockchain-compression = { version = "0.1", default-features = false, features = ["zstd"] }
```

## System Requirements

- **Rust**: 1.70+
- **OS**: Linux, macOS, Windows
- **zstd feature**: Requires a C compiler for `zstd-sys` (automatically handled by `cc` crate)

## Building from Source

```bash
git clone https://github.com/cryptuon/blockchain-compression.git
cd blockchain-compression

# Build with zstd
cargo build --features zstd

# Run tests
cargo test --features zstd

# Build in release mode
cargo build --release --features zstd
```
