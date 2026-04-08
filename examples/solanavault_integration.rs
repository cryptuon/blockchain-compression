//! SolanaVault Integration Example
//!
//! This example demonstrates how to use the blockchain-compression library
//! within the SolanaVault ecosystem using the BlockchainCompressionAdapter.

// Note: This example shows the API but won't compile standalone since it requires
// the vault-core crate. It's provided for documentation purposes.

#[cfg(feature = "vault-integration")]
mod vault_integration {
    use vault_core::compression::{
        BlockchainCompressionAdapter,
        CompressionStrategy,
        CompressionVersion,
        create_block_compressor,
        create_account_compressor,
        create_general_compressor,
        create_archival_compressor,
    };

    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        println!("🏦 SolanaVault Integration Example");
        println!("==================================\n");

        // Example 1: Using pre-configured adapters
        using_preconfigured_adapters()?;

        // Example 2: Custom adapter configuration
        custom_adapter_configuration()?;

        // Example 3: Adapter statistics and metadata
        adapter_statistics_example()?;

        Ok(())
    }

    fn using_preconfigured_adapters() -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Example 1: Pre-configured Adapters");
        println!("-------------------------------------");

        // Create adapters for different use cases
        let block_compressor = create_block_compressor();
        let account_compressor = create_account_compressor();
        let general_compressor = create_general_compressor();
        let archival_compressor = create_archival_compressor();

        let test_data = b"Sample blockchain data for compression testing".repeat(20);

        println!("Original data size: {} bytes\n", test_data.len());

        // Test each adapter
        let adapters = vec![
            ("Block Compressor", &block_compressor),
            ("Account Compressor", &account_compressor),
            ("General Compressor", &general_compressor),
            ("Archival Compressor", &archival_compressor),
        ];

        for (name, adapter) in adapters {
            let compressed = adapter.compress(&test_data)?;
            let decompressed = adapter.decompress(&compressed)?;

            // Verify integrity
            assert_eq!(test_data.as_slice(), decompressed.as_slice());

            let ratio = test_data.len() as f64 / compressed.len() as f64;
            println!("{:18}: {} bytes ({:.2}:1)", name, compressed.len(), ratio);
        }
        println!();

        Ok(())
    }

    fn custom_adapter_configuration() -> Result<(), Box<dyn std::error::Error>> {
        println!("⚙️  Example 2: Custom Adapter Configuration");
        println!("--------------------------------------------");

        // Create adapters with specific presets
        let transaction_adapter = BlockchainCompressionAdapter::for_transactions();
        let account_adapter = BlockchainCompressionAdapter::for_accounts();
        let mixed_adapter = BlockchainCompressionAdapter::for_mixed_data();
        let archival_adapter = BlockchainCompressionAdapter::for_archival();

        let blockchain_data = create_realistic_blockchain_data();

        println!("Blockchain data size: {} bytes", blockchain_data.len());
        println!("Testing different adapter configurations:\n");

        let configurations = vec![
            ("Transactions", &transaction_adapter),
            ("Accounts", &account_adapter),
            ("Mixed Data", &mixed_adapter),
            ("Archival", &archival_adapter),
        ];

        for (config_name, adapter) in configurations {
            let compressed = adapter.compress(&blockchain_data)?;
            let decompressed = adapter.decompress(&compressed)?;

            assert_eq!(blockchain_data, decompressed);

            let ratio = blockchain_data.len() as f64 / compressed.len() as f64;
            let version = adapter.version();

            println!("{:12}: {} bytes ({:5.2}:1) [Version: {:?}]",
                     config_name, compressed.len(), ratio, version);
        }
        println!();

        Ok(())
    }

    fn adapter_statistics_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Example 3: Adapter Statistics and Metadata");
        println!("---------------------------------------------");

        let mut adapter = BlockchainCompressionAdapter::for_transactions();

        // Compress multiple datasets
        let datasets = vec![
            create_transaction_data(),
            create_program_data(),
            create_mixed_blockchain_data(),
        ];

        for (i, data) in datasets.iter().enumerate() {
            let compressed = adapter.compress(data)?;
            let ratio = data.len() as f64 / compressed.len() as f64;
            println!("Dataset {}: {:.2}:1 compression ratio", i + 1, ratio);
        }

        // Get compression statistics
        let stats = adapter.get_stats()?;
        println!("\nCompression Statistics:");
        println!("- Total compressions: {}", stats.compressions);
        println!("- Total input bytes: {}", stats.total_input_bytes);
        println!("- Total output bytes: {}", stats.total_output_bytes);
        println!("- Average ratio: {:.2}:1", stats.average_ratio);
        println!("- Best ratio: {:.2}:1", stats.best_ratio);

        // Get adapter metadata
        let metadata = adapter.get_metadata()?;
        println!("\nAdapter Metadata:");
        println!("- Algorithm: {}", metadata.name);
        println!("- Version: {}", metadata.version);
        println!("- Description: {}", metadata.description);
        println!("- Deterministic: {}", metadata.deterministic);
        println!("- Memory usage: ~{} bytes", metadata.memory_usage);

        // Reset adapter state
        adapter.reset()?;
        println!("\n✅ Adapter state reset successfully");

        Ok(())
    }

    // Helper functions to create test data

    fn create_realistic_blockchain_data() -> Vec<u8> {
        let mut data = Vec::new();

        // Add Solana program IDs
        for _ in 0..10 {
            data.extend_from_slice("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes());
            data.extend_from_slice("11111111111111111111111111111112".as_bytes());
        }

        // Add transaction amounts
        for _ in 0..5 {
            data.extend_from_slice(&1_000_000_000u64.to_le_bytes()); // 1 SOL
            data.extend_from_slice(&100_000_000u64.to_le_bytes());   // 0.1 SOL
        }

        // Add instruction patterns
        for _ in 0..8 {
            data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Transfer
            data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // Initialize
        }

        data
    }

    fn create_transaction_data() -> Vec<u8> {
        let mut data = Vec::new();
        for _ in 0..15 {
            data.extend_from_slice("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes());
            data.extend_from_slice(&1_000_000_000u64.to_le_bytes());
        }
        data
    }

    fn create_program_data() -> Vec<u8> {
        let mut data = Vec::new();
        let programs = [
            "11111111111111111111111111111112",
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
            "BPFLoaderUpgradeab1e11111111111111111111111",
        ];

        for _ in 0..20 {
            for program in &programs {
                data.extend_from_slice(program.as_bytes());
            }
        }
        data
    }

    fn create_mixed_blockchain_data() -> Vec<u8> {
        let mut data = Vec::new();
        for _ in 0..10 {
            data.extend_from_slice("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes());
            data.extend_from_slice(&[0x00, 0x01, 0x02, 0x03]);
            data.extend_from_slice(&500_000_000u64.to_le_bytes());
        }
        data
    }
}

// Standalone example that demonstrates the same concepts without vault-core dependency
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Blockchain Compression - SolanaVault Integration Concepts");
    println!("============================================================\n");

    // This demonstrates the same compression patterns that would be used
    // in SolanaVault, but using the blockchain-compression library directly

    use blockchain_compression::presets::solana::{SolanaCompressor, SolanaPreset};
    use blockchain_compression::core::traits::CompressionStrategy;

    println!("This example shows how blockchain-compression integrates with SolanaVault.");
    println!("The actual integration uses the BlockchainCompressionAdapter wrapper.\n");

    // Demonstrate equivalent functionality
    let presets = vec![
        ("Transactions (for blocks)", SolanaPreset::Transactions),
        ("Accounts (for state)", SolanaPreset::Accounts),
        ("Mixed (general purpose)", SolanaPreset::Mixed),
        ("Max Compression (archival)", SolanaPreset::MaxCompression),
    ];

    let test_data = create_sample_solana_data();
    println!("Sample Solana data size: {} bytes\n", test_data.len());

    for (description, preset) in presets {
        let mut compressor = SolanaCompressor::new(preset);

        let compressed = compressor.compress(&test_data)?;
        let decompressed = compressor.decompress(&compressed)?;

        assert_eq!(test_data, decompressed);

        let ratio = test_data.len() as f64 / compressed.len() as f64;
        println!("{:30}: {:.2}:1 ratio ({} bytes)", description, ratio, compressed.len());
    }

    println!("\n✅ All compression operations completed with perfect data integrity!");
    println!("\n💡 In SolanaVault, these same operations are wrapped in the");
    println!("   BlockchainCompressionAdapter for seamless integration.");

    Ok(())
}

fn create_sample_solana_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Simulate real Solana transaction patterns
    for _ in 0..20 {
        // Program IDs (highly compressible with dictionary)
        data.extend_from_slice("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".as_bytes());
        data.extend_from_slice("11111111111111111111111111111112".as_bytes());

        // Common amounts
        data.extend_from_slice(&1_000_000_000u64.to_le_bytes()); // 1 SOL

        // Common instruction patterns
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Transfer instruction
    }

    data
}