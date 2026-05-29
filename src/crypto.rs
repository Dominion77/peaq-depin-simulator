use sp_core::{crypto::Pair as PairTrait, sr25519::Pair, crypto::Ss58Codec};
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
}
