#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::arch::asm;
use core::panic::PanicInfo;

mod flash;
mod serial;
mod timer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

// === STK500v1 command and response values ===
const RESP_STK_OK: u8 = 0x10;
const RESP_STK_INSYNC: u8 = 0x14;
const EOP: u8 = 0x20;
const CMD_GET_SYNC: u8 = 0x30;
const CMD_GET_PARAMETER: u8 = 0x41;
const CMD_LEAVE_PROGMODE: u8 = 0x51;
const CMD_LOAD_ADDR: u8 = 0x55;
const CMD_STK_UNIVERSAL: u8 = 0x56;
const CMD_PROG_PAGE: u8 = 0x64;
const CMD_STK_READ_PAGE: u8 = 0x74;
const CMD_READ_SIGNATURE: u8 = 0x75;

#[no_mangle]
pub extern "C" fn main() -> ! {
    serial::init(19200);

    let mut cur_addr: u16 = 0x0000;

    loop {
        let cmd_byte: u8 = serial::read_byte();

        match cmd_byte {
            CMD_GET_SYNC => {
                let _eop: u8 = serial::read_byte();

                serial::send_byte(RESP_STK_INSYNC);
                serial::send_byte(RESP_STK_OK);
            }

            CMD_LOAD_ADDR => {
                let addr_lo: u8 = serial::read_byte();
                let addr_hi: u8 = serial::read_byte();

                cur_addr = (((addr_hi as u16) << 8) | addr_lo as u16) << 1;
                cur_addr &= !0x7F;

                let _eop: u8 = serial::read_byte();

                serial::send_byte(RESP_STK_INSYNC);
                serial::send_byte(RESP_STK_OK);
            }

            /*
             * This is UUUGE!
             * Freakin assembly is clobbering r16.
             * r16 just so happens to be Rust's spot
             * for usart data register, so when reading
             * 0x03 at every byte after the write, its because the
             * asm writes 0x03 to r16 for write, then overwrites the
             * freakin usart register that contains the byte to send.
             * So stop using r16 in asm!
             */
            CMD_PROG_PAGE => {
                let len_hi: u8 = serial::read_byte();
                let len_lo: u8 = serial::read_byte();
                let memtype: u8 = serial::read_byte();

                let len: u16 = (len_hi as u16) << 8 | len_lo as u16;

                for i in (0..len).step_by(2) {
                    let word_lo: u8 = serial::read_byte();
                    let word_hi: u8 = serial::read_byte();
                    let word: u16 = ((word_hi as u16) << 8) | word_lo as u16;

                    if memtype == b'F' {
                        flash::word_to_buf(word, cur_addr + i);
                    }
                }

                let _eop: u8 = serial::read_byte();

                serial::send_byte(RESP_STK_INSYNC);
                flash::program_page(cur_addr);
                serial::send_byte(RESP_STK_OK);
            }

            CMD_STK_READ_PAGE => {
                let len_hi = serial::read_byte();
                let len_lo = serial::read_byte();
                let memtype = serial::read_byte();
                let _eop = serial::read_byte();

                let len = ((len_hi as u16) << 8) | len_lo as u16;

                serial::send_byte(RESP_STK_INSYNC);

                if memtype == b'F' {
                    let mut z: u16 = cur_addr & !0x7F;

                    for _ in 0..len {
                        let byte: u8;
                        unsafe {
                            asm!(
                                "lpm {b}, Z+",
                                b = out(reg) byte,
                                inout("Z") z,
                                options(nostack, preserves_flags)
                            );
                        }
                        serial::send_byte(byte);
                    }

                    cur_addr = z;
                }

                serial::send_byte(RESP_STK_OK);
            }

            CMD_READ_SIGNATURE => {
                let _eop = serial::read_byte();

                serial::send_byte(RESP_STK_INSYNC);

                /* atmega328p signature */
                serial::send_byte(0x1E);
                serial::send_byte(0x95);
                serial::send_byte(0x0F);

                serial::send_byte(RESP_STK_OK);
            }

            CMD_STK_UNIVERSAL => {
                serial::read_byte();
                serial::read_byte();
                serial::read_byte();
                serial::read_byte();
                let _eop: u8 = serial::read_byte();

                serial::send_byte(RESP_STK_INSYNC);
                serial::send_byte(0x03);
                serial::send_byte(RESP_STK_OK);
            }

            CMD_GET_PARAMETER => {
                let _parameter: u8 = serial::read_byte();
                let _eop: u8 = serial::read_byte();

                serial::send_byte(RESP_STK_INSYNC);
                serial::send_byte(0x00);
                serial::send_byte(RESP_STK_OK);
            }

            CMD_LEAVE_PROGMODE => {
                let _eop = serial::read_byte();

                serial::send_byte(RESP_STK_INSYNC);
                serial::send_byte(RESP_STK_OK);

                unsafe {
                    asm!("rjmp 0x0000");
                }
            }

            _ => {
                let mut byte: u8 = serial::read_byte();

                while byte != EOP {
                    byte = serial::read_byte();
                }

                serial::send_byte(RESP_STK_INSYNC);
                serial::send_byte(RESP_STK_OK);
            }
        }
    }
}
