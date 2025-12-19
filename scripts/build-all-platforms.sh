#!/bin/bash
set -e

echo "🏗️ Building TabSSH Desktop for all platforms"

VERSION=${VERSION:-"dev"}
OUTPUT_DIR="releases"

mkdir -p $OUTPUT_DIR

# Linux AMD64 (musl static)
echo "Building Linux AMD64..."
docker run --rm -v $(pwd):/workspace -w /workspace tabssh-builder:latest \
    cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/tabssh $OUTPUT_DIR/tabssh-linux-amd64

# Linux ARM64 (musl static)
echo "Building Linux ARM64..."
docker run --rm -v $(pwd):/workspace -w /workspace tabssh-builder:latest \
    cargo build --release --target aarch64-unknown-linux-musl
cp target/aarch64-unknown-linux-musl/release/tabssh $OUTPUT_DIR/tabssh-linux-arm64

# macOS (requires macOS host or cross toolchain)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Building macOS AMD64..."
    cargo build --release --target x86_64-apple-darwin
    cp target/x86_64-apple-darwin/release/tabssh $OUTPUT_DIR/tabssh-macos-amd64
    
    echo "Building macOS ARM64..."
    cargo build --release --target aarch64-apple-darwin
    cp target/aarch64-apple-darwin/release/tabssh $OUTPUT_DIR/tabssh-macos-arm64
else
    echo "Skipping macOS builds (requires macOS host)"
fi

# Windows (requires Windows host or cross toolchain)
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "Building Windows AMD64..."
    cargo build --release --target x86_64-pc-windows-msvc
    cp target/x86_64-pc-windows-msvc/release/tabssh.exe $OUTPUT_DIR/tabssh-windows-amd64.exe
else
    echo "Skipping Windows builds (requires Windows host)"
fi

# Generate checksums
cd $OUTPUT_DIR
sha256sum tabssh-* > checksums.txt

echo "✅ All builds complete!"
ls -lh
