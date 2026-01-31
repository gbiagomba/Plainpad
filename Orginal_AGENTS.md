# Program
I need a rust program named `program_name` that uses package_lib. The purpose of the program is enter_purpose. Add any additional details here...Please use the latest versions of all the dependency software. Please write the full/complete program. Also, could you write a `.gitignore` file & We will be using the GPL-3.0 license. 

# Cargo.toml 
include the `Cargo.toml` file, please use the template below to build the file:
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
```

# Github Actions
 I need a GitHub actions file. I want it to run on every supported OS, and it needs to use checkout@v4. I need GitHub to release the binary after every build. Below is a sample of what I want the output to look like:
 
## **GitHub Actions workflow — .github/workflows/release.yml**
```yaml
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
      - '**'          # Run CI on all branches
    tags:
      - 'v*'          # Also run on version tags like v1.0.0
  pull_request:
    branches:
      - '**'          # Run CI on all PRs

env:
  # PROJECT_TYPE controls which toolchain paths are used.
  # Valid values: "rust", "python"
  PROJECT_TYPE: rust

  # For Rust projects: the built binary name (no path).
  # Example: "terminus", "stormbreaker", "anubis"
  BINARY_NAME: program_name

  # For Python projects: the distribution/package name used in build artifacts.
  # Example: "program_name", "my_tool"
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

      # ------------------------
      # RUST TOOLCHAIN + CI PATH
      # ------------------------
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
          # Example: program_name-Linux-X64, program_name-macOS-arm64, etc.
          name: ${{ env.BINARY_NAME }}-${{ runner.os }}-${{ runner.arch }}
          path: |
            target/release/${{ env.BINARY_NAME }}*

      # ---------------------------
      # PYTHON TOOLCHAIN + CI PATH
      # ---------------------------
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
          # Example:
          # pip install black
          # black --check .

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
          # Allows future customization if needed
          PYTHON_PACKAGE_NAME: ${{ env.PYTHON_PACKAGE_NAME }}

      - name: Upload Python package artifacts
        if: env.PROJECT_TYPE == 'python' && startsWith(github.ref, 'refs/tags/v')
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.PYTHON_PACKAGE_NAME }}-dist
          path: dist/*

  # ---------------------------------------------------
  # RUST-ONLY CROSS BUILDS (aarch64) FOR MULTI-ARCH RELEASES
  # ---------------------------------------------------
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

  # --------------------------
  # UNIFIED RELEASE JOB (TAG)
  # --------------------------
  release:
    name: Release on tag
    runs-on: ubuntu-latest
    # For Rust we need all cross jobs; for Python we only need build-test.
    # Using a superset of needs is fine; skipped jobs satisfy the dependency.
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
          # For Rust, artifacts start with BINARY_NAME-...
          # For Python, artifact is `${PYTHON_PACKAGE_NAME}-dist`
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

          # Extract release notes from CHANGELOG.md
          if [ -f CHANGELOG.md ]; then
            awk "/^## $VERSION/,/^## /{if (/^## $VERSION/) f=1; else if (/^## /) f=0; if (f && !/^## $VERSION/) print}" CHANGELOG.md > release_notes.md
          fi

          # If extraction failed, use tag message
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
          # Attach everything:
          # - Rust: BINARY_NAME-* and BINARY_NAME-*.sha256
          # - Python: dist artifacts already downloaded (pattern "*")
          files: |
            ${{ env.BINARY_NAME }}-*
            dist/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## .github/dependabot.yml
```yaml
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

## **1) Reusable workflow:** 

### **.github/workflows/ci-release-reusable.yml**

```
# .github/workflows/ci-release-reusable.yml
#
# REUSABLE CI+RELEASE TEMPLATE
#
# HOW TO USE (for the caller workflow):
# jobs:
#   ci:
#     uses: ./.github/workflows/ci-release-reusable.yml
#     with:
#       project_type: rust            # or "python"
#       binary_name: terminus        # required if project_type == "rust"
#       python_package_name: ""      # required if project_type == "python"
#       enable_cross: true           # (rust only) enable aarch64 builds

name: Reusable CI and Release

on:
  workflow_call:
    inputs:
      project_type:
        description: "Project type: 'rust' or 'python'"
        required: true
        type: string
      binary_name:
        description: "Rust binary name (for project_type='rust')"
        required: false
        type: string
      python_package_name:
        description: "Python distribution/package name (for project_type='python')"
        required: false
        type: string
      enable_cross:
        description: "Enable Rust aarch64 cross builds (linux/macos/windows)"
        required: false
        type: boolean
        default: true

env:
  PROJECT_TYPE: ${{ inputs.project_type }}
  BINARY_NAME: ${{ inputs.binary_name }}
  PYTHON_PACKAGE_NAME: ${{ inputs.python_package_name }}

jobs:
  # ------------------------------------------------
  # MAIN CI JOB: multi-OS build/test + tag artifacts
  # ------------------------------------------------
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

      # ------------------------
      # RUST PATH
      # ------------------------
      - name: Set up Rust
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        uses: swatinem/rust-cache@v2

      - name: Rust Format (cargo fmt)
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        run: cargo fmt --all -- --check

      - name: Rust Lints (cargo clippy)
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        run: cargo clippy --all-targets -- -D warnings

      - name: Rust Build (release)
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        run: cargo build --release

      - name: Rust Tests
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        run: cargo test --all --no-fail-fast

      # Prepare per-OS x64 binary names for release on tags
      - name: Prepare Rust binary (x64, tagged builds)
        if: ${{ env.PROJECT_TYPE == 'rust' && startsWith(github.ref, 'refs/tags/v') }}
        shell: bash
        run: |
          if [ -z "${BINARY_NAME}" ]; then
            echo "BINARY_NAME is not set; this is required for Rust projects."
            exit 1
          fi

          case "${RUNNER_OS}" in
            Linux)
              PLATFORM="linux-x64"
              SRC="target/release/${BINARY_NAME}"
              SUFFIX=""
              ;;
            macOS)
              PLATFORM="macos-x64"
              SRC="target/release/${BINARY_NAME}"
              SUFFIX=""
              ;;
            Windows)
              PLATFORM="windows-x64"
              SRC="target/release/${BINARY_NAME}.exe"
              SUFFIX=".exe"
              ;;
            *)
              echo "Unknown OS: ${RUNNER_OS}"
              exit 1
              ;;
          esac

          if [ ! -f "${SRC}" ]; then
            echo "Expected binary not found at ${SRC}"
            ls -R
            exit 1
          fi

          cp "${SRC}" "${BINARY_NAME}-${PLATFORM}${SUFFIX}"
        env:
          BINARY_NAME: ${{ env.BINARY_NAME }}

      - name: Upload Rust x64 binary artifact
        if: ${{ env.PROJECT_TYPE == 'rust' && startsWith(github.ref, 'refs/tags/v') }}
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-${{ runner.os }}-x64
          path: |
            ${{ env.BINARY_NAME }}-*

      # ------------------------
      # PYTHON PATH
      # ------------------------
      - name: Set up Python
        if: ${{ env.PROJECT_TYPE == 'python' }}
        uses: actions/setup-python@v5
        with:
          python-version: '3.x'
          cache: 'pip'

      - name: Install Python dependencies
        if: ${{ env.PROJECT_TYPE == 'python' }}
        run: |
          if [ -f "requirements-dev.txt" ]; then
            pip install -r requirements-dev.txt
          elif [ -f "requirements.txt" ]; then
            pip install -r requirements.txt
          else
            echo "No requirements*.txt found; skipping dependency install."
          fi

      - name: Python Lint/Format (placeholder)
        if: ${{ env.PROJECT_TYPE == 'python' }}
        run: |
          echo "Add flake8/black/ruff/mypy here if desired."
          # Example:
          # pip install black
          # black --check .

      - name: Python Tests
        if: ${{ env.PROJECT_TYPE == 'python' }}
        run: |
          if [ -d "tests" ]; then
            pip install pytest
            pytest
          else
            echo "No tests/ directory found; skipping pytest."
          fi

      - name: Build Python package (sdist/wheel)
        if: ${{ env.PROJECT_TYPE == 'python' && startsWith(github.ref, 'refs/tags/v') }}
        run: |
          if [ -z "${PYTHON_PACKAGE_NAME}" ]; then
            echo "PYTHON_PACKAGE_NAME is not set; this is required for Python projects."
            exit 1
          fi

          pip install build
          python -m build
        env:
          PYTHON_PACKAGE_NAME: ${{ env.PYTHON_PACKAGE_NAME }}

      - name: Upload Python dist artifacts
        if: ${{ env.PROJECT_TYPE == 'python' && startsWith(github.ref, 'refs/tags/v') }}
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.PYTHON_PACKAGE_NAME }}-dist
          path: dist/*

  # ---------------------------------------------------
  # RUST CROSS BUILDS (aarch64) FOR MULTI-ARCH RELEASES
  # ---------------------------------------------------
  linux-arm64:
    name: Linux aarch64 (cross)
    runs-on: ubuntu-latest
    if: ${{ env.PROJECT_TYPE == 'rust' && inputs.enable_cross && startsWith(github.ref, 'refs/tags/v') }}
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-unknown-linux-gnu

      - name: Install cross
        run: cargo install cross --git https://github.com/cross-rs/cross

      - name: Build (cross, linux-aarch64)
        run: cross build --release --target aarch64-unknown-linux-gnu

      - name: Prepare linux-aarch64 binary
        shell: bash
        run: |
          if [ -z "${BINARY_NAME}" ]; then
            echo "BINARY_NAME is not set; this is required for Rust projects."
            exit 1
          fi

          SRC="target/aarch64-unknown-linux-gnu/release/${BINARY_NAME}"
          if [ ! -f "${SRC}" ]; then
            echo "Expected binary not found at ${SRC}"
            ls -R
            exit 1
          fi

          cp "${SRC}" "${BINARY_NAME}-linux-aarch64"
        env:
          BINARY_NAME: ${{ env.BINARY_NAME }}

      - name: Upload linux-aarch64 artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-linux-aarch64
          path: ${{ env.BINARY_NAME }}-linux-aarch64

  macos-arm64:
    name: macOS aarch64 (native)
    runs-on: macos-latest
    if: ${{ env.PROJECT_TYPE == 'rust' && inputs.enable_cross && startsWith(github.ref, 'refs/tags/v') }}
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-darwin

      - name: Build (macOS aarch64)
        run: cargo build --release --target aarch64-apple-darwin

      - name: Prepare macos-aarch64 binary
        shell: bash
        run: |
          if [ -z "${BINARY_NAME}" ]; then
            echo "BINARY_NAME is not set; this is required for Rust projects."
            exit 1
          fi

          SRC="target/aarch64-apple-darwin/release/${BINARY_NAME}"
          if [ ! -f "${SRC}" ]; then
            echo "Expected binary not found at ${SRC}"
            ls -R
            exit 1
          fi

          cp "${SRC}" "${BINARY_NAME}-macos-aarch64"
        env:
          BINARY_NAME: ${{ env.BINARY_NAME }}

      - name: Upload macos-aarch64 artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-macos-aarch64
          path: ${{ env.BINARY_NAME }}-macos-aarch64

  windows-arm64:
    name: Windows aarch64 (cross)
    runs-on: windows-latest
    if: ${{ env.PROJECT_TYPE == 'rust' && inputs.enable_cross && startsWith(github.ref, 'refs/tags/v') }}
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-pc-windows-msvc

      - name: Build (windows aarch64)
        run: cargo build --release --target aarch64-pc-windows-msvc

      - name: Prepare windows-aarch64 binary
        shell: bash
        run: |
          if [ -z "${BINARY_NAME}" ]; then
            echo "BINARY_NAME is not set; this is required for Rust projects."
            exit 1
          fi

          SRC="target/aarch64-pc-windows-msvc/release/${BINARY_NAME}.exe"
          if [ ! -f "${SRC}" ]; then
            echo "Expected binary not found at ${SRC}"
            ls -R
            exit 1
          fi

          cp "${SRC}" "${BINARY_NAME}-windows-aarch64.exe"
        env:
          BINARY_NAME: ${{ env.BINARY_NAME }}

      - name: Upload windows-aarch64 artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.BINARY_NAME }}-windows-aarch64
          path: ${{ env.BINARY_NAME }}-windows-aarch64.exe

  # --------------------------
  # UNIFIED RELEASE JOB (TAG)
  # --------------------------
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

      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          pattern: "*"
          merge-multiple: true

      - name: Create checksums (Rust binaries only)
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        shell: bash
        run: |
          if [ -z "${BINARY_NAME}" ]; then
            echo "BINARY_NAME is not set; this is required for Rust projects."
            exit 1
          fi

          for f in ${BINARY_NAME}-*; do
            if [ -f "$f" ]; then
              echo "Creating checksum for $f"
              sha256sum "$f" > "$f.sha256"
            fi
          done
        env:
          BINARY_NAME: ${{ env.BINARY_NAME }}

      - name: Extract release notes
        id: extract_notes
        shell: bash
        run: |
          VERSION="${GITHUB_REF#refs/tags/}"
          echo "version=$VERSION" >> $GITHUB_OUTPUT

          # Extract release notes from CHANGELOG.md
          if [ -f CHANGELOG.md ]; then
            awk "/^## $VERSION/,/^## /{if (/^## $VERSION/) f=1; else if (/^## /) f=0; if (f && !/^## $VERSION/) print}" CHANGELOG.md > release_notes.md
          fi

          # If extraction failed, use tag message
          if [ ! -s release_notes.md ]; then
            git tag -l --format='%(contents)' "$VERSION" > release_notes.md
          fi

      # Step 1: Create or update the Release (no files yet)
      - name: Create/Update Release (metadata only)
        uses: softprops/action-gh-release@v2
        with:
          draft: false
          prerelease: false
          generate_release_notes: true
          body_path: release_notes.md
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      # Step 2: Upload Rust assets (binaries + checksums)
      - name: Upload Rust assets
        if: ${{ env.PROJECT_TYPE == 'rust' }}
        uses: softprops/action-gh-release@v2
        with:
          files: |
            ${{ env.BINARY_NAME }}-*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      # Step 3: Upload Python assets (dist artifacts)
      - name: Upload Python assets
        if: ${{ env.PROJECT_TYPE == 'python' }}
        uses: softprops/action-gh-release@v2
        with:
          files: |
            dist/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### **2) Per-project entry point workflow: .github/workflows/ci.yml** 

You keep this **tiny** and let the reusable do the heavy lifting.
#### **Example A: Rust project (e.g., Terminus)**

```
# .github/workflows/ci.yml
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

#### **Example B: Rust project (Stormbreaker-style)**

```
# .github/workflows/ci.yml
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
      binary_name: stormbreaker
      enable_cross: true
```

#### **Example C: Python project**

```
# .github/workflows/ci.yml
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

### **3) Dependabot: .github/dependabot.yml** 

Same as before, just standardized:

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

  # Uncomment for Python projects
  # - package-ecosystem: "pip"
  #   directory: "/"
  #   schedule:
  #     interval: "weekly"
  #   open-pull-requests-limit: 10
```

# Dockerfile
I need a `Dockerfile`, please use the template below to build the file:
```bash
# Dockerfile
#
# TEMPLATE INSTRUCTIONS FOR CODING AGENT:
# 1. Replace APP_NAME with the actual binary/crate name (e.g., "terminus", "stormbreaker").
# 2. Optionally set APP_DESCRIPTION and APP_VERSION to match the project.
# 3. Ensure Cargo.toml / [[bin]] name matches APP_NAME.

###############################
# 1) BUILDER STAGE (Rust)
###############################

# Use an official Rust image (configurable via build arg)
ARG RUST_IMAGE=rust:slim-bookworm
FROM ${RUST_IMAGE} AS builder

# Set the working directory inside the builder container
WORKDIR /app

# Copy manifest first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs so we can build dependencies only
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release || true

# Now copy the actual source code
COPY . .

# Build the real application in release mode
RUN cargo build --release

###############################
# 2) RUNTIME STAGE (Slim Debian)
###############################

FROM debian:bookworm-slim

# Install only what we need at runtime (HTTPS, etc.)
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# --- Metadata / labels ---
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

# Create a non-root user to run the app
RUN useradd -m -s /usr/sbin/nologin ${APP_NAME}

# Set working directory in runtime image
WORKDIR /home/${APP_NAME}

# Switch to non-root user
USER ${APP_NAME}

# Copy the compiled binary from the builder stage
# NOTE: The file name MUST match the Rust binary (i.e., [[bin]] name in Cargo.toml)
COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}

# Define the entrypoint for the container
ENTRYPOINT ["program_name"]

# Default command line arguments (can be overridden at `docker run`)
CMD ["--help"]
```

# Makefile
Could you write a `Makefile`. It needs to be able to build, install, run, clean and test the code. Below is a **single template block** that contains **two full Makefiles**:
- **Option A – Makefile Lite** (Hermes-style, KISS), if project is **simple**, use OPTION A and swap APP_NAME.
- **Option B – Makefile Pro** (Terminus + Stormbreaker merged), If project is **flagship** (Stormbreaker / Terminus / Sword of Damocles), use OPTION B and swap APP_NAME + customize run-example targets.

```bash
#############################################
# OPTION A: MAKEFILE LITE (KISS / HERMES)
#############################################
# TEMPLATE INSTRUCTIONS FOR CODING AGENT (LITE):
# 1. Create a file named `Makefile`.
# 2. Replace APP_NAME with the final binary/crate name (e.g., hermes, terminus, stormbreaker).
# 3. Ensure Cargo.toml [[bin]] name matches APP_NAME.
#
# Use this version when you want a simple, minimal Makefile.

APP_NAME    := program_name
CARGO       := cargo
BUILD_DIR   := ./target/release/$(APP_NAME)

.PHONY: all build release install run test clean

# Default target
all: release

# Build in release mode
build:
	$(CARGO) build --release

# Alias for build (explicit)
release: build

# Install the binary globally
install:
	$(CARGO) install --path .

# Run the binary via cargo (pass args: make run ARGS="--help")
run:
	$(CARGO) run --release -- $(ARGS)

# Run tests
test:
	$(CARGO) test

# Clean build artifacts
clean:
	$(CARGO) clean



#########################################################
# OPTION B: MAKEFILE PRO (TERMINUS + STORMBREAKER MERGED)
#########################################################
# TEMPLATE INSTRUCTIONS FOR CODING AGENT (PRO):
# 1. Create a file named `Makefile`.
# 2. Replace APP_NAME with the final binary/crate name (e.g., terminus, stormbreaker, hermes).
# 3. Ensure Cargo.toml [[bin]] name matches APP_NAME.
# 4. Optionally customize the sample run targets (run-example-*).
#
# Use this version for “serious” tools where you want CI parity
# (fmt, clippy, check, ci target, run-release, etc.).

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

# Default target: optimized build
all: release

# -------------------------
# Build / Release / Install
# -------------------------

build:
	$(CARGO) build

release:
	$(CARGO) build --release

install:
	$(CARGO) install --path .

uninstall:
	$(CARGO) uninstall $(APP_NAME)

# ---------------
# Quality / Checks
# ---------------

test:
	$(CARGO) test

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt --all

clippy:
	$(CARGO) clippy --all-targets -- -D warnings

# -------------
# Run Commands
# -------------

# Generic run via cargo (debug build by default)
# Usage:
#   make run ARGS="--help"
run:
	$(CARGO) run -- $(ARGS)

# Run the release binary directly (faster startup)
# Usage:
#   make run-release ARGS="--help"
run-release: release
	$(BUILD_DIR)/$(APP_NAME) $(ARGS)

# Example “URL” run (inspired by Terminus)
# Customize or delete per project.
run-example-url: release
	$(BUILD_DIR)/$(APP_NAME) -u http://example.com -X ALL

# Example “file” run (inspired by Terminus)
# Customize or delete per project.
run-example-file: release
	$(BUILD_DIR)/$(APP_NAME) -f urls.txt -X ALL

# -----------
# Housekeeping
# -----------

clean:
	$(CARGO) clean

# CI helper: mirrors what your GH Actions should roughly do locally
ci: fmt clippy test release
```

# README.MD
Could you write a README, and I need to have the following sections:
```markdown
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

This README must be:
- reusable across _all_ your Rust (and occasional Python) CLI tools    
- descriptive but agent-friendly    
- structured, predictable, and clean    
- flexible enough to support future automation (like CLVR, Sword of Damocles, etc.)    

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

# 🎨 NEXT: Firefly Logo Prompt  

## 🔥 Adobe Firefly Logo Prompt (Universal Template)

```text
Create a 4K ultra-realistic portrait-style logo for a cybersecurity and developer tool named "PROGRAM_NAME". 

Art Direction:
- Style inspired by franchise_art-style 
- Mix of futuristic technology, mythological symbolism, and sleek minimalism  
- Strong silhouette, sharp contrast, glowing energy lines  
- Clean but powerful composition  
- High detail, crisp reflections, polished professional finish  

Themes (agent selects based on project):
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
