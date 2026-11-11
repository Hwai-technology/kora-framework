#!/usr/bin/env bash
# ==============================================================================
# Kora Programming Language & Framework — Quick Update & Refresh Tool
# Rebuilds the latest release binary, updates all shell definitions, completions,
# and syncs IDE extensions.
# ==============================================================================

set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "🔄 Updating Kora Language & Toolchain to the latest build..."
"$ROOT_DIR/scripts/setup.sh" --yes

echo "✓ Update complete. Installed version:"
kora version

