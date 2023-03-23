.section .text
.balign 4
.globl _start

_start:
    mov r0, #0
    mov r1, #255
    ldr r2, =dest
    ldr r2, [r2]
_loop:
    add r0, r0, r1
    sub r1, #1
    bne _loop

    str r0, [r2]
dest:
    .word 0x10000000
.section .rodata
.balign 4
