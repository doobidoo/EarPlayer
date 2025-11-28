#!/bin/bash
# Release script for EarPlayer
# Usage: ./scripts/release.sh [patch|minor|major]

set -e

BUMP_TYPE=${1:-patch}
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}EarPlayer Release Script${NC}"
echo "========================"

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: You have uncommitted changes. Please commit or stash them first.${NC}"
    exit 1
fi

# Read current version
CURRENT_VERSION=$(cat VERSION | tr -d '\n')
echo -e "Current version: ${GREEN}$CURRENT_VERSION${NC}"

# Calculate new version
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

case "$BUMP_TYPE" in
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    patch)
        PATCH=$((PATCH + 1))
        ;;
    *)
        echo -e "${RED}Error: Invalid bump type. Use: patch, minor, or major${NC}"
        exit 1
        ;;
esac

NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}"
echo -e "New version: ${GREEN}$NEW_VERSION${NC}"
echo ""

# Confirm
read -p "Proceed with release v$NEW_VERSION? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Update VERSION file
echo "$NEW_VERSION" > VERSION
echo -e "${GREEN}✓${NC} Updated VERSION file"

# Update Cargo.toml
if [ -f "ear-trainer/Cargo.toml" ]; then
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" ear-trainer/Cargo.toml
    echo -e "${GREEN}✓${NC} Updated ear-trainer/Cargo.toml"
fi

# Update CHANGELOG.md
DATE=$(date +%Y-%m-%d)
CHANGELOG_ENTRY="## [$NEW_VERSION] - $DATE

### Changed
- Version bump to $NEW_VERSION
"

# Insert new entry after [Unreleased]
awk -v entry="$CHANGELOG_ENTRY" '
    /^## \[Unreleased\]/ {
        print
        print ""
        print entry
        next
    }
    {print}
' CHANGELOG.md > CHANGELOG.tmp && mv CHANGELOG.tmp CHANGELOG.md

# Update comparison links
sed -i "s|\[Unreleased\]:.*|[Unreleased]: https://github.com/doobidoo/EarPlayer/compare/v$NEW_VERSION...HEAD|" CHANGELOG.md

# Add link for new version if not exists
if ! grep -q "\[$NEW_VERSION\]:" CHANGELOG.md; then
    echo "[$NEW_VERSION]: https://github.com/doobidoo/EarPlayer/compare/v$CURRENT_VERSION...v$NEW_VERSION" >> CHANGELOG.md
fi

echo -e "${GREEN}✓${NC} Updated CHANGELOG.md"

# Build to verify
echo ""
echo "Building release binary..."
cd ear-trainer
cargo build --release
cd ..
echo -e "${GREEN}✓${NC} Build successful"

# Git operations
echo ""
echo "Committing changes..."
git add VERSION CHANGELOG.md ear-trainer/Cargo.toml ear-trainer/Cargo.lock
git commit -m "chore(release): v$NEW_VERSION"
echo -e "${GREEN}✓${NC} Changes committed"

echo "Creating tag..."
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
echo -e "${GREEN}✓${NC} Tag v$NEW_VERSION created"

echo ""
echo -e "${YELLOW}Ready to push!${NC}"
echo "Run the following commands to publish:"
echo ""
echo "  git push origin main"
echo "  git push origin v$NEW_VERSION"
echo ""
echo "Or push everything with:"
echo "  git push origin main --tags"
echo ""
echo "Then create a GitHub release at:"
echo "  https://github.com/doobidoo/EarPlayer/releases/new?tag=v$NEW_VERSION"
