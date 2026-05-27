#!/usr/bin/env bash

# Exit immediately if any command fails
set -e

# Check if a version argument was provided
if [ -z "$1" ]; then
  echo "Error: Please provide a version number."
  echo "Usage: ./release.sh <version>"
  exit 1
fi

VERSION=$1

echo "🔨 Building targets for version v$VERSION..."

# 1. Build the Windows target
echo "--> Building Windows target..."
cargo build --release --target x86_64-pc-windows-gnu

# 2. Build the native (Linux) target
echo "--> Building native Linux target..."
cargo build --release

echo "🚀 Creating GitHub Release v$VERSION and uploading assets..."

# 3. Create the release using the '#' syntax to rename assets on the fly
gh release create "v$VERSION" \
  "./target/release/wf-hub#wf-hub-linux" \
  "./target/x86_64-pc-windows-gnu/release/wf-hub.exe#wf-hub-windows.exe" \
  --generate-notes

echo "🎉 Release v$VERSION successfully created!"