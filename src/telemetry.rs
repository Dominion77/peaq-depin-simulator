use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::{Result, SimulatorError};

/// Telemetry data packet from a simulated IoT device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPacket {
    pub device_id: String,
    pub timestamp: u64,
    pub data: TelemetryData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Various types of telemetry data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TelemetryData {
    Energy {
        consumption_kwh: f64,
        voltage: f64,
        current_amps: f64,
    },
    Location {
        latitude: f64,
        longitude: f64,
        altitude_meters: f64,
    },
    Environmental {
        temperature_celsius: f64,
        humidity_percent: f64,
        pressure_hpa: f64,
    },
    Vehicle {
        speed_kmh: f64,
        battery_percent: f64,
        odometer_km: f64,
    },
}

/// Telemetry generator for simulating IoT device data
pub struct TelemetryGenerator {
    device_id: String,
    rng: rand::rngs::ThreadRng,
}

impl TelemetryGenerator {
    pub fn new(device_id: String) -> Self {
        Self {
            device_id,
            rng: rand::thread_rng(),
        }
    }

    /// Generate a random telemetry packet
    pub fn generate(&mut self) -> TelemetryPacket {
        let data = match self.rng.gen_range(0..4) {
            0 => self.generate_energy(),
            1 => self.generate_location(),
            2 => self.generate_environmental(),
            _ => self.generate_vehicle(),
        };

        TelemetryPacket {
            device_id: self.device_id.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            signature: None,
        }
    }

    fn generate_energy(&mut self) -> TelemetryData {
        TelemetryData::Energy {
            consumption_kwh: self.rng.gen_range(0.5..50.0),
            voltage: self.rng.gen_range(220.0..240.0),
            current_amps: self.rng.gen_range(1.0..20.0),
        }
    }

    fn generate_location(&mut self) -> TelemetryData {
        TelemetryData::Location {
            latitude: self.rng.gen_range(-90.0..90.0),
            longitude: self.rng.gen_range(-180.0..180.0),
            altitude_meters: self.rng.gen_range(0.0..1000.0),
        }
    }

    fn generate_environmental(&mut self) -> TelemetryData {
        TelemetryData::Environmental {
            temperature_celsius: self.rng.gen_range(-20.0..50.0),
            humidity_percent: self.rng.gen_range(0.0..100.0),
            pressure_hpa: self.rng.gen_range(950.0..1050.0),
        }
    }

    fn generate_vehicle(&mut self) -> TelemetryData {
        TelemetryData::Vehicle {
            speed_kmh: self.rng.gen_range(0.0..120.0),
            battery_percent: self.rng.gen_range(0.0..100.0),
            odometer_km: self.rng.gen_range(0.0..500000.0),
        }
    }
}

impl TelemetryPacket {
    /// Serialize the packet to JSON bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| SimulatorError::Telemetry(format!("Serialization failed: {}", e)))
    }

    /// Sign the telemetry packet
    pub fn sign(&mut self, keypair: &crate::crypto::DeviceKeypair) -> Result<()> {
        // Create a canonical representation for signing (without the signature field)
        let mut unsigned = self.clone();
        unsigned.signature = None;
        
        let data = serde_json::to_vec(&unsigned)?;
        let signature = keypair.sign(&data);
        
        self.signature = Some(hex::encode(signature));
        Ok(())
    }

    /// Verify the packet signature
    pub fn verify(&self, keypair: &crate::crypto::DeviceKeypair) -> Result<bool> {
        let signature_hex = self.signature.as_ref()
            .ok_or_else(|| SimulatorError::Telemetry("No signature present".to_string()))?;
        
        let signature = hex::decode(signature_hex)
            .map_err(|e| SimulatorError::Telemetry(format!("Invalid signature hex: {}", e)))?;
        
        let mut unsigned = self.clone();
        unsigned.signature = None;
        let data = serde_json::to_vec(&unsigned)?;
        
        Ok(keypair.verify(&data, &signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::DeviceKeypair;

    #[test]
    fn test_telemetry_generation() {
        let mut gen = TelemetryGenerator::new("test-device".to_string());
        let packet = gen.generate();
        
        assert_eq!(packet.device_id, "test-device");
        assert!(packet.timestamp > 0);
    }

    #[test]
    fn test_packet_signing() {
        let mut gen = TelemetryGenerator::new("test-device".to_string());
        let mut packet = gen.generate();
        let keypair = DeviceKeypair::generate().unwrap();
        
        packet.sign(&keypair).unwrap();
        assert!(packet.signature.is_some());
        assert!(packet.verify(&keypair).unwrap());
    }

    #[test]
    fn test_packet_serialization() {
        let mut gen = TelemetryGenerator::new("test-device".to_string());
        let packet = gen.generate();
        
        let bytes = packet.to_bytes().unwrap();
        assert!(!bytes.is_empty());
        
        let deserialized: TelemetryPacket = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(deserialized.device_id, packet.device_id);
    }
}
