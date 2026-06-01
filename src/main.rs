use peaq_depin_simulator::{
    Config, Result,
    crypto::DeviceKeypair,
    did::Did,
    telemetry::TelemetryGenerator,
    client::PeaqClient,
};
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "peaq_depin_simulator=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!(" Starting peaq DePIN Node Simulator");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded: device_id={}, interval={}s", 
          config.device_id, config.telemetry_interval_secs);

    // Initialize cryptographic identity
    let keypair = if let Some(seed) = &config.seed_phrase {
        info!("Loading keypair from seed phrase");
        DeviceKeypair::from_seed(seed)?
    } else {
        info!("Generating new random keypair");
        DeviceKeypair::generate()?
    };
    
    info!("Device public key: {}", keypair.public_key_hex());
    info!("Device EVM address (MetaMask): {}", keypair.evm_address());
    info!("Device SS58 address (Substrate): {}", keypair.ss58_address());

    // Create DID
    let did = Did::from_public_key(&keypair.public_key_hex())?;
    info!("Device DID: {}", did);
    
    // Print DID document
    let did_doc = did.to_document();
    info!("DID Document:\n{}", serde_json::to_string_pretty(&did_doc)?);

    // Connect to peaq network
    let client = PeaqClient::connect(&config.rpc_url).await?;
    info!("Connected to peaq network at {}", client.rpc_url());

    // Check current block
    match client.current_block().await {
        Ok(block) => info!("Current block number: {}", block),
        Err(e) => warn!("Could not fetch block number: {}", e),
    }

    // Check if DID exists, register if not
    match client.did_exists(&keypair).await {
        Ok(exists) => {
            if !exists {
                info!("DID not found on-chain, registering...");
                match client.register_did(&keypair, did.as_str()).await {
                    Ok(hash) => info!("DID registered with hash: {}", hash),
                    Err(e) => warn!("DID registration failed: {}", e),
                }
            } else {
                info!("DID already exists on-chain");
            }
        }
        Err(e) => warn!("Could not check DID existence: {}", e),
    }

    // Initialize telemetry generator
    let mut telemetry_gen = TelemetryGenerator::new(config.device_id.clone());
    info!("Telemetry generator initialized");

    // Main telemetry loop
    info!("Starting telemetry transmission loop (interval: {}s)", 
          config.telemetry_interval_secs);
    
    let mut interval = tokio::time::interval(config.telemetry_interval());
    let mut packet_count = 0u64;

    loop {
        interval.tick().await;

        // Generate telemetry
        let mut packet = telemetry_gen.generate();
        packet_count += 1;

        info!("📸 Generated telemetry packet #{}: {:?}", packet_count, packet.data);

        // Sign the packet
        if let Err(e) = packet.sign(&keypair) {
            error!("Failed to sign packet: {}", e);
            continue;
        }

        // Verify signature (self-check)
        match packet.verify(&keypair) {
            Ok(true) => info!(" Signature verified"),
            Ok(false) => {
                error!(" Signature verification failed");
                continue;
            }
            Err(e) => {
                error!("Signature verification error: {}", e);
                continue;
            }
        }

        // Serialize packet
        let packet_bytes = match packet.to_bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Failed to serialize packet: {}", e);
                continue;
            }
        };

        info!("Packet size: {} bytes", packet_bytes.len());

        // Submit to peaq network
        match client.submit_telemetry(
            &keypair,
            &config.storage_item_name,
            packet_bytes,
        ).await {
            Ok(hash) => {
                info!("🧱 Telemetry anchored to peaq network");
                info!("Transaction hash: {}", hash);
            }
            Err(e) => {
                error!("Failed to submit telemetry: {}", e);
            }
        }

        info!("---");
    }
}
