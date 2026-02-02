# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Windows installer (Setup.exe) with Start Menu and Desktop shortcuts via Velopack
- Help menu with "Check for Updates" option for auto-update capability
- Delta updates support for faster subsequent updates

## [0.5.4] - 2026-01-31

### Added
- Re-enabled ARM64 builds for Linux, macOS, and Windows in CI

## [0.5.3] - 2026-01-31

### Changed
- Updated macOS CI runner from macos-13 to macos-latest in AGENT.md Rule 4
- Synced docs/AI_INSTRUCTIONS.md with AGENT.md

### Removed
- Deleted template agent files (docs/Agents/)
- Removed empty Skills/ and rules/ directories
- Removed Windows artifact (nul file)

### Fixed
- Cargo.lock now tracked in git (removed from .gitignore for reproducible builds)
- Added nul to .gitignore to prevent future Windows artifacts

## [0.5.2] - 2026-01-31

### Added
- Force Quit option to close the app immediately

## [0.5.1] - 2026-01-31

### Added
- File menu Close All action for tab cleanup

### Fixed
- Replace button now replaces the next match when no selection exists
- Quit prompt discard option now exits as expected

## [0.5.0] - 2026-01-31

### Added
- Find/replace dialog with regex search and replace all
- Tab management actions to close tabs left or right of the active tab
- Chrome-style tab shortcuts (Ctrl+1..9) and quit shortcut (Ctrl+Shift+W)
- Quit confirmation when unsaved edits are open

## [0.4.0] - 2026-01-31

### Added
- View menu toggles for status bar and line numbers
- "+" tab button for quick new tabs
- Save All option that saves non-empty tabs with txt default

### Fixed
- Edit > Paste uses clipboard text directly

## [0.2.6] - 2026-01-31

### Fixed
- Ensure Edit menu actions target the editor focus reliably

### Added
- Save All option that saves non-empty tabs

## [0.2.3] - 2026-01-31

### Fixed
- Restored macOS runner matrix and hardened ARM64 build steps in CI

## [0.2.2] - 2026-01-31

### Changed
- Moved agent prompt templates into `docs/Agents`

## [0.2.1] - 2026-01-31

### Fixed
- Added missing `physical_key` field for synthetic edit events

## [0.2.0] - 2026-01-31

### Added
- File menu print placeholder and Edit menu actions
- Shortcut routing for print command
- Install/release URLs aligned to the Plainpad repository

## [0.1.0] - 2026-01-31

### Added
- Initial plainpad scaffold with egui/eframe app shell
- Ropey-backed document model and tab management
- Native file open/save dialogs via rfd
- Cross-platform install scripts and CI packaging
- Updated project documentation and metadata

---

**AGENT NOTE:** Update this file before EVERY release as per RULE 5
