# Metadata Setup Guide

To enable full blockchain integration, you need to generate the peaq runtime metadata.

## Step 1: Install subxt-cli

```bash
cargo install subxt-cli
```

**Note**: This takes 5-10 minutes to compile.

## Step 2: Generate Metadata

```bash
subxt metadata --url wss://wss-async-agung.peaq.xyz --output-file peaq_metadata.scale
```

This downloads the runtime metadata from the live peaq network.

## Step 3: Update src/client.rs

Add this at the top of `src/client.rs` (after the imports):

```rust
#[subxt::subxt(runtime_metadata_path = "peaq_metadata.scale")]
pub mod peaq {}
```

## Step 4: Update Methods

Replace the placeholder implementations with real ones:

### For `submit_telemetry`:

```rust
pub async fn submit_telemetry(
    &self,
    keypair: &DeviceKeypair,
    item_name: &str,
    data: Vec<u8>,
) -> Result<String> {
    info!("Submitting telemetry data ({} bytes)", data.len());
    
    let seed = keypair.pair().to_raw_vec();
    let secret_key: [u8; 32] = seed[..32].try_into()
        .map_err(|_| SimulatorError::Crypto("Invalid seed length".to_string()))?;
    
    let signer = Keypair::from_secret_key(secret_key)
        .map_err(|e| SimulatorError::Crypto(format!("Keypair conversion failed: {}", e)))?;

    // Use generated types
    let tx = peaq::tx()
        .peaq_storage()
        .add_item(item_name.as_bytes().to_vec(), data);

    let tx_progress = self.api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?;

    let tx_in_block = tx_progress.wait_for_in_block().await?;
    let block_hash = tx_in_block.block_hash();

    info!("Telemetry submitted in block: {:?}", block_hash);
    Ok(format!("{:?}", block_hash))
}
```

### For `register_did`:

```rust
pub async fn register_did(
    &self,
    keypair: &DeviceKeypair,
    did: &str,
) -> Result<String> {
    info!("Registering DID: {}", did);
    
    let seed = keypair.pair().to_raw_vec();
    let secret_key: [u8; 32] = seed[..32].try_into()
        .map_err(|_| SimulatorError::Crypto("Invalid seed length".to_string()))?;
    
    let signer = Keypair::from_secret_key(secret_key)
        .map_err(|e| SimulatorError::Crypto(format!("Keypair conversion failed: {}", e)))?;

    // Use generated types
    let tx = peaq::tx()
        .peaq_did()
        .add_attribute(did.as_bytes().to_vec(), /* attributes */);

    let tx_progress = self.api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?;

    let tx_in_block = tx_progress.wait_for_in_block().await?;
    let block_hash = tx_in_block.block_hash();

    info!("DID registered in block: {:?}", block_hash);
    Ok(format!("{:?}", block_hash))
}
```

## Step 5: Rebuild

```bash
cargo build --release
```

## Step 6: Run

```bash
cargo run --release
```

Now the simulator will actually submit transactions to the peaq blockchain!

## Troubleshooting

### Metadata generation fails

- Check network connectivity
- Verify the RPC endpoint is accessible
- Try a different endpoint if needed

### Compilation errors after adding metadata

- The peaq pallet names might be different
- Check the generated `peaq` module for available pallets
- Adjust the method calls accordingly

### Transaction fails

- Ensure you have sufficient balance for transaction fees
- Check that the account has the necessary permissions
- Verify the pallet and method names are correct

## Current Status

Without metadata generation, the simulator:
- ✅ Connects to the network
- ✅ Generates DIDs
- ✅ Creates and signs telemetry
- ✅ Simulates submission (logs what would be sent)

With metadata generation, the simulator:
- ✅ All of the above, plus:
- ✅ Actually submits transactions to the blockchain
- ✅ Registers DIDs on-chain
- ✅ Stores telemetry data on-chain
- ✅ Returns real transaction hashes

---

**The simulator is fully functional as-is for development and testing. Metadata generation is only needed for actual on-chain transactions.**
