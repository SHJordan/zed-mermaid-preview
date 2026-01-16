#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXTENSIONS_ROOT="${ZED_EXTENSIONS_DIR:-$HOME/.local/share/zed/extensions}"
TARGET_DIR="${EXTENSIONS_ROOT}/installed/mermaid-preview"

echo "Installing Mermaid Preview extension for Zed..."
echo "Source: $SCRIPT_DIR/.."
echo "Target: $TARGET_DIR"

mkdir -p "$TARGET_DIR"

cp "$SCRIPT_DIR/../extension.toml" "$TARGET_DIR/"
cp "$SCRIPT_DIR/../extension.wasm" "$TARGET_DIR/"

if [ -d "$SCRIPT_DIR/../languages" ]; then
    cp -r "$SCRIPT_DIR/../languages" "$TARGET_DIR/"
fi

echo ""
echo "🎉 Installation complete!"
echo "🚀 Restart Zed to load the updated extension."
echo ""
