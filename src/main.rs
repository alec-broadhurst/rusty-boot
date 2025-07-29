#![no_std]
#![no_main]

mod flash;
mod serial;
mod timer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    timer::init();
    serial::init();

    extern "C" {
        fn jmp_to_app();
    }

    let mut has_data = false;

    for _ in 0..500 {
        if serial::data_available() {
            has_data = true;
            break;
        };

        timer::wait_ms(1);
    }

    if !has_data {
        unsafe {
            flash::reenable_rww();
            jmp_to_app();
        }

        loop {}
    }

    let mut page_address: u16 = 0x0000;
    let mut page_buffer: [u8; 128] = [0; 128];

    for _ in (0..0x7530).step_by(128) {
        for i in 0..128 {
            match serial::read_byte(1000) {
                Some(byte) => page_buffer[i] = byte,
                None => unsafe {
                    flash::reenable_rww();
                    jmp_to_app();
                },
            };
        }

        unsafe {
            flash::write_page(page_address, page_buffer.as_ptr());
        }

        page_address += 64;
    }

    unsafe {
        flash::reenable_rww();
        jmp_to_app();
    }

    loop {}
}
