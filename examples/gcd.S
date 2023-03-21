.section .text
.balign 4
.globl _start

_start:
    mov r0, #60 // num 1
    mov r1, #100 // num 2
    mov r2, #1
    mov r3, #0 // Designated zero register
    ldr r2, =dest
    ldr r2, [r2,r3]
_loop:
    cmp r0, r1
    beq _end
    bhi hi
    blo lo
hi:
    sub r0, r0, r1
    b _loop
lo:
    sub r1, r1, r0
    b _loop 
_end:
    str r0, [r2,r3]
dest:
    .word 0x10000000

.section .rodata
.balign 4
