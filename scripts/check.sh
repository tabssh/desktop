#!/bin/bash
set -e

echo "🔍 Running checks..."

echo "1. Cargo check..."
cargo check --all-targets

echo "2. Cargo clippy..."
cargo clippy -- -D warnings

echo "3. Cargo fmt check..."
cargo fmt --check

echo "4. Cargo test..."
cargo test

echo "✅ All checks passed!"
