.section .text
.global spm_poll
.type spm_poll, @function

.equ SPMCSR, 0x37

spm_poll:
    in r24, SPMCSR    ; read SPMCSR
    sbrc r24, 0       ; check if SPMEN bit is set
    rjmp spm_poll     ; if SPMEN bit is set, wait until SPM is done
    ret               ; return when SPM is complete

.size spm_poll, .-spm_poll
