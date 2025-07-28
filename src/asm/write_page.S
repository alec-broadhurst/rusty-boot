.section .text
.global write_page
.type write_page, @function

.equ SPMCSR, 0x37
.equ SREG, 0x3F

write_page:
    in r19, SREG    ; save status register
    cli             ; disable interrupts
    movw r30, r24   ; load page address into Z register
    ldi r16, 5
    out SPMCSR, r16 ; set SPMEN bit to enable page write
    spm             ; execute SPM instruction to write the page
    rcall spm_poll
    out SREG, r19   ; restore status register
    ret

.size write_page, .-write_page
