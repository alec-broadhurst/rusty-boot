.include "regs.inc"

.section .start
.global start
.type start, @function

start:
    cli
    clr r1

    ;--------------------------------------------------------
    ; If watchdog reset, jump to app
    ;--------------------------------------------------------
    in r16, MCUSR
    in r17, MCUSR
    andi r17, 0 << WDRF
    out MCUSR, r17
    sbrc r16, WDRF
    rjmp jmp_to_app

    ;--------------------------------------------------------
    ; Set watchdog timer
    ;--------------------------------------------------------
    lds r16, WDTCSR
    ori r16, (1 << WDCE) | (1 << WDE)
    sts WDTCSR, r16
    ldi r16, (1 << WDE) | (1 << WDP2) | (1 << WDP1)
    sts WDTCSR, r16

    ;--------------------------------------------------------
    ; Set stack pointer
    ;--------------------------------------------------------
    ldi r16, hi8(STACK_TOP)
    out SPH, r16
    ldi r16, lo8(STACK_TOP)
    out SPL, r16

    ;--------------------------------------------------------
    ; Jump to Rust main function
    ;--------------------------------------------------------
    rjmp main

.size start, .-start


jmp_to_app:
    lds r16, WDTCSR
    ori r16, (1 << WDCE) | (1 << WDE)
    sts WDTCSR, r16
    ldi r16, (0 << WDE)
    sts WDTCSR, r16

    jmp 0x0000
