use crate::{Result, SimulatorError};

/// Decentralized Identifier (DID) manager
#[derive(Debug, Clone)]
pub struct Did {
    identifier: String,
}

impl Did {
    /// Create a DID from a public key
    pub fn from_public_key(public_key_hex: &str) -> Result<Self> {
        if !public_key_hex.starts_with("0x") {
            return Err(SimulatorError::Did(
                "Public key must start with 0x".to_string()
            ));
        }

        let identifier = format!("did:peaq:{}", public_key_hex);
        Ok(Self { identifier })
    }

    /// Get the full DID string
    pub fn as_str(&self) -> &str {
        &self.identifier
    }

    /// Extract the public key portion from the DID
    pub fn public_key(&self) -> Result<String> {
        self.identifier
            .strip_prefix("did:peaq:")
            .map(|s| s.to_string())
            .ok_or_else(|| SimulatorError::Did("Invalid DID format".to_string()))
    }

    /// Create a W3C-compliant DID document
    pub fn to_document(&self) -> serde_json::Value {
        serde_json::json!({
            "@context": [
                "https://www.w3.org/ns/did/v1",
                "https://w3id.org/security/suites/sr25519-2020/v1"
            ],
            "id": self.identifier,
            "verificationMethod": [{
                "id": format!("{}#keys-1", self.identifier),
                "type": "Sr25519VerificationKey2020",
                "controller": self.identifier,
                "publicKeyMultibase": self.public_key().unwrap_or_default()
            }],
            "authentication": [
                format!("{}#keys-1", self.identifier)
            ]
        })
    }
}

impl std::fmt::Display for Did {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_creation() {
        let pubkey = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let did = Did::from_public_key(pubkey).unwrap();
        
        assert_eq!(did.as_str(), format!("did:peaq:{}", pubkey));
    }

    #[test]
    fn test_did_invalid_pubkey() {
        let result = Did::from_public_key("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_did_document() {
        let pubkey = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let did = Did::from_public_key(pubkey).unwrap();
        let doc = did.to_document();
        
        assert_eq!(doc["id"], format!("did:peaq:{}", pubkey));
        assert!(doc["verificationMethod"].is_array());
    }

    #[test]
    fn test_extract_public_key() {
        let pubkey = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let did = Did::from_public_key(pubkey).unwrap();
        
        assert_eq!(did.public_key().unwrap(), pubkey);
    }
}
