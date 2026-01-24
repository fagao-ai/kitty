#!/bin/bash
# Sync version from package.json to Cargo.toml

VERSION=$(node -p "require('./package.json').version")
sed -i "s/^version = .*/version = \"$VERSION\"/" src-tauri/Cargo.toml
echo "Updated Cargo.toml version to $VERSION"

# Create git tag with v prefix
git tag -f "v$VERSION"
echo "Created tag v$VERSION"
