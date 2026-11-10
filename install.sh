#!/usr/bin/env bash
# ==============================================================================
# Kora Ecosystem & Editor Extension Installer (Linux / macOS)
# Installs:
#   1. Kora Toolchain CLI (`kora`) compiled natively into ~/.cargo/bin or /usr/local/bin
#   2. Kora Language Support Extension into VS Code, Cursor, and Antigravity
# ==============================================================================

set -euo pipefail

# Text Formatting
BOLD="$(tput bold 2>/dev/null || echo '')"
GREEN="$(tput setaf 2 2>/dev/null || echo '')"
CYAN="$(tput setaf 6 2>/dev/null || echo '')"
YELLOW="$(tput setaf 3 2>/dev/null || echo '')"
RESET="$(tput sgr0 2>/dev/null || echo '')"

echo "${CYAN}${BOLD}"
cat << 'EOF'
    __ __  ____  ____   ___ 
   / //_/ / __ \/ __ \ /   |
  / ,<   / / / / /_/ // /| |
 / /| | / /_/ / _, _// ___ |
/_/ |_| \____/_/ |_|/_/  |_|
Write Less. Build Faster. Run Smarter.
========================================
EOF
echo "${RESET}"

# 1. Determine Repository Root Directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXT_SOURCE_DIR="${SCRIPT_DIR}/integrations/vscode"

if [ ! -d "${EXT_SOURCE_DIR}" ]; then
    echo "${YELLOW}Warning: '${EXT_SOURCE_DIR}' not found relative to script. Searching workspace...${RESET}"
    if [ -d "/home/ahmet/Desktop/Projects/Kora/integrations/vscode" ]; then
        EXT_SOURCE_DIR="/home/ahmet/Desktop/Projects/Kora/integrations/vscode"
    else
        echo "Error: Cannot locate 'integrations/vscode' directory."
        exit 1
    fi
fi

# 2. Build and Install `kora` CLI Binary
echo "${CYAN}==> [1/3] Compiling and Installing Kora CLI...${RESET}"
export PATH="${HOME}/.cargo/bin:${PATH}"

if command -v cargo >/dev/null 2>&1; then
    (
        cd "${SCRIPT_DIR}"
        cargo build --release -p kora-cli
    )
    
    TARGET_BIN_DIR="${HOME}/.local/bin"
    if [ -d "${HOME}/.cargo/bin" ]; then
        TARGET_BIN_DIR="${HOME}/.cargo/bin"
    else
        mkdir -p "${TARGET_BIN_DIR}"
    fi

    cp "${SCRIPT_DIR}/target/release/kora" "${TARGET_BIN_DIR}/kora"
    chmod +x "${TARGET_BIN_DIR}/kora"
    echo "${GREEN}✓ Successfully installed 'kora' CLI to: ${TARGET_BIN_DIR}/kora${RESET}"
else
    echo "${YELLOW}Notice: 'cargo' not found in PATH. Skipping CLI binary compilation.${RESET}"
fi

# 3. Target Editor Extensions Directories
echo "${CYAN}==> [2/3] Installing Editor Extensions for Syntax Highlighting...${RESET}"

INSTALL_COUNT=0

install_extension_to() {
    local EDITOR_NAME="$1"
    local EXT_DIR="$2"

    mkdir -p "${EXT_DIR}"
    local TARGET_DIR="${EXT_DIR}/kora-vscode"
    rm -rf "${TARGET_DIR}"
    cp -r "${EXT_SOURCE_DIR}" "${TARGET_DIR}"
    echo "${GREEN}✓ Installed Kora extension to ${EDITOR_NAME}: ${TARGET_DIR}${RESET}"
    INSTALL_COUNT=$((INSTALL_COUNT + 1))
}

# VS Code
install_extension_to "Visual Studio Code" "${HOME}/.vscode/extensions"

# Cursor
install_extension_to "Cursor Editor" "${HOME}/.cursor/extensions"

# Antigravity (Google / AGY IDE)
if [ -d "${HOME}/.gemini/antigravity" ] || [ -d "${HOME}/.antigravity" ]; then
    install_extension_to "Antigravity IDE" "${HOME}/.gemini/antigravity/extensions"
fi

# VS Code Server / Remote Containers
if [ -d "${HOME}/.vscode-server/extensions" ]; then
    install_extension_to "VS Code Remote Server" "${HOME}/.vscode-server/extensions"
fi

# 4. Verification & Testing
echo "${CYAN}==> [3/3] Verifying Installation...${RESET}"
if command -v kora >/dev/null 2>&1; then
    KORA_VER="$(kora --version 2>/dev/null || echo '0.1.0')"
    echo "${GREEN}✓ 'kora' CLI is active and working: ${KORA_VER}${RESET}"
fi

echo ""
echo "${GREEN}${BOLD}🎉 Installation Complete!${RESET}"
echo "You can now open any ${CYAN}.kora${RESET} file in VS Code, Cursor, or Antigravity to enjoy:"
echo "  • ${BOLD}Full Syntax Highlighting${RESET} (Keywords, Structs, Models, DI Classes, Operators)"
echo "  • ${BOLD}Smart Code Snippets${RESET} (Type 'model', 'struct', 'class', 'get', 'post', 'match')"
echo "  • ${BOLD}Bracket Matching & Indentation${RESET}"
echo ""
echo "Try running:"
echo "  ${CYAN}kora check examples/crud_api.kora${RESET}"
echo "  ${CYAN}kora bench${RESET}"
echo ""

