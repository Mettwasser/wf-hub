#!/usr/bin/env bash

# Exit immediately if any command fails
set -e

VERSION=$(cargo pkgid | cut -d'#' -f2 | cut -d':' -f2)

# Safety check: Ensure we actually got a version string
if [ -z "$VERSION" ]; then
  echo "Error: Could not extract version from Cargo.toml."
  exit 1
fi

# Optional: Confirmation prompt to prevent accidental releases
echo "Detected version v$VERSION from Cargo.toml."
read -p "Do you want to proceed with creating this release? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Release aborted."
    exit 0
fi

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