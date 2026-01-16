# Mermaid Preview for Zed
<img width="1920" height="1080" alt="Screenshot 2025-10-31 at 12 58 05 PM" src="https://github.com/user-attachments/assets/af814afc-6f0f-44a2-8dd4-30518f103fc5" />

Render Mermaid diagrams as SVG images directly in your Markdown files.

## Features

- ⚡ **Zero-config install** - Auto-downloads LSP on first use
- 🎨 **Clean preview** - Shows only rendered diagrams, no source code in preview
- 📝 **Editable source** - Source saved to separate `.mmd` files for easy editing
- 🔒 **Secure** - SVG output sanitized, files written only to project directory
- 🎯 **Proper text rendering** - Native SVG text with correct positioning for all diagram types
- 🚀 **Fast** - Works with any Mermaid diagram type
- ⚡ **Bulk rendering** - Render all diagrams at once with "Render All X Diagrams"

## Requirements

- [Mermaid CLI](https://github.com/mermaid-js/mermaid-cli) (`mmdc`)
- Install with: `npm install -g @mermaid-js/mermaid-cli`

## Installation

### Option 1: Install from Zed Extensions
1. Open Zed
2. Press `Cmd+Shift+P` (or `Ctrl+Shift+P`)
3. Type "Extensions: Install Development Extension"
4. Navigate to the cloned repository directory
5. Select it

### Option 2: Manual Installation
#### macOS / Linux
```bash
git clone https://github.com/dawsh2/zed-mermaid-preview.git
cd zed-mermaid-preview
./scripts/build.sh && ./scripts/install.sh
```

#### Windows (PowerShell)
```powershell
git clone https://github.com/dawsh2/zed-mermaid-preview.git
cd zed-mermaid-preview
.\scripts\build.ps1
.\scripts\install.ps1
```

Restart Zed to load the extension.

## Usage

### Single Diagram
```markdown
```mermaid
flowchart TD
    A[Start] --> B[Process]
    B --> C[End]
```
Place cursor in block → Right-click → **"Render Mermaid Diagram"**

### Multiple Diagrams
When you have multiple mermaid blocks, use:
- **"Render All X Mermaid Diagrams"** - Renders all at once
- **"Render Mermaid Diagram"** - Renders only the current block

### Editing Rendered Diagrams
1. Place cursor on the HTML comment line (e.g., `<!-- mermaid-source-file:...-->`)
2. Right-click → **"Edit Mermaid Source"**
3. The original code block is restored for editing

**Note:** The "Edit Mermaid Source" action only appears when your cursor is on the HTML comment line, not on the image itself.

## File Structure

After rendering:
```
document.md              # Main markdown with images
document_diagram_0.svg   # Rendered diagram
document_diagram_0.mmd   # Source code (editable)
```

## Example

See [`example.md`](example.md) for various diagram types and complexity levels.

## How It Works

**Production (End Users):**
- Extension auto-downloads the LSP from NPM on first use
- No manual setup required!

**Development (Contributors):**
- LSP binary NOT in git (excluded via `.gitignore` to keep repo clean)
- You build locally and point extension to your build using `MERMAID_LSP_PATH`

## Development

### First-Time Setup
```bash
# Clone and setup development environment
git clone https://github.com/dawsh2/zed-mermaid-preview.git
cd zed-mermaid-preview

# One-time: Configure to use local builds
./scripts/dev-setup.sh

# Build everything
./scripts/build.sh

# Restart your terminal to pick up env var
# Then restart Zed
```

### Workflow
#### macOS / Linux
```bash
# Make changes to LSP code
vim lsp/src/render.rs

# Rebuild (MERMAID_LSP_PATH ensures Zed uses this)
cd lsp && cargo build --release

# Restart Zed to load changes
```

#### Windows (PowerShell)
```powershell
# Rebuild
cd lsp
cargo build --release

# Set environment variable for Zed (or in your profile)
$env:MERMAID_LSP_PATH = "$(Get-Location)\..\target\release\mermaid-lsp.exe"

# Restart Zed
```

### Release Process
```bash
# Package binaries for all platforms
./scripts/package-mermaid-lsp.sh <target>

# Bump version in extension.toml
# Create GitHub release with binaries
# Users auto-update!
```

## Security

This extension executes the `mmdc` command-line tool to render diagrams. Security considerations:

- ✅ **Path Traversal Protection**: Validates all file paths stay within project boundaries
- ✅ **No Command Injection**: Uses safe command execution (no shell interpolation)
- ✅ **Script Tag Removal**: Rejects SVGs containing `<script>` tags
- ✅ **Regex DoS Protection**: Uses efficient patterns to prevent catastrophic backtracking
- ✅ **Automatic Cleanup**: Removes orphaned files to prevent disk space accumulation

**Important**: This extension assumes diagram code comes from trusted sources (your own files). If processing untrusted Mermaid diagrams, be aware that malicious syntax could potentially exploit vulnerabilities in the `mmdc` tool itself.

For complete security documentation, see [SECURITY.md](SECURITY.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

MIT
