.section .text
.global reenable_rww
.type reenable_rww, @function

.equ SPMCSR, 0x37
.equ SREG, 0x3F

reenable_rww:
    in r18, SREG        ; save status register
    cli                 ; disable interrupts
    ldi r16, 0x11       ; prepare to re-enable RWW section
    out SPMCSR, r16     ; clear SPMEN bit to re-enable RWW section
    spm                 ; execute SPM instruction to re-enable RWW section
    call spm_poll       ; wait until SPM is done
    out SREG, r18       ; restore status register
    ret                 ; return from function

.size reenable_rww, .-reenable_rww
