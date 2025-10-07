// Math operations for calculator
// Demonstrates function exports

.global _add
.global _multiply

.align 2

// Add two numbers
// x0: first number
// x1: second number
// Returns: x0 = x0 + x1
_add:
    add     x0, x0, x1
    ret

// Multiply two numbers
// x0: first number
// x1: second number
// Returns: x0 = x0 * x1
_multiply:
    mul     x0, x0, x1
    ret

