.section .text
.global erase_page
.type erase_page, @function

.equ SPMCSR, 0x37

erase_page:
    movw r30, r24   ; load Z pointer with page address
    ldi r18, 3      ; 00000011
    out SPMCSR, r18 ; move 00000011 into SPMCSR
    spm             ; execute SPM instruction to erase the page
    ret
