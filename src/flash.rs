/// Writes a 128-byte page to flash memory at the given word address.
pub fn program_page(page_address: u16) {
    extern "C" {
        fn erase_page(word_address: u16);
        fn write_page(page_address: u16);
    }

    unsafe {
        erase_page(page_address);
        write_page(page_address);
        reenable_rww();
    }
}

/// Writes a word to the hardware page buffer
pub fn word_to_buf(word: u16, addr: u16) {
    extern "C" {
        fn word_to_buf(word: u16, addr: u16);
    }

    unsafe {
        word_to_buf(word, addr);
    }
}

/// Re-enables the Read-While-Write (RWW) section of flash.
///
/// Should be called after the last page write to restore read access to
/// the application section of flash memory.
pub fn reenable_rww() {
    extern "C" {
        fn reenable_rww();
    }

    unsafe {
        reenable_rww();
    }
}
