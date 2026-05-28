# Testing Guide

Comprehensive testing documentation for the peaq DePIN Simulator.

## Test Structure

The project includes multiple levels of testing:

### 1. Unit Tests

Located in each module file (`src/*.rs`), testing individual components in isolation.

```bash
# Run all unit tests
cargo test --lib

# Run specific module tests
cargo test crypto::tests
cargo test did::tests
cargo test telemetry::tests
```

### 2. Integration Tests

Network-dependent tests that verify end-to-end functionality.

```bash
# Run integration tests (requires network)
cargo test --test '*' -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored
```

### 3. Example Programs

Demonstrative examples showing how to use the library.

```bash
# Run the simple simulator example
cargo run --example simple_simulator
```

## Test Coverage

### Crypto Module (`src/crypto.rs`)

**Tests:**
- `test_keypair_generation`: Verifies random keypair generation
- `test_keypair_from_seed`: Tests deterministic keypair creation
- `test_sign_and_verify`: Validates signature creation and verification

**Coverage:**
- ✅ Keypair generation (random and from seed)
- ✅ Public key extraction and formatting
- ✅ Data signing
- ✅ Signature verification
- ✅ Invalid signature detection

### DID Module (`src/did.rs`)

**Tests:**
- `test_did_creation`: Validates DID creation from public key
- `test_did_invalid_pubkey`: Tests error handling for invalid input
- `test_did_document`: Verifies W3C-compliant document generation
- `test_extract_public_key`: Tests public key extraction from DID

**Coverage:**
- ✅ DID creation
- ✅ Format validation
- ✅ DID document generation
- ✅ Public key extraction
- ✅ Error handling

### Telemetry Module (`src/telemetry.rs`)

**Tests:**
- `test_telemetry_generation`: Validates data generation
- `test_packet_signing`: Tests packet signing workflow
- `test_packet_serialization`: Verifies JSON serialization

**Coverage:**
- ✅ Random telemetry generation
- ✅ All sensor types (energy, location, environmental, vehicle)
- ✅ Packet signing
- ✅ Signature verification
- ✅ JSON serialization/deserialization

### Client Module (`src/client.rs`)

**Tests:**
- `test_connect_to_peaq`: Network connection test (ignored by default)

**Coverage:**
- ✅ Network connection
- ⚠️  Telemetry submission (requires metadata)
- ⚠️  DID registration (requires metadata)

## Running Tests

### Quick Test

```bash
# Run all tests (excluding network tests)
cargo test
```

### Verbose Output

```bash
# Show test output
cargo test -- --nocapture

# Show test names
cargo test -- --list
```

### Specific Tests

```bash
# Run a single test
cargo test test_keypair_generation

# Run tests matching a pattern
cargo test crypto

# Run tests in a specific module
cargo test --lib crypto::tests
```

### Test with Different Configurations

```bash
# Debug build (faster compilation)
cargo test

# Release build (faster execution)
cargo test --release

# With specific features
cargo test --features "feature-name"
```

## Test Results

Current test status:

```
running 11 tests
test client::tests::test_connect_to_peaq ... ignored
test crypto::tests::test_keypair_from_seed ... ok
test crypto::tests::test_keypair_generation ... ok
test crypto::tests::test_sign_and_verify ... ok
test did::tests::test_did_creation ... ok
test did::tests::test_did_document ... ok
test did::tests::test_did_invalid_pubkey ... ok
test did::tests::test_extract_public_key ... ok
test telemetry::tests::test_packet_serialization ... ok
test telemetry::tests::test_packet_signing ... ok
test telemetry::tests::test_telemetry_generation ... ok

test result: ok. 10 passed; 0 failed; 1 ignored
```

## Code Quality Checks

### Format Check

```bash
# Check formatting
cargo fmt -- --check

# Auto-format code
cargo fmt
```

### Linting

```bash
# Run clippy
cargo clippy

# Clippy with all warnings as errors
cargo clippy -- -D warnings
```

### Build Check

```bash
# Check compilation without building
cargo check

# Check all targets
cargo check --all-targets
```

## Continuous Integration

Recommended CI pipeline:

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --verbose
      - name: Check formatting
        run: cargo fmt -- --check
      - name: Run clippy
        run: cargo clippy -- -D warnings
```

## Benchmarking

For performance testing:

```bash
# Install criterion
cargo install cargo-criterion

# Run benchmarks (if implemented)
cargo criterion
```

## Test Data

### Sample Keypairs

For testing, you can use these deterministic seeds:

```rust
// Alice (standard Substrate test account)
let keypair = DeviceKeypair::from_seed("//Alice")?;

// Bob
let keypair = DeviceKeypair::from_seed("//Bob")?;

// Custom seed
let keypair = DeviceKeypair::from_seed("my custom seed phrase")?;
```

### Sample DIDs

```
did:peaq:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
```

### Sample Telemetry

```json
{
  "device_id": "test-device",
  "timestamp": 1234567890,
  "data": {
    "type": "Energy",
    "consumption_kwh": 24.5,
    "voltage": 230.2,
    "current_amps": 12.3
  },
  "signature": "0x..."
}
```

## Debugging Tests

### Enable Logging

```bash
# Set log level
RUST_LOG=debug cargo test -- --nocapture

# Module-specific logging
RUST_LOG=peaq_depin_simulator::crypto=trace cargo test
```

### Run Single Test with Backtrace

```bash
RUST_BACKTRACE=1 cargo test test_name -- --nocapture
```

### Use Test Debugger

```bash
# With rust-gdb
rust-gdb --args target/debug/deps/peaq_depin_simulator-* test_name

# With rust-lldb
rust-lldb target/debug/deps/peaq_depin_simulator-* -- test_name
```

## Test Best Practices

### Writing New Tests

1. **Arrange-Act-Assert Pattern**
```rust
#[test]
fn test_example() {
    // Arrange
    let input = setup_test_data();
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert_eq!(result, expected);
}
```

2. **Use Descriptive Names**
```rust
#[test]
fn test_keypair_generation_produces_valid_public_key() {
    // ...
}
```

3. **Test Error Cases**
```rust
#[test]
fn test_invalid_seed_returns_error() {
    let result = DeviceKeypair::from_seed("invalid");
    assert!(result.is_err());
}
```

4. **Use Test Fixtures**
```rust
fn create_test_keypair() -> DeviceKeypair {
    DeviceKeypair::from_seed("//Alice").unwrap()
}

#[test]
fn test_with_fixture() {
    let keypair = create_test_keypair();
    // ...
}
```

## Coverage Reports

Generate coverage reports:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin --out Html

# Generate lcov format
cargo tarpaulin --out Lcov

# Open report
open tarpaulin-report.html
```

## Performance Testing

### Measure Test Execution Time

```bash
# Time all tests
time cargo test

# Time specific test
time cargo test test_name
```

### Profile Tests

```bash
# Install flamegraph
cargo install flamegraph

# Profile tests
cargo flamegraph --test test_name
```

## Troubleshooting

### Tests Hang

- Check for infinite loops
- Verify async operations complete
- Use `--test-threads=1` to run serially

### Tests Fail Intermittently

- Check for race conditions
- Verify test isolation
- Use deterministic seeds for randomness

### Network Tests Fail

- Verify network connectivity
- Check RPC endpoint availability
- Ensure firewall allows connections

## Next Steps

1. Add more integration tests
2. Implement property-based testing with `proptest`
3. Add mutation testing with `cargo-mutants`
4. Set up continuous benchmarking
5. Increase code coverage to >90%

---

For more information, see:
- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
