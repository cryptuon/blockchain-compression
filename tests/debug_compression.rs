//! Debug compression roundtrip issues

use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;

#[test]
fn test_pattern_reconstruction() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Test several 64-byte patterns used in failing tests
    for byte_val in [0x00, 0x01, 0x02, 0x0A, 0x0B, 0x0E] {
        let test_data = vec![byte_val; 64];

        let compressed = compressor.compress(&test_data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        let success = decompressed.len() == test_data.len() &&
                     decompressed.iter().all(|&b| b == byte_val);
        println!("64-byte 0x{:02X}: {}", byte_val, if success { "✅ PASS" } else { "❌ FAIL" });
        if !success {
            println!("  Expected: len={}, all 0x{:02X}", test_data.len(), byte_val);
            println!("  Got: len={}, first_few={:02X?}", decompressed.len(), &decompressed[0..decompressed.len().min(5)]);
        }
        assert!(success, "Pattern reconstruction failed for 0x{:02X}", byte_val);
    }

    // Test several 32-byte patterns
    for byte_val in [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06] {
        let test_data = vec![byte_val; 32];

        let compressed = compressor.compress(&test_data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        let success = decompressed.len() == test_data.len() &&
                     decompressed.iter().all(|&b| b == byte_val);
        println!("32-byte 0x{:02X}: {}", byte_val, if success { "✅ PASS" } else { "❌ FAIL" });
        if !success {
            println!("  Expected: len={}, all 0x{:02X}", test_data.len(), byte_val);
            println!("  Got: len={}, first_few={:02X?}", decompressed.len(), &decompressed[0..decompressed.len().min(5)]);
        }
        assert!(success, "Pattern reconstruction failed for 0x{:02X}", byte_val);
    }
}

#[test]
fn test_literal_data_handling() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Test data with mixed patterns and literal data (like the failing tests)
    let mut test_data = Vec::new();

    // Add a 64-byte signature pattern
    test_data.extend_from_slice(&[0x01; 64]);

    // Add literal data (Token program ID from failing test)
    test_data.extend_from_slice(&[0x06, 0xdd, 0xf6, 0xe1, 0xd7, 0x65, 0xa1, 0x93,
                                0xd9, 0xcb, 0xe1, 0x46, 0xce, 0xeb, 0x79, 0xac,
                                0x1c, 0xb4, 0x85, 0xed, 0x5f, 0x5b, 0x37, 0x91,
                                0x3a, 0x8c, 0xf5, 0x85, 0x7e, 0xff, 0x00, 0xa9]);

    // Add another 32-byte pattern
    test_data.extend_from_slice(&[0x02; 32]);

    // Add some more literal data
    test_data.extend_from_slice(&[0x03, 0x04, 0x05, 0x06]);

    println!("Original mixed data: {} bytes", test_data.len());

    let compressed = compressor.compress(&test_data).expect("Compression should work");
    println!("Compressed: {} bytes", compressed.len());

    // Debug compressed data
    println!("Compressed bytes: {:02X?}", &compressed[0..compressed.len().min(20)]);

    let decompressed = compressor.decompress(&compressed).expect("Decompression should work");
    println!("Decompressed: {} bytes", decompressed.len());

    if test_data.len() != decompressed.len() {
        println!("❌ Size mismatch: expected {}, got {}", test_data.len(), decompressed.len());
    }

    if test_data != decompressed {
        println!("❌ Content mismatch");
        // Show differences
        for i in 0..test_data.len().min(decompressed.len()) {
            if test_data[i] != decompressed[i] {
                println!("  First diff at byte {}: expected 0x{:02X}, got 0x{:02X}", i, test_data[i], decompressed[i]);
                break;
            }
        }
    } else {
        println!("✅ Perfect match!");
    }

    assert_eq!(test_data.len(), decompressed.len(), "Size should match");
    assert_eq!(test_data, decompressed, "Content should match");
}

#[test]
fn debug_simple_roundtrip() {
    let mut compressor = SolanaCompressor::new(SolanaPreset::Transactions);

    // Create simple test data with clear patterns
    let mut test_data = Vec::new();

    // Add a clear 64-byte signature pattern
    test_data.extend_from_slice(&[0xAA; 64]);

    // Add a clear 32-byte account pattern
    test_data.extend_from_slice(&[0xBB; 32]);

    // Add some literal data
    test_data.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]);

    println!("Original data: {} bytes", test_data.len());
    println!("First 10 bytes: {:02X?}", &test_data[0..10]);

    // Compress
    let compressed = compressor.compress(&test_data).expect("Compression should work");
    println!("Compressed data: {} bytes", compressed.len());
    println!("Compressed bytes: {:02X?}", compressed);


    // Decompress
    let decompressed = compressor.decompress(&compressed).expect("Decompression should work");
    println!("Decompressed data: {} bytes", decompressed.len());
    if decompressed.len() >= 10 {
        println!("First 10 bytes: {:02X?}", &decompressed[0..10]);
    } else {
        println!("All bytes: {:02X?}", decompressed);
    }

    // Check if they match
    if test_data.len() == decompressed.len() {
        println!("✅ Size matches!");
        if test_data == decompressed {
            println!("✅ Content matches!");
        } else {
            println!("❌ Content differs");
            for i in 0..test_data.len().min(decompressed.len()) {
                if test_data[i] != decompressed[i] {
                    println!("First difference at byte {}: expected {:02X}, got {:02X}", i, test_data[i], decompressed[i]);
                    break;
                }
            }
        }
    } else {
        println!("❌ Size differs: {} vs {}", test_data.len(), decompressed.len());
    }
}