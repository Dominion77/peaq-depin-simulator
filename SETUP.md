# Setup Guide

Complete setup instructions for the peaq DePIN Simulator.

## Prerequisites

### 1. Install Rust

```bash
# Install rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### 2. Install subxt-cli (Required for Metadata)

```bash
cargo install subxt-cli
```

### 3. System Dependencies

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev
```

#### macOS
```bash
brew install openssl pkg-config
```

#### Windows
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
- Or install [MinGW-w64](https://www.mingw-w64.org/)

## Quick Start

### 1. Clone and Build

```bash
# Clone the repository
git clone <repository-url>
cd peaq-depin-simulator

# Build the project
cargo build --release

# This will take 5-10 minutes on first build
```

### 2. Configure Environment

```bash
# Copy example environment file
cp .env.example .env

# Edit configuration (optional)
nano .env
```

### 3. Run the Simulator

```bash
# Run with default configuration
cargo run --release

# Or with custom settings
DEVICE_ID="my-device" cargo run --release
```

## Detailed Configuration

### Environment Variables

Create a `.env` file or export these variables:

```bash
# Required: Peaq network endpoint
export PEAQ_RPC_URL="wss://wsspc-akash-agung.peaq.network"

# Optional: Telemetry interval (default: 10 seconds)
export TELEMETRY_INTERVAL=10

# Optional: Device identifier (default: sim-001)
export DEVICE_ID="sim-001"

# Optional: Seed phrase for deterministic keypair
# If not set, a random keypair is generated
export SEED_PHRASE="//Alice"

# Optional: Logging level
export RUST_LOG="info"
```

### Logging Levels

Control verbosity with `RUST_LOG`:

```bash
# Minimal output
RUST_LOG=error cargo run

# Normal output (default)
RUST_LOG=info cargo run

# Detailed output
RUST_LOG=debug cargo run

# Very verbose
RUST_LOG=trace cargo run

# Module-specific logging
RUST_LOG=peaq_depin_simulator=debug,subxt=warn cargo run
```

## Metadata Generation (Production Setup)

For production use, you need to generate the peaq runtime metadata:

### Step 1: Download Metadata

```bash
# Ensure subxt-cli is installed
cargo install subxt-cli

# Download metadata from peaq testnet
subxt metadata \
  --url wss://wsspc-akash-agung.peaq.network \
  --output peaq_metadata.scale

# This creates a peaq_metadata.scale file
```

### Step 2: Update Client Code

Add this to `src/client.rs`:

```rust
// At the top of the file, after imports
#[subxt::subxt(runtime_metadata_path = "peaq_metadata.scale")]
pub mod peaq_runtime {}
```

### Step 3: Use Generated Types

Update the `submit_telemetry` method:

```rust
pub async fn submit_telemetry(
    &self,
    keypair: &DeviceKeypair,
    item_name: &str,
    data: Vec<u8>,
) -> Result<String> {
    let seed = keypair.pair().to_raw_vec();
    let signer = Keypair::from_seed(seed.try_into().unwrap())?;

    // Use generated types
    let tx = peaq_runtime::tx()
        .peaq_storage()
        .add_item(item_name.as_bytes().to_vec(), data);

    let tx_progress = self.api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?;

    let tx_in_block = tx_progress.wait_for_in_block().await?;
    let block_hash = tx_in_block.block_hash();

    Ok(format!("{:?}", block_hash))
}
```

### Step 4: Rebuild

```bash
cargo build --release
```

## Testing

### Run All Tests

```bash
# Run unit tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test crypto::tests::test_keypair_generation
```

### Run Integration Tests

Integration tests require network connectivity:

```bash
# Run ignored tests (network-dependent)
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored
```

### Code Quality Checks

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Check without building
cargo check
```

## Development Workflow

### 1. Make Changes

Edit source files in `src/`:

```bash
# Open in your editor
code src/telemetry.rs
```

### 2. Run Tests

```bash
cargo test
```

### 3. Run Locally

```bash
cargo run
```

### 4. Build Release

```bash
cargo build --release
```

## Troubleshooting

### Issue: Connection Failed

**Error**: `Failed to connect to peaq network`

**Solutions**:
1. Check network connectivity
2. Verify RPC URL is correct
3. Try alternative endpoint:
   ```bash
   export PEAQ_RPC_URL="wss://wss.agung.peaq.network"
   ```

### Issue: Compilation Errors

**Error**: `error: linking with 'cc' failed`

**Solutions**:
1. Install build tools (see Prerequisites)
2. Update Rust: `rustup update`
3. Clean and rebuild: `cargo clean && cargo build`

### Issue: Metadata Generation Failed

**Error**: `Failed to fetch metadata`

**Solutions**:
1. Check network connection
2. Verify node is running
3. Try different RPC endpoint
4. Check firewall settings

### Issue: Signature Verification Failed

**Error**: `Signature verification failed`

**Solutions**:
1. Ensure keypair is consistent
2. Check data hasn't been modified
3. Verify signature format is correct

### Issue: High Memory Usage

**Solutions**:
1. Reduce telemetry interval
2. Check for memory leaks with `valgrind`
3. Monitor with: `cargo run --release & top -p $!`

## Performance Tuning

### Optimize Build

```bash
# Enable link-time optimization
cargo build --release

# Profile-guided optimization (advanced)
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Adjust Telemetry Rate

```bash
# High frequency (1 second)
TELEMETRY_INTERVAL=1 cargo run --release

# Low frequency (60 seconds)
TELEMETRY_INTERVAL=60 cargo run --release
```

### Monitor Performance

```bash
# CPU and memory usage
cargo run --release &
top -p $!

# Network usage
iftop

# Detailed profiling
cargo install flamegraph
cargo flamegraph
```

## Production Deployment

### 1. Build Optimized Binary

```bash
cargo build --release --locked
```

### 2. Copy Binary

```bash
cp target/release/peaq-depin-simulator /usr/local/bin/
```

### 3. Create Systemd Service (Linux)

Create `/etc/systemd/system/peaq-simulator.service`:

```ini
[Unit]
Description=peaq DePIN Simulator
After=network.target

[Service]
Type=simple
User=peaq
WorkingDirectory=/opt/peaq-simulator
Environment="PEAQ_RPC_URL=wss://wsspc-akash-agung.peaq.network"
Environment="DEVICE_ID=prod-001"
Environment="RUST_LOG=info"
ExecStart=/usr/local/bin/peaq-depin-simulator
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable peaq-simulator
sudo systemctl start peaq-simulator
sudo systemctl status peaq-simulator
```

### 4. Monitor Logs

```bash
# Follow logs
sudo journalctl -u peaq-simulator -f

# View recent logs
sudo journalctl -u peaq-simulator -n 100
```

## Docker Deployment (Optional)

### Create Dockerfile

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/peaq-depin-simulator /usr/local/bin/

ENV RUST_LOG=info
ENV PEAQ_RPC_URL=wss://wsspc-akash-agung.peaq.network

CMD ["peaq-depin-simulator"]
```

### Build and Run

```bash
# Build image
docker build -t peaq-simulator .

# Run container
docker run -d \
  --name peaq-sim \
  -e DEVICE_ID=docker-001 \
  -e TELEMETRY_INTERVAL=10 \
  peaq-simulator

# View logs
docker logs -f peaq-sim
```

## Security Best Practices

### 1. Protect Seed Phrases

```bash
# Never commit .env files
echo ".env" >> .gitignore

# Use environment variables in production
export SEED_PHRASE="your-secret-seed"
```

### 2. Use Secure Connections

Always use `wss://` (WebSocket Secure), never `ws://`.

### 3. Limit Permissions

```bash
# Create dedicated user
sudo useradd -r -s /bin/false peaq

# Set ownership
sudo chown -R peaq:peaq /opt/peaq-simulator
```

### 4. Regular Updates

```bash
# Update dependencies
cargo update

# Check for security advisories
cargo audit
```

## Next Steps

1. **Read the Architecture**: See `ARCHITECTURE.md`
2. **Explore the Code**: Start with `src/main.rs`
3. **Run Tests**: `cargo test`
4. **Generate Metadata**: Follow production setup
5. **Deploy**: Use systemd or Docker

## Getting Help

- **Documentation**: See `README.md` and `ARCHITECTURE.md`
- **Issues**: Open a GitHub issue
- **Community**: Join peaq Discord
- **Logs**: Check with `RUST_LOG=debug`

## Useful Commands Reference

```bash
# Development
cargo run                    # Run in debug mode
cargo run --release          # Run optimized
cargo test                   # Run tests
cargo fmt                    # Format code
cargo clippy                 # Lint code

# Building
cargo build                  # Debug build
cargo build --release        # Release build
cargo clean                  # Clean build artifacts

# Information
cargo tree                   # Show dependency tree
cargo outdated              # Check for updates
cargo audit                 # Security audit

# Metadata
subxt metadata --url <url> --output peaq_metadata.scale
```

---

You're now ready to run the peaq DePIN Simulator! 🚀
