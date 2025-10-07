// Syscall demonstrations for macOS ARM64
// Shows various system calls

.global _start
.align 2

_start:
    // 1. Write to stdout
    mov     x0, #1
    adrp    x1, msg1@PAGE
    add     x1, x1, msg1@PAGEOFF
    mov     x2, #14
    mov     x16, #4             // write syscall
    svc     #0x80
    
    // 2. Get process ID
    mov     x16, #20            // getpid syscall
    svc     #0x80
    // x0 now contains PID (we won't use it, just demo)
    
    // 3. Write another message
    mov     x0, #1
    adrp    x1, msg2@PAGE
    add     x1, x1, msg2@PAGEOFF
    mov     x2, #18
    mov     x16, #4
    svc     #0x80
    
    // 4. Exit with status 0
    mov     x0, #0
    mov     x16, #1             // exit syscall
    svc     #0x80

.data
msg1:
    .ascii  "Syscall demo!\n"
msg2:
    .ascii  "All tests passed!\n"

