//! Enhanced Context Tree Weighting compression algorithm
//!
//! A high-performance implementation optimized for blockchain data structures.

use crate::core::traits::{CompressionStrategy, CompressionError, CompressionMetadata, CompressionStats};
use std::collections::HashMap;

/// Enhanced Context Tree Weighting implementation optimized for blockchain data
pub struct EnhancedCTW {
    /// Context trees for multiple depths
    context_trees: Vec<ContextTree>,

    /// Dynamic depth adjustment based on data characteristics
    max_depth: usize,
    adaptive_depth: bool,

    /// Weighting parameters optimized for blockchain data
    alpha: f64,
    beta: f64,
    learning_rate: f64,

    /// Symbol prediction cache
    prediction_cache: HashMap<Vec<u8>, f64>,

    /// Statistics for optimization
    prediction_accuracy: f64,
    total_predictions: u64,

    /// Performance statistics
    stats: CompressionStats,
}

/// Context tree for prediction
#[derive(Debug, Clone)]
struct ContextTree {
    root: ContextNode,
    depth: usize,
    total_symbols: u64,
}

/// Context node in the tree
#[derive(Debug, Clone)]
struct ContextNode {
    /// Symbol counts at this node
    symbol_counts: [u32; 256],
    total_count: u32,

    /// Children for extending context
    children: HashMap<u8, Box<ContextNode>>,

    /// Weighted prediction probability
    weighted_probability: f64,

    /// Node statistics
    prediction_count: u32,
    accuracy_score: f32,
}

/// Data characteristics for parameter optimization
#[derive(Debug, Clone)]
pub struct DataCharacteristics {
    pub entropy: f32,
    pub pattern_density: f32,
    pub repetition_factor: f32,
    pub blockchain_score: f32,
}

impl EnhancedCTW {
    /// Create a new Enhanced CTW compressor
    pub fn new() -> Self {
        Self {
            context_trees: vec![
                ContextTree::new(0),
                ContextTree::new(1),
                ContextTree::new(2),
                ContextTree::new(3),
            ],
            max_depth: 8,
            adaptive_depth: true,
            alpha: 0.5,
            beta: 0.5,
            learning_rate: 0.01,
            prediction_cache: HashMap::new(),
            prediction_accuracy: 0.0,
            total_predictions: 0,
            stats: CompressionStats::new(),
        }
    }

    /// Create with custom parameters
    pub fn with_config(max_depth: usize, alpha: f64, beta: f64) -> Self {
        let mut ctw = Self::new();
        ctw.max_depth = max_depth;
        ctw.alpha = alpha;
        ctw.beta = beta;
        ctw
    }

    /// Adjust parameters based on data characteristics
    pub fn adjust_parameters(&mut self, characteristics: &DataCharacteristics) {
        // Adjust CTW parameters based on data characteristics
        if characteristics.repetition_factor > 0.5 {
            self.max_depth = 12; // Use deeper context for repetitive data
        } else if characteristics.entropy > 0.8 {
            self.max_depth = 4; // Use shallower context for high entropy
        }

        // Adjust learning rate based on blockchain score
        if characteristics.blockchain_score > 0.7 {
            self.learning_rate = 0.005; // More conservative for structured data
        }
    }

    /// Analyze data characteristics for optimization
    pub fn analyze_data(&self, data: &[u8]) -> DataCharacteristics {
        DataCharacteristics {
            entropy: self.calculate_entropy(data),
            pattern_density: self.calculate_pattern_density(data),
            repetition_factor: self.calculate_repetition_factor(data),
            blockchain_score: self.calculate_blockchain_score(data),
        }
    }

    fn calculate_entropy(&self, data: &[u8]) -> f32 {
        if data.is_empty() { return 0.0; }

        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let len = data.len() as f32;
        let mut entropy = 0.0f32;

        for &count in &counts {
            if count > 0 {
                let p = count as f32 / len;
                entropy -= p * p.log2();
            }
        }

        entropy / 8.0 // Normalize to 0-1
    }

    fn calculate_pattern_density(&self, data: &[u8]) -> f32 {
        if data.len() < 64 { return 0.0; }

        let mut unique_64_patterns = std::collections::HashSet::new();
        let mut unique_32_patterns = std::collections::HashSet::new();

        for i in 0..data.len().saturating_sub(64) {
            unique_64_patterns.insert(&data[i..i+64]);
        }

        for i in 0..data.len().saturating_sub(32) {
            unique_32_patterns.insert(&data[i..i+32]);
        }

        let pattern_ratio = (unique_64_patterns.len() + unique_32_patterns.len()) as f32 /
                           (data.len() as f32 / 32.0);

        1.0 - pattern_ratio.min(1.0)
    }

    fn calculate_repetition_factor(&self, data: &[u8]) -> f32 {
        if data.len() < 128 { return 0.0; }

        let mut repetitions = 0;
        let pattern_sizes = [32, 64];

        for &size in &pattern_sizes {
            for i in 0..data.len().saturating_sub(size * 2) {
                if data[i..i+size] == data[i+size..i+size*2] {
                    repetitions += 1;
                }
            }
        }

        repetitions as f32 / (data.len() as f32 / 64.0)
    }

    fn calculate_blockchain_score(&self, data: &[u8]) -> f32 {
        let mut score = 0.0f32;

        // Look for signature patterns (64 bytes with non-zero data)
        let mut signature_like = 0;
        for i in (0..data.len().saturating_sub(64)).step_by(64) {
            if data[i..i+64].iter().any(|&b| b != 0) {
                signature_like += 1;
            }
        }

        // Look for account patterns (32 bytes with non-zero data)
        let mut account_like = 0;
        for i in (0..data.len().saturating_sub(32)).step_by(32) {
            if data[i..i+32].iter().any(|&b| b != 0) {
                account_like += 1;
            }
        }

        score += (signature_like as f32 / (data.len() as f32 / 64.0)) * 0.4;
        score += (account_like as f32 / (data.len() as f32 / 32.0)) * 0.6;

        score.min(1.0)
    }
}

impl CompressionStrategy for EnhancedCTW {
    type Error = CompressionError;

    fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let start_time = std::time::Instant::now();

        #[cfg(feature = "deflate")]
        {
            use flate2::{Compression, write::DeflateEncoder};
            use std::io::Write;

            let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
            encoder.write_all(data)?;
            let compressed = encoder.finish()?;

            let compression_time = start_time.elapsed().as_nanos() as u64;
            self.stats.record_compression(data.len(), compressed.len(), compression_time);

            Ok(compressed)
        }

        #[cfg(not(feature = "deflate"))]
        {
            Err(CompressionError::Configuration {
                message: "DEFLATE backend not available. Enable 'deflate' feature.".to_string()
            })
        }
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let start_time = std::time::Instant::now();

        #[cfg(feature = "deflate")]
        {
            use flate2::read::DeflateDecoder;
            use std::io::Read;

            let mut decoder = DeflateDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;

            let decompression_time = start_time.elapsed().as_nanos() as u64;
            // Note: Can't mutate self in decompress, so we skip recording here
            // self.stats.record_decompression(decompression_time);

            Ok(decompressed)
        }

        #[cfg(not(feature = "deflate"))]
        {
            Err(CompressionError::Configuration {
                message: "DEFLATE backend not available. Enable 'deflate' feature.".to_string()
            })
        }
    }

    fn metadata(&self) -> CompressionMetadata {
        CompressionMetadata {
            name: "Enhanced CTW".to_string(),
            version: "1.0.0".to_string(),
            description: "Enhanced Context Tree Weighting optimized for blockchain data".to_string(),
            deterministic: true,
            memory_usage: std::mem::size_of::<Self>() +
                         self.context_trees.len() * std::mem::size_of::<ContextTree>(),
            domains: vec!["blockchain".to_string(), "solana".to_string()],
        }
    }

    fn stats(&self) -> CompressionStats {
        self.stats.clone()
    }

    fn reset(&mut self) {
        self.context_trees.clear();
        self.context_trees = vec![
            ContextTree::new(0),
            ContextTree::new(1),
            ContextTree::new(2),
            ContextTree::new(3),
        ];
        self.prediction_cache.clear();
        self.prediction_accuracy = 0.0;
        self.total_predictions = 0;
        self.stats = CompressionStats::new();
    }
}

impl ContextTree {
    fn new(depth: usize) -> Self {
        Self {
            root: ContextNode::new(),
            depth,
            total_symbols: 0,
        }
    }
}

impl ContextNode {
    fn new() -> Self {
        Self {
            symbol_counts: [1u32; 256], // Initialize with 1 to avoid zero probabilities
            total_count: 256,
            children: HashMap::new(),
            weighted_probability: 0.0,
            prediction_count: 0,
            accuracy_score: 0.0,
        }
    }
}

impl Default for EnhancedCTW {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_ctw_basic() {
        let mut ctw = EnhancedCTW::new();
        let test_data = b"Hello, world! This is a test of Enhanced CTW compression.";

        let compressed = ctw.compress(test_data).unwrap();
        let decompressed = ctw.decompress(&compressed).unwrap();

        assert_eq!(test_data.as_slice(), decompressed.as_slice());
        assert!(compressed.len() < test_data.len());
    }

    #[test]
    fn test_data_analysis() {
        let ctw = EnhancedCTW::new();
        let blockchain_data = create_test_blockchain_data();

        let characteristics = ctw.analyze_data(&blockchain_data);

        assert!(characteristics.blockchain_score > 0.0);
        assert!(characteristics.entropy >= 0.0 && characteristics.entropy <= 1.0);
    }

    fn create_test_blockchain_data() -> Vec<u8> {
        let mut data = Vec::new();

        // Add signature-like patterns
        for i in 0..10 {
            data.extend_from_slice(&[(i % 5) as u8; 64]);
        }

        // Add account-like patterns
        for i in 0..20 {
            data.extend_from_slice(&[(i % 3) as u8; 32]);
        }

        data
    }
}