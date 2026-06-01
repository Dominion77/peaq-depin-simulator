use subxt::{OnlineClient, PolkadotConfig};
use sp_core::crypto::Pair as _;
use crate::{Result, SimulatorError, crypto::DeviceKeypair};
use tracing::{info, error};

// Custom signer that uses sp_core keypair directly
use subxt::tx::Signer;
use subxt::config::substrate::MultiSignature;

/// Custom signer implementation that uses sp_core keypair
pub struct SpCoreSigner {
    keypair: sp_core::sr25519::Pair,
}

impl SpCoreSigner {
    pub fn new(keypair: sp_core::sr25519::Pair) -> Self {
        Self { keypair }
    }
}

impl Signer<PolkadotConfig> for SpCoreSigner {
    fn account_id(&self) -> subxt::utils::AccountId32 {
        subxt::utils::AccountId32(self.keypair.public().0)
    }

    fn sign(&self, payload: &[u8]) -> <PolkadotConfig as subxt::Config>::Signature {
        let signature = sp_core::crypto::Pair::sign(&self.keypair, payload);
        MultiSignature::Sr25519(signature.0)
    }
}

// Generate type-safe API from peaq metadata
#[subxt::subxt(runtime_metadata_path = "peaq_metadata.scale")]
pub mod peaq {}

// Import BoundedVec from generated code
use peaq::runtime_types::bounded_collections::bounded_vec::BoundedVec;

/// Peaq network client wrapper
pub struct PeaqClient {
    api: OnlineClient<PolkadotConfig>,
    rpc_url: String,
}

impl PeaqClient {
    /// Connect to the peaq network
    pub async fn connect(rpc_url: &str) -> Result<Self> {
        info!("Connecting to peaq network at {}", rpc_url);
        
        let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to peaq network: {}", e);
                SimulatorError::Network(format!("Connection failed: {}", e))
            })?;
        
        info!("Successfully connected to peaq network");
        
        Ok(Self {
            api,
            rpc_url: rpc_url.to_string(),
        })
    }

    /// Get the current block number
    pub async fn current_block(&self) -> Result<u64> {
        let at_block = self.api
            .at_current_block()
            .await
            .map_err(|e| SimulatorError::Network(format!("Failed to get current block: {}", e)))?;
        
        // Get the block number from the block reference
        Ok(at_block.block_number())
    }

    /// Submit telemetry data to the peaq storage pallet
    pub async fn submit_telemetry(
        &self,
        keypair: &DeviceKeypair,
        _item_name: &str,
        data: Vec<u8>,
    ) -> Result<String> {
        info!("Submitting telemetry data ({} bytes)", data.len());
        
        // Use custom signer with sp_core keypair directly
        let signer = SpCoreSigner::new(keypair.pair().clone());

        // Use system.remark_with_event to store telemetry data on-chain
        let tx = peaq::tx().system().remark_with_event(data.clone());

        // Sign and submit the transaction
        let mut tx_api = self.api.tx().await?;
        let tx_progress = tx_api
            .sign_and_submit_then_watch_default(&tx, &signer)
            .await
            .map_err(|e| SimulatorError::Network(format!("Transaction submission failed: {}", e)))?;

        // Wait for the transaction to be finalized
        let tx_events = tx_progress.wait_for_finalized_success().await
            .map_err(|e| SimulatorError::Network(format!("Transaction finalization failed: {}", e)))?;
        let block_hash = tx_events.extrinsic_hash();

        info!("Telemetry submitted with extrinsic hash: {:?}", block_hash);
        Ok(format!("{:?}", block_hash))
    }

    pub async fn did_exists(&self, _keypair: &DeviceKeypair) -> Result<bool> {
        Ok(false)
    }

    /// Register a new DID on-chain
    pub async fn register_did(
        &self,
        keypair: &DeviceKeypair,
        did: &str,
    ) -> Result<String> {
        info!("Registering DID: {}", did);
        
        // Use custom signer with sp_core keypair directly
        let signer = SpCoreSigner::new(keypair.pair().clone());

        // Get the account ID from the public key
        let account_id = subxt::utils::AccountId32(keypair.pair().public().0);

        // Build the transaction to add a DID attribute
        // Wrap Vec<u8> in BoundedVec as required by the pallet
        let tx = peaq::tx().peaq_did().add_attribute(
            account_id,
            BoundedVec(b"did".to_vec()),
            BoundedVec(did.as_bytes().to_vec()),
            None,
        );

        // Sign and submit the transaction
        let mut tx_api = self.api.tx().await?;
        let tx_progress = tx_api
            .sign_and_submit_then_watch_default(&tx, &signer)
            .await
            .map_err(|e| SimulatorError::Network(format!("Transaction submission failed: {}", e)))?;

        // Wait for the transaction to be finalized
        let tx_events = tx_progress.wait_for_finalized_success().await
            .map_err(|e| SimulatorError::Network(format!("Transaction finalization failed: {}", e)))?;
        let block_hash = tx_events.extrinsic_hash();

        info!("DID registered with extrinsic hash: {:?}", block_hash);
        Ok(format!("{:?}", block_hash))
    }

    /// Get the RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect_to_peaq() {
        let _ = dotenvy::dotenv();
        let rpc_url = std::env::var("PEAQ_RPC_URL")
            .unwrap_or_else(|_| "wss://wss-async-agung.peaq.xyz".to_string());
        
        let result = PeaqClient::connect(&rpc_url).await;
        assert!(result.is_ok(), "Failed to connect to peaq network at {}: {:?}", rpc_url, result.err());
    }
}
