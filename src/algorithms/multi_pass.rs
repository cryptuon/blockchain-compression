//! Multi-pass compression for iterative improvement
//!
//! Applies multiple compression strategies in sequence to achieve maximum compression.

use crate::core::traits::{
    CompressionStrategy, PipelineCompressionStrategy,
    CompressionError, CompressionMetadata, CompressionStats
};
use crate::algorithms::enhanced_ctw::EnhancedCTW;
use std::collections::HashMap;

/// Multi-pass compression engine that applies multiple strategies
pub struct MultiPassCompressor {
    /// Individual compression stages
    stages: Vec<EnhancedCTW>,

    /// Stage enablement flags
    enabled_stages: Vec<bool>,

    /// Number of compression passes
    max_passes: usize,

    /// Improvement threshold to continue passes
    improvement_threshold: f32,

    /// Pass-specific strategies
    pass_strategies: Vec<PassStrategy>,

    /// Performance statistics
    stats: CompressionStats,

    /// Stage-specific statistics
    stage_stats: Vec<CompressionStats>,
}

#[derive(Debug, Clone)]
pub enum PassStrategy {
    /// Pattern replacement pass
    PatternReplacement,
    /// Context prediction pass
    ContextPrediction,
    /// Dictionary compression pass
    DictionaryCompression,
    /// Arithmetic coding pass
    ArithmeticCoding,
}

impl MultiPassCompressor {
    /// Create a new multi-pass compressor
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            enabled_stages: Vec::new(),
            max_passes: 3,
            improvement_threshold: 0.05, // 5% improvement to continue
            pass_strategies: vec![
                PassStrategy::PatternReplacement,
                PassStrategy::ContextPrediction,
                PassStrategy::ArithmeticCoding,
            ],
            stats: CompressionStats::new(),
            stage_stats: Vec::new(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(max_passes: usize, improvement_threshold: f32) -> Self {
        let mut compressor = Self::new();
        compressor.max_passes = max_passes;
        compressor.improvement_threshold = improvement_threshold;
        compressor
    }

    /// Add a compression strategy to the pass
    pub fn add_strategy(&mut self, strategy: PassStrategy) {
        self.pass_strategies.push(strategy);
    }

    /// Apply multi-pass compression
    pub fn compress_with_passes(&mut self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut current_data = data.to_vec();
        let mut best_data = current_data.clone();
        let mut best_ratio = 1.0f32;

        log::info!("Starting multi-pass compression with {} passes", self.max_passes);

        for pass in 0..self.max_passes {
            // Apply compression strategy for this pass
            let compressed = self.apply_pass_strategy(pass, &current_data)?;
            let ratio = current_data.len() as f32 / compressed.len() as f32;

            log::info!("Pass {}: {:.2}:1 compression", pass + 1, ratio);

            // Check if this pass improved compression
            if ratio > best_ratio * (1.0 + self.improvement_threshold) {
                best_data = compressed.clone();
                best_ratio = ratio;
                current_data = compressed; // Use for next pass
            } else {
                log::info!(
                    "Stopping after {} passes (improvement < {:.1}%)",
                    pass + 1,
                    self.improvement_threshold * 100.0
                );
                break;
            }
        }

        Ok(best_data)
    }

    fn apply_pass_strategy(&mut self, pass: usize, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let strategy_index = pass % self.pass_strategies.len();
        let strategy = &self.pass_strategies[strategy_index];

        match strategy {
            PassStrategy::PatternReplacement => self.apply_pattern_replacement(data),
            PassStrategy::ContextPrediction => self.apply_context_prediction(data),
            PassStrategy::DictionaryCompression => self.apply_dictionary_compression(data),
            PassStrategy::ArithmeticCoding => self.apply_arithmetic_coding(data),
        }
    }

    fn apply_pattern_replacement(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Simple pattern replacement for common sequences
        let mut result = Vec::new();
        let mut pos = 0;

        while pos < data.len() {
            // Look for repeating patterns
            let mut found_pattern = false;

            // Check for 4-byte patterns
            if pos + 8 <= data.len() && data[pos..pos+4] == data[pos+4..pos+8] {
                result.push(0xFF); // Pattern marker
                result.push(4); // Pattern length
                result.extend_from_slice(&data[pos..pos+4]);
                pos += 8;
                found_pattern = true;
            }

            if !found_pattern {
                result.push(data[pos]);
                pos += 1;
            }
        }

        Ok(result)
    }

    fn apply_context_prediction(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Use Enhanced CTW for context prediction
        #[cfg(feature = "deflate")]
        {
            use flate2::{Compression, write::DeflateEncoder};
            use std::io::Write;

            let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            encoder.finish().map_err(CompressionError::from)
        }

        #[cfg(not(feature = "deflate"))]
        {
            // Fallback to simple compression
            Ok(data.to_vec())
        }
    }

    fn apply_dictionary_compression(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Build a simple dictionary of common sequences
        let mut dictionary = HashMap::new();
        let mut dict_id = 0u8;

        // Find common 4-byte sequences
        for i in 0..data.len().saturating_sub(4) {
            let sequence = &data[i..i+4];
            *dictionary.entry(sequence.to_vec()).or_insert(0usize) += 1;
        }

        // Keep only sequences that appear more than once
        let useful_dict: HashMap<Vec<u8>, u8> = dictionary
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .take(255) // Limit dictionary size
            .map(|(seq, _)| {
                let id = dict_id;
                dict_id += 1;
                (seq, id)
            })
            .collect();

        // Apply dictionary compression
        let mut result = Vec::new();
        let mut pos = 0;

        while pos < data.len() {
            let mut found = false;

            // Try to match dictionary entries
            for len in (4..=8).rev() {
                if pos + len <= data.len() {
                    let sequence = &data[pos..pos+len];
                    if let Some(&dict_id) = useful_dict.get(sequence) {
                        result.push(0xFE); // Dictionary marker
                        result.push(dict_id);
                        pos += len;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                result.push(data[pos]);
                pos += 1;
            }
        }

        Ok(result)
    }

    fn apply_arithmetic_coding(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Simple arithmetic coding simulation with standard compression
        #[cfg(any(feature = "deflate", feature = "lz4", feature = "zstd"))]
        {
            self.apply_best_available_compression(data)
        }

        #[cfg(not(any(feature = "deflate", feature = "lz4", feature = "zstd")))]
        {
            // No compression backend available
            Ok(data.to_vec())
        }
    }

    #[cfg(any(feature = "deflate", feature = "lz4", feature = "zstd"))]
    fn apply_best_available_compression(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        #[cfg(feature = "zstd")]
        {
            zstd::bulk::compress(data, 19)
                .map_err(|e| CompressionError::Internal {
                    message: format!("Zstd compression failed: {}", e)
                })
        }

        #[cfg(all(feature = "lz4", not(feature = "zstd")))]
        {
            Ok(lz4_flex::compress_prepend_size(data))
        }

        #[cfg(all(feature = "deflate", not(feature = "lz4"), not(feature = "zstd")))]
        {
            use flate2::{Compression, write::DeflateEncoder};
            use std::io::Write;

            let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
            encoder.write_all(data)?;
            encoder.finish().map_err(CompressionError::from)
        }
    }
}

impl CompressionStrategy for MultiPassCompressor {
    type Error = CompressionError;

    fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let start_time = std::time::Instant::now();

        let compressed = self.compress_with_passes(data)?;

        let compression_time = start_time.elapsed().as_nanos() as u64;
        self.stats.record_compression(data.len(), compressed.len(), compression_time);

        Ok(compressed)
    }

    fn decompress(&self, _data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        // Multi-pass decompression would need to reverse each pass
        // For now, this is left as an exercise for complete implementation
        Err(CompressionError::Internal {
            message: "Multi-pass decompression not yet implemented".to_string()
        })
    }

    fn metadata(&self) -> CompressionMetadata {
        CompressionMetadata {
            name: "Multi-Pass Compressor".to_string(),
            version: "1.0.0".to_string(),
            description: "Multi-pass compression with iterative improvement".to_string(),
            deterministic: false, // Due to adaptive behavior
            memory_usage: std::mem::size_of::<Self>(),
            domains: vec!["general".to_string(), "blockchain".to_string()],
        }
    }

    fn stats(&self) -> CompressionStats {
        self.stats.clone()
    }

    fn reset(&mut self) {
        self.stages.clear();
        self.enabled_stages.clear();
        self.stats = CompressionStats::new();
        self.stage_stats.clear();
    }
}

impl PipelineCompressionStrategy for MultiPassCompressor {
    type Stage = EnhancedCTW; // Simplified for now - in a full implementation, this would be a trait object

    fn add_stage(&mut self, stage: Self::Stage) -> Result<(), Self::Error> {
        self.stages.push(stage);
        self.enabled_stages.push(true);
        self.stage_stats.push(CompressionStats::new());
        Ok(())
    }

    fn remove_stage(&mut self, index: usize) -> Result<Self::Stage, Self::Error> {
        if index >= self.stages.len() {
            return Err(CompressionError::Pipeline {
                stage: index,
                message: "Stage index out of bounds".to_string(),
            });
        }

        self.enabled_stages.remove(index);
        self.stage_stats.remove(index);
        Ok(self.stages.remove(index))
    }

    fn stage_count(&self) -> usize {
        self.stages.len()
    }

    fn stage_stats(&self) -> Vec<CompressionStats> {
        self.stage_stats.clone()
    }

    fn set_stage_enabled(&mut self, index: usize, enabled: bool) -> Result<(), Self::Error> {
        if index >= self.enabled_stages.len() {
            return Err(CompressionError::Pipeline {
                stage: index,
                message: "Stage index out of bounds".to_string(),
            });
        }

        self.enabled_stages[index] = enabled;
        Ok(())
    }
}

impl Default for MultiPassCompressor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_pass_basic() {
        let mut compressor = MultiPassCompressor::new();
        // `apply_pattern_replacement` matches immediately-adjacent 4-byte blocks
        // (`data[pos..pos+4] == data[pos+4..pos+8]`), so the test input must
        // contain those doubled blocks for the first pass to improve on the
        // baseline ratio of 1.0 and for multi-pass to return compressed output.
        let test_data = b"AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDD".repeat(20);

        let compressed = compressor.compress(&test_data).unwrap();

        // Should achieve some compression on repetitive data
        assert!(
            compressed.len() < test_data.len(),
            "expected compression on {}-byte doubled-block input, got {} bytes",
            test_data.len(),
            compressed.len()
        );
    }

    #[test]
    fn test_pipeline_operations() {
        let mut compressor = MultiPassCompressor::new();

        assert_eq!(compressor.stage_count(), 0);

        // Pipeline operations would be tested with actual stages
        // This is a placeholder for the interface
    }
}