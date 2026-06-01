use sp_core::{crypto::Pair as PairTrait, sr25519::Pair, crypto::Ss58Codec, H160, blake2_256};
use crate::{Result, SimulatorError};

/// Cryptographic keypair manager for device identity
#[derive(Clone)]
pub struct DeviceKeypair {
    pair: Pair,
}

impl DeviceKeypair {
    /// Generate a new random keypair
    pub fn generate() -> Result<Self> {
        let (pair, _seed) = Pair::generate();
        Ok(Self { pair })
    }

    /// Create keypair from seed phrase
    pub fn from_seed(seed: &str) -> Result<Self> {
        let pair = Pair::from_string(seed, None)
            .map_err(|e| SimulatorError::Crypto(format!("Failed to parse seed: {:?}", e)))?;
        Ok(Self { pair })
    }

    /// Get the public key as hex string
    pub fn public_key_hex(&self) -> String {
        format!("0x{}", hex::encode(self.pair.public().as_ref() as &[u8]))
    }

    /// Get the SS58 address for peaq network (SS58 prefix 42 for generic Substrate)
    /// Peaq uses prefix 42 (generic Substrate address format)
    pub fn ss58_address(&self) -> String {
        self.pair.public().to_ss58check_with_version(42u16.into())
    }

    /// Get the EVM H160 address (Ethereum-style 0x address)
    /// This is derived from the Substrate public key using blake2_256 hash
    pub fn evm_address(&self) -> String {
        let public_key = self.pair.public().0;
        // Hash the public key with blake2_256 and take the last 20 bytes
        let hash = blake2_256(&public_key);
        let h160 = H160::from_slice(&hash[12..32]);
        format!("0x{}", hex::encode(h160.as_bytes()))
    }

    /// Get the underlying pair for signing
    pub fn pair(&self) -> &Pair {
        &self.pair
    }

    /// Sign arbitrary data
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.pair.sign(data).0.to_vec()
    }

    /// Verify a signature
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }
        
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(signature);
        
        Pair::verify(
            &sp_core::sr25519::Signature::from_raw(sig_bytes),
            data,
            &self.pair.public()
        )
    }
}

impl std::fmt::Debug for DeviceKeypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceKeypair")
            .field("public_key", &self.public_key_hex())
            .field("ss58_address", &self.ss58_address())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = DeviceKeypair::generate().unwrap();
        assert!(keypair.public_key_hex().starts_with("0x"));
        assert_eq!(keypair.public_key_hex().len(), 66);
    }

    #[test]
    fn test_keypair_from_seed() {
        let keypair = DeviceKeypair::from_seed("//Alice").unwrap();
        assert!(keypair.public_key_hex().starts_with("0x"));
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = DeviceKeypair::generate().unwrap();
        let data = b"test message";
        let signature = keypair.sign(data);
        
        assert!(keypair.verify(data, &signature));
        assert!(!keypair.verify(b"wrong data", &signature));
    }

    #[test]
    fn test_peaq_simulator_seed_derivation() {
        // This test verifies that our seed produces the expected address
        let seed = "//PeaqSimulator001";
        let keypair = DeviceKeypair::from_seed(seed).unwrap();
        
        let public_key = keypair.public_key_hex();
        let ss58_address = keypair.ss58_address();
        let evm_address = keypair.evm_address();
        
        println!("\n=== PeaqSimulator001 Keypair ===");
        println!("Seed: {}", seed);
        println!("Public key: {}", public_key);
        println!("SS58 address (prefix 42): {}", ss58_address);
        println!("EVM address: {}", evm_address);
        
        // Expected values based on the seed
        assert_eq!(public_key, "0x226f20861c7203eb191f2105dae5118a2fb54acc4bec47a2d0e83140a2fab81f");
        assert_eq!(ss58_address, "5CqrVmifXhjuyfDJvxCt8DyEKj7VTZd4QiapRtn8ZEjfDr9d");
        assert_eq!(evm_address, "0xc3b1a33f0cf1fd7ee77bab1a88aa2cdf1c7aef15");
        
        println!(" All addresses match expected values!");
    }
}

