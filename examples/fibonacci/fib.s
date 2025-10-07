// Fibonacci calculator - ARM64 macOS
// Demonstrates recursive functions

.global _start

.align 2

_start:
    // Calculate fib(10)
    mov     x0, #10
    bl      fibonacci
    
    // x0 now contains fib(10) = 55
    mov     x19, x0             // Save result
    
    // Print message
    mov     x0, #1
    adrp    x1, msg@PAGE
    add     x1, x1, msg@PAGEOFF
    mov     x2, #12
    mov     x16, #4
    svc     #0x80
    
    // Exit
    mov     x0, #0
    mov     x16, #1
    svc     #0x80

// Fibonacci recursive function
// x0: n (input)
// Returns: fib(n) in x0
fibonacci:
    // Save frame
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp
    
    // Base case: if n <= 1, return n
    cmp     x0, #1
    ble     fib_base
    
    // Save n
    stp     x19, x20, [sp, #-16]!
    mov     x19, x0
    
    // Calculate fib(n-1)
    sub     x0, x19, #1
    bl      fibonacci
    mov     x20, x0             // Save fib(n-1)
    
    // Calculate fib(n-2)
    sub     x0, x19, #2
    bl      fibonacci
    
    // Result = fib(n-1) + fib(n-2)
    add     x0, x20, x0
    
    // Restore and return
    ldp     x19, x20, [sp], #16
    ldp     x29, x30, [sp], #16
    ret

fib_base:
    ldp     x29, x30, [sp], #16
    ret

.data
msg:
    .ascii  "Fibonacci!\n"

