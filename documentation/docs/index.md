# blockchain-compression

**Lossless compression for blockchain data -- up to 60:1 ratios with Zstandard and custom dictionaries, built in Rust.**

---

## Overview

`blockchain-compression` is a Rust library that compresses blockchain data far more efficiently than general-purpose compressors. It achieves this by using Zstandard (zstd) with custom dictionaries trained on common blockchain byte patterns -- program IDs, public keys, signatures, transaction amounts, and instruction data.

## Key Features

- **Up to 60:1 compression ratios** on Solana blockchain data
- **100% lossless** -- perfect roundtrip fidelity, always
- **6 compression presets** tuned for different data types and speed/ratio trade-offs
- **Custom Zstandard dictionaries** built from common Solana program IDs, addresses, and patterns
- **Multiple backends** -- Zstandard (recommended), DEFLATE, LZ4
- **Thread-safe** -- use across threads without synchronization
- **Trait-based architecture** -- implement `CompressionStrategy` for custom compressors

## At a Glance

```rust
use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

let compressed = compressor.compress(data)?;
let decompressed = compressor.decompress(&compressed)?;
assert_eq!(data, decompressed.as_slice());
```

## Performance

| Preset | Zstd Level | Ratio | Best For |
|--------|-----------|-------|----------|
| `FastCompression` | 3 | 5--15:1 | Real-time processing |
| `Transactions` | 3 | 10--30:1 | Transaction data |
| `Instructions` | 6 | 10--25:1 | Program instructions |
| `Accounts` | 6 | 15--40:1 | Account state snapshots |
| `Mixed` | 6 | 12--35:1 | General-purpose |
| `MaxCompression` | 19 | 20--60:1 | Archival storage |

## Next Steps

- [Getting Started](getting-started.md) -- install and run your first compression
- [Presets](presets.md) -- choose the right preset for your data
- [API Reference](api-reference.md) -- full trait and type documentation
