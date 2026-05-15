# API Reference

## Core Traits

### `CompressionStrategy`

The primary trait that all compressors implement.

```rust
pub trait CompressionStrategy {
    type Error: std::error::Error + Send + Sync + 'static;

    fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error>;
    fn metadata(&self) -> CompressionMetadata;
    fn stats(&self) -> CompressionStats;
    fn reset(&mut self);
}
```

| Method | Description |
|--------|-------------|
| `compress` | Compresses input bytes. Mutates internal state (statistics). |
| `decompress` | Decompresses previously compressed data. Stateless (takes `&self`). |
| `metadata` | Returns algorithm name, version, and capabilities. |
| `stats` | Returns cumulative compression statistics. |
| `reset` | Resets internal statistics counters. |

### `PatternCompressionStrategy`

Extends `CompressionStrategy` with pattern management:

```rust
pub trait PatternCompressionStrategy: CompressionStrategy {
    type Pattern: Clone + Send + Sync;
    type Config: Clone + Send + Sync;

    fn with_config(config: Self::Config) -> Self;
    fn add_pattern(&mut self, pattern: Self::Pattern) -> Result<(), Self::Error>;
    fn remove_pattern(&mut self, pattern_id: &str) -> Result<(), Self::Error>;
    fn pattern_info(&self) -> HashMap<String, PatternInfo>;
    fn optimize_patterns(&mut self) -> Result<(), Self::Error>;
}
```

### `PipelineCompressionStrategy`

For multi-stage compression pipelines:

```rust
pub trait PipelineCompressionStrategy: CompressionStrategy {
    type Stage: CompressionStrategy;

    fn add_stage(&mut self, stage: Self::Stage) -> Result<(), Self::Error>;
    fn remove_stage(&mut self, index: usize) -> Result<Self::Stage, Self::Error>;
    fn stage_count(&self) -> usize;
    fn stage_stats(&self) -> Vec<CompressionStats>;
    fn set_stage_enabled(&mut self, index: usize, enabled: bool) -> Result<(), Self::Error>;
}
```

### `AdaptiveCompressionStrategy`

For compressors that learn from data:

```rust
pub trait AdaptiveCompressionStrategy: CompressionStrategy {
    fn train(&mut self, training_data: &[&[u8]]) -> Result<(), Self::Error>;
    fn learning_progress(&self) -> f64;
    fn save_model(&self) -> Result<Vec<u8>, Self::Error>;
    fn load_model(&mut self, model_data: &[u8]) -> Result<(), Self::Error>;
    fn learning_info(&self) -> LearningInfo;
}
```

## Core Types

### `CompressionMetadata`

```rust
pub struct CompressionMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub deterministic: bool,
    pub memory_usage: usize,
    pub domains: Vec<String>,
}
```

### `CompressionStats`

```rust
pub struct CompressionStats {
    pub compressions: u64,
    pub decompressions: u64,
    pub total_input_bytes: u64,
    pub total_output_bytes: u64,
    pub best_ratio: f64,
    pub average_ratio: f64,
    pub compression_time_ns: u64,
    pub decompression_time_ns: u64,
    pub errors: u64,
}
```

### `CompressionError`

```rust
pub enum CompressionError {
    InvalidFormat,
    UnsupportedVersion { version: String },
    Configuration { message: String },
    Pattern { message: String },
    Pipeline { stage: usize, message: String },
    Training { message: String },
    Io(std::io::Error),
    Serialization(String),
    Internal { message: String },
}
```

## Solana API

### `SolanaCompressor`

The main compressor for Solana blockchain data.

```rust
// Create with a preset
let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

// Compress/decompress (via CompressionStrategy trait)
let compressed = compressor.compress(&data)?;
let decompressed = compressor.decompress(&compressed)?;

// Solana-specific stats
let solana_stats = compressor.solana_stats();

// Reset
compressor.reset();
```

### `SolanaPreset`

```rust
pub enum SolanaPreset {
    Transactions,
    Accounts,
    Instructions,
    Mixed,
    MaxCompression,
    FastCompression,
}
```

See [Presets](presets.md) for details on each variant.

### `SolanaPatternType`

```rust
pub enum SolanaPatternType {
    PublicKey,      // 32 bytes
    Signature,      // 64 bytes
    ProgramId,      // 32 bytes
    Amount,         // 8 bytes
    Blockhash,      // 32 bytes
    InstructionData,
}
```

### `SolanaCompressionStats`

```rust
pub struct SolanaCompressionStats {
    pub signature_patterns: usize,
    pub pubkey_patterns: usize,
    pub amount_patterns: usize,
    pub total_solana_bytes_saved: u64,
}
```

## Algorithm Implementations

### `EnhancedCTW`

Context Tree Weighting compression. Prediction-based algorithm using context trees:

```rust
use blockchain_compression::algorithms::EnhancedCTW;

let mut ctw = EnhancedCTW::new();
let compressed = ctw.compress(&data)?;
let decompressed = ctw.decompress(&compressed)?;
```

### `MultiPassCompressor`

Applies compression in multiple passes for iterative improvement:

```rust
use blockchain_compression::algorithms::MultiPassCompressor;

let mut compressor = MultiPassCompressor::new();
let compressed = compressor.compress(&data)?;
```

### `PracticalMaxCompression`

Combines pattern engine, CTW, and multi-pass for maximum compression:

```rust
use blockchain_compression::algorithms::PracticalMaxCompression;

let mut compressor = PracticalMaxCompression::new();
let compressed = compressor.compress(&data)?;
```
