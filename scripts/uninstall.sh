#!/bin/bash

# Uninstall script for Torrer
# Removes Torrer binary and configuration files

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "=== Torrer Uninstaller ==="
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo -e "${RED}Error: This script must be run as root${NC}"
    echo "Usage: sudo ./scripts/uninstall.sh"
    exit 1
fi

# Confirm uninstallation
read -p "Are you sure you want to uninstall Torrer? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Uninstallation cancelled."
    exit 0
fi

echo ""
echo "Uninstalling Torrer..."

# Remove binary
if [ -f "/usr/local/bin/torrer" ]; then
    echo "Removing binary..."
    rm -f /usr/local/bin/torrer
    echo -e "${GREEN}✓ Binary removed${NC}"
else
    echo -e "${YELLOW}⚠ Binary not found${NC}"
fi

# Remove configuration (optional)
read -p "Remove configuration files? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -d "/etc/torrer" ]; then
        echo "Removing configuration..."
        rm -rf /etc/torrer
        echo -e "${GREEN}✓ Configuration removed${NC}"
    fi
    
    if [ -d "/var/lib/torrer" ]; then
        echo "Removing data files..."
        rm -rf /var/lib/torrer
        echo -e "${GREEN}✓ Data files removed${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Configuration files preserved${NC}"
fi

# Remove logs (optional)
read -p "Remove log files? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -d "/var/log/torrer" ]; then
        echo "Removing logs..."
        rm -rf /var/log/torrer
        echo -e "${GREEN}✓ Logs removed${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Log files preserved${NC}"
fi

echo ""
echo -e "${GREEN}✓ Uninstallation complete!${NC}"
echo ""
echo "Note: System dependencies (Tor, macchanger, etc.) were not removed."
echo "To remove them, run: sudo apt-get remove tor macchanger obfs4proxy"

