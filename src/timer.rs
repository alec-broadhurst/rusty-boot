use core::ptr::{read_volatile, write_volatile};

// register addresses
const TCCR2A: *mut u8 = 0xB0 as *mut u8; // Timer/Counter Control Register A
const TCCR2B: *mut u8 = 0xB1 as *mut u8; // Timer/Counter Control Register B
const TCNT2: *mut u8 = 0xB2 as *mut u8; // Timer/Counter Value Register
const OCR2A: *mut u8 = 0xB3 as *mut u8; // Output Compare Register A
const TIFR2: *mut u8 = 0x37 as *mut u8; // Timer/Counter Interrupt Flag Register

// bit definitions
const WGM21: u8 = 1 << 1; // CTC mode bit in TCCR2A
const CS22: u8 = 1 << 2; // Clock Select bits in TCCR2B
const OCF2A: u8 = 1 << 1; // Output Compare A Match flag

pub fn init() {
    unsafe {
        write_volatile(TCCR2A, WGM21); // set CTC mode
        write_volatile(TCCR2B, CS22); // set prescaler to 64
        write_volatile(OCR2A, 249); // set top for 1ms tick
        write_volatile(TCNT2, 0); // reset counter
    }
}

pub fn wait_ms(ms: u16) {
    for _ in 0..ms {
        unsafe {
            write_volatile(TIFR2, OCF2A); // Clear match flag
            while read_volatile(TIFR2) & OCF2A == 0 {} // Busy-wait until timer reaches OCR2A
        }
    }
}
