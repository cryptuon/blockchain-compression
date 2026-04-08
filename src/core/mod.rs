//! Core compression framework
//!
//! This module contains the fundamental traits and types that define the compression
//! framework, including pattern engines and pipeline components.

pub mod traits;
pub mod pattern_engine;

pub use traits::{
    CompressionStrategy, PatternCompressionStrategy, PipelineCompressionStrategy,
    AdaptiveCompressionStrategy, CompressionError, CompressionMetadata, CompressionStats,
    PatternInfo, LearningInfo
};
pub use pattern_engine::{PatternEngine, PatternConfig, PatternUsage};