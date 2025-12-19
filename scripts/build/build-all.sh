#!/bin/bash
# Build TabSSH for all supported platforms
# Usage: ./scripts/build/build-all.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_DIR"

echo "=== TabSSH Desktop Build ==="
echo "Project directory: $PROJECT_DIR"
echo ""

# Ensure Docker image exists
echo "Step 1: Building Docker image..."
make docker-build

# Build Linux targets
echo ""
echo "Step 2: Building Linux x86_64..."
make build-linux-amd64

echo ""
echo "Step 3: Building Linux ARM64..."
make build-linux-arm64

# List outputs
echo ""
echo "=== Build Complete ==="
echo "Binaries created in: $PROJECT_DIR/binaries/"
ls -la "$PROJECT_DIR/binaries/" 2>/dev/null || echo "(no binaries yet)"
