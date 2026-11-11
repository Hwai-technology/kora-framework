# Kora Installation & Toolchain Setup

This guide walks you through installing, updating, and managing the Kora compiler and developer toolchain on Linux and macOS environments.

---

## ⚡ 1. Automated One-Command Installation

The easiest way to install Kora and its entire development suite (binary, shell auto-completions, Linux MIME types, and editor extensions) is using the automated setup script:

```bash
# From the project repository root:
./scripts/setup.sh
```

### What this script installs:
1. **Kora Native Compiler Binary**: Compiles the release binary with Cargo and installs it to `~/.local/bin/kora` (and optionally `/usr/local/bin/kora` if root).
2. **Environment PATH**: Automatically adds `~/.local/bin` to your `~/.bashrc` / `~/.zshrc`.
3. **Linux Shared-MIME Database**: Registers `text/x-kora` for `.kora` and `.kr` file extensions system-wide.
4. **Shell Auto-Completions**: Installs native completion scripts for:
   - **Bash**: `~/.local/share/bash-completion/completions/kora`
   - **Zsh**: `~/.zsh/completion/_kora`
   - **Fish**: `~/.config/fish/completions/kora.fish`
5. **IDE Extensions**: Automatically installs and syncs the Kora extension into:
   - Antigravity IDE: `~/.antigravity-ide/extensions/`
   - Visual Studio Code: `~/.vscode/extensions/`

---

## 🔄 2. Updating Kora

Whenever you pull changes or modify compiler sources, update your system in one step:

```bash
# Using the CLI directly:
kora update

# Or via the shell script:
./scripts/update.sh
```

Both commands will recompile the compiler in `--release` mode, refresh all shell completions, and sync all editor definitions.

---

## 🗑️ 3. Clean Uninstallation

If you ever need to completely remove Kora from your system:

```bash
./scripts/uninstall.sh
```

This safely deletes:
- The `kora` executable in `~/.local/bin/` and `/usr/local/bin/`.
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
git clone https://github.com/kora-lang/kora.git
cd kora

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
kora --version
```

Output:
```text
kora 0.1.0-alpha.1
```

Run an inline verification check:

```bash
kora help
```
