//! Solana blockchain-specific compression presets and configurations
//!
//! This module provides optimized compression configurations specifically designed
//! for Solana blockchain data structures and patterns using Zstandard compression.

use crate::core::traits::{CompressionStrategy, CompressionError, CompressionMetadata, CompressionStats};
use serde::{Deserialize, Serialize};

#[cfg(feature = "zstd")]
use std::io::{Read, Write};

#[cfg(feature = "solana")]
use solana_sdk::{pubkey::Pubkey, signature::Signature};

/// Solana-optimized compression engine using Zstandard with custom dictionaries
pub struct SolanaCompressor {
    /// Compression level (1-22)
    compression_level: i32,
    /// Preset configuration
    preset: SolanaPreset,
    /// Custom dictionary for Solana patterns
    dictionary: Option<Vec<u8>>,
    /// Compression statistics
    stats: CompressionStats,
}

impl std::fmt::Debug for SolanaCompressor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolanaCompressor")
            .field("compression_level", &self.compression_level)
            .field("preset", &self.preset)
            .field("dictionary_size", &self.dictionary.as_ref().map(|d| d.len()))
            .field("stats", &self.stats)
            .finish()
    }
}

/// Solana compression configuration presets
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Solana-specific pattern types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SolanaPatternType {
    /// Ed25519 public keys (32 bytes)
    PublicKey,
    /// Ed25519 signatures (64 bytes)
    Signature,
    /// Program IDs (32 bytes)
    ProgramId,
    /// Token amounts (8 bytes)
    Amount,
    /// Blockhashes (32 bytes)
    Blockhash,
    /// Instruction data patterns
    InstructionData,
}

/// Common Solana program IDs and addresses for dictionary training
const SOLANA_DICTIONARY_PATTERNS: &[&str] = &[
    // System and core programs
    "11111111111111111111111111111112",  // System Program
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",  // Token Program
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",  // Associated Token Program
    "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",  // Serum DEX
    "BPFLoaderUpgradeab1e11111111111111111111111",  // BPF Loader
    "Config1111111111111111111111111111111111111", // Config Program
    "Vote111111111111111111111111111111111111111",  // Vote Program
    "Stake11111111111111111111111111111111111111",  // Stake Program

    // Common instruction patterns
    "00000000",  // Transfer instruction
    "01000000",  // Initialize account
    "02000000",  // Close account
    "03000000",  // Approve

    // Common amounts (as bytes)
    "00e1f50500000000",  // 1 SOL in lamports
    "00ca9a3b00000000",  // 0.1 SOL in lamports
    "0010270000000000",  // 0.01 SOL in lamports
    "00e40b5402000000",  // 10 SOL in lamports

    // Common transaction structure markers
    "0100",  // Single signature
    "0200",  // Two signatures
    "0300",  // Three signatures
];

impl SolanaCompressor {
    /// Create a new Solana compressor with the specified preset
    pub fn new(preset: SolanaPreset) -> Self {
        let compression_level = Self::preset_to_level(&preset);
        let dictionary = Self::build_solana_dictionary();

        Self {
            compression_level,
            preset,
            dictionary: Some(dictionary),
            stats: CompressionStats::default(),
        }
    }

    /// Convert preset to compression level
    fn preset_to_level(preset: &SolanaPreset) -> i32 {
        match preset {
            SolanaPreset::FastCompression => 3,     // Fast compression
            SolanaPreset::Transactions => 3,        // Fast for real-time processing
            SolanaPreset::Instructions => 6,        // Balanced
            SolanaPreset::Accounts => 6,            // Balanced
            SolanaPreset::Mixed => 6,               // Balanced for general use
            SolanaPreset::MaxCompression => 19,     // Maximum compression
        }
    }

    /// Build a custom dictionary from common Solana patterns
    fn build_solana_dictionary() -> Vec<u8> {
        let mut dictionary_data = Vec::new();

        // Add all common patterns to dictionary
        for pattern in SOLANA_DICTIONARY_PATTERNS {
            dictionary_data.extend_from_slice(pattern.as_bytes());
            dictionary_data.push(0); // Null separator
        }

        // Add some common Base58 character sequences
        dictionary_data.extend_from_slice(b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

        dictionary_data
    }

    /// Reset internal compression state
    pub fn reset(&mut self) {
        self.stats = CompressionStats::default();
    }

    /// Get common Solana program IDs that appear frequently
    #[cfg(feature = "solana")]
    pub fn common_program_ids() -> Vec<Pubkey> {
        vec![
            solana_sdk::system_program::ID,              // System program
            spl_token::ID,                                // Token program
            spl_associated_token_account::ID,             // Associated token account
            solana_sdk::sysvar::rent::ID,                 // Rent sysvar
            solana_sdk::sysvar::clock::ID,                // Clock sysvar
            // Add more common program IDs as needed
        ]
    }

    /// Pre-populate the compressor with common Solana patterns
    #[cfg(feature = "solana")]
    pub fn pre_populate_common_patterns(&mut self) -> Result<(), CompressionError> {
        // Add common program IDs
        for program_id in Self::common_program_ids() {
            self.add_pubkey_pattern(program_id)?;
        }

        // Add common amounts (powers of 10 in lamports)
        for i in 0..=9 {
            let amount = 10_u64.pow(i);
            self.add_amount_pattern(amount)?;
        }

        Ok(())
    }

    /// Add a specific public key pattern
    #[cfg(feature = "solana")]
    pub fn add_pubkey_pattern(&mut self, pubkey: Pubkey) -> Result<(), CompressionError> {
        // This would integrate with the pattern engine to add the pubkey
        // Implementation would depend on the pattern engine's API
        Ok(())
    }

    /// Add a specific amount pattern
    pub fn add_amount_pattern(&mut self, amount: u64) -> Result<(), CompressionError> {
        // This would integrate with the pattern engine to add the amount
        // Implementation would depend on the pattern engine's API
        Ok(())
    }

    /// Optimize specifically for Solana transaction patterns
    pub fn optimize_for_transactions(&mut self) -> Result<(), CompressionError> {
        // This could analyze usage patterns and optimize for transaction-specific patterns
        Ok(())
    }

    /// Get Solana-specific compression statistics
    pub fn solana_stats(&self) -> SolanaCompressionStats {
        // Simplified for zstd implementation
        SolanaCompressionStats {
            pubkey_patterns: 0,
            signature_patterns: 0,
            amount_patterns: 0,
            total_solana_bytes_saved: 0,
        }
    }
}

/// Solana-specific compression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaCompressionStats {
    /// Number of signature patterns
    pub signature_patterns: usize,
    /// Number of public key patterns
    pub pubkey_patterns: usize,
    /// Number of amount patterns
    pub amount_patterns: usize,
    /// Total bytes saved by Solana-specific patterns
    pub total_solana_bytes_saved: u64,
}

impl CompressionStrategy for SolanaCompressor {
    type Error = CompressionError;

    fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        let _start_time = std::time::Instant::now();

        let compressed: Vec<u8> = match &self.dictionary {
            Some(dict) => {
                // Use custom dictionary for optimal Solana compression
                #[cfg(feature = "zstd")]
                {
                    let mut encoder = zstd::stream::write::Encoder::with_dictionary(Vec::new(), self.compression_level, dict)
                    .map_err(|e| CompressionError::Internal {
                        message: format!("Failed to create zstd encoder with dictionary: {}", e),
                    })?;

                encoder.write_all(data)
                    .map_err(|e| CompressionError::Internal {
                        message: format!("Failed to write data to zstd encoder: {}", e),
                    })?;

                encoder.finish()
                    .map_err(|e| CompressionError::Internal {
                        message: format!("Failed to finish zstd compression: {}", e),
                    })?
                }
                #[cfg(not(feature = "zstd"))]
                {
                    return Err(CompressionError::Internal {
                        message: "zstd feature not enabled".to_string(),
                    });
                }
            }
            None => {
                // Fallback to standard zstd compression
                #[cfg(feature = "zstd")]
                {
                    zstd::bulk::compress(data, self.compression_level)
                        .map_err(|e| CompressionError::Internal {
                            message: format!("zstd compression failed: {}", e),
                        })?
                }
                #[cfg(not(feature = "zstd"))]
                {
                    return Err(CompressionError::Internal {
                        message: "zstd feature not enabled".to_string(),
                    });
                }
            }
        };

        // Update statistics
        self.stats.compressions += 1;
        self.stats.total_input_bytes += data.len() as u64;
        self.stats.total_output_bytes += compressed.len() as u64;

        let ratio = data.len() as f64 / compressed.len() as f64;
        if ratio > self.stats.best_ratio {
            self.stats.best_ratio = ratio;
        }

        Ok(compressed)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        let _start_time = std::time::Instant::now();

        let decompressed: Vec<u8> = match &self.dictionary {
            Some(dict) => {
                // Use custom dictionary for decompression
                #[cfg(feature = "zstd")]
                {
                    let mut decoder = zstd::stream::read::Decoder::with_dictionary(data, dict)
                    .map_err(|e| CompressionError::Internal {
                        message: format!("Failed to create zstd decoder with dictionary: {}", e),
                    })?;

                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)
                    .map_err(|e| CompressionError::Internal {
                        message: format!("Failed to decompress with zstd: {}", e),
                    })?;

                decompressed
                }
                #[cfg(not(feature = "zstd"))]
                {
                    return Err(CompressionError::Internal {
                        message: "zstd feature not enabled".to_string(),
                    });
                }
            }
            None => {
                // Fallback to standard zstd decompression
                #[cfg(feature = "zstd")]
                {
                    zstd::bulk::decompress(data, 1024 * 1024) // 1MB limit for safety
                        .map_err(|e| CompressionError::Internal {
                            message: format!("zstd decompression failed: {}", e),
                        })?
                }
                #[cfg(not(feature = "zstd"))]
                {
                    return Err(CompressionError::Internal {
                        message: "zstd feature not enabled".to_string(),
                    });
                }
            }
        };

        // Note: Can't update stats here due to &self constraint from trait

        Ok(decompressed)
    }

    fn metadata(&self) -> CompressionMetadata {
        CompressionMetadata {
            name: "Solana Zstd Compressor".to_string(),
            description: "Solana-optimized compression using Zstandard with custom dictionaries".to_string(),
            version: "2.0.0".to_string(),
            domains: vec!["Solana".to_string(), "Blockchain".to_string()],
            deterministic: true,
            memory_usage: (self.compression_level * 1024 * 1024) as usize, // Rough estimate
        }
    }

    fn stats(&self) -> CompressionStats {
        self.stats.clone()
    }

    fn reset(&mut self) {
        self.stats = CompressionStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_compressor_creation() {
        let compressor = SolanaCompressor::new(SolanaPreset::Mixed);
        let metadata = compressor.metadata();
        assert_eq!(metadata.name, "Solana Zstd Compressor");
    }

    #[test]
    fn test_transaction_preset() {
        let compressor = SolanaCompressor::new(SolanaPreset::Transactions);
        let stats = compressor.stats();
        assert_eq!(stats.compressions, 0);
    }

    #[test]
    fn test_zstd_compression_with_solana_patterns() {
        let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

        // Create test data with repetitive Solana patterns that should compress well
        let mut test_data = Vec::new();

        // Add common Solana program IDs (from dictionary)
        for _ in 0..20 {
            test_data.extend_from_slice("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes()); // Token Program
            test_data.extend_from_slice("11111111111111111111111111111112".as_bytes()); // System Program
        }

        println!("Original size: {} bytes", test_data.len());

        let compressed = compressor.compress(&test_data).expect("Compression should work");
        println!("Compressed size: {} bytes", compressed.len());

        // Decompress using the trait method
        let decompressed = (&compressor as &dyn CompressionStrategy<Error = _>).decompress(&compressed)
            .expect("Decompression should work");

        // Verify perfect data integrity
        assert_eq!(test_data, decompressed, "zstd must provide perfect data integrity");

        // Should achieve excellent compression on repetitive Solana patterns
        let ratio = test_data.len() as f64 / compressed.len() as f64;
        println!("Compression ratio: {:.2}:1", ratio);
        assert!(ratio > 10.0, "Should achieve excellent compression on Solana patterns, got {:.2}:1", ratio);

        // Verify stats
        let stats = compressor.stats();
        assert_eq!(stats.compressions, 1);
        assert!(stats.best_ratio > 10.0);
    }

    #[test]
    fn test_preset_configurations() {
        let presets = vec![
            SolanaPreset::Transactions,
            SolanaPreset::Accounts,
            SolanaPreset::Instructions,
            SolanaPreset::Mixed,
            SolanaPreset::MaxCompression,
            SolanaPreset::FastCompression,
        ];

        for preset in presets {
            let compressor = SolanaCompressor::new(preset);
            let metadata = compressor.metadata();
            assert_eq!(metadata.name, "Solana Zstd Compressor");
        }
    }
}