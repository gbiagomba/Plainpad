#!/usr/bin/env bash
# install.sh
# Cross-platform installer for NIX/UNX/macOS (satisfies RULE 3)

set -e

APP_NAME="program_name"
REPO="gbiagomba/${APP_NAME}"
INSTALL_DIR="/usr/local/bin"

echo "?? Installing ${APP_NAME}..."

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)   OS=linux ;;
        Darwin*)  OS=macos ;;
        *) 
            echo -e "${RED}? Unsupported OS: $(uname -s)${NC}"
            exit 1 
            ;;
    esac
}

# Detect Architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64) ARCH=x64 ;;
        aarch64|arm64) ARCH=aarch64 ;;
        *) 
            echo -e "${RED}? Unsupported architecture: $(uname -m)${NC}"
            exit 1 
            ;;
    esac
}

# Check if command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Install dependencies based on distribution
install_dependencies() {
    echo -e "${YELLOW}?? Checking dependencies...${NC}"
    
    # Detect package manager and install dependencies
    if command_exists apt-get; then
        # Debian/Ubuntu
        echo "Detected: Debian/Ubuntu (apt)"
        sudo apt-get update
        sudo apt-get install -y curl ca-certificates
    elif command_exists yum; then
        # RHEL/CentOS/Fedora (older)
        echo "Detected: RHEL/CentOS (yum)"
        sudo yum install -y curl ca-certificates
    elif command_exists dnf; then
        # Fedora (newer)
        echo "Detected: Fedora (dnf)"
        sudo dnf install -y curl ca-certificates
    elif command_exists pacman; then
        # Arch Linux
        echo "Detected: Arch Linux (pacman)"
        sudo pacman -Sy --noconfirm curl ca-certificates
    elif command_exists zypper; then
        # openSUSE
        echo "Detected: openSUSE (zypper)"
        sudo zypper install -y curl ca-certificates
    elif command_exists apk; then
        # Alpine Linux
        echo "Detected: Alpine Linux (apk)"
        sudo apk add --no-cache curl ca-certificates
    elif command_exists brew; then
        # macOS Homebrew
        echo "Detected: macOS (Homebrew)"
        brew install curl
    else
        echo -e "${YELLOW}??  Could not detect package manager. Assuming dependencies are installed.${NC}"
    fi
}

# Install via Cargo (preferred method)
install_with_cargo() {
    if command_exists cargo; then
        echo -e "${GREEN}?? Installing via Cargo...${NC}"
        cargo install --git "https://github.com/${REPO}"
        return 0
    fi
    return 1
}

# Install from GitHub Release (fallback)
install_from_release() {
    echo -e "${GREEN}?? Downloading from GitHub Releases...${NC}"
    
    detect_os
    detect_arch
    
    BINARY="${APP_NAME}-${OS}-${ARCH}"
    
    # Get latest release version
    LATEST=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$LATEST" ]; then
        echo -e "${RED}? Failed to fetch latest release version${NC}"
        exit 1
    fi
    
    echo "Latest version: ${LATEST}"
    URL="https://github.com/${REPO}/releases/download/${LATEST}/${BINARY}"
    
    echo "Downloading: ${URL}"
    
    # Download binary
    if ! curl -L -f "$URL" -o "$APP_NAME"; then
        echo -e "${RED}? Failed to download binary${NC}"
        exit 1
    fi
    
    # Make executable
    chmod +x "$APP_NAME"
    
    # Install to system path
    if [ -w "$INSTALL_DIR" ]; then
        mv "$APP_NAME" "${INSTALL_DIR}/${APP_NAME}"
    else
        echo -e "${YELLOW}??  Need sudo to install to ${INSTALL_DIR}${NC}"
        sudo mv "$APP_NAME" "${INSTALL_DIR}/${APP_NAME}"
    fi
    
    echo -e "${GREEN}? Binary installed to: ${INSTALL_DIR}/${APP_NAME}${NC}"
}

# Main installation logic
main() {
    install_dependencies
    
    echo ""
    echo "Choose installation method:"
    echo "1) Cargo (recommended for Rust developers)"
    echo "2) GitHub Release binary (recommended for most users)"
    echo "3) Auto-detect (try Cargo first, fallback to binary)"
    echo ""
    read -p "Enter choice [1-3] (default: 3): " choice
    choice=${choice:-3}
    
    case $choice in
        1)
            if ! install_with_cargo; then
                echo -e "${RED}? Cargo not found. Please install Rust first: https://rustup.rs/${NC}"
                exit 1
            fi
            ;;
        2)
            install_from_release
            ;;
        3)
            if ! install_with_cargo; then
                echo -e "${YELLOW}??  Cargo not found, falling back to binary installation${NC}"
                install_from_release
            fi
            ;;
        *)
            echo -e "${RED}? Invalid choice${NC}"
            exit 1
            ;;
    esac
    
    echo ""
    echo -e "${GREEN}? ${APP_NAME} installed successfully!${NC}"
    echo ""
    echo "Run: ${APP_NAME} --help"
}

main