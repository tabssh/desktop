#!/bin/bash
set -e

PREFIX=${PREFIX:-/usr/local}
BINARY_NAME="tabssh"

echo "📦 Installing TabSSH Desktop to $PREFIX/bin"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64)
        ARCH="amd64"
        ;;
    aarch64|arm64)
        ARCH="arm64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

BINARY="binaries/${BINARY_NAME}-${OS}-${ARCH}"

if [ ! -f "$BINARY" ]; then
    echo "Binary not found: $BINARY"
    echo "Please run 'make build' first"
    exit 1
fi

# Install binary
sudo install -m 755 "$BINARY" "$PREFIX/bin/$BINARY_NAME"

echo "✅ TabSSH Desktop installed successfully!"
echo "Run with: $BINARY_NAME"
