# Architecture

## Module Structure

```
blockchain-compression/
  src/
    lib.rs              # Crate root, re-exports
    core/
      traits.rs         # CompressionStrategy and related traits
      pattern_engine.rs # Generic pattern recognition engine
    algorithms/
      enhanced_ctw.rs   # Context Tree Weighting compression
      multi_pass.rs     # Multi-pass iterative compression
      practical_max.rs  # Combined maximum compression
    presets/
      solana.rs         # Solana-optimized compressor and presets
```

## Design Principles

### Trait-Based Composition

The library is built around the `CompressionStrategy` trait. Every compressor -- whether it's the Solana preset, the pattern engine, or the CTW algorithm -- implements this trait. This makes compressors interchangeable and composable.

```
CompressionStrategy (core trait)
  |
  +-- PatternCompressionStrategy  (pattern management)
  +-- PipelineCompressionStrategy (multi-stage pipelines)
  +-- AdaptiveCompressionStrategy (learning-based)
```

### Three Layers

**Core** (`src/core/`) defines traits and the generic pattern recognition engine. It has no blockchain-specific knowledge.

**Algorithms** (`src/algorithms/`) implements compression algorithms:

- `EnhancedCTW` -- prediction-based compression using context tree weighting
- `MultiPassCompressor` -- applies compression in multiple passes, stopping when improvement drops below a threshold
- `PracticalMaxCompression` -- orchestrates pattern engine + CTW + multi-pass for maximum compression

**Presets** (`src/presets/`) provides blockchain-specific configurations. The `SolanaCompressor` wraps Zstandard with custom dictionaries built from common Solana byte patterns.

### Custom Dictionary Approach

The Solana compressor builds a custom Zstandard dictionary at initialization from hardcoded patterns:

1. Common program IDs (System, Token, Associated Token, Serum DEX, etc.)
2. Instruction discriminators (Transfer, Initialize, Close, Approve)
3. Common token amounts in lamports (0.01, 0.1, 1, 10 SOL)
4. Transaction structure markers (signature counts)
5. Base58 character set

This dictionary is passed to the Zstandard encoder/decoder, allowing it to reference these common byte sequences during compression without storing them in the output.

### Compression Flow

```
Input Data
    |
    v
SolanaCompressor.compress()
    |
    +-- Select compression level from preset
    +-- Load custom Solana dictionary
    +-- Encode via zstd::Encoder with dictionary
    +-- Update compression statistics
    |
    v
Compressed Bytes
```

Decompression reverses the process using the same dictionary.

### Error Handling

All operations return `Result<_, CompressionError>`. The error type is an enum with variants for each failure category (format, configuration, I/O, internal). Errors carry descriptive messages for debugging.

### Feature Flags

Compression backends are behind feature flags to avoid pulling in unnecessary dependencies:

| Flag | Crate | Description |
|------|-------|-------------|
| `deflate` (default) | `flate2` | DEFLATE compression |
| `zstd` | `zstd` | Zstandard compression |
| `lz4` | `lz4_flex` | LZ4 compression |

The Solana presets require the `zstd` feature. The pattern engine can use any backend.
