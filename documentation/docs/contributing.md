# Contributing

## Development Setup

```bash
git clone https://github.com/cryptuon/blockchain-compression.git
cd blockchain-compression
cargo build --features zstd
cargo test --features zstd
```

## Running Tests

```bash
# All tests
cargo test --features zstd

# Integration tests only
cargo test --features zstd --test solana_integration_tests

# With output
cargo test --features zstd -- --nocapture
```

## Running Examples

```bash
cargo run --example basic_usage --features zstd
cargo run --example performance_benchmark --features zstd
```

## Running Benchmarks

```bash
cargo bench --features zstd
```

## Code Style

- Follow standard Rust conventions (`rustfmt`, `clippy`)
- Run `cargo clippy --features zstd` before submitting

## Pull Request Process

1. Fork the repository
2. Create a feature branch from `main`
3. Add tests for new functionality
4. Ensure all tests pass with `cargo test --features zstd`
5. Run `cargo clippy --features zstd` and address warnings
6. Submit a pull request with a clear description of the change

## Documentation

If you change public API, update:

- Rustdoc comments on the affected items
- `docs/API.md` if the change affects documented types or traits
- The mkdocs pages in `documentation/docs/` if user-facing behavior changes

## Building Documentation

```bash
# Rustdoc
cargo doc --features zstd --open

# mkdocs (requires mkdocs-material)
pip install mkdocs-material
cd documentation
mkdocs serve
```

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
