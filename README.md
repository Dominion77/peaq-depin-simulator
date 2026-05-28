# peaq DePIN Node Simulator

A production-grade headless DePIN (Decentralized Physical Infrastructure Network) node simulator for the peaq network, written in Rust.

## Overview

This simulator generates a peaq DID (Decentralized Identity), creates cryptographic keypairs, and simulates IoT data streams (energy consumption, GPS coordinates, environmental sensors, vehicle telemetry). Each data packet is cryptographically signed and submitted to the peaq network via Substrate extrinsics.

## Features

- **Cryptographic Identity Management**: SR25519 keypair generation and management
- **DID Support**: W3C-compliant Decentralized Identifiers
- **Multi-Sensor Simulation**: Energy, location, environmental, and vehicle telemetry
- **Cryptographic Signing**: All data packets are signed and verifiable
- **Substrate Integration**: Direct interaction with peaq network via subxt
- **Async Architecture**: Built on Tokio for high-performance async I/O
- **Comprehensive Testing**: Unit tests for all core modules
- **Production-Ready**: Proper error handling, logging, and configuration

## Architecture

### Module Structure

```
src/
├── main.rs           # Application entry point and main loop
├── lib.rs            # Library exports
├── config.rs         # Configuration management
├── crypto.rs         # Cryptographic keypair operations
├── did.rs            # Decentralized Identity management
├── telemetry.rs      # IoT data generation and signing
├── client.rs         # Peaq network client wrapper
└── error.rs          # Error types and handling
```

### Data Flow

```
┌─────────────────┐
│  Device Keypair │
│   (SR25519)     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   DID Creation  │
│ did:peaq:0x...  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐      ┌──────────────┐
│   Telemetry     │─────▶│  Signature   │
│   Generator     │      │  (Ed25519)   │
└────────┬────────┘      └──────┬───────┘
         │                      │
         └──────────┬───────────┘
                    ▼
         ┌─────────────────────┐
         │  Peaq Network       │
         │  (Substrate RPC)    │
         └─────────────────────┘
```

## Prerequisites

- Rust 1.70+ (2021 edition)
- Access to peaq network (testnet or mainnet)
- Basic understanding of Substrate and Polkadot ecosystem

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd peaq-depin-simulator

# Build the project
cargo build --release

# Run tests
cargo test
```

## Configuration

The simulator can be configured via environment variables:

```bash
# Peaq network RPC endpoint
export PEAQ_RPC_URL="wss://wsspc-akash-agung.peaq.network"

# Telemetry submission interval (seconds)
export TELEMETRY_INTERVAL=10

# Device identifier
export DEVICE_ID="sim-001"

# Optional: Seed phrase for deterministic keypair
export SEED_PHRASE="//Alice"
```

## Usage

### Basic Usage

```bash
# Run with default configuration
cargo run --release

# Run with custom configuration
DEVICE_ID="my-device" TELEMETRY_INTERVAL=5 cargo run --release

# Run with debug logging
RUST_LOG=debug cargo run --release
```

### Using a Specific Keypair

```bash
# Use a seed phrase for deterministic identity
SEED_PHRASE="//Alice" cargo run --release

# Or generate a new random keypair (default)
cargo run --release
```

## Metadata Generation (Required for Production)

To interact with the peaq network's specific pallets, you need to generate the runtime metadata:

```bash
# Install subxt-cli
cargo install subxt-cli

# Download peaq metadata
subxt metadata \
  --url wss://wsspc-akash-agung.peaq.network \
  --output peaq_metadata.scale

# Update src/client.rs to use the metadata
# Add this at the top of the file:
#[subxt::subxt(runtime_metadata_path = "peaq_metadata.scale")]
pub mod peaq_runtime {}
```

Then update the client methods to use the generated types:

```rust
let tx = peaq_runtime::tx()
    .peaq_storage()
    .add_item(item_name.as_bytes().to_vec(), data);

let tx_progress = self.api
    .tx()
    .sign_and_submit_then_watch_default(&tx, &signer)
    .await?;
```

## Telemetry Types

The simulator generates four types of telemetry data:

### 1. Energy Telemetry
```json
{
  "type": "Energy",
  "consumption_kwh": 24.5,
  "voltage": 230.2,
  "current_amps": 12.3
}
```

### 2. Location Telemetry
```json
{
  "type": "Location",
  "latitude": 52.5200,
  "longitude": 13.4050,
  "altitude_meters": 34.5
}
```

### 3. Environmental Telemetry
```json
{
  "type": "Environmental",
  "temperature_celsius": 22.5,
  "humidity_percent": 65.0,
  "pressure_hpa": 1013.25
}
```

### 4. Vehicle Telemetry
```json
{
  "type": "Vehicle",
  "speed_kmh": 85.5,
  "battery_percent": 78.0,
  "odometer_km": 12543.2
}
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test crypto::tests
cargo test telemetry::tests

# Run integration tests (requires network connection)
cargo test --test '*' -- --ignored
```

## Error Handling

The simulator uses a comprehensive error type system:

- `SimulatorError::Subxt`: Substrate/subxt errors
- `SimulatorError::Crypto`: Cryptographic operation failures
- `SimulatorError::Did`: DID-related errors
- `SimulatorError::Telemetry`: Telemetry generation/signing errors
- `SimulatorError::Config`: Configuration errors
- `SimulatorError::Network`: Network connectivity issues

## Performance Considerations

- **Async I/O**: All network operations are non-blocking
- **Efficient Serialization**: Uses serde for fast JSON encoding
- **Minimal Allocations**: Reuses buffers where possible
- **Configurable Intervals**: Adjust telemetry frequency based on needs

## Security

- **Private Key Management**: Keys are never logged or exposed
- **Signature Verification**: All packets are self-verified before submission
- **Secure RNG**: Uses cryptographically secure random number generation
- **No Hardcoded Secrets**: All sensitive data via environment variables

## Roadmap

- [ ] Complete metadata integration with peaq pallets
- [ ] Add support for batch telemetry submission
- [ ] Implement DID document updates
- [ ] Add metrics and monitoring endpoints
- [ ] Support for multiple device simulation
- [ ] WebSocket reconnection logic
- [ ] Persistent storage for device state

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Documentation is updated

## License

MIT License - See LICENSE file for details

## Resources

- [peaq Network Documentation](https://docs.peaq.network/)
- [Substrate Documentation](https://docs.substrate.io/)
- [subxt Documentation](https://docs.rs/subxt/)
- [W3C DID Specification](https://www.w3.org/TR/did-core/)

## Support

For issues and questions:
- Open an issue on GitHub
- Join the peaq Discord community
- Check the peaq documentation

---

Built with ❤️ for the DePIN ecosystem
