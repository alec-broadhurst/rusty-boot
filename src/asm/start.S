.section .text
.global start
.type start, @function

.equ STACK_TOP, 0x08FF

start:
    ldi r16, hi8(STACK_TOP)
    out 0x3E, r16
    ldi r16, lo8(STACK_TOP)
    out 0x3D, r16
    rjmp main
