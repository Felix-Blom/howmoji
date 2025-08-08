#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Repository info
REPO="Felix-Blom/howmoji"

# Use user's local bin directory instead of system-wide
INSTALL_DIR="$HOME/.local/bin"

echo -e "${GREEN}🎉 Installing howmoji...${NC}"

# Create the directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# ...existing code...
# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
    linux)
        if [[ $ARCH == "x86_64" ]]; then
            BINARY_NAME="howmoji-x86_64-unknown-linux-gnu"
        else
            echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
            exit 1
        fi
        ;;
    darwin)
        if [[ $ARCH == "arm64" ]]; then
            BINARY_NAME="howmoji-aarch64-apple-darwin"
        elif [[ $ARCH == "x86_64" ]]; then
            BINARY_NAME="howmoji-x86_64-apple-darwin"
        else
            echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
            exit 1
        fi
        ;;
    *)
        echo -e "${RED}❌ Unsupported OS: $OS${NC}"
        echo "Please download manually from: https://github.com/$REPO/releases"
        exit 1
        ;;
esac

# ...existing code...
# Get latest release info
echo -e "${YELLOW}📡 Fetching latest release...${NC}"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)

if [[ -z "$LATEST_RELEASE" ]]; then
    echo -e "${RED}❌ Failed to fetch latest release${NC}"
    exit 1
fi

echo -e "${GREEN}📦 Found release: $LATEST_RELEASE${NC}"

# Build download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/${LATEST_RELEASE}/${BINARY_NAME}"

# Download to temporary location
echo -e "${YELLOW}⬇️  Downloading $BINARY_NAME...${NC}"
TEMP_FILE="/tmp/howmoji"

if ! curl -L -o "$TEMP_FILE" "$DOWNLOAD_URL"; then
    echo -e "${RED}❌ Failed to download binary${NC}"
    exit 1
fi

# Remove quarantine on macOS
if [[ $OS == "darwin" ]]; then
    echo -e "${YELLOW}🔓 Removing quarantine attribute (macOS)...${NC}"
    xattr -d com.apple.quarantine "$TEMP_FILE" 2>/dev/null || true
fi

# Make executable
chmod +x "$TEMP_FILE"

# Install without sudo (since we're using user directory)
echo -e "${YELLOW}📦 Installing to $INSTALL_DIR...${NC}"
mv "$TEMP_FILE" "$INSTALL_DIR/howmoji"

# Verify installation
if command -v howmoji >/dev/null 2>&1; then
    echo -e "${GREEN}✅ howmoji installed successfully!${NC}"
    echo -e "${GREEN}📍 Location: $INSTALL_DIR/howmoji${NC}"
    echo -e "${YELLOW}🚀 Run 'howmoji --help' to get started${NC}"
else
    echo -e "${YELLOW}⚠️  Installation completed but 'howmoji' not found in PATH${NC}"
    echo -e "${YELLOW}   Add $INSTALL_DIR to your PATH by adding this to your shell profile:${NC}"
    echo -e "${YELLOW}   export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
    echo -e "${YELLOW}   Or run directly: $INSTALL_DIR/howmoji${NC}"
fi