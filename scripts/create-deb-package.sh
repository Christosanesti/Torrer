#!/bin/bash
# Script to create .deb package for Torrer

set -e

VERSION="${1:-0.1.0}"
PACKAGE_NAME="torrer"
BUILD_DIR="build/deb"
DEBIAN_DIR="$BUILD_DIR/$PACKAGE_NAME/DEBIAN"
INSTALL_DIR="$BUILD_DIR/$PACKAGE_NAME"

echo "Creating .deb package for Torrer $VERSION"

# Clean build directory
rm -rf "$BUILD_DIR"
mkdir -p "$DEBIAN_DIR"
mkdir -p "$INSTALL_DIR/usr/local/bin"
mkdir -p "$INSTALL_DIR/etc/torrer"
mkdir -p "$INSTALL_DIR/usr/share/doc/$PACKAGE_NAME"
mkdir -p "$INSTALL_DIR/usr/share/man/man1"

# Build the binary
echo "Building Torrer..."
cargo build --release
cp target/release/torrer "$INSTALL_DIR/usr/local/bin/"

# Copy configuration template
if [ -f "config.toml.example" ]; then
    cp config.toml.example "$INSTALL_DIR/etc/torrer/config.toml.example"
fi

# Copy documentation
if [ -f "README.md" ]; then
    cp README.md "$INSTALL_DIR/usr/share/doc/$PACKAGE_NAME/"
fi
if [ -f "LICENSE" ]; then
    cp LICENSE "$INSTALL_DIR/usr/share/doc/$PACKAGE_NAME/copyright"
fi

# Create control file
cat > "$DEBIAN_DIR/control" <<EOF
Package: $PACKAGE_NAME
Version: $VERSION
Section: net
Priority: optional
Architecture: amd64
Depends: tor, iptables, macchanger, obfs4proxy
Maintainer: Torrer Contributors <torrer@example.com>
Description: System-wide Tor routing for Ubuntu
 Torrer automates the complexity of Tor configuration, DNS leak prevention,
 and intelligent fallback mechanisms to provide reliable, system-wide Tor
 routing for users in censored regions.
EOF

# Create postinst script
cat > "$DEBIAN_DIR/postinst" <<'EOF'
#!/bin/bash
set -e

# Set up configuration directory
if [ ! -d /etc/torrer ]; then
    mkdir -p /etc/torrer
    chmod 755 /etc/torrer
fi

# Set up config file if it doesn't exist
if [ ! -f /etc/torrer/config.toml ]; then
    if [ -f /etc/torrer/config.toml.example ]; then
        cp /etc/torrer/config.toml.example /etc/torrer/config.toml
        chmod 644 /etc/torrer/config.toml
    fi
fi

# Update man page database
if command -v mandb >/dev/null 2>&1; then
    mandb >/dev/null 2>&1 || true
fi

echo "Torrer installed successfully."
echo "Configuration: /etc/torrer/config.toml"
echo "Run 'sudo torrer start' to begin routing traffic through Tor."
EOF
chmod +x "$DEBIAN_DIR/postinst"

# Create prerm script
cat > "$DEBIAN_DIR/prerm" <<'EOF'
#!/bin/bash
set -e

# Stop Torrer if running
if command -v torrer >/dev/null 2>&1; then
    if sudo torrer status 2>/dev/null | grep -q "ACTIVE"; then
        echo "Stopping Torrer..."
        sudo torrer stop || true
    fi
fi
EOF
chmod +x "$DEBIAN_DIR/prerm"

# Build the package
echo "Building .deb package..."
dpkg-deb --build "$BUILD_DIR/$PACKAGE_NAME" "${PACKAGE_NAME}_${VERSION}_amd64.deb"

echo "Package created: ${PACKAGE_NAME}_${VERSION}_amd64.deb"
echo "Install with: sudo dpkg -i ${PACKAGE_NAME}_${VERSION}_amd64.deb"

