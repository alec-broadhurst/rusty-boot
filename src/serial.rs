use core::ptr::{read_volatile, write_volatile};

/// System clock frequency in MHz
const FOSC: u32 = 16_000_000;

/// Baud rate
const BAUD: u32 = 9600;

/// Calculated UBRR value
const UBRR: u32 = FOSC / (16 * BAUD) - 1;

/// === USART0 Register Addresses ===
const UDR0: *mut u8 = 0xC6 as *mut u8; // USART Data Register
const UCSR0A: *mut u8 = 0xC0 as *mut u8; // USART Control and Status Register A
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High

// === Bit Masks ===
const RXEN0: u8 = 1 << 4; // receiver Enable bit
const RXC0: u8 = 1 << 7; // receiver Complete flag

/// Initializes USART0 for receiving at 9600 baud.
///
/// Only the receiver is enabled. This function must be called before
/// using [`read_byte`] or [`data_available`].
pub fn init() {
    unsafe {
        // set baud rate
        write_volatile(UBRR0L, UBRR as u8); // set low byte
        write_volatile(UBRR0H, (UBRR >> 8) as u8); // set high byte

        // enable receiver
        write_volatile(UCSR0B, RXEN0);
    }
}

/// Reads a byte from the serial interface (blocking).
///
/// This function blocks until a byte has been received.
pub fn read_byte() -> u8 {
    unsafe {
        // wait for data to be received
        while read_volatile(UCSR0A) & RXC0 == 0 {}

        // return received byte
        read_volatile(UDR0)
    }
}

/// Checks whether data is available to read from USART0.
///
/// Returns `true` if a byte has been received.
pub fn data_available() -> bool {
    unsafe { read_volatile(UCSR0A) & RXC0 != 0 }
}
