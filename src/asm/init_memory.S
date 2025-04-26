.section .text                  ; Code section (executable code)
.global __init_memory           ; Make the function visible to the linker
.type __init_memory, @function  ; Define as a function

__init_memory:
    ; Load Z with flash address
    ldi r30, lo8(__sram_load_start)
    ldi r31, hi8(__sram_load_start)

    ; Load X with SRAM address
    ldi r26, lo8(__sram_start)
    ldi r27, hi8(__sram_start)

    ; Load end of SRAM address for comparison
    ldi r24, lo8(__sram_end)
    ldi r25, hi8(__sram_end)

    ; Copy from flash to SRAM
    copy_loop:
        lpm r0, Z+         ; Load byte from flash into r0
        st X+, r0          ; Store byte from r0 into SRAM

        cp r26, r24
        cpc r27, r25

        brne copy_loop     ; If not done, repeat

    ldi r26, lo8(__bss_start)
    ldi r27, hi8(__bss_start)
    ldi r24, lo8(__bss_end)
    ldi r25, hi8(__bss_end)

    ldi r16, 0x00

    zero_loop:
        st X+, r16          ; Store zero into SRAM

        cp r26, r24
        cpc r27, r25

        brne zero_loop      ; If not done, repeat

        ret
