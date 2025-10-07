// Multi-module project example
// Demonstrates organized project structure with RASM

.global _start
.extern _string_length
.extern _print_string

.align 2

_start:
    // Get string length
    adrp    x0, greeting@PAGE
    add     x0, x0, greeting@PAGEOFF
    bl      _string_length
    mov     x19, x0             // Save length
    
    // Print the string
    adrp    x0, greeting@PAGE
    add     x0, x0, greeting@PAGEOFF
    mov     x1, x19
    bl      _print_string
    
    // Exit
    mov     x0, #0
    mov     x16, #1
    svc     #0x80

.data
greeting:
    .ascii  "Multi-module project works!\n"

