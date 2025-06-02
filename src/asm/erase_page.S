.section .text
.global erase_page
.type erase_page, @function

.equ SPMCSR, 0x37
.equ SREG, 0x3F

erase_page:
    in r19, SREG    ; save status register
    cli             ; disable interrupts
    movw r30, r24   ; load Z pointer with page address
    ldi r18, 3      ; 00000011
    out SPMCSR, r18 ; move 00000011 into SPMCSR
    spm             ; execute SPM instruction to erase the page
    out SREG, r19   ; restore status register
    ret

.size erase_page, .-erase_page
