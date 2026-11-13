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
SCRIPT_DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"
if [[ -f "$SCRIPT_DIR/../Cargo.toml" ]]; then
    ROOT_DIR="$( cd -P "$SCRIPT_DIR/.." >/dev/null 2>&1 && pwd )"
elif [[ -f "$SCRIPT_DIR/Cargo.toml" ]]; then
    ROOT_DIR="$SCRIPT_DIR"
else
    ROOT_DIR="$( cd -P "$SCRIPT_DIR/.." >/dev/null 2>&1 && pwd )"
fi

BIN_TARGET_DIR="$HOME/.local/bin"
MIME_TARGET_DIR="$HOME/.local/share/mime/packages"
COMPLETION_TARGET_DIR="$HOME/.local/share/bash-completion/completions"

do_uninstall() {
    if [[ -f "$ROOT_DIR/scripts/uninstall.sh" ]]; then
        "$ROOT_DIR/scripts/uninstall.sh"
    else
        echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
        echo -e "${COLOR_RED}  🗑️  Kora Language & Framework — System Uninstaller           ${COLOR_RESET}"
        echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
        echo -e "\n${COLOR_BLUE}[1/4] Removing Kora binaries...${COLOR_RESET}"
        rm -f "$HOME/.local/bin/kora"
        if [[ $EUID -eq 0 ]]; then
            rm -f "/usr/local/bin/kora"
        fi
        echo -e "${COLOR_GREEN}✓ Binary removed from ~/.local/bin/kora${COLOR_RESET}"
        echo -e "\n${COLOR_BLUE}[2/4] Removing shell auto-completions...${COLOR_RESET}"
        rm -f "$HOME/.local/share/bash-completion/completions/kora"
        rm -f "$HOME/.zsh/completion/_kora"
        rm -f "$HOME/.config/fish/completions/kora.fish"
        echo -e "${COLOR_GREEN}✓ Shell completions removed (Bash, Zsh, Fish).${COLOR_RESET}"
        echo -e "\n${COLOR_BLUE}[3/4] Removing Linux Shared MIME registrations...${COLOR_RESET}"
        rm -f "$HOME/.local/share/mime/packages/kora.xml"
        if command -v update-mime-database &>/dev/null; then
            update-mime-database "$HOME/.local/share/mime" &>/dev/null || true
            echo -e "${COLOR_GREEN}✓ MIME database updated.${COLOR_RESET}"
        fi
        echo -e "\n${COLOR_BLUE}[4/4] Removing editor extensions...${COLOR_RESET}"
        rm -rf "$HOME/.antigravity-ide/extensions/kora-lang."*
        rm -rf "$HOME/.vscode/extensions/kora-vscode"
        echo -e "${COLOR_GREEN}✓ Editor extensions removed.${COLOR_RESET}"
        echo -e "\n${COLOR_GREEN}✓ Kora has been cleanly uninstalled from your system.${COLOR_RESET}"
    fi
}

# Ensure cargo is accessible if installed via rustup
if ! command -v cargo &>/dev/null; then
    if [[ -f "$HOME/.cargo/env" ]]; then
        . "$HOME/.cargo/env"
    elif [[ -d "$HOME/.cargo/bin" ]]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

# Check if Kora is already installed
if [[ -f "$BIN_TARGET_DIR/kora" ]] || command -v kora &>/dev/null; then
    INSTALLED_VER=$("$BIN_TARGET_DIR/kora" --version 2>/dev/null || kora --version 2>/dev/null || echo "kora")

    # Handle explicit command line flags
    if [[ "$1" == "--uninstall" ]]; then
        do_uninstall
        exit 0
    elif [[ "$1" == "--clean" || "$1" == "--reinstall" ]]; then
        echo -e "${COLOR_CYAN}Performing clean fresh reinstall...${COLOR_RESET}"
        do_uninstall
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
                do_uninstall
                ;;
            3)
                do_uninstall
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
    if ! command -v cargo &>/dev/null; then
        echo -e "${COLOR_YELLOW}  ⚠️  Cargo is not installed or not in PATH.${COLOR_RESET}"
        AUTO_INSTALL=""
        if [ -t 0 ]; then
            read -rp "  Would you like to install Rust and Cargo via rustup automatically? [Y/n] " AUTO_INSTALL
        elif [ -r /dev/stdin ]; then
            read -r AUTO_INSTALL || AUTO_INSTALL="y"
        fi
        AUTO_INSTALL="${AUTO_INSTALL:-y}"
        if [[ "$AUTO_INSTALL" =~ ^[Yy] ]]; then
            echo -e "${COLOR_CYAN}  Installing Rust toolchain (rustup)...${COLOR_RESET}"
            if command -v curl &>/dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            elif command -v wget &>/dev/null; then
                wget -qO- https://sh.rustup.rs | sh -s -- -y
            else
                echo -e "${COLOR_RED}  error: neither curl nor wget found to download rustup.${COLOR_RESET}"
            fi
            if [[ -f "$HOME/.cargo/env" ]]; then
                . "$HOME/.cargo/env"
            fi
        fi
    fi

    if command -v cargo &>/dev/null; then
        echo -e "${COLOR_CYAN}  Building native release binary from source with Cargo...${COLOR_RESET}"
        cd "$ROOT_DIR"
        cargo build --release -p kora-cli
        RELEASE_BIN="$ROOT_DIR/target/release/kora"
    else
        echo -e "${COLOR_YELLOW}  Cargo unavailable. Attempting to fetch precompiled release binary...${COLOR_RESET}"
    fi
fi

if [[ -z "$RELEASE_BIN" || ! -f "$RELEASE_BIN" ]]; then
    echo -e "${COLOR_CYAN}  Fetching precompiled release binary from GitHub Releases...${COLOR_RESET}"
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCH="$(uname -m)"
    case "$ARCH" in
        x86_64|amd64) ARCH="x86_64" ;;
        aarch64|arm64)
            if [[ "$OS" == "darwin" ]]; then
                ARCH="arm64"
            else
                ARCH="aarch64"
            fi
            ;;
        *) ARCH="x86_64" ;;
    esac

    TMP_DIR="$(mktemp -d)"
    TARBALL="kora-latest-${OS}-${ARCH}.tar.gz"
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

# Ensure ~/.local/bin is in PATH across all active shell configs
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo -e "${COLOR_CYAN}  Adding ~/.local/bin to PATH...${COLOR_RESET}"
    if [[ -f "$HOME/.zshrc" ]] || [[ "$SHELL" == *"zsh"* ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
    fi
    if [[ -f "$HOME/.bashrc" ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
    fi
    if [[ -f "$HOME/.bash_profile" ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bash_profile"
    fi
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

# 5. Install / Update Antigravity IDE, VS Code, and Cursor Extensions
echo -e "\n${COLOR_BLUE}[5/5] Installing & Synchronizing editor extensions for VS Code, Cursor, and Antigravity IDE...${COLOR_RESET}"

VSIX_PATH="$ROOT_DIR/integrations/vscode/kora-vscode-0.1.0.vsix"

# If VSIX does not exist, package it automatically
if [[ ! -f "$VSIX_PATH" ]] && [[ -d "$ROOT_DIR/integrations/vscode" ]]; then
    if command -v python3 &>/dev/null; then
        python3 -c "
import os, zipfile
vsix_path = '$VSIX_PATH'
base_dir = '$ROOT_DIR/integrations/vscode'
manifest = '''<?xml version=\"1.0\" encoding=\"utf-8\"?>
<PackageManifest Version=\"2.0.0\" xmlns=\"http://schemas.microsoft.com/developer/vsx-schema/2011\" xmlns:d=\"http://schemas.microsoft.com/developer/vsx-schema-design/2011\">
  <Metadata>
    <Identity Language=\"en-US\" Id=\"kora-vscode\" Version=\"0.1.0\" Publisher=\"kora-lang\"/>
    <DisplayName>Kora Language Support</DisplayName>
    <Description xml:space=\"preserve\">Official syntax highlighting, rich language configuration, snippets, and real-time compiler diagnostics for the Kora programming language and framework.</Description>
    <Tags>kora,rust,backend,syntax,framework</Tags>
    <Categories>Programming Languages,Snippets,Linters</Categories>
  </Metadata>
  <Installation>
    <InstallationTarget Id=\"Microsoft.VisualStudio.Code\"/>
  </Installation>
  <Dependencies/>
  <Assets>
    <Asset Type=\"Microsoft.VisualStudio.Code.Manifest\" Path=\"extension/package.json\" Addressable=\"true\"/>
  </Assets>
</PackageManifest>
'''
content_types = '''<?xml version=\"1.0\" encoding=\"utf-8\"?>
<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">
  <Default Extension=\"json\" ContentType=\"application/json\" />
  <Default Extension=\"vsixmanifest\" ContentType=\"text/xml\" />
  <Default Extension=\"js\" ContentType=\"application/javascript\" />
  <Default Extension=\"svg\" ContentType=\"image/svg+xml\" />
  <Default Extension=\"png\" ContentType=\"image/png\" />
  <Default Extension=\"md\" ContentType=\"text/markdown\" />
</Types>
'''
with zipfile.ZipFile(vsix_path, 'w', zipfile.ZIP_DEFLATED) as z:
    z.writestr('[Content_Types].xml', content_types)
    z.writestr('extension.vsixmanifest', manifest)
    for root, dirs, files in os.walk(base_dir):
        for f in files:
            if f.endswith('.vsix'): continue
            fp = os.path.join(root, f)
            arcname = os.path.join('extension', os.path.relpath(fp, base_dir))
            z.write(fp, arcname)
" 2>/dev/null || true
    fi
fi

# 5.1 Execute extension install command via CLI if available
for EDITOR_BIN in code code-insiders cursor codium windsurf; do
    if command -v "$EDITOR_BIN" &>/dev/null; then
        echo -e "${COLOR_CYAN}  Found editor CLI: $EDITOR_BIN${COLOR_RESET}"
        if [[ -f "$VSIX_PATH" ]]; then
            echo -e "${COLOR_CYAN}  Executing: $EDITOR_BIN --install-extension \"$VSIX_PATH\" --force${COLOR_RESET}"
            "$EDITOR_BIN" --install-extension "$VSIX_PATH" --force 2>/dev/null && {
                echo -e "${COLOR_GREEN}✓ Extension successfully installed into $EDITOR_BIN.${COLOR_RESET}"
            } || true
        else
            "$EDITOR_BIN" --install-extension "kora-lang.kora-vscode" --force 2>/dev/null || true
        fi
    fi
done

# 5.2 Direct directory installation for Antigravity IDE, VS Code, and Cursor
TARGET_EXT_DIRS=(
    "$HOME/.antigravity-ide/extensions/kora-lang.kora-vscode-0.1.0"
    "$HOME/.antigravity/extensions/kora-lang.kora-vscode-0.1.0"
    "$HOME/.vscode/extensions/kora-lang.kora-vscode-0.1.0"
    "$HOME/.vscode/extensions/kora-vscode"
    "$HOME/.vscode-insiders/extensions/kora-vscode"
    "$HOME/.cursor/extensions/kora-vscode"
    "$HOME/.cursor/extensions/kora-lang.kora-vscode-0.1.0"
    "$HOME/.windsurf/extensions/kora-vscode"
)

# Unpack from VSIX if available, or copy from source
for EXT_PATH in "${TARGET_EXT_DIRS[@]}"; do
    mkdir -p "$EXT_PATH" 2>/dev/null || true
    if [[ -f "$VSIX_PATH" ]] && command -v unzip &>/dev/null; then
        TMP_EXT="$(mktemp -d)"
        if unzip -q -o "$VSIX_PATH" -d "$TMP_EXT" 2>/dev/null && [[ -d "$TMP_EXT/extension" ]]; then
            cp -rf "$TMP_EXT/extension/"* "$EXT_PATH/" 2>/dev/null || true
        fi
        rm -rf "$TMP_EXT"
    elif [[ -d "$ROOT_DIR/integrations/vscode" ]]; then
        mkdir -p "$EXT_PATH/client" "$EXT_PATH/icons" "$EXT_PATH/snippets" "$EXT_PATH/syntaxes" 2>/dev/null || true
        cp -f "$ROOT_DIR/integrations/vscode/package.json" "$EXT_PATH/package.json" 2>/dev/null || true
        cp -f "$ROOT_DIR/integrations/vscode/kora-icons.json" "$EXT_PATH/kora-icons.json" 2>/dev/null || true
        cp -f "$ROOT_DIR/integrations/vscode/language-configuration.json" "$EXT_PATH/language-configuration.json" 2>/dev/null || true
        cp -f "$ROOT_DIR/integrations/vscode/client/extension.js" "$EXT_PATH/client/extension.js" 2>/dev/null || true
        cp -rf "$ROOT_DIR/integrations/vscode/icons/"* "$EXT_PATH/icons/" 2>/dev/null || true
        cp -rf "$ROOT_DIR/integrations/vscode/snippets/"* "$EXT_PATH/snippets/" 2>/dev/null || true
        cp -rf "$ROOT_DIR/integrations/vscode/syntaxes/"* "$EXT_PATH/syntaxes/" 2>/dev/null || true
    else
        # Remote fallback
        BASE_RAW="https://raw.githubusercontent.com/Hwai-technology/kora-framework/main/integrations/vscode"
        mkdir -p "$EXT_PATH/client" "$EXT_PATH/syntaxes" "$EXT_PATH/snippets" 2>/dev/null || true
        curl -fsSL "$BASE_RAW/package.json" -o "$EXT_PATH/package.json" 2>/dev/null || true
        curl -fsSL "$BASE_RAW/language-configuration.json" -o "$EXT_PATH/language-configuration.json" 2>/dev/null || true
        curl -fsSL "$BASE_RAW/client/extension.js" -o "$EXT_PATH/client/extension.js" 2>/dev/null || true
        curl -fsSL "$BASE_RAW/syntaxes/kora.tmLanguage.json" -o "$EXT_PATH/syntaxes/kora.tmLanguage.json" 2>/dev/null || true
        curl -fsSL "$BASE_RAW/snippets/kora.snippets.json" -o "$EXT_PATH/snippets/kora.snippets.json" 2>/dev/null || true
    fi
done
echo -e "${COLOR_GREEN}✓ Editor extension automatically deployed to all IDE profiles.${COLOR_RESET}"

# 5.3 Automatically integrate with Material Icon Theme to prevent conflicts
KORA_SVG="$ROOT_DIR/integrations/vscode/icons/kora-file.svg"
if [[ -f "$KORA_SVG" ]]; then
    while IFS= read -r MAT_DIR; do
        if [[ -d "$MAT_DIR/icons" ]]; then
            cp -f "$KORA_SVG" "$MAT_DIR/icons/kora.svg" 2>/dev/null || true
            echo -e "${COLOR_GREEN}✓ Seamlessly integrated Kora icons into Material Icon Theme (${MAT_DIR##*/}).${COLOR_RESET}"
        fi
    done < <(find "$HOME" -maxdepth 5 -type d -name "pkief.material-icon-theme*" 2>/dev/null)
fi

# Verification
echo -e "\n${COLOR_CYAN}================================================================${COLOR_RESET}"
echo -e "${COLOR_GREEN}  🎉 Kora Environment Setup Completed Successfully!            ${COLOR_RESET}"
echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
echo -e "Installed Version: $(${BIN_TARGET_DIR}/kora version)"
echo -e "Available Commands: check, types, ast, tokens, codegen, build, run, bench, db, pkg, init, version"
echo -e "File Extensions: .kora and .kr are now recognized system-wide."
echo -e "\nRun 'kora help' or 'kora version' in your terminal."
