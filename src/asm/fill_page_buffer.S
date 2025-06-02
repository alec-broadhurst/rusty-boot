.section .text
.global fill_page_buffer
.type fill_page_buffer, @function

.equ SPMCSR, 0x37
.equ SREG, 0x3F

fill_page_buffer:
    in r19, SREG    ; save status register
    cli             ; disable interrupts
    movw r30, r24   ; load page address into Z register
    movw r26, r22   ; load page buffer address into X register
    ldi r17, 64

fill_loop:
    ld r0, X+
    ld r1, X+
    ldi r16, 1
    out SPMCSR, r16 ; set SPMEN bit to enable page write
    spm
    adiw r30, 2     ; next word address (flash)
    dec r17
    brne fill_loop  ; repeat until all words are written
    out SREG, r19   ; restore status register
    ret

.size fill_page_buffer, .-fill_page_buffer
