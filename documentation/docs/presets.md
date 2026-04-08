# Presets

Presets configure the Zstandard compression level and dictionary strategy for specific types of blockchain data. Choose the preset that matches your workload.

## Preset Overview

| Preset | Zstd Level | Ratio | Speed | Best For |
|--------|-----------|-------|-------|----------|
| `FastCompression` | 3 | 5--15:1 | Fastest | Real-time pipelines |
| `Transactions` | 3 | 10--30:1 | Fast | Transaction records |
| `Instructions` | 6 | 10--25:1 | Balanced | Program instruction data |
| `Accounts` | 6 | 15--40:1 | Balanced | Account state snapshots |
| `Mixed` | 6 | 12--35:1 | Balanced | Unknown or mixed data |
| `MaxCompression` | 19 | 20--60:1 | Slowest | Archival, cold storage |

## FastCompression

```rust
SolanaCompressor::new(SolanaPreset::FastCompression)
```

Zstd level 3. Prioritizes throughput over compression ratio. Use this when you need to compress data in real-time (e.g., streaming transaction ingestion) and can tolerate larger compressed output.

## Transactions

```rust
SolanaCompressor::new(SolanaPreset::Transactions)
```

Zstd level 3. Tuned for transaction data which typically contains repeated signatures, program IDs, and account addresses. The custom dictionary includes common Solana program IDs (System Program, Token Program, Associated Token Program) and transaction structure markers.

## Instructions

```rust
SolanaCompressor::new(SolanaPreset::Instructions)
```

Zstd level 6. Optimized for program instruction data -- instruction discriminators, program IDs, and account references. Slightly higher compression level for better ratios on instruction-heavy workloads.

## Accounts

```rust
SolanaCompressor::new(SolanaPreset::Accounts)
```

Zstd level 6. Designed for account state data -- account addresses, balances, owner program IDs, and serialized account data. Works well with snapshots where many accounts share common owners and data layouts.

## Mixed

```rust
SolanaCompressor::new(SolanaPreset::Mixed)
```

Zstd level 6. A balanced preset for workloads that contain a mix of transactions, accounts, and instructions. Use this as a default if you're unsure which preset fits your data.

## MaxCompression

```rust
SolanaCompressor::new(SolanaPreset::MaxCompression)
```

Zstd level 19. Maximum compression for archival storage. Significantly slower than other presets but achieves the highest compression ratios. Use this for data that is written once and read infrequently.

## Custom Dictionary

All presets use a shared custom dictionary built from common Solana patterns:

- **Program IDs**: System Program, Token Program, Associated Token, Serum DEX, BPF Loader, Config, Vote, Stake
- **Instruction markers**: Transfer, Initialize, Close, Approve
- **Common amounts**: 0.01, 0.1, 1, 10 SOL (in lamports)
- **Transaction structure markers**: Signature count headers
- **Base58 character set**: Common in Solana address encoding

The dictionary is automatically constructed when you create a compressor -- no manual configuration needed.
