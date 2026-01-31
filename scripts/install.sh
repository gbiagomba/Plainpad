#!/usr/bin/env bash
# install.sh
# Cross-platform installer for Linux/macOS (satisfies RULE 3)

set -euo pipefail

APP_NAME="plainpad"
REPO="gbiagomba/${APP_NAME}"
INSTALL_DIR="/usr/local/bin"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

command_exists() {
    command -v "$1" &> /dev/null
}

detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "macos" ;;
        *)
            echo -e "${RED}? Unsupported OS: $(uname -s)${NC}"
            exit 1
            ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64) echo "x64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *)
            echo -e "${RED}? Unsupported architecture: $(uname -m)${NC}"
            exit 1
            ;;
    esac
}

main() {
    if ! command_exists curl; then
        echo -e "${RED}? curl is required to install ${APP_NAME}.${NC}"
        exit 1
    fi

    echo -e "${GREEN}?? Installing ${APP_NAME}...${NC}"

    OS=$(detect_os)
    ARCH=$(detect_arch)
    BINARY="${APP_NAME}-${OS}-${ARCH}"
    URL="https://github.com/${REPO}/releases/latest/download/${BINARY}"

    tmp_file=$(mktemp -t "${APP_NAME}.XXXXXX")
    echo -e "${YELLOW}?? Downloading ${URL}${NC}"

    if ! curl -L -f "$URL" -o "$tmp_file"; then
        echo -e "${RED}? Failed to download ${BINARY}${NC}"
        exit 1
    fi

    chmod +x "$tmp_file"

    if [ -w "$INSTALL_DIR" ]; then
        mv "$tmp_file" "${INSTALL_DIR}/${APP_NAME}"
    else
        echo -e "${YELLOW}??  Need sudo to install to ${INSTALL_DIR}${NC}"
        sudo mv "$tmp_file" "${INSTALL_DIR}/${APP_NAME}"
    fi

    echo -e "${GREEN}? Binary installed to: ${INSTALL_DIR}/${APP_NAME}${NC}"
    echo -e "${GREEN}? ${APP_NAME} installed successfully!${NC}"
    echo "Launch ${APP_NAME} from your desktop environment or run '${APP_NAME}' in a terminal."
}

main
