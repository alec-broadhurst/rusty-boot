#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::arch::asm;
use core::panic::PanicInfo;

mod flash;
mod serial;

use serial::Serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

#[repr(u8)]
enum Response {
    Ok = 0x10,
    InSync = 0x14,
    EndOfPacket = 0x20,
}

#[repr(u8)]
enum Command {
    GetSync = 0x30,
    GetParameter = 0x41,
    LeaveProgMode = 0x51,
    LoadAddr = 0x55,
    Universal = 0x56,
    ProgPage = 0x64,
    ReadPage = 0x74,
    ReadSignature = 0x75,
    Unknown = 0xFF,
}

impl From<u8> for Command {
    fn from(byte: u8) -> Self {
        match byte {
            0x30 => Command::GetSync,
            0x41 => Command::GetParameter,
            0x51 => Command::LeaveProgMode,
            0x55 => Command::LoadAddr,
            0x56 => Command::Universal,
            0x64 => Command::ProgPage,
            0x74 => Command::ReadPage,
            0x75 => Command::ReadSignature,
            _ => Command::Unknown,
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut serial: Serial = Serial::new(115200);

    let mut cur_addr: u16 = 0x0000;

    loop {
        let cmd_byte: Command = Command::from(serial.read_byte());

        match cmd_byte {
            Command::GetSync => {
                let _eop: u8 = serial.read_byte();

                serial.send_byte(Response::InSync as u8);
                serial.send_byte(Response::Ok as u8);
            }

            Command::LoadAddr => {
                let addr_lo: u8 = serial.read_byte();
                let addr_hi: u8 = serial.read_byte();

                cur_addr = (((addr_hi as u16) << 8) | addr_lo as u16) << 1;
                cur_addr &= !0x7F;

                let _eop: u8 = serial.read_byte();

                serial.send_byte(Response::InSync as u8);
                serial.send_byte(Response::Ok as u8);
            }

            Command::ProgPage => {
                let len_hi: u8 = serial.read_byte();
                let len_lo: u8 = serial.read_byte();
                let memtype: u8 = serial.read_byte();

                let len: u16 = (len_hi as u16) << 8 | len_lo as u16;

                for i in (0..len).step_by(2) {
                    let word_lo: u8 = serial.read_byte();
                    let word_hi: u8 = serial.read_byte();
                    let word: u16 = ((word_hi as u16) << 8) | word_lo as u16;

                    if memtype == b'F' {
                        unsafe { flash::write_word(word, cur_addr + i) };
                    }
                }

                let _eop: u8 = serial.read_byte();

                serial.send_byte(Response::InSync as u8);
                unsafe {
                    flash::program_page(cur_addr);
                }
                serial.send_byte(Response::Ok as u8);
            }

            Command::ReadPage => {
                let len_hi = serial.read_byte();
                let len_lo = serial.read_byte();
                let memtype = serial.read_byte();
                let _eop = serial.read_byte();

                let len = ((len_hi as u16) << 8) | len_lo as u16;

                serial.send_byte(Response::InSync as u8);

                if memtype == b'F' {
                    for i in 0..len {
                        let mem_byte: u8 = unsafe { flash::read_byte(cur_addr + i) };
                        serial.send_byte(mem_byte);
                    }
                }

                serial.send_byte(Response::Ok as u8);
            }

            Command::ReadSignature => {
                let _eop = serial.read_byte();

                serial.send_byte(Response::InSync as u8);

                /* atmega328p signature */
                serial.send_byte(0x1E);
                serial.send_byte(0x95);
                serial.send_byte(0x0F);

                serial.send_byte(Response::Ok as u8);
            }

            Command::Universal => {
                let _byte1: u8 = serial.read_byte();
                let _byte2: u8 = serial.read_byte();
                let _byte3: u8 = serial.read_byte();
                let _byte4: u8 = serial.read_byte();
                let _eop: u8 = serial.read_byte();

                serial.send_byte(Response::InSync as u8);
                serial.send_byte(0x03);
                serial.send_byte(Response::Ok as u8);
            }

            Command::GetParameter => {
                let _parameter: u8 = serial.read_byte();
                let _eop: u8 = serial.read_byte();

                serial.send_byte(Response::InSync as u8);
                serial.send_byte(0x00);
                serial.send_byte(Response::Ok as u8);
            }

            Command::LeaveProgMode => {
                let _eop = serial.read_byte();

                serial.send_byte(Response::InSync as u8);
                serial.send_byte(Response::Ok as u8);

                // force a watchdog reset
                loop {}
            }

            Command::Unknown => {
                let mut byte: u8 = serial.read_byte();

                while byte != Response::EndOfPacket as u8 {
                    byte = serial.read_byte();
                }

                serial.send_byte(Response::InSync as u8);
                serial.send_byte(Response::Ok as u8);
            }
        }

        unsafe {
            asm!("wdr");
        }
    }
}
