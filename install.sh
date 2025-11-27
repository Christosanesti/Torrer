#!/bin/bash

# Torrer Installation Script
# This script installs all required dependencies and builds Torrer from source
# Usage: sudo ./install.sh [--non-interactive]

set -euo pipefail

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="torrer"
NON_INTERACTIVE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse command line arguments
for arg in "$@"; do
    case $arg in
        --non-interactive)
            NON_INTERACTIVE=true
            shift
            ;;
        -h|--help)
            echo "Torrer Installation Script"
            echo "Usage: sudo ./install.sh [--non-interactive]"
            echo ""
            echo "Options:"
            echo "  --non-interactive    Run in non-interactive mode (for CI/CD)"
            echo "  -h, --help          Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $arg${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Error handling
error_exit() {
    log_error "$1"
    exit 1
}

# Check if running as root or with sudo
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error_exit "This script must be run as root or with sudo. Please run: sudo ./install.sh"
    fi
}

# Check if running in interactive terminal
is_interactive() {
    if [[ -t 0 ]] && [[ "$NON_INTERACTIVE" == false ]]; then
        return 0
    else
        return 1
    fi
}

# Check Ubuntu version
check_ubuntu_version() {
    if [[ ! -f /etc/os-release ]]; then
        error_exit "Cannot determine OS version. This script requires Ubuntu 20.04 LTS or later."
    fi
    
    source /etc/os-release
    if [[ "$ID" != "ubuntu" ]]; then
        log_warn "This script is designed for Ubuntu. Proceeding anyway..."
        return
    fi
    
    VERSION_NUM=$(echo "$VERSION_ID" | cut -d. -f1)
    if [[ $VERSION_NUM -lt 20 ]]; then
        error_exit "Ubuntu 20.04 LTS or later is required. Detected: $VERSION_ID"
    fi
    
    log_info "Detected Ubuntu $VERSION_ID"
}

# Install system dependencies
install_system_dependencies() {
    log_info "Updating package lists..."
    apt-get update -qq || error_exit "Failed to update package lists"
    
    log_info "Installing system dependencies..."
    
    local packages=(
        "tor"
        "macchanger"
        "obfs4proxy"
        "iptables"
        "build-essential"
        "curl"
    )
    
    local missing_packages=()
    
    for package in "${packages[@]}"; do
        if dpkg -l | grep -q "^ii  $package "; then
            log_info "Package $package is already installed"
        else
            missing_packages+=("$package")
        fi
    done
    
    if [[ ${#missing_packages[@]} -gt 0 ]]; then
        log_info "Installing packages: ${missing_packages[*]}"
        DEBIAN_FRONTEND=noninteractive apt-get install -y "${missing_packages[@]}" || \
            error_exit "Failed to install system dependencies"
    fi
    
    # Verify installations
    for package in "${packages[@]}"; do
        if ! command -v "${package}" &> /dev/null && ! dpkg -l | grep -q "^ii  $package "; then
            log_warn "Package $package may not be properly installed"
        fi
    done
    
    log_success "System dependencies installed"
}

# Install Rust toolchain
install_rust() {
    log_info "Checking Rust installation..."
    
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        local rust_version=$(rustc --version)
        local cargo_version=$(cargo --version)
        log_info "Rust is already installed: $rust_version"
        log_info "Cargo is already installed: $cargo_version"
        return 0
    fi
    
    log_info "Rust toolchain not found. Installing via rustup..."
    
    # Download and run rustup installer
    local rustup_url="https://sh.rustup.rs"
    local rustup_installer="/tmp/rustup-init.sh"
    
    if ! curl --proto '=https' --tlsv1.2 -sSf "$rustup_url" -o "$rustup_installer"; then
        error_exit "Failed to download Rust installer. Please check your internet connection."
    fi
    
    chmod +x "$rustup_installer"
    
    # Install Rust in non-interactive mode
    if is_interactive; then
        "$rustup_installer" -y || error_exit "Failed to install Rust toolchain"
    else
        RUSTUP_INIT_SKIP_PATH_CHECK=yes "$rustup_installer" -y --default-toolchain stable --profile default || \
            error_exit "Failed to install Rust toolchain"
    fi
    
    # Source cargo environment
    if [[ -f "$HOME/.cargo/env" ]]; then
        source "$HOME/.cargo/env"
    fi
    
    # Verify installation
    if ! command -v rustc &> /dev/null || ! command -v cargo &> /dev/null; then
        log_warn "Rust may not be in PATH. You may need to run: source \$HOME/.cargo/env"
        log_warn "Or restart your terminal session"
    else
        log_success "Rust toolchain installed: $(rustc --version)"
        log_success "Cargo installed: $(cargo --version)"
    fi
}

# Build Torrer
build_torrer() {
    log_info "Building Torrer from source..."
    
    cd "$SCRIPT_DIR" || error_exit "Failed to change to project directory"
    
    if [[ ! -f "Cargo.toml" ]]; then
        error_exit "Cargo.toml not found. Are you in the correct directory?"
    fi
    
    # Ensure cargo is in PATH
    if [[ -f "$HOME/.cargo/env" ]]; then
        source "$HOME/.cargo/env"
    fi
    
    log_info "Running: cargo build --release"
    if cargo build --release; then
        log_success "Torrer built successfully"
    else
        error_exit "Failed to build Torrer. Please check the build errors above."
    fi
    
    local binary_path="$SCRIPT_DIR/target/release/$BINARY_NAME"
    if [[ ! -f "$binary_path" ]]; then
        error_exit "Build succeeded but binary not found at: $binary_path"
    fi
    
    log_success "Binary created at: $binary_path"
}

# Install Torrer binary
install_binary() {
    log_info "Installing Torrer binary to $INSTALL_DIR..."
    
    local binary_path="$SCRIPT_DIR/target/release/$BINARY_NAME"
    
    if [[ ! -f "$binary_path" ]]; then
        error_exit "Binary not found at: $binary_path. Please build first."
    fi
    
    # Copy binary
    cp "$binary_path" "$INSTALL_DIR/$BINARY_NAME" || \
        error_exit "Failed to copy binary to $INSTALL_DIR"
    
    # Set executable permissions
    chmod +x "$INSTALL_DIR/$BINARY_NAME" || \
        error_exit "Failed to set executable permissions"
    
    log_success "Binary installed to $INSTALL_DIR/$BINARY_NAME"
    
    # Verify installation
    if command -v "$BINARY_NAME" &> /dev/null; then
        log_success "Torrer is now available in PATH"
        if "$BINARY_NAME" --version &> /dev/null || "$BINARY_NAME" -V &> /dev/null; then
            log_success "Binary is executable and working"
        fi
    else
        log_warn "Torrer may not be in PATH. You may need to restart your terminal or run: export PATH=\$PATH:$INSTALL_DIR"
    fi
}

# Verify all dependencies
verify_installation() {
    log_info "Verifying installation..."
    
    local all_ok=true
    
    # Verify Tor
    if command -v tor &> /dev/null; then
        local tor_version=$(tor --version 2>&1 | head -n1 || echo "unknown")
        log_success "Tor installed: $tor_version"
    else
        log_error "Tor not found in PATH"
        all_ok=false
    fi
    
    # Verify macchanger
    if command -v macchanger &> /dev/null; then
        local macchanger_version=$(macchanger --version 2>&1 | head -n1 || echo "installed")
        log_success "macchanger installed: $macchanger_version"
    else
        log_error "macchanger not found in PATH"
        all_ok=false
    fi
    
    # Verify Rust
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        log_success "Rust toolchain installed: $(rustc --version)"
    else
        log_warn "Rust may not be in PATH. Run: source \$HOME/.cargo/env"
    fi
    
    # Verify iptables
    if command -v iptables &> /dev/null; then
        log_success "iptables available"
    else
        log_warn "iptables not found in PATH (may still be installed)"
    fi
    
    # Verify Torrer binary
    if command -v "$BINARY_NAME" &> /dev/null; then
        log_success "Torrer binary installed and in PATH"
    else
        log_warn "Torrer binary may not be in PATH"
    fi
    
    if [[ "$all_ok" == true ]]; then
        log_success "All dependencies verified successfully"
        return 0
    else
        log_warn "Some dependencies may not be properly installed"
        return 1
    fi
}

# Main installation function
main() {
    log_info "=========================================="
    log_info "Torrer Installation Script"
    log_info "=========================================="
    
    check_root
    check_ubuntu_version
    
    log_info "Starting installation process..."
    log_info "Installation directory: $INSTALL_DIR"
    log_info "Non-interactive mode: $NON_INTERACTIVE"
    echo ""
    
    # Installation steps
    install_system_dependencies
    echo ""
    
    install_rust
    echo ""
    
    build_torrer
    echo ""
    
    install_binary
    echo ""
    
    verify_installation
    echo ""
    
    log_success "=========================================="
    log_success "Installation completed successfully!"
    log_success "=========================================="
    log_info ""
    log_info "Next steps:"
    log_info "1. Run 'torrer --help' to see available commands"
    log_info "2. Configure Torrer with 'torrer config'"
    log_info "3. Start Tor routing with 'sudo torrer start'"
    log_info ""
    
    if ! command -v "$BINARY_NAME" &> /dev/null; then
        log_warn "Note: You may need to restart your terminal or run:"
        log_warn "  export PATH=\$PATH:$INSTALL_DIR"
    fi
}

# Run main function
main "$@"
