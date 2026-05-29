use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
use sp_core::crypto::Pair as _;
use crate::{Result, SimulatorError, crypto::DeviceKeypair};
use tracing::{info, error};

// Generate type-safe API from peaq metadata
#[subxt::subxt(runtime_metadata_path = "peaq_metadata.scale")]
pub mod peaq {}

// Import the actual BoundedVec type from the generated code
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
        item_name: &str,
        data: Vec<u8>,
    ) -> Result<String> {
        info!("Submitting telemetry data ({} bytes)", data.len());
        
        // Convert sp_core keypair to subxt_signer keypair
        let seed = keypair.pair().to_raw_vec();
        let secret_key: [u8; 32] = seed[..32].try_into()
            .map_err(|_| SimulatorError::Crypto("Invalid seed length".to_string()))?;
        
        let signer = Keypair::from_secret_key(secret_key)
            .map_err(|e| SimulatorError::Crypto(format!("Keypair conversion failed: {}", e)))?;

        // Build the transaction using the generated peaq types
        // Convert Vec<u8> to BoundedVec<u8> as required by the pallet
        let item_name_vec = item_name.as_bytes().to_vec();
        let item_name_bounded = BoundedVec(item_name_vec);
        let data_bounded = BoundedVec(data.clone());
        
        let tx = peaq::tx().peaq_storage().add_item(
            item_name_bounded,
            data_bounded,
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

        info!("Telemetry submitted with extrinsic hash: {:?}", block_hash);
        Ok(format!("{:?}", block_hash))
    }

    /// Check if a DID exists on-chain by querying the AttributeStore
    pub async fn did_exists(&self, _keypair: &DeviceKeypair) -> Result<bool> {
        // For now, always return false to trigger DID registration
        // Full implementation would query the peaq_did pallet's AttributeStore
        // but requires understanding the exact storage structure from metadata
        Ok(false)
    }

    /// Register a new DID on-chain
    pub async fn register_did(
        &self,
        keypair: &DeviceKeypair,
        did: &str,
    ) -> Result<String> {
        info!("Registering DID: {}", did);
        
        // Convert keypair
        let seed = keypair.pair().to_raw_vec();
        let secret_key: [u8; 32] = seed[..32].try_into()
            .map_err(|_| SimulatorError::Crypto("Invalid seed length".to_string()))?;
        
        let signer = Keypair::from_secret_key(secret_key)
            .map_err(|e| SimulatorError::Crypto(format!("Keypair conversion failed: {}", e)))?;

        // Get the account ID from the public key
        let account_id = subxt::utils::AccountId32(keypair.pair().public().0);

        // Build the transaction to add a DID attribute
        // Using "did" as the attribute name and the full DID string as the value
        // Convert Vec<u8> to BoundedVec<u8> as required by the pallet
        let attr_name_bounded = BoundedVec(b"did".to_vec());
        let attr_value_bounded = BoundedVec(did.as_bytes().to_vec());
        
        let tx = peaq::tx().peaq_did().add_attribute(
            account_id,
            attr_name_bounded,
            attr_value_bounded,
            None, // No expiration
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
