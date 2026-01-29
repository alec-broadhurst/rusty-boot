.include "regs.inc"

.section .start
.global start
.type start, @function

start:
    cli
    clr r1

    ;--------------------------------------------------------
    ; Turn off watchdog timer
    ;--------------------------------------------------------
    wdr
    in r16, MCUSR
    andi r16, (0xFF & (0 << WDRF))
    out MCUSR, r16
    lds r16, WDTCSR
    ori r16, (1<<WDCE) | (1<<WDE)
    sts WDTCSR, r16
    ldi r16, (0 << WDE)
    sts WDTCSR, r16

    ;--------------------------------------------------------
    ; Set stack pointer
    ;--------------------------------------------------------
    ldi r16, hi8(STACK_TOP)
    out SPH, r16
    ldi r16, lo8(STACK_TOP)
    out SPL, r16

    ;--------------------------------------------------------
    ; Move interrupt vectors to boot section
    ;--------------------------------------------------------
    ldi r17, (1 << IVCE)
    out MCUCR, r17
    ldi r17, (1 << IVSEL)
    out MCUCR, r17

    ;--------------------------------------------------------
    ; Jump to Rust main function
    ;--------------------------------------------------------
    rjmp main

.size start, .-start
