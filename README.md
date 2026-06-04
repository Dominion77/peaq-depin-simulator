# peaq DePIN Node Simulator

[![Crates.io](https://img.shields.io/crates/v/peaq-depin-simulator.svg)](https://crates.io/crates/peaq-depin-simulator)
[![docs.rs](https://img.shields.io/docsrs/peaq-depin-simulator)](https://docs.rs/peaq-depin-simulator)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A production-grade, headless **DePIN** (Decentralized Physical Infrastructure Network) node simulator for the [peaq network](https://www.peaq.network/), written in Rust.

The simulator generates a peaq DID, manages an SR25519 cryptographic keypair, produces realistic IoT telemetry (energy, location, environment, vehicle), cryptographically signs every packet, and anchors it to the peaq chain via Substrate extrinsics — all without any UI.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [As a binary (CLI tool)](#as-a-binary-cli-tool)
  - [As a library](#as-a-library)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Binary usage](#binary-usage)
  - [Library usage](#library-usage)
- [Telemetry Types](#telemetry-types)
- [Architecture](#architecture)
- [Testing](#testing)
- [Security](#security)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- 🔑 **SR25519 Identity** — keypair generation (random or from a seed phrase)
- 🪪 **W3C-compliant DIDs** — `did:peaq:0x…` identifiers registered on-chain
- 📡 **Multi-sensor simulation** — energy, GPS, environmental, and vehicle telemetry
- ✍️ **Cryptographic signing** — every packet is signed with Ed25519 and self-verified
- ⛓️ **Substrate integration** — direct on-chain submission via [subxt](https://docs.rs/subxt/)
- ⚡ **Async-first** — built on [Tokio](https://tokio.rs/) for non-blocking I/O
- 🛡️ **No hardcoded secrets** — all sensitive data via environment variables / `.env`
- 🧪 **Unit-tested core modules**

---

## Installation

### As a binary (CLI tool)

Requires **Rust 1.70+**. Install via Cargo:

```bash
cargo install peaq-depin-simulator
```

This compiles and places the `peaq-depin-simulator` binary in `~/.cargo/bin/`.

#### Or build from source

```bash
git clone https://github.com/Dominion77/peaq-depin-simulator
cd peaq-depin-simulator
cargo build --release
# Binary is at: ./target/release/peaq-depin-simulator
```

### As a library

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
peaq-depin-simulator = "0.1"
```

---

## Quick Start

1. **Install** the binary (see above).

2. **Create a `.env` file** in your working directory (or export variables directly):

   ```env
   PEAQ_RPC_URL=wss://wss-async-agung.peaq.xyz
   DEVICE_ID=my-device-001
   TELEMETRY_INTERVAL=10
   # Optional — omit to generate a fresh random keypair each run
   # SEED_PHRASE=//Alice
   ```

3. **Run** the simulator:

   ```bash
   peaq-depin-simulator
   ```

   You should see output like:

   ```
   INFO peaq_depin_simulator: Starting peaq DePIN Node Simulator
   INFO peaq_depin_simulator: Device DID: did:peaq:0x226f20861c…
   INFO peaq_depin_simulator: Connected to peaq network at wss://wss-async-agung.peaq.xyz
   INFO peaq_depin_simulator: ✅ DID registered successfully
   INFO peaq_depin_simulator: 📸 Generated telemetry packet #1: Vehicle { speed_kmh: 37.1, … }
   INFO peaq_depin_simulator: 🧱 Telemetry anchored to peaq network
   ```

---

## Configuration

All configuration is via **environment variables** (a `.env` file is also supported via [dotenvy](https://docs.rs/dotenvy/)):

| Variable | Default | Description |
|---|---|---|
| `PEAQ_RPC_URL` | `wss://wss-async-agung.peaq.xyz` | peaq node WebSocket RPC endpoint |
| `DEVICE_ID` | `peaq-simulator` | Logical name for this simulated device |
| `TELEMETRY_INTERVAL` | `10` | Seconds between telemetry submissions |
| `SEED_PHRASE` | *(none — random keypair)* | BIP-39 mnemonic or dev path (e.g. `//Alice`) for a deterministic identity |
| `STORAGE_ITEM_NAME` | `telemetry` | peaq storage item key used when anchoring data |
| `RUST_LOG` | `peaq_depin_simulator=info` | Standard `tracing` log filter |

### Example `.env`

```env
PEAQ_RPC_URL=wss://wss-async-agung.peaq.xyz
DEVICE_ID=truck-042
TELEMETRY_INTERVAL=5
SEED_PHRASE=word1 word2 word3 ... word12
RUST_LOG=peaq_depin_simulator=debug
```

---

## Usage

### Binary usage

```bash
# Run with defaults (reads .env if present)
peaq-depin-simulator

# Override any variable inline
DEVICE_ID="sensor-7" TELEMETRY_INTERVAL=30 peaq-depin-simulator

# Use a deterministic keypair (same DID across restarts)
SEED_PHRASE="//Alice" peaq-depin-simulator

# Verbose debug output
RUST_LOG=debug peaq-depin-simulator

# Connect to peaq mainnet
PEAQ_RPC_URL=wss://wss.peaq.network peaq-depin-simulator
```

### Library usage

The crate exposes its core modules so you can embed the simulator in your own Rust application:

```rust
use peaq_depin_simulator::{
    Config,
    crypto::DeviceKeypair,
    did::Did,
    telemetry::TelemetryGenerator,
    client::PeaqClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load config from environment / .env
    let config = Config::from_env()?;

    // Generate (or restore) a cryptographic identity
    let keypair = DeviceKeypair::generate()?;
    println!("DID: {}", Did::from_public_key(&keypair.public_key_hex())?);

    // Connect to the peaq network
    let client = PeaqClient::connect(&config.rpc_url).await?;

    // Generate, sign, and submit a telemetry packet
    let mut gen = TelemetryGenerator::new(config.device_id.clone());
    let mut packet = gen.generate();
    packet.sign(&keypair)?;

    let hash = client
        .submit_telemetry(&keypair, &config.storage_item_name, packet.to_bytes()?)
        .await?;

    println!("Anchored: {hash}");
    Ok(())
}
```

#### Deterministic identity (same DID across restarts)

```rust
// From a seed phrase / dev path
let keypair = DeviceKeypair::from_seed("//Alice")?;

// From a full BIP-39 mnemonic
let keypair = DeviceKeypair::from_seed("word1 word2 ... word12")?;
```

#### Generating all telemetry types manually

```rust
use peaq_depin_simulator::telemetry::{TelemetryGenerator, TelemetryData};

let mut gen = TelemetryGenerator::new("device-001".into());

for _ in 0..4 {
    let packet = gen.generate(); // cycles through Energy → Location → Environmental → Vehicle
    println!("{:?}", packet.data);
}
```

---

## Telemetry Types

The simulator cycles through four sensor profiles per device:

| Type | Fields |
|---|---|
| **Energy** | `consumption_kwh`, `voltage`, `current_amps` |
| **Location** | `latitude`, `longitude`, `altitude_meters` |
| **Environmental** | `temperature_celsius`, `humidity_percent`, `pressure_hpa` |
| **Vehicle** | `speed_kmh`, `battery_percent`, `odometer_km` |

Each packet is serialised to JSON, signed with Ed25519, and submitted as a peaq storage item.

---

## Architecture

```
src/
├── main.rs        — CLI entry point and main telemetry loop
├── lib.rs         — Public library surface
├── config.rs      — Environment-based configuration
├── crypto.rs      — SR25519 keypair generation & signing
├── did.rs         — W3C DID creation and document generation
├── telemetry.rs   — IoT data generation, signing, serialisation
├── client.rs      — peaq network client (subxt wrapper)
└── error.rs       — Unified error types
```

**Data flow:**

```
SR25519 Keypair ──► DID (did:peaq:0x…)
        │
        ▼
TelemetryGenerator ──► sign(Ed25519) ──► serialize(JSON)
                                               │
                                               ▼
                                    PeaqClient.submit_telemetry()
                                               │
                                               ▼
                                    peaq Storage Pallet (on-chain)
```

---

## Testing

```bash
# Run all unit tests
cargo test

# Show println! / log output
cargo test -- --nocapture

# Test a specific module
cargo test crypto::tests
cargo test telemetry::tests
cargo test did::tests

# Integration tests (requires a live network connection)
cargo test --test '*' -- --ignored
```

---

## Security

- Private keys are **never logged** or written to disk
- Every telemetry packet is **self-verified** before submission
- Cryptographically secure RNG (`rand` crate) for key generation
- All secrets supplied exclusively via environment variables / `.env`

---

## Roadmap

- [ ] Full metadata integration with peaq DID & Storage pallets
- [ ] Batch telemetry submission
- [ ] DID document update support
- [ ] Multi-device simulation (spawn N devices concurrently)
- [ ] WebSocket auto-reconnect
- [ ] Prometheus metrics endpoint
- [ ] Persistent keypair storage

---

## Contributing

Contributions are welcome!

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Ensure all checks pass:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
4. Open a pull request

---

## License

MIT — see [LICENSE](LICENSE) for details.

---

## Resources

- [peaq Network Documentation](https://docs.peaq.network/)
- [subxt Documentation](https://docs.rs/subxt/)
- [Substrate Documentation](https://docs.substrate.io/)
- [W3C DID Specification](https://www.w3.org/TR/did-core/)
