# RASM - Rust Assembly Manager

A powerful, modern build tool for assembly language projects with parallel compilation, watch mode, and beautiful colored output.

## ✨ Features

### 🚀 Performance
- **Parallel Builds** - Compile multiple assembly files simultaneously using all CPU cores
- **Incremental Building** - Smart file tracking (coming soon)
- **Fast Glob Pattern Expansion** - Process multiple files with wildcards

### 🎨 User Experience
- **Colored Output** - Beautiful, color-coded terminal output
- **Progress Bars** - Visual feedback during compilation
- **Auto-Configuration** - Automatically detects `.rasm.toml` or `rasm.toml` files
- **Shell Completions** - Generate completions for bash, zsh, fish, PowerShell, and Elvish

### 🔧 Build Features
- **Watch Mode** - Automatically rebuild on file changes
- **Dry Run Mode** - Preview commands without executing
- **Clean Mode** - Remove generated files
- **Multiple Assemblers** - Support for `as`, `nasm`, `yasm`, and more
- **Cross-Platform** - Works on macOS, Linux, and Windows

## 📦 Installation

### From Source

```bash
git clone https://github.com/yourusername/rasm
cd rasm
make install
```

Or manually:

```bash
cargo build --release
sudo cp target/release/rasm /usr/local/bin/
```

### Shell Completions

Generate completions for your shell:

```bash
# Bash
rasm --completions bash > /etc/bash_completion.d/rasm

# Zsh
rasm --completions zsh > ~/.zsh/completions/_rasm

# Fish
rasm --completions fish > ~/.config/fish/completions/rasm.fish

# PowerShell
rasm --completions powershell > rasm.ps1
```

## 🚀 Quick Start

### Basic Usage

Compile a single assembly file:

```bash
rasm -o myapp src/main.s
```

Compile multiple files:

```bash
rasm -o myapp src/main.s src/utils.s
```

Use glob patterns:

```bash
rasm -o myapp src/*.s
```

### Watch Mode

Automatically rebuild when files change:

```bash
rasm -w -o myapp src/*.s
```

This will:
1. Build your project immediately
2. Watch for changes to `.s` and `.asm` files
3. Automatically rebuild when changes are detected
4. Show colored output with build status

### Configuration File

Create a `.rasm.toml` file in your project directory:

```toml
input_files = ["src/*.s"]
output_file = "myapp"
assembler = "as"
assembler_flags = ["-g"]
target = "arm64"
verbose = false
color = "auto"
```

Then simply run:

```bash
rasm  # Auto-detects and uses .rasm.toml
```

Supported config file names (in order of precedence):
- `.rasm.toml`
- `rasm.toml`
- `.rasm/config.toml`

## 📖 Usage Examples

### Parallel Compilation

Build multiple files in parallel (automatic):

```bash
rasm -o server src/main.s src/network.s src/utils.s src/handlers.s
```

RASM automatically uses all available CPU cores to compile files in parallel!

### Custom Assembler

Use a different assembler:

```bash
rasm --assembler nasm --assembler-flags "-f elf64" -o myapp src/*.asm
```

### Dry Run

Preview commands without executing:

```bash
rasm --dry-run -o myapp src/*.s
```

### Clean Build

Remove generated files:

```bash
rasm --clean -o myapp src/*.s
```

### Colored Output Control

```bash
# Always use colors
rasm --color always -o myapp src/*.s

# Never use colors (for CI/CD)
rasm --color never -o myapp src/*.s

# Auto-detect (default)
rasm --color auto -o myapp src/*.s
```

### Verbose Output

Enable detailed logging:

```bash
RUST_LOG=debug rasm -v -o myapp src/*.s
```

### Cross-Platform Builds

Target different architectures:

```bash
# ARM64 (Apple Silicon, newer ARM chips)
rasm --target arm64 -o myapp src/*.s

# x86_64 (Intel/AMD)
rasm --target x86_64 -o myapp src/*.s
```

## 🎯 Advanced Configuration

### Full Configuration Example

```toml
# .rasm.toml
input_files = ["src/main.s", "src/lib/*.s"]
output_file = "bin/myapp"

# Assembler settings
assembler = "as"
assembler_flags = ["-g", "--64"]
target = "x86_64"

# Linker settings
extra_flags = ["-lc", "-dynamic"]

# Build options
verbose = true
dry_run = false
clean = false
watch = false
color = "auto"
```

### Multiple Build Profiles

You can have different config files for different builds:

```bash
# Development build
rasm --config-file .rasm.dev.toml

# Production build  
rasm --config-file .rasm.prod.toml
```

## 🛠️ Command-Line Options

```
Usage: rasm [OPTIONS] -o <OUTPUT_FILE> <INPUT_FILES>... [-- <EXTRA_FLAGS>...]

Arguments:
  <INPUT_FILES>...     Input assembly file(s) (supports glob patterns)
  [EXTRA_FLAGS]...     Extra flags for the linker (passed as-is)

Options:
  -o <OUTPUT_FILE>                   Output binary file
      --assembler <ASSEMBLER>        Assembler to use [default: as]
      --assembler-flags <FLAGS>      Additional assembler flags
      --target <TARGET>              Target architecture [default: arm64]
  -v, --verbose                      Enable verbose logging
      --dry-run                      Preview commands without executing
  -c, --clean                        Clean generated files
      --config-file <FILE>           Path to configuration file (TOML)
  -w, --watch                        Watch for changes and rebuild
      --color <WHEN>                 Colored output [default: auto] [values: auto, always, never]
      --completions <SHELL>          Generate shell completions [values: bash, zsh, fish, powershell, elvish]
  -h, --help                         Print help
  -V, --version                      Print version
```

## 🎨 Output Examples

### Successful Build (with colors)

```
✓ Found config: .rasm.toml
→ Processing 4 file(s)
⠁ [########################################] 4/4 ✓ src/utils.s.o
→ Linking myapp
✓ Build complete: myapp
```

### Watch Mode

```
👁 Watching for changes...
→ Processing 2 file(s)
⠁ [########################################] 2/2 Assembly complete
→ Linking server
✓ Build complete: server

→ Rebuilding...
→ Processing 2 file(s)
⠁ [########################################] 2/2 Assembly complete
→ Linking server
✓ Build complete: server
```

### Error Output

```
✗ Assembler failed:
src/main.s:10: Error: invalid instruction
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

All features include unit tests covering:
- Configuration validation
- Path traversal protection
- Glob pattern expansion
- Dry-run mode
- Config merging


### Development Setup

```bash
git clone https://github.com/yourusername/rasm
cd rasm
cargo build
cargo test
```


## 🐛 Troubleshooting

### Build fails with "No files match pattern"

Make sure your glob patterns are quoted:
```bash
rasm -o myapp "src/*.s"  # Correct
rasm -o myapp src/*.s     # May be expanded by shell
```

### Watch mode doesn't detect changes

Ensure you're watching the correct directory and file extensions (`.s` or `.asm`).

### Parallel builds fail

Try running with `--dry-run` first to see the commands, then run them sequentially to identify the issue.

## 🗺️ Roadmap

- [ ] Incremental builds with timestamp checking
- [ ] Build cache for faster rebuilds
- [ ] Dependency tracking for `.include` files
- [ ] Pre/post build scripts
- [ ] Multiple output formats (bin, hex, elf)
- [ ] Built-in disassembler
- [ ] LSP integration for IDE support
- [ ] Package manager for assembly libraries

---

**Happy Assembling! 🚀**

