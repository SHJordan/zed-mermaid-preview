#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

echo "Building Mermaid Preview extension for Zed..."

rustup target add wasm32-wasip2 >/dev/null 2>&1 || true

echo "Building WebAssembly extension..."
cargo build --lib --target wasm32-wasip2 --release

echo "Copying extension.wasm to root directory..."
cp target/wasm32-wasip2/release/mermaid_preview.wasm "$SCRIPT_DIR/../extension.wasm"

echo ""
echo "✅ Build complete!"
echo "📦 Binaries built:"
echo "  - target/wasm32-wasip2/release/mermaid_preview.wasm → extension.wasm"
echo ""
echo "🔧 For development:"
echo "  The extension will now dynamically install the LSP from NPM."
echo "  To use a local LSP build, set MERMAID_LSP_PATH:"
echo "  export MERMAID_LSP_PATH=\"/path/to/your/lsp/binary\""
echo ""
