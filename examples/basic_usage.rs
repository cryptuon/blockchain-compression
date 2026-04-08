//! Basic usage example for blockchain-compression library
//!
//! This example demonstrates the fundamental compression operations:
//! - Creating a compressor with different presets
//! - Compressing and decompressing data
//! - Verifying data integrity
//! - Measuring compression ratios

use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Blockchain Compression Library - Basic Usage Example");
    println!("========================================================\n");

    // Example 1: Basic compression with transaction preset
    basic_compression_example()?;

    // Example 2: Compare different presets
    compare_presets_example()?;

    // Example 3: Real-world Solana data patterns
    solana_patterns_example()?;

    Ok(())
}

fn basic_compression_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Example 1: Basic Compression");
    println!("--------------------------------");

    // Create a compressor optimized for transaction data
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Sample data with some repetitive patterns
    let original_data = b"Hello, blockchain compression! This is a test.".repeat(20);

    println!("Original size: {} bytes", original_data.len());

    // Compress the data
    let compressed = compressor.compress(&original_data)?;
    println!("Compressed size: {} bytes", compressed.len());

    // Decompress and verify integrity
    let decompressed = compressor.decompress(&compressed)?;

    if original_data == decompressed {
        println!("✅ Perfect data integrity verified!");
    } else {
        println!("❌ Data integrity check failed!");
        return Err("Data integrity check failed".into());
    }

    let ratio = original_data.len() as f64 / compressed.len() as f64;
    println!("Compression ratio: {:.2}:1\n", ratio);

    Ok(())
}

fn compare_presets_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Example 2: Comparing Different Presets");
    println!("------------------------------------------");

    let test_data = b"Sample blockchain data with repetitive patterns and common structures".repeat(50);

    let presets = vec![
        ("Fast Compression", SolanaPreset::FastCompression),
        ("Transactions", SolanaPreset::Transactions),
        ("Accounts", SolanaPreset::Accounts),
        ("Mixed Data", SolanaPreset::Mixed),
        ("Max Compression", SolanaPreset::MaxCompression),
    ];

    println!("Test data size: {} bytes\n", test_data.len());

    for (name, preset) in presets {
        let mut compressor = SolanaCompressor::new(preset);
        let compressed = compressor.compress(&test_data)?;

        // Verify integrity
        let decompressed = compressor.decompress(&compressed)?;
        assert_eq!(test_data.as_slice(), decompressed.as_slice());

        let ratio = test_data.len() as f64 / compressed.len() as f64;
        println!("{:15}: {:4} bytes ({:5.2}:1)", name, compressed.len(), ratio);
    }
    println!();

    Ok(())
}

fn solana_patterns_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Example 3: Real Solana Data Patterns");
    println!("---------------------------------------");

    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Create realistic Solana transaction data
    let mut solana_data = Vec::new();

    // Common Solana program IDs (these compress extremely well due to dictionary)
    for _ in 0..10 {
        solana_data.extend_from_slice("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes()); // Token Program
        solana_data.extend_from_slice("11111111111111111111111111111112".as_bytes()); // System Program
        solana_data.extend_from_slice("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".as_bytes()); // Associated Token
    }

    // Common transaction amounts (8-byte little-endian)
    for _ in 0..5 {
        solana_data.extend_from_slice(&1_000_000_000u64.to_le_bytes()); // 1 SOL
        solana_data.extend_from_slice(&100_000_000u64.to_le_bytes());   // 0.1 SOL
    }

    // Common instruction patterns
    for _ in 0..10 {
        solana_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Transfer instruction
        solana_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // Initialize account
    }

    println!("Solana data size: {} bytes", solana_data.len());

    let compressed = compressor.compress(&solana_data)?;
    println!("Compressed size: {} bytes", compressed.len());

    let decompressed = compressor.decompress(&compressed)?;
    assert_eq!(solana_data, decompressed);
    println!("✅ Perfect data integrity verified!");

    let ratio = solana_data.len() as f64 / compressed.len() as f64;
    println!("Solana data compression ratio: {:.2}:1", ratio);

    // Get compression statistics
    let stats = compressor.stats();
    println!("\n📊 Compression Statistics:");
    println!("- Total compressions: {}", stats.compressions);
    println!("- Average ratio: {:.2}:1", stats.average_ratio);
    println!("- Best ratio achieved: {:.2}:1", stats.best_ratio);

    Ok(())
}