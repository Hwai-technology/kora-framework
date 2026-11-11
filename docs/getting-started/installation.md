# Kora Installation & Toolchain Setup

This guide walks you through installing, updating, and managing the Kora compiler and developer toolchain across **Linux**, **macOS**, and **Windows**.

---

## ⚡ 1. Automated Installation by Operating System

### Linux & macOS (Bash / Zsh)

Install Kora and its entire development suite (native binary, shell completions, Linux MIME types, and editor extensions) with a single command:

```bash
# Direct install via curl:
curl -fsSL https://raw.githubusercontent.com/Hwai-technology/kora-framework/main/install.sh | bash

# Or from within a cloned repository root:
./scripts/setup.sh
```

### Windows (PowerShell)

Install Kora on Windows using the PowerShell installer (configures `$USERPROFILE\.kora\bin`, adds PATH, registers `.kora`/`.kr` file associations, and syncs VS Code extensions):

```powershell
# Open PowerShell (Terminal) and run:
irm https://raw.githubusercontent.com/Hwai-technology/kora-framework/main/install.ps1 | iex

# Or from within a cloned repository in Command Prompt:
scripts\setup.bat
```

### Linux Portable AppImage (No Dependencies Required)

For zero-install portable execution on any modern Linux distribution:

```bash
# Download latest AppImage
curl -LO https://github.com/Hwai-technology/kora-framework/releases/latest/download/kora-latest-linux-x86_64.AppImage

# Make executable and run directly
chmod +x kora-latest-linux-x86_64.AppImage
./kora-latest-linux-x86_64.AppImage doctor
```

You can move it to your path:
```bash
mkdir -p ~/.local/bin
mv kora-latest-linux-x86_64.AppImage ~/.local/bin/kora
```

### Docker Container (GHCR)

Run Kora inside an isolated container with all system dependencies pre-configured:

```bash
docker pull ghcr.io/hwai-technology/kora:latest
docker run --rm -it -v $(pwd):/workspace ghcr.io/hwai-technology/kora:latest doctor
```

---

## 📦 What the Installers Configure

1. **Kora Native Compiler Binary**:
   - **Linux / macOS**: Installed to `~/.local/bin/kora` (and `/usr/local/bin/kora` if root).
   - **Windows**: Installed to `%USERPROFILE%\.kora\bin\kora.exe`.
2. **Environment PATH**: Automatically adds the binary directory to shell startup files (`~/.bashrc`, `~/.zshrc`, `~/.bash_profile`, or Windows User PATH environment variable).
3. **MIME & File Associations**: Registers `text/x-kora` for `.kora` and `.kr` file extensions system-wide.
4. **Shell Auto-Completions**: Installs native completion scripts for:
   - **Bash**: `~/.local/share/bash-completion/completions/kora`
   - **Zsh**: `~/.zsh/completion/_kora`
   - **Fish**: `~/.config/fish/completions/kora.fish`
5. **IDE Extensions**: Automatically installs and syncs the Kora extension into:
   - Antigravity IDE: `~/.antigravity-ide/extensions/`
   - Visual Studio Code: `~/.vscode/extensions/` or `%USERPROFILE%\.vscode\extensions\`

---

## 🔄 2. Updating Kora

Whenever you pull changes or modify compiler sources, update your system in one step:

```bash
# Using the CLI directly:
kora update

# Or via the shell script (Linux/macOS):
./scripts/update.sh

# On Windows:
scripts\setup.bat
```

---

## 🗑️ 3. Clean Uninstallation

If you ever need to completely remove Kora from your system:

```bash
# Linux / macOS:
./scripts/uninstall.sh

# Windows (PowerShell):
Remove-Item -Recurse -Force "$env:USERPROFILE\.kora"
```

This safely deletes:
- The `kora` executable in `~/.local/bin/`, `/usr/local/bin/`, or `%USERPROFILE%\.kora\bin`.
- All auto-completion files in Bash, Zsh, and Fish.
- System MIME type associations for `.kora` and `.kr`.
- Editor extensions in Antigravity IDE and VS Code.

---

## 🔨 4. Manual Build from Source

If you prefer building manually using Rust and Cargo:

```bash
# Prerequisites: Rust 1.78+ & Cargo installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/Hwai-technology/kora-framework.git
cd kora-framework

# Compile CLI in release mode
cargo build --release -p kora-cli

# Copy binary to your local bin
mkdir -p ~/.local/bin
cp target/release/kora ~/.local/bin/

# Verify installation
kora --version
```

---

## 🧪 5. Verifying Your Installation

Run:

```bash
kora doctor
```

Output:
```text
🩺 Kora Environment & Toolchain Diagnostics (v0.1.0)

  [OK] Kora Toolchain: v0.1.0
  [OK] Rust Toolchain: rustc (available)
  [OK] Database Engines Supported: PostgreSQL, MySQL 8.x, SQLite, Spanner
  [OK] Security Engine: Argon2id (OWASP 2026), HMAC-SHA256 JWT, Memory-Safe Sandboxing
  [OK] Realtime Engine: Native WebSocket Pub/Sub & High-Throughput Actors
  [OK] Project Root: /path/to/project (Valid kora.toml found)
  [OK] Standard Library: Embedded v0.1.0 runtime loaded
```
