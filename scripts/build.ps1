Write-Host "Building Mermaid Preview extension for Zed..."

rustup target add wasm32-wasip2

Write-Host "Building WebAssembly extension..."
cargo build --lib --target wasm32-wasip2 --release

Write-Host "Copying extension.wasm to root directory..."
Copy-Item -Path "target\wasm32-wasip2\release\mermaid_preview.wasm" -Destination ".\extension.wasm"

Write-Host ""
Write-Host "✅ Build complete!"
Write-Host "📦 Binaries built:"
Write-Host "  - target\wasm32-wasip2\release\mermaid_preview.wasm → extension.wasm"
Write-Host ""
Write-Host "🔧 For development:"
Write-Host "  The extension will now dynamically install the LSP from NPM."
Write-Host "  To use a local LSP build, set MERMAID_LSP_PATH:"
Write-Host "  `$env:MERMAID_LSP_PATH = 'C:\path\to\your\lsp\binary'"
Write-Host ""
