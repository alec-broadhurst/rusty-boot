.include "regs.inc"


;--------------------------------------------------------
; Write Page
;--------------------------------------------------------
.global write_page
.type write_page, @function

write_page:
    in r19, SREG
    cli
    movw r30, r24
    ldi r16, 0x05
    out SPMCSR, r16
    spm
    rcall spm_poll
    out SREG, r19
    ret

.size write_page, .-write_page


;--------------------------------------------------------
; Erase Page
;--------------------------------------------------------
.global erase_page
.type erase_page, @function

erase_page:
    in r19, SREG
    cli
    movw r30, r24
    ldi r18, 0x03
    out SPMCSR, r18
    spm
    rcall spm_poll
    out SREG, r19
    ret

.size erase_page, .-erase_page


;--------------------------------------------------------
; Fill Page Buffer
;--------------------------------------------------------
.global fill_page_buffer
.type fill_page_buffer, @function

fill_page_buffer:
    in r19, SREG
    cli
    movw r30, r24
    movw r26, r22
    ldi r17, 64

fill_loop:
    ld r0, X+
    ld r1, X+
    ldi r16, 0x01
    out SPMCSR, r16
    spm
    rcall spm_poll
    adiw r30, 2
    dec r17
    brne fill_loop
    out SREG, r19
    ret

.size fill_page_buffer, .-fill_page_buffer


;--------------------------------------------------------
; Reenable Read While Write
;--------------------------------------------------------
.global reenable_rww
.type reenable_rww, @function

reenable_rww:
    in r18, SREG
    cli
    ldi r16, 0x11
    out SPMCSR, r16
    spm
    rcall spm_poll
    out SREG, r18
    ret

.size reenable_rww, .-reenable_rww


;--------------------------------------------------------
; SPM Poll
;--------------------------------------------------------
.global spm_poll
.type spm_poll, @function

spm_poll:
    in r24, SPMCSR
    sbrc r24, 0
    rjmp spm_poll
    ret

.size spm_poll, .-spm_poll
