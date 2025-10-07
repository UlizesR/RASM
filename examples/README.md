# RASM Examples

A collection of examples demonstrating various features of RASM and ARM64 assembly programming.

## 📚 Examples Overview

### 1. 👋 Hello World (`hello/`)
**Difficulty**: Beginner  
**Features**: Basic I/O, auto-config detection

A simple "Hello, RASM!" program that demonstrates:
- Basic ARM64 assembly structure
- System calls (write, exit)
- Auto-detected configuration with `.rasm.toml`
- Multiple file compilation

**Build & Run:**
```bash
cd hello
../../target/release/rasm -o hello *.s
./hello
```

**Expected Output:**
```
Hello, RASM!
```

---

### 2. 🧮 Calculator (`calculator/`)
**Difficulty**: Beginner  
**Features**: Multi-file project, function calls, parallel builds

A simple calculator demonstrating:
- Multi-file projects with shared functions
- Function definitions and external references
- Parallel compilation of multiple files
- Basic arithmetic operations

**Files:**
- `main.s` - Main program logic
- `math.s` - Math functions (add, multiply)

**Build & Run:**
```bash
cd calculator
../../target/release/rasm -o calculator main.s math.s
./calculator
```

**What it does:**
- Calculates (5 + 3) × 2 = 16
- Prints the result

---

### 3. 🔢 Fibonacci (`fibonacci/`)
**Difficulty**: Intermediate  
**Features**: Recursive functions, stack management

Fibonacci calculator demonstrating:
- Recursive function calls
- Stack frame management
- Function prologue/epilogue
- Register preservation

**Build & Run:**
```bash
cd fibonacci
../../target/release/rasm -o fibonacci fib.s
./fibonacci
```

**What it does:**
- Calculates fib(10) recursively
- Demonstrates proper stack usage

---

### 4. 📦 Multi-Module (`multi-module/`)
**Difficulty**: Intermediate  
**Features**: Project organization, glob patterns

Well-organized project demonstrating:
- Modular code structure
- String manipulation functions
- I/O abstractions
- Glob pattern compilation (`*.s`)

**Files:**
- `main.s` - Entry point
- `string_utils.s` - String operations
- `io.s` - I/O functions

**Build & Run:**
```bash
cd multi-module
../../target/release/rasm -o multi-module *.s
./multi-module
```

**Expected Output:**
```
Multi-module project works!
```

---

### 5. 🔧 Syscalls (`syscalls/`)
**Difficulty**: Beginner  
**Features**: System call demonstrations

System call examples showing:
- Write syscall
- Getpid syscall
- Exit syscall
- macOS ARM64 syscall conventions

**Build & Run:**
```bash
cd syscalls
../../target/release/rasm -o syscalls demo.s
./syscalls
```

**Expected Output:**
```
Syscall demo!
All tests passed!
```

---

## 🚀 Building All Examples

From the `examples/` directory:

```bash
# Build all examples
for dir in */; do
    echo "Building $dir..."
    cd "$dir"
    ../../target/release/rasm
    cd ..
done
```

Or use the provided script:
```bash
./build_all.sh
```

---

## 🎨 RASM Features Demonstrated

### ✨ Colored Output
Most examples use colored output by default. Try:
```bash
../../target/release/rasm --color always -o output *.s
```

### 📊 Progress Bars
Visible when building projects with multiple files:
```bash
cd calculator
../../target/release/rasm -v -o calculator *.s
```

### 🚀 Parallel Builds
Automatically enabled for multi-file projects:
```bash
cd multi-module
../../target/release/rasm -o multi-module *.s
# Files are assembled in parallel!
```

### 👁️ Watch Mode
Auto-rebuild on file changes (great for development):
```bash
cd hello
../../target/release/rasm -w -o hello *.s
# Edit any .s file and save - automatic rebuild!
```

### 🔍 Auto-Config Detection
All examples include `.rasm.toml` files that are auto-detected:
```bash
cd fibonacci
../../target/release/rasm
# No arguments needed - uses .rasm.toml automatically!
```

### 🧹 Clean Mode
Remove all generated files:
```bash
../../target/release/rasm -c -o output *.s
```

### 🔎 Dry-Run Mode
Preview commands without executing:
```bash
../../target/release/rasm --dry-run -v -o output *.s
```

---

## 📝 Example Templates

### Basic Template
```asm
.global _start
.align 2

_start:
    // Your code here
    
    // Exit
    mov     x0, #0
    mov     x16, #1
    svc     #0x80
```

### With Data Section
```asm
.global _start
.align 2

_start:
    // Use data
    adr     x1, message
    // ... your code ...
    
    mov     x0, #0
    mov     x16, #1
    svc     #0x80

.data
message:
    .ascii  "Hello!\n"
```

### Multi-File Function
```asm
// In utils.s
.global _my_function
.align 2

_my_function:
    // x0 = input parameter
    // x0 = return value
    add     x0, x0, #1
    ret

// In main.s
.global _start
.extern _my_function
.align 2

_start:
    mov     x0, #5
    bl      _my_function
    // x0 now contains 6
```

---

## 🛠️ Configuration Examples

### Development Config (`.rasm.toml`)
```toml
input_files = ["*.s"]
output_file = "myapp"
target = "arm64"
watch = true          # Auto-rebuild
verbose = true        # See details
color = "always"      # Colored output
```

### Production Config
```toml
input_files = ["src/*.s"]
output_file = "release/myapp"
target = "arm64"
color = "never"       # Clean logs for CI
```

---

## 🐛 Troubleshooting

### Build Fails
```bash
# Use dry-run to see commands
../../target/release/rasm --dry-run -v -o output *.s

# Check verbose output
RUST_LOG=debug ../../target/release/rasm -v -o output *.s
```

### Wrong Binary Path
Make sure to use the newly built binary:
```bash
# From examples directory
../../target/release/rasm --help

# Should show --color, --watch, etc.
```

### Clean and Rebuild
```bash
../../target/release/rasm -c -o myapp *.s
../../target/release/rasm -o myapp *.s
```

---

## 📚 Learning Resources

### ARM64 Assembly
- [ARM Developer Documentation](https://developer.arm.com/documentation/)
- [macOS System Calls](https://opensource.apple.com/source/xnu/)

### RASM Documentation
- [Main README](../README.md)
- [Features Guide](../FEATURES.md)

---

## 🤝 Contributing Examples

Want to add an example? Follow this structure:

```
examples/your-example/
├── .rasm.toml          # Configuration
├── main.s              # Main source file
├── other.s             # Additional files (optional)
└── README.md           # Example-specific docs (optional)
```

Then update this README with your example!

---

## 🎯 Next Steps

1. **Try all examples** to understand different features
2. **Modify them** to experiment with assembly
3. **Create your own** using the templates above
4. **Use watch mode** (`-w`) for rapid development

**Happy Assembling! 🚀**

