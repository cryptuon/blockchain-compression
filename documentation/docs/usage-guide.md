# Usage Guide

## Creating a Compressor

Every compression operation starts by creating a `SolanaCompressor` with a preset:

```rust
use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};

let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);
```

The preset determines the Zstandard compression level and dictionary configuration. See [Presets](presets.md) for details on each option.

## Compressing Data

Import the `CompressionStrategy` trait and call `compress()`:

```rust
use blockchain_compression::core::traits::CompressionStrategy;

let compressed = compressor.compress(&data)?;
```

The method returns `Result<Vec<u8>, CompressionError>`. The compressed output includes all metadata needed for decompression.

## Decompressing Data

```rust
let decompressed = compressor.decompress(&compressed)?;
assert_eq!(original_data, decompressed.as_slice());
```

!!! warning "Use the same preset"
    Decompression must use the same preset (and therefore the same dictionary) that was used for compression. Mismatched presets will produce a decompression error.

## Compression Statistics

The compressor tracks cumulative statistics:

```rust
let stats = compressor.stats();
println!("Operations:   {}", stats.compressions);
println!("Input bytes:  {}", stats.total_input_bytes);
println!("Output bytes: {}", stats.total_output_bytes);
println!("Best ratio:   {:.2}:1", stats.best_ratio);
```

Reset statistics with:

```rust
compressor.reset();
```

## Comparing Presets

To find the best preset for your data, compress a representative sample with each:

```rust
use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

let presets = vec![
    ("Fast", SolanaPreset::FastCompression),
    ("Transactions", SolanaPreset::Transactions),
    ("Accounts", SolanaPreset::Accounts),
    ("Mixed", SolanaPreset::Mixed),
    ("Max", SolanaPreset::MaxCompression),
];

for (name, preset) in presets {
    let mut compressor = SolanaCompressor::new(preset);
    let compressed = compressor.compress(&sample_data)?;
    let ratio = sample_data.len() as f64 / compressed.len() as f64;
    println!("{}: {:.2}:1", name, ratio);
}
```

## Error Handling

All operations return `Result<_, CompressionError>`. The error variants are:

```rust
use blockchain_compression::core::traits::CompressionError;

match compressor.compress(&data) {
    Ok(compressed) => { /* use compressed data */ }
    Err(CompressionError::Internal { message }) => {
        eprintln!("Compression failed: {}", message);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Algorithm Metadata

Inspect the compressor's configuration:

```rust
let metadata = compressor.metadata();
println!("Name:    {}", metadata.name);       // "Solana Zstd Compressor"
println!("Version: {}", metadata.version);    // "2.0.0"
println!("Domains: {:?}", metadata.domains);  // ["Solana", "Blockchain"]
```

## Thread Safety

`SolanaCompressor` can be shared across threads using standard Rust concurrency patterns. Each compressor instance maintains its own statistics, so for concurrent workloads, create one compressor per thread:

```rust
use std::thread;

let handles: Vec<_> = data_chunks.into_iter().map(|chunk| {
    thread::spawn(move || {
        let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);
        compressor.compress(&chunk)
    })
}).collect();
```
