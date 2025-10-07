// Simple calculator - ARM64 macOS
// Demonstrates multi-file project with RASM

.global _start
.extern _add
.extern _multiply

.align 2

_start:
    // Calculate: (5 + 3) * 2
    
    // First: 5 + 3
    mov     x0, #5
    mov     x1, #3
    bl      _add                // Call add function
    mov     x19, x0             // Save result (8) in x19
    
    // Then: result * 2
    mov     x0, x19
    mov     x1, #2
    bl      _multiply           // Call multiply function
    
    // Print result (should be 16)
    mov     x19, x0             // Save final result
    
    // Convert to ASCII and print
    add     x0, x19, #48        // Convert to ASCII ('0' + number)
    adrp    x1, result_msg@PAGE
    add     x1, x1, result_msg@PAGEOFF
    strb    w0, [x1, #8]        // Store digit in message
    
    // write(1, message, 10)
    mov     x0, #1
    adrp    x1, result_msg@PAGE
    add     x1, x1, result_msg@PAGEOFF
    mov     x2, #10
    mov     x16, #4
    svc     #0x80
    
    // exit(0)
    mov     x0, #0
    mov     x16, #1
    svc     #0x80

.data
result_msg:
    .ascii  "Result: \n\0"

