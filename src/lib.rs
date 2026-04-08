//! # blockchain-compression
//!
//! A high-performance compression library optimized for blockchain data structures.
//!
//! This library provides specialized compression algorithms that understand the patterns
//! and structures common in blockchain data, achieving significantly better compression
//! ratios than general-purpose algorithms.
//!
//! ## Features
//!
//! - **Pattern Recognition**: Automatically detects and compresses blockchain-specific patterns
//! - **Multi-Algorithm Support**: Multiple compression backends (DEFLATE, LZ4, Zstd)
//! - **Blockchain Presets**: Pre-configured for different blockchain ecosystems
//! - **High Performance**: Optimized for throughput and compression ratio
//! - **Composable**: Trait-based architecture allows for custom compression strategies
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
//! use blockchain_compression::core::traits::CompressionStrategy;
//!
//! let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);
//! # let data = b"test";
//! let compressed = compressor.compress(data).unwrap();
//! let decompressed = compressor.decompress(&compressed).unwrap();
//! ```

pub mod core;
pub mod presets;
pub mod algorithms;

// Re-export commonly used items
pub use core::traits::{CompressionStrategy, CompressionError, CompressionStats, CompressionMetadata};
pub use algorithms::{EnhancedCTW, MultiPassCompressor, PracticalMaxCompression};

// Re-export blockchain presets when available
pub use presets::solana::SolanaCompressor;