use core::ptr::{read_volatile, write_volatile};

// === System Clock Frequency ===
const FOSC: u32 = 16_000_000;

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

// UCSR0B
const TXEN0: u8 = 3;
const RXEN0: u8 = 4;
const UDRIE0: u8 = 5;
const RXCIE0: u8 = 7;

// UCSR0C
const UCSZ00: u8 = 1;
const UCSZ01: u8 = 2;

static mut RX_BUFFER: RingBuffer = RingBuffer::new();
static mut TX_BUFFER: RingBuffer = RingBuffer::new();

extern "C" {
    fn cli();
    fn restore_sreg();
}

pub fn init(baud: u32) {
    let (ubbr_val, use_double_speed) = match (FOSC / (8 * baud) - 1) as u16 {
        val if val > 0x0FFF => ((FOSC / (16 * baud) - 1) as u16, false),
        val => (val, true),
    };

    unsafe {
        write_volatile(UCSR0A, if use_double_speed { 1 << U2X0 } else { 0 });

        write_volatile(UBRR0H, (ubbr_val >> 8) as u8);
        write_volatile(UBRR0L, ubbr_val as u8);

        write_volatile(UCSR0C, 1 << UCSZ00 | 1 << UCSZ01);

        write_volatile(UCSR0B, 1 << TXEN0 | 1 << RXEN0 | 1 << RXCIE0);

        let _ = read_volatile(UDR0);
    }
}

pub fn read_byte() -> Option<u8> {
    critical_section(|| unsafe { RX_BUFFER.pop() })
}

pub fn send_byte(data: u8) {
    critical_section(|| unsafe {
        if TX_BUFFER.head == TX_BUFFER.tail && (read_volatile(UCSR0A) & (1 << UDRE0)) != 0 {
            write_volatile(UDR0, data);
        } else {
            TX_BUFFER.push(data);
            let ucsrb = read_volatile(UCSR0B);
            write_volatile(UCSR0B, ucsrb | 1 << UDRIE0);
        }
    })
}

fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    unsafe {
        cli();
        let result = f();
        restore_sreg();
        result
    }
}

#[no_mangle]
pub extern "C" fn USART_RX_vect() {
    unsafe {
        let byte = read_volatile(UDR0);
        RX_BUFFER.push(byte);
    }
}

#[no_mangle]
pub extern "C" fn USART_UDRE_vect() {
    unsafe {
        if let Some(byte) = TX_BUFFER.pop() {
            write_volatile(UDR0, byte);
        } else {
            let ucsrb = read_volatile(UCSR0B);
            write_volatile(UCSR0B, ucsrb & !(1 << UDRIE0));
        }
    }
}

pub struct RingBuffer {
    buffer: [u8; 64],
    head: u8,
    tail: u8,
}

impl RingBuffer {
    pub const fn new() -> Self {
        RingBuffer {
            buffer: [0; 64],
            head: 0,
            tail: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, byte: u8) {
        let next_head = self.head.wrapping_add(1) & 63;
        if next_head != self.tail {
            self.buffer[self.head as usize] = byte;
            self.head = next_head;
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<u8> {
        if self.head == self.tail {
            None
        } else {
            let byte = self.buffer[self.tail as usize];
            self.tail = self.tail.wrapping_add(1) & 63;
            Some(byte)
        }
    }
}
