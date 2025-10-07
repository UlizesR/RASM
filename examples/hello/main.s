# Hello World in ARM64 Assembly for macOS
.global _start
.align 2

_start:
    // write(1, message, 13)
    mov     x0, #1              // stdout
    adr     x1, msg             // message address
    mov     x2, #13             // message length
    mov     x16, #4             // syscall write
    svc     #0x80

    // exit(0)
    mov     x0, #0              // exit code
    mov     x16, #1             // syscall exit
    svc     #0x80

msg:
    .ascii  "Hello, RASM!\n"

