.include "regs.inc"


;--------------------------------------------------------
; Write Page
;--------------------------------------------------------
.global write_page
.type write_page, @function

write_page:
    movw r30, r24
    ldi r16, (1<<PGWRT) | (1<<SPMEN)
    out SPMCSR, r16
    spm
    rcall spm_poll
    ret

.size write_page, .-write_page


;--------------------------------------------------------
; Erase Page
;--------------------------------------------------------
.global erase_page
.type erase_page, @function

erase_page:
    movw r30, r24
    ldi r16, (1<<PGERS) | (1<<SPMEN)
    out SPMCSR, r16
    spm
    rcall spm_poll
    ret

.size erase_page, .-erase_page


;--------------------------------------------------------
; Write Word to Buffer
;--------------------------------------------------------
.global word_to_buf
.type word_to_buf, @function

word_to_buf:
    movw r0, r24
    movw r30, r22
    ldi r16, 0x01
    out SPMCSR, r16
    spm
    clr r1
    ret

.size word_to_buf, .-word_to_buf


;--------------------------------------------------------
; Fill Page Buffer
;--------------------------------------------------------
.global fill_page_buffer
.type fill_page_buffer, @function

fill_page_buffer:
    movw r30, r24
    movw r26, r22
    ldi r17, 64

fill_loop:
    ld r0, X+
    ld r1, X+
    ldi r16, 0x01
    out SPMCSR, r16
    spm
    adiw r30, 2
    dec r17
    brne fill_loop
    clr r1
    ret

.size fill_page_buffer, .-fill_page_buffer


;--------------------------------------------------------
; Reenable Read While Write
;--------------------------------------------------------
.global reenable_rww
.type reenable_rww, @function

reenable_rww:
    ldi r16, (1<<RWWSRE) | (1<<SPMEN)
    out SPMCSR, r16
    spm
    rcall spm_poll
    ret

.size reenable_rww, .-reenable_rww


;--------------------------------------------------------
; SPM Poll
;--------------------------------------------------------
.global spm_poll
.type spm_poll, @function

spm_poll:
    in r16, SPMCSR
    sbrc r16, SPMEN
    rjmp spm_poll
    clr r1
    ret

.size spm_poll, .-spm_poll


;--------------------------------------------------------
; EEPROM Poll
;--------------------------------------------------------
.global wait_ee
.type wait_ee, @function

wait_ee:
    sbic EECR, EEPE
    rjmp wait_ee
