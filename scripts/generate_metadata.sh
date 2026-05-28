#!/bin/bash
# Script to generate peaq network metadata

set -e

echo "🔧 peaq Metadata Generator"
echo "=========================="
echo ""

# Check if subxt-cli is installed
if ! command -v subxt &> /dev/null; then
    echo "❌ subxt-cli not found"
    echo "Installing subxt-cli..."
    cargo install subxt-cli
fi

# Default RPC URL
RPC_URL="${PEAQ_RPC_URL:-wss://wsspc-akash-agung.peaq.network}"
OUTPUT_FILE="${1:-peaq_metadata.scale}"

echo "📡 Connecting to: $RPC_URL"
echo "📝 Output file: $OUTPUT_FILE"
echo ""

# Generate metadata
echo "Downloading metadata..."
subxt metadata --url "$RPC_URL" --output "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Metadata generated successfully!"
    echo "📦 File: $OUTPUT_FILE"
    echo "📊 Size: $(du -h "$OUTPUT_FILE" | cut -f1)"
    echo ""
    echo "Next steps:"
    echo "1. Add to src/client.rs:"
    echo "   #[subxt::subxt(runtime_metadata_path = \"$OUTPUT_FILE\")]"
    echo "   pub mod peaq_runtime {}"
    echo ""
    echo "2. Rebuild the project:"
    echo "   cargo build --release"
else
    echo ""
    echo "❌ Failed to generate metadata"
    echo "Check your network connection and RPC URL"
    exit 1
fi
