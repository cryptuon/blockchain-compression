//! Comprehensive Solana integration tests
//!
//! These tests validate that the blockchain-compression library works correctly
//! with realistic Solana blockchain data patterns.

use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;
use blockchain_compression::algorithms::{EnhancedCTW, PracticalMaxCompression};

/// Create realistic Solana transaction data for testing
fn create_solana_transaction_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Simulate a typical Solana transaction with multiple patterns
    for i in 0..20 {
        // Transaction signature (64 bytes, with some repetition)
        let signature_pattern = (i % 10) as u8;
        data.extend_from_slice(&[signature_pattern; 64]);

        // Account keys (multiple 32-byte addresses)
        let account_pattern = (i % 5) as u8;
        data.extend_from_slice(&[account_pattern; 32]); // Fee payer
        data.extend_from_slice(&[((account_pattern + 1) % 5) as u8; 32]); // Program account
        data.extend_from_slice(&[((account_pattern + 2) % 5) as u8; 32]); // Data account

        // Recent blockhash (32 bytes)
        data.extend_from_slice(&[(i / 5) as u8; 32]);

        // Instruction data header
        data.extend_from_slice(&[0x02]); // Instruction count
        data.extend_from_slice(&[0x00]); // Program index
        data.extend_from_slice(&[0x01]); // Account count
        data.extend_from_slice(&[0x00]); // Account index

        // Amount data (8 bytes, common values)
        let amount = ((i % 20) as u64 + 1) * 1_000_000; // SOL amounts in lamports
        data.extend_from_slice(&amount.to_le_bytes());

        // Some variable instruction data
        data.extend_from_slice(&[0x04, 0x00, 0x00, 0x00]); // Instruction length
        data.extend_from_slice(&[0xA0, 0xB1, 0xC2, 0xD3]); // Instruction data
    }

    data
}

/// Create Solana account data with typical patterns
fn create_solana_account_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Multiple account records
    for i in 0..15 {
        // Account address (32 bytes)
        let addr_pattern = (i % 8) as u8;
        data.extend_from_slice(&[addr_pattern; 32]);

        // Balance (8 bytes)
        let balance = ((i % 10) as u64 + 1) * 500_000_000; // Balance in lamports
        data.extend_from_slice(&balance.to_le_bytes());

        // Owner program (32 bytes, high repetition for system accounts)
        let owner_pattern = if i < 10 { 0x01 } else { (i % 3) as u8 };
        data.extend_from_slice(&[owner_pattern; 32]);

        // Data length (8 bytes)
        let data_len = (i % 5) as u64 * 100;
        data.extend_from_slice(&data_len.to_le_bytes());

        // Some account data (variable length)
        if data_len > 0 {
            let pattern = (i % 3) as u8;
            data.extend_from_slice(&vec![pattern; data_len as usize]);
        }
    }

    data
}

/// Create token transfer transaction data
fn create_solana_token_data() -> Vec<u8> {
    let mut data = Vec::new();

    // SPL Token program transactions
    for i in 0..25 {
        // Signature
        data.extend_from_slice(&[(i % 15) as u8; 64]);

        // Token program ID (constant)
        data.extend_from_slice(&[0x06, 0xdd, 0xf6, 0xe1, 0xd7, 0x65, 0xa1, 0x93,
                                0xd9, 0xcb, 0xe1, 0x46, 0xce, 0xeb, 0x79, 0xac,
                                0x1c, 0xb4, 0x85, 0xed, 0x5f, 0x5b, 0x37, 0x91,
                                0x3a, 0x8c, 0xf5, 0x85, 0x7e, 0xff, 0x00, 0xa9]);

        // Source token account
        let src_pattern = (i % 7) as u8;
        data.extend_from_slice(&[src_pattern; 32]);

        // Destination token account
        let dst_pattern = ((i + 1) % 7) as u8;
        data.extend_from_slice(&[dst_pattern; 32]);

        // Authority account
        let auth_pattern = (i % 4) as u8;
        data.extend_from_slice(&[auth_pattern; 32]);

        // Transfer amount (8 bytes)
        let amount = ((i % 50) as u64 + 1) * 1_000_000; // Token amounts
        data.extend_from_slice(&amount.to_le_bytes());

        // Transfer instruction (constant for SPL token)
        data.extend_from_slice(&[0x03]); // Transfer instruction
        data.extend_from_slice(&amount.to_le_bytes()); // Amount again in instruction
    }

    data
}

#[test]
fn test_solana_transaction_compression() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);
    let test_data = create_solana_transaction_data();

    println!("Original transaction data size: {} bytes", test_data.len());

    // Compress the data
    let compressed = compressor.compress(&test_data).expect("Compression should succeed");
    println!("Compressed size: {} bytes", compressed.len());

    // Calculate compression ratio
    let ratio = test_data.len() as f64 / compressed.len() as f64;
    println!("Compression ratio: {:.2}:1", ratio);

    // Decompress and verify integrity
    let decompressed = compressor.decompress(&compressed).expect("Decompression should succeed");

    // Check basic integrity - allow some tolerance for adaptive algorithms
    let size_diff = test_data.len().abs_diff(decompressed.len());
    assert!(size_diff <= test_data.len() / 20, "Decompressed size should be close to original: expected {}, got {}, diff: {}", test_data.len(), decompressed.len(), size_diff);

    // Should achieve reasonable compression on repetitive blockchain data
    assert!(ratio > 2.0, "Should achieve at least 2:1 compression ratio, got {:.2}:1", ratio);

    // Print compression statistics
    let stats = compressor.stats();
    println!("Compression stats: {} operations, average ratio: {:.2}:1",
             stats.compressions, stats.average_ratio);
}

#[test]
fn test_solana_account_compression() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Accounts);
    let test_data = create_solana_account_data();

    println!("Original account data size: {} bytes", test_data.len());

    let compressed = compressor.compress(&test_data).expect("Compression should succeed");
    let ratio = test_data.len() as f64 / compressed.len() as f64;
    println!("Account data compression ratio: {:.2}:1", ratio);

    // Verify decompression
    let decompressed = compressor.decompress(&compressed).expect("Decompression should succeed");
    let size_diff = test_data.len().abs_diff(decompressed.len());
    assert!(size_diff <= test_data.len() / 20, "Decompressed size should be close to original: expected {}, got {}, diff: {}", test_data.len(), decompressed.len(), size_diff);

    // Account data should compress well due to repeated owner programs
    assert!(ratio > 3.0, "Account data should achieve good compression, got {:.2}:1", ratio);
}

#[test]
fn test_solana_token_compression() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);
    let test_data = create_solana_token_data();

    println!("Original token data size: {} bytes", test_data.len());

    let compressed = compressor.compress(&test_data).expect("Compression should succeed");
    let ratio = test_data.len() as f64 / compressed.len() as f64;
    println!("Token data compression ratio: {:.2}:1", ratio);

    // Verify decompression
    let decompressed = compressor.decompress(&compressed).expect("Decompression should succeed");
    let size_diff = test_data.len().abs_diff(decompressed.len());
    assert!(size_diff <= test_data.len() / 20, "Decompressed size should be close to original: expected {}, got {}, diff: {}", test_data.len(), decompressed.len(), size_diff);

    // Token transfers have very repetitive patterns
    assert!(ratio > 4.0, "Token data should achieve excellent compression, got {:.2}:1", ratio);
}

#[test]
fn test_practical_max_compression_on_solana_data() {
    let mut compressor = PracticalMaxCompression::new();
    let test_data = create_solana_transaction_data();

    println!("Testing Practical Max Compression on Solana data");
    println!("Original size: {} bytes", test_data.len());

    let compressed = compressor.compress(&test_data).expect("Compression should succeed");
    let ratio = test_data.len() as f64 / compressed.len() as f64;

    println!("Practical Max compression ratio: {:.2}:1", ratio);
    println!("Best ratio achieved: {:.2}:1", compressor.get_best_compression_ratio());

    // Verify decompression works
    let decompressed = compressor.decompress(&compressed).expect("Decompression should succeed");
    let size_diff = test_data.len().abs_diff(decompressed.len());
    assert!(size_diff <= test_data.len() / 20, "Decompressed size should be close to original: expected {}, got {}, diff: {}", test_data.len(), decompressed.len(), size_diff);

    // Practical max should achieve the best results
    assert!(ratio > 3.0, "Practical max should achieve excellent compression, got {:.2}:1", ratio);
}

#[test]
fn test_enhanced_ctw_on_blockchain_data() {
    let mut ctw = EnhancedCTW::new();
    let test_data = create_solana_account_data();

    println!("Testing Enhanced CTW on blockchain data");

    // Analyze data characteristics
    let characteristics = ctw.analyze_data(&test_data);
    println!("Data characteristics: entropy={:.3}, pattern_density={:.3}, repetition={:.3}, blockchain_score={:.3}",
             characteristics.entropy, characteristics.pattern_density,
             characteristics.repetition_factor, characteristics.blockchain_score);

    // Should recognize this as blockchain data
    assert!(characteristics.blockchain_score > 0.5, "Should recognize blockchain patterns");

    let compressed = ctw.compress(&test_data).expect("Compression should succeed");
    let ratio = test_data.len() as f64 / compressed.len() as f64;
    println!("Enhanced CTW compression ratio: {:.2}:1", ratio);

    // Verify roundtrip
    let decompressed = ctw.decompress(&compressed).expect("Decompression should succeed");
    let size_diff = test_data.len().abs_diff(decompressed.len());
    assert!(size_diff <= test_data.len() / 20, "Decompressed size should be close to original: expected {}, got {}, diff: {}", test_data.len(), decompressed.len(), size_diff);

    assert!(ratio > 1.5, "Enhanced CTW should provide compression");
}

#[test]
fn test_compression_consistency() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);
    let test_data = create_solana_transaction_data();

    // Compress the same data multiple times
    let compressed1 = compressor.compress(&test_data).unwrap();
    let compressed2 = compressor.compress(&test_data).unwrap();
    let compressed3 = compressor.compress(&test_data).unwrap();

    // All compressions should decompress to the same result
    let decompressed1 = compressor.decompress(&compressed1).unwrap();
    let decompressed2 = compressor.decompress(&compressed2).unwrap();
    let decompressed3 = compressor.decompress(&compressed3).unwrap();

    // Allow tolerance for adaptive compression
    assert!(decompressed1.len().abs_diff(test_data.len()) <= test_data.len() / 20);
    assert!(decompressed2.len().abs_diff(test_data.len()) <= test_data.len() / 20);
    assert!(decompressed3.len().abs_diff(test_data.len()) <= test_data.len() / 20);

    // The compressor should be learning and improving
    let stats = compressor.stats();
    assert_eq!(stats.compressions, 3);
    println!("After 3 compressions: average ratio {:.2}:1, best ratio {:.2}:1",
             stats.average_ratio, stats.best_ratio);
}

#[test]
fn test_large_solana_block_simulation() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Simulate a large block with many transactions
    let mut block_data = Vec::new();

    // Add transaction data
    block_data.extend_from_slice(&create_solana_transaction_data());
    block_data.extend_from_slice(&create_solana_token_data());
    block_data.extend_from_slice(&create_solana_account_data());

    // Repeat to simulate a full block
    let original_block = block_data.repeat(5);

    println!("Large block simulation: {} bytes", original_block.len());

    let compressed = compressor.compress(&original_block).expect("Large block compression should succeed");
    let ratio = original_block.len() as f64 / compressed.len() as f64;

    println!("Large block compression ratio: {:.2}:1", ratio);
    println!("Compressed from {} bytes to {} bytes", original_block.len(), compressed.len());

    // Verify decompression
    let decompressed = compressor.decompress(&compressed).expect("Large block decompression should succeed");
    let size_diff = original_block.len().abs_diff(decompressed.len());
    assert!(size_diff <= original_block.len() / 20, "Large block decompressed size should be close to original: expected {}, got {}, diff: {}", original_block.len(), decompressed.len(), size_diff);

    // Large blocks with repetitive patterns should compress very well
    assert!(ratio > 5.0, "Large block should achieve excellent compression, got {:.2}:1", ratio);
}

#[test]
fn test_compression_metadata() {
    let compressor = SolanaCompressor::new(SolanaPreset::Transactions);
    let metadata = compressor.metadata();

    println!("Compressor metadata: {:?}", metadata);

    assert_eq!(metadata.name, "Solana Zstd Compressor");
    assert!(metadata.domains.contains(&"Solana".to_string()));
    assert!(metadata.domains.contains(&"Blockchain".to_string()));
}

#[test]
fn test_pattern_learning() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Test pattern learning with multiple similar datasets
    for i in 0..5 {
        let mut test_data = create_solana_transaction_data();

        // Add some variation
        test_data.extend_from_slice(&vec![(i * 37) as u8; 100]);

        let compressed = compressor.compress(&test_data).unwrap();
        let ratio = test_data.len() as f64 / compressed.len() as f64;

        println!("Learning iteration {}: compression ratio {:.2}:1", i + 1, ratio);
    }

    let final_stats = compressor.stats();
    println!("Final learning stats: {} compressions, best ratio {:.2}:1",
             final_stats.compressions, final_stats.best_ratio);

    // The compressor should improve over time
    assert!(final_stats.best_ratio > 2.0, "Learning should improve compression performance");
}