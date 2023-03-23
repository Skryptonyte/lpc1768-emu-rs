.section .text
.balign 4
.globl _start

    LDR r5, =src
    LDR R6, =addr
    LDR R6, [R6]
    mov r0, #0
    mov r1, #5
copyloop:
    ldr r2, [r5,r0]
    str r2, [r6, r0]

    add r0, #4

    sub r1, #1
    bne copyloop

    MOV R0, R6
    MOV R1, #0 // counter i
    
    
OUTERLOOP:
    cmp r1, #5
    bhs endouter

    mov r2, r0 // 
    mov r3, r0 // min

    mov r4, r1

    add r4, #1
    add r2, #4
INNERLOOP:
    cmp r4, #5
    bhs endinner
    ldr r5, [r3] // arr[min]
    ldr r6, [r2] // arr[j]

    cmp r6, r5
    bhi innerloop2
    MOV r3, r2
innerloop2:
    add r4, #1 // j++
    add r2, #4
    b INNERLOOP
endinner:

        ldr r5, [r3] //arr[min]
        ldr r6, [r0] //arr[i]

        str r5, [r0]
        str r6, [r3]

        add r0, #4
        add r1, #1
        b OUTERLOOP

endouter:
        nop

src:
.word 0x1023, 0x9432, 0x8103, 0x1000, 0x12
addr:
.word 0x10000000