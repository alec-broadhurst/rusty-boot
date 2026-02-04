.include "regs.inc"


;--------------------------------------------------------
; Write Page
;--------------------------------------------------------
.global write_page
.type write_page, @function

write_page:
    movw r30, r24
    ldi r18, (1<<PGWRT) | (1<<SPMEN)
    out SPMCSR, r18
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
    ldi r18, (1<<PGERS) | (1<<SPMEN)
    out SPMCSR, r18
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
    ldi r18, (1 << SPMEN)
    out SPMCSR, r18
    spm
    clr r1
    ret

.size word_to_buf, .-word_to_buf


;--------------------------------------------------------
; Reenable Read While Write
;--------------------------------------------------------
.global reenable_rww
.type reenable_rww, @function

reenable_rww:
    ldi r18, (1<<RWWSRE) | (1<<SPMEN)
    out SPMCSR, r18
    spm
rww_wait:
    in r18, SPMCSR
    sbrc r18, RWWSB ; Wait for RWW section to become ready.
    rjmp rww_wait
    ret

.size reenable_rww, .-reenable_rww


;--------------------------------------------------------
; SPM Poll
;--------------------------------------------------------
.global spm_poll
.type spm_poll, @function

spm_poll:
    in r18, SPMCSR
    sbrc r18, SPMEN
    rjmp spm_poll
    clr r1
    ret

.size spm_poll, .-spm_poll
