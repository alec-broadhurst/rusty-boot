use core::ptr::{read_volatile, write_volatile};

// === Timer2 Register Addresses ===
const TCCR2A: *mut u8 = 0xB0 as *mut u8; // Timer/Counter Control Register A
const TCCR2B: *mut u8 = 0xB1 as *mut u8; // Timer/Counter Control Register B
const TCNT2: *mut u8 = 0xB2 as *mut u8; // Timer/Counter Value Register
const OCR2A: *mut u8 = 0xB3 as *mut u8; // Output Compare Register A
const TIFR2: *mut u8 = 0x17 as *mut u8; // Timer/Counter Interrupt Flag Register

// === Bit Masks ===
const WGM21: u8 = 1 << 1; // Enable CTC mode
const CS22: u8 = 1 << 2; // Prescaler select
const OCF2A: u8 = 1 << 1; // Output Compare A Match flag

/// Initializes Timer2 in CTC mode for 1ms ticks.
///
/// - Assumes 16 MHz system clock.
/// - Uses a prescaler of 64.
/// - Sets OCR2A = 249 for 1ms period:
///
///     16_000_000 / 64 / (1kHz) = 250 cycles → OCR2A = 249
pub fn init() {
    unsafe {
        write_volatile(TCCR2A, WGM21); // set CTC mode
        write_volatile(TCCR2B, CS22); // set prescaler to 64
        write_volatile(OCR2A, 249); // set top for 1ms tick
        write_volatile(TCNT2, 0); // reset counter
    }
}

/// Busy-wait for `ms` milliseconds using Timer2.
///
/// This method performs a polling delay. It clears and waits
/// on the OCF2A flag `ms` times, each representing 1 millisecond.
pub fn wait_ms(ms: u16) {
    for _ in 0..ms {
        unsafe {
            write_volatile(TIFR2, OCF2A); // Clear match flag
            while read_volatile(TIFR2) & OCF2A == 0 {} // Busy-wait until timer reaches OCR2A
        }
    }
}
