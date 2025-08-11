.section .vectors, "ax", @progbits
.global vectors
.type vectors, @function

vectors:
    rjmp start             ; 0x0000 RESET
    rjmp default_handler   ; 0x0002 INT0
    rjmp default_handler   ; 0x0004 INT1
    rjmp default_handler   ; 0x0006 PCINT0
    rjmp default_handler   ; 0x0008 PCINT1
    rjmp default_handler   ; 0x000A PCINT2
    rjmp default_handler   ; 0x000C WDT
    rjmp default_handler   ; 0x000E TIMER2 COMPA
    rjmp default_handler   ; 0x0010 TIMER2 COMPB
    rjmp default_handler   ; 0x0012 TIMER2 OVF
    rjmp default_handler   ; 0x0014 TIMER1 CAPT
    rjmp default_handler   ; 0x0016 TIMER1 COMPA
    rjmp default_handler   ; 0x0018 TIMER1 COMPB
    rjmp default_handler   ; 0x001A TIMER1 OVF
    rjmp default_handler   ; 0x001C TIMER0 COMPA
    rjmp default_handler   ; 0x001E TIMER0 COMPB
    rjmp default_handler   ; 0x0020 TIMER0 OVF
    rjmp default_handler   ; 0x0022 SPI STC
    rjmp USART_RX_vect     ; 0x0024 USART RX Complete
    rjmp USART_UDRE_vect   ; 0x0026 USART Data Register Empty
    rjmp USART_TX_vect     ; 0x0028 USART TX Complete
    rjmp default_handler   ; 0x002A ADC Conversion Complete
    rjmp default_handler   ; 0x002C EEPROM Ready
    rjmp default_handler   ; 0x002E Analog Comparator
    rjmp default_handler   ; 0x0030 Two-Wire Interface (I2C)
    rjmp default_handler   ; 0x0032 Store Program Memory Ready

default_handler:
    rjmp default_handler

.size vectors, .- vectors
