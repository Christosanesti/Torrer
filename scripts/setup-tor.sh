#!/bin/bash

# Setup script for Tor daemon configuration
# Configures Tor for use with Torrer

set -e

TORRC="/etc/tor/torrc"
BACKUP="/etc/tor/torrc.backup.$(date +%Y%m%d_%H%M%S)"

echo "=== Tor Configuration Setup ==="

# Backup existing torrc
if [ -f "$TORRC" ]; then
    echo "Backing up existing torrc to $BACKUP"
    sudo cp "$TORRC" "$BACKUP"
fi

# Create torrc if it doesn't exist
if [ ! -f "$TORRC" ]; then
    echo "Creating new torrc file"
    sudo touch "$TORRC"
fi

# Add Torrer-specific configuration
echo ""
echo "Adding Torrer configuration to torrc..."

# Check if configuration already exists
if grep -q "# Torrer Configuration" "$TORRC"; then
    echo "Torrer configuration already exists in torrc"
else
    sudo tee -a "$TORRC" > /dev/null <<EOF

# Torrer Configuration
ControlPort 9051
CookieAuthentication 1
TransPort 9040
DNSPort 5353
EOF
    echo "Configuration added successfully"
fi

# Restart Tor
echo ""
echo "Restarting Tor daemon..."
sudo systemctl restart tor

# Wait for Tor to start
sleep 2

# Check if Tor is running
if systemctl is-active --quiet tor; then
    echo "✓ Tor daemon is running"
else
    echo "✗ Tor daemon failed to start"
    exit 1
fi

echo ""
echo "✓ Tor configuration complete!"

