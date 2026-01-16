#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXTENSIONS_ROOT="${ZED_EXTENSIONS_DIR:-$HOME/.local/share/zed/extensions}"
TARGET_DIR="${EXTENSIONS_ROOT}/installed/mermaid-preview"
SOURCE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Installing Mermaid Preview extension for Zed..."
echo "Source: $SOURCE_DIR"
echo "Target: $TARGET_DIR"

if [ -d "$TARGET_DIR" ]; then
    RESOLVED_TARGET=$(cd "$TARGET_DIR" && pwd)
    RESOLVED_SOURCE=$(cd "$SOURCE_DIR" && pwd)
    
    if [ "$RESOLVED_TARGET" == "$RESOLVED_SOURCE" ]; then
        echo ""
        echo "ℹ️  Source and Target are the same location. No files to copy."
        echo "🎉 Installation complete (development mode)!"
        echo "🚀 Restart Zed to load the updated extension."
        exit 0
    fi
fi

mkdir -p "$TARGET_DIR"

cp "$SOURCE_DIR/extension.toml" "$TARGET_DIR/"
cp "$SOURCE_DIR/extension.wasm" "$TARGET_DIR/"

if [ -d "$SOURCE_DIR/languages" ]; then
    cp -r "$SOURCE_DIR/languages" "$TARGET_DIR/"
fi

echo ""
echo "🎉 Installation complete!"
echo "🚀 Restart Zed to load the updated extension."
echo ""
