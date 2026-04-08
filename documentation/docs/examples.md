# Examples

The `examples/` directory contains three runnable examples. Run them with:

```bash
cargo run --example <name> --features zstd
```

## basic_usage

Demonstrates the core compress/decompress workflow and preset comparison.

```bash
cargo run --example basic_usage --features zstd
```

**What it covers:**

- Creating a `SolanaCompressor` with a preset
- Compressing and decompressing data
- Verifying data integrity
- Comparing compression ratios across presets
- Working with realistic Solana transaction patterns

```rust
use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Simulate Solana transaction data
    let mut data = Vec::new();
    for _ in 0..20 {
        data.extend_from_slice(
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes(),
        );
        data.extend_from_slice(
            "11111111111111111111111111111112".as_bytes(),
        );
    }

    let compressed = compressor.compress(&data)?;
    let decompressed = compressor.decompress(&compressed)?;
    assert_eq!(data, decompressed);

    println!(
        "{} bytes -> {} bytes ({:.1}:1)",
        data.len(),
        compressed.len(),
        data.len() as f64 / compressed.len() as f64
    );

    Ok(())
}
```

## performance_benchmark

Measures compression throughput, decompression speed, and how ratios scale with input size.

```bash
cargo run --example performance_benchmark --features zstd
```

**What it covers:**

- Throughput benchmarking (MB/s for compress and decompress)
- Size scaling -- how compression ratio changes with input size
- Real-world blockchain data patterns

## solanavault_integration

Shows how to use `blockchain-compression` through the `BlockchainCompressionAdapter` pattern for integration with external vault systems.

```bash
cargo run --example solanavault_integration --features zstd
```

!!! note
    This example demonstrates the adapter pattern but requires the `vault-core` crate for full compilation. It serves as a reference for integration architecture.

## Running All Examples

```bash
cargo run --example basic_usage --features zstd
cargo run --example performance_benchmark --features zstd
```

## Running Benchmarks

For formal benchmarks using Criterion:

```bash
cargo bench --features zstd
```
