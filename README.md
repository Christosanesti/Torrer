# Torrer

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**System-wide Tor routing for Ubuntu with built-in censorship resistance**

Torrer automates the complexity of Tor configuration, DNS leak prevention, and intelligent fallback mechanisms to provide reliable, system-wide Tor routing for users in censored regions.

## 🎯 What is Torrer?

Torrer is a system utility that routes **all** network traffic through the Tor network automatically. Unlike browser-only Tor solutions, Torrer ensures that every application on your system uses Tor, providing comprehensive anonymity and censorship circumvention.

### Key Features

- ✅ **System-wide routing** - All TCP traffic automatically routed through Tor
- ✅ **DNS leak prevention** - Comprehensive DNS routing with verification
- ✅ **Intelligent fallback** - Automatic switching to bridges when standard Tor is blocked
- ✅ **Safety first** - Backup verification, dry-run mode, and recovery procedures
- ✅ **Bridge management** - Easy bridge configuration and testing
- ✅ **Country selection** - Choose exit node country
- ✅ **MAC randomization** - Automatic MAC address randomization
- ✅ **IPv6 control** - Enable/disable IPv6 to prevent leaks

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage Guide](#usage-guide)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)


## 🚀 Quick Start

### Prerequisites

- Ubuntu 20.04 LTS or later
- Root/sudo access (required for iptables configuration)
- Internet connection

### Installation

```bash
# Clone the repository
git clone https://github.com/haphaton/torrer.git
cd torrer

# Run installation script (installs dependencies, builds, and installs)
sudo ./install.sh
```

The installation script will:
1. Install system dependencies (Tor, macchanger, obfs4proxy, iptables, etc.)
2. Install Rust toolchain (if not present)
3. Build Torrer from source
4. Install binary to `/usr/local/bin/torrer`

### Basic Usage

```bash
# Start Tor routing (requires sudo)
sudo torrer start

# Check status
torrer status

# Stop Tor routing
sudo torrer stop

# Restart routing
sudo torrer restart
```

## 📖 Installation Guide

### Method 1: Installation Script (Recommended)

```bash
# Interactive mode (default)
sudo ./install.sh

# Non-interactive mode (for automation)
sudo ./install.sh --non-interactive

# Show help
./install.sh --help
```

### Method 2: Manual Installation

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y tor macchanger obfs4proxy iptables build-essential curl

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build Torrer
cargo build --release

# Install binary
sudo cp target/release/torrer /usr/local/bin/
sudo chmod +x /usr/local/bin/torrer
```

### Verify Installation

```bash
# Check if Torrer is installed
which torrer
torrer --version

# Verify dependencies
torrer validate
```

## 📚 Usage Guide

### Starting Tor Routing

```bash
# Start routing (all traffic goes through Tor)
sudo torrer start

# Check if it's working
torrer status
```

**What happens when you start:**
1. Current iptables rules are backed up (with verification)
2. iptables rules are configured to route TCP traffic through Tor
3. DNS queries are redirected to Tor's DNSPort
4. IPv6 is disabled (to prevent leaks)
5. Connection to Tor network is established

### Stopping Tor Routing

```bash
# Stop routing and restore original network configuration
sudo torrer stop
```

**What happens when you stop:**
1. Tor routing rules are removed
2. Original iptables rules are restored from backup
3. DNS configuration is restored
4. Network returns to normal

### Checking Status

```bash
# View current status
torrer status

# Output example:
# Tor routing: ACTIVE
# Tor connected: Yes
# Circuit established: Yes
# Connection method: Tor
# Exit node: [IP Address]
```

### Bridge Management

Bridges help bypass censorship when standard Tor is blocked:

```bash
# Add a bridge
sudo torrer add-bridge 1.2.3.4:443

# List configured bridges
torrer list-bridges

# Test bridge connectivity
sudo torrer test-bridge 1.2.3.4:443
```

**Getting Bridges:**
- Visit [Tor Project Bridge Database](https://bridges.torproject.org/)
- Use `get-bridges` command (when implemented)
- Community-maintained bridge lists

### Configuration

```bash
# Interactive configuration wizard
sudo torrer config

# Export configuration
sudo torrer export /path/to/config.toml

# Import configuration
sudo torrer import /path/to/config.toml
```

### Advanced Features

```bash
# Set exit node country
sudo torrer set-country CA  # Canada
sudo torrer set-country DE  # Germany
sudo torrer set-country US  # United States

# Randomize MAC address
sudo torrer randomize-mac

# View statistics
torrer stats

# View logs
torrer logs
torrer logs --follow  # Follow in real-time

# Test for leaks
sudo torrer leak-test
```

### Viewing Logs

```bash
# View recent logs
torrer logs

# Follow logs in real-time
torrer logs --follow

# View specific log level
RUST_LOG=debug torrer logs
```

## ⚙️ Configuration

### Configuration File Location

- Default: `~/.config/torrer/config.toml`
- System-wide: `/etc/torrer/config.toml` (if configured)

### Configuration Options

```toml
# Example config.toml
[network]
# Network interface (optional, auto-detect if not specified)
interface = "eth0"

[tor]
# Tor control port
control_port = 9050
# Tor TransPort
transport_port = 9040
# Tor DNSPort
dns_port = 5353

[bridges]
# Pre-configured bridges
bridges = [
    "1.2.3.4:443",
    "5.6.7.8:443 abc123def456"
]

[fallback]
# Enable automatic fallback
enabled = true
# Tor connection timeout (seconds)
tor_timeout = 30
# Bridge connection timeout (seconds)
bridge_timeout = 60
# Max retry attempts
max_retries = 3

[security]
# Enable IPv6 (default: false)
ipv6_enabled = false
# Enable MAC randomization (default: true)
mac_randomization = true

[logging]
# Log level: trace, debug, info, warn, error
level = "info"
```

See `examples/config.example.toml` for a complete example.

## 🔧 Troubleshooting

### Installation Issues

**Problem:** Installation fails with "Permission denied"
```bash
# Solution: Run with sudo
sudo ./install.sh
```

**Problem:** Rust installation fails
```bash
# Solution: Install Rust manually
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Problem:** Build fails
```bash
# Solution: Install build dependencies
sudo apt-get install -y build-essential
```

### Runtime Issues

**Problem:** "Tor daemon not running"
```bash
# Solution: Start Tor service
sudo systemctl start tor
sudo systemctl enable tor  # Enable on boot
```

**Problem:** "Permission denied" when running commands
```bash
# Solution: Commands that modify iptables require sudo
sudo torrer start
sudo torrer stop
```

**Problem:** Connection fails
```bash
# Check Tor status
torrer status

# Check Tor daemon
sudo systemctl status tor

# Try adding bridges
sudo torrer add-bridge <bridge>

# View logs for errors
torrer logs
```

**Problem:** Network breaks after starting
```bash
# Emergency recovery: Restore iptables
sudo iptables-restore < /var/lib/torrer/iptables-backup.rules

# Or use Torrer's stop command
sudo torrer stop

# See docs/TROUBLESHOOTING_IPTABLES.md for detailed recovery
```

### DNS Leak Issues

**Problem:** DNS queries not going through Tor
```bash
# Verify DNS routing
sudo torrer validate

# Test for leaks
sudo torrer leak-test

# Check DNS rules
sudo iptables -t nat -L -n | grep 5353
```

### Bridge Issues

**Problem:** Bridges not working
```bash
# Test bridge connectivity
sudo torrer test-bridge <bridge>

# Check bridge format
torrer list-bridges


- Torrer requires root/sudo access to configure iptables
- All traffic is routed through Tor when active
- Original iptables rules are backed up before changes
- Backup verification prevents corrupted backups from being used

### Reporting Security Issues






**Made with ❤️ for users in censored regions**
