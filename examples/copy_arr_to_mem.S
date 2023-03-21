.section .text
.balign 4
.globl _start

_start:
    mov r2, #5
    mov r3, #0
    mov r4, #1
    mov r7, #0
    ldr r0, =src
    ldr r1, =dst

    ldr r0,[r0,r7]
_loop:
    ldrb r6, [r1, r3]
    strb r6, [r0, r3]

    add r3, r3,r4
    sub r2, r2,r4
    bne _loop

.align 4

src:
    .word 0x10000000
dst:
    .byte 0x1, 0x2, 0x3, 0x4, 0x5
.section .rodata
.balign 4
