# Getting Started

This guide walks you through installing `blockchain-compression` and running your first compression.

## Prerequisites

- Rust 1.70 or later
- Cargo (included with Rust)

## Add the Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
blockchain-compression = { version = "0.1", features = ["zstd"] }
```

The `zstd` feature enables Zstandard compression, which is the recommended backend for blockchain data.

## Your First Compression

```rust
use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a compressor with the Transactions preset
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Some sample blockchain data
    let data = b"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".repeat(20);

    // Compress
    let compressed = compressor.compress(&data)?;
    println!(
        "Compressed {} bytes -> {} bytes ({:.1}:1)",
        data.len(),
        compressed.len(),
        data.len() as f64 / compressed.len() as f64
    );

    // Decompress and verify
    let decompressed = compressor.decompress(&compressed)?;
    assert_eq!(data, decompressed.as_slice());
    println!("Data integrity verified.");

    Ok(())
}
```

## Choosing a Preset

The library ships with six presets. Start with the one that matches your data:

| If your data is... | Use this preset |
|---------------------|-----------------|
| Transaction records | `SolanaPreset::Transactions` |
| Account state snapshots | `SolanaPreset::Accounts` |
| Program instruction data | `SolanaPreset::Instructions` |
| Mixed or unknown | `SolanaPreset::Mixed` |
| Archival (ratio over speed) | `SolanaPreset::MaxCompression` |
| Real-time (speed over ratio) | `SolanaPreset::FastCompression` |

See [Presets](presets.md) for detailed guidance.

## Checking Compression Stats

After compressing data, you can inspect statistics:

```rust
let stats = compressor.stats();
println!("Compressions: {}", stats.compressions);
println!("Total input:  {} bytes", stats.total_input_bytes);
println!("Total output: {} bytes", stats.total_output_bytes);
println!("Best ratio:   {:.2}:1", stats.best_ratio);
```

## Next Steps

- [Installation](installation.md) -- feature flags and backend options
- [Usage Guide](usage-guide.md) -- patterns for real-world usage
- [Examples](examples.md) -- runnable code samples
