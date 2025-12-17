;--------------------------------------------------------
; CPU Core
;--------------------------------------------------------
.equ SREG, 0x3F        ; Status Register


;--------------------------------------------------------
; Stack
;--------------------------------------------------------
.equ STACK_TOP, 0x08FF      ; Top of SRAM
.equ SPH, 0x3E              ; Stack Pointer High
.equ SPL, 0x3D              ; Stack Pointer Low


;--------------------------------------------------------
; Program Memory / SPM
;--------------------------------------------------------
.equ SPMCSR, 0x37        ; Store Program Memory Control and Status

.equ SPMEN, 0           ; Store Program Memory Enable
.equ PGERS, 1           ; Page Erase
.equ PGWRT, 2           ; Page Write
.equ RWWSRE, 4           ; Read-While-Write Section Read Enable
.equ RWWSB, 6           ; Read-While-Write Section Busy


;--------------------------------------------------------
; I/O – Port B (LED, debug, etc.)
;--------------------------------------------------------
.equ DDRB, 0x04
.equ PORTB, 0x05

.equ PB5, 5           ; Arduino LED (SCK)


;--------------------------------------------------------
; Reset / Startup
;--------------------------------------------------------
.equ MCUSR, 0x34        ; MCU Status Register
.equ WDRF, 3           ; Watchdog Reset Flag


;--------------------------------------------------------
; Interrupt Control
;--------------------------------------------------------
.equ MCUCR, 0x35        ; MCU Control Register

.equ IVCE, 0           ; Interrupt Vector Change Enable
.equ IVSEL, 1           ; Interrupt Vector Select


;--------------------------------------------------------
; Watchdog Timer
;--------------------------------------------------------
.equ WDTCSR, 0x60        ; Watchdog Timer Control Register

.equ WDE, 3           ; Watchdog Enable
.equ WDCE, 4           ; Watchdog Change Enable
