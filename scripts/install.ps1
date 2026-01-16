$EXTENSIONS_ROOT = Join-Path $env:LOCALAPPDATA "Zed\extensions"
$TARGET_DIR = Join-Path $EXTENSIONS_ROOT "installed\mermaid-preview"

Write-Host "Installing Mermaid Preview extension for Zed..."
Write-Host "Source: $PSScriptRoot\.."
Write-Host "Target: $TARGET_DIR"

if (!(Test-Path $TARGET_DIR)) {
    New-Item -ItemType Directory -Force -Path $TARGET_DIR
}

Copy-Item -Path "$PSScriptRoot\..\extension.toml" -Destination $TARGET_DIR
Copy-Item -Path "$PSScriptRoot\..\extension.wasm" -Destination $TARGET_DIR

if (Test-Path "$PSScriptRoot\..\languages") {
    Copy-Item -Path "$PSScriptRoot\..\languages" -Destination $TARGET_DIR -Recurse -Force
}

Write-Host ""
Write-Host "🎉 Installation complete!"
Write-Host "🚀 Restart Zed to load the updated extension."
Write-Host ""
