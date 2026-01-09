.section .vectors, "ax", @progbits
.global vectors
.type vectors, @function

vectors:
    rjmp start             ; 0x0000 RESET

.size vectors, .- vectors
