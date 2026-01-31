# 🦀 RUST CLI PROJECT TEMPLATE (Universal Agentic Pipeline)

## 🤖 AGENT INSTRUCTIONS - CRITICAL REQUIREMENTS

**Read this section FIRST before generating any code. These rules are MANDATORY and override all other considerations:**

### 🔒 Rule 1: Security & Quality Assurance

- **ALWAYS use secure code practices by default**
- **MUST compile and test ALL code before delivering**
- Run `cargo test`, `cargo clippy`, and `cargo fmt` to verify quality
- Never deliver untested or broken code

### 📚 Rule 2: Documentation & Project Files

**MUST create/update/maintain the following files:**

- ✅ `README.md` (using template provided below)
- ✅ `Makefile` (choose Lite or Pro based on project complexity)
- ✅ `Dockerfile` (using template provided below)
- ✅ `CHANGELOG.md` (Semantic Versioning format)
- ✅ `.gitignore` (add test files like `output.txt`, `test_output.html`, etc.)
- ✅ Remove any test artifacts already pushed to git

### 🔧 Rule 3: Cross-Platform Install Scripts

**IF an install script does not exist, CREATE ONE that:**

- Supports major platforms: **Linux, Unix, macOS, Windows**
- Uses system package managers FIRST:
    - macOS: `brew`
    - Linux: `apt`, `yum`, `dnf`, `pacman`
    - Windows: `winget`, `scoop`, `choco`
- Falls back to language-specific package managers:
    - Rust: `cargo install`
    - Python: `pip install`
    - Go: `go install`
- Must work across distributions (Ubuntu, Debian, Fedora, Arch, etc.)

### 🚀 Rule 4: GitHub Actions CI/CD

**MUST create GitHub workflows that:**

- ✅ Compile on all major platforms: Linux, macOS, Windows
- ✅ Support architectures: **x64** AND **arm64/aarch64**
- ✅ Auto-publish releases on version tags (`v*`)
- ✅ Generate binaries with naming convention:
    - `TOOLNAME-linux-x64`, `TOOLNAME-linux-aarch64`
    - `TOOLNAME-macos-x64`, `TOOLNAME-macos-aarch64`
    - `TOOLNAME-windows-x64.exe`, `TOOLNAME-windows-aarch64.exe`
- Use templates provided below (choose standard or reusable workflow)

### 📊 Rule 5: Semantic Versioning & Tracking

**MUST follow Semantic Versioning 2.0.0:**

- Format: `MAJOR.MINOR.PATCH` (e.g., `1.0.0`, `2.3.1`)
- Auto-increment version based on changes:
    - **MAJOR**: Breaking changes
    - **MINOR**: New features (backward compatible)
    - **PATCH**: Bug fixes (backward compatible)
- **CREATE `.version-tracking.md`** to log all version changes:

```markdown
# Version Tracking

## v1.0.0 (2025-12-05)
- Initial release
- Feature: Multi-target scanning
- Feature: JSON output support

## v1.1.0 (2025-12-10)
- Added: CSV export option
- Fixed: Memory leak in parser
```

### 🌳 Rule 6: Git Branch Strategy

**MUST implement proper branching:**

- Create `main` branch (production-ready code)
- Create `dev` branch (active development)
- **ALWAYS switch to `dev` after initialization**
- Merge to `main` only for releases

```bash
git branch dev
git checkout dev
# or
git checkout -b dev
```

### 🏷️ Rule 7: Git Commits, Tags & Releases

**MUST follow this workflow:**

1. ✅ Commit all changes with descriptive messages
2. ✅ Push to remote repository
3. ✅ Tag the version: `git tag v1.0.0`
4. ✅ Push tags: `git push origin v1.0.0`
5. ✅ Create GitHub Release (automated via workflow)

**Sample workflow:**

```bash
git add .
git commit -m "feat: Add CSV export functionality"
git push origin dev
git tag v1.1.0
git push origin v1.1.0
# GitHub Actions will automatically create release
```

---

## 📋 AGENT CHECKLIST (Use this for EVERY project)

Before delivering code, verify:

- [ ] All code compiles without errors (`cargo build --release`)
- [ ] All tests pass (`cargo test`)
- [ ] Linting passes (`cargo clippy -- -D warnings`)
- [ ] Formatting applied (`cargo fmt --all`)
- [ ] README.md created/updated
- [ ] Makefile created
- [ ] Dockerfile created
- [ ] CHANGELOG.md created/updated
- [ ] .gitignore includes test artifacts
- [ ] GitHub Actions workflow created
- [ ] Install script created (if needed)
- [ ] Version updated in Cargo.toml
- [ ] .version-tracking.md updated
- [ ] Git branches (main/dev) created
- [ ] Code committed to dev branch
- [ ] Version tagged
- [ ] Ready for release

---

# 📦 PROJECT TEMPLATE

## Program Structure

I need a Rust program named `PROGRAM_NAME` that uses `PACKAGE_LIB`.

**Purpose:** ENTER_PURPOSE_HERE

**Additional Details:** ADD_DETAILS_HERE

**Requirements:**

- Use the **latest versions** of all dependencies
- Write the **full/complete program** (no placeholders)
- Include `.gitignore` file
- Use **GPL-3.0 license**

---

## Cargo.toml Template

```toml
[package]
name = "program_name"
version = "1.0.0"
authors = ["Gilles Biagomba <gilles.infosec@gmail.com>"]
edition = "2021"
license = "GPL-3.0"
description = "SHORT_DESCRIPTION_HERE"
repository = "https://github.com/gbiagomba/program_name"
keywords = ["cli", "security", "tool", "automation"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
clap = { version = "4.5", features = ["derive"] }
# ADD_MORE_DEPENDENCIES_HERE

[[bin]]
name = "program_name"
path = "src/main.rs"
```

---

## .gitignore Template

**AGENT: MUST add test artifacts to prevent accidental commits**

```gitignore
# Rust
/target/
**/*.rs.bk
Cargo.lock

# IDEs
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Test artifacts (ADD MORE AS NEEDED)
output.txt
test_output.html
test_output.json
test_output.csv
*.log
*.tmp
temp/
test_data/

# Build artifacts
*.exe
*.dll
*.so
*.dylib
```

---

## GitHub Actions Workflows

**AGENT: Choose ONE of the following based on project needs:**

### Option A: Standard Workflow (Single Project)

<details> <summary>Click to expand: .github/workflows/ci-release.yml</summary>

```yaml
# .github/workflows/ci-release.yml
#
# AGENT INSTRUCTIONS:
# 1. Replace PROJECT_TYPE with "rust" or "python"
# 2. Replace BINARY_NAME with actual binary name from Cargo.toml [[bin]]
# 3. Ensure CHANGELOG.md exists with format: "## v1.0.0"
# 4. This workflow satisfies RULE 4 (multi-platform builds + releases)

name: CI and Release

on:
  push:
    branches:
      - '**'          # Run CI on all branches (satisfies RULE 6)
    tags:
      - 'v*'          # Run on version tags (satisfies RULE 7)
  pull_request:
    branches:
      - '**'

env:
  PROJECT_TYPE: rust  # or "python"
  BINARY_NAME: program_name
  PYTHON_PACKAGE_NAME: program_name

jobs:
  build-test:
    name: Build and Test (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-13, macos-14, windows-latest]

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      # RUST PATH (satisfies RULE 1 - security & testing)
      - name: Set up Rust
        if: env.PROJECT_TYPE == 'rust'
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        if: env.PROJECT_TYPE == 'rust'
        uses: swatinem/rust-cache@v2

      - name: Rust Format (cargo fmt)
        if: env.PROJECT_TYPE == 'rust'
        run: cargo fmt --all -- --check

      - name: Rust Lints (cargo clippy)
        if: env.PROJECT_TYPE == 'rust'
        run: cargo clippy --all-targets -- -D warnings

      - name: Rust Build (release)
        if: env.PROJECT_TYPE == 'rust'
        run: cargo build --release

      - name: Rust Tests
        if: env.PROJECT_TYPE == 'rust'
        run: cargo test --all --no-fail-fast

      - name: Upload Rust binary artifact
        if: env.PROJECT_TYPE == 'rust' && startsWith(github.ref, 'refs/tags/v')
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-${{ runner.os }}-${{ runner.arch }}
          path: |
            target/release/${{ env.BINARY_NAME }}*

      # PYTHON PATH
      - name: Set up Python
        if: env.PROJECT_TYPE == 'python'
        uses: actions/setup-python@v5
        with:
          python-version: '3.x'
          cache: 'pip'

      - name: Install Python dependencies
        if: env.PROJECT_TYPE == 'python'
        run: |
          if [ -f "requirements-dev.txt" ]; then
            pip install -r requirements-dev.txt
          elif [ -f "requirements.txt" ]; then
            pip install -r requirements.txt
          fi

      - name: Python Tests
        if: env.PROJECT_TYPE == 'python'
        run: |
          if [ -d "tests" ]; then
            pip install pytest
            pytest
          fi

      - name: Build Python package
        if: env.PROJECT_TYPE == 'python' && startsWith(github.ref, 'refs/tags/v')
        run: |
          pip install build
          python -m build

      - name: Upload Python package artifacts
        if: env.PROJECT_TYPE == 'python' && startsWith(github.ref, 'refs/tags/v')
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.PYTHON_PACKAGE_NAME }}-dist
          path: dist/*

  # MULTI-ARCH BUILDS (satisfies RULE 4 - x64 + arm64)
  linux-arm64:
    name: Linux aarch64 (cross)
    runs-on: ubuntu-latest
    if: env.PROJECT_TYPE == 'rust' && startsWith(github.ref, 'refs/tags/v')
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-unknown-linux-gnu
      - name: Install cross
        run: cargo install cross --git https://github.com/cross-rs/cross
      - name: Build (cross)
        run: cross build --release --target aarch64-unknown-linux-gnu
      - name: Upload binary
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-linux-aarch64
          path: target/aarch64-unknown-linux-gnu/release/${{ env.BINARY_NAME }}*

  macos-arm64:
    name: macOS aarch64 (native)
    runs-on: macos-latest
    if: env.PROJECT_TYPE == 'rust' && startsWith(github.ref, 'refs/tags/v')
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-darwin
      - name: Build
        run: cargo build --release --target aarch64-apple-darwin
      - name: Upload binary
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-macos-aarch64
          path: target/aarch64-apple-darwin/release/${{ env.BINARY_NAME }}*

  windows-arm64:
    name: Windows aarch64 (cross)
    runs-on: windows-latest
    if: env.PROJECT_TYPE == 'rust' && startsWith(github.ref, 'refs/tags/v')
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-pc-windows-msvc
      - name: Build
        run: cargo build --release --target aarch64-pc-windows-msvc
      - name: Upload binary
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-windows-aarch64
          path: target/aarch64-pc-windows-msvc/release/${{ env.BINARY_NAME }}.exe

  # RELEASE JOB (satisfies RULE 7 - auto-releases)
  release:
    name: Release on tag
    runs-on: ubuntu-latest
    needs: [build-test, linux-arm64, macos-arm64, windows-arm64]
    if: startsWith(github.ref, 'refs/tags/v')
    permissions:
      contents: write

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Download artifacts
        uses: actions/download-artifact@v4
        with:
          pattern: "*"
          merge-multiple: true

      - name: Create checksums
        if: env.PROJECT_TYPE == 'rust'
        run: |
          for f in ${BINARY_NAME}-*; do
            if [ -f "$f" ]; then
              sha256sum "$f" > "$f.sha256"
            fi
          done
        env:
          BINARY_NAME: ${{ env.BINARY_NAME }}

      - name: Extract release notes (satisfies RULE 5 - CHANGELOG)
        id: extract_notes
        run: |
          VERSION="${GITHUB_REF#refs/tags/}"
          echo "version=$VERSION" >> $GITHUB_OUTPUT
          
          if [ -f CHANGELOG.md ]; then
            awk "/^## $VERSION/,/^## /{if (/^## $VERSION/) f=1; else if (/^## /) f=0; if (f && !/^## $VERSION/) print}" CHANGELOG.md > release_notes.md
          fi
          
          if [ ! -s release_notes.md ]; then
            git tag -l --format='%(contents)' "$VERSION" > release_notes.md
          fi

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          draft: false
          prerelease: false
          generate_release_notes: true
          body_path: release_notes.md
          files: |
            ${{ env.BINARY_NAME }}-*
            dist/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

</details>

### Option B: Reusable Workflow (Multi-Project)

<details> <summary>Click to expand: Reusable workflow templates</summary>

**Step 1: Create reusable workflow**

`.github/workflows/ci-release-reusable.yml` - (See original template in your document)

**Step 2: Create per-project entry point**

`.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: ['**']
    tags: ['v*']
  pull_request:
    branches: ['**']

jobs:
  ci:
    uses: ./.github/workflows/ci-release-reusable.yml
    with:
      project_type: rust
      binary_name: program_name
      enable_cross: true
```

</details>

### Dependabot Configuration

**.github/dependabot.yml** (satisfies RULE 2 - maintain project files)

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
```

---

## Dockerfile Template

**AGENT: Replace `program_name` with actual binary name**

```dockerfile
# Dockerfile
# Satisfies RULE 2 - maintain Dockerfile

FROM rust:slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release || true

COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG APP_NAME=program_name
ARG APP_VERSION=1.0.0
ARG APP_MAINTAINER="Gilles Biagomba <gilles.infosec@gmail.com>"

LABEL maintainer="${APP_MAINTAINER}"
LABEL version="${APP_VERSION}"
LABEL org.opencontainers.image.title="${APP_NAME}"

RUN useradd -m -s /usr/sbin/nologin ${APP_NAME}
WORKDIR /home/${APP_NAME}
USER ${APP_NAME}

COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}

ENTRYPOINT ["program_name"]
CMD ["--help"]
```

---

## Makefile Templates

**AGENT: Choose based on project complexity**

### Option A: Makefile Lite (Simple projects)

```makefile
# Makefile (LITE)
# Satisfies RULE 2 - maintain Makefile
# Satisfies RULE 1 - test before delivery

APP_NAME := program_name
CARGO := cargo

.PHONY: all build release install run test clean

all: release

build:
	$(CARGO) build --release

release: build

install:
	$(CARGO) install --path .

run:
	$(CARGO) run --release -- $(ARGS)

test:
	$(CARGO) test

clean:
	$(CARGO) clean
```

### Option B: Makefile Pro (Complex/flagship projects)

```makefile
# Makefile (PRO)
# Satisfies RULE 1 - comprehensive testing & quality checks

APP_NAME := program_name
CARGO := cargo

.PHONY: all build release install test check fmt clippy run clean ci

all: release

build:
	$(CARGO) build

release:
	$(CARGO) build --release

install:
	$(CARGO) install --path .

test:
	$(CARGO) test

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt --all

clippy:
	$(CARGO) clippy --all-targets -- -D warnings

run:
	$(CARGO) run -- $(ARGS)

clean:
	$(CARGO) clean

# CI target (satisfies RULE 1 - test everything)
ci: fmt clippy test release
	@echo "✅ All CI checks passed!"
```

---

## Install Script Template

**AGENT: Create BOTH scripts if they don't exist (RULE 3)**

### Unix/Linux/macOS: `install.sh`

```bash
#!/usr/bin/env bash
# install.sh
# Cross-platform installer for NIX/UNX/macOS (satisfies RULE 3)

set -e

APP_NAME="program_name"
REPO="gbiagomba/${APP_NAME}"
INSTALL_DIR="/usr/local/bin"

echo "🚀 Installing ${APP_NAME}..."

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
            echo -e "${RED}❌ Unsupported OS: $(uname -s)${NC}"
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
            echo -e "${RED}❌ Unsupported architecture: $(uname -m)${NC}"
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
    echo -e "${YELLOW}📦 Checking dependencies...${NC}"
    
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
        echo -e "${YELLOW}⚠️  Could not detect package manager. Assuming dependencies are installed.${NC}"
    fi
}

# Install via Cargo (preferred method)
install_with_cargo() {
    if command_exists cargo; then
        echo -e "${GREEN}📦 Installing via Cargo...${NC}"
        cargo install --git "https://github.com/${REPO}"
        return 0
    fi
    return 1
}

# Install from GitHub Release (fallback)
install_from_release() {
    echo -e "${GREEN}📥 Downloading from GitHub Releases...${NC}"
    
    detect_os
    detect_arch
    
    BINARY="${APP_NAME}-${OS}-${ARCH}"
    
    # Get latest release version
    LATEST=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$LATEST" ]; then
        echo -e "${RED}❌ Failed to fetch latest release version${NC}"
        exit 1
    fi
    
    echo "Latest version: ${LATEST}"
    URL="https://github.com/${REPO}/releases/download/${LATEST}/${BINARY}"
    
    echo "Downloading: ${URL}"
    
    # Download binary
    if ! curl -L -f "$URL" -o "$APP_NAME"; then
        echo -e "${RED}❌ Failed to download binary${NC}"
        exit 1
    fi
    
    # Make executable
    chmod +x "$APP_NAME"
    
    # Install to system path
    if [ -w "$INSTALL_DIR" ]; then
        mv "$APP_NAME" "${INSTALL_DIR}/${APP_NAME}"
    else
        echo -e "${YELLOW}⚠️  Need sudo to install to ${INSTALL_DIR}${NC}"
        sudo mv "$APP_NAME" "${INSTALL_DIR}/${APP_NAME}"
    fi
    
    echo -e "${GREEN}✅ Binary installed to: ${INSTALL_DIR}/${APP_NAME}${NC}"
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
                echo -e "${RED}❌ Cargo not found. Please install Rust first: https://rustup.rs/${NC}"
                exit 1
            fi
            ;;
        2)
            install_from_release
            ;;
        3)
            if ! install_with_cargo; then
                echo -e "${YELLOW}⚠️  Cargo not found, falling back to binary installation${NC}"
                install_from_release
            fi
            ;;
        *)
            echo -e "${RED}❌ Invalid choice${NC}"
            exit 1
            ;;
    esac
    
    echo ""
    echo -e "${GREEN}✅ ${APP_NAME} installed successfully!${NC}"
    echo ""
    echo "Run: ${APP_NAME} --help"
}

main
```

---

### Windows: `install.ps1` (PowerShell)

```powershell
# install.ps1
# Windows installer for program_name (satisfies RULE 3)

param(
    [string]$InstallMethod = "auto"
)

$ErrorActionPreference = "Stop"

$APP_NAME = "program_name"
$REPO = "gbiagomba/$APP_NAME"
$INSTALL_DIR = "$env:ProgramFiles\$APP_NAME"

Write-Host "🚀 Installing $APP_NAME..." -ForegroundColor Cyan

# Detect Architecture
function Get-SystemArchitecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x64" }
        "ARM64" { return "aarch64" }
        default {
            Write-Host "❌ Unsupported architecture: $arch" -ForegroundColor Red
            exit 1
        }
    }
}

# Check if command exists
function Test-CommandExists {
    param($Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

# Install dependencies via package managers
function Install-Dependencies {
    Write-Host "📦 Checking dependencies..." -ForegroundColor Yellow
    
    # Try winget first (Windows 10+)
    if (Test-CommandExists winget) {
        Write-Host "Detected: winget" -ForegroundColor Green
        # Typically curl is built-in on Windows 10+
    }
    # Try Chocolatey
    elseif (Test-CommandExists choco) {
        Write-Host "Detected: Chocolatey" -ForegroundColor Green
        choco install curl -y
    }
    # Try Scoop
    elseif (Test-CommandExists scoop) {
        Write-Host "Detected: Scoop" -ForegroundColor Green
        scoop install curl
    }
    else {
        Write-Host "⚠️  No package manager detected. Assuming dependencies are installed." -ForegroundColor Yellow
    }
}

# Install via Cargo
function Install-WithCargo {
    if (Test-CommandExists cargo) {
        Write-Host "📦 Installing via Cargo..." -ForegroundColor Green
        cargo install --git "https://github.com/$REPO"
        return $true
    }
    return $false
}

# Install from GitHub Release
function Install-FromRelease {
    Write-Host "📥 Downloading from GitHub Releases..." -ForegroundColor Green
    
    $arch = Get-SystemArchitecture
    $BINARY = "$APP_NAME-windows-$arch.exe"
    
    # Get latest release
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest"
        $LATEST = $response.tag_name
        Write-Host "Latest version: $LATEST" -ForegroundColor Cyan
    }
    catch {
        Write-Host "❌ Failed to fetch latest release version" -ForegroundColor Red
        exit 1
    }
    
    $URL = "https://github.com/$REPO/releases/download/$LATEST/$BINARY"
    Write-Host "Downloading: $URL" -ForegroundColor Cyan
    
    # Download binary
    $tempFile = "$env:TEMP\$APP_NAME.exe"
    try {
        Invoke-WebRequest -Uri $URL -OutFile $tempFile -UseBasicParsing
    }
    catch {
        Write-Host "❌ Failed to download binary" -ForegroundColor Red
        exit 1
    }
    
    # Create install directory
    if (-not (Test-Path $INSTALL_DIR)) {
        New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
    }
    
    # Move to install location
    $finalPath = "$INSTALL_DIR\$APP_NAME.exe"
    Move-Item -Path $tempFile -Destination $finalPath -Force
    
    Write-Host "✅ Binary installed to: $finalPath" -ForegroundColor Green
    
    # Add to PATH if not already there
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if ($currentPath -notlike "*$INSTALL_DIR*") {
        Write-Host "Adding $INSTALL_DIR to system PATH..." -ForegroundColor Yellow
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$currentPath;$INSTALL_DIR",
            "Machine"
        )
        Write-Host "✅ Added to PATH. Please restart your terminal." -ForegroundColor Green
    }
}

# Main installation logic
function Main {
    # Check for admin privileges
    $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    
    if (-not $isAdmin) {
        Write-Host "⚠️  This script requires Administrator privileges." -ForegroundColor Yellow
        Write-Host "Please run PowerShell as Administrator and try again." -ForegroundColor Yellow
        exit 1
    }
    
    Install-Dependencies
    
    Write-Host ""
    Write-Host "Choose installation method:" -ForegroundColor Cyan
    Write-Host "1) Cargo (recommended for Rust developers)"
    Write-Host "2) GitHub Release binary (recommended for most users)"
    Write-Host "3) Auto-detect (try Cargo first, fallback to binary)"
    Write-Host ""
    
    if ($InstallMethod -eq "auto") {
        $choice = Read-Host "Enter choice [1-3] (default: 3)"
        if ([string]::IsNullOrWhiteSpace($choice)) { $choice = "3" }
    }
    else {
        $choice = $InstallMethod
    }
    
    switch ($choice) {
        "1" {
            if (-not (Install-WithCargo)) {
                Write-Host "❌ Cargo not found. Please install Rust first: https://rustup.rs/" -ForegroundColor Red
                exit 1
            }
        }
        "2" {
            Install-FromRelease
        }
        "3" {
            if (-not (Install-WithCargo)) {
                Write-Host "⚠️  Cargo not found, falling back to binary installation" -ForegroundColor Yellow
                Install-FromRelease
            }
        }
        default {
            Write-Host "❌ Invalid choice" -ForegroundColor Red
            exit 1
        }
    }
    
    Write-Host ""
    Write-Host "✅ $APP_NAME installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run: $APP_NAME --help" -ForegroundColor Cyan
}

Main
```

---

### Windows: `install.bat` (Batch - Simple Alternative)

```batch
@echo off
REM install.bat
REM Simple Windows installer for program_name (satisfies RULE 3)

setlocal enabledelayedexpansion

set APP_NAME=program_name
set REPO=gbiagomba/%APP_NAME%
set INSTALL_DIR=%ProgramFiles%\%APP_NAME%

echo 🚀 Installing %APP_NAME%...
echo.

REM Check for admin privileges
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ This script requires Administrator privileges.
    echo Please run as Administrator and try again.
    pause
    exit /b 1
)

REM Detect architecture
if "%PROCESSOR_ARCHITECTURE%"=="AMD64" (
    set ARCH=x64
) else if "%PROCESSOR_ARCHITECTURE%"=="ARM64" (
    set ARCH=aarch64
) else (
    echo ❌ Unsupported architecture: %PROCESSOR_ARCHITECTURE%
    pause
    exit /b 1
)

REM Check if cargo exists
where cargo >nul 2>&1
if %errorlevel% equ 0 (
    echo 📦 Installing via Cargo...
    cargo install --git https://github.com/%REPO%
    echo.
    echo ✅ %APP_NAME% installed successfully!
    echo Run: %APP_NAME% --help
    pause
    exit /b 0
)

echo ⚠️  Cargo not found, installing from GitHub Release...
echo.

REM Download binary from GitHub Release
set BINARY=%APP_NAME%-windows-%ARCH%.exe
echo Downloading %BINARY%...

REM Use PowerShell to download (works on Windows 7+)
powershell -Command "& {$tag = (Invoke-RestMethod 'https://api.github.com/repos/%REPO%/releases/latest').tag_name; Invoke-WebRequest -Uri \"https://github.com/%REPO%/releases/download/$tag/%BINARY%\" -OutFile \"%TEMP%\%APP_NAME%.exe\" -UseBasicParsing}"

if not exist "%TEMP%\%APP_NAME%.exe" (
    echo ❌ Failed to download binary
    pause
    exit /b 1
)

REM Create install directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Move binary
move /y "%TEMP%\%APP_NAME%.exe" "%INSTALL_DIR%\%APP_NAME%.exe"

echo ✅ Binary installed to: %INSTALL_DIR%\%APP_NAME%.exe

REM Add to PATH
echo %PATH% | find /i "%INSTALL_DIR%" >nul
if %errorlevel% neq 0 (
    echo Adding to system PATH...
    setx /M PATH "%PATH%;%INSTALL_DIR%"
    echo ✅ Added to PATH. Please restart your terminal.
)

echo.
echo ✅ %APP_NAME% installed successfully!
echo Run: %APP_NAME% --help
pause
```

---

## README Template

**AGENT: Fill in ALL sections marked with TEMPLATE_INSTRUCTIONS**

````markdown
# PROGRAM_NAME

> **VERSION:** 1.0.0  
> **DESCRIPTION:** SHORT_PROJECT_DESCRIPTION  
> **AUTHOR:** Gilles Biagomba  
> **LICENSE:** GPL-3.0

---

## 🧬 Background / Lore

**TEMPLATE INSTRUCTIONS FOR AGENT:**  
Replace with 2–4 sentences describing the theme, inspiration, or purpose.  
Examples: "Inspired by Norse mythology...", "Built to automate...", "A tactical tool for..."

---

## 📚 Table of Contents

- [Background / Lore](#-background--lore)
- [Features](#-features)
- [Installation](#-installation)
  - [Using GitHub Releases](#using-github-releases)
  - [Using Cargo](#using-cargo)
  - [Compiling From Source](#compiling-from-source)
- [Flags](#-flags)
- [Usage](#-usage)
  - [Running Tests](#running-tests)
  - [Using Docker](#using-docker)
  - [Using the Makefile](#using-the-makefile)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🚀 Features

**TEMPLATE INSTRUCTIONS FOR AGENT:**  
List 4–8 key capabilities. Examples:

- ✅ Multi-platform support (Linux, macOS, Windows)
- ✅ Cross-architecture binaries (x64, ARM64)
- ✅ Fast, parallel execution
- ✅ JSON/CSV/Markdown output formats
- ✅ AI-assisted analysis (if applicable)
- ✅ Comprehensive error handling
- ✅ Zero dependencies (or minimal dependencies)
- ✅ Docker support

---

## 🛠 Installation

### 📦 Using GitHub Releases

Download precompiled binaries:

| Platform | Architecture | Binary |
|----------|-------------|--------|
| Linux | x64 | `program_name-linux-x64` |
| Linux | ARM64 | `program_name-linux-aarch64` |
| macOS | x64 | `program_name-macos-x64` |
| macOS | ARM64 | `program_name-macos-aarch64` |
| Windows | x64 | `program_name-windows-x64.exe` |
| Windows | ARM64 | `program_name-windows-aarch64.exe` |

**Install (Linux/macOS):**
```bash
chmod +x program_name-*
sudo mv program_name-* /usr/local/bin/program_name
```

**Install (Windows):**

```powershell
Move-Item program_name-*.exe C:\Windows\System32\program_name.exe
```

---

### 📚 Using Cargo

```bash
cargo install --git https://github.com/gbiagomba/program_name
```

---

### 🧱 Compiling From Source

```bash
git clone https://github.com/gbiagomba/program_name
cd program_name
cargo build --release
# Binary: target/release/program_name
```

### 🔧 Using Install Scripts

**Linux/macOS/Unix:**
```bash
curl -sSL https://raw.githubusercontent.com/gbiagomba/program_name/main/install.sh | bash
# Or download and run locally:
wget https://raw.githubusercontent.com/gbiagomba/program_name/main/install.sh
chmod +x install.sh
./install.sh
```

**Windows (PowerShell - Run as Administrator):**
```powershell
irm https://raw.githubusercontent.com/gbiagomba/program_name/main/install.ps1 | iex
# Or download and run locally:
Invoke-WebRequest -Uri https://raw.githubusercontent.com/gbiagomba/program_name/main/install.ps1 -OutFile install.ps1
.\install.ps1
```

**Windows (Batch - Run as Administrator):**
```batch
curl -L https://raw.githubusercontent.com/gbiagomba/program_name/main/install.bat -o install.bat
install.bat
```

---

## 🔧 Flags

**TEMPLATE INSTRUCTIONS FOR AGENT:**  
Auto-populate from Clap definitions. Example:

```
-h, --help              Show help information
-v, --version           Show version
-u, --url <URL>         Target URL
-f, --file <FILE>       Input file
-o, --output <FILE>     Output file
-X, --method <VERB>     HTTP method (GET, POST, ALL)
--format <FORMAT>       Output format (json, csv, markdown)
```

---

## 📈 Usage

### Basic Examples

```bash
# Show help
program_name --help

# Example command
program_name -u https://example.com

# With output file
program_name -f targets.txt -o results.json
```

---

### 🧪 Running Tests

```bash
# Via Make
make test

# Via Cargo
cargo test

# With coverage
cargo tarpaulin --out Html
```

---

### 🐳 Using Docker

**Build image:**

```bash
docker build -t program_name .
```

**Run:**

```bash
docker run --rm program_name --help
docker run --rm program_name -u https://example.com
```

---

### 🛠 Using the Makefile

```bash
# Build release binary
make build

# Run with arguments
make run ARGS="--help"

# Run full CI (fmt + clippy + test + build)
make ci

# Clean build artifacts
make clean
```

---

## 🤝 Contributing

Pull requests are welcome!

**Before submitting:**

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`make ci`)
4. Commit changes (`git commit -m 'feat: Add amazing feature'`)
5. Push to branch (`git push origin feature/amazing-feature`)
6. Open Pull Request

---

## 📜 License

This project is licensed under **GPL-3.0**.  
See [LICENSE](https://claude.ai/chat/LICENSE) for details.

---

**⚡ Built with Rust | 🛡️ Secured by Design | 🚀 Production Ready**

````

---

## CHANGELOG.md Template

**AGENT: MUST create this file (RULE 2 + RULE 5)**

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Feature in development

## [1.0.0] - 2025-12-05

### Added
- Initial release
- Core functionality implemented
- Multi-platform support (Linux, macOS, Windows)
- Cross-architecture builds (x64, ARM64)
- GitHub Actions CI/CD pipeline
- Docker support
- Comprehensive README and documentation

### Security
- Secure coding practices applied
- Input validation implemented
- Error handling hardened

---

**AGENT NOTE:** Update this file before EVERY release as per RULE 5
````

---

## .version-tracking.md Template

**AGENT: MUST create this file (RULE 5)**

```markdown
# Version Tracking

This file logs all version changes and tracks development progress.

## Current Version: 1.0.0

---

## Version History

### v1.0.0 (2025-12-05)
**Type:** Initial Release (MAJOR)  
**Branch:** main  
**Tag:** v1.0.0

**Changes:**
- ✅ Initial project structure created
- ✅ Core functionality implemented
- ✅ Multi-platform CI/CD configured
- ✅ Documentation completed
- ✅ Docker support added
- ✅ Install scripts created

**Files Modified:**
- `src/main.rs` - Core implementation
- `Cargo.toml` - Dependencies configured
- `.github/workflows/ci-release.yml` - CI/CD pipeline
- `README.md` - Documentation
- `Dockerfile` - Container support

**Testing:**
- ✅ Unit tests passed (cargo test)
- ✅ Linting passed (cargo clippy)
- ✅ Formatting verified (cargo fmt)
- ✅ Cross-compilation verified (all platforms)

**Next Version:** 1.1.0 (planned features...)

---

**AGENT NOTE:** Update this file with EVERY commit/version bump
```

.version-tracking.md (Internal Detail Log)
  - Detailed technical notes for Claude's reference
  - Specific line numbers, function names, file paths
  - Implementation details, regex patterns, variable names
  - Helps me quickly understand "what was done and where" without searching
  - My persistent memory across sessions

---

## 🎨 Adobe Firefly Logo Prompt (Universal)

**AGENT: Generate logo using this prompt**

```text
Create a 4K ultra-realistic portrait-style logo for a cybersecurity and developer tool named "PROGRAM_NAME".

Art Direction:
- Style inspired by [Norse mythology / Greek mythology / Cyberpunk / Military tactical / etc.]
- Mix of futuristic technology, mythological symbolism, and sleek minimalism
- Strong silhouette, sharp contrast, glowing energy lines
- Clean but powerful composition
- High detail, crisp reflections, polished professional finish

Themes (select based on project purpose):
- OSINT, scanning, AI analysis, automation, resilience, stealth, investigation, or mythological motifs

Include:
- Emblem + wordmark
- Symmetric composition
- Metallic, neon, obsidian, or energy-based accents (depending on tool's lore)

Render:
- 4K resolution
- Ultra-realistic detail
- Cinematic lighting
- No background clutter, transparent or dark backdrop preferred
```

---

## 🎯 FINAL AGENT CHECKLIST

**Before marking project as complete, verify ALL items:**

### ✅ Code Quality (RULE 1)

- [ ] Code compiles: `cargo build --release`
- [ ] Tests pass: `cargo test`
- [ ] Linting passes: `cargo clippy -- -D warnings`
- [ ] Formatting applied: `cargo fmt --all`
- [ ] Security audit: `cargo audit` (if applicable)

### ✅ Documentation (RULE 2)

- [ ] README.md complete with all sections
- [ ] CHANGELOG.md created and updated
- [ ] .version-tracking.md created and updated
- [ ] Makefile created (Lite or Pro)
- [ ] Dockerfile created and tested
- [ ] .gitignore includes test artifacts
- [ ] All test files removed from git history

### ✅ Cross-Platform Support (RULE 3)

- [ ] Install script created (`install.sh`)
- [ ] Supports Linux, macOS, Windows
- [ ] Uses system package managers first
- [ ] Falls back to language-specific managers

### ✅ CI/CD Pipeline (RULE 4)

- [ ] GitHub Actions workflow created
- [ ] Compiles on: Ubuntu, macOS, Windows
- [ ] Supports architectures: x64, ARM64
- [ ] Auto-publishes releases on tags
- [ ] Binary naming follows convention

### ✅ Versioning (RULE 5)

- [ ] Follows Semantic Versioning 2.0.0
- [ ] Version updated in Cargo.toml
- [ ] CHANGELOG.md updated
- [ ] .version-tracking.md updated

### ✅ Git Workflow (RULE 6)

- [ ] `main` branch created
- [ ] `dev` branch created
- [ ] Currently on `dev` branch
- [ ] Proper branch strategy documented

### ✅ Release Process (RULE 7)

- [ ] All changes committed
- [ ] Code pushed to remote
- [ ] Version tagged (e.g., `v1.0.0`)
- [ ] Tag pushed to remote
- [ ] GitHub Release auto-created
- [ ] Binaries attached to release

---

## 📝 AGENT FINAL DELIVERY FORMAT

When delivering the completed project, provide:

```markdown
# ✅ PROJECT: PROGRAM_NAME - COMPLETE

## 📊 Summary
- **Version:** 1.0.0
- **Project Type:** Rust CLI Tool
- **Purpose:** [Brief description]
- **Status:** ✅ Production Ready

## 🎯 Deliverables
- ✅ Full source code (`src/`, `Cargo.toml`)
- ✅ Documentation (`README.md`, `CHANGELOG.md`)
- ✅ Build system (`Makefile`, `Dockerfile`)
- ✅ CI/CD pipeline (`.github/workflows/`)
- ✅ Version tracking (`.version-tracking.md`)
- ✅ Install script (`install.sh`)
- ✅ Git repository initialized with `main` and `dev` branches

## ✅ Quality Gates Passed
- Compilation: ✅ Success
- Tests: ✅ All passed (X/X tests)
- Linting: ✅ No warnings
- Formatting: ✅ Applied
- Security: ✅ Audited

## 🚀 Next Steps for Human
1. Review code and documentation
2. Push to GitHub: `git push origin dev && git push origin main`
3. Create first tag: `git tag v1.0.0 && git push origin v1.0.0`
4. GitHub Actions will automatically create release with binaries

## 📦 Release Assets (Will be auto-generated)
- `program_name-linux-x64`
- `program_name-linux-aarch64`
- `program_name-macos-x64`
- `program_name-macos-aarch64`
- `program_name-windows-x64.exe`
- `program_name-windows-aarch64.exe`
- SHA256 checksums for all binaries
```
