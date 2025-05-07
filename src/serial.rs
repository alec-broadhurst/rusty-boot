use core::ptr::write_volatile;

// serial configuration
const FOSC: u32 = 16_000_000; // clock speed (16MHz, no prescaler)
const BAUD: u32 = 9600; // baud rate
const UBRR: u32 = FOSC / (16 * BAUD) - 1; // baud rate register value

// register addresses
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B

// bit definitions
const RXEN0: u8 = 1 << 4; // receiver Enable bit

// function to initialize the UART
pub fn init() {
    unsafe {
        // set baud rate
        write_volatile(UBRR0L, UBRR as u8); // set low byte
        write_volatile(UBRR0H, (UBRR >> 8) as u8); // set high byte

        // enable receiver
        write_volatile(UCSR0B, RXEN0);
    }
}
