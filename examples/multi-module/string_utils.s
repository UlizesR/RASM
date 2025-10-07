// String utility functions

.global _string_length
.align 2

// Calculate string length
// x0: pointer to string
// Returns: length in x0
_string_length:
    mov     x1, #0              // counter
    
strlen_loop:
    ldrb    w2, [x0, x1]
    cbz     w2, strlen_done     // if byte is 0, we're done
    add     x1, x1, #1
    b       strlen_loop
    
strlen_done:
    mov     x0, x1              // return length
    ret

