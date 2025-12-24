.include "regs.inc"

.extern __data_start
.extern __data_end
.extern __data_load_start
.extern __bss_start
.extern __bss_end


.section .text
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
    ; Copy .data from flash to SRAM
    ;--------------------------------------------------------
    ldi r26, lo8(__data_start)
    ldi r27, hi8(__data_start)

    ldi r30, lo8(__data_load_start)
    ldi r31, hi8(__data_load_start)

    ldi r24, lo8(__data_end)
    ldi r25, hi8(__data_end)

    copy_loop:
        cp r26, r24
        cpc r27, r25
        brsh copy_loop_end
        lpm r0, Z+
        st X+, r0
        rjmp copy_loop

    copy_loop_end:

    ;--------------------------------------------------------
    ; Zero out .bss section
    ;--------------------------------------------------------
    ldi r26, lo8(__bss_start)
    ldi r27, hi8(__bss_start)
    ldi r24, lo8(__bss_end)
    ldi r25, hi8(__bss_end)
    ldi r18, 0

    zero_loop:
        cp r26, r24
        cpc r27, r25
        brsh zero_loop_end
        st X+, r18
        rjmp zero_loop

    zero_loop_end:

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
