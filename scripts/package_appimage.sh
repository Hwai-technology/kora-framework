#!/usr/bin/env bash
# ==============================================================================
# Kora Framework — Linux AppImage Packaging Script
# Bundles the compiled Kora binary into a self-contained portable .AppImage
# ==============================================================================

set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$ROOT_DIR/target/appimage"
APPDIR="$BUILD_DIR/Kora.AppDir"
TARGET_ARG="${1:-$ROOT_DIR/target/release}"
mkdir -p "$TARGET_ARG"
OUTPUT_DIR="$(cd "$TARGET_ARG" && pwd)"

echo "📦 Packaging Kora Linux AppImage..."

# 1. Ensure release binary exists
if [[ ! -f "$ROOT_DIR/target/release/kora" ]]; then
    echo "🔨 Building Kora release binary..."
    cargo build --release -p kora-cli
fi

# 2. Prepare AppDir structure
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$APPDIR/usr/share/mime/packages"
mkdir -p "$APPDIR/usr/share/metainfo"

# Copy binary
cp -f "$ROOT_DIR/target/release/kora" "$APPDIR/usr/bin/kora"
chmod +x "$APPDIR/usr/bin/kora"

# Copy desktop file and icons
cp -f "$ROOT_DIR/integrations/linux/kora.desktop" "$APPDIR/kora.desktop"
cp -f "$ROOT_DIR/integrations/vscode/icon.svg" "$APPDIR/kora.svg"
cp -f "$ROOT_DIR/integrations/vscode/icon.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/kora.svg"
cp -f "$ROOT_DIR/integrations/vscode/icon.png" "$APPDIR/kora.png"
cp -f "$ROOT_DIR/integrations/linux/kora.xml" "$APPDIR/usr/share/mime/packages/kora.xml"

# 3. Create AppRun launcher (Handles CLI parameters & GUI desktop launch)
cat << 'EOF' > "$APPDIR/AppRun"
#!/bin/sh
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"

# If executed without arguments or double-clicked from GUI file manager
if [ $# -eq 0 ] && [ ! -t 0 ]; then
    # Spawn terminal window running interactive kora doctor & help
    if command -v x-terminal-emulator >/dev/null 2>&1; then
        exec x-terminal-emulator -e sh -c "${HERE}/usr/bin/kora doctor; echo ''; echo 'Press Enter to exit...'; read line"
    elif command -v gnome-terminal >/dev/null 2>&1; then
        exec gnome-terminal -- sh -c "${HERE}/usr/bin/kora doctor; echo ''; echo 'Press Enter to exit...'; read line"
    elif command -v konsole >/dev/null 2>&1; then
        exec konsole -e sh -c "${HERE}/usr/bin/kora doctor; echo ''; echo 'Press Enter to exit...'; read line"
    elif command -v xterm >/dev/null 2>&1; then
        exec xterm -e sh -c "${HERE}/usr/bin/kora doctor; echo ''; echo 'Press Enter to exit...'; read line"
    fi
fi

exec "${HERE}/usr/bin/kora" "$@"
EOF
chmod +x "$APPDIR/AppRun"

# 4. Package with appimagetool or create self-extracting portable runner
mkdir -p "$OUTPUT_DIR"
APPIMAGE_NAME="kora-x86_64.AppImage"

if command -v appimagetool &>/dev/null; then
    ARCH=x86_64 appimagetool -n "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME"
else
    # Fetch appimagetool if curl exists and running in CI
    TOOL_URL="https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
    TOOL_BIN="$BUILD_DIR/appimagetool"
    if curl -fsSL "$TOOL_URL" -o "$TOOL_BIN" 2>/dev/null; then
        chmod +x "$TOOL_BIN"
        cd "$BUILD_DIR"
        "$TOOL_BIN" --appimage-extract >/dev/null 2>&1 || true
        if [[ -f "$BUILD_DIR/squashfs-root/AppRun" ]]; then
            ARCH=x86_64 "$BUILD_DIR/squashfs-root/AppRun" -n "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME"
        else
            ARCH=x86_64 "$TOOL_BIN" -n "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME" || true
        fi
        cd "$ROOT_DIR"
    fi
fi

# Fallback: if appimagetool is unavailable in local environment, create portable tar package
if [[ ! -f "$OUTPUT_DIR/$APPIMAGE_NAME" ]]; then
    echo "ℹ️  Creating self-contained portable bundle..."
    tar -czf "$OUTPUT_DIR/kora-portable-linux-x86_64.tar.gz" -C "$BUILD_DIR" Kora.AppDir
fi

echo "✓ AppImage packaging completed in $OUTPUT_DIR"

