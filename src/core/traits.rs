//! Core traits for blockchain compression algorithms
//!
//! This module defines the fundamental interfaces that all compression implementations
//! must satisfy, enabling composable and pluggable compression strategies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A trait for compression algorithms that can compress and decompress data
pub trait CompressionStrategy {
    /// The error type produced by this compression strategy
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

/// Metadata about a compression algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Statistics for compression performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    /// Total number of compression operations performed
    pub compressions: u64,
    /// Total number of decompression operations performed
    pub decompressions: u64,
    /// Total original bytes processed
    pub total_input_bytes: u64,
    /// Total compressed bytes produced
    pub total_output_bytes: u64,
    /// Best compression ratio achieved (input/output)
    pub best_ratio: f64,
    /// Average compression ratio
    pub average_ratio: f64,
    /// Total time spent compressing (nanoseconds)
    pub compression_time_ns: u64,
    /// Total time spent decompressing (nanoseconds)
    pub decompression_time_ns: u64,
    /// Number of compression errors encountered
    pub errors: u64,
}

impl CompressionStats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self {
            compressions: 0,
            decompressions: 0,
            total_input_bytes: 0,
            total_output_bytes: 0,
            best_ratio: 1.0,
            average_ratio: 1.0,
            compression_time_ns: 0,
            decompression_time_ns: 0,
            errors: 0,
        }
    }

    /// Record a successful compression operation
    pub fn record_compression(&mut self, input_size: usize, output_size: usize, time_ns: u64) {
        self.compressions += 1;
        self.total_input_bytes += input_size as u64;
        self.total_output_bytes += output_size as u64;
        self.compression_time_ns += time_ns;

        let ratio = input_size as f64 / output_size as f64;
        if ratio > self.best_ratio {
            self.best_ratio = ratio;
        }

        // Update average ratio
        if self.total_output_bytes > 0 {
            self.average_ratio = self.total_input_bytes as f64 / self.total_output_bytes as f64;
        }
    }

    /// Record a successful decompression operation
    pub fn record_decompression(&mut self, time_ns: u64) {
        self.decompressions += 1;
        self.decompression_time_ns += time_ns;
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    /// Get compression throughput in MB/s
    pub fn compression_throughput_mbps(&self) -> f64 {
        if self.compression_time_ns == 0 {
            return 0.0;
        }
        let mb_processed = self.total_input_bytes as f64 / 1_000_000.0;
        let seconds = self.compression_time_ns as f64 / 1_000_000_000.0;
        mb_processed / seconds
    }

    /// Get decompression throughput in MB/s
    pub fn decompression_throughput_mbps(&self) -> f64 {
        if self.decompression_time_ns == 0 {
            return 0.0;
        }
        let mb_processed = self.total_output_bytes as f64 / 1_000_000.0;
        let seconds = self.decompression_time_ns as f64 / 1_000_000_000.0;
        mb_processed / seconds
    }
}

impl Default for CompressionStats {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait for pattern-based compression strategies
pub trait PatternCompressionStrategy: CompressionStrategy {
    /// The type representing a pattern
    type Pattern: Clone + Send + Sync;

    /// The type representing pattern configuration
    type Config: Clone + Send + Sync;

    /// Create a new instance with the given configuration
    fn with_config(config: Self::Config) -> Self;

    /// Add a pattern to the compression dictionary
    fn add_pattern(&mut self, pattern: Self::Pattern) -> Result<(), Self::Error>;

    /// Remove a pattern from the compression dictionary
    fn remove_pattern(&mut self, pattern_id: &str) -> Result<(), Self::Error>;

    /// Get information about registered patterns
    fn pattern_info(&self) -> HashMap<String, PatternInfo>;

    /// Optimize pattern dictionary based on usage statistics
    fn optimize_patterns(&mut self) -> Result<(), Self::Error>;
}

/// Information about a compression pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternInfo {
    /// Unique identifier for this pattern
    pub id: String,
    /// Size of the pattern in bytes
    pub size: usize,
    /// Number of times this pattern has been used
    pub usage_count: u64,
    /// Compression benefit (bytes saved) from this pattern
    pub bytes_saved: u64,
    /// Human-readable description
    pub description: String,
}

/// A trait for multi-stage compression pipelines
pub trait PipelineCompressionStrategy: CompressionStrategy {
    /// The type representing a compression stage
    type Stage: CompressionStrategy;

    /// Add a stage to the compression pipeline
    fn add_stage(&mut self, stage: Self::Stage) -> Result<(), Self::Error>;

    /// Remove a stage from the pipeline
    fn remove_stage(&mut self, index: usize) -> Result<Self::Stage, Self::Error>;

    /// Get the number of stages in the pipeline
    fn stage_count(&self) -> usize;

    /// Get statistics for each stage
    fn stage_stats(&self) -> Vec<CompressionStats>;

    /// Enable or disable a specific stage
    fn set_stage_enabled(&mut self, index: usize, enabled: bool) -> Result<(), Self::Error>;
}

/// A trait for adaptive compression strategies that learn from data
pub trait AdaptiveCompressionStrategy: CompressionStrategy {
    /// Train the compressor on a dataset
    fn train(&mut self, training_data: &[&[u8]]) -> Result<(), Self::Error>;

    /// Get the current learning progress (0.0 to 1.0)
    fn learning_progress(&self) -> f64;

    /// Save the learned model to bytes
    fn save_model(&self) -> Result<Vec<u8>, Self::Error>;

    /// Load a previously learned model from bytes
    fn load_model(&mut self, model_data: &[u8]) -> Result<(), Self::Error>;

    /// Get information about what the algorithm has learned
    fn learning_info(&self) -> LearningInfo;
}

/// Information about what an adaptive algorithm has learned
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInfo {
    /// Number of training samples processed
    pub training_samples: u64,
    /// Estimated model quality (0.0 to 1.0)
    pub model_quality: f64,
    /// Key patterns or features discovered
    pub discovered_features: Vec<String>,
    /// Memory usage of the learned model
    pub model_size_bytes: usize,
}

/// Common error types for compression algorithms
#[derive(Debug, thiserror::Error)]
pub enum CompressionError {
    #[error("Invalid compression format")]
    InvalidFormat,

    #[error("Unsupported algorithm version: {version}")]
    UnsupportedVersion { version: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Pattern error: {message}")]
    Pattern { message: String },

    #[error("Pipeline error at stage {stage}: {message}")]
    Pipeline { stage: usize, message: String },

    #[error("Training error: {message}")]
    Training { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Internal error: {message}")]
    Internal { message: String },
}