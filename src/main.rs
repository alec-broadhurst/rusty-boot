#![no_std]
#![no_main]

mod flash;
mod serial;

use flash::Flash;
use serial::Serial;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialize serial
    Serial::init();

    let mut page_address: u16 = 0x0000;
    let mut page_buffer: [u8; 128] = [0; 128];

    for _ in (0..0x7530).step_by(128) {
        // fill temp page buffer
        for i in 0..128 {
            page_buffer[i] = Serial::read_byte();
        }

        // write page to flash
        unsafe {
            Flash::write_page(page_address, page_buffer.as_ptr());
        }

        page_address += 64;
    }

    unsafe {
        Flash::reenable_rww();
    }

    loop {}
}
