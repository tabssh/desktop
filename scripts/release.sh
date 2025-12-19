#!/bin/bash
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

echo "🚀 Creating release $VERSION"

# Run tests
echo "Running tests..."
cargo test

# Build all targets
echo "Building all targets..."
make release VERSION=$VERSION

# Generate checksums
echo "Generating checksums..."
cd releases
sha256sum tabssh-* > checksums.txt

# Create release notes
cat > RELEASE_NOTES.md << EOF
# TabSSH Desktop $VERSION

Cross-platform SSH/SFTP client

## Features
- Browser-style tabs
- Port forwarding
- SFTP browser
- 8 themes
- SSH config import

## Downloads
$(ls -1 tabssh-* | grep -v checksums | sed 's/^/- /')

## Checksums
\`\`\`
$(cat checksums.txt)
\`\`\`
EOF

echo "✅ Release $VERSION ready in releases/"
