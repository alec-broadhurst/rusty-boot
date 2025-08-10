use crate::timer;
use core::ptr::{read_volatile, write_volatile};

/// System clock frequency in MHz
const FOSC: u32 = 16_000_000;

/// Baud rate
const BAUD: u32 = 9600;

/// Calculated UBRR value
const UBRR: u32 = FOSC / (16 * BAUD) - 1;

// === USART0 Register Addresses ===
const UDR0: *mut u8 = 0xC6 as *mut u8; // USART Data Register
const UCSR0A: *mut u8 = 0xC0 as *mut u8; // USART Control and Status Register A
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // USART Control and Status Register C
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High

// === Bit Masks ===
const TXEN0: u8 = 1 << 3; // transmitter Enable bit
const RXEN0: u8 = 1 << 4; // receiver Enable bit
const RXC0: u8 = 1 << 7; // receiver Complete flag
const TXC0: u8 = 1 << 6; // transmit Complete flag
const UDRE0: u8 = 1 << 5; // USART Data Register Empty flag

/// Initializes USART0 for receiving at 9600 baud.
///
/// Only the receiver is enabled. This function must be called before
/// using [`read_byte`] or [`data_available`].
pub fn init() {
    unsafe {
        // set baud rate
        write_volatile(UBRR0L, UBRR as u8); // set low byte
        write_volatile(UBRR0H, (UBRR >> 8) as u8); // set high byte
        write_volatile(UCSR0C, 0x06); // set 8 data bits, no parity, 1 stop bit

        // enable receiver and transmitter
        write_volatile(UCSR0B, RXEN0 | TXEN0);
    }
}

/// Reads a byte from the serial interface (blocking).
///
/// This function blocks until a byte has been received or the timeout is reached.
pub fn read_byte(timeout_ms: u16) -> Option<u8> {
    for _ in 0..timeout_ms {
        if data_available() {
            return Some(unsafe { read_volatile(UDR0) });
        }
        timer::wait_ms(1);
    }
    None
}

/// Sends a byte over the serial interface (blocking).
///
/// This function blocks until the byte has been sent and the data register is empty.
pub fn send_byte(data: u8) {
    unsafe {
        // Clear TXC0 by writing 1 before sending data
        write_volatile(UCSR0A, TXC0);

        // Wait for empty transmit buffer
        while read_volatile(UCSR0A) & UDRE0 == 0 {}

        // Load data
        write_volatile(UDR0, data);

        // Wait for transmission complete
        while read_volatile(UCSR0A) & TXC0 == 0 {}
    }
}

/// Checks whether data is available to read from USART0.
///
/// Returns `true` if a byte has been received.
pub fn data_available() -> bool {
    unsafe { read_volatile(UCSR0A) & RXC0 != 0 }
}
