#!/bin/bash
# Local CI checks script - matches the GitHub Actions workflow

set -e

echo "🔍 Running local CI checks for pali-types..."
echo

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the project root directory"
    exit 1
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

run_check() {
    local name="$1"
    shift
    echo -e "${BLUE}🔄 $name${NC}"
    if "$@"; then
        echo -e "${GREEN}✅ $name passed${NC}"
    else
        echo -e "${RED}❌ $name failed${NC}"
        return 1
    fi
    echo
}

# Main CI checks
run_check "Code formatting" cargo fmt --all -- --check
run_check "Clippy linting" cargo clippy -- -D warnings
run_check "Run tests" cargo test
run_check "Build library" cargo build --release
run_check "Build documentation" cargo doc --no-deps

echo -e "${GREEN}🎉 All checks passed! Your code is ready for CI.${NC}"