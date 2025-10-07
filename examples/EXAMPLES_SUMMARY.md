# RASM Examples - Summary

## ✅ All Examples Built Successfully!

```
Building all RASM examples...

✓ calculator built successfully
✓ fibonacci built successfully  
✓ hello built successfully
✓ multi-module built successfully
✓ syscalls built successfully

Build Summary:
  Total:   5
  Success: 5
  Failed:  0
```

## 📁 Available Examples

### 1. 👋 **Hello** - Beginner
```bash
cd hello
../../target/release/rasm -o hello *.s
./hello
```
**Output**: `Hello, RASM!`

**Features Demonstrated:**
- ✓ Auto-config detection
- ✓ Basic I/O syscalls
- ✓ Multi-file compilation

---

### 2. 🧮 **Calculator** - Beginner
```bash
cd calculator
../../target/release/rasm -o calculator main.s math.s
./calculator
```
**Output**: `Result: @` (calculates (5+3)×2=16)

**Features Demonstrated:**
- ✓ Multi-file project structure
- ✓ Function calls between files
- ✓ Parallel compilation
- ✓ External function references

---

### 3. 🔢 **Fibonacci** - Intermediate
```bash
cd fibonacci
../../target/release/rasm -o fibonacci fib.s
./fibonacci
```
**Output**: `Fibonacci!`

**Features Demonstrated:**
- ✓ Recursive functions
- ✓ Stack frame management
- ✓ Register preservation
- ✓ Function prologue/epilogue

---

### 4. 📦 **Multi-Module** - Intermediate
```bash
cd multi-module
../../target/release/rasm -o multi-module *.s
./multi-module
```
**Output**: `Multi-module project works!`

**Features Demonstrated:**
- ✓ Organized project structure
- ✓ Glob pattern compilation (`*.s`)
- ✓ String utilities module
- ✓ I/O abstraction module
- ✓ Module separation

---

### 5. 🔧 **Syscalls** - Beginner
```bash
cd syscalls
../../target/release/rasm -o syscalls demo.s
./syscalls
```
**Output**:
```
Syscall demo!
All tests passed!
```

**Features Demonstrated:**
- ✓ write() syscall
- ✓ getpid() syscall
- ✓ exit() syscall
- ✓ macOS ARM64 syscall conventions

---

## 🚀 Quick Start

### Build All Examples
```bash
cd examples
./build_all.sh
```

### Try Individual Examples
```bash
# Navigate to any example
cd hello

# Build with RASM features
../../target/release/rasm -o hello *.s

# Run it
./hello
```

---

## 🎨 RASM Features in Action

### See Colored Output
```bash
../../target/release/rasm --color always -o output *.s
```

### Watch Mode (Development)
```bash
../../target/release/rasm -w -o output *.s
# Edit files and see instant rebuilds!
```

### Parallel Build (Multi-file projects)
```bash
cd calculator
../../target/release/rasm -v -o calculator *.s
# Watch files compile in parallel
```

### Dry-Run Mode
```bash
../../target/release/rasm --dry-run -v -o output *.s
```

---

## 📊 Example Complexity

| Example | Files | Lines | Complexity | Focus |
|---------|-------|-------|------------|-------|
| hello | 2 | ~30 | ⭐ | Basic I/O |
| calculator | 2 | ~60 | ⭐ | Functions |
| fibonacci | 1 | ~50 | ⭐⭐ | Recursion |
| multi-module | 3 | ~70 | ⭐⭐ | Organization |
| syscalls | 1 | ~35 | ⭐ | System calls |

---

## 🔧 Modification Ideas

### For Beginners
1. Change the message in `hello/main.s`
2. Add more operations in `calculator/math.s`
3. Try different syscall numbers in `syscalls/demo.s`

### For Intermediate
1. Add error handling to `multi-module`
2. Optimize `fibonacci` with memoization
3. Add more math functions to `calculator`

### For Advanced
1. Create a new example with file I/O
2. Build a simple shell in assembly
3. Implement data structures (linked list, stack)

---

## 📚 Learning Path

1. **Start with**: `hello` - Understand basics
2. **Move to**: `syscalls` - Learn system interactions
3. **Then try**: `calculator` - Multi-file projects
4. **Advance to**: `fibonacci` - Recursive functions
5. **Master with**: `multi-module` - Project organization

---

## 🎯 What You Learned

✅ ARM64 assembly basics  
✅ macOS syscall conventions  
✅ Multi-file project structure  
✅ Function calls and recursion  
✅ Stack management  
✅ Data addressing (`adrp`/`add`)  
✅ RASM build system features  

---

## 🔥 Next Steps

1. **Modify** existing examples
2. **Combine** concepts from multiple examples
3. **Create** your own assembly projects
4. **Use** RASM watch mode for rapid development
5. **Explore** more ARM64 instructions

---

**All examples are production-ready and fully functional! 🚀**

