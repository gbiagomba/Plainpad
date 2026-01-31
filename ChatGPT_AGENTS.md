# Global Agent Guidelines

You are a coding agent working on a project scaffold for a CLI tool.

Follow **all** of these rules for every request unless explicitly told otherwise:

1. **Secure code + verification**
   - Use secure coding practices by default (safe APIs, error handling, minimal privileges).
   - Compile and run tests where applicable to ensure the code builds and executes.
   - For Rust, run at least: `cargo check`, `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo build --release`.
   - For Python, run at least: `pytest` (if tests exist) and basic lint/format checks (placeholder hooks already included in CI).

2. **Repo hygiene: docs & ignore files**
   - Create/maintain/update:
     - `README.md`
     - `Makefile`
     - `Dockerfile`
     - `CHANGELOG.md`
     - `.gitignore`
   - Add **all test artifacts / temporary outputs** to `.gitignore` so they don’t get pushed:
     - Examples: `output.txt`, `test_output.html`, `logs/`, `tmp/`, `coverage/`, etc.
   - If any such files are already in the repo, remove them from Git tracking (i.e., ensure they’re deleted or untracked).

3. **Install scripts**
   - If an install script does **not** exist and it makes sense for the project, create:
     - `scripts/install_unix.sh` for Linux/Unix/macOS
     - `scripts/install_windows.ps1` or `install_windows.bat` for Windows
   - Install dependencies using **system package managers** first:
     - macOS: `brew`
     - Debian/Ubuntu: `apt-get`, or `apt`
     - RHEL/CentOS/Amazon: `yum` or `dnf`
     - Windows: `winget` or `choco`
   - If a dependency **is not** available via system package managers, then use language-specific managers:
     - Rust: `cargo install`
     - Python: `pip install`
     - Go: `go install`
   - Scripts should:
     - Detect OS/architecture where possible.
     - Install only what’s needed.
     - Exit with a clear error if unsupported.

4. **GitHub Workflow & multi-arch releases**
   - If no GitHub workflow exists, create one. If one exists, **extend it**:
     - Code must compile on:
       - Linux/Unix (x64 + arm64)
       - macOS (Intel + Apple Silicon)
       - Windows (x64 + arm64 where possible)
   - Extend CI to **publish on tags** (`v*`):
     - Attach binaries in standardized naming form (examples):
       - `TOOLNAME_wios-x64.exe`, `TOOLNAME_wios-arm64.exe`
       - `TOOLNAME_macos-intel`, `TOOLNAME_macos-arm`
       - `TOOLNAME_linux-x64`, `TOOLNAME_linux-arm64`
     - You may internally use `BINARY_NAME_os-arch` naming, but final **release assets** should follow the above style.

5. **Semantic Versioning + tracking**
   - Use **Semantic Versioning 2.0.0**.
   - Initial version should generally start at `0.1.0` (or `1.0.0` if truly stable from day one).
   - Whenever a change is requested/implemented:
     - Automatically bump the version in:
       - `Cargo.toml` / `pyproject.toml` / equivalent
       - `CHANGELOG.md` (add a new section like `## vX.Y.Z - YYYY-MM-DD`)
     - Track changes in `.version-tracking.md`:
       - This is your "session notes" that  persist across conversations.
       - Summarize what changed, why, and the new version.
       - is a detailed internal tracking log for you  to  reference in future sessions
       - You don't have to re-explore the entire codebase. 

6. **Git branching**
   - Ensure git branches:
     - Create `main` and `dev` branches if they don’t exist.
     - Switch to `dev` for active development.
   - All feature changes should land on `dev` first logically, then be merged into `main` for releases.

7. **Git operations: commit, tag, release**
   - After making changes:
     - `git add` all relevant files (excluding ignored files).
     - Create a meaningful commit message summarizing the changes.
   - Tag the new version:
     - e.g., `git tag v0.1.0`
   - Push changes and tags:
     - `git push origin dev`
     - `git push origin main` (after merge)
     - `git push --tags`
   - Ensure the GitHub workflow runs and creates a Release using the tagged version, attaching binaries with the naming pattern from Rule 4.

---

# Program

I need a Rust program named `program_name` that uses `package_lib`. The purpose of the program is `enter_purpose`. Add any additional details here…

- Please use the latest stable versions of all dependency software.
- Please write the **full/complete program** (no TODO placeholders).
- We will be using the **GPL-3.0** license.
- Also:
  - Create a `.gitignore` file.
  - Create or update `README.md`, `Makefile`, `Dockerfile`, `CHANGELOG.md`, `.version-tracking.md`.
  - Ensure the code compiles and tests pass (see Global Guidelines).

> NOTE FOR CODING AGENT: this repo will normally be Rust-based, but some templates are Python-aware (GitHub Actions). Default to Rust unless explicitly told otherwise.

---

# Cargo.toml

Include the `Cargo.toml` file. Use this template and adjust as needed:

```yaml
[package]
name = "program_name"
version = "1.0.0"
authors = ["Gilles Biagomba <gilles.infosec@gmail.com>"]
edition = "2021"
license = "GPL-3.0"
description = ""
repository = "https://github.com/gbiagomba/program_name"
keywords = ["cli", "workspace", "management", "project"]
categories = ["command-line-utilities", "productivity"]

[dependencies]
clap = { version = "4.0", features = ["derive"] }

[[bin]]
name = "program_name"
path = "src/main.rs"
````

> NOTE FOR CODING AGENT:
> Update name, description, keywords, and categories as appropriate.    
> Keep version aligned with SemVer and .version-tracking.md.    
> Ensure [[bin]] matches the actual binary name used elsewhere (APP_NAME, BINARY_NAME, etc.).

---

# **GitHub Actions**

I need GitHub Actions workflows that:
- Run tests and builds on all major OSes (Linux, macOS Intel/ARM, Windows x64/ARM).    
- Use actions/checkout@v4.    
- For **tag pushes**, build release binaries and publish them as Release assets.    
- Use the naming scheme described in the Global Guidelines (Rule 4).    

## **GitHub Actions workflow —** .github/workflows/ci-release.yml

> NOTE: This is the **non-reusable** version. It is fine to use either this or the reusable version below, or both if that suits the project.

```
# .github/workflows/ci-release.yml
#
# TEMPLATE INSTRUCTIONS FOR CODING AGENT:
# 1. Replace `PROJECT_TYPE` with either "rust" or "python".
# 2. If PROJECT_TYPE == "rust":
#    - Replace `program_name` in BINARY_NAME with the final binary name (Cargo [[bin]] name).
#    - Ensure Cargo.toml and src/main.rs are set up accordingly.
# 3. If PROJECT_TYPE == "python":
#    - Replace `program_name` in PYTHON_PACKAGE_NAME with the package/distribution name.
#    - Ensure pyproject.toml / setup.cfg / setup.py matches that name.
# 4. Ensure there is a CHANGELOG.md with headings like "## v1.0.0". If not present,
#    the workflow will fall back to the tag message for release notes.

name: CI and Release

on:
  push:
    branches:
      - '**'
    tags:
      - 'v*'
  pull_request:
    branches:
      - '**'

env:
  PROJECT_TYPE: rust
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

      # RUST TOOLCHAIN + CI PATH
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

      # PYTHON TOOLCHAIN + CI PATH
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

      - name: Python Lint/Format (optional placeholder)
        if: env.PROJECT_TYPE == 'python'
        run: |
          echo "Add flake8/black/ruff/mypy here if desired"

      - name: Python Tests
        if: env.PROJECT_TYPE == 'python'
        run: |
          if [ -d "tests" ]; then
            pip install pytest
            pytest
          else
            echo "No tests/ directory found; skipping pytest."
          fi

      - name: Build Python package
        if: env.PROJECT_TYPE == 'python' && startsWith(github.ref, 'refs/tags/v')
        run: |
          pip install build
          python -m build
        env:
          PYTHON_PACKAGE_NAME: ${{ env.PYTHON_PACKAGE_NAME }}

      - name: Upload Python package artifacts
        if: env.PROJECT_TYPE == 'python' && startsWith(github.ref, 'refs/tags/v')
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.PYTHON_PACKAGE_NAME }}-dist
          path: dist/*

  # RUST-ONLY CROSS BUILDS (aarch64)
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
      - name: Upload binary (linux aarch64)
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
      - name: Build (macOS aarch64)
        run: cargo build --release --target aarch64-apple-darwin
      - name: Upload binary (macOS aarch64)
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
      - name: Build (windows aarch64)
        run: cargo build --release --target aarch64-pc-windows-msvc
      - name: Upload binary (windows aarch64)
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-windows-aarch64
          path: target/aarch64-pc-windows-msvc/release/${{ env.BINARY_NAME }}.exe

  release:
    name: Release on tag
    runs-on: ubuntu-latest
    needs:
      - build-test
      - linux-arm64
      - macos-arm64
      - windows-arm64
    if: startsWith(github.ref, 'refs/tags/v')
    permissions:
      contents: write

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Download artifacts (Rust + Python)
        uses: actions/download-artifact@v4
        with:
          pattern: "*"
          merge-multiple: true

      - name: Create checksums (Rust binaries only)
        if: env.PROJECT_TYPE == 'rust'
        run: |
          for f in ${BINARY_NAME}-*; do
            if [ -f "$f" ]; then
              sha256sum "$f" > "$f.sha256"
            fi
          done
        env:
          BINARY_NAME: ${{ env.BINARY_NAME }}

      - name: Extract release notes
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

### **.github/dependabot.yml**

```
# .github/dependabot.yml

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

  # Optional: enable for Python projects
  # - package-ecosystem: "pip"
  #   directory: "/"
  #   schedule:
  #     interval: "weekly"
  #   open-pull-requests-limit: 10
```

## Per-project entrypoint workflow — .github/workflows/ci.yml

Examples:
```
# Rust project (Terminus-style)
name: CI

on:
  push:
    branches:
      - '**'
    tags:
      - 'v*'
  pull_request:
    branches:
      - '**'

jobs:
  ci:
    uses: ./.github/workflows/ci-release-reusable.yml
    with:
      project_type: rust
      binary_name: terminus
      enable_cross: true
```

Examples 2:
```
# Python project
name: CI

on:
  push:
    branches:
      - '**'
    tags:
      - 'v*'
  pull_request:
    branches:
      - '**'

jobs:
  ci:
    uses: ./.github/workflows/ci-release-reusable.yml
    with:
      project_type: python
      python_package_name: program_name
      enable_cross: false
```

---

# **Dockerfile**
Use this Dockerfile template for Rust projects:

```
# Dockerfile
#
# TEMPLATE INSTRUCTIONS FOR CODING AGENT:
# 1. Replace APP_NAME with the actual binary/crate name (e.g., "terminus", "stormbreaker").
# 2. Optionally set APP_DESCRIPTION and APP_VERSION to match the project.
# 3. Ensure Cargo.toml / [[bin]] name matches APP_NAME.

###############################
# 1) BUILDER STAGE (Rust)
###############################

ARG RUST_IMAGE=rust:slim-bookworm
FROM ${RUST_IMAGE} AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release || true

COPY . .

RUN cargo build --release

###############################
# 2) RUNTIME STAGE (Slim Debian)
###############################

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG APP_NAME=program_name
ARG APP_VERSION=1.0.0
ARG APP_DESCRIPTION="CLI tool"
ARG APP_MAINTAINER="Gilles Biagomba <gilles.infosec@gmail.com>"

LABEL maintainer="${APP_MAINTAINER}"
LABEL version="${APP_VERSION}"
LABEL description="${APP_DESCRIPTION}"
LABEL org.opencontainers.image.title="${APP_NAME}"
LABEL org.opencontainers.image.description="${APP_DESCRIPTION}"
LABEL org.opencontainers.image.version="${APP_VERSION}"

RUN useradd -m -s /usr/sbin/nologin ${APP_NAME}

WORKDIR /home/${APP_NAME}
USER ${APP_NAME}

COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}

ENTRYPOINT ["program_name"]
CMD ["--help"]
```

> NOTE FOR CODING AGENT: ensure ENTRYPOINT string is updated to the actual APP_NAME binary.

---

# **Makefile**

We support **two variants**. Pick one per project:
- **Option A – Lite** (simple)    
- **Option B – Pro** (full dev/CI controls)    

```
#############################################
# OPTION A: MAKEFILE LITE (KISS / HERMES)
#############################################
# TEMPLATE INSTRUCTIONS FOR CODING AGENT (LITE):
# 1. Create a file named `Makefile`.
# 2. Replace APP_NAME with the final binary/crate name.
# 3. Ensure Cargo.toml [[bin]] name matches APP_NAME.

APP_NAME    := program_name
CARGO       := cargo
BUILD_DIR   := ./target/release/$(APP_NAME)

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



#########################################################
# OPTION B: MAKEFILE PRO (TERMINUS + STORMBREAKER MERGED)
#########################################################
# TEMPLATE INSTRUCTIONS FOR CODING AGENT (PRO):
# 1. Create a file named `Makefile`.
# 2. Replace APP_NAME with the final binary/crate name.
# 3. Ensure Cargo.toml [[bin]] name matches APP_NAME.
# 4. Optionally customize sample run targets.

APP_NAME     := program_name
CARGO        := cargo
BUILD_DIR    := ./target/release
DEBUG_DIR    := ./target/debug
INSTALL_PATH := $(HOME)/.cargo/bin

.PHONY: \
	all build release install uninstall \
	test check fmt clippy \
	run run-release \
	run-example-url run-example-file \
	clean ci

all: release

build:
	$(CARGO) build

release:
	$(CARGO) build --release

install:
	$(CARGO) install --path .

uninstall:
	$(CARGO) uninstall $(APP_NAME)

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

run-release: release
	$(BUILD_DIR)/$(APP_NAME) $(ARGS)

run-example-url: release
	$(BUILD_DIR)/$(APP_NAME) -u http://example.com -X ALL

run-example-file: release
	$(BUILD_DIR)/$(APP_NAME) -f urls.txt -X ALL

clean:
	$(CARGO) clean

ci: fmt clippy test release
```

---

# **README.md**
Base structure:

```
# program_name
## Background/Lore
## Table of Contents
## Features
## Installation
### Using GitHub Releases
### Using Cargo
### Compiling from Source
## Flags
## Usage
### Running Tests
### Using Docker
### Using the `Makefile`
## Contributing
## License
```

## **✅ UNIVERSAL README TEMPLATE (agentic pipeline)**

````
# PROGRAM_NAME

> VERSION: 1.0.0  
> DESCRIPTION: SHORT_PROJECT_DESCRIPTION  
> AUTHOR: Gilles Biagomba  
> LICENSE: GPL-3.0  

---

## 🧬 Background / Lore
Provide a short, engaging explanation of the story, inspiration, or intended purpose behind this tool.  
TEMPLATE INSTRUCTIONS FOR CODING AGENT:  
Replace with 2–4 sentences describing the theme or “lore” of the tool (e.g., mythology, tactical tooling, AI-augmented workflows, etc).

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
- [License](#license)

---

## 🚀 Features
TEMPLATE INSTRUCTIONS FOR CODING AGENT:  
List 4–8 bullet points describing the tool’s key capabilities.  
Examples:
- Multi-target scanning  
- Auto-detects environment or OS  
- AI-assisted analysis  
- Cross-platform binaries (Linux, macOS, Windows)  
- Configurable output formats (JSON, CSV, Markdown)  
- Parallel execution  

---

## 🛠 Installation

### 📦 Using GitHub Releases
Download the precompiled binary for your platform:

- Linux: `program_name-linux-x64` or `program_name-linux-aarch64`  
- macOS: `program_name-macos-x64` or `program_name-macos-aarch64`  
- Windows: `program_name-windows-x64.exe` or `program_name-windows-aarch64.exe`

Make it executable and move it to your PATH:

```bash
chmod +x program_name-*
sudo mv program_name-* /usr/local/bin/program_name
```

---

### **📚 Using Cargo (Rust projects)**

```
cargo install --git https://github.com/gbiagomba/PROGRAM_NAME
```

---

### **🧱 Compiling From Source**

```
git clone https://github.com/gbiagomba/PROGRAM_NAME
cd PROGRAM_NAME
cargo build --release
```

Binaries will appear in:

```
target/release/
```

---

## **🔧 Flags**

  

TEMPLATE INSTRUCTIONS FOR CODING AGENT:

Populate this section automatically by reading Clap / Argparse definition.

  

Example:

```
-h, --help            Show help information
-v, --version         Show version
-u, --url <URL>       Target URL
-X, --method <VERB>   HTTP method: GET, POST, ALL
-o, --output <FILE>   Output file
```

---

## **📈 Usage**

  

Show examples of how to run the tool.

```
program_name --help

# Example
program_name -u https://example.com -X ALL
```

---

### **🧪 Running Tests**

```
make test
# or
cargo test
```

---

### **🐳 Using Docker**

  

Build image:

```
docker build -t program_name .
```

Run:

```
docker run --rm program_name --help
```

Pass arguments:

```
docker run --rm program_name -u https://example.com -X GET
```

---

### **🛠 Using the Makefile**

  

**Build the tool**

```
make build
```

**Run with arguments**

```
make run ARGS="--help"
```

**Lint + test + release**

```
make ci
```

---

## **🤝 Contributing**

  

Pull requests are welcome.

  

If proposing major changes, please open an issue first to discuss alignment with project goals.

---

## **📜 License**

This project is licensed under **GPL-3.0**.

See LICENSE for details.
````

---

# Firefly Logo Prompt

Use this Adobe Firefly prompt to generate a 4K logo:

```text
Create a 4K ultra-realistic portrait-style logo for a cybersecurity and developer tool named "PROGRAM_NAME". 

Art Direction:
- Style inspired by franchise_art-style 
- Mix of futuristic technology, mythological symbolism, and sleek minimalism  
- Strong silhouette, sharp contrast, glowing energy lines  
- Clean but powerful composition  
- High detail, crisp reflections, polished professional finish  

Themes (select based on the tool’s purpose):
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
````