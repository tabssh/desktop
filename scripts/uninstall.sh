#!/bin/bash
set -e

PREFIX=${PREFIX:-/usr/local}
BINARY_NAME="tabssh"

echo "🗑 Uninstalling TabSSH Desktop from $PREFIX/bin"

if [ -f "$PREFIX/bin/$BINARY_NAME" ]; then
    sudo rm -f "$PREFIX/bin/$BINARY_NAME"
    echo "✅ TabSSH Desktop uninstalled successfully!"
else
    echo "TabSSH Desktop is not installed"
    exit 1
fi
