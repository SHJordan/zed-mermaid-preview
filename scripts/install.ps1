$EXTENSIONS_ROOT = Join-Path $env:LOCALAPPDATA "Zed\extensions"
$TARGET_DIR = Join-Path $EXTENSIONS_ROOT "installed\mermaid-preview"
$SOURCE_DIR = Resolve-Path "$PSScriptRoot\.."

Write-Host "Installing Mermaid Preview extension for Zed..."
Write-Host "Source: $SOURCE_DIR"
Write-Host "Target: $TARGET_DIR"

if (Test-Path $TARGET_DIR) {
    $ResolvedTarget = (Get-Item $TARGET_DIR).FullName
    $ResolvedSource = (Get-Item $SOURCE_DIR).FullName
    
    if ($ResolvedTarget -eq $ResolvedSource) {
        Write-Host ""
        Write-Host "ℹ️  Source and Target are the same location. No files to copy."
        Write-Host "🎉 Installation complete (development mode)!"
        Write-Host "🚀 Restart Zed to load the updated extension."
        exit 0
    }
}

if (!(Test-Path $TARGET_DIR)) {
    New-Item -ItemType Directory -Force -Path $TARGET_DIR
}

Copy-Item -Path "$SOURCE_DIR\extension.toml" -Destination $TARGET_DIR -Force
Copy-Item -Path "$SOURCE_DIR\extension.wasm" -Destination $TARGET_DIR -Force

if (Test-Path "$SOURCE_DIR\languages") {
    $TARGET_LANG = Join-Path $TARGET_DIR "languages"
    if (!(Test-Path $TARGET_LANG)) {
        New-Item -ItemType Directory -Force -Path $TARGET_LANG
    }
    Copy-Item -Path "$SOURCE_DIR\languages\*" -Destination $TARGET_LANG -Recurse -Force
}

Write-Host ""
Write-Host "🎉 Installation complete!"
Write-Host "🚀 Restart Zed to load the updated extension."
Write-Host ""
