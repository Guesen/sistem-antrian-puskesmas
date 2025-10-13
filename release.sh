#!/bin/bash

# Script untuk membuat release aplikasi Sistem Antrian Puskesmas
# Usage: ./release.sh <version>
# Example: ./release.sh 1.0.0

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: ./release.sh <version>"
    echo "Example: ./release.sh 1.0.0"
    exit 1
fi

VERSION=$1
TAG="v$VERSION"

echo -e "${GREEN}🚀 Creating release for version $VERSION${NC}"
echo ""

# Check if git is initialized
if [ ! -d ".git" ]; then
    echo -e "${YELLOW}📦 Initializing git repository...${NC}"
    git init
    git add .
    git commit -m "Initial commit - Sistem Antrian Puskesmas v$VERSION"
fi

# Check if there are uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${YELLOW}📝 Committing changes...${NC}"
    git add .
    git commit -m "Release v$VERSION"
fi

# Check if remote exists
if ! git remote get-url origin > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  No git remote found${NC}"
    echo -e "${YELLOW}Please add a remote repository:${NC}"
    echo "git remote add origin https://github.com/USERNAME/sistem-antrian-puskesmas.git"
    echo ""
    echo -e "${YELLOW}Then run this script again.${NC}"
    exit 1
fi

# Update version in tauri.conf.json
echo -e "${GREEN}📝 Updating version in tauri.conf.json...${NC}"
if command -v jq > /dev/null 2>&1; then
    jq ".version = \"$VERSION\"" src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp
    mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json
    git add src-tauri/tauri.conf.json
    git commit -m "Bump version to $VERSION" || true
else
    echo -e "${YELLOW}⚠️  jq not found, skipping version update${NC}"
    echo "Install jq with: brew install jq"
fi

# Create and push tag
echo -e "${GREEN}🏷️  Creating tag $TAG...${NC}"
git tag -a "$TAG" -m "Release $TAG"

echo -e "${GREEN}⬆️  Pushing to remote...${NC}"
git push origin main || git push origin master
git push origin "$TAG"

echo ""
echo -e "${GREEN}✅ Release created successfully!${NC}"
echo ""
echo -e "${YELLOW}📋 Next steps:${NC}"
echo "1. Go to: https://github.com/YOUR_USERNAME/sistem-antrian-puskesmas/actions"
echo "2. Wait for build to complete (~10-15 minutes)"
echo "3. Download installers from Actions artifacts"
echo "   OR"
echo "4. Check Releases page for automatic release:"
echo "   https://github.com/YOUR_USERNAME/sistem-antrian-puskesmas/releases"
echo ""
echo -e "${GREEN}🎉 Done!${NC}"

