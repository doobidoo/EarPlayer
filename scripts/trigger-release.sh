#!/bin/bash
# Trigger GitHub Release workflow from command line
# Usage: ./scripts/trigger-release.sh [patch|minor|major] ["release notes"]

set -e

BUMP_TYPE=${1:-patch}
RELEASE_NOTES=${2:-""}

# Validate bump type
if [[ ! "$BUMP_TYPE" =~ ^(patch|minor|major)$ ]]; then
    echo "Error: Invalid bump type. Use: patch, minor, or major"
    exit 1
fi

echo "Triggering release workflow..."
echo "  Bump type: $BUMP_TYPE"
echo "  Notes: ${RELEASE_NOTES:-"(none)"}"
echo ""

# Trigger the workflow using GitHub CLI
gh workflow run release.yml \
    -f version_bump="$BUMP_TYPE" \
    -f release_notes="$RELEASE_NOTES"

echo "✓ Workflow triggered!"
echo ""
echo "Watch progress at:"
echo "  https://github.com/doobidoo/EarPlayer/actions"
echo ""
echo "Or run: gh run watch"
