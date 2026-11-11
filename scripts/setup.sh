#!/usr/bin/env bash
# ==============================================================================
# Kora Programming Language & Framework — Automated Setup & System Installer
#
# Installs:
#  1. `kora` release binary into ~/.local/bin (and /usr/local/bin if available)
#  2. Linux FreeDesktop Shared MIME database (text/x-kora for *.kora & *.kr)
#  3. Shell Auto-Completions (Bash & Zsh)
#  4. Editor tooling for Antigravity IDE and Visual Studio Code
# ==============================================================================

set -e

COLOR_BLUE='\033[0;34m'
COLOR_GREEN='\033[0;32m'
COLOR_CYAN='\033[0;36m'
COLOR_YELLOW='\033[1;33m'
COLOR_RED='\033[0;31m'
COLOR_RESET='\033[0m'

# Resolve root directory following symlinks
SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SOURCE" ]; do
  DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE"
done
ROOT_DIR="$( cd -P "$( dirname "$SOURCE" )/.." >/dev/null 2>&1 && pwd )"

BIN_TARGET_DIR="$HOME/.local/bin"
MIME_TARGET_DIR="$HOME/.local/share/mime/packages"
COMPLETION_TARGET_DIR="$HOME/.local/share/bash-completion/completions"

# Check if Kora is already installed
if [[ -f "$BIN_TARGET_DIR/kora" ]] || command -v kora &>/dev/null; then
    INSTALLED_VER=$("$BIN_TARGET_DIR/kora" --version 2>/dev/null || kora --version 2>/dev/null || echo "kora")

    # Handle explicit command line flags
    if [[ "$1" == "--uninstall" ]]; then
        "$ROOT_DIR/scripts/uninstall.sh"
        exit 0
    elif [[ "$1" == "--clean" || "$1" == "--reinstall" ]]; then
        echo -e "${COLOR_CYAN}Performing clean fresh reinstall...${COLOR_RESET}"
        "$ROOT_DIR/scripts/uninstall.sh"
    elif [[ "$1" != "--yes" && "$1" != "-y" ]]; then
        echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
        echo -e "${COLOR_YELLOW}  ⚠️  Kora is already installed on this system (${INSTALLED_VER})${COLOR_RESET}"
        echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
        echo -e "Choose an action:"
        echo -e "  [1] 🔄 Update / Refresh installation  (rebuild binary, update MIME & completions)"
        echo -e "  [2] 🔨 Clean Reinstall from scratch   (uninstall and perform fresh build)"
        echo -e "  [3] 🗑️  Uninstall Kora from system     (remove binary, completions, and IDE tools)"
        echo -e "  [4] ❌ Exit / Cancel                  (abort operation and exit)"
        echo ""
        
        CHOICE=""
        if [ -t 0 ]; then
            read -rp "Enter choice [1-4] (default: 1): " CHOICE
        elif [ -r /dev/stdin ]; then
            read -r CHOICE || CHOICE="1"
        fi
        CHOICE="${CHOICE:-1}"

        case "$CHOICE" in
            1)
                echo -e "${COLOR_CYAN}Updating existing installation...${COLOR_RESET}"
                ;;
            2)
                echo -e "${COLOR_CYAN}Performing clean fresh reinstall...${COLOR_RESET}"
                "$ROOT_DIR/scripts/uninstall.sh"
                ;;
            3)
                "$ROOT_DIR/scripts/uninstall.sh"
                exit 0
                ;;
            4)
                echo -e "${COLOR_CYAN}Operation cancelled by user. Exiting.${COLOR_RESET}"
                exit 0
                ;;
            *)
                echo -e "${COLOR_RED}Invalid choice. Proceeding with standard update.${COLOR_RESET}"
                ;;
        esac
    fi
fi

echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
echo -e "${COLOR_CYAN}  ⚡ Kora Language & Framework — System Installer & Setup      ${COLOR_RESET}"
echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"

# 1. Compile or Download Release Binary
echo -e "\n${COLOR_BLUE}[1/5] Preparing Kora release binary...${COLOR_RESET}"
if [[ -f "$ROOT_DIR/Cargo.toml" ]] && [[ -f "$ROOT_DIR/crates/kora-cli/Cargo.toml" ]]; then
    echo -e "${COLOR_CYAN}  Building native release binary from source with Cargo...${COLOR_RESET}"
    cd "$ROOT_DIR"
    cargo build --release -p kora-cli
    RELEASE_BIN="$ROOT_DIR/target/release/kora"
else
    echo -e "${COLOR_CYAN}  Fetching precompiled release binary from GitHub Releases...${COLOR_RESET}"
    ARCH="$(uname -m)"
    case "$ARCH" in
        x86_64|amd64) ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        *) ARCH="x86_64" ;;
    esac

    TMP_DIR="$(mktemp -d)"
    TARBALL="kora-latest-linux-${ARCH}.tar.gz"
    DOWNLOAD_URL="https://github.com/Hwai-technology/kora-framework/releases/latest/download/${TARBALL}"

    if curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$TARBALL" 2>/dev/null; then
        tar -xzf "$TMP_DIR/$TARBALL" -C "$TMP_DIR"
        RELEASE_BIN="$TMP_DIR/kora"
    elif command -v cargo &>/dev/null; then
        echo -e "${COLOR_YELLOW}  Release binary download unavailable; compiling from git source...${COLOR_RESET}"
        git clone --depth 1 https://github.com/Hwai-technology/kora-framework.git "$TMP_DIR/kora-src"
        cd "$TMP_DIR/kora-src"
        cargo build --release -p kora-cli
        RELEASE_BIN="$TMP_DIR/kora-src/target/release/kora"
        ROOT_DIR="$TMP_DIR/kora-src"
    else
        echo -e "${COLOR_RED}error: could not download precompiled binary and cargo is not installed.${COLOR_RESET}"
        echo -e "${COLOR_YELLOW}Please install Rust/Cargo (https://rustup.rs) or download a release from:${COLOR_RESET}"
        echo -e "${COLOR_CYAN}https://github.com/Hwai-technology/kora-framework/releases${COLOR_RESET}"
        exit 1
    fi
fi

if [[ ! -f "$RELEASE_BIN" ]]; then
    echo -e "${COLOR_RED}error: build failed, $RELEASE_BIN not found${COLOR_RESET}"
    exit 1
fi
echo -e "${COLOR_GREEN}✓ Release binary ready: $RELEASE_BIN${COLOR_RESET}"

# 2. Install Binary into ~/.local/bin
echo -e "\n${COLOR_BLUE}[2/5] Installing 'kora' into user path ($BIN_TARGET_DIR)...${COLOR_RESET}"
mkdir -p "$BIN_TARGET_DIR"
cp -f "$RELEASE_BIN" "$BIN_TARGET_DIR/kora"
chmod +x "$BIN_TARGET_DIR/kora"

# Ensure ~/.local/bin is in PATH in ~/.bashrc if missing
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo -e "${COLOR_CYAN}  Adding ~/.local/bin to PATH in ~/.bashrc...${COLOR_RESET}"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
    export PATH="$HOME/.local/bin:$PATH"
fi
echo -e "${COLOR_GREEN}✓ Binary installed at: $BIN_TARGET_DIR/kora${COLOR_RESET}"

# Optional /usr/local/bin copy if root or passwordless sudo
if [[ $EUID -eq 0 ]]; then
    cp -f "$RELEASE_BIN" /usr/local/bin/kora
    chmod +x /usr/local/bin/kora
    echo -e "${COLOR_GREEN}✓ Global system binary installed at: /usr/local/bin/kora${COLOR_RESET}"
fi

# 3. Register Linux Shared MIME Database for .kora & .kr
echo -e "\n${COLOR_BLUE}[3/5] Registering Linux Shared-MIME type for .kora & .kr...${COLOR_RESET}"
mkdir -p "$MIME_TARGET_DIR"
if [[ -f "$ROOT_DIR/integrations/linux/kora.xml" ]]; then
    cp -f "$ROOT_DIR/integrations/linux/kora.xml" "$MIME_TARGET_DIR/kora.xml"
    if command -v update-mime-database &>/dev/null; then
        update-mime-database "$HOME/.local/share/mime" &>/dev/null || true
        echo -e "${COLOR_GREEN}✓ MIME database updated: text/x-kora (*.kora, *.kr).${COLOR_RESET}"
    fi
fi

# 4. Install Shell Completions (Bash, Zsh, Fish)
echo -e "\n${COLOR_BLUE}[4/5] Installing Shell completions...${COLOR_RESET}"
mkdir -p "$COMPLETION_TARGET_DIR"
if [[ -f "$ROOT_DIR/integrations/completions/kora.bash" ]]; then
    cp -f "$ROOT_DIR/integrations/completions/kora.bash" "$COMPLETION_TARGET_DIR/kora"
    echo -e "${COLOR_GREEN}✓ Bash completion installed.${COLOR_RESET}"
fi

# Zsh completions
ZSH_COMPLETION_DIR="$HOME/.zsh/completion"
if [[ -f "$ROOT_DIR/integrations/completions/_kora" ]]; then
    mkdir -p "$ZSH_COMPLETION_DIR"
    cp -f "$ROOT_DIR/integrations/completions/_kora" "$ZSH_COMPLETION_DIR/_kora"
    echo -e "${COLOR_GREEN}✓ Zsh completion installed at $ZSH_COMPLETION_DIR/_kora.${COLOR_RESET}"
fi

# Fish completions
FISH_COMPLETION_DIR="$HOME/.config/fish/completions"
if [[ -f "$ROOT_DIR/integrations/completions/kora.fish" ]]; then
    mkdir -p "$FISH_COMPLETION_DIR"
    cp -f "$ROOT_DIR/integrations/completions/kora.fish" "$FISH_COMPLETION_DIR/kora.fish"
    echo -e "${COLOR_GREEN}✓ Fish completion installed.${COLOR_RESET}"
fi

# 5. Install / Update Antigravity IDE and VS Code Extensions
echo -e "\n${COLOR_BLUE}[5/5] Synchronizing editor extensions for Antigravity IDE and VS Code...${COLOR_RESET}"
ANTIGRAVITY_EXT="$HOME/.antigravity-ide/extensions/kora-lang.kora-vscode-0.1.0"
VSCODE_EXT="$HOME/.vscode/extensions/kora-vscode"

for EXT_PATH in "$ANTIGRAVITY_EXT" "$VSCODE_EXT"; do
    mkdir -p "$EXT_PATH/client" "$EXT_PATH/icons" "$EXT_PATH/snippets" "$EXT_PATH/syntaxes"
    cp -f "$ROOT_DIR/integrations/vscode/package.json" "$EXT_PATH/package.json"
    cp -f "$ROOT_DIR/integrations/vscode/kora-icons.json" "$EXT_PATH/kora-icons.json"
    cp -f "$ROOT_DIR/integrations/vscode/language-configuration.json" "$EXT_PATH/language-configuration.json"
    cp -f "$ROOT_DIR/integrations/vscode/client/extension.js" "$EXT_PATH/client/extension.js"
    cp -rf "$ROOT_DIR/integrations/vscode/icons/"* "$EXT_PATH/icons/"
    cp -rf "$ROOT_DIR/integrations/vscode/snippets/"* "$EXT_PATH/snippets/"
    cp -rf "$ROOT_DIR/integrations/vscode/syntaxes/"* "$EXT_PATH/syntaxes/"
done
echo -e "${COLOR_GREEN}✓ Editor extension synced to Antigravity IDE and VS Code.${COLOR_RESET}"

# Verification
echo -e "\n${COLOR_CYAN}================================================================${COLOR_RESET}"
echo -e "${COLOR_GREEN}  🎉 Kora Environment Setup Completed Successfully!            ${COLOR_RESET}"
echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
echo -e "Installed Version: $(${BIN_TARGET_DIR}/kora version)"
echo -e "Available Commands: check, types, ast, tokens, codegen, build, run, bench, db, pkg, init, version"
echo -e "File Extensions: .kora and .kr are now recognized system-wide."
echo -e "\nRun 'kora help' or 'kora version' in your terminal."
