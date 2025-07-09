.section .text
.global jmp_to_app
.type jmp_to_app, @function

jmp_to_app:
    cli
    ldi r30, 0x00
    ldi r31, 0x00
    ijmp
