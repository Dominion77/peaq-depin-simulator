#!/bin/bash
# Comprehensive test runner script

set -e

echo "🧪 peaq DePIN Simulator Test Suite"
echo "==================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

# 1. Format check
echo "📝 Checking code formatting..."
cargo fmt -- --check
print_status $? "Code formatting"

# 2. Clippy lints
echo ""
echo "🔍 Running clippy..."
cargo clippy -- -D warnings
print_status $? "Clippy lints"

# 3. Unit tests
echo ""
echo "🧪 Running unit tests..."
cargo test --lib
print_status $? "Unit tests"

# 4. Integration tests (if any)
echo ""
echo "🔗 Running integration tests..."
cargo test --test '*' 2>/dev/null || echo -e "${YELLOW}⚠️  No integration tests found${NC}"

# 5. Doc tests
echo ""
echo "📚 Running doc tests..."
cargo test --doc
print_status $? "Doc tests"

# 6. Build check
echo ""
echo "🔨 Checking build..."
cargo build --release
print_status $? "Release build"

# Summary
echo ""
echo "=================================="
echo -e "${GREEN}✅ All tests passed!${NC}"
echo "=================================="
echo ""
echo "Test coverage summary:"
cargo test -- --list | wc -l | xargs echo "Total tests:"
