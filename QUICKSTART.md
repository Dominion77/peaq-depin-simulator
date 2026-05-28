# Quick Start Guide

Get the peaq DePIN Simulator running in 5 minutes.

## Prerequisites

- Rust 1.70+ installed
- Internet connection

## Installation

```bash
# 1. Navigate to project directory
cd peaq-depin-simulator

# 2. Build the project
cargo build --release

# This takes ~8 minutes on first build
```

## Run the Simulator

### Option 1: Default Configuration

```bash
cargo run --release
```

### Option 2: Custom Configuration

```bash
# Set environment variables
export DEVICE_ID="my-device"
export TELEMETRY_INTERVAL=5
export RUST_LOG=info

# Run
cargo run --release
```

### Option 3: Using .env File

```bash
# Copy example config
cp .env.example .env

# Edit .env with your settings
nano .env

# Run
cargo run --release
```

## Expected Output

```
🚀 Starting peaq DePIN Node Simulator
Configuration loaded: device_id=sim-001, interval=10s
Generating new random keypair
Device public key: 0x...
Device DID: did:peaq:0x...
DID Document:
{
  "@context": [...],
  "id": "did:peaq:0x...",
  ...
}
Connected to peaq network at wss://wsspc-akash-agung.peaq.network
Current block number: 12345
Starting telemetry transmission loop (interval: 10s)
📸 Generated telemetry packet #1: Energy { ... }
✅ Signature verified
Packet size: 316 bytes
🧱 Telemetry anchored to peaq network
Transaction hash: 0x...
---
```

## Run Tests

```bash
# Run all tests
cargo test

# Expected output:
# test result: ok. 10 passed; 0 failed; 1 ignored
```

## Run Example

```bash
# Run the simple example
cargo run --example simple_simulator
```

## Configuration Options

| Variable | Default | Description |
|----------|---------|-------------|
| `PEAQ_RPC_URL` | `wss://wsspc-akash-agung.peaq.network` | Network endpoint |
| `TELEMETRY_INTERVAL` | `10` | Seconds between packets |
| `DEVICE_ID` | `sim-001` | Device identifier |
| `SEED_PHRASE` | (random) | Deterministic keypair seed |
| `RUST_LOG` | `info` | Logging level |

## Common Commands

```bash
# Build release version
cargo build --release

# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Troubleshooting

### Build Fails

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Connection Issues

```bash
# Try alternative endpoint
export PEAQ_RPC_URL="wss://wss.agung.peaq.network"
cargo run --release
```

### Tests Fail

```bash
# Run with output
cargo test -- --nocapture
```

## Next Steps

1. **Read Documentation**: See `README.md` for full details
2. **Generate Metadata**: See `SETUP.md` for production setup
3. **Explore Code**: Start with `src/main.rs`
4. **Run Tests**: `cargo test` to verify everything works

## Production Setup

For production deployment with full blockchain interaction:

```bash
# 1. Install subxt-cli
cargo install subxt-cli

# 2. Generate metadata
subxt metadata \
  --url wss://wsspc-akash-agung.peaq.network \
  --output peaq_metadata.scale

# 3. Update src/client.rs (see SETUP.md)

# 4. Rebuild
cargo build --release

# 5. Deploy
./target/release/peaq-depin-simulator
```

## Getting Help

- **Documentation**: `README.md`, `ARCHITECTURE.md`, `SETUP.md`
- **Testing**: `TESTING.md`
- **Summary**: `PROJECT_SUMMARY.md`
- **Checklist**: `CHECKLIST.md`

---

**You're ready to go! 🚀**

Run `cargo run --release` to start the simulator.
