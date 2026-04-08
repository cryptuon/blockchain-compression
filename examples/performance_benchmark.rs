//! Performance benchmark example for blockchain-compression library
//!
//! This example demonstrates:
//! - Performance characteristics of different presets
//! - Throughput measurements
//! - Memory usage analysis
//! - Compression ratio vs speed trade-offs

use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
use blockchain_compression::core::traits::CompressionStrategy;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Blockchain Compression Library - Performance Benchmark");
    println!("========================================================\n");

    // Run various performance tests
    throughput_benchmark()?;
    size_scaling_benchmark()?;
    real_world_data_benchmark()?;

    Ok(())
}

fn throughput_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Throughput Benchmark");
    println!("----------------------");

    let test_data = create_blockchain_test_data(10_000); // 10KB test data
    println!("Test data size: {} bytes\n", test_data.len());

    let presets = vec![
        ("Fast", SolanaPreset::FastCompression),
        ("Transactions", SolanaPreset::Transactions),
        ("Accounts", SolanaPreset::Accounts),
        ("Max Compression", SolanaPreset::MaxCompression),
    ];

    println!("{:15} | {:>8} | {:>12} | {:>12} | {:>8}",
             "Preset", "Ratio", "Compress", "Decompress", "Size");
    println!("{:-<15}-|-{:-<8}-|-{:-<12}-|-{:-<12}-|-{:-<8}", "", "", "", "", "");

    for (name, preset) in presets {
        let mut compressor = SolanaCompressor::new(preset);

        // Compression benchmark
        let start = Instant::now();
        let compressed = compressor.compress(&test_data)?;
        let compress_time = start.elapsed();

        // Decompression benchmark
        let start = Instant::now();
        let decompressed = compressor.decompress(&compressed)?;
        let decompress_time = start.elapsed();

        // Verify integrity
        assert_eq!(test_data, decompressed);

        let ratio = test_data.len() as f64 / compressed.len() as f64;
        let compress_throughput = test_data.len() as f64 / compress_time.as_secs_f64() / 1_024_1024.0; // MB/s
        let decompress_throughput = test_data.len() as f64 / decompress_time.as_secs_f64() / 1_024_1024.0; // MB/s

        println!("{:15} | {:>6.1}:1 | {:>9.1} MB/s | {:>9.1} MB/s | {:>6} B",
                 name, ratio, compress_throughput, decompress_throughput, compressed.len());
    }
    println!();

    Ok(())
}

fn size_scaling_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("📏 Size Scaling Benchmark");
    println!("-------------------------");

    let sizes = vec![1_000, 10_000, 100_000, 1_000_000]; // 1KB to 1MB
    let preset = SolanaPreset::Transactions;

    println!("{:>10} | {:>8} | {:>12} | {:>10}", "Input Size", "Ratio", "Throughput", "Output");
    println!("{:->10}-|-{:->8}-|-{:->12}-|-{:->10}", "", "", "", "");

    for size in sizes {
        let test_data = create_blockchain_test_data(size);
        let mut compressor = SolanaCompressor::new(preset.clone());

        let start = Instant::now();
        let compressed = compressor.compress(&test_data)?;
        let duration = start.elapsed();

        let decompressed = compressor.decompress(&compressed)?;
        assert_eq!(test_data, decompressed);

        let ratio = test_data.len() as f64 / compressed.len() as f64;
        let throughput = test_data.len() as f64 / duration.as_secs_f64() / 1_024_1024.0; // MB/s

        println!("{:>8} B | {:>6.1}:1 | {:>9.1} MB/s | {:>8} B",
                 size, ratio, throughput, compressed.len());
    }
    println!();

    Ok(())
}

fn real_world_data_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Real-World Data Patterns");
    println!("---------------------------");

    let test_cases = vec![
        ("Pure Solana Programs", create_solana_program_data()),
        ("Mixed Transaction Data", create_mixed_transaction_data()),
        ("Account State Data", create_account_state_data()),
        ("Random Data (worst case)", create_random_data(10_000)),
    ];

    println!("{:20} | {:>8} | {:>10} | {:>8}", "Data Type", "Ratio", "Input", "Output");
    println!("{:-<20}-|-{:-<8}-|-{:-<10}-|-{:-<8}", "", "", "", "");

    for (name, test_data) in test_cases {
        let mut compressor = SolanaCompressor::new(SolanaPreset::Mixed);

        let compressed = compressor.compress(&test_data)?;
        let decompressed = compressor.decompress(&compressed)?;
        assert_eq!(test_data, decompressed);

        let ratio = test_data.len() as f64 / compressed.len() as f64;

        println!("{:20} | {:>6.1}:1 | {:>8} B | {:>6} B",
                 name, ratio, test_data.len(), compressed.len());
    }
    println!();

    Ok(())
}

// Helper functions to create test data

fn create_blockchain_test_data(size: usize) -> Vec<u8> {
    let mut data = Vec::new();
    let patterns = [
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes(),
        "11111111111111111111111111111112".as_bytes(),
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".as_bytes(),
        &1_000_000_000u64.to_le_bytes(),
        &[0x00, 0x00, 0x00, 0x00], // Common instruction
    ];

    while data.len() < size {
        for pattern in &patterns {
            data.extend_from_slice(pattern);
            if data.len() >= size {
                break;
            }
        }
    }

    data.truncate(size);
    data
}

fn create_solana_program_data() -> Vec<u8> {
    let mut data = Vec::new();
    let programs = [
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "11111111111111111111111111111112",
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
        "BPFLoaderUpgradeab1e11111111111111111111111",
    ];

    for _ in 0..50 {
        for program in &programs {
            data.extend_from_slice(program.as_bytes());
        }
    }
    data
}

fn create_mixed_transaction_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Mix of program IDs, amounts, and instruction data
    for _ in 0..20 {
        data.extend_from_slice("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes());
        data.extend_from_slice(&1_000_000_000u64.to_le_bytes());
        data.extend_from_slice(&[0x00, 0x01, 0x02, 0x03]);
        data.extend_from_slice("11111111111111111111111111111112".as_bytes());
        data.extend_from_slice(&100_000_000u64.to_le_bytes());
    }
    data
}

fn create_account_state_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Simulate account state with repeated structures
    for _ in 0..30 {
        data.extend_from_slice(&[0x01]); // Account discriminator
        data.extend_from_slice("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".as_bytes()); // Owner
        data.extend_from_slice(&1_000_000u64.to_le_bytes()); // Amount
        data.extend_from_slice(&[0x00; 32]); // Padding
    }
    data
}

fn create_random_data(size: usize) -> Vec<u8> {
    // Create pseudo-random data (worst case for compression)
    (0..size).map(|i| ((i * 7 + 13) % 256) as u8).collect()
}