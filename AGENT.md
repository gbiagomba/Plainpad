# AGENT INSTRUCTIONS - Rust/Python CLI Project Template

**Audience**: AI coding agents | **Purpose**: Mandatory requirements for project development and delivery

---

## 1. CORE AGENT RULES (MANDATORY)

### Rule 1: Security & Quality Assurance
**MUST execute before EVERY delivery:**
```bash
cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test --all --no-fail-fast && cargo build --release
```
**Python**: `pytest && black --check . && ruff check .` (if applicable)
**Prohibition**: NEVER deliver untested, unformatted, or broken code.

### Rule 2: Project File Hygiene
**Required files**: README.md, Makefile, Dockerfile, CHANGELOG.md, .gitignore, .version-tracking.md (create from .version-tracking-template.md)
**Test artifact management**: Add to `.gitignore`: `output.txt`, `test_output.*`, `*.log`, `*.tmp`, `temp/`, `test_data/`; Remove from git: `git rm --cached <file>`

### Rule 3: Cross-Platform Install Scripts
**Create if missing**: `install.sh` (Linux/Unix/macOS), `install.ps1`/`install.bat` (Windows)
**Package manager hierarchy**: System (brew/apt/yum/dnf/pacman/apk/winget/choco) → Language-specific (cargo/pip/go install)
**Requirements**: Detect OS/arch, exit with clear error if unsupported

### Rule 4: Multi-Architecture CI/CD
**Primary matrix**: ubuntu-latest, macos-13, macos-14, windows-latest (x64)
**Cross-compile** (Rust, tag builds): aarch64-unknown-linux-gnu, aarch64-apple-darwin, aarch64-pc-windows-msvc
**Naming**: `{BINARY_NAME}-{platform}-{arch}[.exe]` (linux-x64, linux-aarch64, macos-x64, macos-aarch64, windows-x64.exe, windows-aarch64.exe)
**Workflow variants**: Single-file (standalone projects) vs Reusable (multi-project repos)
**Triggers**: Push → CI; Tag `v*` → CI + cross-builds + release

### Rule 5: Semantic Versioning & Tracking
**Format**: MAJOR.MINOR.PATCH (SemVer 2.0.0)
**Auto-increment**: Breaking → MAJOR, New feature → MINOR, Bug fix → PATCH
**Update locations**: Cargo.toml, CHANGELOG.md, .version-tracking.md
**.version-tracking.md (HIGH DETAIL)**: Function names + file paths + line numbers + implementation specifics + regex patterns + variable names + testing results
**Purpose**: Cross-session agent memory

### Rule 6: Git Branch Strategy
**Required**: `main` (production), `dev` (active development, default working branch)
**Initialization**: `git branch main && git branch dev && git checkout dev`
**Merge strategy**: Features → dev → main (for releases)

### Rule 7: Git Release Workflow
**Process**: Commit → Push dev → Tag (`git tag v1.0.0`) → Push tag → GitHub Actions auto-release
**Commit format**: `{type}: {description}` (feat/fix/docs/refactor/test/chore)
**Release assets**: All platform binaries + SHA256 checksums

---

## 2. PROJECT CONFIGURATION PARAMETERS

### Template Variables
| Variable | Description | Example |
|----------|-------------|---------|
| `APP_NAME` / `BINARY_NAME` | Executable name | terminus, stormbreaker |
| `PROJECT_TYPE` | Language | rust, python |
| `PYTHON_PACKAGE_NAME` | Python dist name | my_tool |
| `SHORT_DESCRIPTION` | One-liner | CLI tool for workspace management |

### Cargo.toml Critical Fields
See Cargo.toml template in repository for full structure. MUST configure:
- `name` and `[[bin]] name` MUST match `APP_NAME`
- `version` follows SemVer 2.0.0 (MAJOR.MINOR.PATCH)
- Include: `authors`, `edition`, `license`, `description`, `repository`, `keywords`, `categories`

### GitHub Actions Environment Variables
```yaml
env:
  PROJECT_TYPE: rust                 # or "python"
  BINARY_NAME: binary_name           # Rust
  PYTHON_PACKAGE_NAME: package_name  # Python
```
**Platform detection**: `${{ runner.os }}`, `${{ runner.arch }}`

### Docker Build Args
See Dockerfile template. MUST update: `APP_NAME`, `APP_VERSION`, `APP_DESCRIPTION`, `ENTRYPOINT` (must match APP_NAME). Note: Project uses dual licensing (GPL-3.0 + Commercial).

### Makefile Variant Selection
| Variant | Use Case | Targets |
|---------|----------|---------|
| **Lite** | Simple projects, KISS | all, build, release, install, run, test, clean |
| **Pro** | Flagship/complex, full CI parity | Lite + check, fmt, clippy, run-release, ci |

**Decision**: New/simple → Lite; Flagship/production → Pro
**Variables**: `APP_NAME`, `CARGO`, `BUILD_DIR`, `INSTALL_PATH`

---

## 3. DEVELOPMENT WORKFLOW

### Initial Setup
1. Verify/create branches: `git branch main && git branch dev && git checkout dev`
2. Verify scaffolding: README.md, Makefile, Dockerfile, CHANGELOG.md, .gitignore, .version-tracking.md, .github/workflows/, .github/dependabot.yml

### Development Cycle
**Per change request:**
1. **Code changes**: Implement feature/fix, validate inputs, handle errors
2. **Quality checks**: `cargo fmt && cargo clippy -- -D warnings && cargo test && cargo build --release`
3. **Version increment**: Determine MAJOR/MINOR/PATCH, update Cargo.toml (e.g., 1.0.0 → 1.1.0)
4. **Update CHANGELOG.md**:
   ```markdown
   ## [1.1.0] - 2025-01-06
   ### Added
   - New feature: User authentication
   ### Fixed
   - Parser bug (src/parser.rs:42)
   ```
5. **Update .version-tracking.md (HIGH DETAIL)**:
   - Create from `.version-tracking-template.md` if not exists
   - See template for required format: function names, file paths:line numbers, implementation specifics, testing results

### Commit Workflow
```bash
git add src/ tests/ Cargo.toml CHANGELOG.md .version-tracking.md
git commit -m "feat: Add user authentication system"
git push origin dev
```

### Release Workflow
**Option A**: `git checkout main && git merge dev && git push origin main && git tag v1.1.0 && git push origin v1.1.0`
**Option B**: Create PR dev → main, merge, then tag
**Result**: GitHub Actions builds all binaries, creates release with checksums

---

## 4. MULTI-ARCHITECTURE BUILD REQUIREMENTS

### Primary Build Matrix (Native)
| OS | Runner | Arch | Suffix |
|----|--------|------|--------|
| Linux | ubuntu-latest | x64 | - |
| macOS Intel | macos-13 | x64 | - |
| macOS ARM | macos-14 | ARM64 | - |
| Windows | windows-latest | x64 | .exe |

### Cross-Compilation (Rust, Tag Builds)
| Target | Platform | Tool | Runner |
|--------|----------|------|--------|
| aarch64-unknown-linux-gnu | Linux ARM64 | cross | ubuntu-latest |
| aarch64-apple-darwin | macOS ARM64 | native | macos-latest |
| aarch64-pc-windows-msvc | Windows ARM64 | native | windows-latest |

**Install cross**: `cargo install cross --git https://github.com/cross-rs/cross`
**Build**: `cross build --release --target aarch64-unknown-linux-gnu`

### Release Assets (Per Release)
**Rust**:
```
{BINARY_NAME}-linux-x64 + .sha256
{BINARY_NAME}-linux-aarch64 + .sha256
{BINARY_NAME}-macos-x64 + .sha256
{BINARY_NAME}-macos-aarch64 + .sha256
{BINARY_NAME}-windows-x64.exe + .sha256
{BINARY_NAME}-windows-aarch64.exe + .sha256
```
**Python**: `dist/{PACKAGE}-{version}.tar.gz`, `dist/{PACKAGE}-{version}-py3-none-any.whl`

### Checksum Generation
```bash
for f in ${BINARY_NAME}-*; do [ -f "$f" ] && sha256sum "$f" > "$f.sha256"; done
```

### Release Notes Extraction
**Priority**: CHANGELOG.md (version match) → git tag message → auto-generated
```bash
VERSION="${GITHUB_REF#refs/tags/}"
awk "/^## $VERSION/,/^## /{if (/^## $VERSION/) f=1; else if (/^## /) f=0; if (f && !/^## $VERSION/) print}" CHANGELOG.md > release_notes.md
```

---

## 5. QUALITY GATES & PRE-DELIVERY CHECKLIST

### Code Quality (MUST Pass)
- [ ] `cargo build --release` → Success
- [ ] `cargo test` → All pass (X/X)
- [ ] `cargo clippy --all-targets -- -D warnings` → 0 warnings
- [ ] `cargo fmt --all -- --check` → Formatted
- [ ] `cargo audit` → No vulnerabilities (optional)
**Python**: `pytest`, `black --check .`, `ruff check .`

### Documentation (MUST Be Complete)
- [ ] README.md: All sections filled (see Section 7 for details)
- [ ] CHANGELOG.md: Version entry exists with changes
- [ ] .version-tracking.md: High-detail session notes (function names, file paths:line numbers, specifics)
- [ ] Cargo.toml/pyproject.toml: Version, description, keywords updated
- [ ] Makefile: APP_NAME matches binary
- [ ] Dockerfile: APP_NAME, ENTRYPOINT, APP_VERSION match binary

### Git Hygiene
- [ ] .gitignore excludes test artifacts, branches (main/dev) exist, currently on dev

### CI/CD & Scripts
- [ ] .github/workflows/ci-release.yml configured with correct PROJECT_TYPE and BINARY_NAME
- [ ] .github/dependabot.yml exists
- [ ] Install scripts (scripts/install.sh, .ps1, .bat) created with OS/arch detection

### Version Tracking
- [ ] SemVer 2.0.0 applied (Breaking → MAJOR, Feature → MINOR, Fix → PATCH)
- [ ] Synchronized across Cargo.toml, CHANGELOG.md, .version-tracking.md

---

## 6. FINAL DELIVERY FORMAT

**When project complete, provide summary with:**
- Summary: Version, project type, purpose, status (Production Ready)
- Deliverables: Source, documentation (README/CHANGELOG/.version-tracking), build system (Makefile/Dockerfile), CI/CD, install scripts, git (main+dev branches)
- Quality gates: Compilation ✅, Tests ✅, Linting ✅, Formatting ✅, Security ✅
- Next steps: Review → Push (dev, main) → Tag (`v{X.Y.Z}`) → Push tags → GitHub Actions auto-release
- Release assets: 6 platform binaries (linux/macos/windows × x64/aarch64) + SHA256 checksums

---

## 7. REPOSITORY FILE REFERENCE

**Purpose**: File catalog - templates vs production config. **CRITICAL**: Preserve template structure/formatting, only replace placeholders.

### Templates (MUST REPLACE PLACEHOLDERS)
| File | Action | Notes |
|------|--------|-------|
| README.md | Replace `program_name`, fill Background/Lore, Features, Flags | Preserve ToC, headers, emojis |
| Makefile | Replace `APP_NAME` | Lite (simple) vs Pro (flagship: +fmt,clippy,check,ci) |
| Dockerfile | Replace `APP_NAME`, `APP_VERSION`, `ENTRYPOINT` | Preserve multi-stage build, ARG order |
| CHANGELOG.md | Add entries per Rule 5 | Format: `## [X.Y.Z] - YYYY-MM-DD` + Added/Changed/Fixed |
| .version-tracking.md | Create from template, HIGH DETAIL | See .version-tracking-template.md (260 lines) |
| scripts/install.sh | Replace `APP_NAME`, `REPO` | OS/arch detection, package mgr hierarchy |
| scripts/install.ps1 | Replace `$APP_NAME`, `$REPO` | Windows PowerShell installer |
| scripts/install.bat | Replace `APP_NAME`, `REPO` | Windows Batch installer |

### CI/CD (REPLACE PLACEHOLDERS)
| File | Action | Notes |
|------|--------|-------|
| .github/workflows/ci-release.yml | Set `PROJECT_TYPE`, `BINARY_NAME`/`PYTHON_PACKAGE_NAME` | x64+ARM64, cross-compile, auto-release |
| .github/dependabot.yml | Enable pip if Python (uncomment lines 18-22) | Production-ready |
| .github/FUNDING.yml | Maintain or remove | Optional: GitHub Sponsors |

### Legal (DO NOT MODIFY)
| File | Purpose |
|------|---------|
| LICENSE | GNU GPL-3.0 (open source) |
| COMMERCIAL_LICENSE.md | Commercial licensing options |
| COMMERCIAL-EULA.md | Commercial EULA terms |
| CLA.md | Contributor License Agreement |
| TRADEMARK_POLICY.md | Brand/trademark usage policy |

**Note**: Dual-licensed project (GPL-3.0 + Commercial) - respect both tracks.

### Source Agent Files (REFERENCE ONLY)
ChatGPT_AGENTS.md, Claude_AGENTS.md, Orginal_AGENTS.md - Consolidated into AGENT.md, reference if needed.

### Git Submodules (USE GIT COMMANDS)
| Submodule | Purpose | Init/Update |
|-----------|---------|-------------|
| rules/ | Claude Code rules, hooks, workflow | `git submodule update --init --recursive` / `--remote rules` |
| Skills/ | Reusable agent skills, templates | `git submodule update --init --recursive` / `--remote Skills` |

### Config Files
.claude/settings.local.json (local, don't commit), .gitmodules (submodule defs), .gitignore (test artifacts per Rule 2)

---

**END OF AGENT INSTRUCTIONS**
