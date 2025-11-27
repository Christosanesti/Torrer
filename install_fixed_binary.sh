#!/bin/bash
# Script to install the fixed torrer binary

echo "Installing fixed torrer binary..."
echo "Binary location: /tmp/torrer_build/release/torrer"

if [ ! -f "/tmp/torrer_build/release/torrer" ]; then
    echo "Error: Binary not found. Please build first with:"
    echo "  CARGO_TARGET_DIR=/tmp/torrer_build cargo build --release"
    exit 1
fi

sudo cp /tmp/torrer_build/release/torrer /usr/local/bin/torrer
sudo chmod +x /usr/local/bin/torrer

echo "✓ Binary installed successfully!"
echo ""
echo "Testing installation..."
torrer --version

echo ""
echo "You can now test commands:"
echo "  torrer help"
echo "  torrer info"
echo "  torrer validate"
