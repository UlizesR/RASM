// I/O functions

.global _print_string
.align 2

// Print a string to stdout
// x0: pointer to string
// x1: length
_print_string:
    // Save parameters
    mov     x2, x1              // length
    mov     x1, x0              // string pointer
    
    // write(1, string, length)
    mov     x0, #1              // stdout
    mov     x16, #4             // syscall write
    svc     #0x80
    
    ret

