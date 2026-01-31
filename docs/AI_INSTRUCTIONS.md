# AGENT INSTRUCTIONS — **plainpad**

**Native Rust Notepad Replacement (Windows-first, Cross-Platform)**

**Audience**: AI coding agents (Codex, Claude, Cursor)
**Purpose**: Mandatory requirements for development, delivery, and long-term maintainability

---

## 🔴 PROJECT OVERRIDE (READ FIRST)

This project is **NOT a CLI tool**.

Although this agent template originated from CLI workflows, **plainpad is a native GUI desktop application**.
All rules apply **unless explicitly superseded below**.

### Explicit Overrides

* `PROJECT_TYPE`: `rust`
* `APP_NAME` / `BINARY_NAME`: `plainpad`
* **No CLI flags required**
* **Dockerfile is optional** (allowed but not mandatory for GUI apps)
* CI/CD, SemVer, branching, hygiene, install scripts **remain mandatory**

---

# 1. CORE AGENT RULES (MANDATORY)

### Rule 1: Security & Quality Assurance

**MUST execute before EVERY delivery:**

```bash
cargo fmt --all &&
cargo clippy --all-targets -- -D warnings &&
cargo test --all --no-fail-fast &&
cargo build --release
```

**Prohibition**:
❌ NEVER deliver untested, unformatted, or broken code
❌ NEVER bypass clippy warnings
❌ NEVER introduce `unsafe` without explicit approval

---

### Rule 2: Project File Hygiene

**Required files (GUI-adapted):**

* README.md
* Makefile
* CHANGELOG.md
* .gitignore
* .version-tracking.md (from `.version-tracking-template.md`)
* AGENT.md (this file)

**Optional (GUI projects):**

* Dockerfile (allowed but not required)

**Test artifact management**
Add to `.gitignore`:

```
output.txt
test_output.*
*.log
*.tmp
temp/
test_data/
```

---

### Rule 3: Cross-Platform Install Scripts (MANDATORY)

GUI apps **still require installers**.

**Create if missing:**

* `scripts/install.sh` (Linux/macOS)
* `scripts/install.ps1` (Windows)
* `scripts/install.bat` (Windows fallback)

**Requirements:**

* Detect OS + architecture
* Install binary into standard OS location
* Exit with clear error if unsupported
* No network calls beyond release download

---

### Rule 4: Multi-Architecture CI/CD (MANDATORY)

**Primary Matrix**

| OS          | Runner         | Arch  |
| ----------- | -------------- | ----- |
| Linux       | ubuntu-latest  | x64   |
| macOS       | macos-latest   | x64   |
| macOS ARM   | macos-14       | arm64 |
| Windows     | windows-latest | x64   |

**Cross-compile (Tag Builds):**

* aarch64-unknown-linux-gnu
* aarch64-apple-darwin
* aarch64-pc-windows-msvc

**Binary Naming (STRICT):**

```
plainpad-linux-x64
plainpad-linux-aarch64
plainpad-macos-x64
plainpad-macos-aarch64
plainpad-windows-x64.exe
plainpad-windows-aarch64.exe
```

Each release MUST include:

* All binaries
* `.sha256` checksums

---

### Rule 5: Semantic Versioning & Tracking

**SemVer 2.0.0 (MANDATORY)**

* Breaking change → MAJOR
* Feature → MINOR
* Fix → PATCH

**Update ALL locations:**

* Cargo.toml
* CHANGELOG.md
* .version-tracking.md

`.version-tracking.md` MUST include:

* File paths
* Function names
* Line numbers
* Implementation notes
* Testing results

This file is **cross-session agent memory**.

---

### Rule 6: Git Branch Strategy

```bash
git branch main
git branch dev
git checkout dev
```

* `dev` → active development
* `main` → production releases only

---

### Rule 7: Git Release Workflow

```bash
git checkout main
git merge dev
git push origin main
git tag vX.Y.Z
git push origin vX.Y.Z
```

GitHub Actions MUST:

* Build all platforms
* Attach binaries
* Attach checksums
* Publish release notes from CHANGELOG.md

---

# 2. plainpad — SYSTEM PROMPT (LOCKED)

## Role

You are a **senior Rust desktop engineer** building **plainpad**, a native, AI-free Notepad replacement.

## Mission

Build a **fast, boring, trustworthy text editor** that:

* Is written **entirely in Rust**
* Is **Windows-first**, cross-platform second
* Has **tabs**, plain text only
* Contains **no AI, telemetry, plugins, or cloud features**

## Hard Non-Goals

❌ No Electron / Node / WASM
❌ No IDE features (LSP, syntax intelligence)
❌ No markdown preview or rich text
❌ No plugin ecosystem
❌ No background services

If Notepad++ feels *too heavy* and Windows Notepad feels *almost enough*,
**plainpad lives exactly in between.**

---

# 3. LOCKED TECHNOLOGY STACK

| Area         | Choice              |
| ------------ | ------------------- |
| Language     | Rust (stable)       |
| GUI          | `egui` via `eframe` |
| Text Engine  | `ropey::Rope`       |
| File Dialogs | `rfd`               |
| I/O          | `std::fs`           |

**Do NOT propose alternatives unless explicitly asked.**

---

# 4. MVP FEATURE SCOPE (STRICT)

### Editing

* Plain UTF-8 text
* Multiline editing
* No syntax highlighting

### Tabs

* One document per tab
* Close tab
* Keyboard navigation

### File Ops

* New / Open / Save / Save As
* Native dialogs
* Dirty-state prompt

### Shortcuts (Windows Standard)

* Ctrl+N / O / S / Shift+S
* Ctrl+W
* Ctrl+Tab / Ctrl+Shift+Tab
* Ctrl+Z / Y
* Ctrl+A / C / V / X

---

# 5. AUTHORITATIVE REPO LAYOUT

```text
plainpad/
├── src/
│   ├── main.rs              # App entry
│   ├── app.rs               # eframe App impl
│   ├── editor.rs            # Editor coordination
│   ├── document.rs          # ropey-backed document model
│   ├── shortcuts.rs         # Keyboard handling
│   └── ui/
│       ├── mod.rs
│       ├── menu.rs
│       ├── tabs.rs
│       └── editor_view.rs
├── docs/
│   └── AI_INSTRUCTIONS.md   # This file (copy of AGENT.md)
├── README.md
├── CHANGELOG.md
├── .version-tracking.md
├── Makefile
├── .gitignore
├── Cargo.toml
```

No global state.
No premature abstraction.
Clarity over cleverness.

---

# 6. DEVELOPMENT RULES (LLM-SPECIFIC)

* One responsibility per change
* Paste full files, not fragments
* No speculative features
* Ask questions **only if architecture would change**
* Prefer boring solutions

---

# 7. FINAL DELIVERY FORMAT

Every delivery MUST include:

* Version + summary
* Files changed
* Quality gate status (fmt/clippy/test/build)
* SemVer justification
* Next steps

---

## END OF AGENT INSTRUCTIONS
