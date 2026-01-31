# plainpad

> VERSION: 0.5.0  \
> DESCRIPTION: A fast, boring, native Rust notepad replacement.  \
> AUTHOR: Gilles Biagomba  \
> LICENSE: GPL-3.0

---

## Background / Lore
plainpad is built for the quiet moments between heavy IDEs and bare-bones Notepad.
It favors speed, predictability, and a clean UI over clever automation.
The goal is a trustworthy desktop editor that feels immediate on Windows while staying portable.

---

## Table of Contents
- [Background / Lore](#background--lore)
- [Features](#features)
- [Installation](#installation)
  - [Using GitHub Releases](#using-github-releases)
  - [Using Install Scripts](#using-install-scripts)
  - [Compiling From Source](#compiling-from-source)
- [Usage](#usage)
- [Keyboard Shortcuts](#keyboard-shortcuts)
  - [Running Tests](#running-tests)
  - [Using the Makefile](#using-the-makefile)
- [Contributing](#contributing)
- [License](#license)

---

## Features
- Native Rust desktop app built with egui/eframe
- Plain UTF-8 text editing with ropey-backed storage
- Tabbed documents with dirty-state tracking
- Windows-first UX with cross-platform builds
- No AI, telemetry, plugins, or cloud integrations

---

## Installation

### Using GitHub Releases
Download the precompiled binary for your platform:

- Linux: `plainpad-linux-x64` or `plainpad-linux-aarch64`
- macOS: `plainpad-macos-x64` or `plainpad-macos-aarch64`
- Windows: `plainpad-windows-x64.exe` or `plainpad-windows-aarch64.exe`

| Platform | Architecture | Binary |
|----------|-------------|--------|
| Linux | x64 | `plainpad-linux-x64` |
| Linux | ARM64 | `plainpad-linux-aarch64` |
| macOS | x64 | `plainpad-macos-x64` |
| macOS | ARM64 | `plainpad-macos-aarch64` |
| Windows | x64 | `plainpad-windows-x64.exe` |
| Windows | ARM64 | `plainpad-windows-aarch64.exe` |

Make it executable and move it to your PATH:
```bash
chmod +x plainpad-*
sudo mv plainpad-* /usr/local/bin/plainpad
```

---

### Using Install Scripts

**Linux/macOS/Unix:**
```bash
curl -sSL https://raw.githubusercontent.com/gbiagomba/Plainpad/main/scripts/install.sh | bash
# Or download and run locally:
wget https://raw.githubusercontent.com/gbiagomba/Plainpad/main/scripts/install.sh
chmod +x install.sh
./install.sh
```

**Windows (PowerShell - Run as Administrator):**
```powershell
irm https://raw.githubusercontent.com/gbiagomba/Plainpad/main/scripts/install.ps1 | iex
# Or download and run locally:
Invoke-WebRequest -Uri https://raw.githubusercontent.com/gbiagomba/Plainpad/main/scripts/install.ps1 -OutFile install.ps1
.\install.ps1
```

**Windows (Batch - Run as Administrator):**
```batch
curl -L https://raw.githubusercontent.com/gbiagomba/Plainpad/main/scripts/install.bat -o install.bat
install.bat
```

---

### Compiling From Source

```
git clone https://github.com/gbiagomba/plainpad
cd plainpad
cargo build --release
```

Binaries will appear in:

```
target/release/
```

---

## Usage
Launch `plainpad` from your desktop environment, Start Menu, or by running `plainpad` in a terminal.
This is a GUI application and does not require CLI flags.

---

## Keyboard Shortcuts
- Ctrl+N: New tab
- Ctrl+O: Open file
- Ctrl+S: Save
- Ctrl+Shift+S: Save As
- Ctrl+W: Close tab
- Ctrl+Tab / Ctrl+Shift+Tab: Next/previous tab
- Ctrl+Z / Ctrl+Y: Undo/redo (when editor is focused)
- Ctrl+A / C / V / X: Select all / Copy / Paste / Cut

---

### Running Tests

```
make ci
# or
cargo test
```

---

### Using the Makefile

**Build the app**

```
make build
```

**Run the app**

```
make run
```

**Lint + test + release**

```
make ci
```

---

## Contributing

Pull requests are welcome.

If proposing major changes, please open an issue first to discuss alignment with project goals.

### Before submitting

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`make ci`)
4. Commit changes (`git commit -m 'feat: Add amazing feature'`)
5. Push to branch (`git push origin feature/amazing-feature`)
6. Open Pull Request

---

## License

This project is **dual-licensed**.

### Open Source License (GPLv3)
plainpad is available under the **GNU General Public License v3.0 (GPLv3)**.
Use of the software under GPLv3 is subject to the terms and obligations of that license, including its copyleft requirements. See `LICENSE` for details.

### Commercial License
For organizations that require **proprietary internal use** without GPLv3 obligations, a **Commercial License** is available.

The Commercial License allows:
- Internal organizational use
- Private modification
- Use by employees and contractors

The Commercial License does **not** allow:
- Redistribution or resale
- SaaS, hosted, or API offerings
- Embedding into third-party products or services
- Use of project branding without permission

Commercial licenses are governed by the **Commercial End User License Agreement (EULA)** located in `COMMERCIAL-EULA.md`.

### Choosing a License
- If you are building or distributing open-source software: **use GPLv3**
- If you are using the software internally and wish to keep modifications proprietary: **purchase a Commercial License**

For commercial licensing inquiries, contact:  \
gilles.infosec@gmail.com

---

**Built with Rust | Secured by Design | Production Ready**
