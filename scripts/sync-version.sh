#!/bin/bash
# Sync version from package.json to Cargo.toml and tauri.conf.json

VERSION=$(node -p "require('./package.json').version")

# Update Cargo.toml
sed -i "s/^version = .*/version = \"$VERSION\"/" src-tauri/Cargo.toml
echo "Updated Cargo.toml version to $VERSION"

# Update tauri.conf.json
sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" src-tauri/tauri.conf.json
echo "Updated tauri.conf.json version to $VERSION"

# Add files to git staging
git add src-tauri/Cargo.toml src-tauri/tauri.conf.json
