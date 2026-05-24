#!/bin/bash
set -e

echo "Building Void Fissures App in Release mode..."
cargo build --release

echo "Preparing distribution folder..."
mkdir -p dist

# Copy binary
cp target/release/wf-hub dist/

# Copy static assets
cp -r images dist/

echo "------------------------------------------------"
echo "Distribution ready in the 'dist/' folder!"
echo "To install, copy the 'dist/' content to your target location."
echo "Note: The app will automatically generate 'data/', 'drops/', and 'assets/' folders on its first run."
echo "------------------------------------------------"
