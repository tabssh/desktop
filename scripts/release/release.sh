#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_DIR"

VERSION="${1:-devel}"
BRANCH="${2:-$VERSION}"

is_semver() {
    [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
}

if is_semver "$VERSION"; then
    TAG="v$VERSION"
    RELEASE_NAME="v$VERSION"
else
    TAG="$VERSION"
    RELEASE_NAME="$VERSION"
fi

echo "=== TabSSH Desktop Release ==="
echo "Version:  $VERSION"
echo "Branch:   $BRANCH"
echo "Tag:      $TAG"
echo "Release:  $RELEASE_NAME"
echo ""

COMMIT_ID=$(git rev-parse --short=8 HEAD 2>/dev/null || echo "unknown")
BUILD_DATE=$(date "+%m/%d/%Y at %H:%M:%S")

echo "Commit:   $COMMIT_ID"
echo "Built:    $BUILD_DATE"
echo ""

export TABSSH_BUILD_COMMIT="$COMMIT_ID"
export TABSSH_BUILD_DATE="$BUILD_DATE"

echo "=== Building Release Binary ==="
docker run --rm \
    -v "$PROJECT_DIR:/workspace" \
    -w /workspace \
    -e TABSSH_BUILD_COMMIT="$COMMIT_ID" \
    -e TABSSH_BUILD_DATE="$BUILD_DATE" \
    tabssh-builder cargo build --release

BINARY="target/release/tabssh"
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
echo "Binary size: $SIZE"

mkdir -p releases
RELEASE_BINARY="releases/tabssh-linux-amd64"
cp "$BINARY" "$RELEASE_BINARY"
chmod +x "$RELEASE_BINARY"

echo ""
echo "=== Git Operations ==="

if [ "$BRANCH" != "$(git branch --show-current)" ]; then
    if git show-ref --verify --quiet "refs/heads/$BRANCH"; then
        git checkout "$BRANCH"
    else
        git checkout -b "$BRANCH"
    fi
fi

git add -A
git commit -m "Release $RELEASE_NAME

Build: $COMMIT_ID
Date: $BUILD_DATE" 2>/dev/null || echo "No changes to commit"

git push -u origin "$BRANCH" --force

echo ""
echo "=== GitHub Release ==="

echo "Checking for existing release: $RELEASE_NAME"
if gh release view "$RELEASE_NAME" &>/dev/null; then
    echo "Deleting existing release: $RELEASE_NAME"
    gh release delete "$RELEASE_NAME" --yes
fi

if git tag -l | grep -q "^${TAG}$"; then
    echo "Deleting existing tag: $TAG"
    git tag -d "$TAG"
    git push origin ":refs/tags/$TAG" 2>/dev/null || true
fi

echo "Creating release: $RELEASE_NAME"
gh release create "$TAG" \
    --target "$BRANCH" \
    --title "$RELEASE_NAME" \
    --notes "**TabSSH Desktop - $RELEASE_NAME**

- Commit: \`$COMMIT_ID\`
- Built: $BUILD_DATE
- Branch: $BRANCH

### Download
- \`tabssh-linux-amd64\` - Linux x86_64 binary" \
    "$RELEASE_BINARY"

echo ""
echo "=== Release Complete ==="
echo "Release URL: https://github.com/tabssh/desktop/releases/tag/$TAG"
