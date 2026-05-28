use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
use sp_core::crypto::Pair as _;
use crate::{Result, SimulatorError, crypto::DeviceKeypair};
use tracing::{info, warn, error};

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
        // In subxt v0.50, use at_current_block() to get the current block reference
        let at_block = self.api
            .at_current_block()
            .await
            .map_err(|e| SimulatorError::Network(format!("Failed to get current block: {}", e)))?;
        
        // Get the block number from the block reference
        Ok(at_block.block_number())
    }

    /// Submit telemetry data to the peaq storage pallet
    /// 
    /// Note: This is a simplified implementation. In production, you would:
    /// 1. Generate proper metadata using subxt-cli
    /// 2. Use the #[subxt::subxt] macro to generate type-safe calls
    /// 3. Handle the specific peaq pallet structure
    pub async fn submit_telemetry(
        &self,
        keypair: &DeviceKeypair,
        _item_name: &str,
        data: Vec<u8>,
    ) -> Result<String> {
        info!("Submitting telemetry data ({} bytes)", data.len());
        
        // Convert sp_core keypair to subxt_signer keypair
        let seed = keypair.pair().to_raw_vec();
        let seed_array: [u8; 32] = seed.try_into()
            .map_err(|_| SimulatorError::Crypto("Invalid seed length".to_string()))?;
        
        let _signer = Keypair::from_secret_key(seed_array)
            .map_err(|e| SimulatorError::Crypto(format!("Keypair conversion failed: {}", e)))?;

        // In a real implementation, this would use the generated peaq runtime types
        // For now, we'll demonstrate the structure with a generic approach
        
        // Example of what the actual call would look like with proper metadata:
        // let tx = peaq_runtime::tx()
        //     .peaq_storage()
        //     .add_item(item_name.as_bytes().to_vec(), data);
        // 
        // let tx_progress = self.api
        //     .tx()
        //     .sign_and_submit_then_watch_default(&tx, &signer)
        //     .await?;
        // 
        // let tx_in_block = tx_progress.wait_for_in_block().await?;
        // let block_hash = tx_in_block.block_hash();
        
        warn!("Telemetry submission requires proper peaq metadata generation");
        warn!("Run: subxt metadata --url {} --output peaq_metadata.scale", self.rpc_url);
        warn!("Then use #[subxt::subxt(runtime_metadata_path = \"peaq_metadata.scale\")]");
        
        // Return a mock transaction hash for demonstration
        let mock_hash = format!("0x{}", hex::encode(&data[..32.min(data.len())]));
        info!("Telemetry would be submitted with hash: {}", mock_hash);
        
        Ok(mock_hash)
    }

    /// Check if a DID exists on-chain
    pub async fn did_exists(&self, _did: &str) -> Result<bool> {
        // This would query the peaq_did pallet storage
        // Requires proper metadata generation
        warn!("DID existence check requires proper peaq metadata");
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
        let seed_array: [u8; 32] = seed.try_into()
            .map_err(|_| SimulatorError::Crypto("Invalid seed length".to_string()))?;
        
        let _signer = Keypair::from_secret_key(seed_array)
            .map_err(|e| SimulatorError::Crypto(format!("Keypair conversion failed: {}", e)))?;

        // In a real implementation:
        // let tx = peaq_runtime::tx()
        //     .peaq_did()
        //     .add_attribute(did.as_bytes().to_vec(), /* attributes */);
        
        warn!("DID registration requires proper peaq metadata generation");
        
        let mock_hash = format!("0x{}", hex::encode(did.as_bytes()));
        info!("DID would be registered with hash: {}", mock_hash);
        
        Ok(mock_hash)
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
    #[ignore] // Requires actual network connection
    async fn test_connect_to_peaq() {
        let result = PeaqClient::connect("wss://wsspc-akash-agung.peaq.network").await;
        assert!(result.is_ok());
    }
}
