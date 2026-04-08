# Blockchain Compression Library - API Documentation

## Overview

The blockchain-compression library provides high-performance, blockchain-optimized compression with up to 60:1 compression ratios and 100% data integrity guarantee.

## Core Traits

### `CompressionStrategy`

The main trait that all compression algorithms implement.

```rust
pub trait CompressionStrategy {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Compresses the input data
    fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error>;

    /// Decompresses the input data
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error>;

    /// Returns metadata about this compression algorithm
    fn metadata(&self) -> CompressionMetadata;

    /// Returns the current compression statistics
    fn stats(&self) -> CompressionStats;

    /// Resets internal state (useful for stateful compressors)
    fn reset(&mut self);
}
```

## Core Types

### `CompressionMetadata`

Information about a compression algorithm:

```rust
pub struct CompressionMetadata {
    /// Human-readable name of the algorithm
    pub name: String,
    /// Version identifier
    pub version: String,
    /// Description of the algorithm
    pub description: String,
    /// Whether this algorithm is deterministic
    pub deterministic: bool,
    /// Approximate memory usage in bytes
    pub memory_usage: usize,
    /// Supported data types/domains
    pub domains: Vec<String>,
}
```

### `CompressionStats`

Performance statistics for compression operations:

```rust
pub struct CompressionStats {
    /// Total number of compression operations performed
    pub compressions: u64,
    /// Total number of decompression operations performed
    pub decompressions: u64,
    /// Total original bytes processed
    pub total_input_bytes: u64,
    /// Total compressed bytes produced
    pub total_output_bytes: u64,
    /// Average compression ratio
    pub average_ratio: f64,
    /// Best compression ratio achieved
    pub best_ratio: f64,
    /// Total time spent compressing (nanoseconds)
    pub compression_time_ns: u64,
    /// Total time spent decompressing (nanoseconds)
    pub decompression_time_ns: u64,
    /// Number of compression errors encountered
    pub errors: u64,
}
```

### `CompressionError`

Error types that can occur during compression operations:

```rust
pub enum CompressionError {
    /// Invalid compression format
    InvalidFormat,
    /// Unsupported algorithm version
    UnsupportedVersion { version: String },
    /// Configuration error
    Configuration { message: String },
    /// Pattern processing error
    Pattern { message: String },
    /// Pipeline stage error
    Pipeline { stage: usize, message: String },
    /// Training/learning error
    Training { message: String },
    /// IO error
    Io(std::io::Error),
    /// Serialization error
    Serialization(String),
    /// Internal error
    Internal { message: String },
}
```

## Solana-Specific API

### `SolanaCompressor`

The main compression engine optimized for Solana blockchain data:

```rust
pub struct SolanaCompressor {
    // Internal fields...
}

impl SolanaCompressor {
    /// Create a new compressor with the specified preset
    pub fn new(preset: SolanaPreset) -> Self;

    /// Add a program ID pattern to the dictionary
    pub fn add_program_pattern(&mut self, program_id: &str) -> Result<(), CompressionError>;

    /// Add a signature pattern to the dictionary
    pub fn add_signature_pattern(&mut self, signature: &[u8; 64]) -> Result<(), CompressionError>;

    /// Add an amount pattern to the dictionary
    pub fn add_amount_pattern(&mut self, amount: u64) -> Result<(), CompressionError>;
}

impl CompressionStrategy for SolanaCompressor {
    type Error = CompressionError;
    // Implementation details...
}
```

### `SolanaPreset`

Pre-configured compression settings for different use cases:

```rust
pub enum SolanaPreset {
    /// Optimized for transaction data with high signature/account repetition
    Transactions,
    /// Optimized for account state data
    Accounts,
    /// Optimized for program instruction data
    Instructions,
    /// Balanced configuration for mixed workloads
    Mixed,
    /// Maximum compression (slower but best ratio)
    MaxCompression,
    /// Fast compression (lower ratio but faster)
    FastCompression,
}
```

### Preset Characteristics

| Preset | Compression Level | Speed | Typical Ratio | Best For |
|--------|------------------|-------|---------------|----------|
| `FastCompression` | 3 | ⚡⚡⚡ | 5-15:1 | Real-time processing |
| `Transactions` | 3 | ⚡⚡ | 10-30:1 | Transaction data |
| `Accounts` | 6 | ⚡ | 15-40:1 | Account states |
| `Instructions` | 6 | ⚡ | 12-25:1 | Program instructions |
| `Mixed` | 6 | ⚡ | 12-35:1 | General purpose |
| `MaxCompression` | 22 | ⚡ | 20-60:1 | Archival storage |

## SolanaVault Integration API

### `BlockchainCompressionAdapter`

Thread-safe wrapper that integrates blockchain-compression with SolanaVault:

```rust
pub struct BlockchainCompressionAdapter {
    // Internal fields...
}

impl BlockchainCompressionAdapter {
    /// Create a new adapter with the specified preset
    pub fn new(preset: SolanaPreset) -> Self;

    /// Create adapter optimized for transaction data
    pub fn for_transactions() -> Self;

    /// Create adapter optimized for account data
    pub fn for_accounts() -> Self;

    /// Create adapter optimized for mixed blockchain data
    pub fn for_mixed_data() -> Self;

    /// Create adapter optimized for maximum compression (archival)
    pub fn for_archival() -> Self;

    /// Get compression statistics
    pub fn get_stats(&self) -> Result<CompressionStats, CompressionError>;

    /// Get metadata about the compression strategy
    pub fn get_metadata(&self) -> Result<CompressionMetadata, CompressionError>;

    /// Reset internal compression state
    pub fn reset(&self) -> Result<(), CompressionError>;

    /// Get the preset being used
    pub fn preset(&self) -> &SolanaPreset;
}

impl CompressionStrategy for BlockchainCompressionAdapter {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError>;
    fn version(&self) -> CompressionVersion;
}
```

### Convenience Functions

```rust
/// Helper function to create the optimal compressor for block data
pub fn create_block_compressor() -> BlockchainCompressionAdapter;

/// Helper function to create the optimal compressor for account data
pub fn create_account_compressor() -> BlockchainCompressionAdapter;

/// Helper function to create a general-purpose compressor
pub fn create_general_compressor() -> BlockchainCompressionAdapter;

/// Helper function to create the highest compression ratio compressor for archival
pub fn create_archival_compressor() -> BlockchainCompressionAdapter;
```

## Usage Examples

### Basic Compression

```rust
use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

let data = b"Hello, blockchain compression!";
let compressed = compressor.compress(data)?;
let decompressed = compressor.decompress(&compressed)?;

assert_eq!(data.as_slice(), decompressed.as_slice());
```

### With Statistics

```rust
let mut compressor = SolanaCompressor::new(SolanaPreset::MaxCompression);

// Compress some data
let compressed = compressor.compress(&my_data)?;

// Get performance statistics
let stats = compressor.stats();
println!("Average ratio: {:.2}:1", stats.average_ratio);
println!("Best ratio: {:.2}:1", stats.best_ratio);
println!("Total compressions: {}", stats.compressions);
```

### SolanaVault Integration

```rust
use vault_core::compression::{BlockchainCompressionAdapter, CompressionStrategy};

let adapter = BlockchainCompressionAdapter::for_transactions();

let compressed = adapter.compress(&blockchain_data)?;
let decompressed = adapter.decompress(&compressed)?;

// Get metadata
let metadata = adapter.get_metadata()?;
println!("Using: {} v{}", metadata.name, metadata.version);
```

## Error Handling

All compression operations return `Result<T, CompressionError>`. Handle errors appropriately:

```rust
match compressor.compress(&data) {
    Ok(compressed) => {
        // Success - use compressed data
        println!("Compressed {} bytes to {}", data.len(), compressed.len());
    }
    Err(CompressionError::InvalidFormat) => {
        // Handle invalid format
        eprintln!("Invalid data format for compression");
    }
    Err(CompressionError::Internal { message }) => {
        // Handle internal errors
        eprintln!("Compression failed: {}", message);
    }
    Err(e) => {
        // Handle other errors
        eprintln!("Compression error: {}", e);
    }
}
```

## Performance Considerations

### Memory Usage

- **SolanaCompressor**: ~1-10 MB depending on preset and dictionary size
- **BlockchainCompressionAdapter**: Additional ~100 KB for thread-safety overhead

### Thread Safety

- **SolanaCompressor**: Not thread-safe (use one per thread)
- **BlockchainCompressionAdapter**: Thread-safe (can be shared across threads)

### Optimal Usage Patterns

1. **Reuse compressors** when possible to amortize initialization costs
2. **Choose appropriate presets** for your data type
3. **Use adapters** for multi-threaded environments
4. **Monitor statistics** to optimize for your specific workload

## Feature Flags

Enable specific features in your `Cargo.toml`:

```toml
[dependencies]
blockchain-compression = { version = "0.1", features = ["zstd"] }
```

Available features:
- `zstd` - Zstandard compression backend (recommended)
- `deflate` - Deflate compression backend
- `lz4` - LZ4 compression backend

## Version Compatibility

| Library Version | SolanaVault Version | Compression Version |
|----------------|-------------------|-------------------|
| 0.1.x | 0.1.x | V3 |

## Migration Guide

### From Previous Versions

If migrating from older compression algorithms:

1. Replace old compressor with `SolanaCompressor`
2. Update preset selection based on your use case
3. Add error handling for new `CompressionError` types
4. Update tests to verify data integrity

### Example Migration

```rust
// Old code
let mut old_compressor = V2Compression::new();
let compressed = old_compressor.compress(&data);

// New code
let mut new_compressor = SolanaCompressor::new(SolanaPreset::Mixed);
let compressed = new_compressor.compress(&data)?;
```