use peaq_depin_simulator::{
    crypto::DeviceKeypair,
    did::Did,
    telemetry::TelemetryGenerator,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" peaq DePIN Simulator - Simple Example\n");

    // 1. Generate device identity
    println!("1  Generating device keypair...");
    let keypair = DeviceKeypair::generate()?;
    println!("   Public key: {}", keypair.public_key_hex());

    // 2. Create DID
    println!("\n2  Creating DID...");
    let did = Did::from_public_key(&keypair.public_key_hex())?;
    println!("   DID: {}", did);
    
    // Print DID document
    let did_doc = did.to_document();
    println!("\n   DID Document:");
    println!("{}", serde_json::to_string_pretty(&did_doc)?);

    // 3. Generate telemetry
    println!("\n  Generating telemetry data...");
    let mut generator = TelemetryGenerator::new("example-device".to_string());
    
    for i in 1..=5 {
        let mut packet = generator.generate();
        println!("\n   Packet #{}: {:?}", i, packet.data);
        
        // Sign the packet
        packet.sign(&keypair)?;
        println!("   Signature: {}", packet.signature.as_ref().unwrap());
        
        // Verify signature
        let verified = packet.verify(&keypair)?;
        println!("   Verified: {}", if verified { "verified" } else { "not verified" });
        
        // Serialize
        let bytes = packet.to_bytes()?;
        println!("   Size: {} bytes", bytes.len());
    }

    println!("\n Example completed successfully!");
    println!("\nTo run the full simulator:");
    println!("   cargo run --release");
    
    Ok(())
}
