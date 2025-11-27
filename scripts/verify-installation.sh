#!/bin/bash

# Verification script for Torrer installation
# Checks if all dependencies and components are properly installed

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

check_command() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✓${NC} $1 is installed"
        return 0
    else
        echo -e "${RED}✗${NC} $1 is NOT installed"
        return 1
    fi
}

check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} $1 exists"
        return 0
    else
        echo -e "${RED}✗${NC} $1 does NOT exist"
        return 1
    fi
}

echo "=== Torrer Installation Verification ==="
echo ""

errors=0

# Check system dependencies
echo "Checking system dependencies..."
check_command tor || ((errors++))
check_command macchanger || ((errors++))
check_command iptables || ((errors++))
check_command rustc || ((errors++))
check_command cargo || ((errors++))

echo ""
echo "Checking Torrer installation..."
check_command torrer || ((errors++))

echo ""
echo "Checking configuration directories..."
check_file "/etc/torrer/config.toml" || echo -e "${YELLOW}⚠${NC} Config file not found (will be created on first run)"

echo ""
echo "Checking Tor daemon..."
if systemctl is-active --quiet tor 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Tor daemon is running"
else
    echo -e "${YELLOW}⚠${NC} Tor daemon is not running (start with: sudo systemctl start tor)"
fi

echo ""
if [ $errors -eq 0 ]; then
    echo -e "${GREEN}✓ All checks passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Found $errors issue(s)${NC}"
    exit 1
fi

