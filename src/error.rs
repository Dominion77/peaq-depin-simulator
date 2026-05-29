use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("Subxt error: {0}")]
    Subxt(#[from] subxt::Error),

    #[error("Subxt block error: {0}")]
    SubxtBlock(#[from] subxt::error::OnlineClientAtBlockError),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("DID error: {0}")]
    Did(String),

    #[error("Telemetry error: {0}")]
    Telemetry(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SimulatorError>;
