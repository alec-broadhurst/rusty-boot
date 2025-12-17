.include "regs.inc"


;--------------------------------------------------------
; Write Page
;--------------------------------------------------------
.global write_page
.type write_page, @function

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


;--------------------------------------------------------
; Erase Page
;--------------------------------------------------------
.global erase_page
.type erase_page, @function

erase_page:
    in r19, SREG    ; save status register
    cli             ; disable interrupts
    movw r30, r24   ; load Z pointer with page address
    ldi r18, 3      ; 00000011
    out SPMCSR, r18 ; move 00000011 into SPMCSR
    spm             ; execute SPM instruction to erase the page
    rcall spm_poll
    out SREG, r19   ; restore status register
    ret

.size erase_page, .-erase_page


;--------------------------------------------------------
; Fill Page Buffer
;--------------------------------------------------------
.global fill_page_buffer
.type fill_page_buffer, @function

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
    rcall spm_poll
    adiw r30, 2     ; next word address (flash)
    dec r17
    brne fill_loop  ; repeat until all words are written
    out SREG, r19   ; restore status register
    ret

.size fill_page_buffer, .-fill_page_buffer


;--------------------------------------------------------
; Reenable Read While Write
;--------------------------------------------------------
.global reenable_rww
.type reenable_rww, @function

reenable_rww:
    in r18, SREG        ; save status register
    cli                 ; disable interrupts
    ldi r16, 0x11       ; prepare to re-enable RWW section
    out SPMCSR, r16     ; clear SPMEN bit to re-enable RWW section
    spm                 ; execute SPM instruction to re-enable RWW section
    rcall spm_poll      ; wait until SPM is done
    out SREG, r18       ; restore status register
    ret                 ; return from function

.size reenable_rww, .-reenable_rww


;--------------------------------------------------------
; SPM Poll
;--------------------------------------------------------
.global spm_poll
.type spm_poll, @function

spm_poll:
    in r24, SPMCSR    ; read SPMCSR
    sbrc r24, 0       ; check if SPMEN bit is set
    rjmp spm_poll     ; if SPMEN bit is set, wait until SPM is done
    ret               ; return when SPM is complete

.size spm_poll, .-spm_poll
