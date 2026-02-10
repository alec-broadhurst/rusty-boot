use core::ptr::{read_volatile, write_volatile};

// === System Clock Frequency ===
const F_CPU: u32 = 16_000_000;

// === USART0 Register Addresses ===
const UCSR0A: *mut u8 = 0xC0 as *mut u8;
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
const UBRR0H: *mut u8 = 0xC5 as *mut u8;
const UDR0: *mut u8 = 0xC6 as *mut u8;

// === Bit positions ===
// UCSR0A
const U2X0: u8 = 1;
const UDRE0: u8 = 5;
const RXC0: u8 = 7;

// UCSR0B
const TXEN0: u8 = 3;
const RXEN0: u8 = 4;

// UCSR0C
const UCSZ00: u8 = 1;
const UCSZ01: u8 = 2;

pub struct Serial;

impl Serial {
    pub fn new(baud: u32) -> Self {
        let ubrr: u16 = ((F_CPU / (8u32 * baud)) - 1) as u16;

        unsafe {
            write_volatile(UCSR0A, 1 << U2X0);
            write_volatile(UBRR0H, (ubrr >> 8) as u8);
            write_volatile(UBRR0L, ubrr as u8);
            write_volatile(UCSR0C, (1 << UCSZ00) | (1 << UCSZ01));
            write_volatile(UCSR0B, (1 << TXEN0) | (1 << RXEN0));
        }

        Serial
    }

    #[inline(never)]
    pub fn read_byte(&mut self) -> u8 {
        unsafe {
            while (read_volatile(UCSR0A) & (1 << RXC0)) == 0 {}
            read_volatile(UDR0)
        }
    }

    #[inline(never)]
    pub fn send_byte(&mut self, data: u8) {
        unsafe {
            while (read_volatile(UCSR0A) & (1 << UDRE0)) == 0 {}
            write_volatile(UDR0, data);
        }
    }
}
