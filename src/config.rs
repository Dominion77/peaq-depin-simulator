use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// WebSocket RPC URL for the peaq network
    pub rpc_url: String,
    
    /// Telemetry submission interval in seconds
    pub telemetry_interval_secs: u64,
    
    /// Device identifier
    pub device_id: String,
    
    /// Keypair seed phrase (optional, generates new if None)
    pub seed_phrase: Option<String>,
    
    /// Storage pallet item name
    pub storage_item_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: "wss://wsspc-akash-agung.peaq.network".to_string(),
            telemetry_interval_secs: 10,
            device_id: "sim-001".to_string(),
            seed_phrase: None,
            storage_item_name: "telemetry_stream".to_string(),
        }
    }
}

impl Config {
    pub fn telemetry_interval(&self) -> Duration {
        Duration::from_secs(self.telemetry_interval_secs)
    }

    pub fn from_env() -> crate::Result<Self> {
        let mut config = Self::default();
        
        if let Ok(url) = std::env::var("PEAQ_RPC_URL") {
            config.rpc_url = url;
        }
        
        if let Ok(interval) = std::env::var("TELEMETRY_INTERVAL") {
            config.telemetry_interval_secs = interval.parse()
                .map_err(|e| crate::SimulatorError::Config(format!("Invalid interval: {}", e)))?;
        }
        
        if let Ok(device_id) = std::env::var("DEVICE_ID") {
            config.device_id = device_id;
        }
        
        if let Ok(seed) = std::env::var("SEED_PHRASE") {
            config.seed_phrase = Some(seed);
        }
        
        Ok(config)
    }
}
