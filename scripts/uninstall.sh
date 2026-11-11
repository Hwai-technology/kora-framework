#!/usr/bin/env bash
# ==============================================================================
# Kora Programming Language & Framework — Clean Uninstaller
# Safely removes the installed binary, shell completions, MIME types, and editor extensions.
# ==============================================================================

set -e

COLOR_BLUE='\033[0;34m'
COLOR_GREEN='\033[0;32m'
COLOR_CYAN='\033[0;36m'
COLOR_RED='\033[0;31m'
COLOR_RESET='\033[0m'

echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"
echo -e "${COLOR_RED}  🗑️  Kora Language & Framework — System Uninstaller           ${COLOR_RESET}"
echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"

# 1. Remove binaries
echo -e "\n${COLOR_BLUE}[1/4] Removing Kora binaries...${COLOR_RESET}"
rm -f "$HOME/.local/bin/kora"
if [[ $EUID -eq 0 ]]; then
    rm -f "/usr/local/bin/kora"
fi
echo -e "${COLOR_GREEN}✓ Binary removed from ~/.local/bin/kora${COLOR_RESET}"

# 2. Remove completions
echo -e "\n${COLOR_BLUE}[2/4] Removing shell auto-completions...${COLOR_RESET}"
rm -f "$HOME/.local/share/bash-completion/completions/kora"
rm -f "$HOME/.zsh/completion/_kora"
rm -f "$HOME/.config/fish/completions/kora.fish"
echo -e "${COLOR_GREEN}✓ Shell completions removed (Bash, Zsh, Fish).${COLOR_RESET}"

# 3. Remove MIME database definitions
echo -e "\n${COLOR_BLUE}[3/4] Removing Linux Shared MIME registrations...${COLOR_RESET}"
rm -f "$HOME/.local/share/mime/packages/kora.xml"
if command -v update-mime-database &>/dev/null; then
    update-mime-database "$HOME/.local/share/mime" &>/dev/null || true
    echo -e "${COLOR_GREEN}✓ MIME database updated.${COLOR_RESET}"
fi

# 4. Remove Editor Extensions
echo -e "\n${COLOR_BLUE}[4/4] Removing editor extensions...${COLOR_RESET}"
rm -rf "$HOME/.antigravity-ide/extensions/kora-lang.kora-vscode-0.1.0"
rm -rf "$HOME/.vscode/extensions/kora-vscode"
echo -e "${COLOR_GREEN}✓ Editor extensions removed.${COLOR_RESET}"

echo -e "\n${COLOR_CYAN}================================================================${COLOR_RESET}"
echo -e "${COLOR_GREEN}  ✓ Kora has been completely uninstalled from this system.      ${COLOR_RESET}"
echo -e "${COLOR_CYAN}================================================================${COLOR_RESET}"

