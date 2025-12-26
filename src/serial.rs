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

// === System Register Address ===
const SREG: *mut u8 = 0x5F as *mut u8;

// === Bit positions ===
// UCSR0A
const U2X0: u8 = 1;
const UDRE0: u8 = 5;
const RXC0: u8 = 7;

// UCSR0B
const UCSZ02: u8 = 2;
const TXEN0: u8 = 3;
const RXEN0: u8 = 4;
const UDRIE0: u8 = 5;
const RXCIE0: u8 = 7;

// UCSR0C
const UCPOL0: u8 = 0;
const UCSZ00: u8 = 1;
const UCSZ01: u8 = 2;
const USBS0: u8 = 3;
const UPM00: u8 = 4;
const UPM01: u8 = 5;
const UMSEL00: u8 = 6;
const UMSEL01: u8 = 7;

pub fn init(baud: u32) {
    unsafe {
        // Calculate UBRR for normal speed (U2X0 = 0)
        let ubrr: u16 = ((F_CPU / (16u32 * baud)) - 1) as u16;

        let ucsra = read_volatile(UCSR0A);
        write_volatile(UCSR0A, ucsra & !(1 << U2X0));

        // Set baud rate
        write_volatile(UBRR0H, (ubrr >> 8) as u8);
        write_volatile(UBRR0L, ubrr as u8);

        // Set 8N1: 8-bit data, no parity, 1 stop bit
        write_volatile(UCSR0C, (1 << 1) | (1 << 2));

        // Enable TX/RX
        write_volatile(UCSR0B, (1 << TXEN0) | (1 << RXEN0));
    }
}

pub fn read_byte() -> Option<u8> {
    unsafe {
        while (read_volatile(UCSR0A) & (1 << RXC0)) == 0 {}
        Some(read_volatile(UDR0))
    }
}

pub fn send_byte(data: u8) {
    unsafe {
        while (read_volatile(UCSR0A) & (1 << UDRE0)) == 0 {}
        write_volatile(UDR0, data);
    }
}
