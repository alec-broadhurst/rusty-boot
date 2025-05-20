.section .text
.global write_page
.type write_page, @function

.equ SPMCSR, 0x37

write_page:
    movw r30, r24   ; load page address into Z register
    ldi r16, 5
    out SPMCSR, r16 ; set SPMEN bit to enable page write
    spm             ; execute SPM instruction to write the page
    ret
