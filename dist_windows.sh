#!/bin/bash
set -e

echo "Building Void Fissures App for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

echo "Preparing Windows distribution folder..."
mkdir -p dist_windows

# Copy binary (with .exe extension)
cp target/x86_64-pc-windows-gnu/release/wf-hub.exe dist_windows/

# Copy static assets
cp -r images dist_windows/

echo "------------------------------------------------"
echo "Windows distribution ready in the 'dist_windows/' folder!"
echo "To install, copy the 'dist_windows/' content to your friend's PC."
echo "Note: They will need to keep the 'images/' folder next to the .exe."
echo "------------------------------------------------"
